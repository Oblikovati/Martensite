//! Reader extensions and printers shared by the TKGeomBase probe; the counterpart of
//! the implementation.
use super::print::Output;
pub use super::print::{Reader, attempt, end, flag, int, real, show};
use martensite_tkmath::gp::{
    Axis1, Axis2, Axis2d, Axis3, Axis22d, Circle2, Circle3, Cone3, Cylinder3, Direction2,
    Direction3, Ellipse2, Ellipse3, Hyperbola2, Hyperbola3, Line2, Line3, Parabola2, Parabola3,
    Plane3, Point2, Point3, Transform2, Transform3, Vector2, Vector3, Xy, Xyz,
};

pub fn xyz(output: &Output, value: Xyz) {
    real(output, value.x());
    real(output, value.y());
    real(output, value.z());
}
pub fn pnt(output: &Output, value: Point3) {
    xyz(output, value.xyz());
}
pub fn vec(output: &Output, value: Vector3) {
    xyz(output, *value.xyz());
}
pub fn dir(output: &Output, value: Direction3) {
    xyz(output, value.xyz());
}
pub fn ax1(output: &Output, value: Axis1) {
    pnt(output, value.location());
    dir(output, value.direction());
}
pub fn ax2(output: &Output, value: Axis2) {
    pnt(output, value.location());
    dir(output, value.direction());
    dir(output, value.x_direction());
    dir(output, value.y_direction());
}
pub fn ax3(output: &Output, value: Axis3) {
    pnt(output, value.location());
    dir(output, value.direction());
    dir(output, value.x_direction());
    dir(output, value.y_direction());
}
pub fn xy(output: &Output, value: Xy) {
    real(output, value.x());
    real(output, value.y());
}
pub fn pnt2d(output: &Output, value: Point2) {
    xy(output, value.xy());
}
pub fn vec2d(output: &Output, value: Vector2) {
    xy(output, value.xy());
}
pub fn dir2d(output: &Output, value: Direction2) {
    xy(output, value.xy());
}
pub fn ax2d(output: &Output, value: Axis2d) {
    pnt2d(output, value.location());
    dir2d(output, value.direction());
}
pub fn ax22d(output: &Output, value: Axis22d) {
    pnt2d(output, value.location());
    dir2d(output, value.x_direction());
    dir2d(output, value.y_direction());
}

/// The GbReader members of the implementation.
impl Reader<'_> {
    pub fn xyz(&mut self) -> Xyz {
        let (x, y) = (self.real(), self.real());
        Xyz::new(x, y, self.real())
    }
    pub fn pnt(&mut self) -> Point3 {
        let (x, y) = (self.real(), self.real());
        Point3::new(x, y, self.real())
    }
    pub fn vec(&mut self) -> Vector3 {
        let (x, y) = (self.real(), self.real());
        Vector3::new(x, y, self.real())
    }
    pub fn dir(&mut self) -> Direction3 {
        let (x, y) = (self.real(), self.real());
        Direction3::new(x, y, self.real()).expect("probe direction")
    }
    pub fn ax1(&mut self) -> Axis1 {
        let p = self.pnt();
        Axis1::new(p, self.dir())
    }
    pub fn ax2(&mut self) -> Axis2 {
        let (p, n) = (self.pnt(), self.dir());
        Axis2::with_x_direction(p, n, self.dir()).expect("probe frame")
    }
    pub fn ax3(&mut self) -> Axis3 {
        let (p, n) = (self.pnt(), self.dir());
        Axis3::new(p, n, self.dir()).expect("probe frame")
    }
    pub fn xy(&mut self) -> Xy {
        let x = self.real();
        Xy::new(x, self.real())
    }
    pub fn pnt2d(&mut self) -> Point2 {
        let x = self.real();
        Point2::new(x, self.real())
    }
    pub fn vec2d(&mut self) -> Vector2 {
        let x = self.real();
        Vector2::new(x, self.real())
    }
    pub fn dir2d(&mut self) -> Direction2 {
        let x = self.real();
        Direction2::new(x, self.real()).expect("probe direction")
    }
    pub fn ax2d(&mut self) -> Axis2d {
        let p = self.pnt2d();
        Axis2d::new(p, self.dir2d())
    }
    pub fn ax22d(&mut self) -> Axis22d {
        let (p, x) = (self.pnt2d(), self.dir2d());
        Axis22d::with_directions(p, x, self.dir2d()).expect("probe frame")
    }
}

/// The gp results of the construction algorithms (the gb_lin ... gb_trsf2d printers).
pub fn lin(output: &Output, value: Line3) {
    ax1(output, value.position());
}
pub fn lin2d(output: &Output, value: Line2) {
    ax2d(output, value.position());
}
pub fn circ(output: &Output, value: Circle3) {
    ax2(output, value.position());
    real(output, value.radius());
}
pub fn circ2d(output: &Output, value: Circle2) {
    ax22d(output, value.position());
    real(output, value.radius());
}
pub fn elips(output: &Output, value: Ellipse3) {
    ax2(output, value.position());
    real(output, value.major_radius());
    real(output, value.minor_radius());
}
pub fn elips2d(output: &Output, value: Ellipse2) {
    ax22d(output, value.axis());
    real(output, value.major_radius());
    real(output, value.minor_radius());
}
pub fn hypr(output: &Output, value: Hyperbola3) {
    ax2(output, value.position());
    real(output, value.major_radius());
    real(output, value.minor_radius());
}
pub fn hypr2d(output: &Output, value: Hyperbola2) {
    ax22d(output, value.axis());
    real(output, value.major_radius());
    real(output, value.minor_radius());
}
pub fn parab(output: &Output, value: Parabola3) {
    ax2(output, value.position());
    real(output, value.focal());
}
pub fn parab2d(output: &Output, value: Parabola2) {
    ax22d(output, value.axis());
    real(output, value.focal());
}
pub fn pln(output: &Output, value: Plane3) {
    ax3(output, value.position());
}
pub fn cone(output: &Output, value: Cone3) {
    ax3(output, value.position());
    real(output, value.semi_angle());
    real(output, value.ref_radius());
}
pub fn cylinder(output: &Output, value: Cylinder3) {
    ax3(output, value.position());
    real(output, value.radius());
}
pub fn trsf(output: &Output, value: Transform3) {
    int(output, value.form() as i32);
    real(output, value.scale_factor());
    for i in 1..=3 {
        for j in 1..=3 {
            real(output, value.matrix().value(i, j).unwrap());
        }
    }
    xyz(output, value.translation_part());
}
pub fn trsf2d(output: &Output, value: Transform2) {
    int(output, value.form() as i32);
    real(output, value.scale_factor());
    for i in 1..=2 {
        for j in 1..=2 {
            real(output, value.matrix().value(i, j).unwrap());
        }
    }
    xy(output, value.translation_part());
}
