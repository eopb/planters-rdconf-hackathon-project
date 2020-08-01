use crate::Model;
use crate::Sound;

#[derive(Clone, Debug)]
pub enum SoundSchedulerMsg {
    AddSound(f64, usize),
}

#[derive(Clone, Debug)]
pub enum SoundCommand {
    Play,
    Stop,
}

pub fn update(msg: SoundSchedulerMsg, mut model: &mut Model) {
    match msg {
        SoundSchedulerMsg::AddSound(time, row) => {
            model
                .sound_scheduler
                .schedule_sound_at_secs(time, row, SoundCommand::Play)
        }
    }
}

pub struct SoundScheduler {
    pub schedule: Vec<(u64, usize, SoundCommand)>,
}

impl Default for SoundScheduler {
    fn default() -> SoundScheduler {
        SoundScheduler { schedule: vec![] }
    }
}

impl SoundScheduler {
    // insert sound in order
    pub fn schedule_sound_at_secs(&mut self, at_secs: f64, row: usize, cmd: SoundCommand) {
        let timestep = (at_secs * 60.0) as u64;
        self.schedule_sound(timestep, row, cmd);
    }

    pub fn schedule_sound(&mut self, timestep: u64, row: usize, cmd: SoundCommand) {
        let insert = self.schedule.binary_search_by(|s| s.0.cmp(&timestep));

        match insert {
            Ok(idx) => self.schedule.insert(idx, (timestep, row, cmd)),
            Err(idx) => self.schedule.insert(idx, (timestep, row, cmd)),
        }
    }

    pub fn remove_sound(&mut self, timestep: u64, row: usize) {
        let idx = self
            .schedule
            .iter()
            .position(|x| x.0 == timestep && x.1 == row)
            .expect("sound present to remove!");
        self.schedule.remove(idx);
    }
}
