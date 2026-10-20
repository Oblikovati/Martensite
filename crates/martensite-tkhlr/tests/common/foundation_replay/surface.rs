//! Surface-normal evaluation against Contap_SurfProps.
//! Shared fixtures keep original report ordering over request-local tokens.
use super::common::{Sink, attempt, end, pnt, show, xyz};
use super::geometry_records::{common::Reader, contour_surface_fixture};
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::adaptor3d::h_surface_tool::Adaptor3dSurfaceHandle;
use martensite_tkhlr::contap::ContapSurfProps;

pub(super) fn surface_adaptor(reader: &mut Reader) -> Result<Adaptor3dSurfaceHandle, Failure> {
    let kind = reader.integer();
    let frame =
        super::contour_frame::from_fields(reader.pnt(), reader.dir(), reader.dir(), || {
            reader.integer() != 0
        })?;
    let (radius, semi) = (reader.real(), reader.real());
    contour_surface_fixture::from_fields(kind, frame, radius, semi, || {
        (reader.real(), reader.real(), reader.real(), reader.real())
    })
}

/// Compare each native normal overload, including singular cones and generic patches.
/// `properties_case(tokens, values)`.
pub fn properties_case(tokens: &mut Sink, values: &[f64]) {
    let mut reader = Reader::new(values);
    attempt(tokens, |tokens| {
        let surface = surface_adaptor(&mut reader)?;
        let (u, v) = (reader.real(), reader.real());
        surface_properties(tokens, &surface, u, v);
        Ok(())
    });
    end(tokens);
}

fn surface_properties(tokens: &mut Sink, surface: &Adaptor3dSurfaceHandle, u: f64, v: f64) {
    show(
        tokens,
        ContapSurfProps::normale(surface, u, v),
        |tokens, (p, n)| {
            pnt(tokens, p);
            xyz(tokens, *n.xyz());
        },
    );
    first_derivatives(tokens, surface, u, v);
    normal_derivatives(tokens, surface, u, v);
}

fn first_derivatives(tokens: &mut Sink, surface: &Adaptor3dSurfaceHandle, u: f64, v: f64) {
    show(
        tokens,
        ContapSurfProps::deriv_and_norm(surface, u, v),
        |tokens, (p, du, dv, n)| {
            pnt(tokens, p);
            xyz(tokens, *du.xyz());
            xyz(tokens, *dv.xyz());
            xyz(tokens, *n.xyz());
        },
    );
}

fn normal_derivatives(tokens: &mut Sink, surface: &Adaptor3dSurfaceHandle, u: f64, v: f64) {
    show(
        tokens,
        ContapSurfProps::norm_and_dn(surface, u, v),
        |tokens, (p, n, du, dv)| {
            pnt(tokens, p);
            xyz(tokens, *n.xyz());
            xyz(tokens, *du.xyz());
            xyz(tokens, *dv.xyz());
        },
    );
}
