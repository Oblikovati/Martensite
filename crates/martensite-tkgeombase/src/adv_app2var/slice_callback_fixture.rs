//! Independent initialized fixtures for private callback guard regressions.
use super::approx_f2var::{IsoApproxOutput, IsoApproxRequest};
use super::*;
pub(super) struct SliceCallbackFixture {
    values: [[f64; 64]; 8],
    counts: [i32; 3],
}
impl SliceCallbackFixture {
    pub(super) fn new() -> Self {
        Self {
            values: [[7.; 64]; 8],
            counts: [-7; 3],
        }
    }
    pub(super) fn output(&mut self) -> IsoApproxOutput<'_> {
        let [curve, sums, differences, left, right, cuts, maximum, mean] = &mut self.values;
        IsoApproxOutput {
            ncoeff: &mut self.counts,
            courbe: curve,
            nbcrbe: -7,
            somtab: sums,
            diftab: differences,
            contr1: left,
            contr2: right,
            tabdec: cuts,
            errmax: maximum,
            errmoy: mean,
        }
    }
}
pub(super) fn request() -> IsoApproxRequest<'static> {
    IsoApproxRequest {
        ndimen: 1,
        nbsesp: 1,
        ndimse: &[1],
        uvfonc: [0., 1., 0., 1.],
        tconst: 0.5,
        isofav: 1,
        nbroot: 3,
        rootlg: &[-0.7745966692414834, 0., 0.7745966692414834],
        iordre: -1,
        ideriv: 0,
        ndgjac: 2,
        nbcrmx: 1,
        ncflim: 3,
        epsapr: &[1e-6],
    }
}
pub(super) fn integers() -> integer_frame::IntegerFrame {
    integer_frame::IntegerFrame::new(
        vec![1, 1, 0, 0, 0],
        AdvApp2VarIntegerBindings {
            dimension: 0,
            favorite_iso: 1,
            parameter_count: 2,
            u_order: 3,
            v_order: 4,
        },
    )
    .unwrap()
}
pub(super) fn reals() -> AdvApp2VarIsoRealFrame {
    AdvApp2VarIsoRealFrame::new(
        vec![0., 1., 0., 1., 0.5],
        AdvApp2VarRealBindings {
            domain: [0, 1, 2, 3],
            constant_parameter: 4,
        },
    )
    .unwrap()
}
