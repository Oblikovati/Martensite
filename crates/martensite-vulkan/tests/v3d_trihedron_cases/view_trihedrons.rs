//! The view trihedron (z-buffered or wireframe, at a window corner) and the graduated
//! trihedron around the displayed objects.
use super::prelude::*;
use crate::case_view::viewer_session::ViewerSession;
use martensite_tkernel::quantity::ColorSpace;
use martensite_tkernel::standard::Failure;
use martensite_tkernel::tcollection::{AsciiString, ExtendedString};
use martensite_tkservice::aspect::AspectTypeOfTriedronPosition as Corner;
use martensite_tkservice::graphic3d::Graphic3dGraduatedTrihedron;

pub(super) use crate::case_view::viewer_session::ViewTrihedronSettings as ViewTrihedron;

/// Show the view trihedron with `settings`; `view_trihedron(&mut session, &settings)?`.
pub(super) fn view_trihedron(session: &mut ViewerSession, settings: &ViewTrihedron) -> Outcome {
    Ok(session.show_view_trihedron(settings)?)
}

/// A colour of three real components, read as the command reads them (float, linear RGB).
fn linear(r: f32, g: f32, b: f32) -> Result<Color, Failure> {
    Color::new(f64::from(r), f64::from(g), f64::from(b), ColorSpace::Rgb)
}

/// `bug22483`: user-defined label colours of the view trihedron are kept.
pub(crate) fn view_trihedron_positions() -> Outcome {
    banner(&[
        "============",
        "OCC22483: ZBufferTrihedron loses user-defined color",
        "============",
        "",
    ]);
    let mut run = SessionRun::new()?;
    view_trihedron(run.session(), &ViewTrihedron::default())?;
    run.dump("bug22483_1.png")?;
    view_trihedron(run.session(), &ViewTrihedron::default())?;
    run.dump("bug22483_2.png")?;
    let upper = ViewTrihedron {
        position: Corner::RIGHT_UPPER,
        scale: 0.2,
        labels: [linear(0.78, 0.20, 1.0)?; 3],
        ..Default::default()
    };
    view_trihedron(run.session(), &upper)?;
    run.dump("bug22483_3.png")?;
    let centre = ViewTrihedron {
        position: Corner::CENTER,
        wireframe: true,
        labels: [linear(0.0, 0.59, 1.0)?; 3],
        ..Default::default()
    };
    view_trihedron(run.session(), &centre)?;
    run.dump("bug22483_4.png")?;
    run.finish()
}

/// `bug23484_1`: the wireframe view trihedron at a corner and at the centre.
pub(crate) fn wireframe_view_trihedron() -> Outcome {
    banner(&[
        "============",
        "OCC23484: Visualization, Renderer - primitive arrays to become the only way to render geometry",
        "Tests wireframe trihedron rendering with primitive arrays",
        "============",
        "",
    ]);
    let mut run = SessionRun::new()?;
    run.session().fit()?;
    run.session().set_axonometric()?;
    let wire = ViewTrihedron {
        wireframe: true,
        ..Default::default()
    };
    view_trihedron(run.session(), &wire)?;
    run.dump("bug23484_1_1.png")?;
    view_trihedron(
        run.session(),
        &ViewTrihedron {
            position: Corner::CENTER,
            ..wire
        },
    )?;
    run.dump("bug23484_1_2.png")?;
    run.finish()
}

/// `bug27832`: the view trihedron keeps its corner when the view is scaled per axis.
pub(crate) fn scaled_view_trihedron() -> Outcome {
    banner(&[
        "========",
        "0027832: Visualization - Scaled zbuffer trihedron",
        "========",
        "",
    ]);
    let mut run = SessionRun::new()?;
    run.session().display(
        "b",
        &box_sized([10., 10., 10.])?,
        &DisplayRequest::default(),
    )?;
    run.session().fit()?;
    run.session().set_display_mode(1)?;
    view_trihedron(run.session(), &ViewTrihedron::default())?;
    run.dump("bug27832_unscaled.png")?;
    // The axial scale command changes the camera only; the zoom that follows redraws.
    run.session()
        .view()
        .view_base()
        .set_axial_scale(1., 3., 10.)?;
    run.session().zoom(1.0)?;
    for (x, y, expected) in [(55, 360, "RED3"), (50, 350, "BLUE3"), (55, 355, "GREEN3")] {
        let colour = run.session().pixel_color_name(x, y)?;
        if colour != expected {
            println!("ERROR: trihedron does not maintain position");
            println!("       additional investigation is needed");
            println!("       expected color is: {expected}");
            println!("       current color is:  {colour}");
        }
    }
    run.dump("bug27832_scaled.png")?;
    run.finish()
}

/// A shaded sphere of radius 10 in an axonometric, fitted view.
fn shaded_sphere(run: &mut SessionRun) -> Outcome {
    run.session()
        .display("s", &sphere(10.)?, &DisplayRequest::default())?;
    run.session().set_axonometric()?;
    run.session().fit()?;
    run.session().set_object_display_mode("s", 1)?;
    Ok(())
}

/// Show the graduated trihedron of `data` and update the viewer; `graduated(&s, &data)?`.
fn graduated(session: &ViewerSession, data: &Graphic3dGraduatedTrihedron) -> Outcome {
    Ok(session.update_view(|view| {
        view.graduated_trihedron_display(data);
        session.context().update_current_viewer();
        Ok(())
    })?)
}

/// `bug25611_1`: the graduated trihedron and the view trihedron shown together.
pub(crate) fn graduated_sphere() -> Outcome {
    banner(&[
        "========",
        "OCC25611: Displaying zbufftrihedron interferes with vgraduatedtrihedron",
        "========",
        "",
    ]);
    let mut run = SessionRun::new()?;
    shaded_sphere(&mut run)?;
    graduated(run.session(), &Graphic3dGraduatedTrihedron::default())?;
    view_trihedron(run.session(), &ViewTrihedron::default())?;
    run.dump("bug25611_1.png")?;
    run.finish()
}

/// `bug25611_2`: graduated trihedron names, colours, fonts, ticks, offsets and arrows.
pub(crate) fn graduated_options() -> Outcome {
    banner(&[
        "========",
        "OCC25611: Displaying 'zbufftrihedron' interferes with 'vgraduatedtrihedron'",
        "Test graduated trihedron parameters",
        "========",
        "",
    ]);
    let mut run = SessionRun::new()?;
    shaded_sphere(&mut run)?;
    graduated(run.session(), &named_axes())?;
    run.dump("bug25611_2_1.png")?;
    let mut hidden = Graphic3dGraduatedTrihedron::default();
    hidden.axes[0].to_draw_name = false;
    hidden.axes[1].to_draw_values = false;
    hidden.axes[2].to_draw_tickmarks = false;
    graduated(run.session(), &hidden)?;
    run.dump("bug25611_2_2.png")?;
    let mut ticks = Graphic3dGraduatedTrihedron::default();
    let x = &mut ticks.axes[0];
    (x.tickmarks_number, x.tickmarks_length) = (10, 15);
    (x.values_offset, x.name_offset) = (20, 40);
    graduated(run.session(), &ticks)?;
    run.dump("bug25611_2_3.png")?;
    let arrows = Graphic3dGraduatedTrihedron {
        arrows_length: 50.,
        ..Default::default()
    };
    graduated(run.session(), &arrows)?;
    run.dump("bug25611_2_4.png")?;
    run.finish()
}

/// Axis names X(mm), Y(mm), Z(mm), a pink X name, a green-yellow Y axis,
/// Courier for names and values.
fn named_axes() -> Graphic3dGraduatedTrihedron {
    let mut data = Graphic3dGraduatedTrihedron::default();
    for (axis, name) in data.axes.iter_mut().zip(["X(mm)", "Y(mm)", "Z(mm)"]) {
        axis.name = ExtendedString::from_ascii_string(&AsciiString::from(name), true);
    }
    data.axes[0].name_color = Color::named(NamedColor::PINK);
    data.axes[1].color = Color::named(NamedColor::GREENYELLOW);
    data.names_font = AsciiString::from("Courier");
    data.values_font = AsciiString::from("Courier");
    data
}
