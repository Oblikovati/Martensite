//! A gradient setter must select its presentation kind through retained forwarding.
#[path = "retained_gradient_forwarding_fixture.rs"]
mod forwarding;
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkservice::{
    aspect::{AspectGradientBackground, AspectGradientFillMethod},
    graphic3d::{Graphic3dCViewMembers, Graphic3dTypeOfBackground},
};

fn gradient_input(method: AspectGradientFillMethod) -> AspectGradientBackground {
    let first = Color::named(NamedColor::RED);
    let second = Color::named(NamedColor::BLUE);
    AspectGradientBackground::new(first, second, method)
}
#[test]
fn retained_gradient_setters_select_kind_and_preserve_supplied_colors() {
    let scene = super::scene();
    assert_eq!(
        scene.view.background_type(),
        Graphic3dTypeOfBackground::None
    );
    for method in [
        AspectGradientFillMethod::Horizontal,
        AspectGradientFillMethod::Vertical,
    ] {
        let gradient = gradient_input(method);
        scene.view.set_gradient_background(&gradient);
        assert_selected_gradient(&*scene.view, gradient);
    }
}
#[test]
fn disabled_fill_and_explicit_override_keep_their_separate_state() {
    let scene = super::scene();
    let view = &*scene.view;
    let solid = view.cview_base().fields.read().bg_color;
    let gradient = gradient_input(AspectGradientFillMethod::None);
    view.set_background_type(Graphic3dTypeOfBackground::Cubemap);
    view.set_gradient_background(&gradient);
    assert_selected_gradient(view, gradient);
    view.set_background_type(Graphic3dTypeOfBackground::None);
    assert_eq!(view.background_type(), Graphic3dTypeOfBackground::None);
    assert_eq!(view.gradient_background(), gradient);
    assert_eq!(view.cview_base().fields.read().bg_color, solid);
}
#[test]
fn gradient_arguments_and_readback_snapshots_are_independent_values() {
    let scene = super::scene();
    let mut input = gradient_input(AspectGradientFillMethod::Horizontal);
    scene.view.set_gradient_background(&input);
    let original = input;
    input = recolored_gradient(input);
    assert_eq!(scene.view.gradient_background(), original);
    let snapshot = recolored_gradient(scene.view.gradient_background());
    assert_eq!(snapshot, input);
    assert_eq!(scene.view.gradient_background(), original);
    scene.view.set_gradient_background(&input);
    assert_eq!(scene.view.gradient_background(), input);
    let untouched = gradient_input(AspectGradientFillMethod::Horizontal);
    assert_eq!(original, untouched);
}

fn assert_selected_gradient(
    view: &dyn martensite_tkservice::graphic3d::Graphic3dCView,
    expected: AspectGradientBackground,
) {
    assert_eq!(view.background_type(), Graphic3dTypeOfBackground::Gradient);
    assert_eq!(view.gradient_background(), expected);
}
fn recolored_gradient(mut gradient: AspectGradientBackground) -> AspectGradientBackground {
    gradient.set_colors(
        Color::named(NamedColor::GREEN),
        Color::named(NamedColor::WHITE),
        AspectGradientFillMethod::Vertical,
    );
    gradient
}

#[test]
fn forwarded_gradient_dispatch_preserves_kind_override_and_snapshots() {
    use martensite_tkservice::graphic3d::Graphic3dGraphicDriverMembers;
    let scene = super::scene();
    let forwarded = forwarding::ForwardedGradientView::new(&scene.manager, &scene.driver.layers());
    let _: &(dyn Send + Sync) = &forwarded;
    let view: &dyn martensite_tkservice::graphic3d::Graphic3dCView = &forwarded;
    assert_forwarded_gradient_state(view);
}
fn assert_forwarded_gradient_state(view: &dyn martensite_tkservice::graphic3d::Graphic3dCView) {
    let gradient = gradient_input(AspectGradientFillMethod::Horizontal);
    view.set_gradient_background(&gradient);
    assert_selected_gradient(view, gradient);
    let snapshot = recolored_gradient(view.gradient_background());
    view.set_background_type(Graphic3dTypeOfBackground::None);
    assert_eq!(view.background_type(), Graphic3dTypeOfBackground::None);
    assert_eq!(view.gradient_background(), gradient);
    view.set_gradient_background(&snapshot);
    assert_selected_gradient(view, snapshot);
}
