//! Checked rigid coefficient capture without reconstruction (#451).
use super::super::{Axis3, Failure, Matrix3, Point3, Transform3, TransformForm, Xyz};

impl Transform3 {
    /// Retains rigid coefficients admitted by the stored-frame roundoff enclosure (#451).
    /// Refuses scaling, shear and inconclusive frames; does not normalize stored values.
    /// `Transform3::from_stored_rigid_coefficients([[1.,0.,0.,1.],[0.,1.,0.,2.],[0.,0.,1.,3.]])?`.
    pub fn from_stored_rigid_coefficients(rows: [[f64; 4]; 3]) -> Result<Self, Failure> {
        let column = |c: usize| [rows[0][c], rows[1][c], rows[2][c]];
        let origin = Point3::new(rows[0][3], rows[1][3], rows[2][3]);
        let frame = Axis3::from_stored_coefficients(origin, column(2), column(0), column(1))?;
        let scale = if frame.direct()? { 1.0 } else { -1.0 };
        let matrix = Matrix3::new(rows.map(|row| [row[0] * scale, row[1] * scale, row[2] * scale]));
        Ok(Self {
            form: TransformForm::Compound,
            matrix,
            offset: Xyz::new(rows[0][3], rows[1][3], rows[2][3]),
            scale,
        })
    }
}
