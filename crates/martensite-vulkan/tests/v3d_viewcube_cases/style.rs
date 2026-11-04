//! `style`: the view cube without edges or vertices, then five styled cubes (colours,
//! transparency, font, rounded corners, padding) at the window corners and centre under a
//! rendering resolution scale of 2.
use super::prelude::*;
use crate::case_view::viewer_session::{FaceBoundary, corner_persistence};
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkservice::aspect::{
    AspectTypeOfFacingModel, AspectTypeOfTriedronPosition as Corner,
};
use martensite_tkservice::graphic3d::Graphic3dTransModeFlags;
use martensite_tkv3d::prs_mgr::{
    PrsMgrPresentableObject as _, PrsMgrPresentableObjectMembers as _,
};

const NO_EDGES: &str = "Error: Invalid display of View Cube without edges.";
const NO_VERTICES: &str = "Error: Invalid display of View Cube without vertices.";
const NO_BOTH: &str = "Error: Invalid display of View Cube without edges & vertices.";

/// `style`: three cube element switches, then the styled cubes.
pub(crate) fn styled_cubes() -> Outcome {
    banner(&[
        "==================================",
        "0028954: Visualization - implement interactive object AisViewCube for camera manipulations",
        "Display custom styled View Cube",
        "==================================",
    ]);
    let mut run = SessionRun::new()?;
    run.session().set_render_scale(1.)?;
    cube_parts(&mut run, true, false)?;
    expect_pixel(run.session(), (70, 295), "BLACK", true, NO_EDGES)?;
    run.dump("style_noedges.png")?;
    cube_parts(&mut run, false, true)?;
    expect_pixel(run.session(), (100, 308), "BLACK", true, NO_VERTICES)?;
    run.dump("style_noverts.png")?;
    cube_parts(&mut run, false, false)?;
    let session = run.session();
    let black = |(x, y): (i32, i32)| session.pixel_color_name(x, y).map(|name| name == "BLACK");
    if !black((70, 295))? || !black((100, 308))? {
        println!("{NO_BOTH}");
    }
    run.dump("style_noedgeandvert.png")?;
    run.session().clear()?;
    run.session().set_render_scale(2.)?;
    for cube in &STYLED {
        styled_cube(run.session(), cube)?;
    }
    run.dump("style_styles.png")?;
    run.finish()
}

/// The cube `vc` with its corner
/// vertices and edges shown or hidden; the first call creates it.
fn cube_parts(run: &mut SessionRun, vertices: bool, edges: bool) -> Outcome {
    let cube = run.session().view_cube("vc")?;
    cube.set_draw_edges(edges);
    cube.set_draw_vertices(vertices);
    run.session().display_object("vc", cube.cast())?;
    Ok(())
}

/// One styled cube of the second half: its options, its corner, and its
/// face boundaries (with optional width) when `boundary` is set.
struct StyledCube {
    name: &'static str,
    style: fn(&Handle<AisViewCube>) -> Outcome,
    corner: Corner,
    offset: [i32; 2],
    boundary: Option<FaceBoundary>,
}

const STYLED: [StyledCube; 5] = [
    StyledCube {
        name: "vc1",
        style: coloured,
        corner: Corner::LEFT_LOWER,
        offset: [100, 100],
        boundary: None,
    },
    StyledCube {
        name: "vc2",
        style: translucent,
        corner: Corner::LEFT_UPPER,
        offset: [100, 100],
        boundary: None,
    },
    StyledCube {
        name: "vc3",
        style: monospace,
        corner: Corner::RIGHT_LOWER,
        offset: [100, 100],
        boundary: Some(FaceBoundary {
            color: None,
            width: Some(2.),
            line_type: None,
        }),
    },
    StyledCube {
        name: "vc4",
        style: rounded,
        corner: Corner::RIGHT_UPPER,
        offset: [100, 100],
        boundary: Some(FaceBoundary {
            color: None,
            width: None,
            line_type: None,
        }),
    },
    StyledCube {
        name: "vc5",
        style: unpadded,
        corner: Corner::CENTER,
        offset: [0, 0],
        boundary: None,
    },
];

/// Create the cube with its options, display it at the trihedron `corner`, then the
/// optional face boundaries.
fn styled_cube(session: &mut ViewerSession, spec: &StyledCube) -> Outcome {
    let cube = session.view_cube(spec.name)?;
    (spec.style)(&cube)?;
    session.display_object(spec.name, cube.cast())?;
    let flags = Graphic3dTransModeFlags::TRIEDRON_PERS;
    session.redisplay_persistent(
        spec.name,
        corner_persistence(flags, spec.corner, spec.offset)?,
    )?;
    if let Some(style) = &spec.boundary {
        session.draw_face_boundaries(spec.name, style)?;
    }
    Ok(())
}

/// Box and text colours.
fn coloured(cube: &Handle<AisViewCube>) -> Outcome {
    cube.set_box_color(Color::from_hex("B0E0FF").ok_or("a hex colour")?)?;
    cube.set_text_color(Color::from_hex("00668A").ok_or("a hex colour")?);
    Ok(())
}

/// Half transparent.
fn translucent(cube: &Handle<AisViewCube>) -> Outcome {
    Ok(cube.set_transparency(0.5)?)
}

/// Default styles, then the side colour synchronized.
fn white_sides(cube: &Handle<AisViewCube>) -> Outcome {
    cube.reset_styles()?;
    let side = cube.box_side_style_ref().clone_handle();
    let side = side.get().ok_or("a box side style")?;
    side.set_color(
        Color::named(NamedColor::WHITE),
        AspectTypeOfFacingModel::BothSide,
    )?;
    cube.synchronize_aspects();
    Ok(())
}

/// Default styles, white sides and a 16 px monospace font.
fn monospace(cube: &Handle<AisViewCube>) -> Outcome {
    white_sides(cube)?;
    cube.set_font(&AsciiString::from("monospace"));
    cube.set_font_height(16.);
    Ok(())
}

/// Default styles, white sides, rounded corners (0.2) and box edge gap 2.
fn rounded(cube: &Handle<AisViewCube>) -> Outcome {
    white_sides(cube)?;
    cube.set_round_radius(0.2)?;
    cube.set_box_edge_gap(2.);
    Ok(())
}

/// Default styles, no facet extension and no axes padding.
fn unpadded(cube: &Handle<AisViewCube>) -> Outcome {
    cube.reset_styles()?;
    cube.set_box_facet_extension(0.);
    cube.set_axes_padding(0.);
    Ok(())
}
