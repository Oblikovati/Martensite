//! The inner integral of the adaptive `Compute`: along U at the V of one boundary
//! point, from the U lower bound to the boundary's U, bisecting the U intervals.
use super::face::BRepGPropFace;
use super::gauss::{GaussType, Inertia, add_volume_part};
use super::gauss_adaptive::{AdaptiveRun, SUBS_POWER, max_element, max_subs, sm};
use martensite_tkernel::standard::Failure;

impl AdaptiveRun<'_, '_> {
    /// Both outer Gauss orders use the same inner convergence rule in strict mode (#407),
    /// while legacy entry points preserve exact OCCT adaptive arithmetic.
    pub(super) fn u_integral(
        &mut self,
        surface: &BRepGPropFace,
        pass: usize,
    ) -> Result<usize, Failure> {
        let (u1, u2) = (self.input.bounds.0, self.state.u2);
        let end = self.u.fill(u1, u2, &self.input.u_knots, None);
        let max = max_subs(end, SUBS_POWER).min(sm());
        self.u.reset();
        self.state.error_u = 0.0;
        let (mut k_end, mut j) = (1, 0);
        loop {
            j += 1;
            let range = self.u.next_range(j, end);
            let exhausted = if self.strict_refinement {
                self.u.is_exhausted(j, max)
            } else {
                self.u.is_exhausted_legacy(j, max)
            };
            if exhausted {
                if self.strict_refinement {
                    if j == max || k_end != 1 {
                        return Err(self.refinement_failure("U", j, max, self.state.error_u));
                    }
                } else {
                    self.state.refinement_exhausted |=
                        k_end != 1 || self.u.first[j - 1] != self.u.second[j - 1];
                    if k_end != 1 {
                        j -= 1;
                        self.state.eps_u = self.state.error_u;
                        let span = ((u2 - u1) * self.state.dul).abs();
                        self.state.eps = 10. * self.state.eps_u * span;
                        self.state.eps_l = 0.9 * self.state.eps;
                        break;
                    }
                }
                self.u.error[j - 1] = 0.0;
                self.u.inertia[j - 1] = Inertia::default();
            } else {
                for index in range.into_iter().take(k_end) {
                    self.u_span(surface, pass, index)?;
                }
            }
            if j == end {
                k_end = 2;
                if !self.strict_refinement {
                    self.state.error_u = self.u.error[max_element(&self.u.error)];
                }
            }
            if self.strict_refinement {
                let mass: f64 = self.u.inertia[..j].iter().map(|part| part.mass).sum();
                // #407: reserve ten percent of the relative target for the inner integral.
                self.state.eps_u = 0.1 * self.input.epsilon * mass.abs();
                self.state.error_u = self.u.error[..j].iter().sum();
                let refine = self.is_verify && self.state.error_u > self.state.eps_u;
                if !(refine || k_end == 1) {
                    break;
                }
            } else {
                let refine = self.state.error_u - self.state.eps_u > 0.0 && self.state.eps_u != 0.0;
                if !(refine || k_end == 1) {
                    break;
                }
            }
        }
        Ok(j)
    }

    /// Evaluate U orders on one interval.
    fn u_span(
        &mut self,
        surface: &BRepGPropFace,
        pass: usize,
        index: usize,
    ) -> Result<(), Failure> {
        let mut local = [Inertia::default(); 2];
        let um = 0.5 * (self.u.second[index] + self.u.first[index]);
        let ur = 0.5 * (self.u.second[index] - self.u.first[index]);
        let orders = if self.strict_refinement {
            self.input.error_passes
        } else {
            self.input.error_passes - pass
        };
        for (order, sums) in local.iter_mut().enumerate().take(orders) {
            let (points, weights) = &self.input.u_rules.rules[order];
            for (point, weight) in points.iter().zip(weights) {
                let (p, normal) = surface.normal(um + ur * point, self.state.v)?;
                self.add_u_point(order, (p, normal), *weight, sums);
            }
        }
        self.store_u_span(pass, index, ur, &local);
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
    fn store_u_span(&mut self, pass: usize, index: usize, ur: f64, local: &[Inertia; 2]) {
        let g = self.gauss;
        let ui = &mut self.u.inertia[index];
        ui.mass = g.mult(local[0].mass, ur);
        if g.kind == GaussType::Vinert {
            ui.ixx = g.mult(local[0].ixx, ur);
            ui.iyy = g.mult(local[0].iyy, ur);
            ui.izz = g.mult(local[0].izz, ur);
        }
        if !self.strict_refinement && pass > 0 {
            return;
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
        self.u.error[index] = if self.strict_refinement {
            g.mult(d_mass, ur.abs())
        } else {
            g.mult(d_mass, ur)
        };
    }
}
