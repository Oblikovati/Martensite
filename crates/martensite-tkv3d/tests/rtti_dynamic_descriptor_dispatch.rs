//! DynamicType on real objects: the concrete borrowed slot and owner-aware projected handles.
//!
//! Rows equal the recorded tkv3d_dynamic_descriptor_dispatch capture (three native processes).
//! A concrete receiver uses `Transient::dynamic_type_ref`; a projected handle (to
//! `StandardTransient` or to the parent class) uses `Handle::dynamic_type_handle_ref`, which
//! dispatches through the original owner as the native virtual call does.
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkernel::standard::{
    Handle, RuntimeType, RuntimeTyped, StandardTransient, Transient,
};
use martensite_tkservice::aspect::AspectTypeOfLine;
use martensite_tkservice::graphic3d::Graphic3dPresentationAttributesSource;
use martensite_tkv3d::ais::*;
use martensite_tkv3d::prs_dim::{PrsDimDimensionSource, PrsDimLengthDimension};
use martensite_tkv3d::prs3d::*;
use martensite_tkv3d::select_mgr::*;
use martensite_tkv3d::std_prs::*;
use martensite_tkv3d::v3d::V3dTrihedron;

const NATIVE: &str = include_str!("fixtures/rtti-dynamic-descriptor-dispatch-native.txt");

/// The concrete row: name, own canonical slot, borrow/copy/release reference deltas.
fn concrete<T: RuntimeTyped + Transient>(label: &str, object: &Handle<T>) -> String {
    let slot = T::static_type_ref();
    let initial = slot.ref_count();
    let receiver = object
        .get()
        .expect("object=null; expected a constructed owner");
    let borrowed = <T as Transient>::dynamic_type_ref(receiver);
    let borrow = slot.ref_count() - initial;
    let copy = borrowed.clone();
    let owned = slot.ref_count() - initial;
    drop(copy);
    let same = i32::from(std::ptr::eq(borrowed, slot));
    let restored = slot.ref_count() - initial;
    format!(
        "{label}|concrete|{}|{same}|{borrow}|{owned}|{restored}",
        borrowed.name()
    )
}

/// A projected row: the owner-aware borrowed descriptor of a base-typed handle.
fn projected<T: RuntimeTyped, B: ?Sized + RuntimeTyped>(
    label: &str,
    role: &str,
    object: &Handle<T>,
) -> String {
    let base: Handle<B> = object.cast();
    assert!(
        !base.is_null(),
        "{label} as {role}; expected a projected handle"
    );
    let descriptor: &'static Handle<RuntimeType> = base
        .dynamic_type_handle_ref()
        .expect("projected handle=null; expected the owner descriptor");
    let same = i32::from(std::ptr::eq(descriptor, T::static_type_ref()));
    format!("{label}|{role}|{}|{same}", descriptor.name())
}

fn observe<T, P>(label: &str, parent: &str, object: Handle<T>) -> Vec<String>
where
    T: RuntimeTyped + Transient,
    P: ?Sized + RuntimeTyped,
{
    vec![
        concrete(label, &object),
        projected::<T, StandardTransient>(label, "transient", &object),
        projected::<T, P>(label, &format!("parent:{parent}"), &object),
    ]
}

fn iso_aspect() -> Prs3dIsoAspect {
    let color = Color::named(NamedColor::GRAY75);
    Prs3dIsoAspect::new(color, AspectTypeOfLine::Solid, 1., 3)
        .expect("Iso width=1; expected aspect")
}

fn ais_rows() -> Vec<Vec<String>> {
    let f = "SelectMgrFilter";
    let o = "AisInteractiveObject";
    vec![
        observe::<_, dyn SelectMgrFilter>(
            "AisAttributeFilter",
            f,
            Handle::new(AisAttributeFilter::default()),
        ),
        observe::<_, dyn SelectMgrFilter>(
            "AisBadEdgeFilter",
            f,
            Handle::new(AisBadEdgeFilter::default()),
        ),
        observe::<_, dyn AisInteractiveObject>(
            "AisCameraFrustum",
            o,
            AisCameraFrustum::new().unwrap(),
        ),
        observe::<_, dyn AisInteractiveObject>("AisColorScale", o, AisColorScale::new().unwrap()),
        observe::<_, StandardTransient>(
            "AisGlobalStatus",
            "Standard_Transient",
            Handle::new(AisGlobalStatus::new()),
        ),
        observe::<_, dyn AisInteractiveObject>("AisManipulator", o, AisManipulator::new().unwrap()),
        observe::<_, dyn AisInteractiveObject>("AisMediaPlayer", o, AisMediaPlayer::new().unwrap()),
        observe::<_, dyn AisInteractiveObject>(
            "AisMultipleConnectedInteractive",
            o,
            AisMultipleConnectedInteractive::new(),
        ),
        observe::<_, dyn AisInteractiveObject>("AisPointCloud", o, AisPointCloud::new().unwrap()),
        observe::<_, dyn AisInteractiveObject>("AisRubberBand", o, AisRubberBand::new().unwrap()),
        observe::<_, StandardTransient>(
            "AisSelection",
            "Standard_Transient",
            Handle::new(AisSelection::new()),
        ),
        observe::<_, dyn AisInteractiveObject>("AisTextLabel", o, AisTextLabel::new()),
        observe::<_, dyn AisInteractiveObject>("AisViewCube", o, AisViewCube::new().unwrap()),
        observe::<_, dyn AisInteractiveObject>(
            "AisXRTrackedDevice",
            o,
            AisXrTrackedDevice::new().unwrap(),
        ),
    ]
}

fn prs3d_rows() -> Vec<Vec<String>> {
    let b = "Prs3dBasicAspect";
    vec![
        observe::<_, dyn Prs3dBasicAspect>(
            "Prs3dArrowAspect",
            b,
            Handle::new(Prs3dArrowAspect::new().unwrap()),
        ),
        observe::<_, dyn Prs3dBasicAspect>(
            "Prs3dDatumAspect",
            b,
            Handle::new(Prs3dDatumAspect::new().unwrap()),
        ),
        observe::<_, dyn Prs3dBasicAspect>(
            "Prs3dDimensionAspect",
            b,
            Handle::new(Prs3dDimensionAspect::new().unwrap()),
        ),
        observe::<_, dyn Graphic3dPresentationAttributesSource>(
            "Prs3dDrawer",
            "Graphic3d_PresentationAttributes",
            Handle::new(Prs3dDrawer::new()),
        ),
        observe::<_, Prs3dLineAspect>(
            "Prs3dIsoAspect",
            "Prs3dLineAspect",
            Handle::new(iso_aspect()),
        ),
        observe::<_, dyn Prs3dBasicAspect>(
            "Prs3dPlaneAspect",
            b,
            Handle::new(Prs3dPlaneAspect::new().unwrap()),
        ),
        observe::<_, dyn Prs3dBasicAspect>(
            "Prs3dShadingAspect",
            b,
            Handle::new(Prs3dShadingAspect::new().unwrap()),
        ),
        observe::<_, dyn Prs3dBasicAspect>(
            "Prs3dTextAspect",
            b,
            Handle::new(Prs3dTextAspect::new()),
        ),
        observe::<_, dyn PrsDimDimensionSource>(
            "PrsDimLengthDimension",
            "PrsDimDimension",
            PrsDimLengthDimension::new(),
        ),
    ]
}

fn selection_rows() -> Vec<Vec<String>> {
    let c = "SelectMgrCompositionFilter";
    let t = "Standard_Transient";
    let h = "StdPrsHLRShapeI";
    vec![
        observe::<_, dyn SelectMgrCompositionFilter>(
            "SelectMgrAndFilter",
            c,
            Handle::new(SelectMgrAndFilter::default()),
        ),
        observe::<_, StandardTransient>(
            "SelectMgrFrustumBuilder",
            t,
            Handle::new(SelectMgrFrustumBuilder::new()),
        ),
        observe::<_, dyn SelectMgrCompositionFilter>(
            "SelectMgrOrFilter",
            c,
            Handle::new(SelectMgrOrFilter::default()),
        ),
        observe::<_, StandardTransient>(
            "SelectMgrViewerSelector",
            t,
            Handle::new(SelectMgrViewerSelector::new()),
        ),
        observe::<_, StandardTransient>("StdPrsBRepFont", t, Handle::new(StdPrsBRepFont::new())),
        observe::<_, StandardTransient>(
            "StdPrsBRepFontCache",
            t,
            Handle::new(StdPrsBRepFontCache::default()),
        ),
        observe::<_, dyn StdPrsHlrShapeI>(
            "StdPrsHLRPolyShape",
            h,
            Handle::new(StdPrsHlrPolyShape::default()),
        ),
        observe::<_, dyn StdPrsHlrShapeI>(
            "StdPrsHLRShape",
            h,
            Handle::new(StdPrsHlrShape::default()),
        ),
        observe::<_, StandardTransient>("V3dTrihedron", t, V3dTrihedron::new().unwrap()),
    ]
}

#[test]
fn dynamic_descriptors_of_real_objects_and_projected_handles_equal_native() {
    // One test: descriptor reference counts are process-wide and must not race another query.
    let rows: Vec<String> = [ais_rows(), prs3d_rows(), selection_rows()]
        .into_iter()
        .flatten()
        .flatten()
        .collect();
    let actual = rows.join("\n") + "\n";
    print!("{actual}");
    assert_eq!(actual, NATIVE);
}
