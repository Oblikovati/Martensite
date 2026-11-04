//! Execute the real ray selection before displaying non-selectable pick annotations.
use super::datum_objects::{point, ray_axis};
use crate::session_run::SessionRun;
use martensite_tkernel::quantity::NamedColor;
use martensite_tkservice::aspect::AspectTypeOfMarker;
use std::error::Error;

type Intersection = ([f64; 3], [f32; 3], f64);

pub(crate) struct AxisReply {
    pub(crate) text: String,
    pub(crate) points: Vec<[f64; 3]>,
}

/// Preserve command ordering and all ranked hits; native -onlytop is parsed but unused.
pub(crate) fn pick_axis(
    run: &mut SessionRun,
    origin: [f64; 3],
    direction: [f64; 3],
    name: &str,
    show_normal: bool,
) -> Result<AxisReply, Box<dyn Error>> {
    let hits = run.session().axis_intersections(origin, direction)?;
    if hits.is_empty() {
        return Ok(AxisReply {
            text: "There are no any intersections with this axis.".into(),
            points: Vec::new(),
        });
    }
    display_annotation(run, origin, direction, name, &hits, show_normal)?;
    Ok(axis_reply(&hits))
}

fn axis_reply(hits: &[Intersection]) -> AxisReply {
    let points: Vec<_> = hits.iter().map(|hit| hit.0).collect();
    let text = points
        .iter()
        .map(|p| format!("{} {} {}\n", p[0], p[1], p[2]))
        .collect();
    AxisReply { text, points }
}

fn display_annotation(
    run: &mut SessionRun,
    origin: [f64; 3],
    direction: [f64; 3],
    name: &str,
    hits: &[Intersection],
    normal: bool,
) -> Result<(), Box<dyn Error>> {
    let previous = run.session().context().get_auto_activate_selection();
    run.session().context().set_auto_activate_selection(false);
    let result = display_hits(run, origin, direction, name, hits, normal);
    run.session()
        .context()
        .set_auto_activate_selection(previous);
    result
}

fn display_hits(
    run: &mut SessionRun,
    origin: [f64; 3],
    direction: [f64; 3],
    name: &str,
    hits: &[Intersection],
    normal: bool,
) -> Result<(), Box<dyn Error>> {
    display_start(run, origin, direction, name)?;
    let session = run.session();
    for (index, hit) in hits.iter().enumerate() {
        display_hit(session, name, index, *hit, normal)?;
    }
    Ok(())
}

fn display_start(
    run: &mut SessionRun,
    origin: [f64; 3],
    direction: [f64; 3],
    name: &str,
) -> Result<(), Box<dyn Error>> {
    ray_axis(
        run.session(),
        &format!("{name}_axis"),
        origin,
        direction,
        -1.,
        NamedColor::GREEN,
    )?;
    display_start_point(run.session(), name, origin)?;
    Ok(())
}

fn display_start_point(
    session: &mut crate::case_view::viewer_session::ViewerSession,
    name: &str,
    origin: [f64; 3],
) -> Result<(), Box<dyn Error>> {
    point(
        session,
        &format!("{name}_start"),
        origin,
        AspectTypeOfMarker::O,
        Some(NamedColor::GREEN),
    )?;
    Ok(())
}

fn display_hit(
    session: &mut crate::case_view::viewer_session::ViewerSession,
    name: &str,
    index: usize,
    hit: Intersection,
    show_normal: bool,
) -> Result<(), Box<dyn Error>> {
    if show_normal {
        display_normal(session, name, index, hit)?;
    }
    point(
        session,
        &format!("{name}_intersect_{index}"),
        hit.0,
        AspectTypeOfMarker::Plus,
        Some(NamedColor::RED),
    )?;
    Ok(())
}

fn display_normal(
    session: &mut crate::case_view::viewer_session::ViewerSession,
    name: &str,
    index: usize,
    (position, normal, length): Intersection,
) -> Result<(), Box<dyn Error>> {
    let squared = normal.iter().map(|value| value * value).sum::<f32>();
    if squared > f32::EPSILON {
        ray_axis(
            session,
            &format!("{name}_normal_{index}"),
            position,
            normal.map(f64::from),
            length,
            NamedColor::BLUE,
        )?;
    }
    Ok(())
}
