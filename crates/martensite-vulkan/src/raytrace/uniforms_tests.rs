use super::*;

#[test]
fn identity_cameras_shoot_from_the_near_to_the_far_plane() {
    let camera = RaytraceCamera::new(&Mat4::identity(), &Mat4::identity());
    assert_eq!(camera.origins[0], [-1., -1., -1.]);
    assert_eq!(camera.directions[3], [0., 0., 2.]);
}

#[test]
fn the_block_places_members_at_their_std140_offsets() {
    let uniforms = RaytraceUniforms {
        camera: RaytraceCamera::new(&Mat4::identity(), &Mat4::identity()),
        window: [256, 128],
        ambient: [0.1; 4],
        back_top: [1.; 4],
        back_bottom: [0.; 4],
        light_count: 2,
        shadows: true,
        reflections: false,
        scene_radius: 7.,
        scene_epsilon: 1e-4,
        aperture_radius: 0.,
        focal_plane_distance: 1.,
        path_tracing: PathtraceUniforms {
            frame_seed: 9,
            blocked_rng: true,
            accum_samples: 4,
            max_radiance: 30.,
            tile_size: 24,
        },
        eye: RaytraceEye::default(),
        environment: crate::raytrace::environment::EnvironmentUniforms {
            enabled: true,
            for_background: false,
            coefficients: [-1, 1],
        },
    };
    let bytes = uniforms.bytes();
    let int = |at: usize| i32::from_ne_bytes(bytes[at..at + 4].try_into().unwrap());
    let float = |at: usize| f32::from_ne_bytes(bytes[at..at + 4].try_into().unwrap());
    assert_eq!((int(188), int(204), int(220), int(236)), (256, 128, 2, 1));
    // uOriginLT is the left-top corner (-1, 1) on the near plane.
    assert_eq!((float(176), float(180)), (-1., 1.));
    assert_eq!(float(300), 7.);
    assert_eq!((int(400), int(404), int(408), float(412)), (9, 1, 4, 30.));
    // uTileSize of adaptive sampling: square tiles.
    assert_eq!((int(384), int(388)), (24, 24));
    // uEnvMapEnabled, uEnvMapForBack, uYCoeff and uZCoeff.
    assert_eq!((int(268), int(284)), (1, 0));
    assert_eq!((int(392), int(396)), (-1, 1));
}
