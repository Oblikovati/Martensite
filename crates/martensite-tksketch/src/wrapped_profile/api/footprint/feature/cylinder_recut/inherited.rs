//! Certify a completed cylinder carried unchanged into the assembled shell.
use super::*;
use frozen::FrozenRecut;
use martensite_tkmath::predicates::roots::metric_bounds::{
    FrameTransportInput, certify_frame_transport_at_angle,
};

pub(in super::super) struct UnchangedStock {
    seam: Edge,
    parts: [Shape; 7],
    receipt: Vec<u8>,
    spatial_upper: f64,
}

pub(in super::super) fn certify_unchanged(
    attempt: Attempt,
    solid: &Shape,
    side: &Face,
    sew: f64,
) -> FeatureChecked<UnchangedStock> {
    let check = RecutCheck(attempt);
    if !(sew.is_finite() && sew > 0.) {
        return Err(check.refusal("finite positive sewing resolution"));
    }
    let stock = source::Stock::capture_inherited(check, solid, side)?;
    let phase = period_error(check, stock.radius)?;
    let [a, b] = &stock.ends;
    let seam_upper = (phase + stock.seam_error).next_up();
    let spatial_upper = (seam_upper + a.anchor_error.max(b.anchor_error)).next_up();
    if !spatial_upper.is_finite() || spatial_upper >= stock.tolerance.min(sew) {
        return Err(
            check.refusal("inherited seam and rim residuals below the stock and sewing limits")
        );
    }
    let parts = witnesses(&stock);
    let receipt = frozen::stock_receipt(attempt, side, &stock.seam, &parts)?;
    Ok(UnchangedStock {
        seam: stock.seam,
        parts,
        receipt,
        spatial_upper,
    })
}

impl UnchangedStock {
    pub(in super::super) fn spatial_upper(&self) -> f64 {
        self.spatial_upper
    }

    pub(in super::super) fn bind_final(
        &self,
        attempt: Attempt,
        index: &body::Index,
        band: &Face,
    ) -> FeatureChecked<FrozenRecut<'_>> {
        frozen::bind_stock(attempt, index, band, &self.seam, &self.parts, &self.receipt)
    }
}

fn period_error(attempt: RecutCheck, radius: f64) -> FeatureChecked<f64> {
    let identity = FrameTransportInput {
        axis: UP,
        radial: X,
        stored: [[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]],
    };
    let period = certify_frame_transport_at_angle(identity, TAU)
        .map_err(|_| attempt.refusal("certified full-turn seam closure at the stored angle"))?;
    // The same bound applies to a rim's closed anchor and to either seam occurrence.
    let spatial_upper = (radius * period.matrix_error_bound()).next_up();
    if !spatial_upper.is_finite() {
        return Err(attempt.refusal("finite inherited seam positional residual"));
    }
    Ok(spatial_upper)
}

fn witnesses(stock: &source::Stock<'_>) -> [Shape; 7] {
    let [first, last] = &stock.ends;
    let members = |end: &source::Rim| {
        [
            end.face.to_shape(),
            end.edge.to_shape(),
            end.anchor.to_shape(),
        ]
    };
    let [a, b, c] = members(first);
    let [d, e, f] = members(last);
    [a, b, c, d, e, f, stock.seam.to_shape()]
}
