//! `IgesConvGeomGeomBuilder`: records points and a position, and builds the IGES entities they define.
use martensite_tkernel::standard::Failure;
use martensite_tkernel::standard::exceptions;
use martensite_tkmath::gp::{Axis1, Axis2, Axis3, Transform3, TransformForm, Xyz};

/// Length tolerance of the identity and translation answers.
const LENGTH_EPSILON: f64 = 1.0e-10;
/// Coefficient tolerance of the identity and translation answers.
const ANGLE_EPSILON: f64 = 1.0e-10;

/// Collects points with their vector parts, and the position they are evaluated in.
///
/// ```
/// use martensite_tkdeiges::iges_conv_geom::IgesConvGeomGeomBuilder;
/// use martensite_tkmath::gp::Xyz;
/// let mut builder = IgesConvGeomGeomBuilder::new();
/// builder.add_xyz(Xyz::new(1.0, 2.0, 3.0));
/// assert_eq!(builder.nb_points(), 1);
/// assert!(builder.is_identity());
/// ```
#[derive(Clone, Debug)]
pub struct IgesConvGeomGeomBuilder {
    pub(super) points: Vec<Xyz>,
    pub(super) vectors: Vec<Xyz>,
    pub(super) position: Transform3,
}

impl Default for IgesConvGeomGeomBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl IgesConvGeomGeomBuilder {
    /// A builder without points at the identity position; `IgesConvGeomGeomBuilder::new()`.
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
            vectors: Vec::new(),
            position: Transform3::default(),
        }
    }
    /// Forgets the points and the position; `builder.clear()`.
    pub fn clear(&mut self) {
        *self = Self::new();
    }
    /// Adds a point with Z = 0 and a null vector part; `builder.add_xy(1.0, 2.0)`.
    pub fn add_xy(&mut self, x: f64, y: f64) {
        self.add_xyz(Xyz::new(x, y, 0.0));
    }
    /// Adds a point with a null vector part; `builder.add_xyz(point)`.
    pub fn add_xyz(&mut self, point: Xyz) {
        self.points.push(point);
        self.vectors.push(Xyz::new(0.0, 0.0, 0.0));
    }
    /// Sets the vector part of the last point, nothing without a point; `builder.add_vec(vector)`.
    pub fn add_vec(&mut self, vector: Xyz) {
        if let Some(last) = self.vectors.last_mut() {
            *last = vector;
        }
    }
    /// The count of recorded points; `builder.nb_points()`.
    pub fn nb_points(&self) -> i32 {
        self.points.len() as i32
    }
    /// A recorded point by its one-based rank; `builder.point(1)?`.
    pub fn point(&self, rank: i32) -> Result<Xyz, Failure> {
        let found = usize::try_from(rank - 1)
            .ok()
            .and_then(|index| self.points.get(index));
        found.copied().ok_or_else(|| {
            exceptions::out_of_range(&format!(
                "IgesConvGeomGeomBuilder::point: rank {rank}, expected 1..={}",
                self.points.len()
            ))
        })
    }
    /// The position points are evaluated in; `builder.position()`.
    pub fn position(&self) -> Transform3 {
        self.position
    }
    /// Sets the position; `builder.set_position(transform)`.
    pub fn set_position(&mut self, position: Transform3) {
        self.position = position;
    }
    /// Sets the position from a local system; `builder.set_position_axis3(frame)`.
    pub fn set_position_axis3(&mut self, frame: Axis3) {
        self.position = Transform3::transformation_between(frame, Axis3::default());
    }
    /// Sets the position from a right-handed system; `builder.set_position_axis2(frame)`.
    pub fn set_position_axis2(&mut self, frame: Axis2) {
        self.set_position_axis3(frame.axis3());
    }
    /// Sets the position from an origin and a Z axis, the other axes being arbitrary;
    /// `builder.set_position_axis1(axis)?`.
    pub fn set_position_axis1(&mut self, axis: Axis1) -> Result<(), Failure> {
        self.set_position_axis3(Axis3::from_normal(axis.location(), axis.direction())?);
        Ok(())
    }
    /// True when the position is the identity; `builder.is_identity()`.
    pub fn is_identity(&self) -> bool {
        if self.position.form() == TransformForm::Identity {
            return true;
        }
        self.is_translation()
            && self
                .position
                .translation_part()
                .is_equal(&Xyz::new(0.0, 0.0, 0.0), LENGTH_EPSILON)
    }
    /// True when the position only translates; the identity and Z-only answer true;
    /// `builder.is_translation()`.
    pub fn is_translation(&self) -> bool {
        let form = self.position.form();
        if form == TransformForm::Identity || form == TransformForm::Translation {
            return true;
        }
        (1..=3).all(|row| (1..=3).all(|column| self.is_unit_coefficient(row, column)))
    }
    fn is_unit_coefficient(&self, row: i32, column: i32) -> bool {
        let expected = if row == column { 1.0 } else { 0.0 };
        self.position.value(row, column).is_ok_and(|value| {
            value <= expected + ANGLE_EPSILON && value >= expected - ANGLE_EPSILON
        })
    }
    /// True when the position only moves along Z; the identity answers true;
    /// `builder.is_z_only()`.
    pub fn is_z_only(&self) -> bool {
        if !self.is_translation() {
            return false;
        }
        let mut translation = self.position.translation_part();
        translation.set_z(0.0);
        translation.is_equal(&Xyz::new(0.0, 0.0, 0.0), LENGTH_EPSILON)
    }
    /// Evaluates coordinates in the position; for a planar definition Z is the Z displacement;
    /// `let local = builder.eval_xyz(point)?;`.
    pub fn eval_xyz(&self, point: Xyz) -> Result<Xyz, Failure> {
        Ok(self.position.inverted()?.apply_coordinates(point))
    }
}
