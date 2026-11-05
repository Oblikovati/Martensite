//! Face/edge intersections retain the classified curve-surface events in traversal order.
use martensite_tkbrep::{
    brep::BRepTool,
    topods::{self, Edge, Face, Shape},
};
use martensite_tkernel::{precision, standard::Failure};
use martensite_tkg3d::{
    geom_adaptor::GeomAdaptorCurve,
    top_abs::{Orientation, State},
};
use martensite_tkgeomalgo::int_curve_surface::IntersectionPoint;
use martensite_tktopalgo::brep_int_curve_surface::BRepIntCurveSurfaceInter;
mod accessors;
mod vertices;

/// Intersects an edge against a face; `let inter = TopOpeBRepFaceEdgeIntersector::new();`.
#[derive(Default)]
pub struct TopOpeBRepFaceEdgeIntersector {
    face: Face,
    edge: Edge,
    tolerance: f64,
    force_tolerance: bool,
    intersection_done: bool,
    points: Vec<(IntersectionPoint, State)>,
    point_index: i32,
}

impl TopOpeBRepFaceEdgeIntersector {
    /// Creates an empty intersector; `let inter = TopOpeBRepFaceEdgeIntersector::new();`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Computes intersections in forward orientation; `inter.perform(&face, &edge)?;`.
    pub fn perform(&mut self, face: &Shape, edge: &Shape) -> Result<(), Failure> {
        self.reset_intersection();
        if !self.force_tolerance {
            self.shape_tolerances(face, edge)?;
        }
        self.edge = topods::edge(&edge.oriented(Orientation::Forward))?;
        self.face = topods::face(&face.oriented(Orientation::Forward))?;
        // Reference Perform overwrites even a forced value with the edge tolerance.
        self.tolerance = BRepTool::edge_tolerance(&self.edge)?;
        self.collect_intersections()?;
        self.intersection_done = true;
        Ok(())
    }

    fn collect_intersections(&mut self) -> Result<(), Failure> {
        let (curve, first, last) = BRepTool::curve(&self.edge)?;
        let adaptor = GeomAdaptorCurve::with_range(&curve, first, last)?;
        let mut intersector = BRepIntCurveSurfaceInter::new();
        intersector.init_curve(self.face.as_ref(), &adaptor, self.tolerance)?;
        while intersector.more() {
            let state = if intersector.state()? == State::In {
                State::In
            } else {
                State::On
            };
            self.points.push((intersector.point()?, state));
            intersector.next()?;
        }
        Ok(())
    }

    fn reset_intersection(&mut self) {
        self.points.clear();
        self.intersection_done = false;
    }

    fn current_point(&self) -> Result<&(IntersectionPoint, State), Failure> {
        let index = self.point_index;
        if index > 0 && (index as usize) <= self.points.len() {
            return Ok(&self.points[index as usize - 1]);
        }
        Err(Failure::with_message(
            "Standard_OutOfRange",
            &format!(
                "TopOpeBRepFaceEdgeIntersector: point index {index}; expected 1..={}",
                self.points.len()
            ),
        ))
    }

    /// Sets the value returned until Perform overwrites it; `inter.force_tolerance(1e-5);`.
    pub fn force_tolerance(&mut self, tolerance: f64) {
        self.tolerance = tolerance;
        self.force_tolerance = true;
    }

    /// Returns the most recently selected tolerance; `let tolerance = inter.tolerance();`.
    pub fn tolerance(&self) -> f64 {
        self.tolerance
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use martensite_tkg3d::top_abs::ShapeEnum;
    use martensite_tkgeomalgo::int_curve_surface::TransitionOnCurve;
    use martensite_tkmath::gp::Point3;
    #[test]
    fn transition_truth_table_and_tolerance_fallback_match_native_branches() {
        let mut inter = TopOpeBRepFaceEdgeIntersector::new();
        assert_eq!(
            inter
                .tolerance_max(&Shape::default(), ShapeEnum::Edge)
                .unwrap(),
            precision::intersection()
        );
        inter
            .shape_tolerances(&Shape::default(), &Shape::default())
            .unwrap();
        assert_eq!(inter.tolerance(), precision::intersection());
        for (kind, expected) in [
            (TransitionOnCurve::In, (State::Out, State::In)),
            (TransitionOnCurve::Out, (State::In, State::Out)),
            (TransitionOnCurve::Tangent, (State::In, State::In)),
        ] {
            inter.points = vec![(
                IntersectionPoint::new(Point3::default(), 0., 0., 0., kind),
                State::On,
            )];
            inter.init_point();
            let tr = inter.transition(2, Orientation::Forward).unwrap();
            assert_eq!((tr.before(), tr.after()), expected);
            let tr = inter.transition(2, Orientation::Reversed).unwrap();
            assert_eq!((tr.before(), tr.after()), (expected.1, expected.0));
        }
        inter.reset_intersection();
        assert!(!inter.intersection_done);
        assert_eq!(inter.nb_points(), 0);
    }
}
