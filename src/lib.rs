use seed::prelude::*;
use seed::*;
use seed_hooks::*;
use web_sys::OscillatorType;

use seed_style::px; // almost always want seed-style px instead of seed px
use seed_style::{pc, *};
use web_sys::{HtmlCanvasElement, HtmlElement};

mod app;
mod draw;
use draw::Draw;
mod global_styles;
mod sound;
use sound::Sound;
mod rhythm;
use rhythm::{Beat, Neighbours, Rhythm};
mod main_loop;
mod raf_loop;
mod sequencer_controller;
mod sound_scheduler;
mod testing_ui;
use sound_scheduler::*;
mod row_and_bars;
use row_and_bars::{Bar, Row};

//
//  Model, Msg, Update, init(), and start()
//  ---------------------------------------

pub struct Model {
    current_time_step: u64,
    sound_scheduler: SoundScheduler,
    sound: Sound,
    sound_selector: ElRef<HtmlCanvasElement>,
    rows: Vec<Row>,
    beat_bars: Vec<(
        Rhythm,
        // Where the rhythm is attached to the canvas
        Option<(f64, f64)>,
    )>,
    /// The row a user has selected.
    selected_row: usize,
    mouse_down: bool,
    clicked_beat: Beat,
}

impl Model {
    pub fn secs_elapsed(&self) -> f64 {
        (self.current_time_step as f64) / 60.0
    }
}

pub static MAIN_LOOP_DURATION: f64 = 48.0;
pub static TICKS_IN_ONE_BAR: u64 = ((MAIN_LOOP_DURATION / 48.0) * 60.0) as u64;

// In aps that make use of conditional rendering on breakpoints we We just need one Msg
// in order to handle a WindowResized event.
#[derive(Clone, Debug)]
pub enum Msg {
    TimeStepAdvanced,
    TimeStepLoopStopped,
    TimeStepLoopStarted,
    Scheduler(SoundSchedulerMsg),
    // WindowResized, // needed for responsive styles
    // NoOp,
    ProduceSound,
    StopSound,
    Click(i32, i32),
    NoOp,

    ToggleBar(usize, usize),
    ForceToggleBar(usize, usize),
    GlobalMouseDown,
    GlobalMouseUp,
    SelectRow(usize),
    ResizeCanvas,
}
fn update(msg: Msg, mut model: &mut Model, orders: &mut impl Orders<Msg>) {
    // log!(msg);    // always worth logging the message in development for debug purposes.
    match msg {
        Msg::TimeStepAdvanced => main_loop::time_step_advanced(&mut model),
        Msg::TimeStepLoopStopped => main_loop::time_step_loop_stopped(&mut model),
        Msg::TimeStepLoopStarted => main_loop::time_step_loop_started(&mut model),
        Msg::Scheduler(msg) => sound_scheduler::update(msg, &mut model),
        Msg::ProduceSound => {
            model.sound.play();
        }
        Msg::StopSound => {
            model.sound.pause();
        }
        Msg::Click(x, y) => {
            let (_, pos) = model.beat_bars.get_mut(model.selected_row).unwrap();

            let canvas_el = model.sound_selector.get().unwrap();

            set_canvas_size(&canvas_el);

            let width = canvas_el.width() as f32;
            let height = canvas_el.height() as f32;

            let el: HtmlElement = canvas_el.clone().into();

            let relative_pos_x = x - el.offset_left();
            let relative_pos_y = y - el.offset_top();

            *pos = Some((relative_pos_x as f64, relative_pos_y as f64));

            log!("new one");

            log!(x);
            log!(y);

            log!(el.offset_left());
            log!(el.offset_top());

            log!(relative_pos_x);
            log!(relative_pos_y);

            let context = canvas_el
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<web_sys::CanvasRenderingContext2d>()
                .unwrap();

            new_canvas_frame(model, canvas_el, context);
            let freq = (relative_pos_x as f32 * 11_00. / width) as f32;
            let vol = (relative_pos_y as f32 * 10. / height) as f32;

            // model.sound = Sound::default().gain(vol).freq(freq);

            if let Some(selected_row) = model.rows.get_mut(model.selected_row) {
                selected_row.sound = selected_row.sound.clone().gain(vol).freq(freq);
            }
        }
        Msg::ToggleBar(row, pos) => {
            if model.mouse_down {
                let rhythm: &mut Rhythm = &mut model.beat_bars.get_mut(row).unwrap().0;
                let beat: &mut Beat = &mut rhythm.0[pos];
                if model.clicked_beat != *beat {
                    *beat = model.clicked_beat;
                    sequencer_controller::bar_toggled(model, row, pos);
                }
            }
        }
        Msg::ForceToggleBar(row, pos) => {
            log!("force");
            let rhythm: &mut Rhythm = &mut model.beat_bars.get_mut(row).unwrap().0;
            let beat: &mut Beat = &mut rhythm.0[pos];
            *beat = beat.toggle();
            model.clicked_beat = *beat;
            sequencer_controller::bar_toggled(model, row, pos);
        }

        Msg::SelectRow(row) => model.selected_row = row,
        Msg::ResizeCanvas => match &model.sound_selector.get() {
            Some(x) => set_canvas_size(x),
            None => {
                orders.after_next_render(|_| Msg::ResizeCanvas);
            }
        },
        Msg::NoOp => {}
        Msg::GlobalMouseDown => model.mouse_down = true,
        Msg::GlobalMouseUp => model.mouse_down = false,
    }
}

fn new_canvas_frame(
    model: &Model,
    canvas_el: web_sys::HtmlCanvasElement,
    ctx: web_sys::CanvasRenderingContext2d,
) {
    let width = canvas_el.width() as f64;
    let height = canvas_el.height() as f64;

    ctx.clear_rect(0., 0., width, height);

    for (index, (x, y)) in model
        .beat_bars
        .iter()
        .map(|x| x.1)
        .enumerate()
        .flat_map(|(index, pos)| pos.map(|pos| (index, pos)))
    {
        ctx.set_fill_style(&JsValue::from_str(row_colour(index)));
        draw::Rect::crosshair((x, y)).draw(&ctx);
    }
}
fn set_canvas_size(canvas_el: &web_sys::HtmlCanvasElement) {
    let window = web_sys::window().unwrap();

    let window_height = window.inner_height().unwrap();

    let window_width = window.inner_width().unwrap();

    canvas_el.set_width(window_width.as_f64().unwrap().round() as u32);
    canvas_el.set_height(window_height.as_f64().unwrap().round() as u32 - 300);
}

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    global_styles::global_init();

    orders
        .subscribe(
            move |subs::UrlChanged(mut url)| {
                let new_page = match url.remaining_path_parts().as_slice() {
                    ["home"] => Page::MainApp,
                    ["hidden_ui"] => Page::HiddenTestUI,
                    _ => Page::MainApp,
                };

                if current_page().get() != new_page {
                    window().scroll_to_with_x_and_y(0., 0.);
                    current_page().set(new_page);
                }
                Msg::NoOp
            }, //
        )
        .notify(subs::UrlChanged(url));

    orders.send_msg(Msg::ResizeCanvas);

    let sound = Sound::default().gain(1.8).freq(440.0);
    let default_rhythm = Rhythm::standard()
        .into_iter()
        .map(|rhythm| (*rhythm, None))
        .collect();
    let mut scheduler = SoundScheduler::default();
    scheduler.init_with_rhythm(&default_rhythm);
    let mut vec_of_rows = (0..=5).map(|idx| Row::new(idx)).collect();
    row_and_bars::init_rows_in_model(&mut vec_of_rows, &default_rhythm);
    Model {
        sound,
        sound_selector: ElRef::<HtmlCanvasElement>::default(),
        beat_bars: default_rhythm,
        current_time_step: 0,
        rows: vec_of_rows,
        sound_scheduler: scheduler,
        selected_row: 0,
        mouse_down: false,
        clicked_beat: Beat::Play,
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    let app = App::start("app", init, update, view);
    my_app().set(Some(app));
}

// Provide access to the app incase one wants to force an update from anywhere in the app
#[atom]
fn my_app() -> Atom<Option<App<Msg, Model, Node<Msg>>>> {
    None
}

#[derive(Clone, PartialEq)]
pub enum Page {
    MainApp,
    HiddenTestUI,
}

#[atom]
fn current_page() -> Atom<Page> {
    Page::MainApp
}

//  View Entry Here, Sets up theme access, two themes are allowed access
//
//  The first is the app defined theme, the second provides access to seed style presets.
//  The content block also activates themed global_styles.
//  ---------------
pub fn view(model: &Model) -> Node<Msg> {
    match current_page().get() {
        Page::MainApp => app_view(model),
        Page::HiddenTestUI => testing_ui::view(model),
    }
}

pub fn app_view(model: &Model) -> Node<Msg> {
    raf_loop::raf_loop_atom().get();

    div![
        ev(Ev::MouseDown, |_| Msg::GlobalMouseDown),
        ev(Ev::MouseUp, |_| Msg::GlobalMouseUp),
        s().display_grid()
            .grid_template_rows("auto 300px")
            .height(pc(100))
            .width(pc(100)),
        div![canvas![
            el_ref(&model.sound_selector),
            mouse_ev(Ev::MouseDown, |event| Msg::Click(event.x(), event.y()))
        ],],
        div![
            s().height(pc(100)).display_grid(),
            div![
                button![
                    "play",
                    raf_loop::raf_loop_atom().on_click(|raf| raf.start()),
                ],
                button![
                    "pause",
                    raf_loop::raf_loop_atom().on_click(|raf| raf.stop()),
                ],
                p![
                    "Current Time:",
                    format!("{:.2}", model.secs_elapsed()),
                    ", Current Time step:",
                    model.current_time_step
                ]
            ],
            model
                .beat_bars
                .iter()
                .map(|x| &x.0)
                .enumerate()
                .map(beat_bar)
                .collect::<Vec<Node<Msg>>>()
        ],
    ]
}

fn beat_bar((index, bar_data): (usize, &Rhythm)) -> Node<Msg> {
    div![
        s().display_grid()
            .grid_template_columns("200px auto")
            .height(pc(100)),
        div![
            s().background_color(row_colour(index))
                .display_flex()
                .justify_content_center()
                .align_items_center(),
            button![
                "Select this rhythm",
                input_ev(Ev::Click, move |_| Msg::SelectRow(index))
            ]
        ],
        div![
            s().display_grid()
                .grid_auto_flow("column")
                .width(pc(100))
                .margin_top(px(20))
                .margin_bottom(px(20))
                .user_select("none"),
            bar_data
                .0
                .iter()
                .zip(bar_data.neighbours())
                .enumerate()
                .map(beat_bar_box(index))
                .collect::<Vec<Node<Msg>>>()
        ],
    ]
}

fn beat_bar_box(row: usize) -> impl Fn((usize, (&Beat, Neighbours))) -> Node<Msg> {
    move |(index, (beat, mut neighbours))| {
        if index == 0 {
            neighbours.left = true
        } else if index == 47 {
            neighbours.right = true
        };
        div![
            s().background_color(match beat {
                Beat::Play => row_colour_dark(row),
                Beat::Pause => "#FFF",
            })
            .user_select("none"),
            match neighbours {
                Neighbours {
                    left: true,
                    right: true,
                } => {
                    s()
                }
                Neighbours {
                    left: true,
                    right: false,
                } => {
                    s().border_radius("0 1000px 1000px 0")
                }
                Neighbours {
                    left: false,
                    right: true,
                } => {
                    s().border_radius("1000px 0 0 1000px")
                }
                Neighbours {
                    left: false,
                    right: false,
                } => {
                    s().border_radius("1000px")
                }
            },
            // mouse_ev(Ev::Click, move |_| Msg::ToggleBar(row, index)),
            mouse_ev(Ev::MouseOver, move |_| Msg::ToggleBar(row, index)),
            mouse_ev(Ev::MouseDown, move |_| Msg::ForceToggleBar(row, index)),
        ]
    }
}

fn row_colour(index: usize) -> &'static str {
    match index {
        1 => "#F00",
        2 => "#0F0",
        3 => "#00F",
        4 => "#FF0",
        5 => "#F0F",
        _ => "#0FF",
    }
}

fn row_colour_dark(index: usize) -> &'static str {
    match index {
        1 => "#800",
        2 => "#080",
        3 => "#008",
        4 => "#880",
        5 => "#808",
        _ => "#088",
    }
}
