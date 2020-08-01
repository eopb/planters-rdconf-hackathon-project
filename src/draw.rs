//! stuff for drawing on canvas

use web_sys::CanvasRenderingContext2d;

pub struct Rect {
    /// Position from top left corner.
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

impl Rect {
    pub fn from_center((x, y): (f64, f64), width: f64, height: f64) -> Self {
        Self {
            x: x - (width / 2.),
            y: y - (height / 2.),
            width,
            height,
        }
    }
    pub fn crosshair(center: (f64, f64)) -> Vec<Self> {
        vec![
            Self::from_center(center, 1., 19.),
            Self::from_center(center, 19., 1.),
        ]
    }
}

impl Draw for Rect {
    fn draw(self, ctx: &CanvasRenderingContext2d) {
        ctx.fill_rect(self.x, self.y, self.width, self.height);
    }
}
impl Draw for Vec<Rect> {
    fn draw(self, ctx: &CanvasRenderingContext2d) {
        for rect in self {
            ctx.fill_rect(rect.x, rect.y, rect.width, rect.height);
        }
    }
}
pub trait Draw {
    fn draw(self, ctx: &CanvasRenderingContext2d);
}
