//! How the GeomLProp templates evaluate the BRepAdaptor curve and surface: through
//! their own members (`LProp_CurveUtils::DirectAccess`, `LProp_SurfaceUtils::DirectAccess`).
use crate::brep_adaptor::{BRepAdaptorCurve, BRepAdaptorSurface};
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::adaptor3d::{Adaptor3dCurve, Adaptor3dSurface};
use martensite_tkg3d::geom::{GeomSurfaceD1, GeomSurfaceD2};
use martensite_tkgeombase::geom_lprop::surface_access::{
    DirectSurface, PropsBounds, get_surf_bounds_adaptor,
};
use martensite_tkgeombase::lprop::curve_access::{CurveD1, CurveD2, CurveD3, DirectCurve};
use martensite_tkmath::gp::{Point3, Vector3};

impl DirectCurve for &BRepAdaptorCurve {
    type Vector = Vector3;
    fn props_d0(&self, u: f64) -> Result<Point3, Failure> {
        self.d0(u)
    }
    fn props_d1(&self, u: f64) -> Result<CurveD1<Vector3>, Failure> {
        self.d1(u)
    }
    fn props_d2(&self, u: f64) -> Result<CurveD2<Vector3>, Failure> {
        self.d2(u)
    }
    fn props_d3(&self, u: f64) -> Result<CurveD3<Vector3>, Failure> {
        self.d3(u)
    }
    fn props_first_parameter(&self) -> Result<f64, Failure> {
        self.first_parameter()
    }
    fn props_last_parameter(&self) -> Result<f64, Failure> {
        self.last_parameter()
    }
}

impl DirectSurface for &BRepAdaptorSurface {
    fn props_d0(&self, u: f64, v: f64) -> Result<Point3, Failure> {
        self.d0(u, v)
    }
    fn props_d1(&self, u: f64, v: f64) -> Result<GeomSurfaceD1, Failure> {
        self.d1(u, v)
    }
    fn props_d2(&self, u: f64, v: f64) -> Result<GeomSurfaceD2, Failure> {
        self.d2(u, v)
    }
    fn props_bounds(&self) -> Result<PropsBounds, Failure> {
        get_surf_bounds_adaptor(*self)
    }
}
