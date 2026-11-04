//! Immutable scene-derived constants share the existing operation-local status allocation.
use crate::{PrimitiveDraw, RenderError, RenderWindowDepthFailure as Failure};
pub(super) const SCHEMA: u32 = 0x4346_4434;
fn refused(actual: impl std::fmt::Debug) -> RenderError {
    Failure::UnsupportedInterface { actual: format!("V4 reference constants {actual:?}; expected source camera isometry, matching model, orthographic projection and <=8 single-plane chains") }.into()
}
fn checked_source(
    draw: &PrimitiveDraw,
) -> Result<(&crate::RenderProjectionFactors, &crate::RenderClipping), RenderError> {
    let factors = draw
        .projection_factors
        .as_ref()
        .ok_or_else(|| refused("missing source factors"))?;
    let clipping = draw
        .clipping
        .as_ref()
        .ok_or_else(|| refused("missing source clipping"))?;
    let model = factors.matrices()[2];
    if model.map(f32::to_bits) != clipping.world_from_object().map(f32::to_bits) {
        return Err(refused((model, clipping.world_from_object())));
    }
    require_chains(clipping.chains())?;
    Ok((factors, clipping))
}
pub(super) fn equations(draw: &PrimitiveDraw) -> Result<Vec<[f32; 4]>, RenderError> {
    let (factors, clipping) = checked_source(draw)?;
    let [projection, view, _] = factors.matrices();
    clipping
        .chains()
        .iter()
        .map(|chain| {
            crate::clipped_depth_constants::clipped_depth_reference_equation(
                chain[0],
                projection,
                view,
                factors.has_camera_isometry(),
            )
            .map_err(refused)
            .and_then(require_normal_equation)
        })
        .collect()
}
fn require_chains(chains: &[Vec<[f32; 4]>]) -> Result<(), RenderError> {
    if chains.is_empty() || chains.len() > 8 || chains.iter().any(|chain| chain.len() != 1) {
        return Err(refused(chains));
    }
    Ok(())
}
fn require_normal_equation(equation: [f32; 4]) -> Result<[f32; 4], RenderError> {
    if equation
        .iter()
        .any(|value| !value.is_finite() || value.is_subnormal())
    {
        return Err(refused(equation));
    }
    Ok(equation)
}
pub(super) fn encode(draw: &PrimitiveDraw) -> Result<Vec<u8>, RenderError> {
    Ok(encode_equations(equations(draw)?, SCHEMA, 1))
}
fn encode_equations(equations: Vec<[f32; 4]>, schema: u32, mode: u32) -> Vec<u8> {
    let stride = if cfg!(feature = "command-state-diagnostics") {
        64
    } else {
        0
    };
    let header = [schema, equations.len() as u32, mode, stride];
    let mut bytes: Vec<_> = header.into_iter().flat_map(u32::to_ne_bytes).collect();
    bytes.extend(equations.into_iter().flatten().flat_map(f32::to_ne_bytes));
    bytes
}

pub(super) fn incidence_equations(draw: &PrimitiveDraw) -> Result<Vec<[f32; 4]>, RenderError> {
    let (factors, clipping) = checked_source(draw)?;
    let [projection, view, _] = factors.matrices();
    crate::clipped_depth_constants::require_incidence_camera(
        projection,
        view,
        factors.has_camera_isometry(),
    )
    .map_err(refused)?;
    clipping
        .chains()
        .iter()
        .map(|chain| require_normal_equation(chain[0]))
        .collect()
}
pub(super) fn encode_incidence(draw: &PrimitiveDraw) -> Result<Vec<u8>, RenderError> {
    Ok(encode_equations(incidence_equations(draw)?, 0x4346_4936, 2))
}

fn require_tail_lengths(expected: &[u8], observed: &[u8]) -> Result<(), RenderError> {
    if !expected.len().is_multiple_of(4) || !observed.len().is_multiple_of(4) {
        return Err(refused((
            expected.len(),
            observed.len(),
            "expected complete u32 words",
        )));
    }
    if expected.len() != observed.len() {
        return Err(Failure::GpuReadback {
            expected_bytes: expected.len(),
            observed_bytes: observed.len(),
        }
        .into());
    }
    Ok(())
}
pub(super) fn require_tail(expected: &[u8], observed: &[u8]) -> Result<(), RenderError> {
    require_tail_lengths(expected, observed)?;
    for (word, (a, b)) in expected
        .as_chunks::<4>()
        .0
        .iter()
        .zip(observed.as_chunks::<4>().0.iter())
        .enumerate()
    {
        let expected = u32::from_ne_bytes(*a);
        let observed = u32::from_ne_bytes(*b);
        if expected != observed {
            return Err(Failure::GpuConstantWord {
                word,
                expected,
                observed,
            }
            .into());
        }
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn incidence_constants_have_a_distinct_schema_and_preserve_world_equation_bits() {
        let draw = super::super::clipped_depth_gpu_tests::source_draw();
        let incidence = encode_incidence(&draw).unwrap();
        let legacy = encode(&draw).unwrap();
        assert_eq!(
            u32::from_ne_bytes(incidence[..4].try_into().unwrap()),
            0x4346_4936
        );
        assert_eq!(u32::from_ne_bytes(incidence[8..12].try_into().unwrap()), 2);
        assert_eq!(u32::from_ne_bytes(legacy[..4].try_into().unwrap()), SCHEMA);
        assert_eq!(u32::from_ne_bytes(legacy[8..12].try_into().unwrap()), 1);
        let expected: Vec<_> = [1f32, 0., 0., 0.]
            .into_iter()
            .flat_map(f32::to_ne_bytes)
            .collect();
        assert_eq!(incidence[16..], expected);
    }

    #[test]
    fn missing_union_and_excessive_chains_refuse_before_storage_allocation() {
        assert!(require_chains(&[]).is_err());
        assert!(require_chains(&[vec![[1.; 4]; 2]]).is_err());
        assert!(require_chains(&vec![vec![[1.; 4]]; 9]).is_err());
        assert!(require_chains(&[vec![[1.; 4]]]).is_ok());
    }
    #[test]
    fn after_fence_constants_require_byte_identity_and_complete_length() {
        let original = [1u32.to_ne_bytes(), 2u32.to_ne_bytes()].concat();
        assert!(require_tail(&original, &original).is_ok());
        assert!(require_tail(&original, &original[..4]).is_err());
        assert!(require_tail(&[1, 2, 3], &[1, 2, 3]).is_err());
        let mut changed = original.clone();
        changed[4] ^= 1;
        assert!(matches!(
            require_tail(&original, &changed)
                .unwrap_err()
                .window_depth_failure(),
            Some(Failure::GpuConstantWord { word: 1, .. })
        ));
    }
    #[test]
    fn subnormal_or_nonfinite_clip_constants_refuse_instead_of_assuming_denormal_modes() {
        for value in [f32::from_bits(1), f32::NAN, f32::INFINITY] {
            assert!(require_normal_equation([value, 0., 0., 1.]).is_err());
        }
        assert_eq!(
            require_normal_equation([0., 1., 0., -1.]).unwrap(),
            [0., 1., 0., -1.]
        );
    }
}
