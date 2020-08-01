use seed::prelude::*;
use seed::*;
use seed_hooks::*;
use web_sys::OscillatorType;

use seed_style::px; // almost always want seed-style px instead of seed px
use seed_style::{pc, *};
use web_sys::{HtmlCanvasElement, HtmlElement};

mod app;
mod global_styles;
mod sound;
use sound::Sound;
mod rhythm;
use rhythm::{Beat, Neighbours, Rhythm};
use sound::{Tone, ToneBuilder};
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
    sound: Tone,
    sound_selector: ElRef<HtmlCanvasElement>,
    beat_bars: Vec<Rhythm>,
}

impl Model {
    pub fn secs_elapsed(&self) -> f64 {
        (self.current_time_step as f64) / 60.0
    }

    pub fn get_sound_from_selector_pane(&self, row: usize) -> Sound {
        Sound::from_tones(vec![crate::sound::ToneBuilder::new()
            .gain(4.0)
            .freq(400.0)
            .build()
            .unwrap()])
    }
}

pub static MAIN_LOOP_DURATION: f64 = 48.0;
pub static TICKS_IN_ONE_BAR: u64 = ((MAIN_LOOP_DURATION / 48.0) * 60.0) as u64;

// In aps that make use of conditional rendering on breakpoints we We just need one Msg
// in order to handle a WindowResized event.
#[derive(Clone, Debug)]
pub enum Msg {
    BarToggled(usize, usize),
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
}
fn update(msg: Msg, mut model: &mut Model, _orders: &mut impl Orders<Msg>) {
    // log!(msg);    // always worth logging the message in development for debug purposes.
    match msg {
        Msg::BarToggled(row, pos) => sequencer_controller::bar_toggled(model, row, pos),
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
            let canvas_el = model.sound_selector.get().unwrap();
            let width = canvas_el.width() as f32;
            let height = canvas_el.height() as f32;

            let el: HtmlElement = canvas_el.clone().into();

            let relative_pos_x = x - el.offset_left();
            let relative_pos_y = y - el.offset_top();

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

            context.fill_rect(relative_pos_x as f64, relative_pos_y as f64, 5., 5.);

            let freq = (relative_pos_x as f32 * 11_00. / width) as f32;
            let vol = (relative_pos_y as f32 * 10. / height) as f32;

            model.sound = ToneBuilder::new().gain(vol).freq(freq).build().unwrap();
        }
        Msg::ToggleBar(row, pos) => {
            let rhythm: &mut Rhythm = model.beat_bars.get_mut(row).unwrap();
            let beat: &mut Beat = &mut rhythm.0[pos];
            *beat = beat.toggle();
        }
        Msg::NoOp => {}
    }
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

    let sound = Sound::new()
        .add_tone(ToneBuilder::new().freq(500.0).gain(0.5).build().unwrap())
        .add_tone(ToneBuilder::new().freq(250.0).gain(0.5).build().unwrap());
    let sound = ToneBuilder::new().freq(500.).build().unwrap();
    Model {
        sound,
        sound_selector: ElRef::<HtmlCanvasElement>::default(),
        beat_bars: { Rhythm::standard().into() },
        current_time_step: 0,
        sound_scheduler: SoundScheduler::default(),
        rows: (0..=5).map(|idx| Row::new(idx)).collect(),
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
        s().display_grid()
            .grid_template_rows("auto 300px")
            .height(pc(100))
            .width(pc(100)),
        div![canvas![
            el_ref(&model.sound_selector),
            style![
                St::Border => "1px solid black",
            ],
            mouse_ev(Ev::MouseDown, |event| Msg::Click(event.x(), event.y()))
        ],],
        div![
            s().height(pc(100)).display_grid(),
            model
                .beat_bars
                .iter()
                .enumerate()
                .map(beat_bar)
                .collect::<Vec<Node<Msg>>>()
        ]
    ]
}

fn beat_bar((index, bar_data): (usize, &Rhythm)) -> Node<Msg> {
    div![
        s().display_grid()
            .grid_template_columns("200px auto")
            .height(pc(100)),
        div![
            s().background_color("#000")
                .display_flex()
                .justify_content_center()
                .align_items_center(),
            button!["Select this rhythm", input_ev(Ev::Click, |_| panic!())]
        ],
        div![
            s().display_grid()
                .grid_auto_flow("column")
                .width(pc(100))
                .margin_top(px(20))
                .margin_bottom(px(20)),
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
                Beat::Play => "#F00",
                Beat::Pause => "#FFF",
            }),
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
            input_ev(Ev::Click, move |_| Msg::ToggleBar(row, index))
        ]
    }
}
