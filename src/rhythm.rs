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
    /// Draw a visual rep of a rhythm to a canvas using dots dashes and breaks.
    fn draw(self) {}
}
