use crate::Model;

#[derive(Clone,Debug)]
pub enum SoundSchedulerMsg {
    AddSound(f64,String),
}

pub fn update(msg: SoundSchedulerMsg, mut model: &mut Model) {
    match msg {
        SoundSchedulerMsg::AddSound(time, name) => model.sound_scheduler.schedule_sound_at_secs(time, Sound::new(&name))
    }

}

#[derive(Default)]
pub struct SoundScheduler{
    pub schedule: Vec< (u64, Sound)>
}

//placeholder for real sound struct
pub struct Sound{
    pub data: String,
    pub played: bool,
}

impl Sound {
    fn new(name:&str)->Self {
        Self {
            data:name.to_string(),
            played: false,
        }
    }
}


impl SoundScheduler {
    // insert sound in order
    pub fn schedule_sound_at_secs(&mut self, at_secs: f64, sound: Sound){
        let timestep = (at_secs * 60.0) as u64;
        self.schedule_sound(timestep, sound);
    }

    pub fn schedule_sound(&mut self, timestep: u64, sound: Sound){
        let insert = self.schedule.binary_search_by(|s| s.0.cmp( &timestep ));

        match insert {
            Ok(idx) => self.schedule.insert(idx, (timestep,sound)),
            Err(idx) => self.schedule.insert(idx, (timestep,sound)),
        }
    }
}
