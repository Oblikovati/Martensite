use super::*;
use crate::{LineStyle, PrimitiveDraw, RenderVertex};
use martensite_tkservice::graphic3d::Graphic3dAspects;

fn fill() -> PrimitiveDraw {
    let vertices = vec![RenderVertex::new([0.; 3], [1.; 4]); 3];
    PrimitiveDraw::new(vertices, vec![0, 1, 2], crate::identity_clip()).unwrap()
}

fn edge() -> PrimitiveDraw {
    let mut draw = fill();
    draw.polygon_edge_style = Some(LineStyle::new(1., 0xfff0, 1).unwrap());
    draw
}

fn params() -> Graphic3dRenderingParams {
    Graphic3dRenderingParams::new().unwrap()
}

fn hollow(edges: bool, line: AspectTypeOfLine) -> Graphic3dAspects {
    let aspect = Graphic3dAspects::default();
    aspect.set_interior_style(AspectInteriorStyle::Empty);
    aspect.set_draw_edges(edges);
    aspect.set_line_type(line);
    aspect
}

#[test]
fn filled_interiors_keep_every_draw() {
    let aspect = Graphic3dAspects::default();
    let mut draws = vec![fill(), edge()];
    apply_hollow_interior(&mut draws, &aspect, &params()).unwrap();
    assert_eq!(draws.len(), 2);
}

#[test]
fn hollow_interior_without_edges_draws_nothing() {
    let mut draws = vec![fill()];
    apply_hollow_interior(
        &mut draws,
        &hollow(false, AspectTypeOfLine::Dash),
        &params(),
    )
    .unwrap();
    assert!(draws.is_empty());
}

#[test]
fn hollow_interior_with_dashed_edges_keeps_only_the_edges() {
    let mut draws = vec![fill(), edge()];
    apply_hollow_interior(&mut draws, &hollow(true, AspectTypeOfLine::Dash), &params()).unwrap();
    assert_eq!(draws.len(), 1);
    assert!(draws[0].polygon_edge_style.is_some());
}

#[test]
fn hollow_interior_with_solid_polygon_edges_draws_the_faces_in_line_mode() {
    let mut draws = vec![fill(), edge()];
    apply_hollow_interior(
        &mut draws,
        &hollow(true, AspectTypeOfLine::Solid),
        &params(),
    )
    .unwrap();
    assert_eq!(draws.len(), 1);
    assert!(draws[0].polygon_face_lines);
    assert!(!draws[0].is_polygon_edge_pass());
    let style = draws[0].polygon_edge_style.unwrap();
    assert_eq!((style.width, style.pattern, style.factor), (1., 0xffff, 1));
}

#[test]
fn fixed_function_hollow_faces_with_solid_edges_draw_filled_with_the_hollow_cutoff() {
    let mut draws = vec![fill()];
    apply_hollow_interior(
        &mut draws,
        &hollow(true, AspectTypeOfLine::Solid),
        &params(),
    )
    .unwrap();
    assert_eq!(draws.len(), 1);
    assert_eq!(draws[0].surface_cutoff, Some(0.285));
    assert!(draws[0].mesh_edges.is_none() && draws[0].polygon_edge_style.is_none());
}

#[test]
fn hollow_interior_with_dashed_edges_and_no_edge_pass_is_refused() {
    let mut draws = vec![fill()];
    let error = apply_hollow_interior(&mut draws, &hollow(true, AspectTypeOfLine::Dash), &params());
    assert!(error.is_err());
}

#[test]
fn face_lines_take_the_last_line_width_set_earlier_in_the_frame() {
    let mut first = fill();
    first.polygon_edge_style = Some(LineStyle::new(1., 0xffff, 1).unwrap().with_context_width());
    first.polygon_face_lines = true;
    let mut line = fill();
    line.line_style = Some(LineStyle::new(4., 0xffff, 1).unwrap());
    let mut wide_edge = edge();
    wide_edge.polygon_edge_style = Some(LineStyle::new(3., 0xffff, 1).unwrap());
    let later = first.clone();
    let mut draws = vec![first, line, wide_edge, later];
    resolve_ordered_line_widths(&mut draws);
    assert_eq!(draws[0].polygon_edge_style.unwrap().width, 1.);
    assert_eq!(draws[3].polygon_edge_style.unwrap().width, 3.);
}

#[test]
fn adjacency_lines_take_the_last_width_and_do_not_set_one() {
    let context = LineStyle::new(1., 0xffff, 1).unwrap().with_context_width();
    let mut adjacency = fill();
    adjacency.line_style = Some(context);
    let mut wide = fill();
    wide.line_style = Some(LineStyle::new(4., 0xffff, 1).unwrap());
    let mut solid_face_lines = fill();
    solid_face_lines.polygon_edge_style = Some(context);
    solid_face_lines.polygon_face_lines = true;
    let mut draws = vec![adjacency.clone(), wide, adjacency, solid_face_lines];
    resolve_ordered_line_widths(&mut draws);
    let widths: Vec<f32> = draws
        .iter()
        .map(|draw| draw.line_style.or(draw.polygon_edge_style).unwrap().width)
        .collect();
    // The first keeps the Activate default; the later context-width draws inherit 4.
    assert_eq!(widths, [1., 4., 4., 4.]);
}

#[test]
fn hollow_interior_with_solid_mesh_edges_keeps_the_interior_colour_on_edges() {
    let mut mesh = fill();
    mesh.mesh_edges = Some(crate::RenderMeshEdges::new([1., 0., 0., 1.], 1., false).unwrap());
    let mut draws = vec![mesh];
    apply_hollow_interior(
        &mut draws,
        &hollow(true, AspectTypeOfLine::Solid),
        &params(),
    )
    .unwrap();
    assert_eq!(draws.len(), 1);
    assert_eq!(draws[0].mesh_edges.unwrap().color, [-1.; 4]);
    assert_eq!(draws[0].surface_cutoff, Some(0.285));
}
