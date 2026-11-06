//! Line-only polygon offset on the software reference driver (diagnosis 2026-10-28).
//!
//! Polygon offset enables `GL_POLYGON_OFFSET_LINE` alone for
//! `AspectPOM_Line`, so the cap face stays unbiased and the polygon-mode edges, pushed behind
//! the coplanar face, fail the cap's `GL_LESS` test: the frame is the edges-disabled frame, as
//! the recorded AMD capture shows. Mesa's GL frontend copies the offset units and factor whenever
//! any of point, line or fill offset is enabled (`st_atom_rasterizer.c`) and llvmpipe biases
//! every triangle with them (`lp_state_setup.c`), so its line-only capture equals its
//! both-offset capture byte for byte. The software asset of a line-only mode is therefore the
//! llvmpipe edges-disabled asset, which states the GL result.
use super::*;

/// Line-only offset modes and the software asset that states their GL image.
const LINE_ONLY_OFFSET_SOFTWARE: &[(&str, NativeTriangleEdgeAsset)] = &[(
    "cap-section-edges-oblique-dashed-line-offset",
    cap_asset!("cap-section-edges-oblique-edges-disabled"),
)];

/// The asset to grade `asset` against on `driver`; `line_offset_reference(&driver, asset)`.
pub(crate) fn line_offset_reference(
    driver: &Handle<Graphic3dRenderDriver>,
    asset: NativeTriangleEdgeAsset,
) -> NativeTriangleEdgeAsset {
    if !driver.device_name().contains("llvmpipe") {
        return asset;
    }
    let reference = LINE_ONLY_OFFSET_SOFTWARE
        .iter()
        .find(|(mode, _)| *mode == asset.0);
    reference.map_or(asset, |(mode, edges_disabled)| {
        (mode, asset.1, asset.2, edges_disabled.3, edges_disabled.4)
    })
}

#[test]
fn native_assets_prove_the_llvmpipe_line_offset_bias_and_the_gl_image() {
    let both = cap_asset!("cap-section-edges-oblique-dashed-both-offset");
    for (mode, edges_disabled) in LINE_ONLY_OFFSET_SOFTWARE {
        let asset = FULL_CAP_EDGE_ASSETS.iter().find(|a| a.0 == *mode).unwrap();
        // llvmpipe: line-only equals both-offset, the face carries the bias.
        assert_eq!((asset.3, asset.4), (both.3, both.4), "{mode}");
        assert_ne!(
            (asset.3, asset.4),
            (edges_disabled.3, edges_disabled.4),
            "{mode}"
        );
        // AMD native GL: line-only hides the biased edges behind the unbiased face.
        assert_eq!(
            (asset.1, asset.2),
            (edges_disabled.1, edges_disabled.2),
            "{mode}"
        );
    }
}
