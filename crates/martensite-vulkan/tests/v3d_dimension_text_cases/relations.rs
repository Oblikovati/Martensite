//! Relations between selected sub-shapes of a box or a cylinder: concentric circles, equal
//! distances, equal radii, a fixed edge, identic shapes, an offset, parallel, perpendicular
//! and symmetric edges.
use super::prelude::*;
use crate::case_view::viewer_session::{LocationChange, RelationKind};
use crate::primitive_shapes::{box_at, box_sized, cylinder};
use martensite_tkbrep::topods::Shape;
use martensite_tkv3d::ais::{AisSelectionScheme as Scheme, AisShapeSource};
use martensite_tkv3d::prs_dim::{PrsDimRelationMembers as _, PrsDimRelationSource};

const EDGES: i32 = 2;
const VERTICES: i32 = 1;
const FACES: i32 = 4;

fn relation_banner(selection: &str) {
    let rule = "====================================================================";
    banner(&[
        rule,
        "OCC28162: Test Commands - eliminate usage of deprecated Local Context",
        selection,
        rule,
    ]);
}

/// Clear, open a view, display `name`, fit, and activate selection mode `mode`.
fn shown(name: &str, shape: &Shape, mode: i32) -> Result<SessionRun, Box<dyn Error>> {
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    run.session().display(name, shape, &Default::default())?;
    run.session().fit()?;
    run.session().set_object_selection_mode(name, mode, true)?;
    Ok(run)
}

/// Pick at each pixel, the first replacing the selection, the next toggling.
fn picks(run: &mut SessionRun, pixels: &[(i32, i32)]) -> Outcome {
    for (index, &(x, y)) in pixels.iter().enumerate() {
        let scheme = if index == 0 {
            Scheme::Replace
        } else {
            Scheme::Xor
        };
        run.session().select_polyline(&[(x, y)], scheme)?;
    }
    Ok(())
}

/// Switch the selection of `name` from edges to vertices.
fn select_vertices(run: &mut SessionRun, name: &str) -> Outcome {
    run.session()
        .set_object_selection_mode(name, EDGES, false)?;
    run.session()
        .set_object_selection_mode(name, VERTICES, true)?;
    Ok(())
}

/// `concentric`: the two circles of a cylinder.
/// Example: `concentric_circles()?`.
pub(crate) fn concentric_circles() -> Outcome {
    relation_banner("Select two circled edges");
    let mut run = shown("c", &cylinder(15., 30.)?, EDGES)?;
    picks(&mut run, &[(205, 174), (205, 377)])?;
    run.session()
        .display_relation("r", RelationKind::Concentric)?;
    run.dump("concentric.png")?;
    run.finish()
}

/// `equaldistance`: four box edges, then four vertices in a rectangle.
/// Example: `equal_distances()?`.
pub(crate) fn equal_distances() -> Outcome {
    relation_banner("Select 4 edges or vertices");
    let mut run = shown("b", &box_sized([10., 20., 30.])?, EDGES)?;
    picks(&mut run, &[(123, 112), (280, 23), (123, 382), (280, 293)])?;
    run.session()
        .display_relation("r4edges", RelationKind::EqualDistance)?;
    run.session().zoom(0.5)?;
    run.dump("equaldistance_edges.png")?;
    run.session().remove(&["r4edges"])?;
    run.session().fit()?;
    picks(&mut run, &[(0, 0)])?;
    select_vertices(&mut run, "b")?;
    run.session()
        .select_rectangle((75, 0), (335, 140), Scheme::Replace)?;
    run.session()
        .display_relation("r4vertices", RelationKind::EqualDistance)?;
    run.session().zoom(0.5)?;
    run.dump("equaldistance_vertices.png")?;
    run.finish()
}

/// `equalradius`: one circle of a cylinder, then both.
/// Example: `equal_radii()?`.
pub(crate) fn equal_radii() -> Outcome {
    relation_banner("Select 1 or 2 circled edges");
    let mut run = shown("c", &cylinder(15., 30.)?, EDGES)?;
    picks(&mut run, &[(205, 174)])?;
    run.session()
        .display_relation("r1", RelationKind::EqualRadius)?;
    run.dump("equalradius_1.png")?;
    run.session().remove(&["r1"])?;
    picks(&mut run, &[(205, 377), (205, 174)])?;
    run.session()
        .display_relation("r2", RelationKind::EqualRadius)?;
    run.dump("equalradius_2.png")?;
    run.finish()
}

/// `fix`: one box edge in a rectangle.
/// Example: `fixed_edge()?`.
pub(crate) fn fixed_edge() -> Outcome {
    relation_banner("Select 1 edge");
    let mut run = shown("b", &box_sized([10., 20., 30.])?, EDGES)?;
    run.session()
        .select_rectangle((10, 10), (100, 380), Scheme::Replace)?;
    run.session().display_relation("r", RelationKind::Fix)?;
    run.dump("fix.png")?;
    run.finish()
}

/// `identic`: two box edges, then two vertices in two rectangles.
/// Example: `identic_shapes()?`.
pub(crate) fn identic_shapes() -> Outcome {
    relation_banner("Select 2 edges or vertices");
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    run.session()
        .display("b", &box_sized([10., 20., 30.])?, &Default::default())?;
    run.session().fit()?;
    run.session().set_object_selection_mode("b", FACES, false)?;
    run.session().set_object_selection_mode("b", EDGES, true)?;
    picks(&mut run, &[(87, 170), (165, 170)])?;
    run.session()
        .display_relation("redges", RelationKind::Identic)?;
    run.dump("identic_edges.png")?;
    retain_ident_relation_controls(&mut run);
    identic_vertices(&mut run)?;
    run.finish()
}

const IDENTIC_RELATION_CONTROL_ENV: &str = "MARTENSITE_CASE_IDENTIC_RELATION_CONTROLS";

fn identic_relation_controls_requested() -> Result<bool, Box<dyn Error>> {
    match std::env::var_os(IDENTIC_RELATION_CONTROL_ENV) {
        None => Ok(false),
        Some(value) if value == "1" => Ok(true),
        Some(value) => Err(std::io::Error::other(format!(
            "{IDENTIC_RELATION_CONTROL_ENV}={value:?}; expected unset or 1"
        ))
        .into()),
    }
}

fn retain_ident_relation_controls(run: &mut SessionRun) {
    match identic_relation_controls_requested() {
        Ok(true) => {
            let observation = capture_ident_relation_controls(run);
            run.retain_observation(observation);
        }
        Ok(false) => (),
        Err(error) => run.retain_observation(Err(error)),
    }
}

fn capture_ident_relation_controls(run: &mut SessionRun) -> Result<(), Box<dyn Error>> {
    let (source, roles) = identic_relation_sources(run)?;
    super::identic_relation_controls::capture_controls(
        run,
        &source,
        &roles,
        &[(87, 170), (165, 170)],
    )
}

fn identic_relation_sources(run: &mut SessionRun) -> Result<(Shape, [Shape; 2]), Box<dyn Error>> {
    let source_objects = run.session().named_objects(&["b"])?;
    let relation_objects = run.session().named_objects(&["redges"])?;
    let source_object = source_objects[0].cast::<dyn AisShapeSource>();
    let source = source_object
        .get()
        .ok_or("b; expected a displayed AisShape")?
        .shape_base()
        .fields
        .read()
        .shape
        .clone();
    let relation_handle = relation_objects[0].cast::<dyn PrsDimRelationSource>();
    let relation = relation_handle
        .get()
        .ok_or("redges; expected a PrsDimRelation")?;
    let roles = super::identic_relation_controls::relation_endpoints(
        &source,
        relation.first_shape(),
        relation.second_shape(),
    )
    .map_err(|error| {
        std::io::Error::other(format!(
            "identic relation source binding {error:?}; expected two distinct source edges"
        ))
    })?;
    Ok((source, roles))
}

/// `offset`: a box face, then the opposite face after a half turn of the box.
/// Example: `offset_faces()?`.
pub(crate) fn offset_faces() -> Outcome {
    relation_banner("Select 2 faces");
    let mut run = shown("b", &box_at([-5., -10., -15.], [10., 20., 30.])?, FACES)?;
    picks(&mut run, &[(100, 200)])?;
    let turn = LocationChange::Rotate {
        origin: [0.; 3],
        axis: [0., 0., 1.],
        degrees: 180.,
    };
    run.session().change_location("b", turn)?;
    run.session().select_polyline(&[(100, 200)], Scheme::Xor)?;
    run.session().display_relation("r", RelationKind::Offset)?;
    run.dump("offset.png")?;
    run.finish()
}

/// `parallel`: two parallel edges of a centred box.
/// Example: `parallel_edges()?`.
pub(crate) fn parallel_edges() -> Outcome {
    relation_banner("Select 2 edges");
    let mut run = shown("b", &box_at([-5., -10., -15.], [10., 20., 30.])?, EDGES)?;
    picks(&mut run, &[(87, 170), (165, 170)])?;
    run.session()
        .display_relation("r", RelationKind::Parallel)?;
    run.session().zoom(0.5)?;
    run.dump("parallel_edges.png")?;
    run.finish()
}

/// `perpendicular`: two crossing edges of a centred box.
/// Example: `perpendicular_edges()?`.
pub(crate) fn perpendicular_edges() -> Outcome {
    relation_banner("Select 2 edges");
    let mut run = shown("b", &box_at([-5., -10., -15.], [10., 20., 30.])?, EDGES)?;
    picks(&mut run, &[(87, 145), (135, 119)])?;
    run.session()
        .display_relation("r", RelationKind::Perpendicular)?;
    run.dump("perpendicular_edges.png")?;
    run.finish()
}

/// `symmetric`: an axis edge and two edges, then an edge and two vertices.
/// Example: `symmetric_shapes()?`.
pub(crate) fn symmetric_shapes() -> Outcome {
    relation_banner("Select 3 edges or 1 edge and 2 vertices");
    let mut run = shown("b", &box_sized([10., 20., 30.])?, EDGES)?;
    picks(&mut run, &[(280, 23), (123, 112), (123, 382)])?;
    run.session()
        .display_relation("r3edges", RelationKind::Symmetric)?;
    run.dump("symmetric_3edges.png")?;
    run.session().remove(&["r3edges"])?;
    picks(&mut run, &[(0, 0)])?;
    picks(&mut run, &[(123, 112)])?;
    select_vertices(&mut run, "b")?;
    run.session()
        .select_rectangle((230, 0), (325, 55), Scheme::Xor)?;
    run.session()
        .display_relation("redge2vertices", RelationKind::Symmetric)?;
    run.dump("symmetric_edge_2vertices.png")?;
    run.finish()
}

fn identic_vertices(run: &mut SessionRun) -> Outcome {
    run.session().remove(&["redges"])?;
    select_vertices(run, "b")?;
    run.session()
        .select_rectangle((160, 130), (170, 140), Scheme::Replace)?;
    run.session()
        .select_rectangle((240, 0), (250, 10), Scheme::Xor)?;
    run.session()
        .display_relation("rvertices", RelationKind::Identic)?;
    run.dump("identic_vertices.png")?;
    Ok(())
}
