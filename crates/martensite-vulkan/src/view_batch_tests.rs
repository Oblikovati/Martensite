use super::{probe_packet, shares_packet};
use crate::backend::directional_phong_fixture::lit_surface;

#[test]
fn draws_that_differ_only_in_geometry_share_one_packet() {
    let first = lit_surface(6);
    let longer = lit_surface(12);
    let packet = probe_packet(&first).unwrap();
    assert!(shares_packet(&first, &packet));
    assert!(shares_packet(&longer, &packet));
    assert_eq!(packet.vertices.len(), 0);
    assert_eq!(packet.lighting.as_ref().unwrap().normals.len(), 0);
}

#[test]
fn a_changed_body_or_lighting_or_frame_field_leaves_the_packet() {
    let packet = probe_packet(&lit_surface(6)).unwrap();
    let mut mirrored = lit_surface(6);
    mirrored.mirrored = true;
    let mut material = lit_surface(6);
    material.lighting.as_mut().unwrap().material.common[0].specular_shininess[3] += 1.;
    let mut culled = lit_surface(6);
    culled.depth_clamp = !culled.depth_clamp;
    for draw in [mirrored, material, culled] {
        assert!(!shares_packet(&draw, &packet));
    }
}
