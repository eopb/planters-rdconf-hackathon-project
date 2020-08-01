//! Types for creating repeating rhythms.

/// A rhythm is a collection off notes and pauses to make a pattern.
#[derive(Copy, Clone)]
pub struct Rhythm(pub [Beat; 48]);

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Beat {
    Play,
    Pause,
}

impl Beat {
    pub fn is_playing(self) -> bool {
        self == Self::Play
    }
    pub fn is_paused(self) -> bool {
        self == Self::Pause
    }
    pub fn toggle(self) -> Self {
        match self {
            Self::Play => Self::Pause,
            Self::Pause => Self::Play,
        }
    }
}

impl Rhythm {
    /// The set of beats we want to offer the user.
    pub fn standard() -> [Self; 4] {
        [Self([Beat::Play; 48]); 4]
    }
    pub fn neighbours(self) -> Vec<Neighbours> {
        let mut beats = self.0.iter().peekable();
        let mut neighbours = Vec::new();

        let mut beat_before = false;
        while let Some(beat) = beats.next() {
            neighbours.push(Neighbours {
                left: beat_before,
                right: beats.peek().map(|beat| beat.is_playing()).unwrap_or(false),
            });
            beat_before = beat.is_playing();
        }

        neighbours
    }
}

#[derive(Debug)]
pub struct Neighbours {
    pub left: bool,
    pub right: bool,
}
