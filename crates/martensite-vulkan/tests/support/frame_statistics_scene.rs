//! The recorded `scene_mixed` statistics scene: the same Graphic3d arrays as its nine
//! primitive-array objects (position-only vertices, 16-bit indices) and one 16 px text label.
use super::scene::{ViewerScene, unlit_style};
use Graphic3dTypeOfPrimitiveArray as Kind;
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkernel::standard::Handle;
use martensite_tkmath::gp::Point3;
use martensite_tkservice::{aspect::*, graphic3d::*};
use martensite_tkv3d::v3d::V3dViewSource;
use martensite_vulkan::{Graphic3dRenderDriver, Graphic3dRenderView};

/// One array of the scene: type, bound sizes, vertices and 1-based edges.
struct ArraySpec {
    kind: Kind,
    bounds: &'static [i32],
    vertices: &'static [[f32; 2]],
    edges: &'static [i32],
}

const MIXED_ARRAYS: [ArraySpec; 9] = [
    spec(
        Kind::Triangles,
        &[],
        &[[0., 0.], [1., 0.], [1., 1.], [0., 1.]],
        &[1, 2, 3, 1, 3, 4],
    ),
    spec(
        Kind::Trianglestrips,
        &[4, 3],
        &[
            [2., 0.],
            [3., 0.],
            [2., 1.],
            [3., 1.],
            [4., 0.],
            [5., 0.],
            [4., 1.],
        ],
        &[],
    ),
    spec(
        Kind::Trianglefans,
        &[],
        &[[0., 2.], [1., 2.], [1., 3.], [0.5, 3.5], [0., 3.]],
        &[],
    ),
    spec(
        Kind::Segments,
        &[],
        &[[2., 2.], [3., 2.], [2., 3.], [3., 3.]],
        &[],
    ),
    spec(
        Kind::Polylines,
        &[3, 2],
        &[[4., 2.], [5., 2.], [5., 3.], [4., 3.], [4.5, 3.5]],
        &[],
    ),
    spec(
        Kind::Points,
        &[],
        &[[0., 4.], [1., 4.], [2., 4.], [3., 4.], [4., 4.]],
        &[],
    ),
    spec(
        Kind::Quadrangles,
        &[],
        &[
            [0., 5.],
            [1., 5.],
            [1., 6.],
            [0., 6.],
            [2., 5.],
            [3., 5.],
            [3., 6.],
            [2., 6.],
        ],
        &[],
    ),
    spec(
        Kind::Quadranglestrips,
        &[],
        &[[4., 5.], [4., 6.], [5., 5.], [5., 6.], [6., 5.], [6., 6.]],
        &[],
    ),
    spec(
        Kind::Polygons,
        &[],
        &[[0., 7.], [1., 7.], [1.5, 7.5], [1., 8.], [0., 8.]],
        &[],
    ),
];

const fn spec(
    kind: Kind,
    bounds: &'static [i32],
    vertices: &'static [[f32; 2]],
    edges: &'static [i32],
) -> ArraySpec {
    ArraySpec {
        kind,
        bounds,
        vertices,
        edges,
    }
}

pub struct StatisticsScene {
    pub scene: ViewerScene,
    pub view: Handle<dyn V3dViewSource>,
    pub text: Handle<Graphic3dStructure>,
}

impl StatisticsScene {
    /// The native scene_mixed objects, each in its own displayed structure.
    pub fn mixed(driver: &Handle<Graphic3dRenderDriver>) -> Self {
        let scene = ViewerScene::new(driver);
        scene.structure.erase();
        for array in &MIXED_ARRAYS {
            display_array(&scene, array);
        }
        let text = display_text(&scene, "Hi there");
        let view = scene.view.clone();
        Self { scene, view, text }
    }
    pub fn render_view(&self) -> Handle<Graphic3dRenderView> {
        self.scene.render_view()
    }
    /// Erase the label from this view: it stays displayed, but not in this view.
    pub fn hide_text(&self) {
        let view_id = self.scene.cview().identification();
        let cstructure = self.text.cstructure();
        let mut fields = cstructure.cstructure_base().fields.write();
        if fields.view_affinity.is_null() {
            fields.view_affinity = Handle::new(Graphic3dViewAffinity::default());
        }
        fields.view_affinity.set_view_visible(view_id, false);
    }
}

/// Collect `flags` and update the counters on every frame (interval -1).
pub fn collect_every_frame(
    view: &Handle<dyn V3dViewSource>,
    flags: Graphic3dRenderingParamsPerfCounters,
) {
    view.change_rendering_params(&mut |params| {
        params.stats.collected_stats = flags;
        params.stats.stats_update_interval = -1.0;
    });
}

fn new_structure(scene: &ViewerScene) -> Handle<Graphic3dStructure> {
    Graphic3dStructure::new(&scene.viewer.structure_manager(), &Handle::default()).unwrap()
}

fn display_array(scene: &ViewerScene, array: &ArraySpec) {
    let structure = new_structure(scene);
    let group = structure.new_group().unwrap();
    group.set_group_primitives_aspect(&unlit_style([1., 1., 0., 1.]));
    group.add_primitive_array(&primitive_array(array), true);
    structure.display();
}

fn primitive_array(spec: &ArraySpec) -> Handle<Graphic3dArrayOfPrimitives> {
    let (vertices, bounds, edges) = (spec.vertices.len(), spec.bounds.len(), spec.edges.len());
    let array =
        Graphic3dArrayOfPrimitives::new(spec.kind, vertices as i32, bounds as i32, edges as i32, 0)
            .unwrap();
    for bound in spec.bounds {
        array.add_bound(*bound).unwrap();
    }
    for [x, y] in spec.vertices {
        array.add_vertex(*x, *y, 0.).unwrap();
    }
    for edge in spec.edges {
        array.add_edge(*edge).unwrap();
    }
    Handle::new(array)
}

fn display_text(scene: &ViewerScene, value: &str) -> Handle<Graphic3dStructure> {
    let structure = new_structure(scene);
    let group = structure.new_group().unwrap();
    let style = Handle::new(Graphic3dAspectText3d::new(
        Color::named(NamedColor::YELLOW),
        Some("DejaVu Sans"),
        1.,
        0.,
        AspectTypeOfStyleText::Normal,
        AspectTypeOfDisplayText::Normal,
    ));
    let text = Handle::new(Graphic3dText::new(16.));
    text.set_text_str(value);
    text.set_position(&Point3::new(3., 7., 0.));
    group.set_group_primitives_aspect(&style.cast());
    group.add_text(&text, true);
    structure.display();
    structure
}
