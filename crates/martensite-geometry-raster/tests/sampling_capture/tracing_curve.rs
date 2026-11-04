//! A named adaptor records deterministic mathematical evaluation and injected failure.
use martensite_tkernel::{
    impl_transient,
    ncollection::Array1,
    standard::{Failure, StandardTransient},
};
use martensite_tkg3d::adaptor3d::Adaptor3dCurve;
use martensite_tkmath::{
    geom_abs::{CurveType, Shape},
    gp::Point3,
};
use std::sync::Mutex;
pub struct TracingSamplerCurve {
    transient: StandardTransient,
    parameters: Mutex<Vec<f64>>,
    blocked: Option<f64>,
}
impl_transient!(TracingSamplerCurve, "TracingSamplerCurve", dyn Adaptor3dCurve, transient, interfaces: [dyn Adaptor3dCurve]);
impl TracingSamplerCurve {
    pub fn new(blocked: Option<f64>) -> Self {
        Self {
            transient: StandardTransient::default(),
            parameters: Mutex::new(Vec::new()),
            blocked,
        }
    }
    pub fn parameters(&self) -> Vec<f64> {
        self.parameters.lock().unwrap().clone()
    }
}
impl Adaptor3dCurve for TracingSamplerCurve {
    fn first_parameter(&self) -> Result<f64, Failure> {
        Ok(0.)
    }
    fn last_parameter(&self) -> Result<f64, Failure> {
        Ok(2.)
    }
    fn nb_intervals(&self, _shape: Shape) -> Result<i32, Failure> {
        Ok(2)
    }
    fn intervals(&self, bounds: &mut Array1<'_, f64>, _shape: Shape) -> Result<(), Failure> {
        for (index, value) in [0., 1., 2.].into_iter().enumerate() {
            bounds
                .set_value(index as i32 + 1, value)
                .map_err(|error| error.into_failure("TracingSamplerCurve::Intervals"))?;
        }
        Ok(())
    }
    fn get_type(&self) -> Result<CurveType, Failure> {
        Ok(CurveType::OtherCurve)
    }
    fn eval_d0(&self, u: f64) -> Result<Point3, Failure> {
        self.parameters.lock().unwrap().push(u);
        if self
            .blocked
            .is_some_and(|blocked| u.to_bits() == blocked.to_bits())
        {
            return Err(Failure::with_message(
                "CurveEvaluationUnavailable",
                &format!("u={u}: expected defined tracing curve"),
            ));
        }
        Ok(Point3::new(u, 0., 0.))
    }
}
