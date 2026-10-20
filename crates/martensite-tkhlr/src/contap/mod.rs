//! Contour lines of surfaces seen from a direction or a point, and their records.
mod line;
mod point;
mod types;
pub use line::ContapLine;
pub use point::ContapPoint;
pub use types::{ContapIType, ContapTFunction};

mod cont_ana;
mod cont_ana_cone;
mod geometry;
pub use cont_ana::ContapContAna;

mod cont_ana_cylinder;
mod cont_ana_sphere;

mod surf_props;
pub use surf_props::{ContapDerivAndNorm, ContapNormAndDn, ContapSurfProps};

mod h_curve2d_tool;
pub use h_curve2d_tool::ContapHCurve2dTool;

mod h_cont_tool;
pub use h_cont_tool::ContapHContTool;

mod surf_function;
mod surf_function_traits;
pub use surf_function::ContapSurfFunction;

mod arc_function;
mod equation;
mod normal_sampling;
mod sampling_context;
pub use arc_function::ContapArcFunction;
pub use sampling_context::ContapSamplingContext;

mod iwline;
pub use iwline::ContapTheIWLineOfTheIWalking;

mod search_records;
pub use search_records::{ContapThePathPointOfTheSearch, ContapTheSegmentOfTheSearch};

mod search_inside;
pub use search_inside::{ContapTheSearchInside, ContapTheSearchInsideTypes};

mod search_boundary;
pub use search_boundary::{ContapTheSearch, ContapTheSearchTypes};

mod walking;
pub use walking::{ContapTheIWalking, ContapTheIWalkingTypes};

mod contour;
pub use contour::ContapContour;
