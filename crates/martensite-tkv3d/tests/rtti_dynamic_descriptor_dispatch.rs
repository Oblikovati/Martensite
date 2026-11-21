//! DynamicType on concrete objects and handles projected to their interfaces.
//!
//! Rows equal the recorded tkv3d_dynamic_descriptor_dispatch capture (three native processes).
//! Both routes return the canonical process-lived descriptor for the concrete owner.
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkernel::standard::{Handle, RuntimeTyped, Transient, TransientHandleTarget};
use martensite_tkservice::aspect::AspectTypeOfLine;
use martensite_tkservice::graphic3d::Graphic3dPresentationAttributesSource;
use martensite_tkv3d::ais::*;
use martensite_tkv3d::prs_dim::{PrsDimDimensionSource, PrsDimLengthDimension};
use martensite_tkv3d::prs3d::*;
use martensite_tkv3d::select_mgr::*;
use martensite_tkv3d::std_prs::*;
use martensite_tkv3d::v3d::V3dTrihedron;

const NATIVE: &str = include_str!("fixtures/rtti-dynamic-descriptor-dispatch-native.txt");

/// The concrete row records the class name and descriptor identity.
fn concrete<T: RuntimeTyped + Transient>(label: &str, object: &Handle<T>) -> String {
    let receiver = object
        .get()
        .expect("object=null; expected a constructed owner");
    let descriptor = <T as Transient>::dynamic_type(receiver);
    let same = i32::from(std::ptr::eq(descriptor, T::static_type()));
    format!("{label}|concrete|{}|{same}", descriptor.name())
}

/// A projected row: the owner-aware borrowed descriptor of a base-typed handle.
fn projected<T: RuntimeTyped + Transient, B: ?Sized + TransientHandleTarget>(
    label: &str,
    role: &str,
    object: &Handle<T>,
) -> String {
    let base: Handle<B> = object.cast();
    assert!(
        !base.is_null(),
        "{label} as {role}; expected a projected handle"
    );
    let descriptor = base
        .dynamic_type()
        .expect("projected handle=null; expected the owner descriptor");
    let same = i32::from(std::ptr::eq(descriptor, T::static_type()));
    format!("{label}|{role}|{}|{same}", descriptor.name())
}

fn observe<T, P>(label: &str, parent: &str, object: Handle<T>) -> Vec<String>
where
    T: RuntimeTyped + Transient,
    P: ?Sized + TransientHandleTarget,
{
    vec![
        concrete(label, &object),
        projected::<T, dyn Transient>(label, "transient", &object),
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
        observe::<_, dyn Transient>(
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
        observe::<_, dyn Transient>(
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

fn iso_aspect_rows() -> Vec<String> {
    let object = Handle::new(iso_aspect());
    let descriptor = object.dynamic_type().expect("live iso aspect descriptor");
    assert!(std::ptr::eq(
        descriptor.parent().expect("iso aspect parent descriptor"),
        Prs3dLineAspect::static_type(),
    ));
    // The line aspect is composed. The basic-aspect interface still dispatches to the owner.
    observe::<_, dyn Prs3dBasicAspect>("Prs3dIsoAspect", "Prs3dLineAspect", object)
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
        iso_aspect_rows(),
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
        observe::<_, dyn Transient>(
            "SelectMgrFrustumBuilder",
            t,
            Handle::new(SelectMgrFrustumBuilder::new()),
        ),
        observe::<_, dyn SelectMgrCompositionFilter>(
            "SelectMgrOrFilter",
            c,
            Handle::new(SelectMgrOrFilter::default()),
        ),
        observe::<_, dyn Transient>(
            "SelectMgrViewerSelector",
            t,
            Handle::new(SelectMgrViewerSelector::new()),
        ),
        observe::<_, dyn Transient>("StdPrsBRepFont", t, Handle::new(StdPrsBRepFont::new())),
        observe::<_, dyn Transient>(
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
        observe::<_, dyn Transient>("V3dTrihedron", t, V3dTrihedron::new().unwrap()),
    ]
}

#[test]
fn dynamic_descriptors_of_real_objects_and_projected_handles_equal_native() {
    let rows: Vec<String> = [ais_rows(), prs3d_rows(), selection_rows()]
        .into_iter()
        .flatten()
        .flatten()
        .collect();
    let actual = rows.join("\n") + "\n";
    print!("{actual}");
    // The last three native columns count owners of a descriptor handle. In the Rust model
    // descriptors are static borrows. Compare every class name and dispatch observation.
    let expected: Vec<String> = NATIVE
        .lines()
        .map(|row| row.split('|').take(4).collect::<Vec<_>>().join("|"))
        .collect();
    assert_eq!(rows, expected);
}
