//! The retained triangle route preserves real source metadata without invoking a GPU.
use crate::*;
use martensite_tkernel::standard::Handle;
use martensite_tkservice::graphic3d::*;
struct VertexOutputNeverRenderBackend;
impl RenderBackend for VertexOutputNeverRenderBackend {
    fn render(&mut self, _: &RenderFrame) -> Result<FrameCapture, RenderError> {
        Err(crate::error::rejected(
            "unexpected device call",
            "CPU retained selection only",
        ))
    }
}
pub(crate) struct RetainedPhongFixture {
    _driver: Handle<Graphic3dRenderDriver>,
    _structure: Handle<Graphic3dStructure>,
    pub group: Handle<Graphic3dDrawGroup>,
}
impl RetainedPhongFixture {
    pub fn new() -> Self {
        let driver =
            Graphic3dRenderDriver::with_backend(VertexOutputNeverRenderBackend, 64).unwrap();
        let manager = Handle::new(Graphic3dStructureManager::new(&driver.cast()).unwrap());
        let structure = Graphic3dStructure::new(&manager, &Handle::default()).unwrap();
        let group = Graphic3dDrawGroup::new(&structure.cast());
        let style = Handle::new(Graphic3dAspects::default());
        style.set_shading_model(Graphic3dTypeOfShadingModel::Phong);
        group.set_group_primitives_aspect(&style.cast());
        group.add_primitive_array(&retained_triangle_array(), false);
        Self {
            _driver: driver,
            _structure: structure,
            group,
        }
    }
    pub fn draw(&self) -> PrimitiveDraw {
        let mut draws = self
            .group
            .primitive_draws(&Graphic3dCamera::default())
            .unwrap();
        assert_eq!(draws.len(), 1);
        draws.remove(0)
    }
}
fn retained_triangle_array() -> Handle<Graphic3dArrayOfPrimitives> {
    let flags = GRAPHIC3D_ARRAY_FLAGS_VERTEX_NORMAL | GRAPHIC3D_ARRAY_FLAGS_VERTEX_TEXEL;
    let array = Handle::new(
        Graphic3dArrayOfPrimitives::new(Graphic3dTypeOfPrimitiveArray::Triangles, 3, 0, 3, flags)
            .unwrap(),
    );
    for [x, y] in [[0., 0.], [1., 0.], [0., 1.]] {
        array
            .add_vertex_normal_texel([x, y, 0.], [0., 0., 1.], [0., 0.])
            .unwrap();
    }
    array.add_edges3(1, 2, 3).unwrap();
    array
}
