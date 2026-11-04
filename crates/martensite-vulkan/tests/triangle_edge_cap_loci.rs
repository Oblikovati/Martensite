//! Loci of polygon-mode cap edges from the section geometry (user decisions 2026-10-28).
//!
//! The cap is a fan around the centre C whose edges are the plane's local X and Z axes through
//! C, each a pair of rays to vertices at infinity (`cap_edge_infinity_reference`). A polygon-mode
//! edge of width w covers the band within w/2 of its line, so its two sides are fill edges:
//! where a side passes through pixel centres (the 3 px ray at window y = 32, sides 30.5 and
//! 33.5), GL and Vulkan break the tie on opposite sides. Two kinds of band are undefined:
//! - a dashed edge starts at a vertex at infinity (w = 0), so its stipple phase is undefined;
//! - without polygon offset the edges are exactly coplanar with the cap face (the same
//!   triangles), so `GL_LESS` picks edge or face by depth rounding.
use super::cap_edge_infinity_reference::fan_lines;
use super::*;
use crate::exact_tie_reference::{
    DriverControls, TieControls, TieFrame, TieGrade, TieLocus, UndefinedLocus,
    controlled_tie_violation,
};

/// A controlled cap mode: (mode, face-only capture, edge-winning capture), captured by
/// the recorded capping_texture_capture capture on both native drivers. The face-only
/// capture disables the edges. The edge-winning capture draws the same edges over the face:
/// a dashed hollow cap draws them alone (the fill is skipped), and a fill offset pushes the
/// face behind unbiased polygon-mode edges. Solid hollow caps draw face lines instead of the
/// edges, so they are no edge capture; modes without undefined loci need none.
type CapControl = (
    &'static str,
    NativeTriangleEdgeAsset,
    Option<NativeTriangleEdgeAsset>,
);
const CAP_CONTROL_ASSETS: &[CapControl] = &[
    (
        "cap-section-edges-oblique-polygon-fill-offset",
        cap_asset!("cap-section-edges-oblique-polygon-fill-offset-edges-disabled"),
        None,
    ),
    (
        "cap-section-edges-oblique-dashed-fill-offset",
        cap_asset!("cap-section-edges-oblique-dashed-fill-offset-edges-disabled"),
        Some(cap_asset!(
            "cap-section-edges-oblique-dashed-fill-offset-face-hidden"
        )),
    ),
    (
        "cap-section-edges-oblique-polygon",
        cap_asset!("cap-section-edges-oblique-polygon-edges-disabled"),
        Some(cap_asset!("cap-section-edges-oblique-polygon-fill-offset")),
    ),
    (
        "cap-section-edges-oblique-generated-polygon",
        cap_asset!("cap-section-edges-oblique-generated-polygon-edges-disabled"),
        Some(cap_asset!(
            "cap-section-edges-oblique-generated-polygon-fill-offset"
        )),
    ),
];

/// Native hollow-cap (empty interior) and edges-disabled images that the controls rely on.
const HOLLOW_CAP_ASSETS: &[NativeTriangleEdgeAsset] = &[
    cap_asset!("cap-section-edges-oblique-polygon-fill-offset-face-hidden"),
    cap_asset!("cap-section-edges-oblique-dashed-fill-offset-face-hidden"),
    cap_asset!("cap-section-edges-oblique-polygon-face-hidden"),
    cap_asset!("cap-section-edges-oblique-generated-polygon-face-hidden"),
    cap_asset!("cap-section-edges-oblique-polygon-fill-offset-edges-disabled"),
    cap_asset!("cap-section-edges-oblique-dashed-fill-offset-edges-disabled"),
    cap_asset!("cap-section-edges-oblique-polygon-edges-disabled"),
    cap_asset!("cap-section-edges-oblique-generated-polygon-edges-disabled"),
    cap_asset!("cap-section-edges-oblique-generated-polygon-fill-offset"),
];

/// Grade `capture` of a controlled mode by the tie, uncovered-surface and undefined-locus
/// rules; `None` for modes without controls. `grade_with_controls(&driver, mode, &c, native)`.
pub(crate) fn grade_with_controls(
    driver: &Handle<Graphic3dRenderDriver>,
    mode: &str,
    capture: &FrameCapture,
    native: (&[u8], &[u8]),
) -> Option<Result<TieGrade, String>> {
    let (_, face, edges) = CAP_CONTROL_ASSETS.iter().find(|a| a.0 == mode)?;
    let loci = cap_edge_loci(driver, mode);
    let rust = [Some(*face), *edges].map(|a| a.map(|a| capture_cap_boundary(driver, a.0)));
    let assets = [Some(*face), *edges].map(|a| a.map(|a| select_cap_asset(driver, a)));
    let depths = assets.map(|a| a.map(|(_, _, depth)| depth_values(depth)));
    let rust_frame = |i: usize| rust[i].as_ref().map(|c| frame(&c.rgba, &c.depth));
    let rust_controls = own_controls(rust_frame(0).unwrap(), rust_frame(1));
    let native_frame = |i: usize| Some(frame(assets[i]?.1, depths[i].as_ref()?));
    let native_controls = own_controls(native_frame(0).unwrap(), native_frame(1));
    let controls = TieControls {
        rust: rust_controls,
        native: native_controls,
        undefined: &loci.undefined,
    };
    let (width, height) = (capture.width as usize, capture.height as usize);
    let native_depth = depth_values(native.1);
    let (actual, expected) = (
        frame(&capture.rgba, &capture.depth),
        frame(native.0, &native_depth),
    );
    Some(controlled_tie_violation(
        width, height, actual, expected, &loci.ties, &controls,
    ))
}

fn frame<'a>(rgba: &'a [u8], depth: &'a [f32]) -> TieFrame<'a> {
    TieFrame { rgba, depth }
}

/// The edge is the tying primitive, so the face-only capture is also the uncovered one.
fn own_controls<'a>(face: TieFrame<'a>, edges: Option<TieFrame<'a>>) -> DriverControls<'a> {
    DriverControls {
        uncovered: Some(face),
        face_only: Some(face),
        edge_only: edges,
    }
}

/// Little-endian f32 depth bytes of a native asset.
fn depth_values(bytes: &[u8]) -> Vec<f32> {
    bytes
        .as_chunks::<4>()
        .0
        .iter()
        .map(|b| f32::from_le_bytes(*b))
        .collect()
}

/// Proven ties and undefined bands of one cap mode; `cap_edge_loci(mode, &driver)`.
pub(crate) struct CapEdgeLoci {
    pub ties: Vec<TieLocus>,
    pub undefined: Vec<UndefinedLocus>,
}

/// Compute the loci from the live fixture's plane and edge width.
pub(crate) fn cap_edge_loci(driver: &Handle<Graphic3dRenderDriver>, mode: &str) -> CapEdgeLoci {
    let fixture = capping_scene::CappingTextureScene::new(driver, mode);
    let inputs = super::cap_infinity::edge_inputs(&fixture);
    fixture.scene.view.remove();
    let half = edge_raster_width(mode, inputs.width) / 2.;
    let lines = fan_lines(&inputs).map(|(point, direction)| normal_form(point, direction));
    let ties = lines
        .iter()
        .flat_map(|&(a, b, c)| [c - half, c + half].map(|c| TieLocus::FillEdge { a, b, c }))
        .collect();
    let undefined = if undefined_edges(mode) {
        lines
            .iter()
            .map(|&(a, b, c)| UndefinedLocus {
                a,
                b,
                c,
                half_width: half,
            })
            .collect()
    } else {
        Vec::new()
    };
    CapEdgeLoci { ties, undefined }
}

/// A hollow cap with solid polygon-mode edges draws its faces as lines at the context line width,
/// which no earlier draw of these scenes sets (default 1); other modes use the edge width.
fn edge_raster_width(mode: &str, edge_width: f64) -> f64 {
    if mode.contains("face-hidden") && !mode.contains("dashed") {
        return 1.;
    }
    edge_width
}

/// Dashed edges from a vertex at infinity, or polygon edges coplanar with an unbiased face.
fn undefined_edges(mode: &str) -> bool {
    let coplanar = mode.contains("polygon") && !mode.contains("offset");
    mode.contains("dashed") || coplanar
}

/// Unit normal form (a, b, c) of the line through `point` along `direction`.
fn normal_form(point: [f64; 2], direction: [f64; 2]) -> (f64, f64, f64) {
    let length = martensite_cmath::hypot(direction[0], direction[1]);
    let (a, b) = (direction[1] / length, -direction[0] / length);
    (a, b, -(a * point[0] + b * point[1]))
}

#[test]
fn the_oblique_cap_edges_tie_on_rows_30_and_33_and_nowhere_else() {
    let driver = Graphic3dRenderDriver::with_surface_pipelines(
        device_options::validation_options(),
        RenderTextureCoordinatePipeline::Programmable,
        RenderMeshEdgePipeline::PolygonCompatibility,
    )
    .unwrap();
    let loci = cap_edge_loci(&driver, "cap-section-edges-oblique-polygon-fill-offset");
    let tied = |row| (0..64).any(|column| loci.ties.iter().any(|t| t.ties(column, row)));
    let rows: Vec<usize> = (0..64).filter(|row| tied(*row)).collect();
    assert_eq!(rows, [30, 33]);
    assert!(loci.undefined.is_empty());
    let dashed = cap_edge_loci(&driver, "cap-section-edges-oblique-dashed-fill-offset");
    assert!(dashed.undefined[0].contains(17, 33) && !dashed.undefined[0].contains(17, 34));
    assert!(undefined_edges("cap-section-edges-oblique-polygon"));
    assert!(!undefined_edges(
        "cap-section-edges-oblique-polygon-fill-offset"
    ));
}

/// The hollow caps and edge controls themselves against recorded images (an
/// empty interior: dashed edges only, solid polygon-mode edges as face lines at width 1).
#[test]
fn cap_control_captures_match_the_native_controls() {
    let mut failures = Vec::new();
    for asset in HOLLOW_CAP_ASSETS {
        let (coordinates, edges) = cap_policy::cap_fixture_pipelines(asset.0);
        let driver = Graphic3dRenderDriver::with_surface_pipelines(
            device_options::validation_options(),
            coordinates,
            edges,
        )
        .unwrap();
        let (name, rgba, depth) = select_cap_asset(&driver, *asset);
        let capture = capture_cap_boundary(&driver, name);
        if let Err(failure) = grade_control(&driver, name, &capture, (rgba, depth)) {
            failures.push(format!("{name}: {failure}"));
        }
        assert!(driver.validation_errors().unwrap().is_empty());
    }
    assert!(failures.is_empty(), "{failures:#?}");
}

/// A control capture by its own loci; at an undefined pixel each control is its own definition.
fn grade_control(
    driver: &Handle<Graphic3dRenderDriver>,
    name: &str,
    capture: &FrameCapture,
    (rgba, depth): (&[u8], &[u8]),
) -> Result<TieGrade, String> {
    let loci = cap_edge_loci(driver, name);
    let depth = depth_values(depth);
    let rust = frame(&capture.rgba, &capture.depth);
    let native = frame(rgba, &depth);
    let itself = |frame| DriverControls {
        edge_only: Some(frame),
        ..Default::default()
    };
    let controls = TieControls {
        rust: itself(rust),
        native: itself(native),
        undefined: &loci.undefined,
    };
    let (width, height) = (capture.width as usize, capture.height as usize);
    controlled_tie_violation(width, height, rust, native, &loci.ties, &controls)
}
