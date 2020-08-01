//! Types for creating repeating rhythms.

/// A rhythm is a collection off notes and pauses to make a pattern.
struct Rhythm([Beat; 48]);

#[derive(Copy, Clone, Eq, PartialEq)]
enum Beat {
    Play,
    Pause,
}

impl Beat {
    fn is_playing(self) -> bool {
        self == Self::Play
    }
    fn is_paused(self) -> bool {
        self == Self::Pause
    }
}

impl Rhythm {
    /// The set of beats we want to offer the user.
    fn standard() -> Vec<Self> {
        vec![Self([Beat::Play; 48])]
    }
    /// Draw a visual rep of a rhythm to a canvas using dots dashes and breaks.
    fn draw(self) {}
}
