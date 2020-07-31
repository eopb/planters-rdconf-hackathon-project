use seed::prelude::*;
use seed::*;

use seed_style::px; // almost always want seed-style px instead of seed px
use seed_style::*;

mod global_styles;
mod app;

//
//  Model, Msg, Update, init(), and start()
//  ---------------------------------------

pub struct Model {}

// In aps that make use of conditional rendering on breakpoints we We just need one Msg
// in order to handle a WindowResized event.
#[derive(Clone,Debug)]
pub enum Msg {
    ProduceSound,
}

fn update(msg: Msg, _model: &mut Model, _orders: &mut impl Orders<Msg>) {
    log!(msg);    // always worth logging the message in development for debug purposes.
}

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    // allow app to access the user theme, plus seed style presets.
    // app_themes().set(vec![user_theme(), default_color_theme()]);
    // themes values can now be used in the global style init
    global_styles::global_init();
    Model {}
}

#[wasm_bindgen(start)]
pub fn start() {
    let _app  = App::start("app", init, update, view);
}

//  View Entry Here, Sets up theme access, two themes are allowed access
//
//  The first is the app defined theme, the second provides access to seed style presets.
//  The content block also activates themed global_styles.
//  ---------------
pub fn view(model: &Model) -> Node<Msg> {
    div![
        s().display_flex().flex_direction_row(),
        div![
            s().width(px(200)).flex_none(),
            "Hello World"
        ],
    ]
}