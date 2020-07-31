use seed::prelude::*;
use seed::*;
use web_sys::AudioContext;
use web_sys::OscillatorType;
use web_sys::{GainNode, OscillatorNode};

use seed_style::px; // almost always want seed-style px instead of seed px
use seed_style::*;

mod app;
mod global_styles;

//
//  Model, Msg, Update, init(), and start()
//  ---------------------------------------

pub struct Model {
    oscillator: OscillatorNode,
    gain: GainNode,
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
            model.gain.gain().set_value(11.);
            //model.oscillator.start().unwrap();
        }
        Msg::StopSound => {
            model.gain.gain().set_value(0.);
            //model.oscillator.stop().unwrap();
        }
    }
}

fn make_a_oscillator(freq: f32) -> Result<(OscillatorNode, GainNode), JsValue> {
    let audio_context = AudioContext::new()?;

    let oscillator = audio_context.create_oscillator()?;
    oscillator.set_type(OscillatorType::Sine);
    oscillator.frequency().set_value(freq);

    let gain = audio_context.create_gain()?;
    oscillator.connect_with_audio_node(&gain)?;
    gain.gain().set_value(0.);
    gain.connect_with_audio_node(&audio_context.destination())?;
    oscillator.start()?;
    Ok((oscillator, gain))
}

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    global_styles::global_init();
    let (oscillator, gain) = make_a_oscillator(440.).unwrap();
    Model { oscillator, gain }
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
        button!["start", input_ev(Ev::Click, |_| Msg::ProduceSound)],
        button!["stop", input_ev(Ev::Click, |_| Msg::StopSound)]
    ]
}
