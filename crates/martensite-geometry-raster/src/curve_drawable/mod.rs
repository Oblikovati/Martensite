//! Thread-confined retained curve objects own their actual sampling/style/screen cells.
mod planar;
mod spatial;
use crate::{CurveLook, CurveSampling, CurveStyle, state::DrawableState};
pub use planar::PlanarCurveDrawable;
pub use spatial::SpatialCurveDrawable;
struct CurveControls {
    sampling: CurveSampling,
    style: CurveStyle,
    state: DrawableState,
}
impl CurveControls {
    fn new(look: CurveLook, (discret, deflection, mode): (i32, f64, i32)) -> Self {
        Self {
            sampling: CurveSampling::new(discret, deflection, mode),
            style: CurveStyle::new(look),
            state: DrawableState::default(),
        }
    }
}
