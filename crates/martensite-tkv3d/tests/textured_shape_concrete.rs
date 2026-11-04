//! Concrete ownership and genuine interface dispatch regressions, and the inherited selection/HLR.
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::{Handle, RuntimeTyped};
use martensite_tkservice::image::ImagePixMap;
use martensite_tkv3d::{ais::*, prs_mgr::*, select_mgr::*};

#[test]
fn textured_concrete_casts_share_native_descriptor_and_object() {
    let object = AisTexturedShape::new(&Shape::default());
    let source = object.clone().cast::<dyn AisTexturedShapeSource>();
    assert_eq!(source.object_address(), object.object_address());
    assert!(std::ptr::eq(
        <dyn AisTexturedShapeSource>::static_type(),
        AisTexturedShape::static_type()
    ));
    assert_eq!(
        object.dynamic_type().unwrap().parent().unwrap().name(),
        "AisShape"
    );
    assert!(object.is_kind(<dyn AisShapeSource>::static_type()));
    assert_eq!(source.kind_of_interactive(), AisKindOfInteractive::Shape);
    assert_eq!(source.signature(), 0);
}

#[test]
fn textured_concrete_native_parent_alias_retains_real_object_and_descriptor() {
    let object = AisTexturedShape::new(&Shape::default());
    let parent = object.clone().cast::<AisTexturedShapeParent>();
    assert_eq!(parent.object_address(), object.object_address());
    assert!(std::ptr::eq(
        AisTexturedShapeParent::static_type(),
        object.dynamic_type().unwrap().parent().unwrap()
    ));
}

#[test]
fn textured_concrete_virtual_source_retains_actual_pixmap_field() {
    let object = AisTexturedShape::new(&Shape::default());
    let source = object.clone().cast::<dyn AisTexturedShapeSource>();
    let pixels = Handle::new(ImagePixMap::default());
    let retained = source.texture_pix_map();
    source.set_texture_pix_map(&pixels);
    assert_eq!(retained.read().object_address(), pixels.object_address());
    source.set_texture_file_name(&"texture.png".into()).unwrap();
    assert!(retained.read().is_null());
    assert_eq!(
        source.texture_file().read().unwrap().as_bytes(),
        b"texture.png"
    );
}

#[test]
fn textured_concrete_replays_native_rtti_row() {
    let object = AisTexturedShape::new(&Shape::default());
    let row = format!(
        "MARTENSITE_RTTI {} {} {}",
        object.dynamic_type().unwrap().name(),
        i32::from(object.is_kind(<dyn AisShapeSource>::static_type())),
        i32::from(object.is_kind(<dyn AisInteractiveObject>::static_type()))
    );
    let native = include_str!("fixtures/ais_textured_shape/native-state.txt");
    assert!(native.lines().any(|line| line == row));
}

#[test]
fn textured_concrete_null_compute_accepts_modes_without_fake_geometry() {
    let object = AisTexturedShape::new(&Shape::default());
    let source = object.cast::<dyn PrsMgrPresentableObject>();
    for mode in -1..=4 {
        assert_eq!(source.accept_display_mode(mode), (0..=3).contains(&mode));
        source
            .compute(&Handle::default(), &Handle::default(), mode)
            .unwrap();
    }
}

#[test]
fn textured_concrete_null_shape_selection_is_empty() {
    // The inherited AisShape::ComputeSelection returns at once for a null shape.
    let object = AisTexturedShape::new(&Shape::default());
    let selection = object.clone().cast::<dyn SelectMgrSelectableObject>();
    selection.compute_selection(&Handle::default(), 4).unwrap();
}

#[test]
fn textured_concrete_null_shape_hlr_draws_nothing() {
    // The inherited AisShape::computeHlrPresentation returns at once for a null shape.
    let object = AisTexturedShape::new(&Shape::default());
    let presentable = object.cast::<dyn PrsMgrPresentableObject>();
    presentable
        .compute_hlr(&Handle::default(), &Handle::default(), &Handle::default())
        .unwrap();
}

fn textured_box() -> Shape {
    let mut builder =
        martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox::from_sizes(2., 3., 4.).unwrap();
    builder.solid().unwrap().into_shape()
}
fn selection_signature(object: &dyn SelectMgrSelectableObject, mode: i32) -> Vec<String> {
    let selection = Handle::new(SelectMgrSelection::new(mode));
    object.compute_selection(&selection, mode).unwrap();
    selection
        .entities()
        .iter()
        .map(|entity| {
            let sensitive = entity.base_sensitive();
            let kind = sensitive
                .dynamic_type()
                .map_or("<null>", |kind| kind.name());
            format!("{kind}/{}", sensitive.nb_sub_elements())
        })
        .collect()
}

#[test]
fn textured_inherits_the_shape_selection_on_a_real_box() {
    // AisTexturedShape overrides neither ComputeSelection nor its helpers: every mode equals AisShape.
    let shape = textured_box();
    let textured = AisTexturedShape::new(&shape).cast::<dyn SelectMgrSelectableObject>();
    let plain = AisShape::new(&shape).cast::<dyn SelectMgrSelectableObject>();
    for mode in 0..=8 {
        let expected = selection_signature(plain.get().unwrap(), mode);
        assert_eq!(
            selection_signature(textured.get().unwrap(), mode),
            expected,
            "mode={mode}"
        );
    }
    assert_eq!(selection_signature(textured.get().unwrap(), 4).len(), 6);
}

#[test]
fn textured_inherits_the_shape_hlr_on_a_real_box() {
    // The inherited computeHlrPresentation draws the same hidden-line groups as AisShape.
    use martensite_tkservice::{graphic3d::*, null_driver::NullGraphicDriver};
    let viewer = martensite_tkv3d::v3d::V3dViewer::new(&NullGraphicDriver::new_handle()).unwrap();
    let camera = Handle::new(Graphic3dCamera::default());
    let link = Handle::new(martensite_tkv3d::prs3d::Prs3dDrawer::new());
    link.setup_own_defaults().unwrap();
    let groups = |object: Handle<dyn PrsMgrPresentableObject>| {
        // A context links its default drawer; HLR reads its deviation settings natively.
        object.attributes().set_link(&link);
        let prs = Graphic3dStructure::new(&viewer.structure_manager(), &Handle::default()).unwrap();
        object
            .compute_hlr(&camera, &Handle::default(), &prs.clone().cast())
            .unwrap();
        prs.groups().len()
    };
    let shape = textured_box();
    let expected = groups(AisShape::new(&shape).cast());
    assert!(
        expected > 0,
        "AisShape HLR groups={expected}; expected drawn hidden lines"
    );
    assert_eq!(groups(AisTexturedShape::new(&shape).cast()), expected);
}
