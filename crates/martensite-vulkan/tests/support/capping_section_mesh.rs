//! Genuine capping aspects retain their own edge state independently of source geometry.
use martensite_tkernel::quantity::ColorRgba;
use martensite_tkservice::{
    aspect::{AspectInteriorStyle, AspectTypeOfLine},
    graphic3d::{
        Graphic3dAspectsEdgeMembers, Graphic3dAspectsLineMembers, Graphic3dAspectsMembers,
        Graphic3dAspectsRenderMembers, Graphic3dAspectsSource,
    },
};
pub fn configure_section_mesh_edges(aspect: &dyn Graphic3dAspectsSource, mode: &str) {
    if !mode.contains("section-edges") {
        return;
    }
    aspect.set_draw_edges(!mode.contains("edges-disabled"));
    // Edge-only control capture of the undefined-locus rule (user decision 2026-10-28): an
    // empty interior skips the cap fill and keeps its polygon-mode edges.
    if mode.contains("face-hidden") {
        aspect.set_interior_style(AspectInteriorStyle::Empty);
    }
    aspect.set_edge_color_rgba(ColorRgba::new(0., 1., 1., section_edge_alpha(mode)).unwrap());
    aspect
        .set_line_width(if mode.contains("thin") { 1. } else { 3. })
        .unwrap();
    aspect.set_skip_first_edge(mode.contains("skip-first"));
    if mode.contains("dashed") {
        aspect.set_line_type(AspectTypeOfLine::Dash);
    }
    configure_section_mesh_offset(aspect, mode);
}
fn section_edge_alpha(mode: &str) -> f32 {
    if mode.contains("negative-alpha") {
        return -0.25;
    }
    if mode.contains("edge-alpha") {
        return 0.25;
    }
    if mode.contains("edge-background") {
        return 0.05;
    }
    1.
}
fn configure_section_mesh_offset(aspect: &dyn Graphic3dAspectsSource, mode: &str) {
    let mask = if mode.contains("both-offset") {
        3
    } else if mode.contains("line-offset") {
        2
    } else {
        1
    };
    if mode.contains("offset") {
        aspect.set_polygon_offsets(mask, if mode.contains("factor-zero") { 0. } else { 2. }, 3.);
    }
}
#[cfg(test)]
#[path = "capping_section_mesh_tests.rs"]
mod tests;
