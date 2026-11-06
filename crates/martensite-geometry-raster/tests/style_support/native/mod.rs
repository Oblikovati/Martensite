//! Live pixels for the shared helpers, independent of any curve/surface drawable engine.
mod fixtures;
mod pixels;
mod probe;
#[derive(Clone, Copy)]
pub enum Workload {
    Polygon(bool),
    Knots(bool, bool),
    Net(bool),
    Curvature(bool, bool),
}
impl Workload {
    fn spatial(self) -> bool {
        match self {
            Self::Polygon(spatial) | Self::Knots(spatial, _) | Self::Curvature(spatial, _) => {
                spatial
            }
            Self::Net(_) => true,
        }
    }
}
const CASES: [(&str, Workload); 12] = [
    ("polygon-spatial", Workload::Polygon(true)),
    ("polygon-planar", Workload::Polygon(false)),
    ("knots-spatial", Workload::Knots(true, true)),
    ("knots-planar", Workload::Knots(false, true)),
    ("knots-spatial-flag-cleared", Workload::Knots(true, false)),
    ("knots-planar-flag-cleared", Workload::Knots(false, false)),
    ("net-open", Workload::Net(false)),
    ("net-periodic", Workload::Net(true)),
    ("curvature-spatial", Workload::Curvature(true, false)),
    ("curvature-planar", Workload::Curvature(false, false)),
    ("curvature-spatial-capped", Workload::Curvature(true, true)),
    ("curvature-planar-capped", Workload::Curvature(false, true)),
];
#[test]
fn shared_style_helpers_match_closed_form_pixels() {
    for (label, workload) in CASES {
        pixels::capture(label, workload);
    }
}
