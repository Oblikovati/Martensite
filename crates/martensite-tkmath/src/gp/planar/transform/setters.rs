//! Transform2 Set* members that replace the whole state; each delegates to the matching
//! constructor so the setter and constructor spellings cannot drift apart.
use super::super::{Axis2d, Point2, Vector2};
use super::Transform2;
impl Transform2 {
    /// Transform2::set_mirror_point; `transform.set_mirror_point(center)`.
    pub fn set_mirror_point(&mut self, center: Point2) {
        *self = Self::mirror_point(center);
    }
    /// Transform2::set_mirror_point; `transform.set_mirror_axis(axis)`.
    pub fn set_mirror_axis(&mut self, axis: Axis2d) {
        *self = Self::mirror_axis(axis);
    }
    /// Transform2::set_rotation; `transform.set_rotation(center,0.5)`.
    pub fn set_rotation(&mut self, center: Point2, angle: f64) {
        *self = Self::rotation(center, angle);
    }
    /// Transform2::set_scale; `transform.set_scale(center,2.0)`.
    pub fn set_scale(&mut self, center: Point2, scale: f64) {
        *self = Self::scaling(center, scale);
    }
    /// Transform2::transformation_to; `transform.set_transformation_between(a,b)`.
    pub fn set_transformation_between(&mut self, from: Axis2d, to: Axis2d) {
        *self = Self::transformation_between(from, to);
    }
    /// Transform2::transformation_to; `transform.set_transformation_to(axis)`.
    pub fn set_transformation_to(&mut self, axis: Axis2d) {
        *self = Self::transformation_to(axis);
    }
    /// Transform2::set_translation; `transform.set_translation(vector)`.
    pub fn set_translation(&mut self, vector: Vector2) {
        *self = Self::translation_vector(vector);
    }
    /// Transform2::set_translation; `transform.set_translation_between(a,b)`.
    pub fn set_translation_between(&mut self, from: Point2, to: Point2) {
        *self = Self::translation_between(from, to);
    }
}
