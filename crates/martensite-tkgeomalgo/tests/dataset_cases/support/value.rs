//! The values a case variable holds in the cases: shapes, curves, surfaces, points and
//! numbers, with the `whatis` text of each drawable kind.
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::Handle;
use martensite_tkg2d::geom2d::Geom2dCurve;
use martensite_tkg3d::geom::{GeomCurve, GeomSurface};
use martensite_tkmath::gp::{Point2, Point3};

/// A case variable's value.
#[derive(Clone)]
pub enum Value {
    /// `DBRep_DrawableShape`.
    Shape(Shape),
    /// `DrawTrSurf_Curve` and its B-spline/Bezier subclasses.
    Curve(Handle<dyn GeomCurve>),
    /// `DrawTrSurf_Curve2d` and its subclasses.
    Curve2d(Handle<dyn Geom2dCurve>),
    /// `DrawTrSurf_Surface` and its subclasses.
    Surface(Handle<dyn GeomSurface>),
    /// A 3D `DrawTrSurf_Point`.
    Point(Point3),
    /// A 2D `DrawTrSurf_Point`.
    Point2d(Point2),
    /// `Draw_Number`.
    Number(f64),
}

impl Value {
    /// The `whatis` text of the drawable (`Draw_Drawable3D::Whatis` overrides).
    pub fn whatis(&self) -> String {
        match self {
            Value::Shape(shape) => format!(
                "shape {}",
                format!("{:?}", shape.shape_type()).to_uppercase()
            ),
            Value::Curve(_) => " a 3d curve".into(),
            Value::Curve2d(_) => "2d curve".into(),
            Value::Surface(_) => "a surface".into(),
            Value::Point(_) | Value::Point2d(_) => "point".into(),
            Value::Number(_) => "numeric".into(),
        }
    }

    /// Whether the drawable lives in 3D views (`Draw_Drawable3D::Is3D`).
    pub fn is_3d(&self) -> bool {
        !matches!(self, Value::Curve2d(_) | Value::Point2d(_))
    }

    /// Whether `Script::Set` displays the value (numbers are not drawn).
    pub fn is_displayed(&self) -> bool {
        !matches!(self, Value::Number(_))
    }
}

impl From<Shape> for Value {
    fn from(shape: Shape) -> Self {
        Value::Shape(shape)
    }
}

impl From<Handle<dyn GeomCurve>> for Value {
    fn from(curve: Handle<dyn GeomCurve>) -> Self {
        Value::Curve(curve)
    }
}

impl From<Handle<dyn Geom2dCurve>> for Value {
    fn from(curve: Handle<dyn Geom2dCurve>) -> Self {
        Value::Curve2d(curve)
    }
}

impl From<Handle<dyn GeomSurface>> for Value {
    fn from(surface: Handle<dyn GeomSurface>) -> Self {
        Value::Surface(surface)
    }
}

impl From<Point3> for Value {
    fn from(point: Point3) -> Self {
        Value::Point(point)
    }
}

impl From<Point2> for Value {
    fn from(point: Point2) -> Self {
        Value::Point2d(point)
    }
}

impl From<f64> for Value {
    fn from(number: f64) -> Self {
        Value::Number(number)
    }
}
