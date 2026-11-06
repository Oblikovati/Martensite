use super::*;
use martensite_tkbo::bop_algo::Operation;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeCone;
const SOURCE: &str = "tests/bugs/modalg_8/bug32721";

fn assert_counts(label: &str, shape: &Shape, expected: [usize; 6]) {
    for (kind, count_expected) in [
        ShapeEnum::Vertex,
        ShapeEnum::Edge,
        ShapeEnum::Wire,
        ShapeEnum::Face,
        ShapeEnum::Shell,
        ShapeEnum::Solid,
    ]
    .into_iter()
    .zip(expected)
    {
        assert_eq!(
            count_located(shape, kind),
            count_expected,
            "{SOURCE}: checknbshapes {label} -t {kind:?}"
        );
    }
}

pub fn checked_cone_prism() -> Shape {
    let prism = restored_shape(include_bytes!(
        "../../../../external/martensite-test-data/dataset-7.9.0/brep/bug32721.brep"
    ));
    let mut cone_maker = BRepPrimAPIMakeCone::new(6., 0., 10.).expect("source pcone cone 6 0 10");
    let cone = cone_maker.shape().expect("source pcone result").clone();
    let mut filler = common_operation::intersections(&cone, &prism);
    let fuse = common_operation::prepared_with_filler(&cone, &prism, Operation::Fuse, &mut filler);
    let common =
        common_operation::prepared_with_filler(&cone, &prism, Operation::Common, &mut filler);
    let cut = common_operation::prepared_with_filler(&cone, &prism, Operation::Cut, &mut filler);
    let tuc = common_operation::prepared_with_filler(&cone, &prism, Operation::Cut21, &mut filler);
    let results = [
        ("r1", &fuse, [8, 17, 8, 8, 1, 1], 388.634, 406.357),
        ("r2", &common, [3, 7, 4, 4, 1, 1], 57.8605, 22.8116),
        ("r3", &cut, [4, 10, 4, 4, 1, 1], 358.735, 354.179),
        ("r4", &tuc, [7, 14, 8, 8, 2, 2], 87.7592, 29.3659),
    ];
    for (label, result, counts, _, _) in results {
        check_valid(label, result);
        assert_counts(label, result, counts);
    }
    assert!(
        maximum_tolerance(&fuse) <= 0.0002,
        "{SOURCE}: checkmaxtol r1 = {}",
        maximum_tolerance(&fuse)
    );
    for (label, result, _, area, volume) in results {
        check_area(label, result, area);
        check_volume(label, result, volume);
    }
    fuse
}
