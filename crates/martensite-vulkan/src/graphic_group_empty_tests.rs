//! Native early EMPTY handling leaves no draw, upload or sticky-width effect.
use crate::{Graphic3dDrawGroup, RenderFrame};
use martensite_tkernel::standard::Handle;
use martensite_tkservice::{aspect::*, graphic3d::*};

struct EmptyLineScene {
    _structure: Handle<Graphic3dStructure>,
    _driver: Handle<crate::Graphic3dRenderDriver>,
    group: Handle<Graphic3dDrawGroup>,
    camera: Graphic3dCamera,
}
impl EmptyLineScene {
    fn new(kind: Graphic3dTypeOfPrimitiveArray) -> Self {
        let driver = empty_test_driver();
        let manager = Handle::new(Graphic3dStructureManager::new(&driver.cast()).unwrap());
        let structure = Graphic3dStructure::new(&manager, &Handle::default()).unwrap();
        let group = Graphic3dDrawGroup::new(&structure.cast());
        let aspect = Handle::new(Graphic3dAspects::default());
        aspect.set_line_type(AspectTypeOfLine::Empty);
        group.set_group_primitives_aspect(&aspect.cast());
        structure.set_visible(true);
        group.add_primitive_array(&empty_test_array(kind), true);
        Self {
            _structure: structure,
            _driver: driver,
            group,
            camera: Graphic3dCamera::default(),
        }
    }
    fn draws(&self) -> Result<Vec<crate::PrimitiveDraw>, crate::RenderError> {
        self.group.primitive_draws_for_view(&self.camera, [64, 64])
    }
}
struct EmptyNeverRenderBackend;
impl crate::RenderBackend for EmptyNeverRenderBackend {
    fn render(&mut self, _frame: &RenderFrame) -> Result<crate::FrameCapture, crate::RenderError> {
        panic!("unexpected rendering; expected CPU-only retained extraction")
    }
}
fn empty_test_driver() -> Handle<crate::Graphic3dRenderDriver> {
    let policies = crate::RenderPipelinePolicies {
        mesh_edges: crate::RenderMeshEdgePipeline::PolygonCompatibility,
        ..Default::default()
    };
    crate::Graphic3dRenderDriver::with_backend_and_line_width_configuration(
        EmptyNeverRenderBackend,
        64,
        std::sync::Arc::new(crate::FreeTypeTextSource::default()),
        policies,
        Default::default(),
    )
    .unwrap()
}

fn empty_test_array(kind: Graphic3dTypeOfPrimitiveArray) -> Handle<Graphic3dArrayOfPrimitives> {
    let count = if kind == Graphic3dTypeOfPrimitiveArray::Triangles {
        3
    } else {
        4
    };
    let array = Handle::new(Graphic3dArrayOfPrimitives::new(kind, count, 0, 0, 0).unwrap());
    for [x, y, z] in [
        [-0.5, -0.5, 0.],
        [0.5, -0.5, 0.],
        [0.5, 0.5, 0.],
        [-0.5, 0.5, 0.],
    ][..count as usize]
        .iter()
        .copied()
    {
        array.add_vertex(x, y, z).unwrap();
    }
    array
}

#[test]
fn every_empty_line_topology_is_elided_and_live_style_restores_it() {
    use Graphic3dTypeOfPrimitiveArray as K;
    for kind in [
        K::Segments,
        K::Polylines,
        K::LinesAdjacency,
        K::LineStripAdjacency,
    ] {
        let scene = EmptyLineScene::new(kind);
        assert!(
            scene.draws().unwrap().is_empty(),
            "EMPTY {kind:?} emitted a draw"
        );
        scene.group.aspects().set_line_type(AspectTypeOfLine::Solid);
        assert!(
            !scene.draws().unwrap().is_empty(),
            "solid {kind:?} lost retained geometry"
        );
        scene.group.aspects().set_line_type(AspectTypeOfLine::Empty);
        assert!(scene.draws().unwrap().is_empty());
    }
}
#[test]
fn empty_lines_skip_shader_resolution_before_upload() {
    let scene = EmptyLineScene::new(Graphic3dTypeOfPrimitiveArray::Segments);
    scene
        .group
        .aspects()
        .set_shader_program(Handle::new(Graphic3dShaderProgram::default()));
    assert!(scene.draws().unwrap().is_empty());
    scene.group.aspects().set_line_type(AspectTypeOfLine::Solid);
    assert!(
        scene.draws().is_err(),
        "visible unsupported shader must still refuse"
    );
}
#[test]
fn empty_line_type_does_not_hide_fills_or_points() {
    for kind in [
        Graphic3dTypeOfPrimitiveArray::Triangles,
        Graphic3dTypeOfPrimitiveArray::Points,
    ] {
        let scene = EmptyLineScene::new(kind);
        assert!(!scene.draws().unwrap().is_empty(), "line type hid {kind:?}");
    }
}
#[test]
fn empty_element_keeps_native_element_counters_without_uploaded_geometry() {
    let scene = EmptyLineScene::new(Graphic3dTypeOfPrimitiveArray::Segments);
    assert!(scene.draws().unwrap().is_empty());
    let mut stats = Graphic3dFrameStatsDataTmp::default();
    for element in scene.group.element_stats() {
        element.update_mem_stats(&mut stats);
        element.update_draw_stats(&mut stats, true);
    }
    use Graphic3dFrameStatsCounter as C;
    assert_eq!(*stats.change_counter_value(C::EstimatedBytesGeom), 0);
    assert_eq!(*stats.change_counter_value(C::NbElemsLineNotCulled), 1);
    assert_eq!(*stats.change_counter_value(C::NbLinesNotCulled), 0);
}
#[test]
fn hidden_wide_line_cannot_change_later_hollow_polygon_width() {
    let scene = EmptyLineScene::new(Graphic3dTypeOfPrimitiveArray::Segments);
    scene.group.aspects().set_line_width(7.).unwrap();
    let hollow = Handle::new(Graphic3dAspects::default());
    hollow.set_interior_style(AspectInteriorStyle::Empty);
    hollow.set_draw_edges(true);
    scene.group.set_primitives_aspect(&hollow.cast());
    scene.group.add_primitive_array(
        &empty_test_array(Graphic3dTypeOfPrimitiveArray::Triangles),
        true,
    );
    let frame = RenderFrame::new(64, 64, [0.; 4], scene.draws().unwrap()).unwrap();
    let width = frame
        .draws()
        .last()
        .unwrap()
        .polygon_edge_style
        .unwrap()
        .raster_width();
    assert_eq!(
        width, 1.,
        "hidden EMPTY width changed native sticky line state"
    );
    assert_eq!(frame.draws().len(), 1);
}
