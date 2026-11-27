use super::*;
use crate::surface_texture::SurfaceImageEncoding;

fn eye(byte: u8) -> Arc<SurfaceTextureImage> {
    Arc::new(SurfaceTextureImage {
        lifetime: Arc::default(),
        id: vec![],
        revision: 0,
        extent: [4, 2],
        depth: None,
        encoding: SurfaceImageEncoding::Srgb8,
        alpha_only: false,
        has_alpha: false,
        bytes: Arc::new(vec![byte; 32]),
        top_down: true,
        mipmaps: false,
        supplied_levels: vec![],
    })
}

fn params(mode: Graphic3dStereoMode) -> Graphic3dStereoParams {
    let mut params = martensite_tkservice::graphic3d::Graphic3dRenderingParams::new()
        .unwrap()
        .stereo;
    params.stereo_mode = mode;
    params
}

fn sampler_bytes(pair: &StereoPairComposite) -> [u8; 2] {
    pair.samplers.clone().map(|image| image.bytes[0])
}

#[test]
fn quad_buffer_page_flip_and_openvr_compose_red_cyan() {
    for mode in [
        Graphic3dStereoMode::QuadBuffer,
        Graphic3dStereoMode::SoftPageFlip,
        Graphic3dStereoMode::OpenVR,
    ] {
        assert_eq!(StereoProgram::of(mode), StereoProgram::RedCyan);
    }
    assert_eq!(
        StereoProgram::of(Graphic3dStereoMode::OverUnder),
        StereoProgram::OverUnder
    );
}

#[test]
fn window_row_oddness_swaps_the_eyes_of_interlaced_modes_only() {
    let row = params(Graphic3dStereoMode::RowInterlaced);
    let even = StereoPairComposite::new(&row, [eye(1), eye(2)], (0, 0, 4, 2));
    assert_eq!(sampler_bytes(&even), [1, 2]);
    // Bottom 3 is odd: the rows swap.
    let odd = StereoPairComposite::new(&row, [eye(1), eye(2)], (0, 1, 4, 3));
    assert_eq!(sampler_bytes(&odd), [2, 1]);
    // An odd left does not affect rows, but reverses columns; the chess board takes both.
    let column = params(Graphic3dStereoMode::ColumnInterlaced);
    let shifted = StereoPairComposite::new(&column, [eye(1), eye(2)], (1, 0, 5, 2));
    assert_eq!(sampler_bytes(&shifted), [2, 1]);
    let chess = params(Graphic3dStereoMode::ChessBoard);
    let both = StereoPairComposite::new(&chess, [eye(1), eye(2)], (1, 1, 5, 3));
    assert_eq!(sampler_bytes(&both), [1, 2]);
    let mut side = params(Graphic3dStereoMode::SideBySide);
    assert_eq!(
        sampler_bytes(&StereoPairComposite::new(
            &side,
            [eye(1), eye(2)],
            (1, 1, 5, 3)
        )),
        [1, 2]
    );
    side.to_reverse_stereo = true;
    assert_eq!(
        sampler_bytes(&StereoPairComposite::new(
            &side,
            [eye(1), eye(2)],
            (0, 0, 4, 2)
        )),
        [2, 1]
    );
}

#[test]
fn smooth_interlacing_offsets_half_an_eye_texel() {
    let mut chess = params(Graphic3dStereoMode::ChessBoard);
    // Graphic3dRenderingParams smooths interlacing by default.
    assert!(chess.to_smooth_interlacing);
    chess.to_smooth_interlacing = false;
    let plain = StereoPairComposite::new(&chess, [eye(1), eye(2)], (0, 0, 4, 2));
    assert_eq!(plain.tex_offset, [0., 0.]);
    chess.to_smooth_interlacing = true;
    let smooth = StereoPairComposite::new(&chess, [eye(1), eye(2)], (0, 0, 4, 2));
    assert_eq!(
        (smooth.tex_offset, smooth.view_size),
        ([0.125, -0.25], [4., 2.])
    );
    let mut row = params(Graphic3dStereoMode::RowInterlaced);
    row.to_smooth_interlacing = true;
    let rows = StereoPairComposite::new(&row, [eye(1), eye(2)], (0, 0, 4, 2));
    assert_eq!(rows.tex_offset, [0., -0.25]);
}

#[test]
fn anaglyph_filters_are_column_major_rows_of_the_native_matrices() {
    let mut anaglyph = params(Graphic3dStereoMode::Anaglyph);
    anaglyph.anaglyph_filter = Anaglyph::RedCyanOptimized;
    let pair = StereoPairComposite::new(&anaglyph, [eye(1), eye(2)], (0, 0, 4, 2));
    let [left, right] = pair.filters;
    // Row 0 of uMultL is (0.4154, 0.4710, 1/6, 0): column-major elements 0, 4, 8, 12.
    assert_eq!(
        [left[0], left[4], left[8], left[12]],
        [0.4154, 0.4710, 0.166_666_67, 0.]
    );
    assert_eq!(right[2 + 8], 1.2971);
    anaglyph.anaglyph_filter = Anaglyph::GreenMagentaSimple;
    let green = StereoPairComposite::new(&anaglyph, [eye(1), eye(2)], (0, 0, 4, 2));
    assert_eq!(green.filters[0].iter().sum::<f32>(), 1.);
    assert_eq!(
        (
            green.filters[0][5],
            green.filters[1][0],
            green.filters[1][10]
        ),
        (1., 1., 1.)
    );
    anaglyph.anaglyph_filter = Anaglyph::UserDefined;
    anaglyph.anaglyph_left = Mat4::identity();
    let user = StereoPairComposite::new(&anaglyph, [eye(1), eye(2)], (0, 0, 4, 2));
    assert_eq!(user.filters[0], Mat4::<f32>::identity().data());
    let rows = StereoPairComposite::new(
        &params(Graphic3dStereoMode::RowInterlaced),
        [eye(1), eye(2)],
        (0, 0, 4, 2),
    );
    assert_eq!(rows.filters, [[0.; 16]; 2]);
}
