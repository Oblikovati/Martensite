//! Elementary, swept, offset and replica surfaces, and the surface dispatch.
use super::StepToGeom;
use crate::step_data::StepDataFactors;
use crate::step_entity::step_live;
use crate::step_geom::{
    StepGeomBoundedSurface, StepGeomConicalSurface, StepGeomCylindricalSurface,
    StepGeomElementarySurface, StepGeomOffsetSurface, StepGeomPlane,
    StepGeomRectangularTrimmedSurface, StepGeomSphericalSurface, StepGeomSurface,
    StepGeomSurfaceOfLinearExtrusion, StepGeomSurfaceOfRevolution, StepGeomSurfaceReplica,
    StepGeomSweptSurface, StepGeomToroidalSurface,
};
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::topods;
use martensite_tkernel::precision;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::geom::{
    GeomCircle, GeomConic, GeomConicalSurface, GeomCurve, GeomCylindricalSurface,
    GeomElementarySurface, GeomEllipse, GeomLine, GeomOffsetSurface, GeomPlane,
    GeomRectangularTrimmedSurface, GeomSphericalSurface, GeomSurface, GeomSurfaceOfLinearExtrusion,
    GeomSurfaceOfRevolution, GeomSweptSurface, GeomToroidalSurface, GeomTrimmedCurve, GeomVector,
};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::geom_abs::Shape as Continuity;
use martensite_tkmath::gp::{Axis1, Axis2, Axis3, Direction3, Line3};
use martensite_tkshhealing::shape_algo::ShapeAlgo;
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeFace, BRepBuilderAPIMakeShape};
use std::f64::consts::PI;

impl StepToGeom {
    /// The frame of a surface position, null when the position cannot be made.
    fn surface_frame(
        position: &Handle<crate::step_geom::StepGeomAxis2Placement3d>,
        factors: &StepDataFactors,
    ) -> Result<Option<Axis3>, Failure> {
        let made = Self::make_axis2_placement(position, factors)?;
        made.get()
            .map(|placement| placement.ax2().map(Axis3::from_axis2))
            .transpose()
    }
    /// `MakePlane`: null when the position cannot be made.
    pub fn make_plane(
        sp: &Handle<StepGeomPlane>,
        factors: &StepDataFactors,
    ) -> Result<Handle<GeomPlane>, Failure> {
        let live = step_live(sp, "StepToGeom::MakePlane")?;
        let Some(frame) = Self::surface_frame(&live.position(), factors)? else {
            return Ok(Handle::default());
        };
        Ok(Handle::new(GeomPlane::new(frame)))
    }
    /// `MakeCylindricalSurface`: null when the position cannot be made.
    pub fn make_cylindrical_surface(
        ss: &Handle<StepGeomCylindricalSurface>,
        factors: &StepDataFactors,
    ) -> Result<Handle<GeomCylindricalSurface>, Failure> {
        let live = step_live(ss, "StepToGeom::MakeCylindricalSurface")?;
        let Some(frame) = Self::surface_frame(&live.position(), factors)? else {
            return Ok(Handle::default());
        };
        Ok(Handle::new(GeomCylindricalSurface::new(
            frame,
            live.radius() * factors.length_factor(),
        )?))
    }
    /// `MakeConicalSurface`: #2(K3-3) rln 12/02/98 ProSTEP ct_turbine-A.stp entity #518, #3571
    /// (gp::Resolution() is too little) for the lower bound of the angle.
    pub fn make_conical_surface(
        ss: &Handle<StepGeomConicalSurface>,
        factors: &StepDataFactors,
    ) -> Result<Handle<GeomConicalSurface>, Failure> {
        let live = step_live(ss, "StepToGeom::MakeConicalSurface")?;
        let Some(frame) = Self::surface_frame(&live.position(), factors)? else {
            return Ok(Handle::default());
        };
        let radius = live.radius() * factors.length_factor();
        let angle = live.semi_angle() * factors.plane_angle_factor();
        Ok(Handle::new(GeomConicalSurface::new(
            frame,
            angle.max(precision::angular()),
            radius,
        )?))
    }
    /// `make_spherical_surface`: null when the position cannot be made.
    pub fn make_spherical_surface(
        ss: &Handle<StepGeomSphericalSurface>,
        factors: &StepDataFactors,
    ) -> Result<Handle<GeomSphericalSurface>, Failure> {
        let live = step_live(ss, "StepToGeom::MakeSphericalSurface")?;
        let Some(frame) = Self::surface_frame(&live.position(), factors)? else {
            return Ok(Handle::default());
        };
        Ok(Handle::new(GeomSphericalSurface::new(
            frame,
            live.radius() * factors.length_factor(),
        )?))
    }
    /// `make_toroidal_surface`: the radii are made positive.
    pub fn make_toroidal_surface(
        ss: &Handle<StepGeomToroidalSurface>,
        factors: &StepDataFactors,
    ) -> Result<Handle<GeomToroidalSurface>, Failure> {
        let live = step_live(ss, "StepToGeom::MakeToroidalSurface")?;
        let Some(frame) = Self::surface_frame(&live.position(), factors)? else {
            return Ok(Handle::default());
        };
        let lf = factors.length_factor();
        let (major, minor) = (
            (live.major_radius() * lf).abs(),
            (live.minor_radius() * lf).abs(),
        );
        Ok(Handle::new(GeomToroidalSurface::new(frame, major, minor)?))
    }
    /// `make_elementary_surface`: null for another surface.
    pub fn make_elementary_surface(
        ss: &Handle<StepGeomElementarySurface>,
        factors: &StepDataFactors,
    ) -> Result<Handle<dyn GeomElementarySurface>, Failure> {
        let plane = ss.cast::<StepGeomPlane>();
        if plane.is_some() {
            return Ok(Self::make_plane(&plane, factors)?.cast());
        }
        let cylinder = ss.cast::<StepGeomCylindricalSurface>();
        if cylinder.is_some() {
            return Ok(Self::make_cylindrical_surface(&cylinder, factors)?.cast());
        }
        let cone = ss.cast::<StepGeomConicalSurface>();
        if cone.is_some() {
            return Ok(Self::make_conical_surface(&cone, factors)?.cast());
        }
        let sphere = ss.cast::<StepGeomSphericalSurface>();
        if sphere.is_some() {
            return Ok(Self::make_spherical_surface(&sphere, factors)?.cast());
        }
        let torus = ss.cast::<StepGeomToroidalSurface>();
        if torus.is_some() {
            return Ok(Self::make_toroidal_surface(&torus, factors)?.cast());
        }
        Ok(Handle::default())
    }
    /// `make_surface_of_linear_extrusion`: sln 23.10.2001. CTS23496: null when the axis cannot be
    /// made; a line extruded along itself is null too.
    pub fn make_surface_of_linear_extrusion(
        ss: &Handle<StepGeomSurfaceOfLinearExtrusion>,
        factors: &StepDataFactors,
    ) -> Result<Handle<GeomSurfaceOfLinearExtrusion>, Failure> {
        let live = step_live(ss, "StepToGeom::MakeSurfaceOfLinearExtrusion")?;
        let curve = Self::make_curve(&live.swept_curve(), factors)?;
        if curve.is_null() {
            return Ok(Handle::default());
        }
        let vector = Self::make_vector_with_magnitude(&live.extrusion_axis(), factors)?;
        let Some(vector) = vector.get() else {
            return Ok(Handle::default());
        };
        let direction = Direction3::from_vector(vector.vec())?;
        if let Some(line) = curve.cast::<GeomLine>().get()
            && line
                .lin()
                .direction()
                .is_parallel(direction, precision::angular())
        {
            return Ok(Handle::default());
        }
        Ok(Handle::new(GeomSurfaceOfLinearExtrusion::new(
            &curve, direction,
        )?))
    }
    /// A full circle or ellipse revolved around a diameter is half of it: skl for bug 952 (one bad
    /// case revolution of circle); `curve` itself otherwise.
    fn revolution_basis(
        curve: Handle<dyn GeomCurve>,
        axis: &Axis1,
    ) -> Result<Handle<dyn GeomCurve>, Failure> {
        let is_circle = curve.cast::<GeomCircle>().is_some();
        if !is_circle && curve.cast::<GeomEllipse>().is_null() {
            return Ok(curve);
        }
        let conic = curve.cast::<dyn GeomConic>();
        let live = step_live(&conic, "StepToGeom::MakeSurfaceOfRevolution")?;
        let center = live.location();
        if Line3::from(*axis).distance(center) >= precision::confusion() {
            return Ok(curve);
        }
        let line_direction = axis.direction();
        let normal = live.axis().direction();
        let x_axis = live.x_axis().direction();
        // checking the axis lies on the plane of the circle
        let on_plane = line_direction.is_normal(normal, precision::angular())
            && (line_direction.is_parallel(x_axis, precision::angular()) || is_circle);
        if !on_plane {
            return Ok(curve);
        }
        // change parametrization for trimming
        live.set_position(Axis2::with_x_direction(
            center,
            normal,
            line_direction.reversed(),
        )?);
        let conic_curve: Handle<dyn GeomCurve> = conic.cast();
        Ok(Handle::new(GeomTrimmedCurve::new(&conic_curve, 0.0, PI, true, true)?).cast())
    }
    /// `make_surface_of_revolution`: null when the curve or the axis cannot be made.
    pub fn make_surface_of_revolution(
        ss: &Handle<StepGeomSurfaceOfRevolution>,
        factors: &StepDataFactors,
    ) -> Result<Handle<GeomSurfaceOfRevolution>, Failure> {
        let live = step_live(ss, "StepToGeom::MakeSurfaceOfRevolution")?;
        let curve = Self::make_curve(&live.swept_curve(), factors)?;
        if curve.is_null() {
            return Ok(Handle::default());
        }
        let placement = Self::make_axis1_placement(&live.axis_position(), factors)?;
        let Some(placement) = placement.get() else {
            return Ok(Handle::default());
        };
        let axis = placement.ax1();
        let basis = Self::revolution_basis(curve, &axis)?;
        Ok(Handle::new(GeomSurfaceOfRevolution::new(&basis, axis)?))
    }
    /// `make_swept_surface`: null for another surface.
    pub fn make_swept_surface(
        ss: &Handle<StepGeomSweptSurface>,
        factors: &StepDataFactors,
    ) -> Result<Handle<dyn GeomSweptSurface>, Failure> {
        let extrusion = ss.cast::<StepGeomSurfaceOfLinearExtrusion>();
        if extrusion.is_some() {
            return Ok(Self::make_surface_of_linear_extrusion(&extrusion, factors)?.cast());
        }
        let revolution = ss.cast::<StepGeomSurfaceOfRevolution>();
        if revolution.is_some() {
            return Ok(Self::make_surface_of_revolution(&revolution, factors)?.cast());
        }
        Ok(Handle::default())
    }
    /// The factors from STEP parameters to the parameters of a basis surface.
    fn trimming_factors(basis: &Handle<dyn GeomSurface>, factors: &StepDataFactors) -> (f64, f64) {
        let length = factors.length_factor();
        // abv 30.06.00 trj4_k1_geo-tc-214.stp #1477: was PI/180.
        let angle = factors.plane_angle_factor();
        if basis.cast::<GeomSphericalSurface>().is_some()
            || basis.cast::<GeomToroidalSurface>().is_some()
        {
            return (angle, angle);
        }
        if basis.cast::<GeomCylindricalSurface>().is_some() {
            return (angle, length);
        }
        if basis.cast::<GeomSurfaceOfRevolution>().is_some() {
            return (angle, 1.0);
        }
        if let Some(cone) = basis.cast::<GeomConicalSurface>().get() {
            return (angle, length / martensite_cmath::cos(cone.semi_angle()));
        }
        if basis.cast::<GeomPlane>().is_some() {
            return (length, length);
        }
        (1.0, 1.0)
    }
    /// `make_rectangular_trimmed_surface`: null when the basis cannot be made.
    pub fn make_rectangular_trimmed_surface(
        ss: &Handle<StepGeomRectangularTrimmedSurface>,
        factors: &StepDataFactors,
    ) -> Result<Handle<GeomRectangularTrimmedSurface>, Failure> {
        let live = step_live(ss, "StepToGeom::MakeRectangularTrimmedSurface")?;
        let basis = Self::make_surface(&live.basis_surface(), factors)?;
        if basis.is_null() {
            return Ok(Handle::default());
        }
        let (u_factor, v_factor) = Self::trimming_factors(&basis, factors);
        let trimmed = GeomRectangularTrimmedSurface::new(
            &basis,
            (live.u1() * u_factor, live.u2() * u_factor),
            (live.v1() * v_factor, live.v2() * v_factor),
            (live.usense(), live.vsense()),
        )?;
        Ok(Handle::new(trimmed))
    }
    /// A C0 basis made C1 so that it can carry an offset, when the conversion gives a face.
    fn smoothed_offset_basis(
        basis: Handle<dyn GeomSurface>,
        offset: f64,
    ) -> Result<Handle<dyn GeomSurface>, Failure> {
        let face = BRepBuilderAPIMakeFace::from_surface(&basis, precision::confusion())?;
        if !face.is_done() {
            return Ok(basis);
        }
        let divided =
            ShapeAlgo::algo_container().c0_shape_to_c1_shape(&face.face()?.into(), offset.abs())?;
        if divided.shape_type()? != ShapeEnum::Face {
            return Ok(basis);
        }
        BRepTool::surface(&topods::face(&divided)?)
    }
    /// An offset_surface: :d4 abv 12 Mar 98; sln 03.10.01. BUC61003: the creation is corrected.
    fn make_offset_surface(
        offset: &StepGeomOffsetSurface,
        factors: &StepDataFactors,
    ) -> Result<Handle<dyn GeomSurface>, Failure> {
        let mut basis = Self::make_surface(&offset.basis_surface(), factors)?;
        let Some(live) = basis.get() else {
            return Ok(Handle::default());
        };
        let distance = offset.distance() * factors.length_factor();
        if live.continuity() == Continuity::C0 {
            basis = Self::smoothed_offset_basis(basis.clone(), distance)?;
        }
        let live = step_live(&basis, "StepToGeom::MakeSurface")?;
        if live.continuity() == Continuity::C0 {
            return Ok(Handle::default());
        }
        Ok(Handle::new(GeomOffsetSurface::new(&basis, distance, false)?).cast())
    }
    /// A surface_replica: :n7 abv 16 Feb 99; protects against cyclic references and a wrong operator.
    fn make_surface_replica(
        replica: &StepGeomSurfaceReplica,
        surface: &Handle<StepGeomSurface>,
        factors: &StepDataFactors,
    ) -> Result<Handle<dyn GeomSurface>, Failure> {
        let parent = replica.parent_surface();
        let operator = replica.transformation();
        if operator.is_null() || parent == *surface {
            return Ok(Handle::default());
        }
        let made = Self::make_surface(&parent, factors)?;
        let Some(live) = made.get() else {
            return Ok(Handle::default());
        };
        let Some(transform) = Self::make_transformation_3d(&operator, factors)? else {
            return Ok(Handle::default());
        };
        live.transform(transform)?;
        Ok(made)
    }
    /// The conversion of a surface; the members raise and `make_surface` catches them.
    fn make_surface_guarded(
        ss: &Handle<StepGeomSurface>,
        factors: &StepDataFactors,
    ) -> Result<Handle<dyn GeomSurface>, Failure> {
        let bounded = ss.cast::<StepGeomBoundedSurface>();
        if bounded.is_some() {
            return Ok(Self::make_bounded_surface(&bounded, factors)?.cast());
        }
        let elementary = ss.cast::<StepGeomElementarySurface>();
        if let Some(live) = elementary.get() {
            if live.position().is_null() {
                return Ok(Handle::default());
            }
            return Ok(Self::make_elementary_surface(&elementary, factors)?.cast());
        }
        let swept = ss.cast::<StepGeomSweptSurface>();
        if swept.is_some() {
            return Ok(Self::make_swept_surface(&swept, factors)?.cast());
        }
        if let Some(offset) = ss.cast::<StepGeomOffsetSurface>().get() {
            return Self::make_offset_surface(offset, factors);
        }
        if let Some(replica) = ss.cast::<StepGeomSurfaceReplica>().get() {
            return Self::make_surface_replica(replica, ss, factors);
        }
        Ok(Handle::default())
    }
    /// `make_surface`: sln 01.10.2001 BUC61003: null for a null entity. A failure inside the
    /// conversion gives a null handle, as the `catch (a failure)` does.
    pub fn make_surface(
        ss: &Handle<StepGeomSurface>,
        factors: &StepDataFactors,
    ) -> Result<Handle<dyn GeomSurface>, Failure> {
        if ss.is_null() {
            return Ok(Handle::default());
        }
        Ok(Self::make_surface_guarded(ss, factors).unwrap_or_default())
    }
}
