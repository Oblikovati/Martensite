//! Read existing solved-profile selections without changing authored parameter identity.
use super::*;
use martensite_tkbo::profile_extrusion::{
    LoopUse, ProfileCurve, ProfileCurveId, ProfileLoop, ProfilePlane, ProfileVertexId,
    SketchProfile,
};
use std::collections::BTreeMap;
mod curves;
mod error;
pub(super) use error::ProfileInputFailure;
type Checked<T> = Result<T, ProfileInputFailure>;

pub(super) struct ProfileSelection {
    pub region: SurfaceRegionId,
    pub outer: ProfileLoop,
    pub holes: Vec<ProfileLoop>,
}
pub(super) struct OrientedRegion {
    pub region: SurfaceRegionId,
    pub loops: Vec<Vec<AuthoredPiece>>,
    pub directions: CurveDirections,
}
pub(super) struct CurveDirections(BTreeMap<SurfaceCurveId, bool>);
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct SourceParameterInterval {
    pub curve: SurfaceCurveId,
    pub level: u8,
    pub index: u64,
    pub reversed: bool,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct SourceParameterPoint {
    pub curve: SurfaceCurveId,
    pub level: u8,
    pub numerator: u64,
}
struct ProfileIndex<'a> {
    vertices: BTreeMap<ProfileVertexId, [f64; 2]>,
    curves: BTreeMap<ProfileCurveId, &'a ProfileCurve>,
}

pub(super) struct ProfileReader<'a>(ProfileIndex<'a>);

impl<'a> ProfileReader<'a> {
    pub(super) fn new(profile: &'a SketchProfile) -> Checked<Self> {
        ProfileIndex::new(profile).map(Self)
    }
    pub(super) fn select(&self, selection: &ProfileSelection) -> Checked<OrientedRegion> {
        let mut directions = BTreeMap::new();
        let loops = std::iter::once(&selection.outer)
            .chain(&selection.holes)
            .enumerate()
            .map(|(contour, uses)| {
                read_loop(uses, selection.region, contour, &self.0, &mut directions)
            })
            .collect::<Checked<_>>()?;
        Ok(OrientedRegion {
            region: selection.region,
            loops,
            directions: CurveDirections(directions),
        })
    }
}

impl OrientedRegion {
    pub(super) fn into_developed(
        self,
        depths: [f64; 2],
        tolerance: f64,
        max_fit_level: u8,
    ) -> (DevelopedRegionRequest, CurveDirections) {
        let source = DevelopedRegionRequest {
            region: self.region,
            loops: self.loops,
            depths,
            tolerance,
            max_fit_level,
        };
        (source, self.directions)
    }
}

#[cfg(test)]
pub(super) fn from_profile(
    profile: &SketchProfile,
    selection: &ProfileSelection,
) -> Checked<OrientedRegion> {
    ProfileReader::new(profile)?.select(selection)
}

fn read_loop(
    contour: &ProfileLoop,
    region: SurfaceRegionId,
    contour_index: usize,
    index: &ProfileIndex<'_>,
    directions: &mut BTreeMap<SurfaceCurveId, bool>,
) -> Checked<Vec<AuthoredPiece>> {
    if contour.uses.is_empty() {
        return Err(ProfileInputFailure::EmptyLoop {
            region,
            contour: contour_index,
        });
    }
    contour
        .uses
        .iter()
        .map(|use_| read_use(use_, index, directions))
        .collect()
}

fn read_use(
    use_: &LoopUse,
    index: &ProfileIndex<'_>,
    directions: &mut BTreeMap<SurfaceCurveId, bool>,
) -> Checked<AuthoredPiece> {
    let id = SurfaceCurveId(use_.curve.0);
    if directions.insert(id, use_.reversed).is_some() {
        return Err(ProfileInputFailure::ReusedCurve(use_.curve));
    }
    let curve = index
        .curves
        .get(&use_.curve)
        .ok_or(ProfileInputFailure::MissingCurve(use_.curve))?;
    curves::read(curve, use_.reversed, index)
}

impl ProfileIndex<'_> {
    fn new(profile: &SketchProfile) -> Checked<ProfileIndex<'_>> {
        if profile.plane != ProfilePlane::world_xy() {
            return Err(ProfileInputFailure::Plane(profile.plane));
        }
        Ok(ProfileIndex {
            vertices: unique(
                profile.vertices.iter().map(|v| (v.id, v.point)),
                ProfileInputFailure::DuplicateVertex,
            )?,
            curves: unique(
                profile.curves.iter().map(|c| (c.id(), c)),
                ProfileInputFailure::DuplicateCurve,
            )?,
        })
    }
    fn vertex(&self, curve: ProfileCurveId, vertex: ProfileVertexId) -> Checked<[f64; 2]> {
        self.vertices
            .get(&vertex)
            .copied()
            .ok_or(ProfileInputFailure::MissingVertex { curve, vertex })
    }
}

fn unique<K: Ord + Copy, V>(
    entries: impl Iterator<Item = (K, V)>,
    duplicate: impl Fn(K) -> ProfileInputFailure,
) -> Checked<BTreeMap<K, V>> {
    let mut indexed = BTreeMap::new();
    for (key, value) in entries {
        if indexed.insert(key, value).is_some() {
            return Err(duplicate(key));
        }
    }
    Ok(indexed)
}

impl CurveDirections {
    pub(super) fn source_interval(&self, span: CurveSpan) -> Checked<SourceParameterInterval> {
        mapped_interval(span, self.reversed(span.curve)?)
    }
    pub(super) fn source_point(
        &self,
        curve: SurfaceCurveId,
        numerator: u32,
        level: u8,
    ) -> Checked<SourceParameterPoint> {
        mapped_point(curve, numerator, level, self.reversed(curve)?)
    }
    fn reversed(&self, curve: SurfaceCurveId) -> Checked<bool> {
        self.0
            .get(&curve)
            .copied()
            .ok_or(ProfileInputFailure::UnknownSource(curve))
    }
}

fn mapped_point(
    curve: SurfaceCurveId,
    numerator: u32,
    level: u8,
    reversed: bool,
) -> Checked<SourceParameterPoint> {
    let whole = point_domain(curve, numerator, level)?;
    let point = if reversed {
        whole - u64::from(numerator)
    } else {
        u64::from(numerator)
    };
    Ok(canonical_point(curve, point, level))
}

fn point_domain(curve: SurfaceCurveId, numerator: u32, level: u8) -> Checked<u64> {
    1_u64
        .checked_shl(u32::from(level))
        .filter(|whole| numerator > 0 && u64::from(numerator) < *whole)
        .ok_or(ProfileInputFailure::Point {
            curve,
            numerator,
            level,
        })
}

fn canonical_point(curve: SurfaceCurveId, numerator: u64, level: u8) -> SourceParameterPoint {
    let shift = numerator.trailing_zeros().min(u32::from(level));
    SourceParameterPoint {
        curve,
        level: level - shift as u8,
        numerator: numerator >> shift,
    }
}

fn mapped_interval(span: CurveSpan, reversed: bool) -> Checked<SourceParameterInterval> {
    let count = 1_u64
        .checked_shl(u32::from(span.level))
        .filter(|count| u64::from(span.index) < *count)
        .ok_or(ProfileInputFailure::Interval(span))?;
    let index = if reversed {
        count - 1 - u64::from(span.index)
    } else {
        u64::from(span.index)
    };
    Ok(SourceParameterInterval {
        curve: span.curve,
        level: span.level,
        index,
        reversed,
    })
}

#[cfg(test)]
mod tests;
