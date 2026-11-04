//! Alpha, cutoff and blending of one programmable mesh-edge fragment over a section cap.
//!
//! Defined behaviour (crates/martensite-vulkan/shaders/surface.frag, shading material setup): the
//! fragment source is `mix(edge, face, t)` in linear RGBA, or `(face.rgb, 1 - t)` for a negative
//! (hollow) edge alpha. An edge alpha below one sets the alpha-test cutoff 0.285; otherwise Mask
//! and MaskBlend use the aspect cutoff. A fragment below the cutoff is discarded and the frame
//! keeps what lies behind the cap. Blend, MaskBlend and a transparent BlendAuto material blend
//! `src.rgb * a + dst * (1 - a)` in linear colour; Blend and transparent BlendAuto draw in the
//! transparent pass without depth writes.
use martensite_tkernel::quantity::Color;

/// Fragment alpha within this distance of the cutoff has no single defined outcome.
const CUTOFF_MARGIN: f64 = 1e-4;

/// The cap aspect's alpha handling, read from the live fixture.
#[derive(Clone, Debug)]
pub struct CapAlphaPolicy {
    pub cutoff: Option<f64>,
    pub blend: bool,
    pub depth_write: bool,
    /// Texel colour bytes and their alpha; an empty table means every texel is opaque.
    pub texel_alpha: Vec<([u8; 3], f64)>,
}

impl CapAlphaPolicy {
    /// An opaque cap without alpha test; `CapAlphaPolicy::opaque()`.
    pub fn opaque() -> Self {
        Self {
            cutoff: None,
            blend: false,
            depth_write: true,
            texel_alpha: Vec::new(),
        }
    }

    /// The alpha of the texel drawn with `rgb`; `policy.source_alpha([255, 0, 0])`.
    pub fn source_alpha(&self, rgb: [u8; 3]) -> f64 {
        if self.texel_alpha.is_empty() {
            return 1.;
        }
        let texel = self.texel_alpha.iter().find(|(color, _)| *color == rgb);
        texel
            .unwrap_or_else(|| {
                panic!(
                    "source colour {rgb:?}; expected one of {:?}",
                    self.texel_alpha
                )
            })
            .1
    }
}

/// The edge style of the composite: linear colour, alpha (negative is hollow) and the ramp.
#[derive(Clone, Copy, Debug)]
pub struct EdgeStyle {
    pub color: [f64; 3],
    pub alpha: f64,
    pub width: f64,
    pub feather: f64,
}

/// What one pixel may show: behind the cap, a colour range, or either near the cutoff.
#[derive(Clone, Debug, PartialEq)]
pub enum Outcome {
    Discarded,
    Drawn([(f64, f64); 3]),
    Either([(f64, f64); 3]),
}

/// One pixel's control bytes: the opaque cap source and what lies behind the cap.
pub struct PixelControls<'a> {
    pub source: &'a [u8],
    pub behind: &'a [u8],
}

/// Expected outcome of one cap pixel at edge distance `distance`; `outcome(&p, &e, &c, 1.5)`.
pub fn outcome(
    policy: &CapAlphaPolicy,
    edge: &EdgeStyle,
    controls: &PixelControls,
    distance: f64,
) -> Outcome {
    let half = edge.feather * 0.5;
    let t = smoothstep(edge.width - half, edge.width + half, distance);
    let rgb = [0, 1, 2].map(|k| controls.source[k]);
    let face_alpha = policy.source_alpha(rgb);
    let alpha = fragment_alpha(edge.alpha, face_alpha, t);
    let range = [0, 1, 2].map(|k| channel_range(policy, edge, controls, (k, t, alpha)));
    let Some(cutoff) = policy.cutoff else {
        return Outcome::Drawn(range);
    };
    if (alpha - cutoff).abs() <= CUTOFF_MARGIN {
        return Outcome::Either(range);
    }
    if alpha < cutoff {
        Outcome::Discarded
    } else {
        Outcome::Drawn(range)
    }
}

fn fragment_alpha(edge_alpha: f64, face_alpha: f64, t: f64) -> f64 {
    if edge_alpha < 0. {
        return 1. - t;
    }
    edge_alpha + (face_alpha - edge_alpha) * t
}

/// Encoded bounds of one channel over the half-level quantization of both control bytes.
fn channel_range(
    policy: &CapAlphaPolicy,
    edge: &EdgeStyle,
    controls: &PixelControls,
    (k, t, alpha): (usize, f64, f64),
) -> (f64, f64) {
    // The composite is non-decreasing in both control values, so the corners bound it.
    let at = |offset: f64| {
        let source = decode(f64::from(controls.source[k]) + offset);
        let behind = decode(f64::from(controls.behind[k]) + offset);
        let color = if edge.alpha < 0. {
            source
        } else {
            edge.color[k] + (source - edge.color[k]) * t
        };
        let weight = alpha.clamp(0., 1.);
        let linear = if policy.blend {
            color * weight + behind * (1. - weight)
        } else {
            color
        };
        Color::linear_to_srgb(linear.clamp(0., 1.)) * 255.
    };
    (at(-0.5), at(0.5))
}

fn decode(byte: f64) -> f64 {
    Color::srgb_to_linear((byte / 255.).clamp(0., 1.))
}
fn smoothstep(low: f64, high: f64, value: f64) -> f64 {
    let t = ((value - low) / (high - low)).clamp(0., 1.);
    t * t * (3. - 2. * t)
}
