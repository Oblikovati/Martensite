//! Fresh native copies exercise binder refusals without another public operation.
use super::*;
use martensite_tkbrep::brep::{BRepBuilder, BRepTool};
mod raw_source;
pub(super) fn check(result: &WrappedConeNormalResult) {
    let body = result.family.budget.actual_certificate.as_ref().unwrap();
    let mut members = actual::binding::retained_members(&result.family.family).unwrap();
    body.rebind_family(&members).unwrap();
    cap_flag(body, &members);
    side_flag(body, &members);
    omitted_cap(body, &mut members);
    aliased_material(body, &mut members);
    raw_source::check(result, body, &members);
    body.rebind_family(&members).unwrap();
}
fn cap_flag(
    body: &martensite_tkbo::body_check::CertifiedConicalBody,
    members: &[martensite_tkbo::body_check::ConicalNativeMember],
) {
    let cap = &members[0].caps[0];
    assert!(!BRepTool::natural_restriction(&cap.face).unwrap());
    let builder = BRepBuilder::default();
    builder.natural_restriction(&cap.face, true).unwrap();
    let error = body.rebind_family(members).unwrap_err();
    assert_eq!(error.cap, Some(cap.id));
    assert!(error.cause.to_string().contains("natural_restriction"));
    builder.natural_restriction(&cap.face, false).unwrap();
}
fn side_flag(
    body: &martensite_tkbo::body_check::CertifiedConicalBody,
    members: &[martensite_tkbo::body_check::ConicalNativeMember],
) {
    let (id, face) = &members[0].sides[0];
    assert!(!BRepTool::natural_restriction(face).unwrap());
    let builder = BRepBuilder::default();
    builder.natural_restriction(face, true).unwrap();
    let error = body.rebind_family(members).unwrap_err();
    assert_eq!(error.side, Some(*id));
    assert!(error.cause.to_string().contains("natural_restriction"));
    builder.natural_restriction(face, false).unwrap();
}
fn omitted_cap(
    body: &martensite_tkbo::body_check::CertifiedConicalBody,
    members: &mut [martensite_tkbo::body_check::ConicalNativeMember],
) {
    let cap = members[0].caps.pop().unwrap();
    let error = body.rebind_family(members).unwrap_err();
    assert_eq!(error.member, Some(members[0].id));
    members[0].caps.push(cap);
}
fn aliased_material(
    body: &martensite_tkbo::body_check::CertifiedConicalBody,
    members: &mut [martensite_tkbo::body_check::ConicalNativeMember],
) {
    let other = members[1].caps[0].material.clone();
    let original = std::mem::replace(&mut members[0].caps[0].material, other);
    let error = body.rebind_family(members).unwrap_err();
    assert_eq!(error.cap, Some(members[0].caps[0].id));
    members[0].caps[0].material = original;
}
