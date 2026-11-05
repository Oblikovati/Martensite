//! Ordered chained fusions with independent intermediate and original final checks.
#[path = "boolean_unconditional_case.rs"]
pub(super) mod boolean_unconditional_case;
use martensite_tkbo::bop_algo::Operation;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::storage::format_general;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeombase::gprop::GProps;
use martensite_tktopalgo::brep_gprop::BRepGProp;

fn checked_chain(fixtures: [(&[u8], ShapeEnum); 3], intermediate_mass: &str, area: f64) -> Shape {
    let [first, second, third] =
        fixtures.map(|(bytes, kind)| boolean_unconditional_case::restored_shape(bytes, kind));
    let intermediate = boolean_unconditional_case::direct(&first, &second, Operation::Fuse);
    check_intermediate(&intermediate, intermediate_mass);
    boolean_unconditional_case::checked_fuse_shapes(&intermediate, &third, area)
}

fn check_intermediate(shape: &Shape, expected: &str) {
    // Independent captured intermediate evidence catches skipped/reordered first operations.
    assert_eq!(shape.shape_type().unwrap(), ShapeEnum::Compound);
    let mut properties = GProps::default();
    BRepGProp::surface_properties_eps(shape, &mut properties, 1.0e-4, false).unwrap();
    assert_eq!(format_general(properties.mass(), 6), expected);
    BRepGProp::surface_properties(shape, &mut properties, false, false).unwrap();
    assert_eq!(format_general(properties.mass(), 6), expected);
}

macro_rules! chained_fixture {
    ($name:literal, $kind:ident) => {
        (
            include_bytes!(concat!(
                "../../../external/martensite-test-data/dataset-7.9.0/brep/",
                $name
            ))
            .as_slice(),
            ShapeEnum::$kind,
        )
    };
}

/// Replay both ordered H5 fusions; `checked_h5()`.
pub fn checked_h5() -> Shape {
    checked_chain(
        [
            chained_fixture!("CTO902_cts20482a.rle", Solid),
            chained_fixture!("CTO902_cts20482b.rle", Solid),
            chained_fixture!("CTO902_cts20482c.rle", Solid),
        ],
        "182147",
        189131.0,
    )
}

/// Replay both ordered J2 fusions; `checked_j2()`.
pub fn checked_j2() -> Shape {
    checked_chain(
        [
            chained_fixture!("CTO902_cts20482a.rle", Solid),
            chained_fixture!("CTO902_cts20482b.rle", Solid),
            chained_fixture!("CTO902_cts20482c.rle", Solid),
        ],
        "182147",
        189131.0,
    )
}

/// Replay both ordered J3 fusions; `checked_j3()`.
pub fn checked_j3() -> Shape {
    checked_chain(
        [
            chained_fixture!("CTO902_cts20484a.rle", Compound),
            chained_fixture!("cts20484-tool1.rle", Solid),
            chained_fixture!("cts20484-tool2.rle", Solid),
        ],
        "123859",
        133085.0,
    )
}

/// Replay both ordered M5 fusions; `checked_m5()`.
pub fn checked_m5() -> Shape {
    checked_chain(
        [
            chained_fixture!("cts21306a.rle", Compound),
            chained_fixture!("cts21306b.rle", Solid),
            chained_fixture!("cts21306c.rle", Compound),
        ],
        "9406.97",
        12335.6,
    )
}

/// Replay both ordered N3 fusions; `checked_n3()`.
pub fn checked_n3() -> Shape {
    checked_chain(
        [
            chained_fixture!("CTO904_pro14785a.rle", Solid),
            chained_fixture!("CTO904_pro14785b.rle", Solid),
            chained_fixture!("CTO904_pro14785c.rle", Solid),
        ],
        "417819",
        533100.0,
    )
}
