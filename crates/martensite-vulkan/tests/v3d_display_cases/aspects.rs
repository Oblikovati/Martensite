//! Interior and edge aspects of a shaded box changed after display.
use super::prelude::*;
use crate::case_view::viewer_session::SurfaceAspects;
use martensite_tkernel::quantity::ColorRgba;
use martensite_tkservice::aspect::{AspectInteriorStyle, AspectTypeOfLine};

const EDGE_HEADING: [&str; 4] = [
    "============",
    "Changing shaded-box edge styles",
    "============",
    "",
];

/// A shaded unit box, then dashed green face edges, then a hollow interior.
/// Example: `edge_type_and_hollow_interior()?`.
pub(crate) fn edge_type_and_hollow_interior() -> Result<(), Box<dyn Error>> {
    banner(&EDGE_HEADING);
    let mut run = SessionRun::new()?;
    display_shaded_box(&mut run)?;
    restyle(&mut run, solid(None))?;
    run.dump("bug6897_1_3.png")?;
    // Preserve the authored sRGB hex colour without alpha.
    let green = ColorRgba::from_hex("64FF64", true).expect("a six-digit sRGB hex colour");
    restyle(&mut run, solid(Some(green)))?;
    run.dump("bug6897_1_1.png")?;
    restyle(&mut run, hollow())?;
    run.dump("bug6897_1_2.png")?;
    run.finish()
}

fn display_shaded_box(run: &mut SessionRun) -> Result<(), Box<dyn Error>> {
    run.session().set_axonometric()?;
    let shaded = DisplayRequest {
        display_mode: Some(1),
        ..DisplayRequest::default()
    };
    run.session()
        .display("b", &box_sized([1., 1., 1.])?, &shaded)?;
    Ok(run.session().fit()?)
}

fn hollow() -> SurfaceAspects {
    SurfaceAspects {
        // A hollow interior shares the empty-interior rendering policy.
        interior_style: Some(AspectInteriorStyle::Empty),
        draw_edges: Some(true),
        ..SurfaceAspects::default()
    }
}

/// Solid interior, with dashed edges in `edges` when given.
fn solid(edges: Option<ColorRgba>) -> SurfaceAspects {
    SurfaceAspects {
        interior_style: Some(AspectInteriorStyle::Solid),
        draw_edges: edges.map(|_| true),
        edge_type: edges.map(|_| AspectTypeOfLine::Dash),
        edge_color: edges,
        ..SurfaceAspects::default()
    }
}

fn restyle(run: &mut SessionRun, aspects: SurfaceAspects) -> Result<(), Box<dyn Error>> {
    Ok(run.session().set_surface_aspects("b", &aspects)?)
}
