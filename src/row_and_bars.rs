use crate::rhythm::{Beat, Rhythm};
use crate::sound::Sound;
use crate::Model;

#[derive(Clone)]
pub struct Row {
    pub row: usize,
    pub sound: Sound,
    pub bars: Vec<Bar>,
}

impl Row {
    pub fn new(row_idx: usize) -> Row {
        Row {
            row: row_idx,
            sound: Sound::default(),
            bars: (0..=47).map(|idx| Bar::new(row_idx, idx)).collect(),
        }
    }
}
#[derive(Clone)]
pub struct Bar {
    pub row: usize,
    pub pos: usize,
    pub on: bool,
}

impl Bar {
    pub fn new(row: usize, pos: usize) -> Bar {
        Bar {
            row,
            pos,
            on: false,
        }
    }
}

pub fn init_rows_in_model(
    vec_of_rows: &mut Vec<Row>,
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
                (None, beat) => {
                    if beat.is_playing() {
                        vec_of_rows
                            .get_mut(row_idx)
                            .unwrap()
                            .bars
                            .get_mut(pos)
                            .unwrap()
                            .on = true;
                    }
                }
                (Some(last), beat) => match (last.is_playing(), beat.is_playing()) {
                    (false, false) => {}
                    (false, true) => {
                        if beat.is_playing() {
                            vec_of_rows
                                .get_mut(row_idx)
                                .unwrap()
                                .bars
                                .get_mut(pos)
                                .unwrap()
                                .on = true;
                        }
                    }
                    (true, false) => {}
                    (true, true) => {
                        if beat.is_playing() {
                            vec_of_rows
                                .get_mut(row_idx)
                                .unwrap()
                                .bars
                                .get_mut(pos)
                                .unwrap()
                                .on = true;
                        }
                    }
                },
            }
            last_beat = Some(beat);
        }
        last_beat = None;
    }
}
