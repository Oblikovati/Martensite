// SPDX-License-Identifier: GPL-2.0-only
//! Exact finite-trim incidence before any source cut coordinate is rounded.
use super::{
    CertifiedPlanarAttachment, ExactRatio as Ratio, PlanarAttachmentError as Error,
    PlanarAttachmentLocation as Location,
    chart::Uv,
    poll,
    trim::{self, Loop, Side},
};
use crate::predicates::roots::metric_bounds::{
    placed_plane::{ExactVector, exact_vector},
    segment_plane::{SegmentPlaneCut, SegmentPlaneFailure, enclose_cut},
};
use std::cmp::Ordering;

/// Contact of a directed segment with the exact closed planar trim.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PlanarSegmentContact {
    /// The segment has no support or coplanar trim contact.
    Disjoint,
    /// A unique support hit was outside the outer trim or inside the hole.
    TrimmedOut(Location),
    /// The directed start endpoint is on retained material.
    Start(Location),
    /// The directed end endpoint is on retained material.
    End(Location),
    /// A coplanar segment meets the retained face only at isolated boundary points.
    CoplanarTouch,
    /// A coplanar segment shares a positive-length interval with retained material.
    CoplanarOverlap,
    /// A unique interior crossing of the support lies within the closed trim.
    Crossing {
        /// Outward source parameter and world point evidence.
        cut: SegmentPlaneCut,
        /// Exact material, outer rim, or hole rim incidence.
        location: Location,
    },
}

impl CertifiedPlanarAttachment {
    /// Classify one directed segment on the exact located trim.
    /// Example: `proof.classify_segment([[-1., 0., 0.], [2., 1., 0.]], || false)?`.
    pub fn classify_segment(
        &self,
        endpoints: [[f64; 3]; 2],
        mut cancelled: impl FnMut() -> bool,
    ) -> Result<PlanarSegmentContact, Error> {
        use PlanarSegmentContact as Contact;
        poll(&mut cancelled)?;
        let start = exact_vector(endpoints[0], 0)?;
        let end = exact_vector(endpoints[1], 1)?;
        if start == end {
            return Err(Error::Segment(SegmentPlaneFailure::CoincidentEndpoints));
        }
        let [a, b] = self.world.signed_support([&start, &end]);
        let contact = match (a.sign(), b.sign()) {
            (Ordering::Equal, Ordering::Equal) => self.coplanar([&start, &end], &mut cancelled)?,
            (Ordering::Equal, _) => self.endpoint(&start, Contact::Start, &mut cancelled)?,
            (_, Ordering::Equal) => self.endpoint(&end, Contact::End, &mut cancelled)?,
            (left, right) if left == right => Contact::Disjoint,
            _ => self.crossing([&start, &end], a.divide(&a.subtract(&b)), &mut cancelled)?,
        };
        poll(&mut cancelled)?;
        Ok(contact)
    }

    fn crossing(
        &self,
        source: [&ExactVector; 2],
        parameter: Ratio,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<PlanarSegmentContact, Error> {
        let point = interpolate(source, &parameter);
        let location = self.locate(&self.world.pullback(&point), cancelled)?;
        if trimmed_out(location) {
            return Ok(PlanarSegmentContact::TrimmedOut(location));
        }
        let cut = enclose_cut(&point, &parameter).map_err(Error::Segment)?;
        Ok(PlanarSegmentContact::Crossing { cut, location })
    }

    fn endpoint(
        &self,
        point: &ExactVector,
        contact: impl FnOnce(Location) -> PlanarSegmentContact,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<PlanarSegmentContact, Error> {
        let location = self.locate(&self.world.pullback(point), cancelled)?;
        Ok(if trimmed_out(location) {
            PlanarSegmentContact::TrimmedOut(location)
        } else {
            contact(location)
        })
    }

    fn locate(&self, point: &Uv, cancelled: &mut impl FnMut() -> bool) -> Result<Location, Error> {
        trim::locate(&self.outer, self.hole.as_ref(), point, cancelled)
    }

    fn coplanar(
        &self,
        source: [&ExactVector; 2],
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<PlanarSegmentContact, Error> {
        use PlanarSegmentContact as Contact;
        let uv = source.map(|point| self.world.pullback(point));
        let source = uv.each_ref();
        let Some([low, high]) = self.outer.clip(source, cancelled)? else {
            return Ok(Contact::Disjoint);
        };
        if low < high && !inside_hole(self.hole.as_ref(), source, [&low, &high], cancelled)? {
            return Ok(Contact::CoplanarOverlap);
        }
        for t in [&low, &high] {
            if !trimmed_out(self.locate(&interpolate(source, t), cancelled)?) {
                return Ok(Contact::CoplanarTouch);
            }
        }
        Ok(Contact::Disjoint)
    }
}

fn inside_hole(
    hole: Option<&Loop>,
    source: [&Uv; 2],
    [low, high]: [&Ratio; 2],
    cancelled: &mut impl FnMut() -> bool,
) -> Result<bool, Error> {
    let Some(hole) = hole else { return Ok(false) };
    let Some([a, b]) = hole.clip(source, cancelled)? else {
        return Ok(false);
    };
    if &a > low || &b < high {
        return Ok(false);
    }
    let middle = low.add(high).multiply(&Ratio::from_finite(0.5));
    Ok(hole.position(&interpolate(source, &middle), cancelled)? == Side::Inside)
}

fn interpolate<const N: usize>([a, b]: [&[Ratio; N]; 2], t: &Ratio) -> [Ratio; N] {
    std::array::from_fn(|i| a[i].add(&b[i].subtract(&a[i]).multiply(t)))
}

fn trimmed_out(location: Location) -> bool {
    matches!(location, Location::OutsideOuter | Location::HoleInterior)
}
