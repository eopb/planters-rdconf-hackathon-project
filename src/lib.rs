use seed::prelude::*;
use seed::*;
use seed_hooks::*;

use seed_style::px; // almost always want seed-style px instead of seed px
use seed_style::*;

mod global_styles;
mod theme;
mod home;
mod app;
mod shared;

mod raf_loop;
mod main_loop;

use theme::*;

//
//  Model, Msg, Update, init(), and start()
//  ---------------------------------------

pub struct Model {
    current_time_step: u64,
}

impl Model {
    pub fn secs_elapsed(&self) -> f64 {
        (self.current_time_step as f64) * 1000.0/60.0
    }
}

// In aps that make use of conditional rendering on breakpoints we We just need one Msg
// in order to handle a WindowResized event.
#[derive(Clone,Debug)]
pub enum Msg {
    TimeStepAdvanced,
    TimeStepLoopStopped,
    TimeStepLoopStarted,
    WindowResized, // needed for responsive styles
    NoOp,
}
fn update(msg: Msg, mut model: &mut Model, _orders: &mut impl Orders<Msg>) {
    // log!(msg);    // always worth logging the message in development for debug purposes.
    match msg {
        Msg::TimeStepAdvanced => main_loop::time_step_advanced(&mut model),
        Msg::TimeStepLoopStopped => main_loop::time_step_loop_stopped(&mut model),
        Msg::TimeStepLoopStarted => main_loop::time_step_loop_started(&mut model),
        _ => {}
    }
}


fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    
    orders
        .subscribe(move |subs::UrlChanged(mut url)| {
            let new_page = match url.remaining_path_parts().as_slice() {
                ["home"] => Page::Home,
                ["app"] => Page::App,
                _ =>  Page::Home,
            };

            if current_page().get() != new_page {
                window().scroll_to_with_x_and_y(0., 0.);
                current_page().set(new_page);
            }
            Msg::NoOp
            }
            // 
        )
        .notify(subs::UrlChanged(url));
    // allow app to access the user theme, plus seed style presets.
    // app_themes().set(vec![user_theme(), default_color_theme()]);
    load_app_themes(&[ user_theme, default_colors_theme,default_breakpoint_theme]);
    // themes values can now be used in the global style init
    global_styles::global_init();
    Model {
        current_time_step: 0,
    }

}

#[wasm_bindgen(start)]
pub fn start() {
    let app  = App::start("app", init, update, view);
    my_app().set(Some(app));
}

#[derive(Clone,PartialEq,Debug,Copy)]
enum Page {
    Home,
    App,
}

#[atom]
fn current_page() -> Atom<Page>{
    Page::Home
}

// Provide access to the app incase one wants to force an update from anywhere in the app
#[atom]
fn my_app() -> Atom<Option<App<Msg,Model,Node<Msg>>>>{
    None
}

//  View Entry Here, Sets up theme access, two themes are allowed access
//
//  The first is the app defined theme, the second provides access to seed style presets.
//  The content block also activates themed global_styles.
//  ---------------
#[topo::nested] // Needed for Seed Hooks
pub fn view(model: &Model) -> Node<Msg> {
    raf_loop::raf_loop_atom().get();

    div![
        s().display_flex().flex_direction_row(),
        div![
            s().width(px(200)).flex_none().bg_color(seed_colors::Gray::No4),
            shared::sidebar(model),
        ],
        button![
            "Start Event Loop",
            raf_loop::raf_loop_atom().on_click(|raf| raf.start()),
        ],
        button![
            "Stop Event Loop",
            raf_loop::raf_loop_atom().on_click(|raf| raf.stop()),
        ],
        div![
            s().flex("0 1 100%"),
            match current_page().get() {
                Page::Home => home::view(model),
                Page::App => app::view(model),
            }
        ]
        
    ]
}


