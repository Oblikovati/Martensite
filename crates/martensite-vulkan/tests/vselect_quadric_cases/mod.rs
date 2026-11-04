//! Selection of analytic quadric sensitives (cylinders, cones, spheres): detection, picked
//! depth, point, rectangle and polygon selection, and selection-buffer images.
mod circles;
mod detection;
mod normal_images;
mod picks;
mod scenes;
mod transformed;
pub(crate) use circles::{circle_sector_picks, circle_wire_picks, filled_circle_picks};
pub(crate) use detection::{
    cone_cylinder_depth, cone_cylinder_detecting, sphere_depth, sphere_detecting,
};
pub(crate) use normal_images::{cone_cylinder_normal_images, sphere_normal_images};
pub(crate) use scenes::{
    cone_cylinder_polygons, cone_cylinder_rectangles, cone_single_clicks, cylinder_single_clicks,
    sphere_polygons, sphere_rectangles, sphere_single_clicks, truncated_cone_single_clicks,
};
pub(crate) use transformed::transformed_circle_images;

/// What every quadric selection regression program uses.
mod prelude {
    pub(super) use super::picks::{Pick, QuadricRun, ViewStep};
    pub(super) use crate::primitive_shapes::{cone, cylinder, sphere, translated};
    pub(super) use crate::session_run::banner;
    pub(super) use martensite_tkv3d::v3d::V3dTypeOfOrientation as Orientation;
    pub(super) use std::error::Error;
    /// A regression program's result.
    pub(super) type Outcome = Result<(), Box<dyn Error>>;
}

mod axis_cases;
mod datum_objects;
mod hollow;
mod ray_command;
mod source_checks;
mod transformed_input;
mod transformed_shapes;
mod zero_tolerance;
pub(crate) use axis_cases::{axis_a1, axis_a2};
pub(crate) use hollow::hollow_cone_cyl;
pub(crate) use transformed_shapes::{transformed_cone, transformed_cylinder};
pub(crate) use zero_tolerance::axis_a3;

mod transformed_sphere;
mod transformed_sphere_input;
pub(crate) use transformed_sphere::sphere_trsf;
