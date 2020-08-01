use seed::prelude::JsValue;
use web_sys::OscillatorType;
use web_sys::{AudioContext, GainNode, OscillatorNode};

#[derive(Debug, Clone)]
pub enum SoundStatus {
    Unplayed,
    Played,
}

impl Default for SoundStatus {
    fn default() -> Self {
        SoundStatus::Unplayed
    }
}

#[derive(Clone, Debug, Default)]
pub struct Sound {
    tones: Vec<Tone>,
    status: SoundStatus,
}

impl Sound {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_tones(tones: Vec<Tone>) -> Self {
        Self {
            tones,
            status: SoundStatus::Unplayed,
        }
    }

    pub fn add_tone(mut self, s: Tone) -> Self {
        self.tones.push(s);
        self
    }

    pub fn play(&mut self) {
        self.status = SoundStatus::Played;
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
        self.gain
            .gain() //.set_value(self.gain_val);
            .set_target_at_time(self.gain_val, self.context.current_time(), 0.015)
            .unwrap();
    }

    pub fn pause(&self) {
        self.gain
            .gain()
            .set_target_at_time(0.0, self.context.current_time(), 0.015)
            .unwrap();
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
        gain.gain().set_value(0.0);
        gain.connect_with_audio_node(&context.destination())?;

        let oscillator = context.create_oscillator()?;
        oscillator.set_type(self.osc_type);
        oscillator.frequency().set_value(self.freq);
        oscillator.connect_with_audio_node(&gain)?;

        context.resume()?;
        oscillator.start()?;

        Ok(Tone {
            oscillator,
            gain,
            context,
            gain_val: self.gain,
        })
    }

    pub fn to_sound(self) -> Result<Sound, JsValue> {
        let tone = self.build()?;
        Ok(Sound::from_tones(vec![tone]))
    }
}

struct SSound {
    oscillator: OscillatorNode,
    gain_node: GainNode,
    context: AudioContext,
    gain: f32,
    freq: f32,
    shape: OscillatorType,
}

impl Default for SSound {
    fn default() -> Self {
        Self::build().unwrap()
    }
}

impl SSound {
    fn build() -> Result<Self, JsValue> {
        let shape = OscillatorType::Sine;
        let freq = 440.0;
        let gain = 0.0;

        let context = AudioContext::new()?;

        let gain_node = context.create_gain()?;
        gain_node.gain().set_value(gain);
        gain_node.connect_with_audio_node(&context.destination())?;

        let oscillator = context.create_oscillator()?;
        oscillator.set_type(shape);
        oscillator.frequency().set_value(freq);
        oscillator.connect_with_audio_node(&gain_node)?;

        Ok(SSound {
            oscillator,
            gain_node,
            context,
            gain,
            freq,
            shape,
        })
    }

    pub fn freq(mut self, freq: f32) -> Self {
        self.oscillator.frequency().set_value(freq);
        self.freq = freq;
        self
    }

    pub fn gain(mut self, gain: f32) -> Self {
        self.gain_node.gain().set_value(gain);
        self.gain = gain;
        self
    }

    pub fn shape(mut self, shape: OscillatorType) -> Self {
        self.oscillator.set_type(shape);
        self.shape = shape;
        self
    }

    pub fn play(&self) {
        self.context.resume().unwrap(); // Fix for Chromium
        self.gain_node
            .gain()
            .set_target_at_time(self.gain, self.context.current_time(), 0.015)
            .unwrap();
    }

    pub fn pause(&self) {
        self.gain_node
            .gain()
            .set_target_at_time(0.0, self.context.current_time(), 0.015)
            .unwrap();
    }

}
