//! Solid initialization keeps complete incidence lists independently of bound openness.
use martensite_tkbo::bop_ds::DataStructure;
use martensite_tkbrep::{
    brep::BRepBuilder,
    topods::{Face, Shape, Shell, Solid},
};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::Plane3;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeFace;

fn initialize_single_solid(solid: &Shape) -> DataStructure {
    let mut structure = DataStructure::default();
    structure.set_arguments(std::slice::from_ref(solid));
    structure.init(1e-4).unwrap();
    structure
}

#[test]
fn inverted_solid_initialization_has_whole_bounds() {
    let solid = BRepPrimAPIMakeBox::from_sizes(1., 2., 3.)
        .unwrap()
        .solid()
        .unwrap()
        .into_shape();
    let ordinary = initialize_single_solid(&solid);
    assert!(!solid_has_whole_bounds(&ordinary, &solid));
    let inverted = solid.reversed();
    let structure = initialize_single_solid(&inverted);
    let record = structure.shape_info(structure.index(&inverted)).unwrap();
    assert!(record.box_bounds().is_whole());
    assert_eq!(record.sub_shapes().len(), 26);
}

fn open_shell_face(width: f64) -> (Shape, Face) {
    let builder = BRepBuilder::default();
    let face = BRepBuilderAPIMakeFace::from_plane_bounds(Plane3::default(), (0., width, 0., 1.))
        .unwrap()
        .face()
        .unwrap();
    let mut shell = Shell::default();
    builder.make_shell(&mut shell);
    let mut shape = shell.into_shape();
    builder.add(&mut shape, &face).unwrap();
    (shape, face)
}

fn two_open_shell_solid() -> (Shape, [Face; 2]) {
    let builder = BRepBuilder::default();
    let (first_shell, first_face) = open_shell_face(1.);
    let (second_shell, second_face) = open_shell_face(2.);
    let mut solid = Solid::default();
    builder.make_solid(&mut solid);
    let mut shape = solid.into_shape();
    builder.add(&mut shape, &first_shell).unwrap();
    builder.add(&mut shape, &second_shell).unwrap();
    (shape, [first_face, second_face])
}

#[test]
fn open_first_shell_does_not_discard_later_shell_incidence() {
    let (shape, faces) = two_open_shell_solid();
    let structure = initialize_single_solid(&shape);
    let record = structure.shape_info(structure.index(&shape)).unwrap();
    assert!(record.box_bounds().is_whole());
    for face in faces {
        assert!(record.sub_shapes().contains(&structure.index(&face)));
    }
    assert!(
        record
            .sub_shapes()
            .iter()
            .all(|index| structure.shape_info(*index).unwrap().shape_type() != ShapeEnum::Shell)
    );
}

fn solid_has_whole_bounds(structure: &DataStructure, solid: &Shape) -> bool {
    structure
        .shape_info(structure.index(solid))
        .unwrap()
        .box_bounds()
        .is_whole()
}
