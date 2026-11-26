// SPDX-License-Identifier: GPL-2.0-only
//! Contracts for parallel face meshing determinism (#55) and mid-operation
//! cancellation with zero published partial mesh (#56).
use super::{BRepMeshDiscretRoot, BRepMeshIncrementalMesh};
use crate::imesh_tools::MeshParameters;
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::brep_tools;
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::{Shape, face};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::poly::mesh_purpose;
use martensite_tkernel::message::{ProgressIndicator, ProgressIndicatorHooks, ProgressRange, ProgressScope};
use martensite_tkernel::storage::IoStream;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
use std::io::{Read, Seek, SeekFrom};

struct ImmediateCancelHooks;

impl ProgressIndicatorHooks for ImmediateCancelHooks {
    fn show(&self, _scope: &ProgressScope<'_>, _position: f64, _force: bool) {}
    fn user_break(&self) -> bool {
        true
    }
}

fn serialize_brep(shape: &Shape) -> Vec<u8> {
    let mut stream = IoStream::from_bytes(Vec::new());
    brep_tools::write(shape, &mut stream, &ProgressRange::default()).expect("write brep");
    let mut bytes = Vec::new();
    let mut device = stream.into_device().expect("memory device");
    device.seek(SeekFrom::Start(0)).expect("rewind");
    device.read_to_end(&mut bytes).expect("read");
    bytes
}

#[test]
fn parallel_face_meshing_is_byte_identical_to_sequential() {
    let sequential_shape = BRepPrimAPIMakeBox::from_sizes(2.0, 3.0, 4.0)
        .expect("box")
        .shape()
        .expect("shape")
        .clone();
    let parallel_shape = BRepPrimAPIMakeBox::from_sizes(2.0, 3.0, 4.0)
        .expect("box")
        .shape()
        .expect("shape")
        .clone();

    let seq = BRepMeshIncrementalMesh::with_shape(&sequential_shape, 0.1, false, 0.5, false)
        .expect("sequential mesh");
    let par = BRepMeshIncrementalMesh::with_shape(&parallel_shape, 0.1, false, 0.5, true)
        .expect("parallel mesh");

    assert!(seq.is_done());
    assert!(par.is_done());
    assert_eq!(seq.get_status_flags(), 0);
    assert_eq!(par.get_status_flags(), 0);
    assert_eq!(serialize_brep(&parallel_shape), serialize_brep(&sequential_shape));
}

#[test]
fn cancelled_meshing_publishes_no_partial_triangulation() {
    let shape = BRepPrimAPIMakeBox::from_sizes(2.0, 3.0, 4.0)
        .expect("box")
        .shape()
        .expect("shape")
        .clone();
    let indicator = ProgressIndicator::new(Box::new(ImmediateCancelHooks));
    let range = indicator.start();
    let parameters = MeshParameters {
        deflection: 0.1,
        angle: 0.5,
        in_parallel: true,
        ..MeshParameters::default()
    };

    let _mesher = BRepMeshIncrementalMesh::with_parameters(&shape, &parameters, &range)
        .expect("cancelled mesher must return cleanly");

    let mut explorer = Explorer::new(&shape, ShapeEnum::Face, ShapeEnum::Shape).expect("explorer");
    let mut triangulated_faces = 0usize;
    while explorer.more() {
        let face = face(explorer.current()).expect("face");
        if BRepTool::triangulation(&face, mesh_purpose::NONE)
            .is_ok_and(|(tri, _)| !tri.is_null())
        {
            triangulated_faces += 1;
        }
        explorer.next().expect("next face");
    }
    assert_eq!(
        triangulated_faces, 0,
        "cancelled meshing must not publish partial face triangulations"
    );
}
