// the implementation and the implementation at

//! The type codes that open each compact record. The 3D and 2D curve files define
//! the same values.

/// The codes of `Curve` and `Curve` records.
pub(crate) mod curve {
    pub(crate) const LINE: i32 = 1;
    pub(crate) const CIRCLE: i32 = 2;
    pub(crate) const ELLIPSE: i32 = 3;
    pub(crate) const PARABOLA: i32 = 4;
    pub(crate) const HYPERBOLA: i32 = 5;
    pub(crate) const BEZIER: i32 = 6;
    pub(crate) const BSPLINE: i32 = 7;
    pub(crate) const TRIMMED: i32 = 8;
    pub(crate) const OFFSET: i32 = 9;
}

/// The codes of `Surface` records.
pub(crate) mod surface {
    pub(crate) const PLANE: i32 = 1;
    pub(crate) const CYLINDER: i32 = 2;
    pub(crate) const CONE: i32 = 3;
    pub(crate) const SPHERE: i32 = 4;
    pub(crate) const TORUS: i32 = 5;
    pub(crate) const LINEAR_EXTRUSION: i32 = 6;
    pub(crate) const REVOLUTION: i32 = 7;
    pub(crate) const BEZIER: i32 = 8;
    pub(crate) const BSPLINE: i32 = 9;
    pub(crate) const RECTANGULAR: i32 = 10;
    pub(crate) const OFFSET: i32 = 11;
}
