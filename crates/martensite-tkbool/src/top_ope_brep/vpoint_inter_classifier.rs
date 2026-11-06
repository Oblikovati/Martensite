//! Surface-intersection point classification preserves the two classifier passes.
use super::{TopOpeBRepPointClassifier, TopOpeBRepVPointInter};
use crate::top_ope_brep_tool::TopOpeBRepToolShapeTool;
use martensite_tkbrep::topods::{self, Face, Shape};
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::State;
use martensite_tkmath::gp::Point2;
use martensite_tktopalgo::brep_class::BRepClassFaceClassifier;

mod boundary;
use boundary::slow_boundary;

/// Classifies each side of a surface-intersection point; `Self::new()`.
pub struct TopOpeBRepVPointInterClassifier {
    slow: BRepClassFaceClassifier,
    state: State,
    null_shape: Shape,
}

impl Default for TopOpeBRepVPointInterClassifier {
    fn default() -> Self {
        Self::new()
    }
}

impl TopOpeBRepVPointInterClassifier {
    /// Starts with UNKNOWN state and no classified edge; `Self::new().edge_parameter()?`.
    pub fn new() -> Self {
        Self {
            slow: BRepClassFaceClassifier::new(),
            state: State::Unknown,
            null_shape: Shape::default(),
        }
    }

    /// Classifies and updates the requested point side; `classifier.vpoint_position(&face, &mut point, 1, &mut cache, false, 1e-7)?`.
    pub fn vpoint_position(
        &mut self,
        shape: &Shape,
        point: &mut TopOpeBRepVPointInter<'_>,
        index: i32,
        cache: &mut TopOpeBRepPointClassifier,
        assume_in_on: bool,
        tolerance: f64,
    ) -> Result<State, Failure> {
        self.state = State::Unknown;
        let uv = checked_parameters(point, index)?;
        if on_domain(point, index)? {
            return self.domain_position(point, index);
        }
        let face = topods::face(shape)?;
        let uv = adjusted_parameters(shape, uv)?;
        self.classify(&face, uv, (point, index), cache, assume_in_on, tolerance)
    }

    fn domain_position(
        &mut self,
        point: &mut TopOpeBRepVPointInter<'_>,
        index: i32,
    ) -> Result<State, Failure> {
        point.set_state(State::On, index)?;
        let edge = point.edge(index)?;
        point.set_edge_on(&edge, point.edge_parameter(index)?, index);
        self.state = State::On;
        Ok(self.state)
    }

    fn classify(
        &mut self,
        face: &Face,
        uv: Point2,
        side: (&mut TopOpeBRepVPointInter<'_>, i32),
        cache: &mut TopOpeBRepPointClassifier,
        assume: bool,
        tolerance: f64,
    ) -> Result<State, Failure> {
        let (point, index) = side;
        let fast = cache.classify(face, uv, tolerance)?;
        self.state = fast;
        point.set_state(fast, index)?;
        let assumed_side = assume && index == point.shape_index();
        if assumed_side || matches!(fast, State::Out | State::On) {
            self.slow.perform(face, &uv, tolerance, false, 0.1)?;
            self.state = self.slow.state();
            self.refine(face, uv, point, index, assumed_side, assume)?;
        }
        point.set_state(self.state, index)?;
        Ok(self.state)
    }

    fn refine(
        &mut self,
        face: &Face,
        uv: Point2,
        point: &mut TopOpeBRepVPointInter<'_>,
        index: i32,
        assumed_side: bool,
        assume: bool,
    ) -> Result<(), Failure> {
        if self.state == State::On || (assume && !assumed_side && self.state == State::Out) {
            self.state = slow_boundary(point.value()?, uv, &mut self.slow, face)?;
            if self.state == State::On {
                point.set_edge_on(self.slow.edge()?.edge(), self.slow.edge_parameter()?, index);
            }
        } else if assumed_side && self.state == State::Out {
            self.state = State::In;
        }
        Ok(())
    }

    /// Returns the last slow classifier edge only when ON; `classifier.edge()?`.
    pub fn edge(&self) -> Result<&Shape, Failure> {
        if self.state == State::On {
            return Ok(self.slow.edge()?.edge().as_ref());
        }
        Ok(&self.null_shape)
    }

    /// Returns the last slow edge parameter when ON, otherwise zero; `classifier.edge_parameter()?`.
    pub fn edge_parameter(&self) -> Result<f64, Failure> {
        if self.state == State::On {
            return self.slow.edge_parameter();
        }
        Ok(0.0)
    }
}

fn checked_parameters(point: &TopOpeBRepVPointInter<'_>, index: i32) -> Result<Point2, Failure> {
    if index != 1 && index != 2 {
        return Err(Failure::with_message(
            "Standard_ProgramError",
            "VPointClassifier : wrong Index",
        ));
    }
    point.surface_parameters(index)
}

fn on_domain(point: &TopOpeBRepVPointInter<'_>, index: i32) -> Result<bool, Failure> {
    if index == 1 {
        return point.is_on_dom_s1();
    }
    point.is_on_dom_s2()
}

fn adjusted_parameters(shape: &Shape, uv: Point2) -> Result<Point2, Failure> {
    let (mut u, mut v) = (uv.x(), uv.y());
    TopOpeBRepToolShapeTool::adjust_on_periodic(shape, &mut u, &mut v)?;
    Ok(Point2::new(u, v))
}
