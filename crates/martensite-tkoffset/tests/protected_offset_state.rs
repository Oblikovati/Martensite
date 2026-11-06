//! Extension adapters retain the algorithm selection and progress-weight contracts.
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::Failure;
use martensite_tkmath::geom_abs::JoinType;
use martensite_tkoffset::brep_offset::{BRepOffsetMakeOffset, MakeOffsetOptions};
use martensite_tkoffset::brep_offset_api::{BRepOffsetAPIMakeOffsetShape, OffsetAlgo};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;

#[test]
fn default_extension_state_has_no_algorithm_or_result() {
    let maker = BRepOffsetAPIMakeOffsetShape::new();
    assert_eq!(maker.my_last_used_algo, OffsetAlgo::None);
    assert!(!maker.my_offset_shape.is_done());
    assert!(!maker.my_simple_offset_shape.is_done());
    assert!(!maker.is_done());
}

#[test]
fn extension_state_can_select_each_existing_algorithm() {
    let mut maker = BRepOffsetAPIMakeOffsetShape::new();
    for algorithm in [OffsetAlgo::Join, OffsetAlgo::Simple, OffsetAlgo::None] {
        maker.my_last_used_algo = algorithm;
        assert_eq!(maker.my_last_used_algo, algorithm);
    }
    maker.my_simple_offset_shape.set_offset_value(2.5);
    assert_eq!(maker.my_simple_offset_shape.offset_value(), 2.5);
    assert!(!maker.is_done());
}

#[test]
fn arc_progress_has_exact_relative_weights() -> Result<(), Failure> {
    let maker = BRepOffsetMakeOffset::new();
    let mut steps = [f64::NAN; 8];
    maker.analyze_progress(83., &mut steps)?;
    assert_eq!(steps, [1., 2., 20., 50., 0., 5., 5., 0.]);
    Ok(())
}

#[test]
fn thick_intersection_progress_adds_walls_and_swaps_build_intersection_weights()
-> Result<(), Failure> {
    let mut maker = BRepOffsetMakeOffset::new();
    let options = MakeOffsetOptions {
        join: JoinType::Intersection,
        thickening: true,
        ..MakeOffsetOptions::default()
    };
    maker.initialize(&Shape::default(), 1., 1e-7, options);
    let mut steps = [f64::NAN; 8];
    maker.analyze_progress(88., &mut steps)?;
    assert_eq!(steps, [1., 2., 50., 20., 5., 5., 5., 0.]);
    Ok(())
}

#[test]
fn zero_whole_progress_resets_every_output_stage() -> Result<(), Failure> {
    let maker = BRepOffsetMakeOffset::new();
    let mut steps = [f64::NAN; 8];
    maker.analyze_progress(0., &mut steps)?;
    assert_eq!(steps, [0.; 8]);
    Ok(())
}
