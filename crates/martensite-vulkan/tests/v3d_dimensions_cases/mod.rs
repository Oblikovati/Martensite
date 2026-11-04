//! Dimensions: lengths, angles, radii and diameters of points, edges, faces and circles,
//! with their working planes, fonts, units and highlighting.
#[path = "../support/dimension_input.rs"]
mod dimension_input;
#[path = "../support/dimension_params.rs"]
mod dimension_params;
#[path = "../support/dimension_request.rs"]
mod dimension_request;
mod edge_dimensions;
mod first_dimensions;
mod length_directions;
pub(crate) use edge_dimensions::{
    angle_arrows, circular_edge_lengths, collinear_angle, flipped_edge_highlight,
    highlighted_flyout,
};
pub(crate) use first_dimensions::{
    EdgeOperand, edge_vertex_length, face_pair_length, highlighted_length, straight_angle,
    valued_dimensions, vertex_pair_length,
};
pub(crate) use length_directions::{LengthDirections, length_directions};

/// What every dimension program uses.
mod prelude {
    pub(super) use super::dimension_input::{display_point, register_sans_font};
    pub(super) use super::dimension_params::DimensionStyle;
    pub(super) use super::dimension_request::{
        DimensionKind, DimensionRequest, Measured, WorkingPlane, display_dimension,
    };
    pub(super) use crate::case_view::viewer_session::DisplayRequest;
    pub(super) use crate::session_run::{ImageControl, SessionRun, banner};
    pub(super) use martensite_tkbrep::topods::Shape;
    pub(super) use martensite_tkernel::standard::{Failure, Handle};
    pub(super) use std::error::Error;

    pub(super) type Outcome = Result<(), Box<dyn Error>>;

    /// Dump `image` with the undefined-locus controls of a dimension whose lines coincide
    /// with displayed edges at equal depth: the scene without the dimension (`locus-1`), then
    /// without the `edges` (`locus-2`); `dump_coincident(&mut run, "a.png", "len", &["e"])?`.
    pub(super) fn dump_coincident(
        run: &mut SessionRun,
        image: &'static str,
        dimension: &str,
        edges: &[&str],
    ) -> Outcome {
        let without_dimension = [dimension];
        let controls = [
            ImageControl {
                suffix: "locus-1",
                erased: &without_dimension,
            },
            ImageControl {
                suffix: "locus-2",
                erased: edges,
            },
        ];
        run.dump_with_controls(image, &controls)
    }

    /// A vertex at `position`; `vertex([10., 0., 0.])?`.
    pub(super) fn vertex([x, y, z]: [f64; 3]) -> Result<Shape, Failure> {
        use martensite_tkmath::gp::Point3;
        use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeVertex;
        Ok(BRepBuilderAPIMakeVertex::new(Point3::new(x, y, z))?
            .vertex()?
            .as_ref()
            .clone())
    }

    /// The straight edge between two new vertices; `edge([0.; 3], [1., 0., 0.])?`.
    pub(super) fn edge(from: [f64; 3], to: [f64; 3]) -> Result<Shape, Failure> {
        use martensite_tkmath::gp::Point3;
        use martensite_tktopalgo::brep_builder_api::{
            BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeVertex,
        };
        let vertex =
            |[x, y, z]: [f64; 3]| BRepBuilderAPIMakeVertex::new(Point3::new(x, y, z))?.vertex();
        let maker = BRepBuilderAPIMakeEdge::from_vertices(&vertex(from)?, &vertex(to)?)?;
        Ok(maker.edge()?.as_ref().clone())
    }

    /// Face `index` (1-based); `face_of(&b, 1)`.
    pub(super) fn face_of(shape: &Shape, index: usize) -> Shape {
        use martensite_tkg3d::top_abs::ShapeEnum;
        crate::boolean_typed_explode::all_distinct(shape, ShapeEnum::Face)[index - 1].clone()
    }
}
