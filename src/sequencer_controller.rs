use crate::sound_scheduler::SoundCommand;
use crate::{Model, TICKS_IN_ONE_BAR};

pub fn bar_toggled(model: &mut Model, row_idx: usize, pos_idx: usize) {
    let row = model.rows.get_mut(row_idx).expect("row should be present");
    // let sound = &row.sound;

    let bars_idxs = match pos_idx {
        0 => (None, pos_idx, Some(pos_idx + 1)),
        47 => (Some(pos_idx - 1), pos_idx, None),
        _ => ((Some(pos_idx - 1)), pos_idx, Some(pos_idx + 1)),
    };

    match bars_idxs {
        (None, current, Some(next)) => match (
            row.bars.get(current).unwrap().on,
            row.bars.get(next).unwrap().on,
        ) {
            (false, false) => {
                row.bars.get_mut(current).unwrap().on = true;
                model
                    .sound_scheduler
                    .schedule_sound(0, row_idx, SoundCommand::Play);
                model
                    .sound_scheduler
                    .schedule_sound(TICKS_IN_ONE_BAR, row_idx, SoundCommand::Stop);
            }
            (false, true) => {
                row.bars.get_mut(current).unwrap().on = true;
                model
                    .sound_scheduler
                    .schedule_sound(0, row_idx, SoundCommand::Play);
                model
                    .sound_scheduler
                    .remove_sound(TICKS_IN_ONE_BAR, row_idx); //removing the existing play at start of next bar
            }
            (true, false) => {
                row.bars.get_mut(current).unwrap().on = false;
                model.sound_scheduler.remove_sound(0, row_idx); // remove eixsting play at start of 0th bar
                model
                    .sound_scheduler
                    .remove_sound(TICKS_IN_ONE_BAR, row_idx); // remove eixsting stop at start of 1st bar
            }
            (true, true) => {
                row.bars.get_mut(current).unwrap().on = false;
                model.sound_scheduler.remove_sound(0, row_idx); // remove eixsting play

                model
                    .sound_scheduler
                    .schedule_sound(TICKS_IN_ONE_BAR, row_idx, SoundCommand::Play)
            }
        },
        (Some(prev), current, Some(next)) => match (
            row.bars.get(prev).unwrap().on,
            row.bars.get(current).unwrap().on,
            row.bars.get(next).unwrap().on,
        ) {
            (false, false, false) => {
                row.bars.get_mut(current).unwrap().on = true;
                model
                    .sound_scheduler
                    .schedule_sound(0, row_idx, SoundCommand::Play);
                model
                    .sound_scheduler
                    .schedule_sound(TICKS_IN_ONE_BAR, row_idx, SoundCommand::Stop);
            }
            (false, false, true) => {
                row.bars.get_mut(current).unwrap().on = true;
                model
                    .sound_scheduler
                    .schedule_sound(0, row_idx, SoundCommand::Play);
                model
                    .sound_scheduler
                    .remove_sound(TICKS_IN_ONE_BAR, row_idx); //removing the existing play at start of next bar
            }
            (false, true, false) => {
                row.bars.get_mut(current).unwrap().on = false;
                model.sound_scheduler.remove_sound(0, row_idx); // remove eixsting play at start of 0th bar
                model
                    .sound_scheduler
                    .remove_sound(TICKS_IN_ONE_BAR, row_idx); // remove eixsting stop at start of 1st bar
            }
            (false, true, true) => {
                row.bars.get_mut(current).unwrap().on = false;
                model.sound_scheduler.remove_sound(0, row_idx); // remove eixsting play

                model
                    .sound_scheduler
                    .schedule_sound(TICKS_IN_ONE_BAR, row_idx, SoundCommand::Play)
            }

            (true, false, false) => {
                row.bars.get_mut(current).unwrap().on = true;
                model
                    .sound_scheduler
                    .remove_sound(TICKS_IN_ONE_BAR * (pos_idx as u64), row_idx); // remove the stop
                model.sound_scheduler.schedule_sound(
                    TICKS_IN_ONE_BAR * (pos_idx as u64 + 1),
                    row_idx,
                    SoundCommand::Stop, // and putit back 1 bar
                );
            }
            (true, false, true) => {
                row.bars.get_mut(current).unwrap().on = true;
                model
                    .sound_scheduler
                    .remove_sound(TICKS_IN_ONE_BAR * (pos_idx as u64), row_idx); // remove the stop
                model
                    .sound_scheduler
                    .remove_sound(TICKS_IN_ONE_BAR * (pos_idx as u64 + 1), row_idx);
                // remove the start
            }
            (true, true, false) => {
                row.bars.get_mut(current).unwrap().on = false;
                model.sound_scheduler.schedule_sound(
                    TICKS_IN_ONE_BAR * (pos_idx as u64),
                    row_idx,
                    SoundCommand::Stop, // insert a stop
                );
                model
                    .sound_scheduler
                    .remove_sound(TICKS_IN_ONE_BAR * (pos_idx as u64 + 1), row_idx);
                // remove he other stop
            }
            (true, true, true) => {
                row.bars.get_mut(current).unwrap().on = false;
                model.sound_scheduler.schedule_sound(
                    TICKS_IN_ONE_BAR * (pos_idx as u64),
                    row_idx,
                    SoundCommand::Stop, // insert a stop
                );

                model.sound_scheduler.schedule_sound(
                    TICKS_IN_ONE_BAR * (pos_idx as u64 + 1),
                    row_idx,
                    SoundCommand::Play, // insert a stop
                );
            }
        },
        (Some(prev), current, None) => match (
            row.bars.get(prev).unwrap().on,
            row.bars.get(current).unwrap().on,
        ) {
            (false, false) => {
                row.bars.get_mut(current).unwrap().on = true;
                model.sound_scheduler.schedule_sound(
                    TICKS_IN_ONE_BAR * (pos_idx as u64),
                    row_idx,
                    SoundCommand::Play,
                );
            }
            (false, true) => {
                row.bars.get_mut(current).unwrap().on = false;
                model
                    .sound_scheduler
                    .remove_sound(TICKS_IN_ONE_BAR * (pos_idx as u64), row_idx);
                // remove the start
            }
            (true, false) => {
                row.bars.get_mut(current).unwrap().on = true;
                model
                    .sound_scheduler
                    .remove_sound(TICKS_IN_ONE_BAR * (pos_idx as u64), row_idx);
                // remove the stop
            }
            (true, true) => {
                row.bars.get_mut(current).unwrap().on = false;
                model.sound_scheduler.schedule_sound(
                    TICKS_IN_ONE_BAR * (pos_idx as u64),
                    row_idx,
                    SoundCommand::Stop,
                );
            }
        },

        (None, _current, None) => panic!("never should be here!"),
    }
}
