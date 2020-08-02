use crate::rhythm::{Beat, Rhythm};
use crate::Model;
use crate::Sound;
use seed::{prelude::*, *};

#[derive(Clone, Debug)]
pub enum SoundSchedulerMsg {
    AddSound(f64, usize, usize),
}

#[derive(Clone, Debug)]
pub enum SoundCommand {
    Play,
    Stop,
}

pub fn update(msg: SoundSchedulerMsg, mut model: &mut Model) {
    match msg {
        SoundSchedulerMsg::AddSound(time, row, index) => model
            .sound_scheduler
            .schedule_sound_at_secs(time, row, index, SoundCommand::Play),
    }
}

#[derive(Clone)]
pub struct SoundScheduler {
    pub schedule: Vec<(u64, usize, usize, SoundCommand)>,
}

impl Default for SoundScheduler {
    fn default() -> SoundScheduler {
        SoundScheduler { schedule: vec![] }
    }
}

impl SoundScheduler {
    // insert sound in order

    pub fn init_with_rhythm(
        &mut self,
        ticks_in_one_bar: u64,
        rs: &Vec<(
            Rhythm,
            // Where the rhythm is attached to the canvas
            Option<(f64, f64)>,
        )>,
    ) {
        let mut last_beat: Option<&Beat> = None;
        for (row_idx, (r, _)) in rs.iter().enumerate() {
            for (pos, beat) in r.0.iter().enumerate() {
                match (last_beat, beat) {
                    (None, this_beat) => {
                        if this_beat.is_playing() {
                            // log!(row_idx);
                            // log!(pos);
                            self.schedule_sound(0, row_idx, pos, SoundCommand::Play);
                        }
                    }
                    (Some(last), beat) => match (last.is_playing(), beat.is_playing()) {
                        (false, false) => {}
                        (false, true) => {
                            self.schedule_sound(
                                ticks_in_one_bar * (pos as u64),
                                row_idx,
                                pos,
                                SoundCommand::Play,
                            );
                        }
                        (true, false) => self.schedule_sound(
                            ticks_in_one_bar * (pos as u64),
                            row_idx,
                            pos,
                            SoundCommand::Stop,
                        ),
                        (true, true) => {}
                    },
                }
                last_beat = Some(beat);
            }
            last_beat = None;
        }
    }

    pub fn schedule_sound_at_secs(
        &mut self,
        at_secs: f64,
        row: usize,
        index: usize,
        cmd: SoundCommand,
    ) {
        let timestep = (at_secs * 60.0) as u64;
        self.schedule_sound(timestep, row, index, cmd);
    }

    pub fn schedule_sound(&mut self, timestep: u64, row: usize, index: usize, cmd: SoundCommand) {
        let insert = self.schedule.binary_search_by(|s| s.0.cmp(&timestep));

        match insert {
            Ok(idx) => self.schedule.insert(idx, (timestep, row, index, cmd)),
            Err(idx) => self.schedule.insert(idx, (timestep, row, index, cmd)),
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
