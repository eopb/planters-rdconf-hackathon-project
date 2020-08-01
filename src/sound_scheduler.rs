use crate::Model;
use crate::Sound;

#[derive(Clone, Debug)]
pub enum SoundSchedulerMsg {
    AddSound(f64, Sound),
}

pub fn update(msg: SoundSchedulerMsg, mut model: &mut Model) {
    match msg {
        SoundSchedulerMsg::AddSound(time, sound) => {
            model.sound_scheduler.schedule_sound_at_secs(time, sound)
        }
    }
}

#[derive(Default)]
pub struct SoundScheduler {
    pub schedule: Vec<(u64, Sound)>,
}

impl SoundScheduler {
    // insert sound in order
    pub fn schedule_sound_at_secs(&mut self, at_secs: f64, sound: Sound) {
        let timestep = (at_secs * 60.0) as u64;
        self.schedule_sound(timestep, sound);
    }

    pub fn schedule_sound(&mut self, timestep: u64, sound: Sound) {
        let insert = self.schedule.binary_search_by(|s| s.0.cmp(&timestep));

        match insert {
            Ok(idx) => self.schedule.insert(idx, (timestep, sound)),
            Err(idx) => self.schedule.insert(idx, (timestep, sound)),
        }
    }
}
