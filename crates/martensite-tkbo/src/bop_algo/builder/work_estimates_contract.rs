//! Counter fixtures deliberately test DS state, not geometric acceptance of same-domain bindings.
use super::{BuilderProgressProfile, builder_work_estimates, source_work_counts};
use crate::bop_ds::DataStructure;
use martensite_tkbrep::topods::Shape;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;

fn box_structure(active: bool) -> DataStructure {
    let mut maker = BRepPrimAPIMakeBox::from_sizes(2.0, 3.0, 4.0).unwrap();
    let shape: Shape = maker.shape().unwrap().clone();
    let mut structure = DataStructure::default();
    structure.set_arguments(&[shape]);
    structure.init(1.0e-7).unwrap();
    if active {
        activate_work_references(&mut structure);
    }
    structure
}

fn first_kind(structure: &DataStructure, kind: ShapeEnum) -> i32 {
    (0..structure.nb_source_shapes())
        .find(|index| structure.shape_info(*index).unwrap().shape_type() == kind)
        .unwrap_or_else(|| panic!("source kind={kind:?}; expected fixture entry"))
}

fn activate_work_references(structure: &mut DataStructure) {
    let edge = first_kind(structure, ShapeEnum::Edge);
    let face = first_kind(structure, ShapeEnum::Face);
    structure.change_pave_blocks(edge).unwrap();
    structure.change_face_info(face).unwrap();
    structure.add_shape_sd(3, 8);
    structure.add_shape_sd(8, 11);
}

fn pool_reference_snapshot(structure: &DataStructure) -> Vec<i32> {
    (0..structure.nb_source_shapes())
        .map(|index| structure.shape_info(index).unwrap().reference())
        .collect()
}

fn assert_estimates(profile: BuilderProgressProfile, history: bool, expected: &[f64]) {
    let structure = box_structure(true);
    let before = pool_reference_snapshot(&structure);
    let counts = source_work_counts(&structure).unwrap();
    let steps = builder_work_estimates(&structure, profile, history).unwrap();
    let observed: Vec<String> = (0..expected.len())
        .map(|i| format!("{:016x}", steps.get_step(i as i32).to_bits()))
        .collect();
    for (index, value) in expected.iter().enumerate() {
        assert_eq!(
            steps.get_step(index as i32).to_bits(),
            value.to_bits(),
            "step={index}"
        );
    }
    assert_eq!(pool_reference_snapshot(&structure), before);
    println!(
        r#"{{"profile":"{profile:?}","history":{history},"counts":{counts:?},"steps_bits":{observed:?}}}"#
    );
}

#[test]
fn inactive_pools_are_not_counted_or_initialized_by_estimation() {
    let structure = box_structure(false);
    let references = pool_reference_snapshot(&structure);
    assert_eq!(
        source_work_counts(&structure).unwrap(),
        [0, 0, 6, 0, 1, 1, 0, 0]
    );
    assert_eq!(pool_reference_snapshot(&structure), references);
}

#[test]
fn counts_include_only_direct_domain_bindings_and_active_edge_face_pools() {
    let structure = box_structure(true);
    assert_eq!(structure.shape_sd(3), Some(11));
    assert_eq!(
        source_work_counts(&structure).unwrap(),
        [2, 1, 6, 1, 1, 1, 0, 0]
    );
    assert_eq!(structure.same_domain_binding_count(), 2);
}

#[test]
fn general_fuse_history_reserves_five_and_three_percent() {
    let expected = [
        184.0 / 80.0,
        92.0 / 80.0,
        552.0 / 80.0,
        1840.0 / 80.0,
        92.0 / 80.0,
        4600.0 / 80.0,
        0.0,
        0.0,
        5.0,
        3.0,
    ];
    assert_estimates(BuilderProgressProfile::GeneralFuse, true, &expected);
    let profile = BuilderProgressProfile::GeneralFuse;
    let _: &dyn Send = &profile;
    let _: &dyn Sync = &profile;
}

#[test]
fn general_fuse_without_history_does_not_reserve_history_budget() {
    let expected = [
        194.0 / 80.0,
        97.0 / 80.0,
        582.0 / 80.0,
        1940.0 / 80.0,
        97.0 / 80.0,
        4850.0 / 80.0,
        0.0,
        0.0,
        0.0,
        3.0,
    ];
    assert_estimates(BuilderProgressProfile::GeneralFuse, false, &expected);
    assert_eq!(expected[8], 0.0);
}

#[test]
fn intersecting_volume_estimates_face_work_and_solid_construction_separately() {
    let expected = [
        184.0 / 109.0,
        92.0 / 109.0,
        552.0 / 109.0,
        4600.0 / 109.0,
        0.0,
        0.0,
        0.0,
        0.0,
        5.0,
        3.0,
        4600.0 / 109.0,
    ];
    let profile = BuilderProgressProfile::MakerVolume { intersect: true };
    assert_estimates(profile, true, &expected);
}

#[test]
fn separated_volume_does_not_allocate_interference_image_work() {
    let expected = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 5.0, 3.0, 92.0];
    let profile = BuilderProgressProfile::MakerVolume { intersect: false };
    assert_estimates(profile, true, &expected);
}

#[test]
fn zero_work_retains_constants_without_fabricating_remaining_work() {
    let structure = DataStructure::default();
    let steps =
        builder_work_estimates(&structure, BuilderProgressProfile::GeneralFuse, true).unwrap();
    for index in 0..8 {
        assert_eq!(steps.get_step(index), 0.0);
    }
    assert_eq!(steps.get_step(8), 5.0);
    assert_eq!(steps.get_step(9), 3.0);
}

#[test]
fn section_uses_its_five_declared_slots_and_distinct_history_budget() {
    for (history, expected) in [
        (true, [34., 17., 34., 10., 5.]),
        (false, [38., 19., 38., 0., 5.]),
    ] {
        let structure = box_structure(true);
        let references = pool_reference_snapshot(&structure);
        let steps =
            builder_work_estimates(&structure, BuilderProgressProfile::Section, history).unwrap();
        assert_eq!((steps.steps().lower(), steps.steps().upper()), (0, 4));
        for (index, value) in expected.into_iter().enumerate() {
            assert_eq!(steps.get_step(index as i32), value, "Section slot {index}");
        }
        assert_eq!(pool_reference_snapshot(&structure), references);
    }
}
