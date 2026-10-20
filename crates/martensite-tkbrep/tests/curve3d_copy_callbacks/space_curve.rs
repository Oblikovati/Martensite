//! Named extension curve invokes an injected, owned evaluator hook.
use martensite_tkernel::standard::dump::DumpStream;
use martensite_tkernel::{
    impl_transient,
    standard::{Failure, Handle, SharedState, StandardTransient},
};
use martensite_tkg3d::geom::{
    GeomCurve, GeomCurveD1, GeomCurveD2, GeomCurveD3, GeomGeometry, GeomLine,
};
use martensite_tkmath::{
    geom_abs::Shape,
    gp::{Point3, Transform3, Vector3},
};
use std::io;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

type BoundQuery = Arc<dyn Fn(&'static str) -> Result<f64, Failure> + Send + Sync>;

pub struct RebindingSpaceCurve {
    parent: StandardTransient,
    line: GeomLine,
    evaluations: Arc<AtomicUsize>,
    evaluator: Arc<dyn Fn(f64) -> Result<Point3, Failure> + Send + Sync>,
    parameter_hook: SharedState<Option<Arc<dyn Fn() + Send + Sync>>>,
    parameter_history: SharedState<Vec<&'static str>>,
    fallible_query: SharedState<Option<BoundQuery>>,
    dump_hook: SharedState<Option<Arc<dyn Fn() + Send + Sync>>>,
}
impl_transient!(RebindingSpaceCurve, "RebindingSpaceCurve", dyn GeomCurve, parent,
    interfaces: [dyn GeomCurve, dyn GeomGeometry]);

impl RebindingSpaceCurve {
    pub fn new(
        evaluations: Arc<AtomicUsize>,
        evaluator: Arc<dyn Fn(f64) -> Result<Point3, Failure> + Send + Sync>,
    ) -> Self {
        Self {
            parent: StandardTransient::default(),
            line: GeomLine::new(martensite_tkmath::gp::Axis1::default()),
            evaluations,
            evaluator,
            parameter_hook: SharedState::new(None),
            parameter_history: SharedState::new(Vec::new()),
            fallible_query: SharedState::new(None),
            dump_hook: SharedState::new(None),
        }
    }
}

impl RebindingSpaceCurve {
    pub fn set_dump_hook(&self, hook: Arc<dyn Fn() + Send + Sync>) {
        *self.dump_hook.write() = Some(hook);
    }
    pub fn set_parameter_hook(&self, hook: Arc<dyn Fn() + Send + Sync>) {
        *self.parameter_hook.write() = Some(hook);
    }
    pub fn set_fallible_query(&self, query: BoundQuery) {
        *self.fallible_query.write() = Some(query);
    }
    pub fn parameter_history(&self) -> Vec<&'static str> {
        self.parameter_history.read().clone()
    }
    fn before_parameter_query(&self, name: &'static str) {
        self.parameter_history.write().push(name);
        let hook = self.parameter_hook.read().clone();
        if let Some(hook) = hook {
            hook();
        }
    }
}

impl GeomGeometry for RebindingSpaceCurve {
    fn dump_json(&self, stream: &mut DumpStream, depth: i32) -> io::Result<()> {
        let hook = self.dump_hook.read().clone();
        if let Some(hook) = hook {
            hook();
        }
        self.line.dump_json(stream, depth)
    }
    fn transform(&self, transform: Transform3) -> Result<(), Failure> {
        self.line.transform(transform)
    }
    fn copy(&self) -> Result<Handle<dyn GeomGeometry>, Failure> {
        Ok(Handle::new(Self::new(self.evaluations.clone(), self.evaluator.clone())).cast())
    }
}
impl GeomCurve for RebindingSpaceCurve {
    fn reverse(&self) -> Result<(), Failure> {
        self.line.reverse()
    }
    fn reversed_parameter(&self, u: f64) -> Result<f64, Failure> {
        self.line.reversed_parameter(u)
    }
    fn first_parameter(&self) -> f64 {
        self.before_parameter_query("first");
        self.line.first_parameter()
    }
    fn last_parameter(&self) -> f64 {
        self.before_parameter_query("last");
        self.line.last_parameter()
    }
    fn try_first_parameter(&self) -> Result<f64, Failure> {
        let query = self.fallible_query.read().clone();
        query.map_or_else(|| Ok(self.first_parameter()), |query| query("first"))
    }
    fn try_last_parameter(&self) -> Result<f64, Failure> {
        let query = self.fallible_query.read().clone();
        query.map_or_else(|| Ok(self.last_parameter()), |query| query("last"))
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
    fn eval_d0(&self, u: f64) -> Result<Point3, Failure> {
        self.evaluations.fetch_add(1, Ordering::SeqCst);
        (self.evaluator)(u)
    }
    fn eval_d1(&self, u: f64) -> Result<GeomCurveD1, Failure> {
        self.line.eval_d1(u)
    }
    fn eval_d2(&self, u: f64) -> Result<GeomCurveD2, Failure> {
        self.line.eval_d2(u)
    }
    fn eval_d3(&self, u: f64) -> Result<GeomCurveD3, Failure> {
        self.line.eval_d3(u)
    }
    fn eval_dn(&self, u: f64, n: i32) -> Result<Vector3, Failure> {
        self.line.eval_dn(u, n)
    }
}
