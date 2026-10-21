//! Reader extensions and printers shared by the geometry corpus families.
use super::print::Output;
pub use super::print::{Reader, attempt, end, flag, int, real, show};
use martensite_tkernel::standard::Failure;
use martensite_tkmath::gp::{Direction2, Direction3, Point2, Point3, Xy, Xyz};

pub fn xyz(output: &Output, value: Xyz) {
    real(output, value.x());
    real(output, value.y());
    real(output, value.z());
}
pub fn pnt(output: &Output, value: Point3) {
    xyz(output, value.xyz());
}
pub fn dir(output: &Output, value: Direction3) {
    xyz(output, value.xyz());
}
pub fn xy(output: &Output, value: Xy) {
    real(output, value.x());
    real(output, value.y());
}
pub fn pnt2d(output: &Output, value: Point2) {
    xy(output, value.xy());
}

/// `ga_text`: a text as one `t<hex bytes>` token.
pub fn text(output: &Output, bytes: &[u8]) {
    let hex: String = bytes.iter().map(|byte| format!("{byte:02x}")).collect();
    output.write(format_args!("t{hex} "));
}
/// `ga_captured`: the text a member wrote, then its exception type, if any.
pub fn captured(output: &Output, body: impl FnOnce(&mut String) -> Result<(), Failure>) {
    let mut console = String::new();
    let result = body(&mut console);
    text(output, console.as_bytes());
    if let Err(error) = result {
        output.write(format_args!("{} ", error.exception_type()));
    }
}

/// The reader members shared by the corpus families.
impl Reader<'_> {
    pub fn xyz(&mut self) -> Xyz {
        let (x, y) = (self.real(), self.real());
        Xyz::new(x, y, self.real())
    }
    pub fn pnt(&mut self) -> Point3 {
        let (x, y) = (self.real(), self.real());
        Point3::new(x, y, self.real())
    }
    pub fn dir(&mut self) -> Direction3 {
        let (x, y) = (self.real(), self.real());
        Direction3::new(x, y, self.real()).expect("corpus direction")
    }
    pub fn dir2d(&mut self) -> Direction2 {
        let x = self.real();
        Direction2::new(x, self.real()).expect("corpus direction")
    }
    pub fn pnt2d(&mut self) -> Point2 {
        let x = self.real();
        Point2::new(x, self.real())
    }
}
