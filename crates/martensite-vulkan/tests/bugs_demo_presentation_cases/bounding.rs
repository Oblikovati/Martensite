//! The actual existing presentation box is shown/printed; input-solid bounds are not substituted.
use super::*;
use crate::primitive_shapes::sphere;
use martensite_tkernel::standard::exceptions::program_error;
use martensite_tkernel::storage::format_general;
use martensite_tkmath::bnd::BoundingBox3;
use martensite_tkservice::{
    aspect::AspectTypeOfHighlightMethod,
    graphic3d::{
        GRAPHIC3D_ZLAYER_ID_TOPMOST, Graphic3dPresentationAttributesSource as _,
        Graphic3dStructureBounds as _,
    },
};
use martensite_tkv3d::{
    ais::AisInteractiveObject, prs_mgr::PrsMgrPresentableObjectMembers as _, prs3d::Prs3dDrawer,
};
type Object = Handle<dyn AisInteractiveObject>;

fn sphere_geometry(session: &mut ViewerSession) -> Result<(), Failure> {
    session.clear()?;
    session.set_axonometric()?;
    session.set_display_mode(1)?;
    session.display("s", &sphere(2.)?, &Default::default())?;
    session.fit()
}

fn sphere_session() -> Result<SessionRun, Box<dyn std::error::Error>> {
    let mut run = SessionRun::new()?;
    sphere_geometry(run.session())?;
    Ok(run)
}

fn presentation_mode(session: &ViewerSession, object: &Object) -> Result<i32, Failure> {
    let manager = session.context().main_prs_mgr();
    for mode in [object.display_mode(), session.context().display_mode()] {
        if manager.has_presentation(&object.cast(), mode) {
            return Ok(mode);
        }
    }
    Err(program_error(
        "s presentation; expected an existing object/default display-mode presentation",
    ))
}

fn presentation_bounds(object: &Object, mode: i32) -> Result<[f64; 6], Failure> {
    let mut bounds = BoundingBox3::default();
    for presentation in object.presentations() {
        if let Some(presentation) = presentation.get().filter(|value| value.mode() == mode) {
            bounds = presentation.min_max_values(false);
        }
    }
    let low = bounds.corner_min()?;
    let high = bounds.corner_max()?;
    Ok([low.x(), low.y(), low.z(), high.x(), high.y(), high.z()])
}

fn named_sphere(session: &ViewerSession) -> Result<Object, Failure> {
    session
        .object("s")
        .cloned()
        .ok_or_else(|| program_error("s object; expected the source displayed sphere"))
}

fn bounding_style() -> Handle<Prs3dDrawer> {
    let style = Handle::new(Prs3dDrawer::default());
    let drawer = style
        .get()
        .expect("bounding drawer; expected a live new drawer");
    drawer.set_method(AspectTypeOfHighlightMethod::Boundbox);
    drawer.set_color(Color::named(NamedColor::GRAY99));
    style
}

fn show_bounds(session: &ViewerSession) -> Result<(), Failure> {
    let object = named_sphere(session)?;
    let mode = presentation_mode(session, &object)?;
    session.update_context(|context| {
        context.main_prs_mgr().color(
            &object.cast(),
            &bounding_style(),
            mode,
            &Handle::default(),
            GRAPHIC3D_ZLAYER_ID_TOPMOST,
        )
    })
}

const BOUNDING_BANNER: &[&str] = &["============", "CR24873", "============", ""];

/// The source's actual bounding-box highlight and white-pixel predicate; `shown_sphere_bounds()?`.
pub(crate) fn shown_sphere_bounds() -> Outcome {
    banner(BOUNDING_BANNER);
    let mut run = sphere_session()?;
    show_bounds(run.session())?;
    run.session().check_color(378, 102, [1., 1., 1.])?;
    run.dump("bug24873_1.png")?;
    run.finish()
}

fn bounds_report(values: [f64; 6]) -> String {
    let values = values.map(|value| format_general(value, 17));
    format!("s\n{}\n", values.join(" "))
}

fn parse_bounds(report: &str) -> Result<[f64; 6], Box<dyn std::error::Error>> {
    let line = report
        .strip_prefix("s\n")
        .ok_or_else(|| format!("report {report:?}; expected s then six reals"))?;
    let values = line
        .split_whitespace()
        .map(str::parse::<f64>)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|cause| format!("report {report:?}; expected six numeric coordinates: {cause}"))?;
    values.try_into().map_err(|values: Vec<f64>| {
        format!("report {report:?} values {values:?}; expected exactly six reals").into()
    })
}

fn print_bounds(session: &ViewerSession) -> Result<String, Failure> {
    let object = named_sphere(session)?;
    let mode = presentation_mode(session, &object)?;
    Ok(bounds_report(presentation_bounds(&object, mode)?))
}

/// The six source checkreal calls consume the actual printed presentation bounds; `printed_sphere_bounds()?`.
pub(crate) fn printed_sphere_bounds() -> Outcome {
    banner(BOUNDING_BANNER);
    let mut run = sphere_session()?;
    let report = print_bounds(run.session())?;
    let values = parse_bounds(&report)?;
    check_bounds(values);
    run.dump("bug24873_2.png")?;
    run.finish()
}

fn check_bounds(values: [f64; 6]) {
    let names = ["x1", "y1", "z1", "x2", "y2", "z2"];
    let expected = [-2., -2., -2., 2., 2., 2.];
    for ((name, value), expected) in names.into_iter().zip(values).zip(expected) {
        crate::shape_mesh::check_real(name, value, expected, 0.1, 0.1);
    }
}

#[test]
fn bounding_report_preserves_full_di_precision_and_refuses_missing_coordinates() {
    assert_eq!(
        bounds_report([0.1, -2., -2., 2., 2., 2.]),
        "s\n0.10000000000000001 -2 -2 2 2 2\n"
    );
    assert_eq!(
        parse_bounds("s\n-2 -2 -2 2 2 2\n").unwrap(),
        [-2., -2., -2., 2., 2., 2.]
    );
    assert!(parse_bounds("s\n-2 -2 -2 2 2\n").is_err());
    assert!(parse_bounds("other\n-2 -2 -2 2 2 2\n").is_err());
}

#[cfg(test)]
fn null_sphere_session() -> ViewerSession {
    let driver = martensite_tkservice::null_driver::NullGraphicDriver::new_handle();
    let mut session = ViewerSession::with_graphic_driver(&driver, 409, 409).unwrap();
    sphere_geometry(&mut session).unwrap();
    session
}

#[test]
fn bounding_show_keeps_the_actual_shaded_presentation_and_boundbox_style() {
    let session = null_sphere_session();
    let object = named_sphere(&session).unwrap();
    let mode = presentation_mode(&session, &object).unwrap();
    assert_eq!(mode, 1);
    show_bounds(&session).unwrap();
    let presentation = object
        .presentations()
        .into_iter()
        .find(|value| value.mode() == mode)
        .unwrap();
    assert_boundbox_style(&presentation);
}

#[cfg(test)]
fn assert_boundbox_style(presentation: &martensite_tkv3d::prs_mgr::PrsMgrPresentation) {
    use martensite_tkservice::graphic3d::{
        Graphic3dPresentationAttributesMembers as _, Graphic3dStructureMembers as _,
        Graphic3dStructureSource as _,
    };
    assert_eq!(
        presentation.highlight_style().method(),
        AspectTypeOfHighlightMethod::Boundbox
    );
    assert_eq!(
        presentation.highlight_style().color(),
        Color::named(NamedColor::GRAY99)
    );
    assert!(presentation.is_highlighted());
}

#[test]
fn bounding_print_observes_presentation_storage_instead_of_the_input_sphere() {
    use martensite_tkservice::graphic3d::{
        Graphic3dGroupMembers as _, Graphic3dStructureMembers as _,
    };
    let session = null_sphere_session();
    let object = named_sphere(&session).unwrap();
    let presentation = object
        .presentations()
        .into_iter()
        .find(|value| value.mode() == 1)
        .unwrap();
    let fixture_bounds = [-7., -5., -3., 11., 13., 17.];
    // An owned negative fixture changes only the displayed group bounds, not the source sphere.
    for group in presentation.groups() {
        group.set_min_max_values(fixture_bounds);
    }
    let actual = parse_bounds(&print_bounds(&session).unwrap()).unwrap();
    assert_eq!(actual, fixture_bounds);
    check_bounds([-2., -2., -2., 2., 2., 2.]);
}
