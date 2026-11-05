//! Passive actual state serialization for the existing test fixture calls.
use super::BOPAlgoPaveFiller;
use crate::bop_ds::DataStructure;
use martensite_tkbrep::{brep::BRepTool, topods::Vertex};
use martensite_tkernel::message::Gravity;
use martensite_tkg3d::top_abs::ShapeEnum;

/// Reads source identities and records after the actual operation has joined.
/// Example: `let actual = vf_geometry_json(filler);` for a test diagnostic record.
pub(super) fn vf_geometry_json(filler: &BOPAlgoPaveFiller) -> String {
    let structure = filler.data_structure().unwrap();
    let shapes = structure.nb_shapes();
    let more = filler.iterator().unwrap().more();
    let vf = vf_records_json(structure);
    let vertices = vf_vertices_json(structure);
    let faces = vf_faces_json(structure);
    let errors = vf_alert_names(filler, Gravity::Fail);
    let warnings = vf_alert_names(filler, Gravity::Warning);
    format!(
        "\"shapes\":{shapes},\"iterator_more\":{more},\"vf\":[{vf}],\"vertices\":[{vertices}],\"faces\":[{faces}],\"errors\":{errors:?},\"warnings\":{warnings:?}"
    )
}

fn vf_records_json(structure: &DataStructure) -> String {
    structure
        .interf_vf_records()
        .iter()
        .map(|record| {
            let (u, v) = record.uv();
            format!(
                "[{},{},{},{},{}]",
                record.index1(),
                record.index2(),
                record.index_new(),
                u.to_bits(),
                v.to_bits()
            )
        })
        .collect::<Vec<_>>()
        .join(",")
}

fn vf_vertices_json(structure: &DataStructure) -> String {
    (0..structure.nb_shapes())
        .filter(|index| structure.shape_info(*index).unwrap().shape_type() == ShapeEnum::Vertex)
        .map(|index| {
            let vertex = Vertex::unchecked(structure.shape(index).unwrap().clone());
            let tolerance = BRepTool::vertex_tolerance(&vertex).unwrap().to_bits();
            format!(
                "[{index},{tolerance},{}]",
                structure.same_domain_index(index)
            )
        })
        .collect::<Vec<_>>()
        .join(",")
}

fn vf_faces_json(structure: &DataStructure) -> String {
    (0..structure.nb_shapes())
        .filter(|index| structure.shape_info(*index).unwrap().shape_type() == ShapeEnum::Face)
        .map(|index| {
            let initialized = structure.has_face_info(index).unwrap();
            let inside = structure
                .face_info(index)
                .unwrap()
                .vertices_in()
                .iter()
                .copied()
                .collect::<Vec<_>>();
            format!("[{index},{initialized},{inside:?}]")
        })
        .collect::<Vec<_>>()
        .join(",")
}

fn vf_alert_names(filler: &BOPAlgoPaveFiller, gravity: Gravity) -> Vec<String> {
    filler
        .options()
        .report()
        .get_alerts(gravity)
        .iter()
        .map(|alert| alert.dynamic_type().name().to_owned())
        .collect()
}
