//! Object trihedron options: display mode, placement, axis lengths and radii, part colours,
//! drawn axes and hidden labels or arrows, applied in the order the trihedron command does.
use super::prelude::*;
use crate::case_view::viewer_session::ViewerSession;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::geom::{GeomAxis2Placement, GeomAxisPlacement as _};
use martensite_tkmath::gp::{Axis2, Direction3, Point3};
use martensite_tkservice::graphic3d::Graphic3dPresentationAttributesSource as _;
use martensite_tkv3d::ais::{AisTrihedron, AisTrihedronSource as _};
use martensite_tkv3d::prs_mgr::{
    PrsMgrPresentableObject as _, PrsMgrPresentableObjectMembers as _,
};
use martensite_tkv3d::prs3d::{
    Prs3dDatumAspect, Prs3dDatumAttribute as Attr, Prs3dDatumAxes, Prs3dDatumMode,
    Prs3dDatumParts as Part,
};

/// A trihedron on the default XOY placement at the origin; `new_trihedron()?`.
pub(super) fn new_trihedron() -> Result<Handle<AisTrihedron>, Failure> {
    let frame = Axis2::with_x_direction(
        Point3::new(0., 0., 0.),
        Direction3::new(0., 0., 1.)?,
        Direction3::new(1., 0., 0.)?,
    )?;
    AisTrihedron::new(&Handle::new(GeomAxis2Placement::new(frame)))
}

/// The trihedron's own datum aspect, created on first use; `own_datum_aspect(&t)?`.
pub(super) fn own_datum_aspect(
    trihedron: &AisTrihedron,
) -> Result<Handle<Prs3dDatumAspect>, Failure> {
    let drawer = trihedron.attributes();
    if !drawer.has_own_datum_aspect() {
        drawer.set_datum_aspect(Handle::new(Prs3dDatumAspect::new()?));
    }
    Ok(drawer.datum_aspect())
}

/// Apply `options` to the trihedron `name` (created at the origin when unbound), then
/// redisplay it, or display it the first time; `trihedron(&mut s, "t1", |t| ...)?`.
fn trihedron(
    session: &mut ViewerSession,
    name: &str,
    options: impl FnOnce(&AisTrihedron) -> Result<(), Failure>,
) -> Outcome {
    let bound = session
        .object(name)
        .map(|object| object.cast::<AisTrihedron>());
    let existing = bound.filter(|t| !t.is_null());
    let object = match &existing {
        Some(trihedron) => trihedron.clone(),
        None => new_trihedron()?,
    };
    options(object.get().ok_or("a trihedron object")?)?;
    if existing.is_some() {
        let cast = object.cast();
        return Ok(session.update_context(|c| c.redisplay(&cast, true, false))?);
    }
    Ok(session.display_object(name, object.cast())?)
}

fn set_origin(trihedron: &AisTrihedron, [x, y, z]: [f64; 3]) {
    let component = trihedron.component();
    if let Some(placement) = component.get() {
        placement.set_location(Point3::new(x, y, z));
    }
}

fn set_axes(trihedron: &AisTrihedron, x: [f64; 3], z: [f64; 3]) -> Result<(), Failure> {
    let component = trihedron.component();
    let Some(placement) = component.get() else {
        return Ok(());
    };
    let direction = |[a, b, c]: [f64; 3]| Direction3::new(a, b, c);
    placement.set_ax2(Axis2::with_x_direction(
        placement.location(),
        direction(z)?,
        direction(x)?,
    )?);
    Ok(())
}

fn set_attributes(
    trihedron: &AisTrihedron,
    attributes: &[Attr],
    value: f64,
) -> Result<(), Failure> {
    let aspect = own_datum_aspect(trihedron)?;
    for &attribute in attributes.iter().rev() {
        if let Some(aspect) = aspect.get() {
            aspect.set_attribute(attribute, value);
        }
    }
    Ok(())
}

/// Set the colour of `parts` and the text colour of `text`, parts applied last first.
fn colour_axis(
    trihedron: &AisTrihedron,
    parts: &[Part],
    text: Part,
    colour: Color,
) -> Result<(), Failure> {
    for &part in parts.iter().rev() {
        trihedron.set_datum_part_color(part, colour)?;
    }
    trihedron.set_text_color_part(text, colour)
}

/// `bug27958`: shaded and wireframe trihedrons with custom placement, sizes and colours.
pub(crate) fn trihedron_options() -> Outcome {
    banner(&[
        "===========",
        "OCC27958: Visualization - add a shading mode for AisTrihedron",
        "===========",
        "",
    ]);
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    super::view_trihedrons::view_trihedron(run.session(), &labelled_view_trihedron())?;
    trihedron(run.session(), "t0", |_| Ok(()))?;
    trihedron(run.session(), "t1", |_| Ok(()))?;
    let red = Color::named(NamedColor::RED);
    if let Some(style) = run.session().context().selection_style().get() {
        style.set_color(red);
    }
    selectable_parts(run.session(), "t1")?;
    shaded_custom_trihedron(run.session())?;
    trihedron(run.session(), "t2", |t| {
        set_origin(t, [0., 0., -300.]);
        Ok(())
    })?;
    let peach = Color::named(NamedColor::PEACHPUFF);
    trihedron(run.session(), "t2", |t| t.set_color(peach))?;
    selectable_parts(run.session(), "t2")?;
    two_axis_trihedrons(run.session())?;
    run.session().fit()?;
    run.dump("bug27958.png")?;
    run.finish()
}

/// A view trihedron with labels coloured red, green and blue.
fn labelled_view_trihedron() -> super::view_trihedrons::ViewTrihedron {
    super::view_trihedrons::ViewTrihedron {
        labels: [NamedColor::RED, NamedColor::GREEN, NamedColor::BLUE1].map(Color::named),
        ..Default::default()
    }
}

/// Activate selection modes 1, 2 and 3 on `name`.
fn selectable_parts(session: &mut ViewerSession, name: &str) -> Outcome {
    for mode in 1..=3 {
        session.set_object_selection_mode(name, mode, true)?;
    }
    Ok(())
}

/// The successive trihedron changes of `bug27958`.
fn shaded_custom_trihedron(session: &mut ViewerSession) -> Outcome {
    trihedron(session, "t1", |t| {
        t.set_datum_display_mode(Prs3dDatumMode::Shaded);
        Ok(())
    })?;
    trihedron(session, "t1", |t| {
        set_origin(t, [-200., -200., -300.]);
        Ok(())
    })?;
    let lengths = [Attr::XAxisLength, Attr::YAxisLength, Attr::ZAxisLength];
    trihedron(session, "t1", |t| set_attributes(t, &lengths, 80.))?;
    trihedron(session, "t1", |t| {
        set_axes(t, [40., 60., 120.], [-120., 0., 40.])
    })?;
    let tube = [Attr::ShadingTubeRadiusPercent];
    trihedron(session, "t1", |t| set_attributes(t, &tube, 0.03))?;
    let radii = [
        Attr::ShadingConeRadiusPercent,
        Attr::ShadingOriginRadiusPercent,
    ];
    trihedron(session, "t1", |t| set_attributes(t, &radii, 0.05))?;
    let (red, green, blue) = (NamedColor::RED, NamedColor::GREEN, NamedColor::BLUE1);
    let x = [Part::XAxis, Part::XArrow];
    trihedron(session, "t1", |t| {
        colour_axis(t, &x, Part::XAxis, Color::named(red))
    })?;
    let y = [Part::YAxis, Part::YArrow];
    trihedron(session, "t1", |t| {
        colour_axis(t, &y, Part::YAxis, Color::named(green))
    })?;
    let z = [Part::ZAxis, Part::ZArrow, Part::Origin];
    trihedron(session, "t1", |t| {
        colour_axis(t, &z, Part::ZAxis, Color::named(blue))
    })
}

/// Trihedrons `t3` (shaded red, XY axes, no labels) and `t4` (arrows hidden).
fn two_axis_trihedrons(session: &mut ViewerSession) -> Outcome {
    trihedron(session, "t3", |t| {
        set_origin(t, [200., 200., -300.]);
        t.set_datum_display_mode(Prs3dDatumMode::Shaded);
        t.set_color(Color::named(NamedColor::RED))
    })?;
    trihedron(session, "t3", |t| {
        let aspect = own_datum_aspect(t)?;
        let aspect = aspect
            .get()
            .ok_or_else(|| Failure::with_message("Standard_NullObject", "datum aspect"))?;
        aspect.set_draw_labels(false);
        aspect.set_draw_datum_axes(Prs3dDatumAxes::XYAxes);
        Ok(())
    })?;
    trihedron(session, "t4", |t| {
        set_origin(t, [-200., -200., 300.]);
        if let Some(aspect) = own_datum_aspect(t)?.get() {
            aspect.set_draw_arrows(false);
        }
        Ok(())
    })
}
