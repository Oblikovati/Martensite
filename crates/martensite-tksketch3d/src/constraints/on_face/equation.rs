// SPDX-License-Identifier: GPL-2.0-only
//! A local affine point attachment evaluated in the original chart coordinates.
use martensite_tkbrep::brep::planar_attachment::PlanarAttachmentChart;
use martensite_tkmath::gp::Vector3;
use martensite_tksolve::{ad::DualNumber, residual::ResidualError};

#[derive(Clone, Copy)]
pub(in crate::constraints) struct PlanarEquation {
    chart: PlanarAttachmentChart,
    inverse_length: f64,
    pub steps: [f64; 2],
}

impl PlanarEquation {
    pub(super) fn new(chart: PlanarAttachmentChart, length_scale: f64) -> Option<Self> {
        let steps = chart
            .tangents
            .map(|[x, y, z]| length_scale / Vector3::new(x, y, z).magnitude());
        steps
            .into_iter()
            .all(|step| step.is_finite() && step > 0.)
            .then_some(Self {
                chart,
                inverse_length: 1. / length_scale,
                steps,
            })
    }

    pub(in crate::constraints) fn evaluate(
        self,
        values: &[DualNumber],
    ) -> Result<Vec<DualNumber>, ResidualError> {
        let [x, y, z, u, v] = values else {
            return Err(ResidualError::Domain(
                "planar attachment requires five coordinates".into(),
            ));
        };
        let du = u.shifted(-self.chart.uv[0])?;
        let dv = v.shifted(-self.chart.uv[1])?;
        let mut rows = Vec::with_capacity(3);
        for (axis, point) in [x, y, z].into_iter().enumerate() {
            let chart_delta = du
                .scaled(self.chart.tangents[0][axis])?
                .plus(&dv.scaled(self.chart.tangents[1][axis])?)?;
            rows.push(
                point
                    .shifted(-self.chart.world[axis])?
                    .minus(&chart_delta)?
                    .scaled(self.inverse_length)?,
            );
        }
        Ok(rows)
    }
}
