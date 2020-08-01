use web_sys::OscillatorType;
use web_sys::{AudioContext, GainNode, OscillatorNode};
use seed::prelude::JsValue;

pub struct Sound {
    oscillator: OscillatorNode,
    gain: GainNode,
    context: AudioContext,
    gain_val: f32,
}

impl Sound {
    pub fn play(&self) {
        self.gain.gain().set_value(self.gain_val);
    }

    pub fn pause(&self) {
        self.gain.gain().set_value(0.0);
    }
}

pub struct SoundBuilder {
    osc_type: OscillatorType,
    freq: f32,
    gain: f32,
}

impl SoundBuilder {
    pub fn new() -> Self {
        SoundBuilder {
            osc_type: OscillatorType::Sine,
            freq: 440.0,
            gain: 0.3,
        }
    }

    pub fn osc_type(self, osc_type: OscillatorType) -> Self {
        SoundBuilder { osc_type, ..self }
    }

    pub fn freq(self, freq: f32) -> Self {
        SoundBuilder { freq, ..self }
    }

    pub fn gain(self, gain: f32) -> Self {
        SoundBuilder { gain, ..self }
    }
    
    pub fn build(self) -> Result<Sound, JsValue> {
        // should this be declared here?
        let context = AudioContext::new()?;

        let gain = context.create_gain()?;
        gain.gain().set_value(0.);
        gain.connect_with_audio_node(&context.destination())?;

        let oscillator = context.create_oscillator()?;
        oscillator.set_type(self.osc_type);
        oscillator.frequency().set_value(self.freq);
        oscillator.connect_with_audio_node(&gain)?;
        oscillator.start()?;

        Ok(Sound { 
            oscillator, 
            gain, 
            context,
            gain_val: self.gain,
        })
    }
}
