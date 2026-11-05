use super::*;

fn api_common(first: &Shape, second: &Shape) -> Shape {
    let mut operation = martensite_tkbo::brep_algo_api::BRepAlgoAPICommon::new();
    operation.set_arguments(&std::iter::once(first.clone()).collect::<List<_>>());
    operation.set_tools(&std::iter::once(second.clone()).collect::<List<_>>());
    operation.set_run_parallel(false);
    operation.set_fuzzy_value(precision::confusion());
    operation.set_non_destructive(false);
    operation.set_glue(martensite_tkbo::bop_algo::Glue::Off);
    operation.set_check_inverted(true);
    operation.set_use_obb(false);
    operation.set_to_fill_history(true);
    operation.build().expect("source bcommon");
    assert!(operation.is_done(), "source bcommon must complete");
    operation
        .simplify_result(false, false, precision::angular())
        .expect("source disabled simplification");
    operation.shape().clone()
}

fn common_case(source: &str, input: &[u8], expected_area: f64) -> Shape {
    let compound = restored_shape(input);
    let operands: Vec<_> = ShapeIterator::new(&compound, false, true).collect();
    assert_eq!(operands.len(), 2, "{source}: source explode a");
    let result_shape = api_common(&operands[0], &operands[1]);
    let result = &result_shape;
    let checker = BRepCheckAnalyzer::new(result, true, false, false).expect("source checkshape");
    assert!(
        checker.is_valid().expect("source validity"),
        "{source}: checkshape result"
    );
    for (kind, expected) in [
        (ShapeEnum::Vertex, 8),
        (ShapeEnum::Edge, 12),
        (ShapeEnum::Wire, 6),
        (ShapeEnum::Face, 6),
        (ShapeEnum::Shell, 1),
        (ShapeEnum::Solid, 1),
    ] {
        assert_eq!(
            count_entries(result, kind),
            expected,
            "{source}: checknbshapes result {kind:?}"
        );
    }
    assert!(
        maximum_tolerance(result) <= 1.0e-5,
        "{source}: checkmaxtol result -min_tol 1.e-5; got {}",
        maximum_tolerance(result)
    );
    check_area(source, result, expected_area);
    result_shape
}

pub fn checked_32942() -> Shape {
    common_case(
        "tests/bugs/modalg_8/bug32942",
        include_bytes!(
            "../../../../external/martensite-test-data/dataset-7.9.0/brep/bug32942.brep"
        ),
        21_746.8,
    )
}

pub fn checked_32943() -> Shape {
    common_case(
        "tests/bugs/modalg_8/bug32943",
        include_bytes!(
            "../../../../external/martensite-test-data/dataset-7.9.0/brep/bug32943.brep"
        ),
        17_615.5,
    )
}

pub fn checked_32944() -> Shape {
    common_case(
        "tests/bugs/modalg_8/bug32944",
        include_bytes!(
            "../../../../external/martensite-test-data/dataset-7.9.0/brep/bug32944.brep"
        ),
        36_741.6,
    )
}

pub fn checked_32945() -> Shape {
    common_case(
        "tests/bugs/modalg_8/bug32945",
        include_bytes!(
            "../../../../external/martensite-test-data/dataset-7.9.0/brep/bug32945.brep"
        ),
        83_754.8,
    )
}
