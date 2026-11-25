// SPDX-License-Identifier: GPL-2.0-only
//! Capture the exact directed source used by both analytic support families.
use super::*;
use martensite_tkbo::body_check::DirectedPlanarFace;

pub(in crate::surface_sketch::wrapped_profile) fn capture(
    originals: &[impl AsRef<DevelopedRegionRequest>],
) -> Result<(SketchProfile, Vec<DirectedPlanarFace>), PlanarFailure> {
    let mut profile = combined_profile(originals)?;
    literal_lines(&mut profile)?;
    let faces = originals
        .iter()
        .map(|source| face(source.as_ref()))
        .collect::<Result<_, _>>()?;
    Ok((profile, faces))
}

fn literal_lines(profile: &mut SketchProfile) -> Result<(), PlanarFailure> {
    let by_id: BTreeMap<_, _> = profile
        .vertices
        .iter()
        .map(|vertex| (vertex.id, vertex.point))
        .collect();
    for curve in &mut profile.curves {
        if let ProfileCurve::Line { id, start, end } = *curve {
            let a = by_id
                .get(&start)
                .ok_or(PlanarFailure::MissingVertex(start))?;
            let b = by_id.get(&end).ok_or(PlanarFailure::MissingVertex(end))?;
            *curve = ProfileCurve::Spline {
                id,
                start,
                end,
                degree: 1,
                poles: vec![*a, *b],
                knots: vec![0., 0., 1., 1.],
                weights: None,
            };
        }
    }
    Ok(())
}

fn face(original: &DevelopedRegionRequest) -> Result<DirectedPlanarFace, PlanarFailure> {
    let mut loops = original.loops.iter().map(|wire| ProfileLoop {
        uses: wire
            .iter()
            .map(|piece| LoopUse {
                curve: ProfileCurveId(piece.span.curve.0),
                reversed: false,
            })
            .collect(),
    });
    let outer = loops.next().ok_or(PlanarFailure::EmptyMaterial)?;
    Ok(DirectedPlanarFace {
        outer,
        holes: loops.collect(),
    })
}
