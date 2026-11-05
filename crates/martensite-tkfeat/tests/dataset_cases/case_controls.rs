//! Refusals proved on real case bodies: each runs a whole case in its child process with one thing
//! added or changed, and the parent must not accept it.
use crate::support::{
    checkprops::{Expected, checkprops},
    construction::mkplane,
    feat_definitions::featdprism,
    feat_perform::{Kas, featperform_all},
    feat_script::FeatScript,
    hooks::{self, CaseGroup},
    process::verdict_of,
    properties::Prop,
    raster_session::RasterSession,
    restore::restore,
    rules::Verdict,
    script_error::run_script,
    sorted_explode::nexplode,
};
use martensite_tkg3d::top_abs::ShapeEnum;

fn verdict_with(
    identity: &str,
    group: &CaseGroup,
    body: fn(&mut FeatScript<'_>),
) -> Option<Verdict> {
    let run = || {
        let mut session = RasterSession::new(&identity.replace("::", "_"));
        let mut script = FeatScript::new(&mut session);
        run_script(|| {
            (group.begin)(&mut script);
            body(&mut script);
            (group.end)(&mut script);
        });
    };
    verdict_of(identity, group, run).unwrap_or_else(|failure| panic!("{identity}: {failure}"))
}

/// The inputs and feature of `featdprism/A1`, up to its check.
fn drafted_cylinder(s: &mut FeatScript<'_>) {
    restore(s, "cylindre.rle", "cyl");
    restore(s, "contour.rle", "cont");
    mkplane(s, "cont", "cont");
    nexplode(s, "cyl", ShapeEnum::Face);
    featdprism(s, "cyl", "cont", "cont", -10., 0, 1);
    featperform_all(s, Kas::DPrism, "result");
}

#[test]
fn a_correct_area_is_accepted() {
    let body = |s: &mut FeatScript<'_>| {
        drafted_cylinder(s);
        checkprops(s, "result", Prop::Area, Expected::Mass(93872.9));
    };
    let Some(verdict) = verdict_with(
        "case_controls::a_correct_area_is_accepted",
        &hooks::FEAT,
        body,
    ) else {
        return;
    };
    assert!(verdict.passes(), "{verdict:?}");
}

#[test]
fn a_wrong_area_fails_the_case() {
    let body = |s: &mut FeatScript<'_>| {
        drafted_cylinder(s);
        checkprops(s, "result", Prop::Area, Expected::Mass(90000.));
    };
    let Some(verdict) = verdict_with(
        "case_controls::a_wrong_area_fails_the_case",
        &hooks::FEAT,
        body,
    ) else {
        return;
    };
    assert!(!verdict.passes(), "{verdict:?}");
}

#[test]
fn a_missing_input_does_not_pass() {
    let body = |s: &mut FeatScript<'_>| restore(s, "bug27520_SplitShape1.brep", "shape");
    let Some(verdict) = verdict_with(
        "case_controls::a_missing_input_does_not_pass",
        &hooks::BUGS,
        body,
    ) else {
        return;
    };
    assert!(!verdict.passes(), "{verdict:?}");
}

#[test]
fn a_faulty_line_fails_the_case() {
    let body = |_: &mut FeatScript<'_>| println!("Faulty shapes in variables faulty_1 to faulty_2");
    let Some(verdict) = verdict_with(
        "case_controls::a_faulty_line_fails_the_case",
        &hooks::FEAT,
        body,
    ) else {
        return;
    };
    assert!(!verdict.passes(), "{verdict:?}");
}

#[test]
fn an_undefined_feature_stops_the_script() {
    let body = |s: &mut FeatScript<'_>| featperform_all(s, Kas::Prism, "result");
    let Some(verdict) = verdict_with(
        "case_controls::an_undefined_feature_stops_the_script",
        &hooks::FEAT,
        body,
    ) else {
        return;
    };
    assert!(!verdict.passes(), "{verdict:?}");
}
