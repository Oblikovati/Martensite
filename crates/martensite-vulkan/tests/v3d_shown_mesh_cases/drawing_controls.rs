//! Independent topology/report fixtures pin the drawing-only retained boundary.
use super::*;
use martensite_geometry_raster::TopologyDrawable;
use martensite_raster_core::{
    PaletteKind, RasterDisplay, RasterReports, RecordingReports, drawable::RasterDrawable,
};
use martensite_tkbrep::{brep::BRepBuilder, topods::Edge};
use martensite_tkmath::gp::Point3;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeEdge;
use std::rc::Rc;

fn independent_edge() -> Edge {
    BRepBuilderAPIMakeEdge::from_points(Point3::default(), Point3::new(2., 0., 0.))
        .unwrap()
        .edge()
        .unwrap()
}
fn drawing(edge: &Edge, reports: Rc<dyn RasterReports>) -> TopologyDrawable {
    triangulated::retained_mesh_drawing(edge.as_ref(), reports)
}
#[test]
fn retained_mesh_factory_uses_owned_defaults_and_the_same_topology() {
    let edge = independent_edge();
    let reports = Rc::new(RecordingReports::default());
    let drawable = drawing(&edge, reports.clone());
    assert!(drawable.rendering().shape().is_same(edge.as_ref()));
    assert_default_rendering(drawable.rendering());
    assert_eq!(reports.output(), "");
    assert_eq!(reports.errors(), "");
}
fn assert_default_rendering(rendering: &martensite_geometry_raster::TopologyRendering) {
    assert_eq!(
        (
            rendering.nb_isos(),
            rendering.discret(),
            rendering.infinity_size()
        ),
        (2, 30, 100.)
    );
    assert!(!rendering.display_triangulation());
    assert!(!rendering.display_polygons());
    assert_default_colors(rendering.colors());
    assert_default_hlr(rendering.hlr_settings());
}
fn assert_default_colors(colors: martensite_geometry_raster::ShapeColors) {
    let actual = [
        colors.free.id(),
        colors.connected.id(),
        colors.edge.id(),
        colors.isos.id(),
    ];
    let expected = [
        PaletteKind::Vert,
        PaletteKind::Jaune,
        PaletteKind::Rouge,
        PaletteKind::Bleu,
    ];
    assert_eq!(actual, expected);
}

fn assert_default_hlr(settings: martensite_geometry_raster::HlrSettings) {
    assert!(!settings.with_hlr);
    assert!(settings.with_rg1);
    assert!(!settings.with_rgn);
    assert!(!settings.with_hid);
    assert_eq!(
        settings.angle.to_bits(),
        (35. * std::f64::consts::PI / 180.).to_bits()
    );
}
#[test]
fn retained_mesh_controls_are_local_while_topology_identity_is_shared() {
    let edge = independent_edge();
    let reports = Rc::new(RecordingReports::default());
    let first = drawing(&edge, reports.clone());
    let second = drawing(&edge, reports.clone());
    change_first_drawing(&first, reports.as_ref());
    assert_eq!(second.rendering().nb_isos(), 2);
    assert_eq!(second.rendering().discret(), 30);
    assert!(!second.rendering().display_triangulation());
    assert_eq!(second.bounds(), [0.; 4]);
    assert!(!second.visible());
    assert!(
        first
            .rendering()
            .shape()
            .is_same(&second.rendering().shape())
    );
}
fn change_first_drawing(first: &TopologyDrawable, reports: &dyn RasterReports) {
    first.rendering().change_nb_isos(0, reports).unwrap();
    first.rendering().change_discret(7);
    first.rendering().set_display_triangulation(true);
    first.set_bounds([1., 2., 3., 4.]);
    first.set_visible(true);
}

#[test]
fn actual_bad_edge_reports_are_routed_only_to_the_supplied_recorder() {
    let edge = independent_edge();
    BRepBuilder::default().range(&edge, 1., 0., true).unwrap();
    let first_reports = Rc::new(RecordingReports::default());
    let second_reports = Rc::new(RecordingReports::default());
    let first = drawing(&edge, first_reports.clone());
    let second = drawing(&edge, second_reports.clone());
    first.draw_on(&mut RasterDisplay::batch()).unwrap();
    let frozen = first_reports.output();
    assert!(frozen.starts_with("DBRep_DrawableShape : Bad parameters on edge.\n"));
    assert_eq!(second_reports.output(), "");
    assert_eq!(second_reports.errors(), "");
    second.draw_on(&mut RasterDisplay::batch()).unwrap();
    assert_eq!(first_reports.output(), frozen);
    assert!(
        second_reports
            .output()
            .starts_with("DBRep_DrawableShape : Bad parameters on edge.\n")
    );
}
