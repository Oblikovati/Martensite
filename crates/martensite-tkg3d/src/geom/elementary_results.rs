//! Conversions of the ElSLib derivative bundles into the Geom surface results,
//! field for field, as the retained `ElSLib::*D2/D3(..., aResult.Point, ...)` calls fill them.
use super::surface::{GeomSurfaceD2, GeomSurfaceD3};
use martensite_tkmath::elslib::{self, SurfaceD3};

/// The retained D2/D3 result from an ElSLib third-order evaluation.
pub fn surface_d3(values: SurfaceD3) -> GeomSurfaceD3 {
    let second = values.second;
    GeomSurfaceD3 {
        point: second.point,
        d1u: second.du,
        d1v: second.dv,
        d2u: second.duu,
        d2v: second.dvv,
        d2uv: second.duv,
        d3u: values.duuu,
        d3v: values.dvvv,
        d3uuv: values.duuv,
        d3uvv: values.duvv,
    }
}

/// The retained D2 result from an ElSLib second-order evaluation.
pub fn surface_d2(values: elslib::SurfaceD2) -> GeomSurfaceD2 {
    GeomSurfaceD2 {
        point: values.point,
        d1u: values.du,
        d1v: values.dv,
        d2u: values.duu,
        d2v: values.dvv,
        d2uv: values.duv,
    }
}
