//! The stereo pair composition GlView::drawStereoPair: the left and right
//! eye scenes (rendered with Graphic3dCamera::Projection_MonoLeftEye / MonoRightEye) sampled
//! bilinearly by the Graphic3dShaderManager::getStdProgramStereo program of the view's
//! Graphic3dStereoMode. QuadBuffer, SoftPageFlip and OpenVR without an XR session have no
//! stereo buffers here, so they compose with the default red/cyan program, as natively without
//! a quad-buffered context.
use crate::surface_texture::SurfaceTextureImage;
use martensite_tkernel::ncollection::{Mat4, Vec4};
use martensite_tkservice::graphic3d::{
    Graphic3dRenderingParamsAnaglyph as Anaglyph, Graphic3dStereoMode, Graphic3dStereoParams,
};
use std::sync::Arc;

/// The fragment program of getStdProgramStereo.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum StereoProgram {
    Anaglyph,
    RowInterlaced,
    ColumnInterlaced,
    ChessBoard,
    SideBySide,
    OverUnder,
    /// The default program: left eye red, right eye green and blue.
    RedCyan,
}

impl StereoProgram {
    /// The program of a stereo mode; `StereoProgram::of(Graphic3dStereoMode::Anaglyph)`.
    pub fn of(mode: Graphic3dStereoMode) -> Self {
        match mode {
            Graphic3dStereoMode::Anaglyph => Self::Anaglyph,
            Graphic3dStereoMode::RowInterlaced => Self::RowInterlaced,
            Graphic3dStereoMode::ColumnInterlaced => Self::ColumnInterlaced,
            Graphic3dStereoMode::ChessBoard => Self::ChessBoard,
            Graphic3dStereoMode::SideBySide => Self::SideBySide,
            Graphic3dStereoMode::OverUnder => Self::OverUnder,
            _ => Self::RedCyan,
        }
    }
    /// Whether the program reads `uTexOffset` (the interlaced programs).
    fn interlaced(self) -> bool {
        matches!(
            self,
            Self::RowInterlaced | Self::ColumnInterlaced | Self::ChessBoard
        )
    }
}

/// One composed stereo frame: the eye images bound to uLeftSampler and uRightSampler (after
/// `ToReverseStereo` and the window row oddness swap) and the program uniforms.
#[derive(Clone, Debug)]
pub(crate) struct StereoPairComposite {
    pub program: StereoProgram,
    pub samplers: [Arc<SurfaceTextureImage>; 2],
    /// uMultL and uMultR (column-major), anaglyph only.
    pub filters: [[f32; 16]; 2],
    /// uTexOffset, interlaced programs only.
    pub tex_offset: [f32; 2],
    /// The composed frame's size (the native window, whose gl_FragCoord the equivalence reads).
    pub view_size: [f32; 2],
    /// Whether the composed frame is presented flipped (GlView::myToFlipOutput).
    pub flip_output: bool,
}

impl StereoPairComposite {
    /// Compose `eyes` (left, right) for a window at `position` (left, top, right, bottom);
    /// `StereoPairComposite::new(&params.stereo, [left, right], window.position())`.
    pub fn new(
        stereo: &Graphic3dStereoParams,
        eyes: [Arc<SurfaceTextureImage>; 2],
        position: (i32, i32, i32, i32),
    ) -> Self {
        let program = StereoProgram::of(stereo.stereo_mode);
        let [width, height] = eyes[0].extent.map(|size| size as f32);
        let mut samplers = eyes;
        if reverses_pair(stereo, position) {
            samplers.swap(0, 1);
        }
        let filters = if program == StereoProgram::Anaglyph {
            anaglyph_filters(stereo)
        } else {
            [[0.; 16]; 2]
        };
        let tex_offset = if program.interlaced() && stereo.to_smooth_interlacing {
            smooth_offset(program, [width, height])
        } else {
            [0.; 2]
        };
        Self {
            program,
            samplers,
            filters,
            tex_offset,
            view_size: [width, height],
            flip_output: false,
        }
    }
    /// Compose into a frame of `size` presented flipped or not;
    /// `pair.into_target([640, 480], false)`.
    pub fn into_target(mut self, size: [u32; 2], flip_output: bool) -> Self {
        self.view_size = size.map(|value| value as f32);
        self.flip_output = flip_output;
        self
    }
}

/// `ToReverseStereo`, toggled for an odd window bottom (row interlaced, chess board) and an odd
/// window left (column interlaced, chess board) so that the equivalence follows the screen.
fn reverses_pair(stereo: &Graphic3dStereoParams, position: (i32, i32, i32, i32)) -> bool {
    let (left, top, _, bottom) = position;
    let mode = stereo.stereo_mode;
    let mut reverse = stereo.to_reverse_stereo;
    let rows = matches!(
        mode,
        Graphic3dStereoMode::RowInterlaced | Graphic3dStereoMode::ChessBoard
    );
    let columns = matches!(
        mode,
        Graphic3dStereoMode::ColumnInterlaced | Graphic3dStereoMode::ChessBoard
    );
    if (top + (bottom - top)) % 2 == 1 && rows {
        reverse = !reverse;
    }
    if left % 2 == 1 && columns {
        reverse = !reverse;
    }
    reverse
}

/// The half-texel offsets of `ToSmoothInterlacing`.
fn smooth_offset(program: StereoProgram, [width, height]: [f32; 2]) -> [f32; 2] {
    match program {
        StereoProgram::RowInterlaced => [0., -0.5 / height],
        StereoProgram::ColumnInterlaced => [0.5 / width, 0.],
        _ => [0.5 / width, -0.5 / height],
    }
}

/// uMultL and uMultR of the anaglyph filter, column-major as uploaded.
fn anaglyph_filters(stereo: &Graphic3dStereoParams) -> [[f32; 16]; 2] {
    if stereo.anaglyph_filter == Anaglyph::UserDefined {
        return [stereo.anaglyph_left.data(), stereo.anaglyph_right.data()];
    }
    let (left, right) = filter_rows(stereo.anaglyph_filter);
    [matrix_of_rows(&left), matrix_of_rows(&right)]
}

type FilterRows = Vec<(usize, [f32; 4])>;

/// The rows set on the zero matrices (`SetDiagonal(0)` then `SetRow`) per filter.
fn filter_rows(filter: Anaglyph) -> (FilterRows, FilterRows) {
    match filter {
        Anaglyph::RedCyanSimple => (
            vec![(0, [1., 0., 0., 0.])],
            vec![(1, [0., 1., 0., 0.]), (2, [0., 0., 1., 0.])],
        ),
        Anaglyph::RedCyanOptimized => (
            vec![
                (0, [0.4154, 0.4710, 0.166_666_67, 0.]),
                (1, [-0.0458, -0.0484, -0.0257, 0.]),
                (2, [-0.0547, -0.0615, 0.0128, 0.]),
            ],
            vec![
                (0, [-0.010_909_09, -0.036_363_64, -0.006_060_61, 0.]),
                (1, [0.375_6, 0.733_333_33, 0.011_111_11, 0.]),
                (2, [-0.0651, -0.1287, 1.2971, 0.]),
            ],
        ),
        Anaglyph::YellowBlueSimple => (
            vec![(0, [1., 0., 0., 0.]), (1, [0., 1., 0., 0.])],
            vec![(2, [0., 0., 1., 0.])],
        ),
        Anaglyph::YellowBlueOptimized => (
            vec![
                (0, [1.062, -0.205, 0.299, 0.]),
                (1, [-0.026, 0.908, 0.068, 0.]),
                (2, [-0.038, -0.173, 0.022, 0.]),
            ],
            vec![
                (0, [-0.016, -0.123, -0.017, 0.]),
                (1, [0.006, 0.062, -0.017, 0.]),
                (2, [0.094, 0.185, 0.911, 0.]),
            ],
        ),
        // GreenMagentaSimple (UserDefined returns earlier).
        _ => (
            vec![(1, [0., 1., 0., 0.])],
            vec![(0, [1., 0., 0., 0.]), (2, [0., 0., 1., 0.])],
        ),
    }
}

fn matrix_of_rows(rows: &FilterRows) -> [f32; 16] {
    let mut matrix = Mat4::<f32>::zero();
    for (row, [a, b, c, d]) in rows {
        matrix.set_row(*row, Vec4::new(*a, *b, *c, *d));
    }
    matrix.data()
}

#[cfg(test)]
#[path = "stereo_pair_tests.rs"]
mod tests;

/// Composites for backend unit tests.
#[cfg(test)]
pub(crate) mod tests_support {
    use super::*;
    /// A 1x1 black pair composed by `program`; `tests_support::composite(StereoProgram::RedCyan)`.
    pub fn composite(program: StereoProgram) -> StereoPairComposite {
        let eye = Arc::new(SurfaceTextureImage {
            lifetime: Arc::default(),
            id: vec![],
            revision: 0,
            extent: [1, 1],
            depth: None,
            encoding: crate::surface_texture::SurfaceImageEncoding::Srgb8,
            alpha_only: false,
            has_alpha: false,
            bytes: Arc::new(vec![0; 4]),
            top_down: true,
            mipmaps: false,
            supplied_levels: vec![],
        });
        StereoPairComposite {
            program,
            samplers: [eye.clone(), eye],
            filters: [[0.; 16]; 2],
            tex_offset: [0.; 2],
            view_size: [1.; 2],
            flip_output: false,
        }
    }
}
