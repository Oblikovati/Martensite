//! Transform3 Set* members that replace the whole state; each delegates to the matching
//! constructor so the two spellings cannot drift apart.
use super::super::{
    Axis1, Axis2, Axis3, Failure, Point3, Quaternion, Transform3, TransformForm, Vector3,
    construction,
};
impl Transform3 {
    /// Transform3::set_mirror_point; `transform.set_mirror_point(center)`.
    pub fn set_mirror_point(&mut self, center: Point3) {
        *self = Self::mirror_point(center);
    }
    /// Transform3::set_mirror_point; `transform.set_mirror_axis(axis)`.
    pub fn set_mirror_axis(&mut self, axis: Axis1) {
        *self = Self::mirror_axis(axis);
    }
    /// Transform3::set_mirror_point; `transform.set_mirror_plane(frame)`.
    pub fn set_mirror_plane(&mut self, frame: Axis2) {
        *self = Self::mirror_plane(frame);
    }
    /// Transform3::set_rotation; `transform.set_rotation(axis, 0.5)?`.
    pub fn set_rotation(&mut self, axis: Axis1, angle: f64) -> Result<(), Failure> {
        *self = Self::rotation(axis, angle)?;
        Ok(())
    }
    /// Transform3::set_rotation; `transform.set_rotation_quaternion(q)`.
    pub fn set_rotation_quaternion(&mut self, rotation: Quaternion) {
        *self = Self::from_quaternion(rotation);
    }
    /// Transform3::set_scale: the form, scale and offset are assigned before the zero-scale
    /// raise; the matrix becomes identity only on success;
    /// `transform.set_scale(center, 2.0)?`.
    pub fn set_scale(&mut self, center: Point3, scale: f64) -> Result<(), Failure> {
        self.form = TransformForm::Scale;
        self.scale = scale;
        self.offset = center.xyz();
        if scale.abs() <= f64::MIN_POSITIVE {
            return Err(construction(&format!(
                "gp_Trsf::SetScaleFactor - scale {scale} is not above gp::Resolution"
            )));
        }
        self.matrix.set_identity();
        self.offset *= 1.0 - scale;
        Ok(())
    }
    /// Transform3::set_displacement; `transform.set_displacement(from, to)`.
    pub fn set_displacement(&mut self, from: Axis3, to: Axis3) {
        *self = Self::displacement(from, to);
    }
    /// Transform3::set_rotation_translation; `transform.set_transformation_between(a, b)`.
    pub fn set_transformation_between(&mut self, from: Axis3, to: Axis3) {
        *self = Self::transformation_between(from, to);
    }
    /// Transform3::set_rotation_translation; `transform.set_transformation_to(frame)`.
    pub fn set_transformation_to(&mut self, frame: Axis3) {
        *self = Self::transformation_to(frame);
    }
    /// Transform3::set_rotation_translation; `transform.set_rotation_translation(q, v)`.
    pub fn set_rotation_translation(&mut self, rotation: Quaternion, translation: Vector3) {
        *self = Self::rotation_translation(rotation, translation);
    }
    /// Transform3::set_translation; `transform.set_translation(vector)`.
    pub fn set_translation(&mut self, vector: Vector3) {
        *self = Self::translation_vector(vector);
    }
    /// Transform3::set_translation; `transform.set_translation_between(a, b)`.
    pub fn set_translation_between(&mut self, from: Point3, to: Point3) {
        *self = Self::translation_between(from, to);
    }
    /// Overwrites the form without touching the numbers, as `SetForm`; `transform.set_form(form)`.
    pub fn set_form(&mut self, form: TransformForm) {
        self.form = form;
    }
}
