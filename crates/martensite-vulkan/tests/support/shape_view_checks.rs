//! Shape checks and the standard 3D check view shared by the shape regressions.
use crate::case_view::viewer_session::DisplayRequest;
use crate::session_run::SessionRun;
use martensite_tkbrep::topexp::{self, Explorer};
use martensite_tkbrep::topods::Shape;
use martensite_tkbrep::toptools::{IndexedMapOfShape, MapOfShape};
use martensite_tkg3d::top_abs::ShapeEnum;
use std::error::Error;

/// The sub-shape kinds counted, in the order of the expected counts.
const COUNTED_KINDS: [(ShapeEnum, &str); 8] = [
    (ShapeEnum::Vertex, "VERTEX"),
    (ShapeEnum::Edge, "EDGE"),
    (ShapeEnum::Wire, "WIRE"),
    (ShapeEnum::Face, "FACE"),
    (ShapeEnum::Shell, "SHELL"),
    (ShapeEnum::Solid, "SOLID"),
    (ShapeEnum::CompSolid, "COMPSOLID"),
    (ShapeEnum::Compound, "COMPOUND"),
];

/// Compare the distinct sub-shapes of each kind and their total with `expected` (vertex, edge,
/// wire, face, shell, solid, compsolid, compound, total), printing an error per mismatch.
/// Example: `check_sub_shape_counts(&face, &[4, 4, 1, 1, 0, 0, 0, 0, 10])?`.
pub(crate) fn check_sub_shape_counts(
    shape: &Shape,
    expected: &[usize; 9],
) -> Result<(), Box<dyn Error>> {
    let mut total = 0;
    for ((kind, label), want) in COUNTED_KINDS.into_iter().zip(expected) {
        let mut unique = IndexedMapOfShape::default();
        topexp::map_shapes_of_type(shape, kind, &mut unique)?;
        total += unique.extent();
        report_count(label, unique.extent(), *want);
    }
    report_count("SHAPE", total, expected[8]);
    Ok(())
}

fn report_count(label: &str, found: usize, expected: usize) {
    if found != expected {
        println!("Error : number of {label} entities in shape is {found}, expected {expected}");
    }
}

/// Print the number of vertices used once only (the free vertices of a section).
/// Example: `report_alone_vertices(&section)?` prints ` nb alone Vertices : 0`.
pub(crate) fn report_alone_vertices(shape: &Shape) -> Result<usize, Box<dyn Error>> {
    let mut vertices = MapOfShape::default();
    let mut explorer = Explorer::new(shape, ShapeEnum::Vertex, ShapeEnum::Shape)?;
    while explorer.more() {
        let current = explorer.current().clone();
        if !vertices.add(current.clone()) {
            vertices.remove(&current);
        }
        explorer.next()?;
    }
    println!(" nb alone Vertices : {}", vertices.extent());
    Ok(vertices.extent())
}

/// The standard 3D check view: a cleared view showing `shape` shaded and fitted, dumped as
/// `image`, then completion; `checkview_3d(&result, "case.png")`.
pub(crate) fn checkview_3d(shape: &Shape, image: &'static str) -> Result<(), Box<dyn Error>> {
    let mut run = SessionRun::new()?;
    checkview_in(&mut run, shape, image)?;
    run.finish()
}

/// The 3D check view in the regression's open view: cleared, `shape` shown shaded, fitted
/// and dumped as `image`; `checkview_in(&mut run, &result, "case.png")?`.
pub(crate) fn checkview_in(
    run: &mut SessionRun,
    shape: &Shape,
    image: &'static str,
) -> Result<(), Box<dyn Error>> {
    run.session().clear()?;
    run.session()
        .display("result", shape, &DisplayRequest::default())?;
    run.session().set_display_mode(1)?;
    run.session().fit()?;
    run.dump(image)
}

#[path = "regression_completion.rs"]
mod regression_completion;
// Bounded viewer hosts share this module without using its no-view completion path.
#[allow(unused_imports)]
pub(crate) use regression_completion::finish_without_view;
