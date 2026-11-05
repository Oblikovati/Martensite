//! Real authored boxes, prism subtractions, shape copies and bounding-box-driven hollowing.
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::{bnd::BoundingBox3, gp::Vector3};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakePrism;
use martensite_tktopalgo::{
    brep_bnd_lib::BRepBndLib,
    brep_builder_api::{BRepBuilderAPICopy, BRepBuilderAPIMakeShape},
};

const CORNERS: [[f64; 3]; 10] = [
    [0., 0., 0.],
    [2., 0., 0.],
    [4., 0., 0.],
    [6., 0., 0.],
    [6., -1.5, 0.],
    [6., -3., 0.],
    [6., -4.5, 0.],
    [6., -6., 0.],
    [6., -6., 1.],
    [6., -6., 2.],
];

pub(super) fn boxes() -> Result<Vec<Shape>, Failure> {
    let mut scale = 1.;
    CORNERS
        .into_iter()
        .map(|corner| {
            scale *= 0.94;
            crate::primitive_shapes::box_at(corner, [1.8 * scale, 1.8 * scale, 1.3 * scale])
        })
        .collect()
}

fn prism(shape: &Shape, face: usize, vector: [f64; 3]) -> Result<Shape, Failure> {
    let faces = crate::boolean_typed_explode::all_distinct(shape, ShapeEnum::Face);
    let [x, y, z] = vector;
    let mut builder =
        BRepPrimAPIMakePrism::new(&faces[face - 1], Vector3::new(x, y, z), false, true)?;
    Ok(builder.shape()?.clone())
}

fn source_copy(shape: &Shape) -> Result<Shape, Failure> {
    let mut copy = BRepBuilderAPICopy::with_shape(shape, true, false)?;
    Ok(copy.shape()?.clone())
}

pub(super) fn trim_last(boxes: &mut [Shape]) -> Result<(), Failure> {
    let first_tool = prism(&boxes[0], 5, [12.3, -14., 6.8])?;
    let first_cut = crate::common_operation::direct(
        &boxes[9],
        &first_tool,
        martensite_tkbo::bop_algo::Operation::Cut,
    );
    let second_tool = prism(&boxes[1], 3, [12., -14., 6.8])?;
    let second_cut = crate::common_operation::direct(
        &first_cut,
        &second_tool,
        martensite_tkbo::bop_algo::Operation::Cut,
    );
    boxes[9] = source_copy(&second_cut)?;
    Ok(())
}

pub(super) fn hollow_first(
    boxes: &mut [Shape],
) -> Result<super::bounding_preview::BoundingPreview, Failure> {
    let mut bounds = BoundingBox3::default();
    BRepBndLib::add(&boxes[0], &mut bounds, true)?;
    let preview = super::bounding_preview::BoundingPreview::new(&bounds)?;
    let [xmin, ymin, zmin, xmax, _, _] = bounds.get()?;
    let dx = xmax - xmin;
    let tool = crate::primitive_shapes::box_at(
        [xmin + 0.1 * dx, ymin + 0.1 * dx, zmin + 0.1 * dx],
        [0.8 * dx, 0.8 * dx, dx],
    )?;
    let hollow = crate::common_operation::direct(
        &boxes[0],
        &tool,
        martensite_tkbo::bop_algo::Operation::Cut,
    );
    boxes[0] = source_copy(&hollow)?;
    Ok(preview)
}

pub(super) fn complete_boxes()
-> Result<(Vec<Shape>, super::bounding_preview::BoundingPreview), Failure> {
    let mut shapes = boxes()?;
    trim_last(&mut shapes)?;
    let preview = hollow_first(&mut shapes)?;
    Ok((shapes, preview))
}
