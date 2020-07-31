use seed::prelude::*;
use seed::*;
use web_sys::AudioContext;
use web_sys::OscillatorType;

use seed_style::px; // almost always want seed-style px instead of seed px
use seed_style::{pc, *};

mod app;
mod global_styles;
mod sound;
use sound::{Sound, SoundBuilder};

//
//  Model, Msg, Update, init(), and start()
//  ---------------------------------------

pub struct Model {
    sound: Sound,
}

// In aps that make use of conditional rendering on breakpoints we We just need one Msg
// in order to handle a WindowResized event.
#[derive(Clone, Debug)]
pub enum Msg {
    ProduceSound,
    StopSound,
}

fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    log!(msg);

    match msg {
        Msg::ProduceSound => {
            model.sound.play();
        }
        Msg::StopSound => {
            model.sound.pause();
        }
    }
}

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    global_styles::global_init();
    let sound = SoundBuilder::new()
        .freq(500.)
        .build()
        .unwrap();
    Model { sound }
}

#[wasm_bindgen(start)]
pub fn start() {
    let _app = App::start("app", init, update, view);
}

//  View Entry Here, Sets up theme access, two themes are allowed access
//
//  The first is the app defined theme, the second provides access to seed style presets.
//  The content block also activates themed global_styles.
//  ---------------
pub fn view(model: &Model) -> Node<Msg> {
    div![
        s().display_grid()
            .grid_template_rows("auto 300px")
            .height(pc(100))
            .width(pc(100)),
        div![
            "hello world",
            // Where to put the canvas
        ],
        div![
            s().display_flex().flex_direction_row(),
            div![s().width(px(200)).flex_none(), "Hello World"],
            button!["start", input_ev(Ev::Click, |_| Msg::ProduceSound)],
            button!["stop", input_ev(Ev::Click, |_| Msg::StopSound)]
        ]
    ]
}
