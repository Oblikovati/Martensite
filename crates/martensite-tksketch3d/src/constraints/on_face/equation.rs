// SPDX-License-Identifier: GPL-2.0-only
//! A local affine point attachment evaluated in the original chart coordinates.
use martensite_tkbrep::brep::{
    cylindrical_split::CylinderPointChart, planar_attachment::PlanarAttachmentChart,
};
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
        let steps = steps(chart.tangents, length_scale)?;
        Some(Self {
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

#[derive(Clone, Copy)]
pub(in crate::constraints) enum AttachmentEquation {
    Plane(PlanarEquation),
    Cylinder(CylindricalEquation),
}

impl AttachmentEquation {
    pub(in crate::constraints) fn steps(self) -> [f64; 2] {
        match self {
            Self::Plane(source) => source.steps,
            Self::Cylinder(source) => source.steps,
        }
    }

    pub(in crate::constraints) fn evaluate(
        self,
        values: &[DualNumber],
    ) -> Result<Vec<DualNumber>, ResidualError> {
        match self {
            Self::Plane(source) => source.evaluate(values),
            Self::Cylinder(source) => source.evaluate(values),
        }
    }
}

#[derive(Clone, Copy)]
pub(in crate::constraints) struct CylindricalEquation {
    chart: CylinderPointChart,
    inverse_length: f64,
    steps: [f64; 2],
}

impl CylindricalEquation {
    pub(super) fn new(chart: CylinderPointChart, length_scale: f64) -> Option<Self> {
        let steps = steps(chart.tangents, length_scale)?;
        Some(Self {
            chart,
            inverse_length: 1. / length_scale,
            steps,
        })
    }

    fn evaluate(self, values: &[DualNumber]) -> Result<Vec<DualNumber>, ResidualError> {
        let [x, y, z, u, v] = values else {
            return Err(ResidualError::Domain(
                "cylinder attachment requires five coordinates".into(),
            ));
        };
        let delta = u.shifted(-self.chart.uv[0])?;
        // Use the anchored sine identity for cos(delta)-1 to preserve small radial motion.
        let half_sine = delta.scaled(0.5)?.sin()?;
        let radial = half_sine.product(&half_sine)?.scaled(-2.)?;
        let tangential = delta.sin()?;
        let axial = v.shifted(-self.chart.uv[1])?;
        [x, y, z]
            .into_iter()
            .enumerate()
            .map(|(axis, point)| {
                let motion = radial
                    .scaled(self.chart.radial[axis])?
                    .plus(&tangential.scaled(self.chart.tangents[0][axis])?)?
                    .plus(&axial.scaled(self.chart.tangents[1][axis])?)?;
                Ok(point
                    .shifted(-self.chart.world[axis])?
                    .minus(&motion)?
                    .scaled(self.inverse_length)?)
            })
            .collect()
    }
}

fn steps(tangents: [[f64; 3]; 2], length_scale: f64) -> Option<[f64; 2]> {
    let steps = tangents.map(|[x, y, z]| length_scale / Vector3::new(x, y, z).magnitude());
    steps
        .into_iter()
        .all(|step| step.is_finite() && step > 0.)
        .then_some(steps)
}
