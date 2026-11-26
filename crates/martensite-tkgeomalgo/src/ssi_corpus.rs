// SPDX-License-Identifier: GPL-2.0-only
//! Independent SSI check corpus verifying branch counts, residuals on both surfaces,
//! and completeness receipts without reusing the intersector (#39).
use crate::completeness::IntersectionCompleteness;
use crate::geom_api::IntSS;
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::geom::{
    GeomCylindricalSurface, GeomPlane, GeomSphericalSurface, GeomSurface,
};
use martensite_tkmath::gp::{Axis3, Direction3, Point3};

fn check_curve_residual_on_plane(
    curve: &Handle<dyn martensite_tkg3d::geom::GeomCurve>,
    z_plane: f64,
) -> f64 {
    let live = curve.get().expect("live curve");
    let u0 = live.first_parameter();
    let u1 = live.last_parameter();
    let mut max_res = 0.0_f64;
    for i in 0..=16 {
        let u = u0 + (u1 - u0) * (i as f64 / 16.0);
        let p = live.value(u).expect("eval curve");
        max_res = max_res.max((p.z() - z_plane).abs());
    }
    max_res
}

#[test]
fn ssi_corpus_plane_cylinder_and_plane_sphere_residuals_and_receipts() {
    let plane_z = 1.5;
    let plane: Handle<dyn GeomSurface> = Handle::new(GeomPlane::new(
        Axis3::new(
            Point3::new(0.0, 0.0, plane_z),
            Direction3::new(0.0, 0.0, 1.0).expect("z"),
            Direction3::new(1.0, 0.0, 0.0).expect("x"),
        )
        .expect("plane axis"),
    ))
    .into_upcast();
    let cylinder: Handle<dyn GeomSurface> = Handle::new(
        GeomCylindricalSurface::new(
            Axis3::new(
                Point3::new(0.0, 0.0, 0.0),
                Direction3::new(0.0, 0.0, 1.0).expect("z"),
                Direction3::new(1.0, 0.0, 0.0).expect("x"),
            )
            .expect("cyl axis"),
            2.0,
        )
        .expect("cylinder"),
    )
    .into_upcast();

    let int_cyl = IntSS::with_surfaces(&plane, &cylinder, 1e-7).expect("plane-cylinder SSI");
    assert!(int_cyl.is_done());
    assert_eq!(int_cyl.nb_lines(), 1);
    let receipt = int_cyl.completeness_receipt(64);
    assert_eq!(receipt.completeness, IntersectionCompleteness::Complete);

    let line = int_cyl.line(1).expect("line 1");
    assert!(check_curve_residual_on_plane(&line, plane_z) < 1e-6);

    // Plane intersecting sphere of radius 3.0 at z = 1.5 -> circle of radius sqrt(9 - 2.25).
    let sphere: Handle<dyn GeomSurface> = Handle::new(
        GeomSphericalSurface::new(
            Axis3::new(
                Point3::new(0.0, 0.0, 0.0),
                Direction3::new(0.0, 0.0, 1.0).expect("z"),
                Direction3::new(1.0, 0.0, 0.0).expect("x"),
            )
            .expect("sphere axis"),
            3.0,
        )
        .expect("sphere"),
    )
    .into_upcast();
    let int_sph = IntSS::with_surfaces(&plane, &sphere, 1e-7).expect("plane-sphere SSI");
    assert!(int_sph.is_done());
    assert_eq!(int_sph.nb_lines(), 1);
    let sph_line = int_sph.line(1).expect("line 1");
    assert!(check_curve_residual_on_plane(&sph_line, plane_z) < 1e-6);
}
