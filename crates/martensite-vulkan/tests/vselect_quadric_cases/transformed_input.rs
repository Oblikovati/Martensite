//! Original primitive construction, occurrence traversal and forced location scaling.
use crate::boolean_typed_explode::all_distinct;
use crate::primitive_shapes::{compound_of, cone, cylinder, rotated_about, translated};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::{
    gp::{Point3, Transform3},
    toploc::Location,
};

/// Preserve `fscale` as a non-isometric location, never baked geometry; `scaled_location(&shape, 0.1)?`.
pub(crate) fn scaled_location(shape: &Shape, factor: f64) -> Result<Shape, Failure> {
    let mut scale = Transform3::default();
    scale.set_scale(Point3::new(0., 0., 0.), factor)?;
    shape.moved(&Location::new(scale), false)
}

/// The source's scale, rotation, translation in that order; `transformed_quadric(true)?`.
pub(crate) fn transformed_quadric(is_cone: bool) -> Result<Shape, Failure> {
    let source = if is_cone {
        cone(10., 5., 10.)?
    } else {
        cylinder(10., 20.)?
    };
    let scaled = scaled_location(&source, 0.1)?;
    let rotated = rotated_about(&scaled, [0.; 3], [1., 1., 0.], 25.)?;
    translated(&rotated, [2500., 3500., 1000.])
}

/// The exact distinct shell occurrence Compound; `shell_compound(&shape)?`.
pub(crate) fn shell_compound(shape: &Shape) -> Result<Shape, Failure> {
    let shells = all_distinct(shape, ShapeEnum::Shell);
    compound_of(&shells)
}

/// Original construction then cylinder/cone face traversal order; `hollow_faces()?`.
pub(crate) fn hollow_faces() -> Result<[Shape; 2], Failure> {
    let cone = hollow_shape(true)?;
    let cylinder = hollow_shape(false)?;
    let cylinder_face = all_distinct(&cylinder, ShapeEnum::Face)[0].clone();
    let cone_face = all_distinct(&cone, ShapeEnum::Face)[0].clone();
    Ok([cylinder_face, cone_face])
}

fn hollow_shape(is_cone: bool) -> Result<Shape, Failure> {
    let source = if is_cone {
        cone(10., 5., 10.)?
    } else {
        cylinder(7., 7.)?
    };
    let rotated = rotated_about(&source, [0.; 3], [1., 1., 0.], 30.)?;
    if is_cone {
        return translated(&rotated, [15., 15., 0.]);
    }
    Ok(rotated)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn forced_scale_keeps_the_source_identity_and_scaled_location() {
        let source = cylinder(10., 20.).unwrap();
        let scaled = scaled_location(&source, 0.1).unwrap();
        assert!(source.is_partner(&scaled));
        assert_eq!(scaled.location().transformation().scale_factor(), 0.1);
        assert_eq!(source.location().transformation().scale_factor(), 1.);
    }
}
