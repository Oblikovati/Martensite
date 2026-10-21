//! Explicit replay adapters over public geometry and retained development support.
pub use super::reader::Reader;
pub use super::sink::{Sink, attempt, end, flag, int, real};
use martensite_tkmath::gp::{Direction3, Point3, Xyz};

pub fn xyz(out: &mut Sink, value: Xyz) {
    real(out, value.x());
    real(out, value.y());
    real(out, value.z());
}
pub fn pnt(out: &mut Sink, value: Point3) {
    xyz(out, value.xyz());
}
pub fn dir(out: &mut Sink, value: Direction3) {
    xyz(out, value.xyz());
}
impl Reader<'_> {
    pub fn pnt(&mut self) -> Point3 {
        let (x, y) = (self.real(), self.real());
        Point3::new(x, y, self.real())
    }
    pub fn dir(&mut self) -> Direction3 {
        let (x, y) = (self.real(), self.real());
        Direction3::new(x, y, self.real()).expect("probe direction")
    }
}
