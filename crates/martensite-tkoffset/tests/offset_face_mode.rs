//! Reference differential for the face mode of `BRepOffsetOffset`.
use martensite_tkbrep::{brep::BRepTool, topexp::Explorer, topods::Face};
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeombase::gprop::GProps;
use martensite_tkmath::{geom_abs::JoinType, gp::Plane3};
use martensite_tkoffset::brep_offset::{BRepOffsetOffset, BRepOffsetStatus};
use martensite_tktopalgo::{
    brep_builder_api::{BRepBuilderAPIMakeFace, BRepBuilderAPIMakeShape},
    brep_check::BRepCheckAnalyzer,
    brep_gprop::BRepGProp,
};

#[test]
fn bounded_plane_uses_the_reference_default_face_mode()
-> Result<(), martensite_tkernel::standard::Failure> {
    let mut maker =
        BRepBuilderAPIMakeFace::from_plane_bounds(Plane3::default(), (0.0, 1.0, 0.0, 2.0))?;
    let source = Face::unchecked(maker.shape()?.clone());
    let offset = BRepOffsetOffset::from_face(&source, 0.5, true, JoinType::Arc)?;
    let image = offset.face();
    let image_shape = image.clone().into_shape();
    assert_eq!(offset.status(), BRepOffsetStatus::Good);
    assert!(offset.initial_shape().is_same(&source.clone().into_shape()));
    assert_eq!(count(&image_shape, ShapeEnum::Edge)?, 4);
    assert_eq!(count(&image_shape, ShapeEnum::Vertex)?, 8);
    assert!(BRepCheckAnalyzer::new(&image_shape, true, false, false)?.is_valid()?);
    let mut properties = GProps::default();
    BRepGProp::surface_properties(&image_shape, &mut properties, false, false)?;
    assert!((properties.mass() - 2.0).abs() < 1.0e-12);
    assert!(
        !offset
            .generated(&first_edge(&source)?.into_shape())
            .is_null()
    );
    assert_eq!(BRepTool::face_tolerance(image)?, 1.0e-7);
    Ok(())
}

fn first_edge(face: &Face) -> Result<martensite_tkbrep::topods::Edge, Failure> {
    let edges = Explorer::new(
        &face.clone().into_shape(),
        ShapeEnum::Edge,
        ShapeEnum::Shape,
    )?;
    Ok(martensite_tkbrep::topods::Edge::unchecked(
        edges.current().clone(),
    ))
}

fn count(shape: &martensite_tkbrep::topods::Shape, kind: ShapeEnum) -> Result<usize, Failure> {
    Ok(Explorer::new(shape, kind, ShapeEnum::Shape)?.count())
}
