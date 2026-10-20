//! Named callback fixture with exact traces and injected failure positions.
use martensite_tkernel::{
    impl_transient,
    standard::{Failure, Handle, StandardTransient},
};
use martensite_tkg3d::{
    adaptor3d::{Adaptor3dSurface, h_surface_tool::Adaptor3dSurfaceHandle},
    geom::GeomSurfaceD1,
};
use martensite_tkmath::{
    geom_abs::SurfaceType,
    gp::{Point3, Vector3},
};
use std::sync::{Arc, Mutex};
pub type CallbackTrace = Arc<Mutex<Vec<&'static str>>>;

pub struct SamplingCallbackSurface {
    parent: StandardTransient,
    pub bounds: [f64; 4],
    pub trace: CallbackTrace,
    pub fail_at: Option<usize>,
    pub spline: Option<[i32; 4]>,
}
impl_transient!(SamplingCallbackSurface, "SamplingCallbackSurface", dyn Adaptor3dSurface, parent, interfaces: [dyn Adaptor3dSurface]);

impl SamplingCallbackSurface {
    pub fn callback(&self, name: &'static str) -> Result<(), Failure> {
        let mut trace = self.trace.lock().unwrap();
        trace.push(name);
        if self.fail_at == Some(trace.len()) {
            return Err(Failure::with_message(
                "SamplingCallbackFailure",
                &format!(
                    "callback {name} at position {}: expected successful sampling callback",
                    trace.len()
                ),
            ));
        }
        Ok(())
    }
    fn bound(&self, index: usize, name: &'static str) -> Result<f64, Failure> {
        self.callback(name)?;
        Ok(self.bounds[index])
    }
    fn spline_field(&self, index: usize, name: &'static str) -> Result<i32, Failure> {
        self.callback(name)?;
        Ok(self.spline.unwrap()[index])
    }
}

impl Adaptor3dSurface for SamplingCallbackSurface {
    fn first_u_parameter(&self) -> Result<f64, Failure> {
        self.bound(0, "first_u")
    }
    fn last_u_parameter(&self) -> Result<f64, Failure> {
        self.bound(2, "last_u")
    }
    fn first_v_parameter(&self) -> Result<f64, Failure> {
        self.bound(1, "first_v")
    }
    fn last_v_parameter(&self) -> Result<f64, Failure> {
        self.bound(3, "last_v")
    }
    fn get_type(&self) -> Result<SurfaceType, Failure> {
        self.callback("kind")?;
        Ok(if self.spline.is_some() {
            SurfaceType::BSplineSurface
        } else {
            SurfaceType::OtherSurface
        })
    }
    fn nb_u_knots(&self) -> Result<i32, Failure> {
        self.spline_field(0, "u_knots")
    }
    fn u_degree(&self) -> Result<i32, Failure> {
        self.spline_field(1, "u_degree")
    }
    fn nb_v_knots(&self) -> Result<i32, Failure> {
        self.spline_field(2, "v_knots")
    }
    fn v_degree(&self) -> Result<i32, Failure> {
        self.spline_field(3, "v_degree")
    }
    fn eval_d1(&self, u: f64, v: f64) -> Result<GeomSurfaceD1, Failure> {
        self.callback("normal")?;
        Ok(GeomSurfaceD1 {
            point: Point3::new(u, v, 0.),
            d1u: Vector3::new(1., 0., 0.),
            d1v: Vector3::new(0., 1., 0.),
        })
    }
}

pub fn make_surface(
    bounds: [f64; 4],
    fail_at: Option<usize>,
) -> (Adaptor3dSurfaceHandle, CallbackTrace) {
    make_spline_surface(bounds, fail_at, None)
}
pub fn make_spline_surface(
    bounds: [f64; 4],
    fail_at: Option<usize>,
    spline: Option<[i32; 4]>,
) -> (Adaptor3dSurfaceHandle, CallbackTrace) {
    let trace = Arc::new(Mutex::new(Vec::new()));
    let surface = SamplingCallbackSurface {
        parent: StandardTransient::default(),
        bounds,
        trace: trace.clone(),
        fail_at,
        spline,
    };
    (Handle::new(surface).cast(), trace)
}
