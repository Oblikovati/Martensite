//! Selection of circles as the circle command displays them: filled circles, outlines, arcs and
//! filled sectors, checked by the sensitive entity types the last selection picked.
use super::prelude::*;
use crate::session_run::SessionRun;
use crate::viewer_objects::{circle_object, circle_through, point_object};

/// One selection at pixels (with overlap allowed when `overlap`), then whether an entity
/// whose type name contains `entity` must be among the picked ones, with the case's message.
struct EntityPick {
    pixels: &'static [(i32, i32)],
    overlap: bool,
    entity: &'static str,
    picked: bool,
    message: &'static str,
}

/// The circle program's banner, shared by the circle regressions of the group.
pub(super) fn circle_banner() {
    banner(&[
        "=================================",
        "0032547: Visualization, Select3DSensitiveCylinder - implement picking of a hollow cylinder",
        "",
        "=================================",
    ]);
}

/// Display three named points and the circle through them.
pub(super) fn show_circle(
    run: &mut SessionRun,
    names: [&str; 4],
    points: [[f64; 3]; 3],
    filled: bool,
    degrees: Option<[f64; 2]>,
) -> Result<(), Box<dyn Error>> {
    for (name, point) in names.iter().zip(points) {
        run.session().display_object(name, point_object(point)?)?;
    }
    let circle = circle_through(points)?;
    run.session()
        .display_object(names[3], circle_object(&circle, filled, degrees))?;
    Ok(())
}

/// Run each select command and print its message when the picked entity types disagree.
fn entity_picks(run: &mut SessionRun, picks: &[EntityPick]) -> Outcome {
    for pick in picks {
        run.session().select_pixels(pick.pixels, pick.overlap)?;
        let types = run.session().picked_entity_types()?;
        if types.iter().any(|name| name.contains(pick.entity)) != pick.picked {
            println!("{}", pick.message);
        }
    }
    Ok(())
}

const CIRCLE: &str = "Select3DSensitiveCircle";
const POLY: &str = "Select3DSensitivePoly";

/// The circle through (0, 0, 0), (50, 50, 0) and (100, 0, 0), filled or not, seen axonometric.
fn radial_circle(filled: bool) -> Result<SessionRun, Box<dyn Error>> {
    let mut run = SessionRun::new()?;
    let names = ["radP1", "radP2", "radP3", "circle"];
    let points = [[0., 0., 0.], [50., 50., 0.], [100., 0., 0.]];
    show_circle(&mut run, names, points, filled, None)?;
    run.session().set_axonometric()?;
    run.session().fit()?;
    Ok(run)
}

/// A pick expecting the circle entity (`picked`) or not.
const fn circle_pick(
    pixels: &'static [(i32, i32)],
    overlap: bool,
    picked: bool,
    message: &'static str,
) -> EntityPick {
    EntityPick {
        pixels,
        overlap,
        entity: CIRCLE,
        picked,
        message,
    }
}

const POLYGON_AROUND: &[(i32, i32)] = &[(50, 300), (360, 300), (360, 100), (50, 100)];
const POLYGON_PARTIAL: &[(i32, i32)] = &[(250, 300), (360, 300), (360, 100), (250, 100)];
const POLYGON_INSIDE: &[(i32, i32)] = &[(150, 200), (200, 200), (200, 150), (150, 150)];

/// `filled_circle`: a filled circle is picked inside, on and around it.
pub(crate) fn filled_circle_picks() -> Outcome {
    circle_banner();
    let mut run = radial_circle(true)?;
    entity_picks(
        &mut run,
        &[
            circle_pick(
                &[(200, 200)],
                false,
                true,
                "Error: circle should be detected",
            ),
            circle_pick(
                &[(177, 279)],
                false,
                true,
                "Error: circle should be detected",
            ),
            circle_pick(
                &[(360, 360)],
                false,
                false,
                "Error: circle should not be detected",
            ),
            circle_pick(
                POLYGON_AROUND,
                false,
                true,
                "Error: circle inside a polygon should be detected",
            ),
            circle_pick(
                POLYGON_PARTIAL,
                false,
                false,
                "Error: circle partially covered by a polygon should not be detected",
            ),
            circle_pick(
                POLYGON_PARTIAL,
                true,
                true,
                "Error: circle partially covered by a polygon should be detected",
            ),
            circle_pick(
                POLYGON_INSIDE,
                false,
                false,
                "Error: circle around the polygon should not be detected",
            ),
            circle_pick(
                POLYGON_INSIDE,
                true,
                true,
                "Error: circle around the polygon should be detected",
            ),
            circle_pick(
                &[(50, 300), (360, 100)],
                false,
                true,
                "Error: circle inside a rectangle should be detected",
            ),
            circle_pick(
                &[(250, 300), (360, 100)],
                false,
                false,
                "Error: circle partially covered by a rectangle should not be detected",
            ),
            circle_pick(
                &[(250, 300), (360, 100)],
                true,
                true,
                "Error: circle partially covered by a rectangle should be detected",
            ),
            circle_pick(
                &[(150, 200), (200, 150)],
                false,
                false,
                "Error: circle around the rectangle should not be detected",
            ),
            circle_pick(
                &[(150, 200), (200, 150)],
                true,
                true,
                "Error: circle around the rectangle should be detected",
            ),
        ],
    )?;
    run.finish_without_image()
}

/// `circle_wire`: an outline circle is picked only on or across its outline.
pub(crate) fn circle_wire_picks() -> Outcome {
    circle_banner();
    let mut run = radial_circle(false)?;
    entity_picks(
        &mut run,
        &[
            circle_pick(
                &[(200, 200)],
                false,
                false,
                "Error: circle should not be detected",
            ),
            circle_pick(
                &[(177, 285)],
                false,
                true,
                "Error: circle should be detected",
            ),
            circle_pick(
                &[(360, 360)],
                false,
                false,
                "Error: circle should not be detected",
            ),
            circle_pick(
                POLYGON_AROUND,
                false,
                true,
                "Error: circle inside a polygon should be detected",
            ),
            circle_pick(
                POLYGON_PARTIAL,
                false,
                false,
                "Error: circle partially covered by a polygon should not be detected",
            ),
            circle_pick(
                POLYGON_PARTIAL,
                true,
                true,
                "Error: circle partially covered by a polygon should be detected",
            ),
            circle_pick(
                POLYGON_INSIDE,
                false,
                false,
                "Error: circle around the polygon should not be detected",
            ),
            circle_pick(
                POLYGON_INSIDE,
                true,
                false,
                "Error: circle around the polygon should not be detected",
            ),
            circle_pick(
                &[(50, 300), (360, 100)],
                false,
                true,
                "Error: circle inside a rectangle should be detected",
            ),
            circle_pick(
                &[(250, 300), (360, 100)],
                false,
                false,
                "Error: circle partially covered by a rectangle should not be detected",
            ),
            circle_pick(
                &[(250, 300), (360, 100)],
                true,
                true,
                "Error: circle partially covered by a rectangle should be detected",
            ),
            circle_pick(
                &[(150, 200), (200, 150)],
                false,
                false,
                "Error: circle around the rectangle should not be detected",
            ),
            circle_pick(
                &[(150, 200), (200, 150)],
                true,
                false,
                "Error: circle around the rectangle should not be detected",
            ),
        ],
    )?;
    run.finish_without_image()
}

/// The four circles of the sector and transformed regressions: names, points and filling.
type CircleSpec = ([&'static str; 4], [[f64; 3]; 3]);
const SECTOR_CIRCLES: [CircleSpec; 4] = [
    (
        ["C1P1", "C1P2", "C1P3", "c1"],
        [[10., 10., 10.], [60., 60., -10.], [35., 100., 10.]],
    ),
    (
        ["C2P1", "C2P2", "C2P3", "c2"],
        [[10., -10., -10.], [60., -60., 10.], [35., -100., 10.]],
    ),
    (
        ["C3P1", "C3P2", "C3P3", "c3"],
        [[-80., -10., 10.], [-140., -50., -10.], [-110., -100., -10.]],
    ),
    (
        ["C4P1", "C4P2", "C4P3", "c4"],
        [[-80., 10., 10.], [-140., 60., 10.], [-110., 100., -10.]],
    ),
];

/// `circle_sector`: a filled sector, an arc, a filled circle and an outline, seen from below.
pub(crate) fn circle_sector_picks() -> Outcome {
    circle_banner();
    let mut run = SessionRun::new()?;
    let shapes = [
        (true, [90., 150.]),
        (false, [90., 150.]),
        (true, [0., 360.]),
        (false, [0., 360.]),
    ];
    for ((names, points), (filled, range)) in SECTOR_CIRCLES.into_iter().zip(shapes) {
        show_circle(&mut run, names, points, filled, Some(range))?;
    }
    run.session()
        .set_standard_projection(Orientation::ZUP_BOTTOM)?;
    run.session().fit()?;
    let pick = |pixels, entity, message| EntityPick {
        pixels,
        overlap: false,
        entity,
        picked: true,
        message,
    };
    entity_picks(
        &mut run,
        &[
            pick(
                &[(100, 100)],
                CIRCLE,
                "Error: filled circle should be detected",
            ),
            pick(
                &[(100, 225)],
                CIRCLE,
                "Error: circle wire should be detected",
            ),
            pick(&[(390, 50)], POLY, "Error: circle arc should be detected"),
            pick(
                &[(360, 350)],
                POLY,
                "Error: circle sector should be detected",
            ),
        ],
    )?;
    run.finish_without_image()
}
