//! Point clouds built from a meshed shape: detection of the cloud's bounding box under view
//! clipping planes.
mod cloud;
mod sensitive_box;
pub(crate) use sensitive_box::clipped_box_detection;

// The older wide host selects only sensitivebox; the bounded sphere host owns the new body.
#[allow(dead_code)]
mod sphere;
#[allow(unused_imports)]
pub(crate) use sphere::colored_textured_sphere;

#[allow(unused_imports)]
pub(crate) use sphere::green_vertex_output_prefix;
