//! Real AIS presentation and pick lowering with a host capture backend.
use crate::{FrameCapture, Graphic3dRenderDriver, RenderBackend, RenderError, RenderFrame};
use martensite_tkernel::standard::Handle;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeSphere;
use martensite_tkservice::{graphic3d::Graphic3dBufferType, image::ImagePixMap};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
use martensite_tkv3d::{
    ais::{AisConnectedInteractive, AisInteractiveContext, AisShape, AisShapeParameters},
    prs_mgr::{PrsMgrDisplayStatus, PrsMgrTypeOfPresentation3d},
    std_prs::{StdPrsMeshLevelRequest, StdPrsMeshLod},
    v3d::{V3dView, V3dViewer},
};
use std::sync::{Arc, Mutex};

type FrameMesh = (usize, bool, bool);

#[derive(Clone, Default)]
struct CaptureMesh(Arc<Mutex<Vec<FrameMesh>>>);

impl RenderBackend for CaptureMesh {
    fn render(&mut self, frame: &RenderFrame) -> Result<FrameCapture, RenderError> {
        let [width, height] = frame.extent();
        let count = frame.draws().iter().map(|draw| draw.indices.len()).sum();
        let kept = frame.immediate_main.is_some();
        self.0
            .lock()
            .unwrap()
            .push((count, frame.uses_mesh_lod, kept));
        let pixels = (width * height) as usize;
        Ok(FrameCapture {
            width,
            height,
            rgba: [40, 60, 90, 255].repeat(pixels),
            depth: vec![0.5; pixels],
        })
    }
}

fn part_with_approval() -> Handle<AisShape> {
    let mut maker = BRepPrimAPIMakeSphere::new(1.).unwrap();
    let part = AisShape::new(maker.shape().unwrap());
    part.set_own_deviation_coefficient(0.001);
    let request = StdPrsMeshLevelRequest {
        linear_deflection: 0.3,
        angular_deflection: 0.8,
        maximum_deviation: 0.5,
    };
    part.set_presentation_mesh_lod(Some(StdPrsMeshLod::new(vec![request], 3.).unwrap()));
    part
}

fn capture_at_scale(view: &V3dView, seen: &CaptureMesh, scale: f64) -> FrameMesh {
    let _ = capture_image(view, scale);
    *seen.0.lock().unwrap().last().unwrap()
}

fn capture_image(view: &V3dView, scale: f64) -> ImagePixMap {
    view.camera().set_scale(scale);
    let target = ImagePixMap::default();
    assert!(
        view.to_pix_map_sized(&target, 128, 128, Graphic3dBufferType::Rgb, true)
            .unwrap()
    );
    target
}

struct ConnectedPart {
    view: Handle<V3dView>,
    context: Handle<AisInteractiveContext>,
    _occurrence: Handle<AisConnectedInteractive>,
}

fn display_connected_part(driver: &Handle<Graphic3dRenderDriver>) -> ConnectedPart {
    let viewer = V3dViewer::new(&driver.cast()).unwrap();
    let view = crate::immediate_redraw_fixture::lit_sphere_view(&viewer);
    let view = view.cast::<V3dView>();
    let context = AisInteractiveContext::new(&viewer).unwrap();
    let part = part_with_approval();
    let occurrence = AisConnectedInteractive::new(PrsMgrTypeOfPresentation3d::AllView);
    occurrence.connect(&part.cast()).unwrap();
    context
        .display_with_modes(&occurrence.cast(), 1, 0, false, PrsMgrDisplayStatus::None)
        .unwrap();
    view.auto_z_fit().unwrap();
    ConnectedPart {
        view,
        context,
        _occurrence: occurrence,
    }
}

#[test]
fn a_connected_part_reselects_meshes_for_capture_and_preserves_picking() {
    let seen = CaptureMesh::default();
    let driver = Graphic3dRenderDriver::with_backend(seen.clone(), 4096).unwrap();
    let part = display_connected_part(&driver);
    let fine = capture_at_scale(&part.view, &seen, 5.);
    let coarse = capture_at_scale(&part.view, &seen, 40.);
    assert!(
        fine.0 > coarse.0 && coarse.0 > 0,
        "{fine:?} versus {coarse:?}"
    );
    assert_eq!(
        (fine.1, fine.2, coarse.1, coarse.2),
        (true, false, true, false)
    );
    part.context
        .move_to(64, 64, &part.view.cast(), false)
        .unwrap();
    assert!(part.context.has_detected());
    part.context.clear_detected(false);
    assert_eq!(fine, capture_at_scale(&part.view, &seen, 5.));
}

#[test]
fn native_connected_part_captures_at_both_scales_and_returns_to_exact_fine_pixels() {
    let driver = Graphic3dRenderDriver::new(crate::VulkanOptions {
        validation: true,
        device_index: Some(0),
    })
    .unwrap();
    let part = display_connected_part(&driver);
    part.view.redraw();
    let fine = capture_image(&part.view, 5.);
    let coarse = capture_image(&part.view, 40.);
    let fine_again = capture_image(&part.view, 5.);
    for image in [&fine, &coarse] {
        assert_ne!(
            image.pixel_color(64, 64, false).unwrap(),
            image.pixel_color(0, 0, false).unwrap(),
        );
    }
    assert_eq!(fine.data().data.data(), fine_again.data().data.data());
    assert!(driver.validation_errors().unwrap().is_empty());
}
