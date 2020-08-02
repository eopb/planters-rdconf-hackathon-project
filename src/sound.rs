use seed::prelude::JsValue;
use seed::{prelude::*, *};
use web_sys::OscillatorType;
use web_sys::{AudioContext, GainNode, OscillatorNode};
#[derive(Debug, Clone, PartialEq)]
pub enum SoundStatus {
    Unplayed,
    Played,
}

impl Default for SoundStatus {
    fn default() -> Self {
        SoundStatus::Unplayed
    }
}

#[derive(Clone, Debug)]
pub struct Sound {
    oscillator: OscillatorNode,
    gain_node: GainNode,
    context: AudioContext,
    gain: f32,
    freq: f32,
    pub status: SoundStatus,
    shape: OscillatorType,
}

impl Default for Sound {
    fn default() -> Self {
        Self::build().unwrap()
    }
}

impl Sound {
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
        oscillator.start()?;
        let status = SoundStatus::Unplayed;
        Ok(Self {
            oscillator,
            gain_node,
            context,
            gain,
            freq,
            status,
            shape,
        })
    }

    pub fn freq(mut self, freq: f32) -> Self {
        self.oscillator.frequency().set_value(freq);
        self.freq = freq;
        self
    }

    pub fn gain(mut self, gain: f32) -> Self {
        log!("pooka");
        log!(self.status);

        if self.status == SoundStatus::Played {
            log!("ticka");
            self.gain_node
                .gain()
                .set_target_at_time(self.gain, self.context.current_time(), 0.1)
                .unwrap();
        }
        self.gain = gain;
        self
    }

    pub fn shape(mut self, shape: OscillatorType) -> Self {
        self.oscillator.set_type(shape);
        self.shape = shape;
        self
    }

    pub fn play(&mut self, spookiness: f64) {
        self.status = SoundStatus::Played;
        self.context.resume().unwrap(); // Fix for Chromium
        self.gain_node
            .gain()
            .set_target_at_time(self.gain, self.context.current_time(), spookiness)
            .unwrap();
    }

    pub fn pause(&mut self, spookiness: f64) {
        self.status = SoundStatus::Unplayed;
        self.gain_node
            .gain()
            .set_target_at_time(0.0, self.context.current_time(), spookiness)
            .unwrap();
    }
}
