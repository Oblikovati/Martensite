//! Located face mesh edges use the same shared classification and drawing order.
use martensite_raster_core::{MeshEdgeSets, PaletteColor, PaletteKind, RasterDisplay};
use martensite_tkbrep::{brep::BRepTool, topods::Face};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkmath::{
    gp::Transform3,
    poly::{Triangulation, mesh_purpose},
};
/// The triangulation of a face (located): free edges in red, internal ones in blue.
pub(super) fn draw_face_triangulation(
    face: &Face,
    display: &mut RasterDisplay<'_>,
) -> Result<(), Failure> {
    let (triangulation, location) = BRepTool::triangulation(face, mesh_purpose::NONE)?;
    if triangulation.is_null() {
        return Ok(());
    }
    draw_triangulation(&triangulation, location.transformation(), display)
}

/// `TopologyDrawableShape::display(T, tr, dis)`.
pub(super) fn draw_triangulation(
    triangulation: &Handle<Triangulation>,
    transform: Transform3,
    display: &mut RasterDisplay<'_>,
) -> Result<(), Failure> {
    let Some(mesh) = triangulation.get() else {
        return Ok(());
    };
    let edges = MeshEdgeSets::compute(triangulation)?;
    let sets = [
        (PaletteKind::Rouge, &edges.free),
        (PaletteKind::Bleu, &edges.internal),
    ];
    for (color, pairs) in sets {
        display.set_color(PaletteColor::new(color));
        draw_triangle_pairs(mesh, pairs, transform, display)?;
    }
    Ok(())
}

fn draw_triangle_pairs(
    mesh: &Triangulation,
    pairs: &[(i32, i32)],
    transform: Transform3,
    display: &mut RasterDisplay<'_>,
) -> Result<(), Failure> {
    for (n1, n2) in pairs {
        let (p1, p2) = (mesh.node(*n1)?, mesh.node(*n2)?);
        display.draw_3d(p1.transformed(transform), p2.transformed(transform));
    }
    Ok(())
}
