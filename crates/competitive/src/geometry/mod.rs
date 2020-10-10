mod circle;
mod closest_pair;
mod line;
mod polygon;

pub use circle::Circle;
pub use closest_pair::closest_pair;
pub use line::*;
pub use polygon::*;

use crate::num::Complex;

#[snippet::entry("geometry", include("Real", "CCW", "Complex", "TotalOrd"))]
pub type Point = Complex<f64>;

#[snippet::entry("EPS")]
pub const EPS: f64 = 1e-8;
#[snippet::entry("Real", include("EPS"))]
#[derive(Clone, Debug)]
pub struct Real(pub f64);
#[snippet::entry("Real")]
impl PartialEq for Real {
    fn eq(&self, other: &Real) -> bool {
        (self.0 - other.0).abs() < EPS
    }
}
#[snippet::entry("Real")]
impl PartialOrd for Real {
    fn partial_cmp(&self, other: &Real) -> Option<std::cmp::Ordering> {
        if self == other {
            Some(std::cmp::Ordering::Equal)
        } else {
            self.0.partial_cmp(&other.0)
        }
    }
}

#[snippet::entry("CCW")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum CCW {
    /// a--b--c
    OnlineFront = -2,
    /// a--b-vc
    Clockwise = -1,
    /// a--c--b
    OnSegment = 0,
    /// a--b-^c
    CounterClockwise = 1,
    /// c--a--b
    OnlineBack = 2,
}
#[snippet::entry("CCW")]
pub fn ccw(a: Point, b: Point, c: Point) -> CCW {
    let x = b - a;
    let y = c - a;
    if Real(x.cross(y)) > Real(0.) {
        CCW::CounterClockwise
    } else if Real(x.cross(y)) < Real(0.) {
        CCW::Clockwise
    } else if Real(x.dot(y)) < Real(0.) {
        CCW::OnlineBack
    } else if Real(x.abs()) < Real(y.abs()) {
        CCW::OnlineFront
    } else {
        CCW::OnSegment
    }
}
