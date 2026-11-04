//! Pole picks use pixel units; knot picks intentionally use projected model units.
use super::super::{
    fixtures,
    spline_api::{self, Planar, Spatial},
};
#[test]
fn spline_picks_keep_first_after_index_and_distinct_precision_units() {
    let space = Spatial::new(fixtures::space(0));
    let plane = Planar::new(fixtures::plane(0));
    let mut viewer = spline_api::viewer(true);
    let display = viewer.make_display(1);
    assert_eq!(space.find_pole((20., 40.), &display, 0.1, 0).unwrap(), 2);
    assert_eq!(plane.find_pole((20., 40.), &display, 0.1, 0).unwrap(), 2);
    assert_eq!(space.find_pole((20., 40.), &display, 0.1, 2).unwrap(), 0);
    assert_eq!(plane.find_pole((999., 999.), &display, 0.1, 0).unwrap(), 0);
    assert_eq!(space.find_knot((2., 4.), &display, 0.1, 0).unwrap(), 2);
    assert_eq!(plane.find_knot((2., 4.), &display, 0.1, 0).unwrap(), 2);
    assert_eq!(space.find_knot((20., 40.), &display, 0.1, 0).unwrap(), 0);
    null_picks(&display);
}
fn null_picks(display: &martensite_raster_core::RasterDisplay<'_>) {
    assert_null(
        space_null()
            .find_pole((0., 0.), display, 1., 0)
            .unwrap_err(),
    );
    assert_null(
        Planar::new(Default::default())
            .find_knot((0., 0.), display, 1., 0)
            .unwrap_err(),
    );
}
fn space_null() -> Spatial {
    Spatial::new(Default::default())
}
fn assert_null(error: martensite_tkernel::standard::Failure) {
    assert_eq!(error.exception_type(), "Standard_NullObject");
    assert_eq!(error.message(), "null geometry in a drawable");
}
