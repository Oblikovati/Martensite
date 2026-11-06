//! Owned expected coordinates for the borrowed walking-point view.
use martensite_tkbool::top_ope_brep::TopOpeBRepWPointInter;
use martensite_tkgeomalgo::int_surf::IntSurfPntOn2S;
use martensite_tkmath::gp::{Point2, Point3};

#[test]
fn walking_point_borrows_and_rebinds_without_copying_or_reordering() {
    let mut point = IntSurfPntOn2S::new();
    point.set_value_all(Point3::new(1., 2., 3.), 4., 5., 6., 7.);
    let mut view = TopOpeBRepWPointInter::new();
    view.set(&point);
    assert!(std::ptr::eq(view.point_on_two_surfaces().unwrap(), &point));
    assert_eq!(view.parameters_on_s1().unwrap(), (4., 5.));
    assert_eq!(view.parameters_on_s2().unwrap(), (6., 7.));
    assert_eq!(view.parameters().unwrap(), (4., 5., 6., 7.));
    assert_eq!(view.value_on_s1().unwrap(), Point2::new(4., 5.));
    assert_eq!(view.value_on_s2().unwrap(), Point2::new(6., 7.));
    assert_eq!(view.value().unwrap(), Point3::new(1., 2., 3.));
    let second = IntSurfPntOn2S::new();
    view.set(&second);
    assert!(std::ptr::eq(view.point_on_two_surfaces().unwrap(), &second));
    assert_eq!(view.parameters().unwrap(), (0., 0., 0., 0.));
}

#[test]
fn uninitialized_walking_view_has_a_defined_error_instead_of_pointer_ub() {
    for view in [
        TopOpeBRepWPointInter::new(),
        TopOpeBRepWPointInter::default(),
    ] {
        assert!(view.point_on_two_surfaces().is_none());
        assert_eq!(
            view.value().unwrap_err().exception_type(),
            "Standard_NullObject"
        );
        assert!(
            view.value()
                .unwrap_err()
                .message()
                .contains("source point=None")
        );
        assert!(view.parameters_on_s1().is_err());
        assert!(view.parameters_on_s2().is_err());
        assert!(view.parameters().is_err());
        assert!(view.value_on_s1().is_err());
        assert!(view.value_on_s2().is_err());
    }
}

#[test]
fn borrowed_walking_view_is_send_and_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<TopOpeBRepWPointInter<'static>>();
}
