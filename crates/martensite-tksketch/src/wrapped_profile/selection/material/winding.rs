//! Exact source-curve winding spends one subdivision allowance across every loop and image.
use super::*;
use martensite_tkmath::predicates::roots::bezier_line::BezierWireResult;
pub(super) struct WindingWork<'a> {
    cancelled: &'a mut (dyn FnMut() -> bool + Send),
    pub options: MaterialOptions,
    pub bisections: u16,
    pub locations: Vec<LoopLocation>,
}
struct LocationQuery {
    region: SurfaceRegionId,
    contour: SurfaceCurveId,
    point: [f64; 2],
    probe: usize,
}
impl<'a> WindingWork<'a> {
    pub fn new(options: MaterialOptions, cancelled: &'a mut (dyn FnMut() -> bool + Send)) -> Self {
        Self {
            cancelled,
            options,
            bisections: 0,
            locations: Vec::new(),
        }
    }
    pub fn locate(
        &mut self,
        region: SurfaceRegionId,
        contour: &[AuthoredPiece],
        point: [f64; 2],
        probe: usize,
    ) -> Checked<(SurfaceCurveId, i64)> {
        let query = LocationQuery::new(region, contour, point, probe)?;
        let (effective, result) = self.query(contour, &query)?;
        let winding = off_boundary(region, query.contour, probe, result.location())?;
        self.record(
            &query,
            winding,
            effective.max_bisections,
            result.bisections(),
        )?;
        Ok((query.contour, winding))
    }
    fn query(
        &mut self,
        contour: &[AuthoredPiece],
        location: &LocationQuery,
    ) -> Checked<(RootIsolationOptions, BezierWireResult)> {
        self.capacity()?;
        let segments: Vec<_> = contour.iter().map(segment).collect();
        let effective = self.remaining();
        let query = BezierWireQuery {
            segments: &segments,
            line: ray(location.point),
            options: effective,
        };
        let result = classify_bezier_wire(query, &mut *self.cancelled)
            .map_err(|cause| location.failure(self.bisections, effective, cause))?;
        Ok((effective, result))
    }
    fn record(
        &mut self,
        query: &LocationQuery,
        winding: i64,
        effective: u16,
        spent: u16,
    ) -> Checked<()> {
        let total = u32::from(self.bisections) + u32::from(spent);
        if total > u32::from(self.options.winding.max_bisections) {
            return Err(MaterialFailure::Limit {
                role: "winding subdivisions",
                actual: total.into(),
                maximum: self.options.winding.max_bisections.into(),
            });
        }
        self.bisections = total as u16;
        self.locations
            .push(query.receipt(winding, effective, spent));
        Ok(())
    }
    fn remaining(&self) -> RootIsolationOptions {
        RootIsolationOptions {
            max_bisections: self.options.winding.max_bisections - self.bisections,
            ..self.options.winding
        }
    }
    fn capacity(&self) -> Checked<()> {
        if self.locations.len() as u64 >= u64::from(self.options.max_locations) {
            return Err(MaterialFailure::Limit {
                role: "wire locations",
                actual: self.locations.len() as u64 + 1,
                maximum: self.options.max_locations.into(),
            });
        }
        Ok(())
    }
}
impl LocationQuery {
    fn new(
        region: SurfaceRegionId,
        pieces: &[AuthoredPiece],
        point: [f64; 2],
        probe: usize,
    ) -> Checked<Self> {
        let contour = pieces
            .first()
            .ok_or(MaterialFailure::Source(region))?
            .span
            .curve;
        Ok(Self {
            region,
            contour,
            point,
            probe,
        })
    }
    fn failure(
        &self,
        previous_bisections: u16,
        effective: RootIsolationOptions,
        cause: BezierWireError,
    ) -> MaterialFailure {
        MaterialFailure::Winding {
            region: self.region,
            contour: self.contour,
            probe: self.probe,
            previous_bisections,
            effective,
            cause: Box::new(cause),
        }
    }
    fn receipt(&self, winding: i64, effective_bisections: u16, bisections: u16) -> LoopLocation {
        LoopLocation {
            target: self.region,
            contour: self.contour,
            probe: self.probe,
            winding,
            effective_bisections,
            bisections,
        }
    }
}
fn segment(piece: &AuthoredPiece) -> BezierWireSegment<'_> {
    BezierWireSegment {
        id: piece.span.curve.0,
        controls: &piece.poles,
        weights: None,
        interval: [0., 1.],
        reversed: false,
    }
}
fn ray(point: [f64; 2]) -> [[f64; 2]; 2] {
    [point, [if point[0] == 0. { 1. } else { 0. }, point[1]]]
}
fn off_boundary(
    region: SurfaceRegionId,
    contour: SurfaceCurveId,
    probe: usize,
    location: &BezierWireLocation,
) -> Checked<i64> {
    match location {
        BezierWireLocation::Outside => Ok(0),
        BezierWireLocation::Inside {
            winding: value @ (-1 | 1),
        } => Ok(*value),
        actual => Err(MaterialFailure::NonJordan {
            region,
            contour,
            probe,
            actual: actual.clone(),
        }),
    }
}
