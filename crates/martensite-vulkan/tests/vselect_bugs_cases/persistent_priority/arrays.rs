//! Triangle, segment and filled-circle detection phases of the whole array original.
use super::commands::{owner_priority, persistent_existing, white_dump};
use super::*;
use martensite_tkv3d::ais::AisSelectionScheme;

/// Retain array geometry after removing source point operands; `triangle_scene(session)?`.
pub(super) fn triangle_scene(session: &mut ViewerSession) -> Result<(), Failure> {
    objects::points(session, &["pt1", "pt2", "pt3", "pt4"], &objects::CORNERS)?;
    objects::triangle(session, "tri1", ["pt1", "pt2", "pt3"])?;
    objects::triangle(session, "tri2", ["pt3", "pt4", "pt1"])?;
    persistent_existing(session, "tri1", Some(1))?;
    persistent_existing(session, "tri2", Some(1))?;
    material_color(session, "tri1", NamedColor::ORANGE)?;
    material_color(session, "tri2", NamedColor::ORANGE)?;
    session.remove(&["pt1", "pt2", "pt3", "pt4"])?;
    face_boundaries(session, "tri1")?;
    face_boundaries(session, "tri2")
}

fn segment_scene(session: &mut ViewerSession) -> Result<(), Failure> {
    for (name, coords) in objects::SEG_NAMES.into_iter().zip(&objects::SEGMENTS) {
        objects::display_array(session, name, objects::segment_array(coords)?)?;
    }
    for name in objects::SEG_NAMES {
        persistent_existing(session, name, Some(1))?;
    }
    for name in objects::SEG_NAMES {
        session.set_material(name, commands::PLASTIC)?;
    }
    for name in objects::SEG_NAMES {
        session.set_color(name, Color::named(NamedColor::RED))?;
    }
    Ok(())
}

/// Remove only temporary points after the filled circle retains geometry; `circle_scene(session)?`.
pub(super) fn circle_scene(session: &mut ViewerSession) -> Result<(), Failure> {
    let points = [[170., -145., 0.], [180., -145., 0.], [180., -155., 0.]];
    objects::points(session, &["pr1", "pr2", "pr3"], &points)?;
    objects::filled_circle(session)?;
    persistent_existing(session, "circle1", None)?;
    material_color(session, "circle1", NamedColor::BROWN3)?;
    session.remove(&["pr1", "pr2", "pr3"])?;
    face_boundaries(session, "circle1")
}

fn default_segment_triangle(run: &mut SessionRun) -> Outcome {
    run.dump("bug30484_2_quad_segments_circle.png")?;
    white_dump(
        run,
        (75, 150),
        (75, 100),
        "Error: segment should be highlighted",
        "bug30484_2_undefined_segmenthighlight.png",
    )?;
    white_dump(
        run,
        (100, 150),
        (100, 100),
        "Error: tri2 should be highlighted",
        "bug30484_2_undefined_tri2highlight.png",
    )?;
    Ok(())
}

fn default_triangle_circle(run: &mut SessionRun) -> Outcome {
    white_dump(
        run,
        (200, 150),
        (200, 200),
        "Error: tri1 should be highlighted",
        "bug30484_2_undefined_tri1highlight.png",
    )?;
    run.session().move_cursor(175, 150)?;
    run.dump("bug30484_2_undefined_circle.png")
}

fn default_arrays(run: &mut SessionRun) -> Outcome {
    default_segment_triangle(run)?;
    default_triangle_circle(run)
}

fn priorities(
    session: &ViewerSession,
    triangle: i32,
    segment: i32,
    circle: i32,
) -> Result<(), Failure> {
    owner_priority(session, "tri1", triangle)?;
    owner_priority(session, "tri2", triangle)?;
    for name in objects::SEG_NAMES {
        owner_priority(session, name, segment)?;
    }
    owner_priority(session, "circle1", circle)
}

fn triangle_focus(run: &mut SessionRun) -> Outcome {
    priorities(run.session(), 6, 4, 4)?;
    white_dump(
        run,
        (175, 150),
        (200, 200),
        "Error: tri1 should be highlighted",
        "bug30484_2_quad_focused1.png",
    )?;
    white_dump(
        run,
        (170, 140),
        (100, 100),
        "Error: tri2 should be highlighted",
        "bug30484_2_quad_focused2.png",
    )
}

fn circle_focus(run: &mut SessionRun) -> Outcome {
    run.session().move_cursor(0, 0)?;
    priorities(run.session(), 4, 4, 6)?;
    run.dump("bug30484_2_circle_focused.png")?;
    run.session().move_cursor(175, 150)?;
    run.session().select_detected(AisSelectionScheme::Replace)?;
    run.dump("bug30484_2_circle_selected.png")
}

fn segment_focus(run: &mut SessionRun) -> Outcome {
    run.session().move_cursor(0, 0)?;
    run.session().select_detected(AisSelectionScheme::Replace)?;
    priorities(run.session(), 4, 6, 4)?;
    for (xy, image) in [
        ((175, 250), "bug30484_2_segment1_focused.png"),
        ((276, 150), "bug30484_2_segment2_focused.png"),
        ((175, 49), "bug30484_2_segment3_focused.png"),
        ((75, 150), "bug30484_2_segment4_focused.png"),
    ] {
        run.session().move_cursor(xy.0, xy.1)?;
        run.dump(image)?;
    }
    Ok(())
}

/// Complete triangle/segments/circle priority original; `priority_arrays()`.
pub(crate) fn priority_arrays() -> Outcome {
    let mut run = priority_begin()?;
    triangle_scene(run.session())?;
    segment_scene(run.session())?;
    circle_scene(run.session())?;
    default_arrays(&mut run)?;
    triangle_focus(&mut run)?;
    circle_focus(&mut run)?;
    segment_focus(&mut run)?;
    run.finish()
}
