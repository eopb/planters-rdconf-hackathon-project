use crate::sound_scheduler::SoundCommand;
use crate::Model;
use crate::Msg;
use seed::app::Orders;
use seed::*;

// occurs deterministically every timestep
pub fn time_step_advanced(model: &mut Model) {
    // log!(format!("time_step:{}", model.current_time_step));
    // log!(format!("elapsed_time:{}", model.secs_elapsed()));
    advance_time_step_counter(model);
    // register_new_sounds(model);
    trigger_scheduled_sounds(model);
    // remove_expired_sounds(model);
    update_view_data(model);
}

pub fn time_step_loop_stopped(model: &mut Model) {
    model.current_time_step = 0;
    for row in &model.rows {
        row.sound.pause(model.spookiness)
    }
}

pub fn time_step_loop_started(model: &mut Model) {
    model.current_time_step = 0;
}

fn advance_time_step_counter(model: &mut Model) {
    model.current_time_step += 1;
}

fn trigger_scheduled_sounds(model: &mut Model) {
    for (ts, row, index, cmd) in model.sound_scheduler.schedule.iter_mut() {
        if *ts == model.current_time_step {
            if let Some(row_data) = model.rows.get(*row) {
                log(cmd.clone());

                match cmd {
                    SoundCommand::Play => {
                        *model
                            .currently_playing
                            .get_mut(*row)
                            .unwrap()
                            .get_mut(*index)
                            .unwrap() = true
                    }
                    SoundCommand::Stop => (model.currently_playing) = vec![vec![false; 48]; 4],
                }
                match cmd {
                    SoundCommand::Play => row_data.sound.play(model.spookiness),
                    SoundCommand::Stop => row_data.sound.pause(model.spookiness),
                }
            }
        }
    }
}

// fn register_new_sounds(_model: &mut Model) {}
// fn remove_expired_sounds(_model: &mut Model) {}

fn update_view_data(_model: &mut Model) {}
