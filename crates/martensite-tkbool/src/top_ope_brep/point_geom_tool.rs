//! Geometric point records retain the tolerance rules of intersection fillers.
use super::{TopOpeBRepPoint2d, TopOpeBRepVPointInter};
use crate::top_ope_brep_ds::TopOpeBRepDSPoint;
use martensite_tkbrep::{
    brep::BRepTool,
    topods::{self, Shape},
};
use martensite_tkernel::standard::Failure;

/// Converts intersection records to retained geometric points; `TopOpeBRepPointGeomTool`.
pub struct TopOpeBRepPointGeomTool;

impl TopOpeBRepPointGeomTool {
    /// Preserves the intersection's edge-derived tolerance; `Self::from_vpoint(&view)?`.
    pub fn from_vpoint(point: &TopOpeBRepVPointInter<'_>) -> Result<TopOpeBRepDSPoint, Failure> {
        let tolerance = point.tolerance()?;
        let edge_tolerance = match point.shape_index() {
            1 | 2 => Self::edge_tolerance(point, point.shape_index())?,
            3 => 2.5 * (Self::first_edge_tolerance(point)? + Self::edge_tolerance(point, 2)?),
            _ => tolerance,
        };
        let result = if edge_tolerance < tolerance {
            tolerance
        } else {
            edge_tolerance
        };
        Ok(TopOpeBRepDSPoint::new(point.value()?, result))
    }

    fn first_edge_tolerance(point: &TopOpeBRepVPointInter<'_>) -> Result<f64, Failure> {
        // The repeated first-edge cap in the source intentionally leaves the second uncapped.
        let tolerance = Self::edge_tolerance(point, 1)?;
        Ok(if tolerance > 0.9 { 0.9 } else { tolerance })
    }

    fn edge_tolerance(point: &TopOpeBRepVPointInter<'_>, index: i32) -> Result<f64, Failure> {
        BRepTool::edge_tolerance(&topods::edge(&point.edge(index)?)?)
    }

    /// Copies a 2D intersection's spatial position and tolerance; `Self::from_point2d(&point)`.
    pub fn from_point2d(point: &TopOpeBRepPoint2d) -> TopOpeBRepDSPoint {
        TopOpeBRepDSPoint::new(*point.value(), point.tolerance())
    }

    /// Copies the current face-edge event and its tolerance; `Self::from_face_edge(&intersector)?`.
    pub fn from_face_edge(
        point: &super::TopOpeBRepFaceEdgeIntersector,
    ) -> Result<TopOpeBRepDSPoint, Failure> {
        Ok(TopOpeBRepDSPoint::new(point.value()?, point.tolerance()))
    }

    /// Extracts a vertex point and tolerance; `Self::from_shape(&vertex)?`.
    pub fn from_shape(shape: &Shape) -> Result<TopOpeBRepDSPoint, Failure> {
        TopOpeBRepDSPoint::from_shape(shape)
    }

    /// Tests equality at the larger recorded tolerance; `Self::is_equal(&first, &second)`.
    pub fn is_equal(first: &TopOpeBRepDSPoint, second: &TopOpeBRepDSPoint) -> bool {
        first.is_equal(second)
    }
}
