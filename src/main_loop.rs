use crate::sound_scheduler::SoundCommand;
use crate::Model;
use seed::*;

// occurs deterministically every timestep
pub fn time_step_advanced(model: &mut Model) {
    // log!(format!("time_step:{}", model.current_time_step));
    // log!(format!("elapsed_time:{}", model.secs_elapsed()));
    advance_time_step_counter(model);
    register_new_sounds(model);
    trigger_scheduled_sounds(model);
    remove_expired_sounds(model);
    update_view_data(model);
}

pub fn time_step_loop_stopped(model: &mut Model) {
    model.current_time_step = 0;
    for (ts, row, _) in model.sound_scheduler.schedule.iter_mut() {
        let sound = &model
            .rows
            .get(*row)
            .expect("row to  be present")
            .sound
            .as_ref()
            .expect("sound to be prseent");
        sound.pause()
    }
}

pub fn time_step_loop_started(model: &mut Model) {
    model.current_time_step = 0;
}

fn advance_time_step_counter(model: &mut Model) {
    model.current_time_step += 1;
}

fn register_new_sounds(_model: &mut Model) {}

fn trigger_scheduled_sounds(model: &mut Model) {
    for (ts, row, cmd) in model.sound_scheduler.schedule.iter_mut() {
        if *ts == model.current_time_step {
            let mut sound = model
                .rows
                .get_mut(*row)
                .expect("row to be present")
                .sound
                .as_mut()
                .expect("sound to be prseent");
            match cmd {
                SoundCommand::Play => sound.play(),
                SoundCommand::Stop => sound.pause(),
            }
        }
    }
}

fn remove_expired_sounds(_model: &mut Model) {}

fn update_view_data(_model: &mut Model) {}
