//! Four point-to-point lengths with internal arrows: automatic, oblique, X and Y directions.
use super::dimension_params::{
    DimensionParams, DimensionStyle, set_dimension_params, set_length_direction,
};
use super::prelude::*;
use martensite_tkmath::gp::Direction3;
use martensite_tkv3d::prs3d::{
    Prs3dDimensionArrowOrientation as Orientation, Prs3dDimensionTextHorizontalPosition as HPos,
};
use martensite_tkv3d::v3d::V3dTypeOfOrientation;

/// The variant of the length-direction program.
#[derive(Clone, Copy, Debug)]
pub(crate) struct LengthDirections {
    /// The second point is the first operand.
    pub reversed_points: bool,
    /// The flyout is -1 instead of 1.
    pub reversed_flyout: bool,
    /// The custom directions point the other way: (-1,-1,1), -X and -Y.
    pub reversed_directions: bool,
}

/// Two points and the custom direction of their length.
type PointPair = ([f64; 3], [f64; 3], Option<Direction3>);

/// The four point pairs, their dimension names and directions (`None`: automatic).
fn pairs(variant: LengthDirections) -> Result<[PointPair; 4], Failure> {
    let sign = if variant.reversed_directions { -1. } else { 1. };
    Ok([
        ([2., 22., 0.], [7., 29., 0.], None),
        (
            [22., 22., 0.],
            [27., 29., 0.],
            Some(Direction3::new(sign, sign, 1.)?),
        ),
        (
            [2., 2., 0.],
            [7., 9., 0.],
            Some(Direction3::new(sign, 0., 0.)?),
        ),
        (
            [22., 2., 0.],
            [27., 9., 0.],
            Some(Direction3::new(0., sign, 0.)?),
        ),
    ])
}

/// `lengthparam*`: dimension pairs on the XOY plane seen from the top.
pub(crate) fn length_directions(variant: LengthDirections, image: &'static str) -> Outcome {
    let mut run = SessionRun::new()?;
    run.session().display_trihedron("t1")?;
    for (index, (a, b, direction)) in pairs(variant)?.into_iter().enumerate() {
        let id = index + 1;
        let (first, second) = (format!("p{id}1"), format!("p{id}2"));
        display_point(&mut run, &first, a)?;
        display_point(&mut run, &second, b)?;
        let mut shapes = [Measured::Object(&first), Measured::Object(&second)];
        if variant.reversed_points {
            shapes.reverse();
        }
        let name = format!("dim{id}");
        let request = DimensionRequest {
            plane: Some(WorkingPlane::Xoy),
            ..DimensionRequest::new(DimensionKind::Length, &shapes)
        };
        display_dimension(run.session(), &name, &request)?;
        let flyout = if variant.reversed_flyout { -1. } else { 1. };
        let params = DimensionParams {
            flyout: Some(flyout),
            style: DimensionStyle {
                arrow_length: Some(0.75),
                arrow: Some(Orientation::Internal),
                label: Some(HPos::Center),
                ..DimensionStyle::default()
            },
            ..DimensionParams::default()
        };
        set_dimension_params(run.session(), &name, &params)?;
        if index > 0 {
            set_length_direction(run.session(), &name, direction)?;
        }
    }
    run.session()
        .set_standard_projection(V3dTypeOfOrientation::ZUP_TOP)?;
    run.session().fit()?;
    run.session().zoom(0.75)?;
    run.dump(image)?;
    run.finish()
}
