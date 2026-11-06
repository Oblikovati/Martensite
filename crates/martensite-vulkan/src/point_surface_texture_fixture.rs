//! Named CPU extraction fixture with independent palette, UVs and an unreachable renderer.
use crate::{Graphic3dDrawGroup, PrimitiveDraw, RenderError, RenderFrame};
use martensite_tkernel::standard::Handle;
use martensite_tkservice::{
    graphic3d::*,
    image::{ImageFormat, ImagePixMap},
};

struct PointTextureNeverRender;
impl crate::RenderBackend for PointTextureNeverRender {
    fn render(&mut self, _: &RenderFrame) -> Result<crate::FrameCapture, RenderError> {
        panic!("CPU extraction control must not render");
    }
}
pub(crate) struct PointTextureFixture {
    pub driver: Handle<crate::Graphic3dRenderDriver>,
    _structure: Handle<Graphic3dStructure>,
    _manager: Handle<Graphic3dStructureManager>,
    group: Handle<Graphic3dDrawGroup>,
    pub aspect: Handle<Graphic3dAspects>,
    pub texture: Handle<Graphic3dTexture2D>,
}
impl PointTextureFixture {
    pub fn new(top_down: bool, colors: bool) -> Self {
        Self::with_normals(top_down, colors, true)
    }
    pub fn with_normals(top_down: bool, colors: bool, normals: bool) -> Self {
        let driver =
            crate::Graphic3dRenderDriver::with_backend(PointTextureNeverRender, 64).unwrap();
        let manager = Handle::new(Graphic3dStructureManager::new(&driver.cast()).unwrap());
        let structure = Graphic3dStructure::new(&manager, &Handle::default()).unwrap();
        let group = Graphic3dDrawGroup::new(&structure.cast());
        let (texture, aspect) = configure_group(&group, top_down, colors, normals);
        structure.set_visible(true);
        Self {
            driver,
            _structure: structure,
            _manager: manager,
            group,
            aspect,
            texture,
        }
    }
    /// Fail before any resident image exists; `PointTextureFixture::unavailable_base_image(false)`.
    pub fn unavailable_base_image(colors: bool) -> Self {
        let fixture = Self::new(true, colors);
        fixture.texture.set_image(Handle::default());
        fixture
    }
    pub fn draws(&self) -> Result<Vec<PrimitiveDraw>, RenderError> {
        self.group
            .primitive_draws_for_view(&Graphic3dCamera::default(), [64, 64])
    }
}
fn configure_group(
    group: &Handle<Graphic3dDrawGroup>,
    top_down: bool,
    colors: bool,
    normals: bool,
) -> (Handle<Graphic3dTexture2D>, Handle<Graphic3dAspects>) {
    let texture = Handle::new(Graphic3dTexture2D::from_pix_map(texture_pixels(top_down)));
    let aspect = mapped_aspect(&texture);
    group.set_group_primitives_aspect(&aspect.cast());
    group.add_primitive_array(&texture_array(colors, normals), true);
    (texture, aspect)
}
fn mapped_aspect(texture: &Handle<Graphic3dTexture2D>) -> Handle<Graphic3dAspects> {
    let aspect = Handle::new(Graphic3dAspects::default());
    aspect.set_texture_map(texture.cast());
    aspect.set_texture_map_on(true);
    aspect
}
fn texture_pixels(top_down: bool) -> Handle<ImagePixMap> {
    let image = Handle::new(ImagePixMap::default());
    let rgba = vec![
        255, 0, 0, 255, 0, 255, 0, 128, 0, 0, 255, 255, 255, 255, 0, 0,
    ];
    assert!(image.init_wrapper(ImageFormat::Rgba, Some(rgba), 2, 2, 8));
    image.set_top_down(top_down);
    image
}
fn texture_array(colors: bool, normals: bool) -> Handle<Graphic3dArrayOfPrimitives> {
    let flags = texture_flags(colors, normals);
    let points = Handle::new(
        Graphic3dArrayOfPrimitives::new(Graphic3dTypeOfPrimitiveArray::Points, 2, 0, 0, flags)
            .unwrap(),
    );
    for (x, u) in [(-0.5, 0.25), (0.5, 0.75)] {
        let index = points.add_vertex(x, 0., 0.).unwrap();
        configure_texture_vertex(&points, index, u, colors, normals);
    }
    points
}
fn configure_texture_vertex(
    points: &Graphic3dArrayOfPrimitives,
    index: i32,
    u: f64,
    colors: bool,
    normals: bool,
) {
    if normals {
        points.set_vertex_normal(index, 0., 0., 1.).unwrap();
    }
    if colors {
        let red = f64::from(index - 1);
        points
            .set_vertex_color_rgb(index, red, 1. - red, 1.)
            .unwrap();
    }
    points.set_vertex_texel(index, u, 0.25).unwrap();
}
fn texture_flags(colors: bool, normals: bool) -> i32 {
    (if normals {
        GRAPHIC3D_ARRAY_FLAGS_VERTEX_NORMAL
    } else {
        0
    }) | GRAPHIC3D_ARRAY_FLAGS_VERTEX_TEXEL
        | if colors {
            GRAPHIC3D_ARRAY_FLAGS_VERTEX_COLOR
        } else {
            0
        }
}
pub(crate) fn point_draw(model: Graphic3dTypeOfShadingModel, size: f32) -> PrimitiveDraw {
    let fixture = PointTextureFixture::new(true, false);
    fixture.aspect.set_shading_model(model);
    fixture.aspect.set_marker_scale(size).unwrap();
    fixture.draws().unwrap().remove(0)
}
