use seed::prelude::JsValue;
use web_sys::OscillatorType;
use web_sys::{AudioContext, GainNode, OscillatorNode};

#[derive(Clone, Debug, Default)]
pub struct Sound {
    tones: Vec<Tone>,
}

impl Sound {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_tones(tones: Vec<Tone>) -> Self {
        Self { tones }
    }

    pub fn add_tone(mut self, s: Tone) -> Self {
        self.tones.push(s);
        self
    }

    pub fn play(&self) {
        for tone in &self.tones {
            tone.play();
        }
    }

    pub fn pause(&self) {
        for tone in &self.tones {
            tone.pause();
        }
    }

    pub fn half_gain(&self) {
        for tone in &self.tones {
            tone.half_gain();
        }
    }
}

#[derive(Clone, Debug)]
pub struct Tone {
    oscillator: OscillatorNode,
    gain: GainNode,
    context: AudioContext,
    gain_val: f32,
}

impl Tone {
    pub fn play(&self) {
        self.context.resume().unwrap(); // Fix for Chromium
        self.gain.gain().set_value(self.gain_val);
    }

    pub fn pause(&self) {
        self.gain.gain().set_value(0.0);
    }

    fn half_gain(&self) {
        let val = self.gain.gain().value();
        self.gain.gain().set_value(val / 2.0);
    }
}

pub struct ToneBuilder {
    osc_type: OscillatorType,
    freq: f32,
    gain: f32,
}

impl ToneBuilder {
    pub fn new() -> Self {
        ToneBuilder {
            osc_type: OscillatorType::Sine,
            freq: 440.0,
            gain: 0.3,
        }
    }

    pub fn osc_type(self, osc_type: OscillatorType) -> Self {
        ToneBuilder { osc_type, ..self }
    }

    pub fn freq(self, freq: f32) -> Self {
        ToneBuilder { freq, ..self }
    }

    pub fn gain(self, gain: f32) -> Self {
        ToneBuilder { gain, ..self }
    }

    pub fn build(self) -> Result<Tone, JsValue> {
        let context = AudioContext::new()?;

        let gain = context.create_gain()?;
        gain.gain().set_value(0.);
        gain.connect_with_audio_node(&context.destination())?;

        let oscillator = context.create_oscillator()?;
        oscillator.set_type(self.osc_type);
        oscillator.frequency().set_value(self.freq);
        oscillator.connect_with_audio_node(&gain)?;
        oscillator.start()?;

        Ok(Tone {
            oscillator,
            gain,
            context,
            gain_val: self.gain,
        })
    }
}
