//! Named extension curve records evaluation before returning its owned failure.
use martensite_tkernel::{
    impl_transient,
    standard::{Failure, Handle, StandardTransient},
};
use martensite_tkg2d::geom2d::{
    Geom2dCurve, Geom2dCurveD1, Geom2dCurveD2, Geom2dCurveD3, Geom2dGeometry, Geom2dLine,
};
use martensite_tkmath::{
    geom_abs::Shape,
    gp::{Point2, Transform2, Vector2},
};
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

pub struct FailingParameterCurve {
    parent: StandardTransient,
    line: Geom2dLine,
    evaluations: Arc<AtomicUsize>,
}
impl_transient!(FailingParameterCurve, "FailingParameterCurve", dyn Geom2dCurve, parent,
    interfaces: [dyn Geom2dCurve, dyn Geom2dGeometry]);

impl FailingParameterCurve {
    pub fn new(evaluations: Arc<AtomicUsize>) -> Self {
        Self {
            parent: StandardTransient::default(),
            line: Geom2dLine::new(martensite_tkmath::gp::Axis2d::default()),
            evaluations,
        }
    }
}

impl Geom2dGeometry for FailingParameterCurve {
    fn transform(&self, transform: Transform2) -> Result<(), Failure> {
        self.line.transform(transform)
    }
    fn copy(&self) -> Result<Handle<dyn Geom2dGeometry>, Failure> {
        Ok(Handle::new(Self::new(self.evaluations.clone())).cast())
    }
}
impl Geom2dCurve for FailingParameterCurve {
    fn reverse(&self) -> Result<(), Failure> {
        self.line.reverse()
    }
    fn reversed_parameter(&self, u: f64) -> Result<f64, Failure> {
        self.line.reversed_parameter(u)
    }
    fn first_parameter(&self) -> f64 {
        self.line.first_parameter()
    }
    fn last_parameter(&self) -> f64 {
        self.line.last_parameter()
    }
    fn is_closed(&self) -> Result<bool, Failure> {
        self.line.is_closed()
    }
    fn is_periodic(&self) -> bool {
        self.line.is_periodic()
    }
    fn continuity(&self) -> Shape {
        self.line.continuity()
    }
    fn is_cn(&self, n: i32) -> Result<bool, Failure> {
        self.line.is_cn(n)
    }
    fn eval_d0(&self, u: f64) -> Result<Point2, Failure> {
        self.evaluations.fetch_add(1, Ordering::SeqCst);
        Err(Failure::with_type(
            "Standard_RangeError",
            &format!("parameter {u}; expected an accepted extension parameter"),
            "",
        ))
    }
    fn eval_d1(&self, u: f64) -> Result<Geom2dCurveD1, Failure> {
        self.line.eval_d1(u)
    }
    fn eval_d2(&self, u: f64) -> Result<Geom2dCurveD2, Failure> {
        self.line.eval_d2(u)
    }
    fn eval_d3(&self, u: f64) -> Result<Geom2dCurveD3, Failure> {
        self.line.eval_d3(u)
    }
    fn eval_dn(&self, u: f64, n: i32) -> Result<Vector2, Failure> {
        self.line.eval_dn(u, n)
    }
}
