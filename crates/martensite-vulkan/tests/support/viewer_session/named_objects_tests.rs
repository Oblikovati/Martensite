use super::*;

fn object() -> Handle<dyn AisInteractiveObject> {
    AisConnectedInteractive::new(martensite_tkv3d::prs_mgr::PrsMgrTypeOfPresentation3d::AllView)
        .cast()
}

#[test]
fn earliest_binding_resolves_and_order_is_kept() {
    let (a, b) = (object(), object());
    let mut bound = NamedObjects::default();
    bound.push("x", a.clone());
    bound.push("x", b.clone());
    assert_eq!(
        bound.get("x").map(|o| o.object_address()),
        Some(a.object_address())
    );
    bound.remove("x");
    assert_eq!(
        bound.get("x").map(|o| o.object_address()),
        Some(b.object_address())
    );
    bound.remove_object(&b);
    assert!(bound.get("x").is_none() && bound.iter().next().is_none());
}
