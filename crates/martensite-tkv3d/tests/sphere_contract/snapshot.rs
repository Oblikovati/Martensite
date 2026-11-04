//! The byte schema stores integer words in little-endian order, with floats encoded by bits.
use super::style;
use martensite_tkmath::poly::Triangulation;
use martensite_tkv3d::{ais::AisTriangulation, prs_mgr::PrsMgrPresentableObjectMembers as _};

pub(super) fn object_bytes(object: &AisTriangulation) -> Vec<u8> {
    let mut bytes = b"sphere-factory-snapshot-v1\0".to_vec();
    let triangulation = object.get_triangulation();
    let tri = triangulation.get().unwrap();
    header(&mut bytes, tri);
    nodes(&mut bytes, tri);
    triangles(&mut bytes, tri);
    colors(&mut bytes, object, tri.nb_nodes());
    style::appearance(&mut bytes, object);
    bytes
}

fn header(bytes: &mut Vec<u8>, tri: &Triangulation) {
    bytes.extend(tri.nb_nodes().to_le_bytes());
    bytes.extend(tri.nb_triangles().to_le_bytes());
    bytes.extend([
        u8::from(tri.is_double_precision()),
        u8::from(tri.has_normals()),
        u8::from(tri.has_uv_nodes()),
        u8::from(tri.has_cached_min_max()),
    ]);
    bytes.extend(tri.deflection().to_bits().to_le_bytes());
}

fn nodes(bytes: &mut Vec<u8>, tri: &Triangulation) {
    let mut bounds = [
        f64::INFINITY,
        f64::INFINITY,
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::NEG_INFINITY,
        f64::NEG_INFINITY,
    ];
    for index in 1..=tri.nb_nodes() {
        node_bytes(bytes, tri, index, &mut bounds);
    }
    for value in bounds {
        bytes.extend(value.to_bits().to_le_bytes());
    }
}

fn node_bytes(bytes: &mut Vec<u8>, tri: &Triangulation, index: i32, bounds: &mut [f64; 6]) {
    let point = tri.node(index).unwrap();
    let xyz = [point.x(), point.y(), point.z()];
    for (axis, value) in xyz.into_iter().enumerate() {
        bytes.extend(value.to_bits().to_le_bytes());
        bounds[axis] = bounds[axis].min(value);
        bounds[axis + 3] = bounds[axis + 3].max(value);
    }
    let normal = tri.normal_vec3(index).unwrap();
    for value in [normal.x(), normal.y(), normal.z()] {
        bytes.extend(value.to_bits().to_le_bytes());
    }
}

fn triangles(bytes: &mut Vec<u8>, tri: &Triangulation) {
    for index in 1..=tri.nb_triangles() {
        let (a, b, c) = tri.triangle(index).unwrap().get();
        for vertex in [a, b, c] {
            bytes.extend(vertex.to_le_bytes());
        }
    }
}

fn colors(bytes: &mut Vec<u8>, object: &AisTriangulation, count: i32) {
    bytes.push(u8::from(object.has_vertex_colors()));
    let colors = object.get_colors();
    let colors = colors.get().unwrap();
    for index in 1..=count {
        bytes.extend(colors.array1().value(index).unwrap().to_le_bytes());
    }
    bytes.push(u8::from(object.attributes().has_own_shading_aspect()));
}
