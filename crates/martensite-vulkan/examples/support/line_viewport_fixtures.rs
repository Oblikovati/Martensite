//! Owned input bits for independent viewport line captures; no runtime reference process.
use martensite_tkernel::ncollection::Mat4;
use martensite_vulkan::{LineStyle, PrimitiveDraw, RenderError, RenderVertex};
/// One finite two-point input; `VIEWPORT_LINES[0].draw()?` retains its source endpoints.
#[derive(Clone, Copy)]
pub struct ViewportLineFixture {
    pub name: &'static str,
    pub perspective: bool,
    pub varying_colors: bool,
    pub point_bits: [[u32; 3]; 2],
}
const ORTHOGRAPHIC: [[u32; 4]; 4] = [
    [0x3f800000, 0x00000000, 0x00000000, 0x00000000],
    [0x00000000, 0x3f800000, 0x00000000, 0x00000000],
    [0x00000000, 0x00000000, 0xbca57eb5, 0xbf8295fb],
    [0x00000000, 0x00000000, 0x00000000, 0x3f800000],
];
const PERSPECTIVE: [[u32; 4]; 4] = [
    [0x401a8279, 0x00000000, 0x00000000, 0x00000000],
    [0x00000000, 0x401a8279, 0x00000000, 0x00000000],
    [0x00000000, 0x00000000, 0xbf8295fb, 0xc0014afd],
    [0x00000000, 0x00000000, 0xbf800000, 0x00000000],
];
const ORIENTATION: [[u32; 4]; 4] = [
    [0x3f800000, 0x00000000, 0x00000000, 0x00000000],
    [0x00000000, 0x3f800000, 0x00000000, 0x00000000],
    [0x00000000, 0x00000000, 0x3f800000, 0xc1a00000],
    [0x00000000, 0x00000000, 0x00000000, 0x3f800000],
];
/// The fixed eighteen-input corpus; `VIEWPORT_LINES.iter()` preserves declared order.
pub const VIEWPORT_LINES: [ViewportLineFixture; 18] = [
    ViewportLineFixture {
        name: "plane-0",
        perspective: false,
        varying_colors: false,
        point_bits: [
            [0xbf467e90, 0xbe94f2a0, 0x00000000],
            [0xbfb306a0, 0xbf36b6a0, 0x00000000],
        ],
    },
    ViewportLineFixture {
        name: "plane-0-reverse",
        perspective: false,
        varying_colors: false,
        point_bits: [
            [0xbfb306a0, 0xbf36b6a0, 0x00000000],
            [0xbf467e90, 0xbe94f2a0, 0x00000000],
        ],
    },
    ViewportLineFixture {
        name: "plane-1",
        perspective: false,
        varying_colors: false,
        point_bits: [
            [0x3e94f2a0, 0xbf467e90, 0x00000000],
            [0x3f36b6a0, 0xbfb306a0, 0x00000000],
        ],
    },
    ViewportLineFixture {
        name: "plane-1-reverse",
        perspective: false,
        varying_colors: false,
        point_bits: [
            [0x3f36b6a0, 0xbfb306a0, 0x00000000],
            [0x3e94f2a0, 0xbf467e90, 0x00000000],
        ],
    },
    ViewportLineFixture {
        name: "plane-2",
        perspective: false,
        varying_colors: false,
        point_bits: [
            [0x3f467e90, 0x3e94f2a0, 0x00000000],
            [0x3fb306a0, 0x3f36b6a0, 0x00000000],
        ],
    },
    ViewportLineFixture {
        name: "plane-2-reverse",
        perspective: false,
        varying_colors: false,
        point_bits: [
            [0x3fb306a0, 0x3f36b6a0, 0x00000000],
            [0x3f467e90, 0x3e94f2a0, 0x00000000],
        ],
    },
    ViewportLineFixture {
        name: "plane-3",
        perspective: false,
        varying_colors: false,
        point_bits: [
            [0xbe94f2a0, 0x3f467e90, 0x00000000],
            [0xbf36b6a0, 0x3fb306a0, 0x00000000],
        ],
    },
    ViewportLineFixture {
        name: "plane-3-reverse",
        perspective: false,
        varying_colors: false,
        point_bits: [
            [0xbf36b6a0, 0x3fb306a0, 0x00000000],
            [0xbe94f2a0, 0x3f467e90, 0x00000000],
        ],
    },
    ViewportLineFixture {
        name: "inside",
        perspective: false,
        varying_colors: false,
        point_bits: [
            [0xbf400000, 0xbe800000, 0x00000000],
            [0x3f400000, 0x3e800000, 0x00000000],
        ],
    },
    ViewportLineFixture {
        name: "offgrid",
        perspective: false,
        varying_colors: false,
        point_bits: [
            [0xbf40509c, 0xbe81d3ed, 0x00000000],
            [0x3f41a565, 0x3e840cc8, 0x00000000],
        ],
    },
    ViewportLineFixture {
        name: "horizontal-cross",
        perspective: false,
        varying_colors: false,
        point_bits: [
            [0xbfc00000, 0x3e000000, 0x00000000],
            [0x3fc00000, 0x3e000000, 0x00000000],
        ],
    },
    ViewportLineFixture {
        name: "vertical-cross",
        perspective: false,
        varying_colors: false,
        point_bits: [
            [0x3e000000, 0xbfc00000, 0x00000000],
            [0x3e000000, 0x3fc00000, 0x00000000],
        ],
    },
    ViewportLineFixture {
        name: "outside-reject",
        perspective: false,
        varying_colors: false,
        point_bits: [
            [0xbfc00000, 0xbf400000, 0x00000000],
            [0xbfa00000, 0x3f400000, 0x00000000],
        ],
    },
    ViewportLineFixture {
        name: "perspective-w",
        perspective: true,
        varying_colors: false,
        point_bits: [
            [0xc0400000, 0xbf800000, 0x41200000],
            [0x41700000, 0x40a00000, 0xc1200000],
        ],
    },
    ViewportLineFixture {
        name: "near-cross",
        perspective: true,
        varying_colors: false,
        point_bits: [
            [0xbd4ccccd, 0x00000000, 0x419c0000],
            [0x3f000000, 0x3dcccccd, 0x41900000],
        ],
    },
    ViewportLineFixture {
        name: "near-reject",
        perspective: true,
        varying_colors: false,
        point_bits: [
            [0xbdcccccd, 0x00000000, 0x419e0000],
            [0x3dcccccd, 0x3dcccccd, 0x419c0000],
        ],
    },
    ViewportLineFixture {
        name: "depth-slope",
        perspective: false,
        varying_colors: false,
        point_bits: [
            [0xbf400000, 0xbe800000, 0x41200000],
            [0x3f400000, 0x3e800000, 0xc1200000],
        ],
    },
    ViewportLineFixture {
        name: "varying-cross",
        perspective: false,
        varying_colors: true,
        point_bits: [
            [0xbfc00000, 0xbe99999a, 0x00000000],
            [0x3f400000, 0x3ecccccd, 0x00000000],
        ],
    },
];

impl ViewportLineFixture {
    /// Construct the retained source segment without clipping it; `fixture.draw()?`.
    pub fn draw(self) -> Result<PrimitiveDraw, RenderError> {
        let colors = if self.varying_colors {
            [[1., 0., 0., 1.], [0., 1., 0., 1.]]
        } else {
            [[0., 1., 1., 1.]; 2]
        };
        let vertices = self
            .point_bits
            .into_iter()
            .zip(colors)
            .map(|(point, color)| RenderVertex::new(point.map(f32::from_bits), color))
            .collect();
        PrimitiveDraw::segments(
            vertices,
            vec![0, 1],
            self.clip_matrix(),
            LineStyle::new(2., 0xffff, 1)?,
        )
    }
    /// Compose the declared camera through the shared matrix API; `fixture.clip_matrix()`.
    pub fn clip_matrix(self) -> [f32; 16] {
        let mut projection = fixture_matrix(if self.perspective {
            PERSPECTIVE
        } else {
            ORTHOGRAPHIC
        });
        projection.multiply(&fixture_matrix(ORIENTATION));
        projection.data()
    }
}
fn fixture_matrix(rows: [[u32; 4]; 4]) -> Mat4<f32> {
    Mat4::from_data(std::array::from_fn(|index| {
        f32::from_bits(rows[index % 4][index / 4])
    }))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn corpus_preserves_four_boundaries_both_endpoint_orders_and_live_colors() {
        assert_eq!(VIEWPORT_LINES.len(), 18);
        for pair in VIEWPORT_LINES[..8].chunks_exact(2) {
            assert_eq!(
                pair[0].point_bits,
                [pair[1].point_bits[1], pair[1].point_bits[0]]
            );
            assert_eq!(
                pair[0].draw().unwrap().line_style().unwrap().parameters(),
                (2., 0xffff, 1)
            );
        }
        let varying = VIEWPORT_LINES
            .iter()
            .find(|case| case.varying_colors)
            .unwrap()
            .draw()
            .unwrap();
        assert_eq!(varying.vertices()[0].color, [1., 0., 0., 1.]);
        assert_eq!(varying.vertices()[1].color, [0., 1., 0., 1.]);
    }
    #[test]
    fn matrix_inputs_keep_xy_identity_or_perspective_w_without_vertex_rewrites() {
        for fixture in VIEWPORT_LINES {
            let draw = fixture.draw().unwrap();
            assert_eq!(draw.indices(), [0, 1]);
            for (vertex, bits) in draw.vertices().iter().zip(fixture.point_bits) {
                assert_eq!(vertex.position.map(f32::to_bits), bits);
            }
        }
        let matrix = VIEWPORT_LINES[0].clip_matrix();
        assert_eq!([matrix[0], matrix[5], matrix[15]], [1., 1., 1.]);
        let perspective = VIEWPORT_LINES
            .iter()
            .find(|case| case.perspective)
            .unwrap()
            .clip_matrix();
        assert_eq!(perspective[11], -1.);
    }
}
