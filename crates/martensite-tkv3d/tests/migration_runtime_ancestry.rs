//! Actual native descriptor chains distinguish inherited template RTTI from missing parents.
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::{
    standard::{Handle, RuntimeType, RuntimeTyped},
    tcollection::ExtendedString,
};
use martensite_tkmath::gp::{self, Ellipse3};
use martensite_tkservice::graphic3d::{Graphic3dCLight, Graphic3dTypeOfLightSource};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeEdge;
use martensite_tkv3d::{ais::*, prs_dim::*, select_mgr::*};

fn descriptor_chain(mut descriptor: Option<&RuntimeType>) -> String {
    let mut names: Vec<&str> = Vec::new();
    while let Some(current) = descriptor {
        names.push(current.name());
        descriptor = current.parent();
    }
    format!("{};", names.join(";"))
}

fn native_chain(index: usize) -> &'static str {
    include_str!("fixtures/migration-runtime-ancestry/native.txt")
        .lines()
        .nth(index)
        .unwrap()
        .split('|')
        .nth(1)
        .unwrap()
}

#[test]
fn template_bases_reuse_their_native_inherited_descriptors() {
    let triangular = SelectMgrTriangularFrustum::static_type();
    let sensitive = SelectMgrSensitiveEntitySet::static_type();
    println!("{}", descriptor_chain(Some(triangular)));
    println!("{}", descriptor_chain(Some(sensitive)));
    assert_eq!(descriptor_chain(Some(triangular)), native_chain(0));
    assert_eq!(descriptor_chain(Some(sensitive)), native_chain(1));
}

#[test]
fn light_descriptor_has_one_interactive_parent_without_duplicate_self() {
    let actual = descriptor_chain(Some(AisLightSource::static_type()));
    println!("{actual}");
    assert_eq!(actual, native_chain(2));
    assert_eq!(actual.matches("AisLightSource;").count(), 1);
}

#[test]
fn radius_descriptors_retain_the_actual_ellipse_parent() {
    let major = PrsDimMaxRadiusDimension::static_type();
    let minor = PrsDimMinRadiusDimension::static_type();
    println!("{}", descriptor_chain(Some(major)));
    println!("{}", descriptor_chain(Some(minor)));
    assert_eq!(descriptor_chain(Some(major)), native_chain(3));
    assert_eq!(descriptor_chain(Some(minor)), native_chain(4));
}

fn measured_ellipse() -> Shape {
    let ellipse = Ellipse3::new(gp::xoy(), 5., 3.).unwrap();
    BRepBuilderAPIMakeEdge::from_elips(ellipse)
        .unwrap()
        .edge()
        .unwrap()
        .into()
}

#[test]
fn actual_radius_interface_keeps_identity_counts_and_live_fields() {
    let shape = measured_ellipse();
    let text = ExtendedString::new("r");
    assert_radius_projection(PrsDimMaxRadiusDimension::new(&shape, 5., &text));
    assert_radius_projection(PrsDimMinRadiusDimension::new(&shape, 3., &text));
}

fn assert_radius_projection<T: PrsDimEllipseRadiusDimension + RuntimeTyped>(radius: Handle<T>) {
    let base = radius.cast::<dyn PrsDimEllipseRadiusDimension>();
    assert!(
        !base.is_null(),
        "actual radius must project its declared ellipse parent"
    );
    assert_eq!(base.object_address(), radius.object_address());
    assert_eq!(radius.ref_count(), 2);
    assert_eq!(base.value(), radius.value());
    assert_eq!(base.cast::<T>(), radius);
    drop(base);
    assert_eq!(radius.ref_count(), 1);
    let _: &(dyn Send + Sync) = &radius;
}

#[test]
fn actual_light_keeps_its_live_interactive_interface_and_counter() {
    let light = Handle::new(Graphic3dCLight::new(Graphic3dTypeOfLightSource::Ambient));
    let object = AisLightSource::new(&light).unwrap();
    let base = object.cast::<dyn AisInteractiveObject>();
    assert!(
        !base.is_null(),
        "actual light must project its interactive parent"
    );
    assert_eq!(base.object_address(), object.object_address());
    assert_eq!(object.ref_count(), 2);
    assert_eq!(base.cast::<AisLightSource>(), object);
    drop(base);
    assert_eq!(object.ref_count(), 1);
    let _: &(dyn Send + Sync) = &object;
}

#[test]
fn light_source_and_concrete_views_share_the_native_descriptor() {
    let concrete = AisLightSource::static_type();
    let source = <dyn AisLightSourceSource>::static_type();
    assert!(std::ptr::eq(concrete, source));
}
