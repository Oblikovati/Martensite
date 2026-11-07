//! The inner integral of the adaptive `Compute`: along U at the V of one boundary
//! point, from the U lower bound to the boundary's U, bisecting the U intervals.
use super::face::BRepGPropFace;
use super::gauss::{GaussType, Inertia, add_volume_part};
use super::gauss_adaptive::{AdaptiveRun, SUBS_POWER, max_subs, sm};
use martensite_tkernel::standard::Failure;

impl AdaptiveRun<'_, '_> {
    /// Both outer Gauss orders use the same inner convergence rule. Update the total
    /// U error after each subdivision; a stale estimate cannot govern refinement (#407).
    pub(super) fn u_integral(&mut self, surface: &BRepGPropFace) -> Result<usize, Failure> {
        let (u1, u2) = (self.input.bounds.0, self.state.u2);
        let end = self.u.fill(u1, u2, &self.input.u_knots, None);
        let max = max_subs(end, SUBS_POWER).min(sm());
        self.u.reset();
        self.state.error_u = 0.0;
        let (mut k_end, mut j) = (1, 0);
        loop {
            j += 1;
            let range = self.u.next_range(j, end);
            if self.u.is_exhausted(j, max) {
                if j == max || k_end != 1 {
                    return Err(self.refinement_failure("U", j, max, self.state.error_u));
                }
                self.u.error[j - 1] = 0.0;
                self.u.inertia[j - 1] = Inertia::default();
            } else {
                for index in range.into_iter().take(k_end) {
                    self.u_span(surface, index)?;
                }
            }
            if j == end {
                k_end = 2;
            }
            let mass: f64 = self.u.inertia[..j].iter().map(|part| part.mass).sum();
            // #407: reserve ten percent of the relative target for the inner integral.
            self.state.eps_u = 0.1 * self.input.epsilon * mass.abs();
            self.state.error_u = self.u.error[..j].iter().sum();
            let refine = self.is_verify && self.state.error_u > self.state.eps_u;
            if !(refine || k_end == 1) {
                break;
            }
        }
        Ok(j)
    }

    /// Evaluate both U orders on one interval. The second order estimates area error;
    /// the first supplies all moments, independently of the outer order (#407).
    fn u_span(&mut self, surface: &BRepGPropFace, index: usize) -> Result<(), Failure> {
        let mut local = [Inertia::default(); 2];
        let um = 0.5 * (self.u.second[index] + self.u.first[index]);
        let ur = 0.5 * (self.u.second[index] - self.u.first[index]);
        let orders = self.input.error_passes;
        for (order, sums) in local.iter_mut().enumerate().take(orders) {
            let (points, weights) = &self.input.u_rules.rules[order];
            for (point, weight) in points.iter().zip(weights) {
                let (p, normal) = surface.normal(um + ur * point, self.state.v)?;
                self.add_u_point(order, (p, normal), *weight, sums);
            }
        }
        self.store_u_span(index, ur, &local);
        Ok(())
    }

    /// One Gauss point of a U interval into the sums of Gauss order `order`.
    fn add_u_point(
        &self,
        order: usize,
        (point, normal): (
            martensite_tkmath::gp::Point3,
            martensite_tkmath::gp::Vector3,
        ),
        weight: f64,
        local: &mut Inertia,
    ) {
        let input = &self.input;
        if self.gauss.kind == GaussType::Vinert {
            let plane = (input.coeff, input.by_point);
            add_volume_part(point, normal, input.location, weight, plane, local);
        } else if order > 0 {
            local.mass += weight * normal.magnitude();
        } else {
            self.gauss
                .add_surface_part(point, normal, input.location, weight, local);
        }
    }

    /// The moments of a U interval (times its half range) and its error.
    fn store_u_span(&mut self, index: usize, ur: f64, local: &[Inertia; 2]) {
        let g = self.gauss;
        let ui = &mut self.u.inertia[index];
        ui.mass = g.mult(local[0].mass, ur);
        if g.kind == GaussType::Vinert {
            ui.ixx = g.mult(local[0].ixx, ur);
            ui.iyy = g.mult(local[0].iyy, ur);
            ui.izz = g.mult(local[0].izz, ur);
        }
        let d_mass = (local[1].mass - local[0].mass).abs();
        ui.ix = g.mult(local[0].ix, ur);
        ui.iy = g.mult(local[0].iy, ur);
        ui.iz = g.mult(local[0].iz, ur);
        if g.kind == GaussType::Sinert {
            ui.ixx = g.mult(local[0].ixx, ur);
            ui.iyy = g.mult(local[0].iyy, ur);
            ui.izz = g.mult(local[0].izz, ur);
        }
        ui.ixy = g.mult(local[0].ixy, ur);
        ui.ixz = g.mult(local[0].ixz, ur);
        ui.iyz = g.mult(local[0].iyz, ur);
        self.u.error[index] = g.mult(d_mass, ur.abs());
    }
}
