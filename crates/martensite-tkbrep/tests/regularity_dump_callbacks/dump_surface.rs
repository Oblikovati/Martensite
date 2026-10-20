//! Named plane fixture with an injected virtual dump callback.
use martensite_tkernel::{
    impl_transient,
    standard::{Failure, Handle, SharedState, StandardTransient, dump::DumpStream},
};
use martensite_tkg3d::geom::{
    GeomCurve, GeomGeometry, GeomPlane, GeomSurface, GeomSurfaceD1, GeomSurfaceD2, GeomSurfaceD3,
    SurfaceBounds,
};
use martensite_tkmath::{
    geom_abs::Shape,
    gp::{Axis3, Point3, Transform3, Vector3},
};
use std::{io, sync::Arc};

pub struct CallbackDumpPlane {
    parent: StandardTransient,
    plane: GeomPlane,
    hook: SharedState<Option<Arc<dyn Fn() + Send + Sync>>>,
}
impl_transient!(CallbackDumpPlane, "CallbackDumpPlane", dyn GeomSurface, parent,
    interfaces: [dyn GeomSurface, dyn GeomGeometry]);
impl CallbackDumpPlane {
    pub fn new() -> Self {
        Self {
            parent: StandardTransient::default(),
            plane: GeomPlane::new(Axis3::default()),
            hook: SharedState::new(None),
        }
    }
    pub fn set_dump_hook(&self, hook: Arc<dyn Fn() + Send + Sync>) {
        *self.hook.write() = Some(hook);
    }
}
impl GeomGeometry for CallbackDumpPlane {
    fn dump_json(&self, stream: &mut DumpStream, depth: i32) -> io::Result<()> {
        let hook = self.hook.read().clone();
        if let Some(hook) = hook {
            hook();
        }
        self.plane.dump_json(stream, depth)
    }
    fn transform(&self, transform: Transform3) -> Result<(), Failure> {
        self.plane.transform(transform)
    }
    fn copy(&self) -> Result<Handle<dyn GeomGeometry>, Failure> {
        self.plane.copy()
    }
}
impl GeomSurface for CallbackDumpPlane {
    fn u_reverse(&self) -> Result<(), Failure> {
        self.plane.u_reverse()
    }
    fn u_reversed_parameter(&self, u: f64) -> Result<f64, Failure> {
        self.plane.u_reversed_parameter(u)
    }
    fn v_reverse(&self) -> Result<(), Failure> {
        self.plane.v_reverse()
    }
    fn v_reversed_parameter(&self, v: f64) -> Result<f64, Failure> {
        self.plane.v_reversed_parameter(v)
    }
    fn bounds(&self) -> SurfaceBounds {
        self.plane.bounds()
    }
    fn is_u_closed(&self) -> Result<bool, Failure> {
        self.plane.is_u_closed()
    }
    fn is_v_closed(&self) -> Result<bool, Failure> {
        self.plane.is_v_closed()
    }
    fn is_u_periodic(&self) -> bool {
        self.plane.is_u_periodic()
    }
    fn is_v_periodic(&self) -> bool {
        self.plane.is_v_periodic()
    }
    fn u_iso(&self, u: f64) -> Result<Handle<dyn GeomCurve>, Failure> {
        self.plane.u_iso(u)
    }
    fn v_iso(&self, v: f64) -> Result<Handle<dyn GeomCurve>, Failure> {
        self.plane.v_iso(v)
    }
    fn continuity(&self) -> Shape {
        self.plane.continuity()
    }
    fn is_cn_u(&self, n: i32) -> Result<bool, Failure> {
        self.plane.is_cn_u(n)
    }
    fn is_cn_v(&self, n: i32) -> Result<bool, Failure> {
        self.plane.is_cn_v(n)
    }
    fn eval_d0(&self, u: f64, v: f64) -> Result<Point3, Failure> {
        self.plane.eval_d0(u, v)
    }
    fn eval_d1(&self, u: f64, v: f64) -> Result<GeomSurfaceD1, Failure> {
        self.plane.eval_d1(u, v)
    }
    fn eval_d2(&self, u: f64, v: f64) -> Result<GeomSurfaceD2, Failure> {
        self.plane.eval_d2(u, v)
    }
    fn eval_d3(&self, u: f64, v: f64) -> Result<GeomSurfaceD3, Failure> {
        self.plane.eval_d3(u, v)
    }
    fn eval_dn(&self, u: f64, v: f64, nu: i32, nv: i32) -> Result<Vector3, Failure> {
        self.plane.eval_dn(u, v, nu, nv)
    }
}
