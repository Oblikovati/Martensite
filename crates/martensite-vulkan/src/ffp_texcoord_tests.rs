//! A live view selects homogeneous texture-matrix semantics before cap UVs are lowered.
use super::*;
use crate::{FrameCapture, Graphic3dRenderDriver, Graphic3dRenderView, RenderBackend};
use crate::{RenderLightingPipeline as Lighting, RenderTextureCoordinatePipeline as Coordinates};
use martensite_tkernel::{
    ncollection::{Mat4, Vec2},
    standard::Handle,
};
use martensite_tkservice::{
    aspect::{AspectNeutralWindow, AspectWindowMembers},
    image::{ImageFormat, ImagePixMap},
};
use std::sync::Arc;

struct UnusedCoordinateBackend;
impl RenderBackend for UnusedCoordinateBackend {
    fn render(&mut self, frame: &RenderFrame) -> Result<FrameCapture, RenderError> {
        Err(RenderError::rejected(
            frame.extent(),
            "CPU extraction without device rendering",
        ))
    }
}
struct CapCoordinateFixture {
    driver: Handle<Graphic3dRenderDriver>,
    view: Handle<Graphic3dRenderView>,
    plane: Handle<Graphic3dClipPlane>,
    source: crate::PrimitiveDraw,
}
impl CapCoordinateFixture {
    fn new(requested: Coordinates) -> Self {
        let driver = Graphic3dRenderDriver::with_backend_and_texture_coordinates(
            UnusedCoordinateBackend,
            64,
            Arc::new(crate::FreeTypeTextSource::default()),
            requested,
        )
        .unwrap();
        let view = coordinate_view(&driver);
        Self {
            driver,
            view,
            plane: coordinate_plane(),
            source: coordinate_cap_geometry(),
        }
    }
    fn draw(&self) -> crate::PrimitiveDraw {
        let rendering = self.view.rendering_params();
        let context = frame_context_state(self.view.get().unwrap(), [1, 1], &rendering, None);
        let pass = crate::render_element_pass::RenderElementPass::opaque(true)
            .with_context_state(Arc::new(context));
        let mut draw = self.source.clone();
        self.texture_cap(&mut draw, &pass, &rendering);
        let frame = RenderFrame::new(1, 1, [0.; 4], vec![draw]).unwrap();
        frame.draws.into_iter().next().unwrap()
    }
    fn texture_cap(
        &self,
        draw: &mut crate::PrimitiveDraw,
        pass: &RenderElementPass,
        rendering: &Graphic3dRenderingParams,
    ) {
        let camera = self.view.camera();
        let fields = coordinate_structure_fields(&self.driver);
        let scene = crate::retained_capping::CappingScene {
            camera: &camera,
            extent: [1, 1],
            fields: &fields,
            rendering,
            clipping: &Handle::default(),
            elements: pass,
        };
        self.apply_texture(draw, &scene);
    }
    fn apply_texture(
        &self,
        draw: &mut crate::PrimitiveDraw,
        scene: &crate::retained_capping::CappingScene<'_>,
    ) {
        crate::capping_texture::apply_cap_texture(
            draw,
            &self.plane.capping_aspect().cast(),
            &self.driver.texture_images,
            scene,
            Mat4::identity(),
            &self.plane,
        )
        .unwrap();
    }
}
fn coordinate_structure_fields(
    driver: &Handle<Graphic3dRenderDriver>,
) -> Graphic3dCStructureFields {
    let manager = Handle::new(Graphic3dStructureManager::new(&driver.cast()).unwrap());
    Graphic3dCStructureBase::new(&manager)
        .unwrap()
        .fields
        .read()
        .clone()
}
fn coordinate_view(driver: &Handle<Graphic3dRenderDriver>) -> Handle<Graphic3dRenderView> {
    let manager = Handle::new(Graphic3dStructureManager::new(&driver.cast()).unwrap());
    let view = driver
        .create_view(&manager)
        .unwrap()
        .cast::<Graphic3dRenderView>();
    let window = Handle::new(AspectNeutralWindow::default());
    window.set_virtual(true);
    window.set_native_handle(91);
    window.set_size(1, 1);
    view.set_window(&Handle::default(), &window.cast(), 0);
    view
}
fn coordinate_plane() -> Handle<Graphic3dClipPlane> {
    let image = Handle::new(ImagePixMap::default());
    assert!(image.init_zero(ImageFormat::Rgba, 2, 2, 0, 0));
    image.set_top_down(true);
    let texture = Handle::new(Graphic3dTexture2D::from_pix_map(image));
    texture.get_params().set_scale(Vec2::new(2., -4.));
    texture.get_params().set_translation(Vec2::new(0.25, 0.5));
    let plane = Handle::new(Graphic3dClipPlane::default());
    plane.set_capping_texture(texture.cast()).unwrap();
    plane
}
fn coordinate_cap_geometry() -> crate::PrimitiveDraw {
    crate::PrimitiveDraw::new(
        crate::capping_geometry::cap_vertices([1.; 4]),
        (0..12).collect(),
        crate::identity_clip(),
    )
    .unwrap()
}
#[test]
fn fixed_view_cap_uvs_preserve_zero_one_and_mixed_homogeneous_q() {
    let fixture = CapCoordinateFixture::new(Coordinates::Programmable);
    fixture
        .driver
        .set_lighting_pipeline(Lighting::FixedFunction)
        .unwrap();
    assert_cap_matrix(&fixture.draw(), true);
}
#[test]
fn programmable_view_preserves_unweighted_texture_translation() {
    let fixture = CapCoordinateFixture::new(Coordinates::Programmable);
    assert_cap_matrix(&fixture.draw(), false);
}
#[test]
fn live_fixed_view_keeps_user_program_texture_coordinates_programmable() {
    let fixture = CapCoordinateFixture::new(Coordinates::Programmable);
    fixture
        .driver
        .set_lighting_pipeline(Lighting::FixedFunction)
        .unwrap();
    fixture
        .plane
        .capping_aspect()
        .set_shader_program(Handle::new(Graphic3dShaderProgram::default()));
    assert_cap_matrix(&fixture.draw(), false);
}
#[test]
fn explicit_generated_coordinates_preserve_programmable_lighting_selection() {
    let fixture = CapCoordinateFixture::new(Coordinates::GeneratedCompatibility);
    assert_eq!(fixture.driver.lighting_pipeline(), Lighting::Programmable);
    assert_cap_matrix(&fixture.draw(), true);
}
#[test]
fn live_fixed_capability_round_trip_restores_requested_programmable_coordinates() {
    let fixture = CapCoordinateFixture::new(Coordinates::Programmable);
    let baseline = fixture.draw();
    assert_cap_matrix(&baseline, false);
    let image = baseline.surface_texture.as_ref().unwrap().image.clone();
    for (mode, homogeneous) in [
        (Lighting::FixedFunction, true),
        (Lighting::Programmable, false),
        (Lighting::FixedFunction, true),
    ] {
        check_coordinate_transition(&fixture, mode, homogeneous, &image);
    }
}
fn check_coordinate_transition(
    fixture: &CapCoordinateFixture,
    mode: Lighting,
    homogeneous: bool,
    image: &Arc<crate::surface_texture::SurfaceTextureImage>,
) {
    fixture.driver.set_lighting_pipeline(mode).unwrap();
    let draw = fixture.draw();
    assert_cap_matrix(&draw, homogeneous);
    assert!(Arc::ptr_eq(
        image,
        &draw.surface_texture.as_ref().unwrap().image
    ));
    assert_eq!(draw.vertices, fixture.source.vertices);
    assert_eq!(draw.indices, fixture.source.indices);
}
fn assert_cap_matrix(draw: &crate::PrimitiveDraw, homogeneous: bool) {
    let uv = &draw.surface_texture.as_ref().unwrap().uv;
    assert_eq!(uv.len(), 12);
    for (vertex, actual) in draw.vertices.iter().zip(uv) {
        let expected = matrix_sample(vertex, homogeneous);
        assert_eq!(
            *actual, expected,
            "matrix input={vertex:?}, homogeneous={homogeneous}"
        );
    }
    assert_mixed_q(uv, homogeneous);
}
fn matrix_sample(vertex: &crate::RenderVertex, homogeneous: bool) -> [f32; 4] {
    // Independent matrix: rows (2,0,0,-1/2), (0,4,0,-3), (0,0,1,0), (0,0,0,1).
    let q = if homogeneous { vertex.weight } else { 1. };
    [
        2. * vertex.position[0] - 0.5 * q,
        -4. * vertex.position[2] - 3. * q,
        0.,
        vertex.weight,
    ]
}
fn assert_mixed_q(uv: &[[f32; 4]], homogeneous: bool) {
    // The supplied cap buffer starts with center(Q=1), +Z(infinity), +X(infinity).
    let mixed: [f32; 4] = std::array::from_fn(|i| 0.25 * uv[0][i] + 0.75 * uv[2][i]);
    let expected = if homogeneous {
        [1.375, -0.75, 0., 0.25]
    } else {
        [1., -3., 0., 0.25]
    };
    assert_eq!(mixed, expected);
    let quotient = [mixed[0] / mixed[3], mixed[1] / mixed[3]];
    assert_eq!(quotient, if homogeneous { [5.5, -3.] } else { [4., -12.] });
}
