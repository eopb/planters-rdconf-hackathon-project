//! Types for creating repeating rhythms.

/// A rhythm is a collection off notes and pauses to make a pattern.
struct Rhythm {
    inner: Vec<Beat>,
}

#[derive(Copy, Clone)]
enum Beat {
    Play(f64),
    Pause(f64),
}

impl Beat {
    fn len(self) -> f64 {
        match self {
            Self::Play(x) => x,
            Self::Pause(x) => x,
        }
    }
}

impl Rhythm {
    fn new(inner: Vec<Beat>) -> Self {
        Self { inner }
    }
    /// The set of beats we want to offer the user.
    fn standard() -> Vec<Self> {
        vec![
            Self::new(vec![Beat::Play(1.), Beat::Pause(1.)]),
            Self::new(vec![Beat::Play(1.), Beat::Pause(1.)]),
            Self::new(vec![Beat::Play(1.), Beat::Pause(1.)]),
            Self::new(vec![Beat::Play(1.), Beat::Pause(1.)]),
            Self::new(vec![Beat::Play(1.), Beat::Pause(1.)]),
            Self::new(vec![Beat::Play(1.), Beat::Pause(1.)]),
        ]
    }
    /// Draw a visual rep of a rhythm to a canvas using dots dashes and breaks.
    fn draw(self) {
        let len: f64 = self.inner.into_iter().map(Beat::len).sum();
    }
}
