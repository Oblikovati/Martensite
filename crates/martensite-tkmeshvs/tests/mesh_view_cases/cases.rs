// SPDX-License-Identifier: GPL-2.0-only
//! The cases, step by step. What a case selects by pixel position is the same set in the reference
//! images; the sets below were read from a run whose images match them pixel for pixel.
use crate::color_commands::{color_elements, color_nodes, color_nodes_by_texture};
use crate::mesh_commands::*;
use crate::view_host::ViewHost;
use martensite_process_policy::case_runner::{CaseRunner, CaseSpec, FactorProfile};
use martensite_tkernel::quantity::NamedColor;
use martensite_tkmeshvs::MeshVSMeshMembers as _;
use martensite_tkmeshvs::selection_policy::{MESHVS_SMF_NODE, MESHVS_SMF_VOLUME};

/// Runs one case alone in a child process, under the budget of 1000 CPU seconds of its group.
fn run_alone(test_name: &'static str, body: impl FnOnce()) {
    let identity = format!("cases::{test_name}");
    let root = std::env::temp_dir().join("martensite-mesh-view-cases");
    let spec = CaseSpec::new(&identity, 1000, FactorProfile::RequireUnset);
    let receipt = CaseRunner::new(&root, None).run(spec, || {
        assert_eq!(martensite_process_policy::reported_process_limit(), 1000);
        body();
        Ok(())
    });
    receipt.unwrap_or_else(|error| panic!("{identity}: {error}"));
}

const ALL_ELEMENTS: [i32; 5] = [1, 2, 3, 4, 5];
const ALL_NODES: [i32; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
/// The nodes the box from (0, 0) to (300, 200) holds in the turned view.
const NODES_IN_BOX: [i32; 8] = [1, 2, 4, 5, 6, 7, 8, 11];
/// The nodes outside that box.
const NODES_OUT_OF_BOX: [i32; 8] = [3, 9, 10, 12, 13, 14, 15, 16];
const YELLOW: [f64; 3] = [1., 1., 0.];
const GREEN: [f64; 3] = [0., 1., 0.];
const WHITE: [f64; 3] = [1., 1., 1.];

/// Arrows on the elements, whose model has no normals, so they all take the Z axis.
const ELEMENT_ARROWS: VectorRequest = VectorRequest {
    mode: VectorMode::Element,
    max_length: 10.,
    color: NamedColor::GREEN,
    arrow_part: 0.5,
    is_simple: false,
};
/// Simple lines on the nodes, along each node's position.
const NODE_LINES: VectorRequest = VectorRequest {
    mode: VectorMode::Nodal,
    max_length: 20.,
    color: NamedColor::WHITE,
    arrow_part: 0.1,
    is_simple: true,
};

/// The opening of most cases: the model, turned twice.
fn turned_mesh(host: &ViewHost) -> DisplayedMesh {
    let mesh = create_element_mesh(host);
    host.rotate(1.5, 0., 0.);
    host.rotate(0., 0.5, 0.);
    mesh
}

fn assert_hidden(mesh: &DisplayedMesh, elements: &[i32], nodes: &[i32]) {
    assert_eq!(
        hidden_ids(&mesh.mesh.get_hidden_elems()),
        elements,
        "hidden elements"
    );
    assert_eq!(
        hidden_ids(&mesh.mesh.get_hidden_nodes()),
        nodes,
        "hidden nodes"
    );
}

/// Wire, shaded and shrunk draw more and more; a hidden mesh draws nothing.
#[test]
fn display_modes_then_hiding_the_mesh() {
    run_alone("display_modes_then_hiding_the_mesh", || {
        let host = ViewHost::open("B1");
        let mesh = turned_mesh(&host);
        let wire = host.dump("crtd");
        assert!(wire.of_color(YELLOW) > 0, "the wireframe has yellow edges");
        host.set_display_mode(&mesh.object, 2);
        let shaded = host.dump("m2");
        host.set_display_mode(&mesh.object, 3);
        let shrunk = host.dump("m3");
        assert!(
            wire.drawn() < shrunk.drawn(),
            "wire {} shrunk {}",
            wire.drawn(),
            shrunk.drawn()
        );
        assert!(
            shrunk.drawn() < shaded.drawn(),
            "shrunk {} shaded {}",
            shrunk.drawn(),
            shaded.drawn()
        );
        hide_mesh(&host, &mesh);
        assert_eq!(host.dump("hdn").drawn(), 0, "a hidden mesh draws nothing");
        host.close();
    });
}

#[test]
fn hiding_two_picked_volumes() {
    run_alone("hiding_two_picked_volumes", || {
        let mut host = ViewHost::open("B2");
        let mesh = turned_mesh(&host);
        host.activate_selection(&mesh.object, MESHVS_SMF_VOLUME.0);
        host.toggle_at(200, 200);
        host.toggle_at(200, 300);
        hide_selected(&host, &mesh);
        assert_hidden(&mesh, &[2, 5], &ALL_NODES);
        assert!(
            host.dump("B2").of_color(YELLOW) > 0,
            "the other volumes stay drawn"
        );
        host.close();
    });
}

#[test]
fn showing_only_two_picked_volumes() {
    run_alone("showing_only_two_picked_volumes", || {
        let mut host = ViewHost::open("B3");
        let mesh = turned_mesh(&host);
        host.activate_selection(&mesh.object, MESHVS_SMF_VOLUME.0);
        host.toggle_at(200, 200);
        host.toggle_at(200, 300);
        show_only_selected(&host, &mesh);
        assert_hidden(&mesh, &[1, 3, 4], &ALL_NODES);
        assert!(host.dump("B3").drawn() > 0, "the picked volumes stay drawn");
        host.close();
    });
}

#[test]
fn showing_only_the_nodes_in_a_box() {
    run_alone("showing_only_the_nodes_in_a_box", || {
        let mut host = ViewHost::open("B4");
        let mesh = turned_mesh(&host);
        host.activate_selection(&mesh.object, MESHVS_SMF_NODE.0);
        host.select_box((0, 0), (300, 200));
        show_only_selected(&host, &mesh);
        assert_hidden(&mesh, &ALL_ELEMENTS, &NODES_OUT_OF_BOX);
        assert!(host.dump("B4").drawn() > 0, "the boxed nodes stay drawn");
        host.close();
    });
}

#[test]
fn hiding_the_nodes_in_a_box() {
    run_alone("hiding_the_nodes_in_a_box", || {
        let mut host = ViewHost::open("B5");
        let mesh = turned_mesh(&host);
        show_all(&host, &mesh);
        host.activate_selection(&mesh.object, MESHVS_SMF_NODE.0);
        host.select_box((0, 0), (300, 200));
        hide_selected(&host, &mesh);
        assert_hidden(&mesh, &[], &NODES_IN_BOX);
        assert!(
            host.dump("B5").of_color(YELLOW) > 0,
            "the elements stay drawn"
        );
        host.close();
    });
}

/// Colors change what is drawn, not where: the three images cover the same pixels.
#[test]
fn element_node_and_texture_colors() {
    run_alone("element_node_and_texture_colors", || {
        let host = ViewHost::open("B6");
        let mesh = turned_mesh(&host);
        color_elements(&host, &mesh, false);
        let elements = host.dump("el1");
        color_nodes(&host, &mesh, true);
        let nodes = host.dump("nd");
        color_nodes_by_texture(&host, &mesh, true);
        let texture = host.dump("ndtx");
        assert!(elements.drawn() > 0, "the mesh is drawn");
        assert_eq!(
            elements.drawn(),
            nodes.drawn(),
            "element against node colors"
        );
        assert_eq!(
            elements.drawn(),
            texture.drawn(),
            "element against texture colors"
        );
        assert!(
            elements.colors() >= ALL_ELEMENTS.len(),
            "a color per element"
        );
        assert!(
            nodes.colors() > elements.colors(),
            "node colors blend across faces"
        );
        assert!(
            texture.colors() > elements.colors(),
            "the ramp blends across faces"
        );
        host.close();
    });
}

#[test]
fn element_labels_and_two_kinds_of_vectors() {
    run_alone("element_labels_and_two_kinds_of_vectors", || {
        let host = ViewHost::open("B7");
        let mesh = turned_mesh(&host);
        add_element_labels(&mesh);
        host.set_display_mode(&mesh.object, 33);
        assert!(
            host.dump("txt").of_color(YELLOW) > 0,
            "yellow labels and edges"
        );
        show_vectors(&host, &mesh, &ELEMENT_ARROWS);
        host.set_display_mode(&mesh.object, 5);
        assert!(host.dump("vct1").of_color(GREEN) > 0, "green arrows");
        show_vectors(&host, &mesh, &NODE_LINES);
        host.fit();
        assert!(host.dump("vct2").of_color(WHITE) > 0, "white lines");
        host.close();
    });
}

#[test]
fn deforming_the_mesh_moves_what_is_drawn() {
    run_alone("deforming_the_mesh_moves_what_is_drawn", || {
        let host = ViewHost::open("B8");
        let mesh = turned_mesh(&host);
        let undeformed = host.dump("undeformed");
        deform_mesh(&host, &mesh, 20.);
        host.rotate(0., -0.7, 0.);
        let deformed = host.dump("B8");
        assert!(deformed.drawn() > 0, "the deformed mesh is drawn");
        assert!(deformed.differing(&undeformed) > 0, "the deformation shows");
        host.close();
    });
}

/// A regression: arrows used to take a random direction, so repeating the request changed them.
/// The three images are equal. This request keeps the arrow presentation.
#[test]
fn repeating_the_vectors_leaves_them_in_place() {
    run_alone("repeating_the_vectors_leaves_them_in_place", || {
        let host = ViewHost::open("bug27905");
        let mesh = create_element_mesh(&host);
        host.set_display_mode(&mesh.object, 5);
        show_vectors(&host, &mesh, &ELEMENT_ARROWS);
        let first = host.dump("1");
        show_vectors(&host, &mesh, &ELEMENT_ARROWS);
        let second = host.dump("2");
        show_vectors(&host, &mesh, &ELEMENT_ARROWS);
        let third = host.dump("3");
        assert!(first.of_color(GREEN) > 0, "green arrows");
        assert_eq!(
            first.differing(&second),
            0,
            "second image against the first"
        );
        assert_eq!(first.differing(&third), 0, "third image against the first");
        host.close();
    });
}
