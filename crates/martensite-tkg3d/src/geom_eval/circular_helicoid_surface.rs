//! `GeomEvalCircularHelicoidSurface`: the ruled helicoid
//! `S(u, v) = O + v cos(u) XDir + v sin(u) YDir + (P u / (2 Pi)) ZDir` on the whole
//! plane, P being the pitch.
use super::elementary_eval::{
    EvalElementarySurface, EvalFrame, PatchDerivatives, impl_eval_elementary_surface,
};
use martensite_tkernel::impl_transient;
use martensite_tkernel::precision;
use martensite_tkernel::standard::SharedState;
use martensite_tkernel::standard::dump::{self, DumpStream};
use martensite_tkernel::standard::{Failure, StandardTransient};
use martensite_tkmath::gp::{Axis3, Vector3, Xyz};
use std::f64::consts::PI;
use std::io;

const CLASS: &str = "GeomEval_CircularHelicoidSurface";

#[derive(Clone, Copy)]
struct HelicoidState {
    pos: Axis3,
    pitch: f64,
}

/// A circular helicoid about ZDir; `UReverse`, `VReverse`, the isos and `Transform`
/// raise StandardNotImplemented;
/// `GeomEvalCircularHelicoidSurface::new(Axis3::default(), 2.0)?`.
pub struct GeomEvalCircularHelicoidSurface {
    parent: StandardTransient,
    state: SharedState<HelicoidState>,
}
impl_transient!(
    GeomEvalCircularHelicoidSurface,
    "GeomEval_CircularHelicoidSurface",
    dyn crate::geom::GeomElementarySurface,
    parent,
    interfaces: [
        dyn crate::geom::GeomGeometry,
        dyn crate::geom::GeomSurface,
        dyn crate::geom::GeomElementarySurface
    ]
);
// Copy() reruns the constructor on the stored, already valid values.
impl Clone for GeomEvalCircularHelicoidSurface {
    fn clone(&self) -> Self {
        Self::stored(*self.state.read())
    }
}

// Only an exact zero is refused (`thePitch == 0.0`): a negative pitch gives a
// left-handed helicoid, and NaN passes the retained test.
fn nonzero_pitch(pitch: f64, member: &str) -> Result<f64, Failure> {
    if pitch == 0.0 {
        return Err(super::eval_failures::construction(&format!(
            "{member}: pitch must be != 0"
        )));
    }
    Ok(pitch)
}

impl GeomEvalCircularHelicoidSurface {
    fn stored(state: HelicoidState) -> Self {
        Self {
            parent: StandardTransient::default(),
            state: SharedState::new(state),
        }
    }
    /// Helicoid advancing `pitch` along ZDir per turn; StandardConstructionError when
    /// the pitch is 0; `GeomEvalCircularHelicoidSurface::new(frame, 2.0)?`.
    pub fn new(position: Axis3, pitch: f64) -> Result<Self, Failure> {
        let pitch: f64 = nonzero_pitch(pitch, CLASS)?;
        Ok(Self::stored(HelicoidState {
            pos: position,
            pitch,
        }))
    }
    /// The pitch (`Pitch`); `surface.pitch()`.
    pub fn pitch(&self) -> f64 {
        self.state.read().pitch
    }
    /// Set the pitch; StandardConstructionError when 0 (`SetPitch`);
    /// `surface.set_pitch(-1.0)?`.
    pub fn set_pitch(&self, pitch: f64) -> Result<(), Failure> {
        self.state.write().pitch =
            nonzero_pitch(pitch, "GeomEval_CircularHelicoidSurface::SetPitch")?;
        Ok(())
    }
}

/// The helicoid formulas at one `(U, V)`.
pub(crate) struct HelicoidPatch {
    frame: EvalFrame,
    pitch: f64,
    u: f64,
    v: f64,
    cos_u: f64,
    sin_u: f64,
}
impl PatchDerivatives for HelicoidPatch {
    fn point(&self) -> Xyz {
        let (f, v) = (&self.frame, self.v);
        let z_comp: f64 = self.pitch * self.u / (2.0 * PI);
        f.origin + f.x * (v * self.cos_u) + f.y * (v * self.sin_u) + f.z * z_comp
    }
    fn first(&self) -> [Xyz; 2] {
        let (f, v, cu, su) = (&self.frame, self.v, self.cos_u, self.sin_u);
        let z_rate: f64 = self.pitch / (2.0 * PI);
        [
            f.x * (v * (-su)) + f.y * (v * cu) + f.z * z_rate,
            f.x * cu + f.y * su,
        ]
    }
    // d2S/dv2 is zero: the surface is ruled along V.
    fn second(&self) -> [Xyz; 3] {
        let (f, v, cu, su) = (&self.frame, self.v, self.cos_u, self.sin_u);
        [
            f.x * (v * (-cu)) + f.y * (v * (-su)),
            Xyz::default(),
            f.x * (-su) + f.y * cu,
        ]
    }
    fn third(&self) -> [Xyz; 4] {
        let (f, v, cu, su) = (&self.frame, self.v, self.cos_u, self.sin_u);
        [
            f.x * (v * su) + f.y * (v * (-cu)),
            Xyz::default(),
            f.x * (-cu) + f.y * (-su),
            Xyz::default(),
        ]
    }
}

impl EvalElementarySurface for GeomEvalCircularHelicoidSurface {
    const CLASS: &'static str = CLASS;
    const BOUNDS: (f64, f64, f64, f64) = (
        -precision::infinite(),
        precision::infinite(),
        -precision::infinite(),
        precision::infinite(),
    );
    const U_CLOSED: bool = false;
    type Patch = HelicoidPatch;
    fn patch(&self, u: f64, v: f64) -> HelicoidPatch {
        let state: HelicoidState = *self.state.read();
        HelicoidPatch {
            frame: EvalFrame::new(state.pos),
            pitch: state.pitch,
            u,
            v,
            cos_u: martensite_cmath::cos(u),
            sin_u: martensite_cmath::sin(u),
        }
    }
    // Linear in V (zero for Nv >= 2) and linear in U on ZDir (only Nu == 1 keeps it).
    fn derivative_n(&self, u: f64, v: f64, nu: i32, nv: i32) -> Vector3 {
        if nv >= 2 {
            return Vector3::new(0.0, 0.0, 0.0);
        }
        let state: HelicoidState = *self.state.read();
        let frame: EvalFrame = EvalFrame::new(state.pos);
        let phase_u: f64 = f64::from(nu) * PI / 2.0;
        let (cos_n, sin_n) = (
            martensite_cmath::cos(u + phase_u),
            martensite_cmath::sin(u + phase_u),
        );
        if nv == 1 {
            return Vector3::from(frame.x * cos_n + frame.y * sin_n);
        }
        let mut result: Xyz = frame.x * (v * cos_n) + frame.y * (v * sin_n);
        if nu == 1 {
            result += frame.z * (state.pitch / (2.0 * PI));
        }
        Vector3::from(result)
    }
    fn dump_parameters(&self, stream: &mut DumpStream) -> io::Result<()> {
        dump::dump_real(stream, "Pitch", self.state.read().pitch)
    }
}
impl_eval_elementary_surface!(GeomEvalCircularHelicoidSurface);
