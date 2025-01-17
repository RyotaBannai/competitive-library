use super::{Approx, Ccw, Point};

#[derive(Clone, Debug, PartialEq)]
pub struct Line {
    p1: Point,
    p2: Point,
}
impl Line {
    pub fn new(p1: Point, p2: Point) -> Self {
        Line { p1, p2 }
    }
    pub fn dir(&self) -> Point {
        self.p2 - self.p1
    }
    pub fn ccw(&self, p: Point) -> Ccw {
        Ccw::ccw(self.p1, self.p2, p)
    }
    pub fn projection(&self, p: Point) -> Point {
        let e = self.dir().unit();
        self.p1 + e * (p - self.p1).dot(e)
    }
    pub fn reflection(&self, p: Point) -> Point {
        p + (self.projection(p) - p) * 2.0
    }
    pub fn distance_point(&self, p: Point) -> f64 {
        (p / self.dir().unit()).re
    }
    pub fn is_parallel(&self, other: &Self) -> bool {
        Approx(self.dir().cross(other.dir())) == Approx(0.)
    }
    pub fn is_orthogonal(&self, other: &Self) -> bool {
        Approx(self.dir().dot(other.dir())) == Approx(0.)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct LineSegment {
    p1: Point,
    p2: Point,
}
impl LineSegment {
    pub fn new(p1: Point, p2: Point) -> Self {
        LineSegment { p1, p2 }
    }
    pub fn dir(&self) -> Point {
        self.p2 - self.p1
    }
    pub fn ccw(&self, p: Point) -> Ccw {
        Ccw::ccw(self.p1, self.p2, p)
    }
    pub fn projection(&self, p: Point) -> Point {
        let e = self.dir().unit();
        self.p1 + e * (p - self.p1).dot(e)
    }
    pub fn reflection(&self, p: Point) -> Point {
        p + (self.projection(p) - p) * 2.0
    }
    pub fn is_parallel(&self, other: &Self) -> bool {
        Approx(self.dir().cross(other.dir())) == Approx(0.)
    }
    pub fn is_orthogonal(&self, other: &Self) -> bool {
        Approx(self.dir().dot(other.dir())) == Approx(0.)
    }
    pub fn intersect(&self, other: &Self) -> bool {
        self.ccw(other.p1) as i8 * self.ccw(other.p2) as i8 <= 0
            && other.ccw(self.p1) as i8 * other.ccw(self.p2) as i8 <= 0
    }
    pub fn intersect_point(&self, p: Point) -> bool {
        self.ccw(p) == Ccw::OnSegment
    }
    pub fn cross_point(&self, other: &Self) -> Option<Point> {
        if self.intersect(other) {
            let a = self.dir().cross(other.dir());
            let b = self.dir().cross(self.p2 - other.p1);
            if Approx(a.abs()) == Approx(0.) && Approx(b.abs()) == Approx(0.) {
                Some(other.p1)
            } else {
                Some(other.p1 + (other.dir() * b / a))
            }
        } else {
            None
        }
    }
    pub fn distance_point(&self, p: Point) -> f64 {
        let r = self.projection(p);
        if self.intersect_point(r) {
            (r - p).abs()
        } else {
            (self.p1 - p).abs().min((self.p2 - p).abs())
        }
    }
    pub fn distance(&self, other: &Self) -> f64 {
        if self.intersect(other) {
            0.
        } else {
            let d1 = self.distance_point(other.p1);
            let d2 = self.distance_point(other.p2);
            let d3 = other.distance_point(self.p1);
            let d4 = other.distance_point(self.p2);
            d1.min(d2).min(d3).min(d4)
        }
    }
}
