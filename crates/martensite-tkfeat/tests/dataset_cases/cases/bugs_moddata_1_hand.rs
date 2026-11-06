//! The `bugs/moddata_1` scripts that branch on what a command answers.
use crate::support::{
    checkview::{CheckView, checkview},
    commands::explode,
    construction::{edge, vertex},
    feat_script::FeatScript,
    inspect::renamevar,
    nbshapes::{Counting, nbshapes},
    splitshape::{buc60854 as split_side, splitshape},
    surface_variables::{mkface_bounded, plane_xoy},
};
use martensite_tkg3d::top_abs::ShapeEnum;

/// Where a dump lists the faces: vertices, edges, wires, then faces.
const FACE_SLOT: usize = 3;

pub fn buc60854(s: &mut FeatScript<'_>) {
    println!("========");
    println!("BUC60854");
    println!("========");
    println!();
    plane_xoy(s, "test_plane");
    mkface_bounded(s, "test_face", "test_plane", (0., 1., 0., 1.));
    vertex(s, "test_vertex_1", [0., 0.5, 0.]);
    vertex(s, "test_vertex_2", [1., 0.5, 0.]);
    edge(s, "test_edge", "test_vertex_1", "test_vertex_2");
    splitshape(
        s,
        "test_shape",
        "test_face",
        &["test_face", "test_edge"],
        &[],
    );
    explode(s, "test_shape", Some(ShapeEnum::Face));
    renamevar(s, "test_shape_1", "test_face");
    vertex(s, "test_vertex_3", [0.2, 0.5, 0.]);
    vertex(s, "test_vertex_4", [0.8, 0.5, 0.]);
    edge(s, "test_edge_1", "test_vertex_4", "test_vertex_3");
    vertex(s, "test_vertex_5", [0., 1., 0.]);
    edge(s, "test_edge_2", "test_vertex_3", "test_vertex_5");
    vertex(s, "test_vertex_6", [1., 1., 0.]);
    edge(s, "test_edge_3", "test_vertex_6", "test_vertex_4");
    let tools = ["test_edge_2", "test_edge_3"];
    let on_edges = [("test_edge", "test_edge_1")];
    split_side(
        s,
        "result",
        "test_shape",
        "test_face",
        &tools,
        &on_edges,
        true,
    );
    if nbshapes(s, "result", Counting::Located)[FACE_SLOT] != 3 {
        println!("BUC60854: Error");
    }
    split_side(
        s,
        "result_2",
        "test_shape",
        "test_face",
        &tools,
        &on_edges,
        false,
    );
    if nbshapes(s, "result_2", Counting::Located)[FACE_SLOT] != 1 {
        println!("BUC60854: Error");
    }
    checkview(s, &CheckView::of("result").with(&["result_2"]));
}
