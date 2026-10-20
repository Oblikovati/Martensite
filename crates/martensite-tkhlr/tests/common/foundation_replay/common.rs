//! Request-local fields and tokens over public geometry interfaces.
pub use super::reader::Reader;
pub use super::sink::{Sink, attempt, captured, end, flag, int, real, show};
use martensite_tkmath::gp::{Direction3, Point2, Point3, Xy, Xyz};
impl Reader<'_> {
    pub fn dir(&mut self) -> Direction3 {
        let (x, y) = (self.real(), self.real());
        Direction3::new(x, y, self.real()).expect("probe direction")
    }
}

pub fn xyz(tokens: &mut Sink, value: Xyz) {
    real(tokens, value.x());
    real(tokens, value.y());
    real(tokens, value.z());
}
pub fn xy(tokens: &mut Sink, value: Xy) {
    real(tokens, value.x());
    real(tokens, value.y());
}
pub fn pnt(tokens: &mut Sink, value: Point3) {
    xyz(tokens, value.xyz());
}
pub fn pnt2d(tokens: &mut Sink, value: Point2) {
    xy(tokens, value.xy());
}
pub fn dir(tokens: &mut Sink, value: Direction3) {
    xyz(tokens, value.xyz());
}
impl Reader<'_> {
    pub fn xyz(&mut self) -> Xyz {
        let (x, y) = (self.real(), self.real());
        Xyz::new(x, y, self.real())
    }
    pub fn pnt(&mut self) -> Point3 {
        let (x, y) = (self.real(), self.real());
        Point3::new(x, y, self.real())
    }
    pub fn pnt2d(&mut self) -> Point2 {
        let x = self.real();
        Point2::new(x, self.real())
    }
}
