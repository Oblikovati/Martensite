//! Admit native ruled-side controls before interval evaluation.
use super::*;

impl SideSource<'_> {
    pub(super) fn check(&self, issuer: &Issuer) -> Result<(), Refusal> {
        let l = self.affine;
        if l[0] != 1.
            || l[1..3] != [0., 0.]
            || [l[6], l[9], l[10], l[11], l[12]] != [0., 0., 0., 0., 1.]
            || l.iter().any(|&v| !finite_range(v, -1024., 1024.))
        {
            return Err(Refusal::Location);
        }
        if self.normalized_domain != [0., 1., 0., 1.]
            || !(2..=MAX).contains(&self.first.len())
            || self.last.len() != self.first.len()
        {
            return Err(Refusal::Carrier);
        }
        for (a, b) in self.first.iter().zip(self.last) {
            if a[3].to_bits() != b[3].to_bits()
                || !finite_range(a[3], 0.001, 1024.)
                || a[..3]
                    .iter()
                    .chain(&b[..3])
                    .any(|&v| !finite_range(v, -1024., 1024.))
            {
                return Err(Refusal::Carrier);
            }
        }
        if !(2..=128).contains(&self.seam_knots.len())
            || self.seam_knots.first() != Some(&0.)
            || self.seam_knots.last() != Some(&1.)
            || self
                .seam_knots
                .windows(2)
                .any(|w| w[0].partial_cmp(&w[1]) != Some(std::cmp::Ordering::Less))
            || self.seam_uv.len() != 3 * (self.seam_knots.len() - 1) + 1
            || self
                .seam_uv
                .iter()
                .any(|[u, v]| !finite_range(*u, 0., 6.) || !finite_range(*v, 0., 30.))
        {
            return Err(Refusal::Pcurve);
        }
        if !finite_range(self.seam_tolerance, 0., issuer.edge_max)
            || !finite_range(self.side_tolerance, 0., issuer.face_max)
        {
            return Err(Refusal::Tolerance);
        }
        Ok(())
    }
}
