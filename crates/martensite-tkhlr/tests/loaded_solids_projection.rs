//! The two-solid projection belongs with its hidden-line consumer, keeping test
//! dependencies acyclic and source packages independently verifiable (#241).
use martensite_tkbrep::toptools::ShapeSet;
use martensite_tkbrep::{
    brep::BRepBuilder,
    brep_tools::BRepToolsShapeSet,
    topods::{Compound, Shape},
};
use martensite_tkernel::{standard::Handle, tcollection::AsciiString};
use martensite_tkhlr::{
    hlr_algo::Projector,
    hlr_brep::{Algo, HLRToShape},
    hlr_topo_brep::{MapOfShapeTool, OutLiner},
};
use martensite_tkmath::gp::{Axis2, Direction3, Point3};
use martensite_tkprim::brep_prim_api::{
    BRepPrimAPIMakeBox, BRepPrimAPIMakeCylinder, BRepPrimAPIMakeOneAxis,
};

fn input_solids() -> [Shape; 2] {
    let block = BRepPrimAPIMakeBox::at_point(Point3::new(-10., -10., 0.), 20., 20., 5.)
        .and_then(|mut builder| builder.solid())
        .unwrap()
        .into_shape();
    let cylinder = BRepPrimAPIMakeCylinder::new(5., 10.)
        .solid()
        .unwrap()
        .into_shape();
    [block, cylinder]
}

fn load_outline(algorithm: &Algo, shape: &Shape, projector: &Projector) {
    let outline = Handle::new(OutLiner::with_original(shape));
    outline
        .fill(projector, &mut MapOfShapeTool::default(), 0)
        .unwrap();
    algorithm.load(&outline, 0);
}

fn output_classes(algorithm: &Handle<Algo>) -> [Shape; 10] {
    let output = HLRToShape::new(algorithm);
    [
        output.v_compound().unwrap(),
        output.rg1_line_v_compound().unwrap(),
        output.rgn_line_v_compound().unwrap(),
        output.out_line_v_compound().unwrap(),
        output.iso_line_v_compound().unwrap(),
        output.h_compound().unwrap(),
        output.rg1_line_h_compound().unwrap(),
        output.rgn_line_h_compound().unwrap(),
        output.out_line_h_compound().unwrap(),
        output.iso_line_h_compound().unwrap(),
    ]
}

fn combined_outputs(algorithm: &Handle<Algo>) -> Shape {
    let builder = BRepBuilder::default();
    let mut compound = Compound::default();
    builder.make_compound(&mut compound);
    let mut result = compound.into_shape();
    for output in output_classes(algorithm) {
        if !output.is_null() {
            builder.add(&mut result, &output).unwrap();
        }
    }
    result
}

fn unique_edge_count(shape: &Shape) -> usize {
    let mut shapes = BRepToolsShapeSet::new(true, false);
    shapes.add(shape).unwrap();
    let mut extent = AsciiString::default();
    shapes.dump_extent_string(&mut extent).unwrap();
    let text = String::from_utf8_lossy(extent.as_bytes());
    let edges = text
        .lines()
        .find(|line| line.trim_start().starts_with("EDGE"))
        .unwrap();
    edges.split(':').nth(1).unwrap().trim().parse().unwrap()
}

#[test]
fn projection_keeps_edges_from_both_loaded_solids() {
    let frame = Axis2::with_x_direction(Point3::default(), Direction3::Y, Direction3::Z).unwrap();
    let projector = Projector::with_frame(frame).unwrap();
    let algorithm = Handle::new(Algo::new().unwrap());
    while algorithm.nb_shapes() > 0 {
        algorithm.remove(1).unwrap();
    }
    for shape in input_solids() {
        load_outline(&algorithm, &shape, &projector);
    }
    algorithm.set_projector(&projector);
    algorithm.update().unwrap();
    algorithm.hide().unwrap();
    assert_eq!(unique_edge_count(&combined_outputs(&algorithm)), 16);
}
