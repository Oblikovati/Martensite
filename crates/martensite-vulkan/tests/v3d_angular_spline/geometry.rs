use super::*;
use martensite_tkbrep::{topexp::Explorer, topods};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::{Point3, Transform3};
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPICopy, BRepBuilderAPIMakeShape, BRepBuilderAPINurbsConvert,
    BRepBuilderAPITransform,
};

fn first_copied_face(shape: &Shape) -> Result<Shape, BoxError> {
    let first = Iterator::next(&mut Explorer::new(
        shape,
        ShapeEnum::Face,
        ShapeEnum::Shape,
    )?)
    .ok_or("explode rc f faces=0; expected rc_1")?;
    let face = topods::face(&first)?.into_shape();
    let mut copy = BRepBuilderAPICopy::with_shape(&face, true, false)?;
    Ok(copy.shape()?.clone())
}

fn scaled_nurbs_face(shape: &Shape) -> Result<Shape, BoxError> {
    let mut transform = Transform3::default();
    transform.set_scale(Point3::new(0., 0., 0.), 0.01)?;
    let mut maker = BRepBuilderAPITransform::new(transform);
    maker.perform(shape, false, false)?;
    let scaled = maker.shape()?.clone();
    let mut converted = BRepBuilderAPINurbsConvert::with_shape(&scaled, false)?;
    Ok(converted.shape()?.clone())
}

pub(super) fn actual_scaled_cut_face() -> Result<Shape, BoxError> {
    let cylinder = crate::primitive_shapes::cylinder(100., 500.)?;
    let half_space = super::surface::case_half_space()?;
    let (cut, _) = crate::common_operation::direct_with_fuzzy_observed(
        &cylinder,
        &half_space,
        martensite_tkbo::bop_algo::Operation::Cut,
        martensite_tkernel::precision::confusion(),
        |operation| crate::common_operation::report_result_alerts(operation.options()),
    );
    scaled_nurbs_face(&first_copied_face(&cut)?)
}
