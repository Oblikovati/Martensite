use super::*;

/// Published samplers move and share across threads like the other renderer values.
const _: fn() = || {
    fn require_send_sync<T: Send + Sync>() {}
    require_send_sync::<RaytraceTileSampler>();
    require_send_sync::<RaytraceHaltonSampler>();
};

/// A 100x70 view in 32-pixel tiles: 4x3 tiles, the last column 4 and the last row 6 pixels.
fn sized(nb_tiles: i32) -> RaytraceTileSampler {
    let mut sampler = RaytraceTileSampler::default();
    sampler.set_size(32, nb_tiles, [100, 70]);
    sampler
}

#[test]
fn set_size_counts_clipped_tiles_and_the_error_scale() {
    let sampler = sized(0);
    assert_eq!((sampler.nb_tiles_x(), sampler.nb_tiles_y()), (4, 3));
    assert_eq!(sampler.nb_tiles(), 12);
    assert_eq!(sampler.tile_size(), [32, 32]);
    assert_eq!(sampler.view_size(), [100, 70]);
    assert_eq!(sampler.variance_scale_factor(), 1.0e6);
    assert_eq!(sampler.tile_area(0, 0), 1024);
    assert_eq!(sampler.tile_area(3, 0), 4 * 32);
    assert_eq!(sampler.tile_area(3, 2), 4 * 6);
    // Without a shrunk tile count both offset maps cover the full grid.
    assert_eq!(sampler.nb_offset_tiles(true), [4, 3]);
    assert_eq!(sampler.offset_tiles_viewport(false), [128, 96]);
    assert_eq!(sampler.max_tile_samples(), 1);
}

#[test]
fn shrunk_offsets_double_alternately_from_eight_by_eight() {
    let sampler = sized(256);
    assert_eq!(sampler.nb_offset_tiles(true), [16, 16]);
    assert_eq!(sampler.nb_offset_tiles_max(), [16, 16]);
    assert_eq!(sampler.offset_tiles_viewport_max(), [512, 512]);
    assert_eq!(sized(65).nb_offset_tiles(true), [16, 8]);
    assert_eq!(sized(64).nb_offset_tiles(true), [8, 8]);
}

#[test]
fn an_empty_view_keeps_the_maps_and_an_unsized_sampler_uploads_nothing() {
    let mut sampler = sized(0);
    sampler.set_size(32, 0, [0, 50]);
    assert_eq!(sampler.view_size(), [100, 70]);
    assert!(
        RaytraceTileSampler::default()
            .update_offsets(false)
            .is_none()
    );
}

#[test]
fn the_regular_upload_redirects_each_tile_to_itself() {
    let mut sampler = sized(0);
    let offsets = sampler.update_offsets(false).unwrap().to_vec();
    assert_eq!(offsets[0], [0, 0]);
    assert_eq!(offsets[5], [1, 1]);
    assert_eq!(offsets[11], [3, 2]);
    assert_eq!(sampler.max_tile_samples(), 1);
    assert_eq!(sampler.tile_samples()[11], 24);
}

#[test]
fn the_variance_map_rejects_wrong_sizes_and_negative_errors() {
    let mut sampler = sized(0);
    assert!(sampler.grab_variance_map(&[0; 11]).is_err());
    let mut raw = [0; 12];
    raw[4] = -1;
    assert!(sampler.grab_variance_map(&raw).is_err());
}

#[test]
fn adaptive_tiles_follow_the_error_distribution() {
    let mut sampler = sized(64);
    // All the error in tile (2, 1): every draw lands there except the first, whose Halton
    // point (0, 0) selects the first tile of a zero-error column (native `<=` search).
    let mut raw = [0; 12];
    raw[4 + 2] = 1_000_000;
    sampler.grab_variance_map(&raw).unwrap();
    let offsets = sampler.update_offsets(true).unwrap().to_vec();
    assert_eq!(offsets.len(), 64);
    assert_eq!(offsets[0], [0, 0]);
    assert!(
        offsets[1..].iter().all(|offset| *offset == [2, 1]),
        "{offsets:?}"
    );
    assert_eq!(sampler.max_tile_samples(), 63);
    assert_eq!(sampler.tile_samples()[6], 63 * 1024);
    assert_eq!(sampler.tile_samples()[0], 1024);
    // Restarting the sequence repeats the same draws.
    sampler.reset();
    let again = sampler.update_offsets(true).unwrap().to_vec();
    assert_eq!(again, offsets);
}

#[test]
fn uniform_error_spreads_the_draws_over_the_tiles() {
    let mut sampler = sized(64);
    // Error proportional to the area: uniform per pixel, so the draws follow the Halton points.
    let raw: Vec<i32> = (0..12)
        .map(|tile| sampler.tile_area(tile % 4, tile / 4) * 100)
        .collect();
    sampler.grab_variance_map(&raw).unwrap();
    sampler.update_offsets(true).unwrap();
    // The first draw is Halton (0, 0): the first column and row.
    sampler.reset();
    let first = sampler.update_offsets(true).unwrap()[0];
    assert_eq!(first, [0, 0]);
    assert!(sampler.max_tile_samples() < 64);
}
