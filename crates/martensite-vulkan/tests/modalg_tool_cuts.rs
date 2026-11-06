//! Rigid placement of the genuine source tool preserves its independently measured volume.
use crate::modalg_display_cases as cases;
use martensite_tkbrep::topods::Shape;
use martensite_tkgeombase::gprop::GProps;
use martensite_tktopalgo::brep_gprop::BRepGProp;

#[test]
fn actual_source_tool_placements_preserve_rigid_volume() {
    let input = crate::dataset_shape::read_dataset_shape("bug29807-tool.brep").unwrap();
    let original = volume(&input);
    assert!(original > 0.);
    for source in [
        cases::SourceCut::Tilted,
        cases::SourceCut::RaisedTodo,
        cases::SourceCut::Tightened,
    ] {
        let placed = cases::source_tool(source).unwrap();
        assert!(!placed.is_null());
        assert!((volume(&placed) - original).abs() < 1e-8 * original.abs().max(1.));
    }
}

fn volume(shape: &Shape) -> f64 {
    let mut props = GProps::new();
    BRepGProp::volume_properties(shape, &mut props, false, false, false).unwrap();
    props.mass()
}
