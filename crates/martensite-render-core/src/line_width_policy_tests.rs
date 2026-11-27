use super::*;
use crate::{LineStyle, PrimitiveDraw, RenderFrame, RenderVertex};

fn line(width: f32) -> PrimitiveDraw {
    let vertices = vec![
        RenderVertex::new([-0.5, 0., 0.], [1.; 4]),
        RenderVertex::new([0.5, 0., 0.], [1.; 4]),
    ];
    let style = LineStyle::new(width, 0xfff0, 3).unwrap();
    PrimitiveDraw::segments(vertices, vec![0, 1], crate::identity_clip(), style).unwrap()
}

fn frame(draws: Vec<PrimitiveDraw>) -> RenderFrame {
    RenderFrame::new(64, 64, [0.; 4], draws).unwrap()
}

#[test]
fn single_pixel_policy_preserves_requested_aspects_and_geometry() {
    let original = frame(vec![line(2.), line(7.)]);
    let single = original
        .clone()
        .with_line_width_policy(RenderLineWidthPolicy::SinglePixelV1);
    for (before, after) in original.draws().iter().zip(single.draws()) {
        assert_eq!(after.line_style.unwrap().raster_width(), 1.);
        assert_eq!(
            before.line_style().unwrap().parameters(),
            after.line_style().unwrap().parameters()
        );
        assert_eq!(before.vertices, after.vertices);
        assert_eq!(before.indices, after.indices);
    }
    assert_eq!(original.draws()[0].line_style.unwrap().raster_width(), 2.);
}

#[test]
fn default_policy_and_explicit_replay_retain_aspect_widths() {
    assert_eq!(
        RenderLineWidthPolicy::default(),
        RenderLineWidthPolicy::AspectWidthsV1
    );
    let original = frame(vec![line(4.)]);
    let replay = original
        .clone()
        .with_line_width_policy(RenderLineWidthPolicy::SinglePixelV1)
        .with_line_width_policy(RenderLineWidthPolicy::AspectWidthsV1);
    assert_eq!(replay.draws(), original.draws());
    assert_eq!(replay.draws()[0].line_style.unwrap().raster_width(), 4.);
}

#[test]
fn polygon_and_inherited_widths_obey_the_same_raster_policy() {
    let mut inherited = line(1.);
    inherited.line_style = inherited.line_style.map(LineStyle::with_context_width);
    let original = frame(vec![line(5.), inherited, polygon()]);
    assert_eq!(original.draws()[1].line_style.unwrap().parameters().0, 5.);
    let single = original.with_line_width_policy(RenderLineWidthPolicy::SinglePixelV1);
    for draw in single.draws() {
        assert_eq!(
            draw.line_style
                .or(draw.polygon_edge_style)
                .unwrap()
                .raster_width(),
            1.
        );
    }
}

fn polygon() -> PrimitiveDraw {
    let mut polygon = PrimitiveDraw::new(
        vec![RenderVertex::new([0.; 3], [1.; 4]); 3],
        vec![0, 1, 2],
        crate::identity_clip(),
    )
    .unwrap();
    polygon.polygon_edge_style = Some(LineStyle::new(3., 0xffff, 1).unwrap());
    polygon
}

#[test]
fn color_highlighting_keeps_its_requested_width_under_single_pixel_rasterization() {
    let mut draw = line(2.);
    let fields = cyan_highlight();
    crate::highlight_style::apply_highlight(&mut draw, &fields).unwrap();
    assert_eq!(draw.vertices[0].color, [0., 1., 1., 1.]);
    let single = frame(vec![draw]).with_line_width_policy(RenderLineWidthPolicy::SinglePixelV1);
    assert_eq!(single.draws()[0].line_style.unwrap().parameters().0, 2.);
    assert_eq!(single.draws()[0].line_style.unwrap().raster_width(), 1.);
}

fn cyan_highlight() -> martensite_tkservice::graphic3d::Graphic3dCStructureFields {
    use martensite_tkernel::{quantity::ColorRgba, standard::Handle};
    use martensite_tkservice::graphic3d::*;
    let driver = martensite_tkservice::null_driver::NullGraphicDriver::new_handle();
    let manager = Handle::new(Graphic3dStructureManager::new(&driver).unwrap());
    let mut fields = Graphic3dCStructureBase::new(&manager)
        .unwrap()
        .fields
        .read()
        .clone();
    fields.highlight = true;
    fields.highlight_style = Handle::new(Graphic3dPresentationAttributes::default()).cast();
    fields
        .highlight_style
        .set_color(*ColorRgba::new(0., 1., 1., 1.).unwrap().rgb());
    fields
}

struct ExpectedRasterWidthBackend {
    width: f32,
}
impl crate::RenderBackend for ExpectedRasterWidthBackend {
    fn render(&mut self, frame: &RenderFrame) -> Result<crate::FrameCapture, crate::RenderError> {
        assert_eq!(frame.draws().len(), 1);
        let style = frame.draws()[0].line_style().unwrap();
        assert_eq!(style.parameters(), (2., 0xfff0, 3));
        assert_eq!(style.raster_width(), self.width);
        Ok(crate::FrameCapture {
            width: 64,
            height: 64,
            rgba: vec![0; 64 * 64 * 4],
            depth: vec![1.; 64 * 64],
        })
    }
}

#[test]
fn injected_driver_passes_immutable_width_policy_to_view_and_gui_draws() {
    for (policy, width) in [
        (RenderLineWidthPolicy::AspectWidthsV1, 2.),
        (RenderLineWidthPolicy::SinglePixelV1, 1.),
    ] {
        let driver = raster_driver(policy, width);
        assert_eq!(driver.line_width_policy(), policy);
        let view = raster_view(&driver);
        let layer = crate::RenderLayer::new(vec![line(2.)], false, false, false);
        view.attach_gui_layer([64, 64], layer).unwrap();
        view.try_redraw().unwrap();
    }
}

fn raster_driver(
    policy: RenderLineWidthPolicy,
    width: f32,
) -> martensite_tkernel::standard::Handle<crate::Graphic3dRenderDriver> {
    crate::Graphic3dRenderDriver::with_backend_and_line_width_configuration(
        ExpectedRasterWidthBackend { width },
        64,
        std::sync::Arc::new(crate::FreeTypeTextSource::default()),
        Default::default(),
        policy,
    )
    .unwrap()
}

fn raster_view(
    driver: &martensite_tkernel::standard::Handle<crate::Graphic3dRenderDriver>,
) -> martensite_tkernel::standard::Handle<crate::Graphic3dRenderView> {
    use martensite_tkernel::standard::Handle;
    use martensite_tkservice::{aspect::*, graphic3d::*};
    let manager = Handle::new(Graphic3dStructureManager::new(&driver.cast()).unwrap());
    let view = driver
        .create_view(&manager)
        .unwrap()
        .cast::<crate::Graphic3dRenderView>();
    let window = Handle::new(AspectNeutralWindow::default());
    window.set_virtual(true);
    window.set_native_handle(91);
    window.set_size(64, 64);
    view.set_window(&Handle::default(), &window.cast(), 0);
    view
}
