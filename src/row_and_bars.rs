use crate::sound::Sound;

pub struct Row {
    pub row: usize,
    pub sound: Option<Sound>,
    pub bars: Vec<Bar>,
}

impl Row {
    pub fn new(row_idx: usize) -> Row {
        Row {
            row: row_idx,
            sound: None,
            bars: (0..=47).map(|idx| Bar::new(row_idx, idx)).collect(),
        }
    }
}
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
