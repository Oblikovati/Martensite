// ============================================================================
// Weighted Bezier Surface Tests
// ============================================================================

#[test]
fn surface_hasher_bezier_surface_weighted_copied_surfaces_same_hash() {
    let myHasher = SetUp();
    let aPoles = square_poles(1.0);
    let aWeights = net(&[&[1.0, 1.0], &[1.0, 2.0]]);

    let aSurf1 = Handle::new(GeomBezierSurface::with_weights(&aPoles, &aWeights).unwrap());
    let aSurf2: Handle<GeomBezierSurface> = copied(&aSurf1);

    assert_eq!(myHasher.hash(&surface(&aSurf1)), myHasher.hash(&surface(&aSurf2)));
    assert!(myHasher.equal(&surface(&aSurf1), &surface(&aSurf2)));
}

// ============================================================================
// Axis Orientation Tests
// ============================================================================

#[test]
fn surface_hasher_plane_different_normal_different_hash() {
    let myHasher = SetUp();
    let anAxis1 = ax3(pnt(0.0, 0.0, 0.0), dir(0.0, 0.0, 1.0));
    let anAxis2 = ax3(pnt(0.0, 0.0, 0.0), dir(0.0, 1.0, 0.0));
    let aPlane1 = plane(anAxis1);
    let aPlane2 = plane(anAxis2);

    assert_ne!(myHasher.hash(&surface(&aPlane1)), myHasher.hash(&surface(&aPlane2)));
    assert!(!myHasher.equal(&surface(&aPlane1), &surface(&aPlane2)));
}

#[test]
fn surface_hasher_cylindrical_surface_different_axis_different_hash() {
    let myHasher = SetUp();
    let anAxis1 = ax3(pnt(0.0, 0.0, 0.0), dir(0.0, 0.0, 1.0));
    let anAxis2 = ax3(pnt(0.0, 0.0, 0.0), dir(1.0, 0.0, 0.0));
    let aCyl1 = Handle::new(GeomCylindricalSurface::new(anAxis1, 5.0).unwrap());
    let aCyl2 = Handle::new(GeomCylindricalSurface::new(anAxis2, 5.0).unwrap());

    assert_ne!(myHasher.hash(&surface(&aCyl1)), myHasher.hash(&surface(&aCyl2)));
    assert!(!myHasher.equal(&surface(&aCyl1), &surface(&aCyl2)));
}

// ============================================================================
// Revolution with Different Basis Curve Tests
// ============================================================================

#[test]
fn surface_hasher_surface_of_revolution_different_basis_curve_different_comparison() {
    let myHasher = SetUp();
    let aLine1 = line(pnt(1.0, 0.0, 0.0), dir(0.0, 0.0, 1.0));
    let aLine2 = line(pnt(2.0, 0.0, 0.0), dir(0.0, 0.0, 1.0));
    let anAxis = Axis1::new(pnt(0.0, 0.0, 0.0), dir(0.0, 0.0, 1.0));
    let aRev1 = Handle::new(GeomSurfaceOfRevolution::new(&curve(&aLine1), anAxis).unwrap());
    let aRev2 = Handle::new(GeomSurfaceOfRevolution::new(&curve(&aLine2), anAxis).unwrap());

    assert!(!myHasher.equal(&surface(&aRev1), &surface(&aRev2)));
}

// ============================================================================
// Extrusion with Different Direction Tests
// ============================================================================

#[test]
fn surface_hasher_surface_of_linear_extrusion_different_direction_different_hash() {
    let myHasher = SetUp();
    let aLine = line(pnt(0.0, 0.0, 0.0), dir(1.0, 0.0, 0.0));
    let aDir1 = dir(0.0, 0.0, 1.0);
    let aDir2 = dir(0.0, 1.0, 0.0);
    let anExt1 = Handle::new(GeomSurfaceOfLinearExtrusion::new(&curve(&aLine), aDir1).unwrap());
    let anExt2 = Handle::new(GeomSurfaceOfLinearExtrusion::new(&curve(&aLine), aDir2).unwrap());

    assert_ne!(myHasher.hash(&surface(&anExt1)), myHasher.hash(&surface(&anExt2)));
    assert!(!myHasher.equal(&surface(&anExt1), &surface(&anExt2)));
}

// ============================================================================
// BSpline Surface with Different Knots Tests
// ============================================================================

#[test]
fn surface_hasher_b_spline_surface_different_knots_different_comparison() {
    let myHasher = SetUp();
    let aPoles = square_poles(1.0);
    let aUKnots1 = reals(&[0.0, 1.0]);
    // The last U knot differs.
    let aUKnots2 = reals(&[0.0, 2.0]);
    let aVKnots = reals(&[0.0, 1.0]);
    let (aUMults, aVMults) = (integers(&[2, 2]), integers(&[2, 2]));

    let bilinear = |aUKnots: &Array1<'_, f64>| {
        Handle::new(
            GeomBSplineSurface::new(
                &aPoles,
                (aUKnots, &aUMults),
                (&aVKnots, &aVMults),
                (1, 1),
                (false, false),
            )
            .unwrap(),
        )
    };
    let aSurf1 = bilinear(&aUKnots1);
    let aSurf2 = bilinear(&aUKnots2);

    assert!(!myHasher.equal(&surface(&aSurf1), &surface(&aSurf2)));
}

// ============================================================================
// Transformed Surface Tests
// ============================================================================

#[test]
fn surface_hasher_plane_translated_different_hash() {
    let myHasher = SetUp();
    let anAxis = standard_axis();
    let aPlane1 = plane(anAxis);
    let aPlane2: Handle<GeomPlane> = copied(&aPlane1);
    aPlane2.translate(Vector3::new(0.0, 0.0, 1.0)).unwrap();

    assert_ne!(myHasher.hash(&surface(&aPlane1)), myHasher.hash(&surface(&aPlane2)));
    assert!(!myHasher.equal(&surface(&aPlane1), &surface(&aPlane2)));
}

#[test]
fn surface_hasher_sphere_scaled_different_hash() {
    let myHasher = SetUp();
    let anAxis = standard_axis();
    let aSphere1 = Handle::new(GeomSphericalSurface::new(anAxis, 5.0).unwrap());
    let aSphere2: Handle<GeomSphericalSurface> = copied(&aSphere1);
    aSphere2.scale(pnt(0.0, 0.0, 0.0), 2.0).unwrap();

    assert_ne!(myHasher.hash(&surface(&aSphere1)), myHasher.hash(&surface(&aSphere2)));
    assert!(!myHasher.equal(&surface(&aSphere1), &surface(&aSphere2)));
}

// ============================================================================
// UReversed/VReversed Surface Tests
// ============================================================================

#[test]
fn surface_hasher_bezier_surface_u_reversed_different_comparison() {
    let myHasher = SetUp();
    let aPoles = square_poles(1.0);

    let aSurf1 = Handle::new(GeomBezierSurface::new(&aPoles).unwrap());
    let aSurf2: Handle<GeomBezierSurface> = aSurf1.u_reversed().unwrap().cast();

    assert!(!myHasher.equal(&surface(&aSurf1), &surface(&aSurf2)));
}

#[test]
fn surface_hasher_bezier_surface_v_reversed_different_comparison() {
    let myHasher = SetUp();
    let aPoles = square_poles(1.0);

    let aSurf1 = Handle::new(GeomBezierSurface::new(&aPoles).unwrap());
    let aSurf2: Handle<GeomBezierSurface> = aSurf1.v_reversed().unwrap().cast();

    assert!(!myHasher.equal(&surface(&aSurf1), &surface(&aSurf2)));
}

// ============================================================================
// Higher Degree BSpline Surface Tests
// ============================================================================

#[test]
fn surface_hasher_b_spline_surface_higher_degree_copied_surfaces_same_hash() {
    let myHasher = SetUp();
    let mut aPoles: Array2<'static, Point3> = Array2::with_bounds(1, 4, 1, 4).unwrap();
    for i in 1..=4 {
        for j in 1..=4 {
            aPoles.set_value(i, j, pnt(
                f64::from(i - 1) * 1.0,
                f64::from(j - 1) * 1.0,
                f64::from(i + j) * 0.1,
            )).unwrap();
        }
    }
    let (aUKnots, aVKnots) = (reals(&[0.0, 1.0]), reals(&[0.0, 1.0]));
    let (aUMults, aVMults) = (integers(&[4, 4]), integers(&[4, 4]));

    let aSurf1 = Handle::new(
        GeomBSplineSurface::new(
            &aPoles,
            (&aUKnots, &aUMults),
            (&aVKnots, &aVMults),
            (3, 3),
            (false, false),
        )
        .unwrap(),
    );
    let aSurf2: Handle<GeomBSplineSurface> = copied(&aSurf1);

    assert_eq!(myHasher.hash(&surface(&aSurf1)), myHasher.hash(&surface(&aSurf2)));
    assert!(myHasher.equal(&surface(&aSurf1), &surface(&aSurf2)));
}

// ============================================================================
// Cross-type Elementary Surface Tests
// ============================================================================

#[test]
fn surface_hasher_cylinder_vs_cone_different_comparison() {
    let myHasher = SetUp();
    let anAxis = standard_axis();
    let aCylinder = Handle::new(GeomCylindricalSurface::new(anAxis, 5.0).unwrap());
    let aCone = Handle::new(GeomConicalSurface::new(anAxis, PI / 6.0, 5.0).unwrap());

    assert!(!myHasher.equal(&surface(&aCylinder), &surface(&aCone)));
}

#[test]
fn surface_hasher_sphere_vs_torus_different_comparison() {
    let myHasher = SetUp();
    let anAxis = standard_axis();
    let aSphere = Handle::new(GeomSphericalSurface::new(anAxis, 5.0).unwrap());
    let aTorus = Handle::new(GeomToroidalSurface::new(anAxis, 5.0, 1.0).unwrap());

    assert!(!myHasher.equal(&surface(&aSphere), &surface(&aTorus)));
}

// ============================================================================
// Trimmed vs Base Surface Tests
// ============================================================================

#[test]
fn surface_hasher_rectangular_trimmed_surface_vs_base_surface_different_comparison() {
    let myHasher = SetUp();
    let anAxis = standard_axis();
    let aPlane = plane(anAxis);
    let aTrimmed = trimmed(&surface(&aPlane), (0.0, 10.0), (0.0, 10.0));

    assert!(!myHasher.equal(&surface(&aPlane), &surface(&aTrimmed)));
}

// ============================================================================
// Edge Cases - Degenerate/Special Values
// ============================================================================

#[test]
fn surface_hasher_sphere_very_small_radius_copied_spheres_same_hash() {
    let myHasher = SetUp();
    let anAxis = standard_axis();
    let aSphere1 = Handle::new(GeomSphericalSurface::new(anAxis, 1e-10).unwrap());
    let aSphere2: Handle<GeomSphericalSurface> = copied(&aSphere1);

    assert_eq!(myHasher.hash(&surface(&aSphere1)), myHasher.hash(&surface(&aSphere2)));
    assert!(myHasher.equal(&surface(&aSphere1), &surface(&aSphere2)));
}

#[test]
fn surface_hasher_sphere_very_large_radius_copied_spheres_same_hash() {
    let myHasher = SetUp();
    let anAxis = standard_axis();
    let aSphere1 = Handle::new(GeomSphericalSurface::new(anAxis, 1e10).unwrap());
    let aSphere2: Handle<GeomSphericalSurface> = copied(&aSphere1);

    assert_eq!(myHasher.hash(&surface(&aSphere1)), myHasher.hash(&surface(&aSphere2)));
    assert!(myHasher.equal(&surface(&aSphere1), &surface(&aSphere2)));
}

#[test]
fn surface_hasher_plane_at_origin_vs_far_from_origin_different_hash() {
    let myHasher = SetUp();
    let anAxis1 = ax3(pnt(0.0, 0.0, 0.0), dir(0.0, 0.0, 1.0));
    let anAxis2 = ax3(pnt(1e10, 1e10, 1e10), dir(0.0, 0.0, 1.0));
    let aPlane1 = plane(anAxis1);
    let aPlane2 = plane(anAxis2);

    assert_ne!(myHasher.hash(&surface(&aPlane1)), myHasher.hash(&surface(&aPlane2)));
    assert!(!myHasher.equal(&surface(&aPlane1), &surface(&aPlane2)));
}

// ============================================================================
// Revolution with Circle Basis (forms Torus-like)
// ============================================================================

#[test]
fn surface_hasher_surface_of_revolution_circle_basis_copied_surfaces_same_hash() {
    let myHasher = SetUp();
    let aCircleAxis = Axis2::new(pnt(5.0, 0.0, 0.0), dir(0.0, 1.0, 0.0)).unwrap();
    let aCircle = Handle::new(GeomCircle::new(aCircleAxis, 1.0).unwrap());
    let aRevAxis = Axis1::new(pnt(0.0, 0.0, 0.0), dir(0.0, 0.0, 1.0));
    let aRev1 = Handle::new(GeomSurfaceOfRevolution::new(&curve(&aCircle), aRevAxis).unwrap());
    let aRev2: Handle<GeomSurfaceOfRevolution> = copied(&aRev1);

    assert_eq!(myHasher.hash(&surface(&aRev1)), myHasher.hash(&surface(&aRev2)));
    assert!(myHasher.equal(&surface(&aRev1), &surface(&aRev2)));
}

// ============================================================================
// Offset Surface with Different Base
// ============================================================================

#[test]
fn surface_hasher_offset_surface_different_base_surface_different_comparison() {
    let myHasher = SetUp();
    let anAxis1 = ax3(pnt(0.0, 0.0, 0.0), dir(0.0, 0.0, 1.0));
    let anAxis2 = ax3(pnt(1.0, 0.0, 0.0), dir(0.0, 0.0, 1.0));
    let aPlane1 = plane(anAxis1);
    let aPlane2 = plane(anAxis2);
    let anOffset1 = Handle::new(GeomOffsetSurface::new(&surface(&aPlane1), 5.0, false).unwrap());
    let anOffset2 = Handle::new(GeomOffsetSurface::new(&surface(&aPlane2), 5.0, false).unwrap());

    assert!(!myHasher.equal(&surface(&anOffset1), &surface(&anOffset2)));
}
