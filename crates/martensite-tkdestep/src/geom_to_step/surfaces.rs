//! Planes, elementary and swept surfaces, the bounded surface and surface dispatches.
use super::bspline_surfaces::{
    GeomToStepMakeBSplineSurfaceWithKnots,
    GeomToStepMakeBSplineSurfaceWithKnotsAndRationalBSplineSurface,
};
use super::curves::GeomToStepMakeCurve;
use super::placements::{
    GeomToStepMakeAxis1Placement, GeomToStepMakeAxis2Placement3d, GeomToStepMakeVector,
    live_geometry,
};
use super::{empty_step_name, geom_to_step_class};
use crate::step_data::{StepDataFactors, StepDataLogical};
use crate::step_geom::{
    StepGeomBoundedSurface, StepGeomConicalSurface, StepGeomCylindricalSurface,
    StepGeomElementarySurface, StepGeomOffsetSurface, StepGeomPlane,
    StepGeomRectangularTrimmedSurface, StepGeomSphericalSurface, StepGeomSurface,
    StepGeomSurfaceOfLinearExtrusion, StepGeomSurfaceOfRevolution, StepGeomSweptSurface,
    StepGeomToroidalSurface,
};
use martensite_tkernel::standard::{Failure, Handle, exceptions};
use martensite_tkg3d::geom::{
    GeomBSplineSurface, GeomBezierSurface, GeomBoundedSurface, GeomConicalSurface,
    GeomCylindricalSurface, GeomElementarySurface, GeomGeometry, GeomOffsetSurface, GeomPlane,
    GeomRectangularTrimmedSurface, GeomSphericalSurface, GeomSurface, GeomSurfaceOfLinearExtrusion,
    GeomSurfaceOfRevolution, GeomSweptSurface, GeomToroidalSurface,
};
use martensite_tkgeombase::geom_convert;
use martensite_tkmath::gp::{Plane3, Vector3};

geom_to_step_class!(
    /// `GeomToStepMakePlane`: a plane.
    GeomToStepMakePlane,
    "GeomToStepMakePlane",
    StepGeomPlane
);

impl GeomToStepMakePlane {
    /// `GeomToStepMakePlane(gp_Pln, factors)`.
    pub fn from_pln(plane: &Plane3, factors: &StepDataFactors) -> Result<Self, Failure> {
        let entity = StepGeomPlane::new();
        entity.set_position(
            &GeomToStepMakeAxis2Placement3d::from_ax3(&plane.position(), factors)?.value()?,
        );
        entity.set_name(&empty_step_name());
        Ok(Self::done_with(Handle::new(entity)))
    }
    /// `GeomToStepMakePlane(GeomPlane, factors)`.
    pub fn from_geom(
        plane: &Handle<GeomPlane>,
        factors: &StepDataFactors,
    ) -> Result<Self, Failure> {
        Self::from_pln(&live_geometry(plane, "GeomToStepMakePlane")?.pln(), factors)
    }
}

geom_to_step_class!(
    /// `GeomToStepMakeCylindricalSurface`: a cylindrical_surface.
    GeomToStepMakeCylindricalSurface,
    "GeomToStepMakeCylindricalSurface",
    StepGeomCylindricalSurface
);

impl GeomToStepMakeCylindricalSurface {
    /// `GeomToStepMakeCylindricalSurface(GeomCylindricalSurface, factors)`.
    pub fn from_geom(
        surface: &Handle<GeomCylindricalSurface>,
        factors: &StepDataFactors,
    ) -> Result<Self, Failure> {
        let live = live_geometry(surface, "GeomToStepMakeCylindricalSurface")?;
        let position =
            GeomToStepMakeAxis2Placement3d::from_ax3(&live.position(), factors)?.value()?;
        let entity = StepGeomCylindricalSurface::new();
        entity.init(
            &empty_step_name(),
            &position,
            live.radius() / factors.length_factor(),
        );
        Ok(Self::done_with(Handle::new(entity)))
    }
}

geom_to_step_class!(
    /// `GeomToStepMakeConicalSurface`: a conical_surface.
    GeomToStepMakeConicalSurface,
    "GeomToStepMakeConicalSurface",
    StepGeomConicalSurface
);

impl GeomToStepMakeConicalSurface {
    /// `GeomToStepMakeConicalSurface(GeomConicalSurface, factors)`.
    ///
    /// # Errors
    /// a domain failure for a semi-angle outside `[0, pi/2]`, which STEP cannot express.
    pub fn from_geom(
        surface: &Handle<GeomConicalSurface>,
        factors: &StepDataFactors,
    ) -> Result<Self, Failure> {
        let live = live_geometry(surface, "GeomToStepMakeConicalSurface")?;
        let position =
            GeomToStepMakeAxis2Placement3d::from_ax3(&live.position(), factors)?.value()?;
        let semi_angle = live.semi_angle();
        if !(0.0..=std::f64::consts::FRAC_PI_2).contains(&semi_angle) {
            return Err(exceptions::domain_error(&format!(
                "Conicalsurface not STEP conformant: semi-angle {semi_angle} outside [0, pi/2]"
            )));
        }
        let entity = StepGeomConicalSurface::new();
        entity.init(
            &empty_step_name(),
            &position,
            live.ref_radius() / factors.length_factor(),
            semi_angle,
        );
        Ok(Self::done_with(Handle::new(entity)))
    }
}

geom_to_step_class!(
    /// `GeomToStepMakeSphericalSurface`: a spherical_surface.
    GeomToStepMakeSphericalSurface,
    "GeomToStepMakeSphericalSurface",
    StepGeomSphericalSurface
);

impl GeomToStepMakeSphericalSurface {
    /// `GeomToStepMakeSphericalSurface(GeomSphericalSurface, factors)`.
    pub fn from_geom(
        surface: &Handle<GeomSphericalSurface>,
        factors: &StepDataFactors,
    ) -> Result<Self, Failure> {
        let live = live_geometry(surface, "GeomToStepMakeSphericalSurface")?;
        let position =
            GeomToStepMakeAxis2Placement3d::from_ax3(&live.position(), factors)?.value()?;
        let entity = StepGeomSphericalSurface::new();
        entity.init(
            &empty_step_name(),
            &position,
            live.radius() / factors.length_factor(),
        );
        Ok(Self::done_with(Handle::new(entity)))
    }
}

geom_to_step_class!(
    /// `GeomToStepMakeToroidalSurface`: a toroidal_surface.
    GeomToStepMakeToroidalSurface,
    "GeomToStepMakeToroidalSurface",
    StepGeomToroidalSurface
);

impl GeomToStepMakeToroidalSurface {
    /// `GeomToStepMakeToroidalSurface(GeomToroidalSurface, factors)`.
    pub fn from_geom(
        surface: &Handle<GeomToroidalSurface>,
        factors: &StepDataFactors,
    ) -> Result<Self, Failure> {
        let live = live_geometry(surface, "GeomToStepMakeToroidalSurface")?;
        let position =
            GeomToStepMakeAxis2Placement3d::from_ax3(&live.position(), factors)?.value()?;
        let factor = factors.length_factor();
        let entity = StepGeomToroidalSurface::new();
        entity.init(
            &empty_step_name(),
            &position,
            live.major_radius() / factor,
            live.minor_radius() / factor,
        );
        Ok(Self::done_with(Handle::new(entity)))
    }
}

geom_to_step_class!(
    /// `GeomToStepMakeElementarySurface`: the dispatch over the elementary surfaces.
    GeomToStepMakeElementarySurface,
    "GeomToStepMakeElementarySurface",
    StepGeomElementarySurface
);

impl GeomToStepMakeElementarySurface {
    /// `GeomToStepMakeElementarySurface(GeomElementarySurface, factors)`.
    pub fn from_geom(
        surface: &Handle<dyn GeomElementarySurface>,
        factors: &StepDataFactors,
    ) -> Result<Self, Failure> {
        let cylinder = surface.cast::<GeomCylindricalSurface>();
        if cylinder.is_some() {
            return Ok(Self::done_with(
                GeomToStepMakeCylindricalSurface::from_geom(&cylinder, factors)?
                    .value()?
                    .cast(),
            ));
        }
        let cone = surface.cast::<GeomConicalSurface>();
        if cone.is_some() {
            return Ok(Self::done_with(
                GeomToStepMakeConicalSurface::from_geom(&cone, factors)?
                    .value()?
                    .cast(),
            ));
        }
        let sphere = surface.cast::<GeomSphericalSurface>();
        if sphere.is_some() {
            return Ok(Self::done_with(
                GeomToStepMakeSphericalSurface::from_geom(&sphere, factors)?
                    .value()?
                    .cast(),
            ));
        }
        let torus = surface.cast::<GeomToroidalSurface>();
        if torus.is_some() {
            return Ok(Self::done_with(
                GeomToStepMakeToroidalSurface::from_geom(&torus, factors)?
                    .value()?
                    .cast(),
            ));
        }
        let plane = surface.cast::<GeomPlane>();
        if plane.is_some() {
            return Ok(Self::done_with(
                GeomToStepMakePlane::from_geom(&plane, factors)?
                    .value()?
                    .cast(),
            ));
        }
        Ok(Self::not_done())
    }
}

geom_to_step_class!(
    /// `GeomToStepMakeSurfaceOfLinearExtrusion`: an extrusion.
    GeomToStepMakeSurfaceOfLinearExtrusion,
    "GeomToStepMakeSurfaceOfLinearExtrusion",
    StepGeomSurfaceOfLinearExtrusion
);

impl GeomToStepMakeSurfaceOfLinearExtrusion {
    /// `GeomToStepMakeSurfaceOfLinearExtrusion(GeomSurfaceOfLinearExtrusion, factors)`.
    pub fn from_geom(
        surface: &Handle<GeomSurfaceOfLinearExtrusion>,
        factors: &StepDataFactors,
    ) -> Result<Self, Failure> {
        let live = live_geometry(surface, "GeomToStepMakeSurfaceOfLinearExtrusion")?;
        let curve = GeomToStepMakeCurve::from_geom(&live.basis_curve(), factors)?.value()?;
        let axis = Vector3::from(live.direction());
        let vector = GeomToStepMakeVector::from_vec(&axis, factors)?.value()?;
        let entity = StepGeomSurfaceOfLinearExtrusion::new();
        entity.init(&empty_step_name(), &curve, &vector);
        Ok(Self::done_with(Handle::new(entity)))
    }
}

geom_to_step_class!(
    /// `GeomToStepMakeSurfaceOfRevolution`: a revolution.
    GeomToStepMakeSurfaceOfRevolution,
    "GeomToStepMakeSurfaceOfRevolution",
    StepGeomSurfaceOfRevolution
);

impl GeomToStepMakeSurfaceOfRevolution {
    /// `GeomToStepMakeSurfaceOfRevolution(GeomSurfaceOfRevolution, factors)`.
    pub fn from_geom(
        surface: &Handle<GeomSurfaceOfRevolution>,
        factors: &StepDataFactors,
    ) -> Result<Self, Failure> {
        let live = live_geometry(surface, "GeomToStepMakeSurfaceOfRevolution")?;
        let curve = GeomToStepMakeCurve::from_geom(&live.basis_curve(), factors)?.value()?;
        let axis = GeomToStepMakeAxis1Placement::from_ax1(&live.axis(), factors)?.value()?;
        let entity = StepGeomSurfaceOfRevolution::new();
        entity.init(&empty_step_name(), &curve, &axis);
        Ok(Self::done_with(Handle::new(entity)))
    }
}

geom_to_step_class!(
    /// `GeomToStepMakeSweptSurface`: the dispatch over the swept surfaces.
    GeomToStepMakeSweptSurface,
    "GeomToStepMakeSweptSurface",
    StepGeomSweptSurface
);

impl GeomToStepMakeSweptSurface {
    /// `GeomToStepMakeSweptSurface(GeomSweptSurface, factors)`.
    pub fn from_geom(
        surface: &Handle<dyn GeomSweptSurface>,
        factors: &StepDataFactors,
    ) -> Result<Self, Failure> {
        let extrusion = surface.cast::<GeomSurfaceOfLinearExtrusion>();
        if extrusion.is_some() {
            return Ok(Self::done_with(
                GeomToStepMakeSurfaceOfLinearExtrusion::from_geom(&extrusion, factors)?
                    .value()?
                    .cast(),
            ));
        }
        let revolution = surface.cast::<GeomSurfaceOfRevolution>();
        if revolution.is_some() {
            return Ok(Self::done_with(
                GeomToStepMakeSurfaceOfRevolution::from_geom(&revolution, factors)?
                    .value()?
                    .cast(),
            ));
        }
        Ok(Self::not_done())
    }
}

geom_to_step_class!(
    /// `GeomToStepMakeRectangularTrimmedSurface`: a trimmed surface with STEP parameters.
    GeomToStepMakeRectangularTrimmedSurface,
    "GeomToStepMakeRectangularTrimmedSurface",
    StepGeomRectangularTrimmedSurface
);

impl GeomToStepMakeRectangularTrimmedSurface {
    /// The factors from the parameters of a basis surface to the STEP ones: angles in degrees.
    fn parameter_factors(basis: &Handle<dyn GeomSurface>, length: f64) -> (f64, f64) {
        let angle = 180.0 / std::f64::consts::PI;
        if basis.cast::<GeomCylindricalSurface>().is_some() {
            return (angle, 1.0 / length);
        }
        if basis.cast::<GeomSurfaceOfRevolution>().is_some() {
            return (angle, 1.0);
        }
        if basis.cast::<GeomToroidalSurface>().is_some()
            || basis.cast::<GeomSphericalSurface>().is_some()
        {
            return (angle, angle);
        }
        if let Some(cone) = basis.cast::<GeomConicalSurface>().get() {
            return (angle, cone.semi_angle().cos() / length);
        }
        if basis.cast::<GeomPlane>().is_some() {
            return (1.0 / length, 1.0 / length);
        }
        (1.0, 1.0)
    }
    /// `GeomToStepMakeRectangularTrimmedSurface(GeomRectangularTrimmedSurface, factors)`.
    pub fn from_geom(
        surface: &Handle<GeomRectangularTrimmedSurface>,
        factors: &StepDataFactors,
    ) -> Result<Self, Failure> {
        let live = live_geometry(surface, "GeomToStepMakeRectangularTrimmedSurface")?;
        let basis = live.basis_surface();
        let made = GeomToStepMakeSurface::from_geom(&basis, factors)?;
        if !made.is_done() {
            return Ok(Self::not_done());
        }
        let (u1, u2, v1, v2) = live.bounds();
        let (u_factor, v_factor) = Self::parameter_factors(&basis, factors.length_factor());
        let entity = StepGeomRectangularTrimmedSurface::new();
        entity.init(
            &empty_step_name(),
            &made.value()?,
            u1 * u_factor,
            u2 * u_factor,
            v1 * v_factor,
            v2 * v_factor,
            true,
            true,
        );
        Ok(Self::done_with(Handle::new(entity)))
    }
}

geom_to_step_class!(
    /// `GeomToStepMakeBoundedSurface`: the dispatch over the bounded surfaces.
    GeomToStepMakeBoundedSurface,
    "GeomToStepMakeBoundedSurface",
    StepGeomBoundedSurface
);

impl GeomToStepMakeBoundedSurface {
    fn from_spline(
        spline: &Handle<GeomBSplineSurface>,
        factors: &StepDataFactors,
    ) -> Result<Self, Failure> {
        let live = live_geometry(spline, "GeomToStepMakeBoundedSurface")?;
        if live.is_u_rational() || live.is_v_rational() {
            let made = GeomToStepMakeBSplineSurfaceWithKnotsAndRationalBSplineSurface::from_geom(
                spline, factors,
            )?;
            return Ok(Self::done_with(made.value()?.cast()));
        }
        let made = GeomToStepMakeBSplineSurfaceWithKnots::from_geom(spline, factors)?;
        Ok(Self::done_with(made.value()?.cast()))
    }
    /// A periodic spline written unperiodic on a copy; the source is untouched.
    fn unperiodic(
        spline: &Handle<GeomBSplineSurface>,
    ) -> Result<Handle<GeomBSplineSurface>, Failure> {
        let copy = spline.copy()?.cast::<GeomBSplineSurface>();
        let live = live_geometry(&copy, "GeomToStepMakeBoundedSurface")?;
        live.set_u_not_periodic()?;
        live.set_v_not_periodic()?;
        Ok(copy)
    }
    /// `GeomToStepMakeBoundedSurface(GeomBoundedSurface, factors)`.
    pub fn from_geom(
        surface: &Handle<dyn GeomBoundedSurface>,
        factors: &StepDataFactors,
    ) -> Result<Self, Failure> {
        let spline = surface.cast::<GeomBSplineSurface>();
        if let Some(live) = spline.get() {
            if live.is_u_periodic() || live.is_v_periodic() {
                return Self::from_spline(&Self::unperiodic(&spline)?, factors);
            }
            return Self::from_spline(&spline, factors);
        }
        if surface.cast::<GeomBezierSurface>().is_some() {
            return Self::from_spline(
                &geom_convert::surface_to_bspline_surface(&surface.cast())?,
                factors,
            );
        }
        let trimmed = surface.cast::<GeomRectangularTrimmedSurface>();
        if trimmed.is_some() {
            let made = GeomToStepMakeRectangularTrimmedSurface::from_geom(&trimmed, factors)?;
            return Ok(Self::done_with(made.value()?.cast()));
        }
        Ok(Self::not_done())
    }
}

geom_to_step_class!(
    /// `GeomToStepMakeSurface`: the dispatch over every surface.
    GeomToStepMakeSurface,
    "GeomToStepMakeSurface",
    StepGeomSurface
);

impl GeomToStepMakeSurface {
    fn from_offset(offset: &GeomOffsetSurface, factors: &StepDataFactors) -> Result<Self, Failure> {
        let basis = Self::from_geom(&offset.basis_surface(), factors)?;
        if !basis.is_done() {
            return Ok(Self::not_done());
        }
        let entity = StepGeomOffsetSurface::new();
        entity.init(
            &empty_step_name(),
            &basis.value()?,
            offset.offset() / factors.length_factor(),
            StepDataLogical::LFalse,
        );
        Ok(Self::done_with(Handle::new(entity).cast()))
    }
    /// `GeomToStepMakeSurface(GeomSurface, factors)`; another surface is not done.
    pub fn from_geom(
        surface: &Handle<dyn GeomSurface>,
        factors: &StepDataFactors,
    ) -> Result<Self, Failure> {
        let bounded = surface.cast::<dyn GeomBoundedSurface>();
        if bounded.is_some() {
            return Ok(Self::done_with(
                GeomToStepMakeBoundedSurface::from_geom(&bounded, factors)?
                    .value()?
                    .cast(),
            ));
        }
        let elementary = surface.cast::<dyn GeomElementarySurface>();
        if elementary.is_some() {
            return Ok(Self::done_with(
                GeomToStepMakeElementarySurface::from_geom(&elementary, factors)?
                    .value()?
                    .cast(),
            ));
        }
        let swept = surface.cast::<dyn GeomSweptSurface>();
        if swept.is_some() {
            return Ok(Self::done_with(
                GeomToStepMakeSweptSurface::from_geom(&swept, factors)?
                    .value()?
                    .cast(),
            ));
        }
        if let Some(offset) = surface.cast::<GeomOffsetSurface>().get() {
            return Self::from_offset(offset, factors);
        }
        Ok(Self::not_done())
    }
}
