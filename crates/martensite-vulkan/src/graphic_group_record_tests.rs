//! Group draw records: replay with a new camera equals lowering again, and every recorded
//! input invalidates the record when it changes.
use crate::Graphic3dDrawGroup;
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkernel::standard::Handle;
use martensite_tkmath::gp::Point3;
use martensite_tkservice::graphic3d::*;

struct RecordScene {
    structure: Handle<Graphic3dStructure>,
    _driver: Handle<crate::Graphic3dRenderDriver>,
    group: Handle<Graphic3dDrawGroup>,
    aspect: Handle<Graphic3dAspects>,
    camera: Graphic3dCamera,
    /// A view keeps its parameters; new ones would have new statistics handles.
    rendering: Graphic3dRenderingParams,
}

impl RecordScene {
    fn new() -> Self {
        let driver = record_test_driver();
        let manager = Handle::new(Graphic3dStructureManager::new(&driver.cast()).unwrap());
        let structure = Graphic3dStructure::new(&manager, &Handle::default()).unwrap();
        let group = structure.new_group().unwrap().cast::<Graphic3dDrawGroup>();
        let aspect = Handle::new(Graphic3dAspects::default());
        aspect.set_interior_color(Color::named(NamedColor::RED));
        group.set_group_primitives_aspect(&aspect.clone().cast());
        structure.set_visible(true);
        group.add_primitive_array(&triangle().upcast(), true);
        Self {
            structure,
            _driver: driver,
            group,
            aspect,
            camera: Graphic3dCamera::default(),
            rendering: Graphic3dRenderingParams::new().unwrap(),
        }
    }
    fn draws(&self) -> Vec<crate::PrimitiveDraw> {
        let cstructure = self.structure.cstructure();
        let fields = cstructure.cstructure_base().fields.read();
        self.group
            .draws_in_structure(
                &self.camera,
                Some([64, 64]),
                &fields,
                &self.rendering,
                &Handle::default(),
            )
            .unwrap()
    }
    /// The draws of one lowering in `pass`.
    fn draws_in(
        &self,
        pass: &crate::render_element_pass::RenderElementPass,
    ) -> Vec<crate::PrimitiveDraw> {
        let cstructure = self.structure.cstructure();
        let fields = cstructure.cstructure_base().fields.read();
        let clipping = Handle::default();
        self.group
            .program_in_structure(
                &self.camera,
                Some([64, 64]),
                &fields,
                &self.rendering,
                &clipping,
                (pass, &mut Default::default()),
            )
            .unwrap()
            .draws
    }
    fn translate(&self, x: f64) {
        let mut trsf = martensite_tkmath::gp::Transform3::default();
        trsf.set_translation(martensite_tkmath::gp::Vector3::new(x, 0., 0.));
        let datum = Handle::new(martensite_tkmath::toploc::Datum3::new(trsf));
        let cstructure = self.structure.cstructure();
        cstructure.cstructure_base().fields.write().trsf = datum;
    }
    /// Lower twice, so the second pass keeps a record; the first only notes its pass.
    fn recorded(&self) -> Vec<crate::PrimitiveDraw> {
        self.draws();
        self.draws()
    }
    /// The structure-level replay of all groups, or `None` when it takes the lowering path.
    fn structure_replay(&self) -> Option<Vec<crate::PrimitiveDraw>> {
        let render = self
            .structure
            .cstructure()
            .cast::<crate::Graphic3dRenderStructure>();
        let pass = crate::render_element_pass::RenderElementPass::unfiltered();
        let clipping = Handle::default();
        let scene = (&self.camera, [64, 64], &self.rendering, &clipping, &pass);
        render
            .get()?
            .replayed_groups(scene)
            .map(|(program, _)| program.draws)
    }
    /// The draws a group without a record lowers for the current camera.
    fn lowered_again(&self) -> Vec<crate::PrimitiveDraw> {
        *self.group.draw_record.write() = super::RecordSlot::Empty;
        self.draws()
    }
    /// How often the current record replayed; a rebuilt record starts again at 0.
    fn replays(&self) -> Option<usize> {
        let slot = self.group.draw_record.read();
        slot.record()
            .map(|record| record.replays.load(std::sync::atomic::Ordering::Relaxed))
    }
}

struct RecordNeverRenderBackend;
impl crate::RenderBackend for RecordNeverRenderBackend {
    fn render(
        &mut self,
        _: &crate::RenderFrame,
    ) -> Result<crate::FrameCapture, crate::RenderError> {
        panic!("unexpected rendering; expected CPU-only group lowering")
    }
}

fn record_test_driver() -> Handle<crate::Graphic3dRenderDriver> {
    crate::Graphic3dRenderDriver::with_backend_and_line_width_configuration(
        RecordNeverRenderBackend,
        64,
        std::sync::Arc::new(crate::FreeTypeTextSource::default()),
        Default::default(),
        Default::default(),
    )
    .unwrap()
}

fn triangle() -> Handle<Graphic3dArrayOfPrimitives> {
    let kind = Graphic3dTypeOfPrimitiveArray::Triangles;
    let array = Handle::new(Graphic3dArrayOfPrimitives::new(kind, 3, 0, 0, 0).unwrap());
    for [x, y, z] in [[-0.5, -0.5, 0.], [0.5, -0.5, 0.], [0., 0.5, 0.]] {
        array.add_vertex(x, y, z).unwrap();
    }
    array
}

#[test]
fn a_replay_for_a_moved_camera_equals_lowering_again() {
    let scene = RecordScene::new();
    let first = scene.recorded();
    assert_eq!(
        scene.replays(),
        Some(0),
        "an ordinary surface group keeps a record"
    );
    scene.camera.set_eye(&Point3::new(3., -2., 5.)).unwrap();
    let replayed = scene.draws();
    assert_eq!(scene.replays(), Some(1), "the record replayed, not rebuilt");
    assert_ne!(replayed[0].clip_transform, first[0].clip_transform);
    assert_eq!(replayed, scene.lowered_again());
}

#[test]
fn an_aspects_write_rebuilds_the_record() {
    let scene = RecordScene::new();
    let before = scene.recorded();
    scene.draws();
    assert_eq!(scene.replays(), Some(1));
    scene
        .aspect
        .set_interior_color(Color::named(NamedColor::GREEN));
    let after = scene.draws();
    assert_eq!(scene.replays(), Some(0), "the aspects revision changed");
    assert_ne!(after, before);
    assert_eq!(after, scene.lowered_again());
}

#[test]
fn a_structure_transformation_rebuilds_the_record() {
    let scene = RecordScene::new();
    scene.recorded();
    scene.draws();
    assert_eq!(scene.replays(), Some(1));
    let mut trsf = martensite_tkmath::gp::Transform3::default();
    trsf.set_translation(martensite_tkmath::gp::Vector3::new(1., 2., 3.));
    let datum = Handle::new(martensite_tkmath::toploc::Datum3::new(trsf));
    let cstructure = scene.structure.cstructure();
    cstructure.cstructure_base().fields.write().trsf = datum;
    let moved = scene.draws();
    assert_eq!(scene.replays(), Some(0), "the model matrix changed");
    assert_eq!(moved, scene.lowered_again());
}

#[test]
fn a_highlighted_structure_keeps_no_record() {
    let scene = RecordScene::new();
    let cstructure = scene.structure.cstructure();
    cstructure.cstructure_base().fields.write().highlight = true;
    scene.recorded();
    assert_eq!(scene.replays(), None);
}

#[test]
fn a_text_group_keeps_no_record() {
    let scene = RecordScene::new();
    let text = Handle::new(Graphic3dText::new(10.));
    text.set_text(&martensite_tkernel::ncollection::String8::from_unicode_units(b"label", -1));
    scene.group.add_text(&text, true);
    scene.recorded();
    assert_eq!(scene.replays(), None);
}

#[test]
fn a_group_lowered_by_one_pass_keeps_no_record() {
    let scene = RecordScene::new();
    scene.draws();
    assert_eq!(
        scene.replays(),
        None,
        "scenes drawn once keep no copies of their draws"
    );
    assert!(matches!(
        *scene.group.draw_record.read(),
        super::RecordSlot::Lowered(_)
    ));
}

#[test]
fn a_group_lowered_twice_in_one_pass_drops_its_record_for_good() {
    use crate::render_element_pass::RenderElementPass;
    let scene = RecordScene::new();
    scene.recorded();
    let pass = RenderElementPass::unfiltered();
    scene.draws_in(&pass);
    assert_eq!(scene.replays(), Some(1));
    scene.translate(5.);
    let instance = scene.draws_in(&pass);
    assert_eq!(
        scene.replays(),
        None,
        "an instanced group keeps no copy of its draws"
    );
    assert!(matches!(
        *scene.group.draw_record.read(),
        super::RecordSlot::Instanced
    ));
    scene.draws();
    scene.draws();
    assert_eq!(scene.replays(), None, "later passes do not record it again");
    assert_eq!(instance, scene.lowered_again());
}

#[test]
fn lowering_a_group_again_shares_the_bodies_of_the_first_lowering() {
    let scene = RecordScene::new();
    let first = scene.lowered_again();
    let second = scene.lowered_again();
    assert_eq!(first.len(), second.len());
    // A connected presentation lowers one group for every instance, every frame; a copied
    // body per instance cost 5% more peak memory in frame_connected_30k (#594).
    for (earlier, later) in first.iter().zip(&second) {
        assert!(std::sync::Arc::ptr_eq(&earlier.body, &later.body));
    }
}

#[test]
fn a_pass_reads_the_surface_policies_of_each_structure_manager() {
    let programmable = RecordScene::new();
    let fixed = RecordScene::new();
    fixed
        ._driver
        .set_lighting_pipeline(crate::RenderLightingPipeline::FixedFunction)
        .unwrap();
    let frame = crate::group_record_frame::RecordFrame::default();
    for scene in [&programmable, &fixed, &programmable, &fixed] {
        let structure = scene.group.structure();
        assert_eq!(
            frame.surface_policies(&structure),
            crate::lighting_pipeline::structure_surface_policies(&structure),
        );
    }
}

#[test]
fn the_labels_of_a_group_share_its_transform_but_keep_their_own_anchor() {
    let scene = RecordScene::new();
    for (index, x) in [0., 0.5].into_iter().enumerate() {
        let text = Handle::new(Graphic3dText::new(10.));
        text.set_text_str(&format!("label {index}"));
        text.set_position(&Point3::new(x, 0., 0.));
        scene.group.add_text(&text, true);
    }
    let labels = |draws: Vec<crate::PrimitiveDraw>| -> Vec<[u32; 16]> {
        draws
            .iter()
            .filter(|draw| draw.textured_style.is_some())
            .map(|draw| draw.clip_transform.map(f32::to_bits))
            .collect()
    };
    let first = labels(scene.draws());
    assert_eq!(first.len(), 2);
    assert_ne!(first[0], first[1], "each label keeps its anchor");
    // The second frame replays the kept labels with the camera of this frame.
    assert_eq!(labels(scene.draws()), first);
}

#[test]
fn a_structure_whose_groups_have_records_replays_them_as_lowering_would() {
    let scene = RecordScene::new();
    assert!(scene.structure_replay().is_none(), "no record yet");
    scene.recorded();
    scene.camera.set_eye(&Point3::new(3., -2., 5.)).unwrap();
    let replayed = scene.structure_replay().expect("every group has a record");
    assert_eq!(replayed, scene.lowered_again());
}

#[test]
fn a_highlighted_or_stale_structure_takes_the_lowering_path() {
    let scene = RecordScene::new();
    scene.recorded();
    scene
        .aspect
        .set_interior_color(Color::named(NamedColor::BLUE));
    assert!(scene.structure_replay().is_none(), "an aspects write");
    scene.recorded();
    let cstructure = scene.structure.cstructure();
    cstructure.cstructure_base().fields.write().highlight = true;
    assert!(scene.structure_replay().is_none(), "a highlight");
}
