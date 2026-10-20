//! A retained cut result: the BRep of a cylinder (10, 20) cut by a box (10, 10, 10), with
//! its topology and global properties (the result has no symmetry axis).
#[path = "common/floating.rs"]
mod floating;
use floating::within_absolute_error as near;
use martensite_tkbrep::brep::BRepBuilder;
use martensite_tkbrep::brep_tools;
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::storage::IoStream;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeombase::gprop::GProps;
use martensite_tktopalgo::brep_gprop::BRepGProp;

// Native reference of BRepAlgoAPI_Cut(MakeCylinder(10, 20), MakeBox(10, 10, 10)).
#[test]
fn cut_shape_has_no_symmetry_axis_from_retained_brep() {
    let source = include_bytes!("data/brep_gprop/cylinder-minus-box.brep");
    let mut stream = IoStream::from_bytes(source.to_vec());
    let mut cut_result = Shape::default();
    brep_tools::read(
        &mut cut_result,
        &mut stream,
        &BRepBuilder::default(),
        &ProgressRange::default(),
    )
    .unwrap();
    assert!(!cut_result.is_null());
    assert_eq!(cut_result.shape_type().unwrap(), ShapeEnum::Compound);
    assert_eq!(
        Explorer::new(&cut_result, ShapeEnum::Vertex, ShapeEnum::Shape)
            .unwrap()
            .count(),
        44
    );
    assert_eq!(
        Explorer::new(&cut_result, ShapeEnum::Edge, ShapeEnum::Shape)
            .unwrap()
            .count(),
        22
    );
    assert_eq!(
        Explorer::new(&cut_result, ShapeEnum::Face, ShapeEnum::Shape)
            .unwrap()
            .count(),
        6
    );
    let mut props = GProps::new();
    BRepGProp::volume_properties(&cut_result, &mut props, false, false, false).unwrap();
    assert!(near(props.mass(), 5_497.787_143_782_137, 1.0e-9));
    let center = props.centre_of_mass();
    assert!(near(center.x(), -0.606_304_545_111_983_6, 1.0e-12));
    assert!(near(center.y(), -0.606_304_545_111_983, 1.0e-12));
    assert!(near(center.z(), 10.714285714285715, 1.0e-12));
    let inertia = props.matrix_of_inertia();
    let expected = [
        315878.24146288086,
        14521.015150373298,
        -19047.619047619028,
        14521.015150373298,
        315_878.241_462_879_1,
        -19047.619047619057,
        -19047.619047619028,
        -19047.619047619057,
        270847.32688836136,
    ];
    for (index, value) in expected.into_iter().enumerate() {
        let row = (index / 3 + 1) as i32;
        let column = (index % 3 + 1) as i32;
        assert!(near(inertia.value(row, column).unwrap(), value, 1.0e-7));
    }
    assert!(!props.principal_properties().unwrap().has_symmetry_axis());
}
