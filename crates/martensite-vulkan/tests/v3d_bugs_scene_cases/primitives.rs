//! Interactive lines and points built from Cartesian points.
use super::prelude::*;
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::geom::{GeomCartesianPoint, GeomPoint};
use martensite_tkv3d::ais::{AisLine, AisPoint};
use martensite_tkv3d::v3d::V3dTypeOfOrientation;

fn cartesian([x, y, z]: [f64; 3]) -> Handle<dyn GeomPoint> {
    Handle::new(GeomCartesianPoint::new(x, y, z)).cast()
}

/// The banner of `line_from_top`.
const LINE_FROM_TOP_BANNER: &[&str] = &[
    "================================================================",
    "OCC25063: Visualization - 2D objects are not displayed at some camera positions",
    "================================================================",
    "",
];

/// `bug25063`: a line seen end-on from the top is still drawn after fitting.
pub(crate) fn line_from_top() -> Result<(), Box<dyn Error>> {
    banner(LINE_FROM_TOP_BANNER);
    let mut run = SessionRun::new()?;
    run.session()
        .set_standard_projection(V3dTypeOfOrientation::ZUP_TOP)?;
    let (start, end) = (cartesian([-90., -90., 130.]), cartesian([90., 90., 130.]));
    let line = AisLine::from_points(&start, &end, crate::line_fixture::units());
    run.session().display_object("1", line.cast())?;
    run.session().fit()?;
    run.dump("bug25063.png")?;
    run.finish()
}

/// The banner of `huge_point_fitted`.
const HUGE_POINT_FITTED_BANNER: &[&str] = &[
    "================================================================",
    "OCC25767: 'vdisplay' command raises e x c e p t i o n",
    "================================================================",
    "",
];

/// The points of `bug25767`, the second one far beyond the first.
// The regression literal 8.4813999999999999e+102 rounds to 8..
const HUGE_POINTS: [(&str, [f64; 3]); 2] = [
    (
        "p1",
        [-108.4710001, -108.56800009999999, -145.30700009999998],
    ),
    ("p2", [8.4814e102, 117.1230001, 128.35480981703273]),
];

/// `bug25767`: a point at 8.5e102 is displayed and fitted without an exception.
pub(crate) fn huge_point_fitted() -> Result<(), Box<dyn Error>> {
    banner(HUGE_POINT_FITTED_BANNER);
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    run.session().set_axonometric()?;
    for (name, xyz) in HUGE_POINTS {
        let point = AisPoint::new(&cartesian(xyz))?;
        run.session().display_object(name, point.cast())?;
    }
    run.session().fit()?;
    run.dump("bug25767.png")?;
    run.finish()
}

/// The banner of `flat_points_fitted`.
const FLAT_POINTS_FITTED_BANNER: &[&str] = &[
    "============",
    "0029837: Visualization, Graphic3d_Camera - Fit All operation works incorrectly on flat object",
    "============",
    "",
];

/// `bug29837`: two large point markers and the segment between them, seen from the top, fit
/// the window's width: each marker shows at the window's border.
pub(crate) fn flat_points_fitted() -> Result<(), Box<dyn Error>> {
    use crate::viewer_objects::{point_object, segment_object};
    use martensite_tkernel::quantity::{Color, NamedColor};
    use martensite_tkservice::aspect::AspectTypeOfMarker;
    banner(FLAT_POINTS_FITTED_BANNER);
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    run.session()
        .set_standard_projection(V3dTypeOfOrientation::ZUP_TOP)?;
    let ends = [
        ("p1", [0., 0., 4000.], NamedColor::RED),
        ("p2", [200., 0., 4000.], NamedColor::GREEN),
    ];
    for (name, point, _) in ends {
        run.session().display_object(name, point_object(point)?)?;
    }
    for (name, _, color) in ends {
        run.session().set_color(name, Color::named(color))?;
    }
    for (name, _, _) in ends {
        run.session()
            .set_marker(name, AspectTypeOfMarker::Point, 20.)?;
    }
    run.session()
        .display_object("s", segment_object([ends[0].1, ends[1].1])?)?;
    run.session().fit()?;
    for ((x, y), expected, message) in [
        ((5, 200), "RED", "Error: p1 not found"),
        ((402, 200), "GREEN", "Error: p2 not found"),
    ] {
        if run.session().pixel_color_name(x, y)? != expected {
            println!("{message}");
        }
    }
    run.dump("bug29837.png")?;
    run.finish()
}
