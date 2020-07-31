use seed::prelude::*;
use seed::*;
use web_sys::AudioContext;
use web_sys::OscillatorType;

use seed_style::px; // almost always want seed-style px instead of seed px
use seed_style::*;

mod app;
mod global_styles;

//
//  Model, Msg, Update, init(), and start()
//  ---------------------------------------

pub struct Model {
    //oscillator: OscillatorNode,
}

// In aps that make use of conditional rendering on breakpoints we We just need one Msg
// in order to handle a WindowResized event.
#[derive(Clone, Debug)]
pub enum Msg {
    ProduceSound,
}

fn update(msg: Msg, _model: &mut Model, _orders: &mut impl Orders<Msg>) {
    log!(msg);

    match msg {
        Msg::ProduceSound => {
            make_a_sound().unwrap();
        }
    }
}

fn make_a_sound() -> Result<(), JsValue> {
    let audio_context = AudioContext::new()?;

    let oscillator = audio_context.create_oscillator()?;
    oscillator.set_type(OscillatorType::Sine);
    oscillator.frequency().set_value(840.);

    let gain = audio_context.create_gain()?;
    oscillator.connect_with_audio_node(&gain)?;
    gain.connect_with_audio_node(&audio_context.destination())?;

    oscillator.start()?;
    Ok(())
}

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.send_msg(Msg::ProduceSound);

    global_styles::global_init();
    Model {}
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
        s().display_flex().flex_direction_row(),
        div![s().width(px(200)).flex_none(), "Hello World"],
    ]
}
