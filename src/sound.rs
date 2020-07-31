use web_sys::AudioContext;
use web_sys::OscillatorType;
use web_sys::{GainNode, OscillatorNode};
use seed::prelude::JsValue;

pub struct Sound {
    oscillator: OscillatorNode,
    gain: GainNode,
    gain_val: f32,
}

impl Sound {
    pub fn play(&self) {
        self.gain.gain().set_value(self.gain_val);
    }

    pub fn pause(&self) {
        self.gain.gain().set_value(0.);
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
            freq: 0.0,
            gain: 0.0,
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
        let audio_context = AudioContext::new()?;

        let oscillator = audio_context.create_oscillator()?;
        oscillator.set_type(self.osc_type);
        oscillator.frequency().set_value(self.freq);

        let gain = audio_context.create_gain()?;
        oscillator.connect_with_audio_node(&gain)?;
        gain.gain().set_value(0.);
        gain.connect_with_audio_node(&audio_context.destination())?;
        oscillator.start()?;
        Ok(Sound { 
            oscillator, 
            gain, 
            gain_val: self.gain 
        })
    }
}
