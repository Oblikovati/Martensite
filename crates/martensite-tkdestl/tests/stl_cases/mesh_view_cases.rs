//! Meshes read from STL files on the real viewer: display modes, hidden and selected entities,
//! colours, vectors, labels and clipping. Each case runs alone in a child process.
use super::checks::input;
use super::mesh_view::*;
use super::real_view::{ViewerHost, run_viewer_case};
use martensite_tkernel::{
    message::{ProgressIndicator, ProgressIndicatorHooks, ProgressRange, ProgressScope},
    quantity::NamedColor,
};
use martensite_tkv3d::ais::AisSelectionScheme::{Replace, Xor};
use martensite_tkv3d::prs_mgr::PrsMgrPresentableObjectMembers as _;
use martensite_tkv3d::v3d::V3dTypeOfOrientation;
use std::sync::{Arc, Mutex};

const MESH_B: &str = "dataset-7.9.0/others/b.stl";
const MESH_B2: &str = "dataset-7.9.0/others/b2.stl";
const MESH_B3: &str = "dataset-7.9.0/others/b3.stl";
const MESH_BOX: &str = "dataset-7.9.0/others/OMF6391_box.stl";
const MESH_SHAPE: &str = "occt-source-data/stl/shape.stl";
const MESH_BEARING: &str = "occt-source-data/stl/bearing.stl";
const MESH_HEAD: &str = "occt-source-data/stl/head.stl";
const MESH_SH1: &str = "occt-source-data/stl/sh1.stl";
const ELEMENTS: i32 = 8;
const NODES: i32 = 1;
const SHADED: i32 = 2;

fn read_mesh(host: &ViewerHost, file: &str) -> ShownMesh {
    mesh_from_stl(host, &input(file), &ProgressRange::default())
}

fn assert_view_draws(host: &ViewerHost, name: &str) {
    assert!(host.dump(name) > 0, "{name}: expected a drawn image");
}

fn select_five_elements(host: &ViewerHost, shown: &ShownMesh) {
    select_mode_on(host, &shown.object, ELEMENTS);
    for [x, y] in [[150, 100], [200, 200], [250, 250], [200, 100], [250, 100]] {
        select_point(host, x, y, Xor);
    }
}

#[test]
fn clicked_elements_can_be_hidden_and_the_view_turned() {
    run_viewer_case(
        "mesh_view_cases::clicked_elements_can_be_hidden_and_the_view_turned",
        || {
            let host = ViewerHost::open();
            let shown = read_mesh(&host, MESH_B);
            select_five_elements(&host, &shown);
            hide_selected(&host, &shown);
            host.view.rotate(-0.9, 0., 0., true).unwrap();
            assert_view_draws(&host, "mesh_A1");
        },
    );
}

#[test]
fn only_clicked_elements_can_be_shown() {
    run_viewer_case(
        "mesh_view_cases::only_clicked_elements_can_be_shown",
        || {
            let host = ViewerHost::open();
            let shown = read_mesh(&host, MESH_B);
            select_five_elements(&host, &shown);
            show_only_selected(&host, &shown);
            assert_view_draws(&host, "mesh_A2");
        },
    );
}

#[test]
fn only_nodes_inside_a_rectangle_can_be_shown() {
    run_viewer_case(
        "mesh_view_cases::only_nodes_inside_a_rectangle_can_be_shown",
        || {
            let host = ViewerHost::open();
            let shown = read_mesh(&host, MESH_B);
            select_mode_on(&host, &shown.object, NODES);
            select_rectangle(&host, [0, 0], [500, 500]);
            show_only_selected(&host, &shown);
            assert_view_draws(&host, "mesh_A3");
        },
    );
}

#[test]
fn nodes_inside_a_rectangle_can_be_hidden_after_showing_all() {
    run_viewer_case(
        "mesh_view_cases::nodes_inside_a_rectangle_can_be_hidden_after_showing_all",
        || {
            let host = ViewerHost::open();
            let shown = read_mesh(&host, MESH_B);
            show_all(&host, &shown);
            select_mode_on(&host, &shown.object, NODES);
            select_rectangle(&host, [0, 0], [250, 200]);
            hide_selected(&host, &shown);
            assert_view_draws(&host, "mesh_A4");
        },
    );
}

#[test]
fn three_meshes_show_in_different_modes_and_one_can_be_erased() {
    run_viewer_case(
        "mesh_view_cases::three_meshes_show_in_different_modes_and_one_can_be_erased",
        || {
            let host = ViewerHost::open();
            let first = read_mesh(&host, MESH_B);
            let second = read_mesh(&host, MESH_B2);
            let third = read_mesh(&host, MESH_B3);
            display_mode(&host, &first, SHADED);
            display_mode(&host, &second, 3);
            assert_view_draws(&host, "mesh_A5_dsd");
            erase(&host, &third);
            assert_view_draws(&host, "mesh_A5_hdn");
        },
    );
}

#[test]
fn triangle_labels_and_vectors_show() {
    run_viewer_case("mesh_view_cases::triangle_labels_and_vectors_show", || {
        let host = ViewerHost::open();
        let shown = read_mesh(&host, MESH_B);
        text_labels(&shown);
        display_mode(&host, &shown, 33);
        host.view.rotate(0.3, 0., 0., true).unwrap();
        assert_view_draws(&host, "mesh_A7_txt");
        // The script spells `-issipmle`, an option the command ignores: the arrows stay.
        vectors(&host, &shown, true, 10.0, NamedColor::GREEN, 0.5, false);
        display_mode(&host, &shown, 5);
        assert_view_draws(&host, "mesh_A7_vct1");
        vectors(&host, &shown, false, 20.0, NamedColor::WHITE, 0.5, true);
        host.fit();
        assert_view_draws(&host, "mesh_A7_vct2");
    });
}

#[test]
fn a_detected_edge_widens_with_the_edge_width() {
    run_viewer_case(
        "mesh_view_cases::a_detected_edge_widens_with_the_edge_width",
        || {
            let host = ViewerHost::open();
            host.view
                .set_proj_orientation(V3dTypeOfOrientation::XposYnegZpos, false)
                .unwrap();
            let shown = read_mesh(&host, MESH_BOX);
            let (x, y) = (206, 280);
            host.context.move_to(x, y, &host.view, true).unwrap();
            assert_eq!(host.pixel_name(x, y), NamedColor::BLACK, "one pixel wide");
            edge_width(&host, &shown, 6.0);
            assert_ne!(host.pixel_name(x, y), NamedColor::BLACK, "six pixels wide");
            assert_view_draws(&host, "mesh_bug10781");
        },
    );
}

#[test]
fn face_borders_show_in_shrink_mode() {
    run_viewer_case("mesh_view_cases::face_borders_show_in_shrink_mode", || {
        let host = ViewerHost::open();
        let shown = read_mesh(&host, MESH_BOX);
        host.context.move_to(157, 280, &host.view, true).unwrap();
        display_mode(&host, &shown, 3);
        host.assert_color_near(137, 277, [0., 0., 0.]);
        assert_view_draws(&host, "mesh_bug11095");
    });
}

#[test]
fn closed_meshes_are_capped_by_a_clipping_plane() {
    run_viewer_case(
        "mesh_view_cases::closed_meshes_are_capped_by_a_clipping_plane",
        || {
            let host = ViewerHost::open();
            host.view
                .set_proj_orientation(V3dTypeOfOrientation::XposYnegZpos, false)
                .unwrap();
            let shown = closed_mesh_grid(&host);
            host.fit();
            clip_with_capping(&host);
            for mesh in &shown {
                closed(&host, mesh, false);
            }
            assert_view_draws(&host, "mesh_bug26028_open");
            for mesh in &shown {
                closed(&host, mesh, true);
            }
            let capped = NamedColor::GRAY72;
            assert_eq!(host.pixel_name(125, 200), capped);
            assert_eq!(host.pixel_name(225, 300), capped);
            assert_ne!(
                host.pixel_name(325, 325),
                capped,
                "the third is not expected to be capped"
            );
            assert_eq!(host.pixel_name(225, 150), capped);
            assert_eq!(host.pixel_name(325, 200), capped);
            assert_view_draws(&host, "mesh_bug26028_closed");
        },
    );
}

fn closed_mesh_grid(host: &ViewerHost) -> Vec<ShownMesh> {
    let placed = |mode: Option<&str>, reflection: bool, offset: [f64; 3]| {
        let shown = read_mesh(host, MESH_SH1);
        match mode {
            Some(mode) => colors(host, &shown, mode, reflection),
            None => display_mode(host, &shown, SHADED),
        }
        move_by(host, &shown, offset);
        shown
    };
    vec![
        placed(None, false, [-100., 0., 0.]),
        placed(Some("elem1"), false, [0., 0., 0.]),
        placed(Some("elem2"), false, [100., 0., 0.]),
        placed(Some("nodal"), true, [0., 0., 100.]),
        placed(Some("nodaltex"), true, [100., 0., 100.]),
    ]
}

#[test]
fn hidden_elements_stay_hidden_under_nodal_colors() {
    run_viewer_case(
        "mesh_view_cases::hidden_elements_stay_hidden_under_nodal_colors",
        || {
            let host = ViewerHost::open();
            let shown = read_mesh(&host, MESH_BOX);
            let (x, y) = (180, 260);
            display_mode(&host, &shown, SHADED);
            host.assert_color_near(x, y, [0., 0., 0.36]);
            select_mode_on(&host, &shown.object, ELEMENTS);
            select_point(&host, x, y, Xor);
            hide_selected(&host, &shown);
            host.assert_color_near(x, y, [0., 0., 0.36]);
            colors(&host, &shown, "nodal", false);
            host.assert_color_near(x, y, [0.72, 0.72, 0.70]);
            assert_view_draws(&host, "vselect_bug10702");
        },
    );
}

#[test]
fn a_fit_of_the_selection_follows_the_mesh_location() {
    run_viewer_case(
        "mesh_view_cases::a_fit_of_the_selection_follows_the_mesh_location",
        || {
            let host = ViewerHost::open();
            let shown = read_mesh(&host, MESH_BEARING);
            host.fit();
            move_by(&host, &shown, [500., 0., 0.]);
            host.fit();
            host.view.set_zoom(0.5, true).unwrap();
            display_mode(&host, &shown, SHADED);
            select_mode_on(&host, &shown.object, ELEMENTS);
            select_point(&host, 200, 200, Replace);
            fit_selected(&host);
            assert_ne!(
                host.pixel_name(200, 200),
                NamedColor::BLACK,
                "mesh should be visible"
            );
            assert_view_draws(&host, "vselect_bug1164_2");
        },
    );
}

#[test]
fn a_polygon_selects_exact_counts_of_elements_and_nodes() {
    run_viewer_case(
        "mesh_view_cases::a_polygon_selects_exact_counts_of_elements_and_nodes",
        || {
            let host = ViewerHost::open();
            let shown = read_mesh(&host, MESH_HEAD);
            display_mode(&host, &shown, SHADED);
            select_mode_on(&host, &shown.object, ELEMENTS);
            select_polygon(&host, &[[0, 0], [409, 409], [0, 409]]);
            assert_eq!(host.context.nb_selected(), 55607, "mode 8");
            assert_view_draws(&host, "vselect_bug27008_mode_8");
            select_point(&host, 0, 0, Replace);
            select_mode_off(&host, &shown.object, ELEMENTS);
            select_mode_on(&host, &shown.object, NODES);
            select_polygon(&host, &[[0, 0], [409, 409], [0, 409]]);
            assert_eq!(host.context.nb_selected(), 30830, "mode 1");
            assert_view_draws(&host, "vselect_bug27008_mode_1");
        },
    );
}

#[test]
fn a_selected_mesh_does_not_hide_the_highlight_of_a_box() {
    run_viewer_case(
        "mesh_view_cases::a_selected_mesh_does_not_hide_the_highlight_of_a_box",
        || {
            let host = ViewerHost::open();
            let shown = read_mesh(&host, MESH_B);
            select_mode_on(&host, &shown.object, ELEMENTS);
            let solid_box = super::shape_view_cases::solid_box(100.);
            host.display_shape(&solid_box, None);
            host.fit();
            select_point(&host, 200, 200, Replace);
            select_point(&host, 290, 370, Replace);
            host.assert_color_near(321, 128, [1., 1., 0.]);
            host.assert_color_near(316, 321, [0.8, 0.8, 0.8]);
            select_point(&host, 200, 200, Replace);
            host.assert_color_near(321, 128, [0.8, 0.8, 0.8]);
            host.assert_color_near(316, 321, [0., 0., 0.]);
            assert_view_draws(&host, "vselect_bug26566");
        },
    );
}

fn assert_fit_shows_box(
    host: &ViewerHost,
    box_object: &martensite_tkernel::standard::Handle<
        dyn martensite_tkv3d::ais::AisInteractiveObject,
    >,
    name: &str,
) {
    host.context.display(box_object, true).unwrap();
    host.context.set_display_mode(box_object, 1, true).unwrap();
    box_object.set_hilight_mode(1);
    host.fit();
    assert_eq!(
        host.pixel_name(165, 400),
        NamedColor::DARKGOLDENROD,
        "{name}"
    );
    assert_view_draws(host, name);
}

#[test]
fn a_custom_selection_presentation_leaves_the_view_with_its_mesh() {
    run_viewer_case(
        "mesh_view_cases::a_custom_selection_presentation_leaves_the_view_with_its_mesh",
        || {
            let host = ViewerHost::open();
            let solid_box = super::shape_view_cases::box_at([-100., 0., 0.], [10., 20., 30.]);
            let box_object = host.shape_object(&solid_box);
            let shown = read_mesh(&host, MESH_BOX);
            host.fit();
            display_mode(&host, &shown, SHADED);
            select_mode_on(&host, &shown.object, ELEMENTS);
            select_polygon(&host, &[[0, 0], [409, 409], [0, 409]]);
            host.context.erase(&shown.object, true).unwrap();
            assert_fit_shows_box(&host, &box_object, "vselect_bug28527_1");
            host.context.erase(&box_object, true).unwrap();
            host.context.display(&shown.object, true).unwrap();
            host.fit();
            select_mode_on(&host, &shown.object, ELEMENTS);
            select_polygon(&host, &[[0, 0], [409, 409], [0, 409]]);
            host.context.remove(&shown.object, true).unwrap();
            assert_fit_shows_box(&host, &box_object, "vselect_bug28527_2");
        },
    );
}

#[test]
fn a_clipping_plane_of_a_mesh_applies_to_its_selection_presentation() {
    run_viewer_case(
        "mesh_view_cases::a_clipping_plane_of_a_mesh_applies_to_its_selection_presentation",
        || {
            let host = ViewerHost::open();
            host.view
                .set_proj_orientation(V3dTypeOfOrientation::XposYnegZpos, false)
                .unwrap();
            let shown = read_mesh(&host, MESH_SHAPE);
            host.fit();
            display_mode(&host, &shown, SHADED);
            select_mode_on(&host, &shown.object, ELEMENTS);
            select_rectangle(&host, [160, 170], [320, 370]);
            assert_ne!(host.pixel_name(230, 260), NamedColor::BLACK);
            clip_object(&shown, [0., 1., 0., 4.]);
            host.context.update_current_viewer();
            assert_eq!(host.pixel_name(230, 260), NamedColor::BLACK);
            host.context.move_to(75, 172, &host.view, true).unwrap();
            assert_eq!(host.pixel_name(70, 170), NamedColor::BLACK);
            assert_view_draws(&host, "vselect_bug29947");
        },
    );
}

#[derive(Clone, Default)]
struct PrintedProgress(Arc<Mutex<String>>);
impl ProgressIndicatorHooks for PrintedProgress {
    fn show(&self, scope: &ProgressScope<'_>, position: f64, _force: bool) {
        let _ = scope;
        self.0
            .lock()
            .unwrap()
            .push_str(&format!("Progress: {:.0}%\n", 100.0 * position));
    }
}

#[test]
fn reading_a_mesh_reports_progress_only_when_asked() {
    run_viewer_case(
        "mesh_view_cases::reading_a_mesh_reports_progress_only_when_asked",
        || {
            let host = ViewerHost::open();
            let silent = read_mesh(&host, MESH_BEARING);
            let hooks = PrintedProgress::default();
            let printed = hooks.0.clone();
            let indicator = ProgressIndicator::new(Box::new(hooks));
            let reported = mesh_from_stl(&host, &input(MESH_BEARING), &indicator.start());
            let text = printed.lock().unwrap().clone();
            assert!(text.contains("Progress:"), "{text:?}");
            let counts = |mesh: &ShownMesh| {
                let mesh = mesh.triangulation.get().unwrap();
                (mesh.nb_nodes(), mesh.nb_triangles())
            };
            assert_eq!(counts(&silent), counts(&reported));
        },
    );
}

#[test]
fn a_large_file_keeps_its_shared_nodes() {
    run_viewer_case(
        "mesh_view_cases::a_large_file_keeps_its_shared_nodes",
        || {
            let host = ViewerHost::open();
            let shown = read_mesh(&host, MESH_HEAD);
            let source = martensite_tkmeshvs::MeshVSMeshMembers::get_data_source(&*shown.mesh);
            assert_eq!(source.get_all_nodes().extent(), 64215, "nodes");
            assert_eq!(source.get_all_elements().extent(), 117694, "triangles");
            assert_view_draws(&host, "mesh_bug25405");
        },
    );
}
