// SPDX-License-Identifier: GPL-2.0-only
//! Literal single-cell nonrational ruled patches with exact iso rectangular trims.
use super::{
    affine_curve::clamped,
    interval::{Point, Span, determinant, placed},
};
use martensite_tkg3d::geom::GeomBSplineSurface;
use martensite_tkmath::{
    gp::Transform3,
    predicates::{
        bilinear_patch::{BilinearExclusion, BilinearPatchInput, certify_bilinear_patch},
        compare_scalar_triple_product_3d,
    },
};
use std::cmp::Ordering;

#[derive(Clone, Copy)]
pub(super) struct Bilinear {
    uv: [[f64; 2]; 4],
    pub(super) poles: [Point; 4],
}

impl Bilinear {
    pub(super) fn decode(
        patch: &GeomBSplineSurface,
        location: Transform3,
        reference: [f64; 3],
    ) -> Option<Self> {
        let lease = patch.try_input_lease().ok()?;
        let stored = lease.stored_input().ok()?;
        if stored.shape() != [2, 2]
            || stored.rational() != [false; 2]
            || stored.evaluation_override()
            || stored.weights() != [1.; 4]
            || stored.weights_shape() != [2, 2]
        {
            return None;
        }
        for axis in stored.axes() {
            if axis.degree() != 1
                || axis.periodic()
                || !clamped(
                    axis.knots(),
                    axis.multiplicities(),
                    axis.flat_knots(),
                    axis.domain(),
                )
            {
                return None;
            }
        }
        let [u, v] = stored.axes().each_ref().map(|axis| axis.domain());
        let uv = [[u[0], v[0]], [u[1], v[0]], [u[1], v[1]], [u[0], v[1]]];
        let corners = [0, 2, 3, 1].map(|i| stored.poles()[i]);
        let input = BilinearPatchInput::new(corners, [u, v], [1.; 4]).ok()?;
        if !matches!(
            certify_bilinear_patch(input),
            BilinearExclusion::Certified(_)
        ) || !injective_location(location)
        {
            return None;
        }
        let poles = corners.map(|corner| placed(corner.map(Span::at), location, reference));
        poles
            .iter()
            .flatten()
            .all(|coordinate| coordinate.finite())
            .then_some(Self { uv, poles })
    }

    pub(super) fn corner(self, uv: [Span; 2]) -> Option<usize> {
        self.uv.iter().position(|point| point.map(Span::at) == uv)
    }
}

fn injective_location(location: Transform3) -> bool {
    if location.is_identity() {
        return true;
    }
    let m = location.vectorial_part();
    let columns = [1, 2, 3].map(|axis| m.column(axis).map(|vector| *vector.coordinates()));
    let [Ok(a), Ok(b), Ok(c)] = columns else {
        return false;
    };
    matches!(
        compare_scalar_triple_product_3d(a, b, c),
        Ok(Ordering::Greater | Ordering::Less)
    )
}

pub(super) fn oriented_flux(poles: [Point; 4], forward: bool) -> Span {
    let [a, bu, buv, bv] = poles;
    let delta = |from: Point, to: Point| std::array::from_fn(|i| to[i] - from[i]);
    let (b, c) = (delta(a, bu), delta(a, bv));
    let d = delta(delta(a, bu), delta(bv, buv));
    // For S = a+ub+vc+uvd, the flux density is the determinant of
    // (S, dS/du, dS/dv). Terms integrate over the entire proven unit rectangle.
    let flux = Span::at(2.) * determinant(a, b, c) + determinant(a, b, d) + determinant(a, d, c)
        - Span::at(0.5) * determinant(b, c, d);
    if forward { flux } else { -flux }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn planar_literal_box_side_has_the_expected_oriented_flux() {
        let poles =
            [[0., 0., 2.], [3., 0., 2.], [3., 1., 2.], [0., 1., 2.]].map(|p| p.map(Span::at));
        let oriented = oriented_flux(poles, true);
        assert!(oriented.lo <= 12. && oriented.hi >= 12.);
        let reversed = oriented_flux(poles, false);
        assert!(reversed.lo <= -12. && reversed.hi >= -12.);
    }
}
