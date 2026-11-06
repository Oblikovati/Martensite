//! `StepTidyCircleReducer` and `StepTidyLineReducer`: the same sharing kinds hold both curves.
use crate::step_tidy_fixture::{Fixture, name, one_item};
use martensite_tkdestep::step_geom::{
    StepGeomCurve, StepGeomLine, StepGeomPcurveOrSurface,
    StepGeomPreferredSurfaceCurveRepresentation, StepGeomSeamCurve, StepGeomSurfaceCurve,
    StepGeomTrimmedCurve, StepGeomTrimmingPreference, StepGeomTrimmingSelect,
};
use martensite_tkdestep::step_repr::{
    StepReprDefinitionalRepresentation, StepReprRepresentationContext,
};
use martensite_tkdestep::step_shape::{StepShapeEdgeCurve, StepShapeVertex};
use martensite_tkdestep::step_tidy::{StepTidyCircleReducer, StepTidyLineReducer};
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::Handle;

const CURVE_3D: StepGeomPreferredSurfaceCurveRepresentation =
    StepGeomPreferredSurfaceCurveRepresentation::PscrCurve3d;

fn add_edge_curve(fixture: &Fixture, curve: &Handle<StepGeomCurve>) {
    let edge = Handle::new(StepShapeEdgeCurve::new());
    let (start, end) = (
        Handle::new(StepShapeVertex::new()),
        Handle::new(StepShapeVertex::new()),
    );
    edge.init(&name(""), &start, &end, curve, true);
    fixture.add(&edge);
}

fn add_surface_curve(fixture: &Fixture, curve: &Handle<StepGeomCurve>) {
    let surface_curve = Handle::new(StepGeomSurfaceCurve::new());
    let geometry = HArray1::<StepGeomPcurveOrSurface>::default();
    surface_curve.init(&name(""), curve, &Handle::new(geometry), CURVE_3D);
    fixture.add(&surface_curve);
}

fn add_seam_curve(fixture: &Fixture, curve: &Handle<StepGeomCurve>) {
    let seam = Handle::new(StepGeomSeamCurve::new());
    let geometry = HArray1::<StepGeomPcurveOrSurface>::default();
    seam.init(&name(""), curve, &Handle::new(geometry), CURVE_3D);
    fixture.add(&seam);
}

fn add_trimmed_curve(fixture: &Fixture, curve: &Handle<StepGeomCurve>) {
    let trimmed = Handle::new(StepGeomTrimmedCurve::new());
    trimmed.init(
        &name(""),
        curve,
        &Handle::new(HArray1::<StepGeomTrimmingSelect>::default()),
        &Handle::new(HArray1::<StepGeomTrimmingSelect>::default()),
        true,
        StepGeomTrimmingPreference::TpUnspecified,
    );
    fixture.add(&trimmed);
}

fn definitional_representation(
    line: &Handle<StepGeomLine>,
) -> Handle<StepReprDefinitionalRepresentation> {
    let representation = Handle::new(StepReprDefinitionalRepresentation::new());
    representation.init(
        &name(""),
        &one_item(line.cast()),
        &Handle::new(StepReprRepresentationContext::new()),
    );
    representation
}

mod circle {
    use super::*;
    use crate::step_tidy_fixture::ReplacedEntities;

    fn replaced(fixture: &Fixture) -> ReplacedEntities {
        fixture.replaced_by(&mut StepTidyCircleReducer::new(&fixture.ws))
    }

    fn assert_merged_in(share: fn(&Fixture, &Handle<StepGeomCurve>)) {
        let fixture = Fixture::new();
        let (first, second) = (fixture.add_circle(""), fixture.add_circle(""));
        share(&fixture, &first.cast());
        share(&fixture, &second.cast());
        replaced(&fixture).assert_one_of(&first, &second);
    }

    #[test]
    fn different_names() {
        let fixture = Fixture::new();
        let (first, second) = (fixture.add_circle("Circle1"), fixture.add_circle("Circle2"));
        add_edge_curve(&fixture, &first.cast());
        add_edge_curve(&fixture, &second.cast());
        replaced(&fixture).assert_empty();
    }

    #[test]
    fn step_shape_edge_curve() {
        assert_merged_in(add_edge_curve);
    }

    #[test]
    fn step_geom_surface_curve() {
        assert_merged_in(add_surface_curve);
    }

    #[test]
    fn step_geom_seam_curve() {
        assert_merged_in(add_seam_curve);
    }
}

mod line {
    use super::*;
    use crate::step_tidy_fixture::ReplacedEntities;

    fn replaced(fixture: &Fixture) -> ReplacedEntities {
        fixture.replaced_by(&mut StepTidyLineReducer::new(&fixture.ws))
    }

    fn assert_merged_in(share: fn(&Fixture, &Handle<StepGeomCurve>)) {
        let fixture = Fixture::new();
        let (first, second) = (fixture.add_line(""), fixture.add_line(""));
        share(&fixture, &first.cast());
        share(&fixture, &second.cast());
        replaced(&fixture).assert_one_of(&first, &second);
    }

    #[test]
    fn different_names() {
        let fixture = Fixture::new();
        let (first, second) = (fixture.add_line("Line1"), fixture.add_line("Line2"));
        add_edge_curve(&fixture, &first.cast());
        add_edge_curve(&fixture, &second.cast());
        replaced(&fixture).assert_empty();
    }

    #[test]
    fn step_shape_edge_curve() {
        assert_merged_in(add_edge_curve);
    }

    #[test]
    fn step_geom_trimmed_curve() {
        assert_merged_in(add_trimmed_curve);
    }

    #[test]
    fn step_geom_surface_curve() {
        assert_merged_in(add_surface_curve);
    }

    /// adds only the first representation to the model, so the second line has no sharing
    /// entity at all and is still replaced.
    #[test]
    fn step_repr_definitional_representation() {
        let fixture = Fixture::new();
        let (first, second) = (fixture.add_line(""), fixture.add_line(""));
        fixture.add(&definitional_representation(&first));
        let _not_added = definitional_representation(&second);
        replaced(&fixture).assert_one_of(&first, &second);
    }

    #[test]
    fn step_geom_seam_curve() {
        assert_merged_in(add_seam_curve);
    }
}
