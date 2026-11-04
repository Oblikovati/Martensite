use super::*;
use martensite_tkg3d::{adaptor3d::Adaptor3dSurface, geom_adaptor::GeomAdaptorSurface};
use martensite_tkmath::poly::{Triangle, Triangulation};

pub(super) fn sizes(nbu: i32, nbv: i32) -> Result<(i32, i32), Failure> {
    let nodes = nbu
        .checked_add(1)
        .and_then(|u| nbv.checked_add(1).and_then(|v| u.checked_mul(v)));
    let triangles = nbu.checked_mul(nbv).and_then(|n| n.checked_mul(2));
    if nbu <= 0 || nbv <= 0 || nodes.is_none() || triangles.is_none() {
        return Err(program_error(&format!(
            "tessellate nbu={nbu}, nbv={nbv}; expected positive subdivisions with representable i32 counts"
        )));
    }
    Ok((nodes.unwrap(), triangles.unwrap()))
}

pub(super) fn triangulate(
    surface: &Handle<dyn GeomSurface>,
    bounds: Bounds,
    divisions: (i32, i32),
    sizes: (i32, i32),
) -> Result<Handle<Triangulation>, Failure> {
    let mesh = Triangulation::with_sizes(sizes.0, sizes.1, false, false)?;
    nodes(
        &mesh,
        &GeomAdaptorSurface::with_surface(surface)?,
        bounds,
        divisions,
    )?;
    triangles(&mesh, divisions)?;
    Ok(Handle::new(mesh))
}

fn nodes(
    mesh: &Triangulation,
    surface: &GeomAdaptorSurface,
    (umin, umax, vmin, vmax): Bounds,
    (nbu, nbv): (i32, i32),
) -> Result<(), Failure> {
    let du = (umax - umin) / f64::from(nbu);
    let dv = (vmax - vmin) / f64::from(nbv);
    for u in 0..=nbu {
        for v in 0..=nbv {
            mesh.set_node(
                1 + u * (nbv + 1) + v,
                surface.value(umin + f64::from(u) * du, vmin + f64::from(v) * dv)?,
            )?;
        }
    }
    Ok(())
}

fn triangles(mesh: &Triangulation, (nbu, nbv): (i32, i32)) -> Result<(), Failure> {
    let mut index = 1;
    for u in 0..nbu {
        for v in 0..nbv {
            let base = 1 + u * (nbv + 1) + v;
            mesh.set_triangle(index, Triangle::new(base, base + nbv + 2, base + 1))?;
            mesh.set_triangle(
                index + 1,
                Triangle::new(base, base + nbv + 1, base + nbv + 2),
            )?;
            index += 2;
        }
    }
    Ok(())
}
