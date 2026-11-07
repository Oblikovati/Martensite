//! Swept surfaces: the tabulated cylinder (type 122) and the surface of revolution (type 120).
use super::curve::EntityResult;
use super::surface::SurfaceRange;
use super::{GeomToIgesGeomCurve, GeomToIgesGeomSurface, bounded_range};
use crate::iges_data::iges_live;
use crate::iges_geom::{IgesGeomLine, IgesGeomSurfaceOfRevolution, IgesGeomTabulatedCylinder};
use martensite_tkernel::precision;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::geom::{
    GeomCurve, GeomLine, GeomSurface, GeomSurfaceOfLinearExtrusion, GeomSurfaceOfRevolution,
    GeomSweptSurface, GeomTrimmedCurve,
};
use martensite_tkmath::gp::Xyz;
use std::f64::consts::PI;

const WHERE: &str = "GeomToIgesGeomSurface";

/// The curve without the trims around it.
fn untrimmed(curve: &Handle<dyn GeomCurve>) -> Handle<dyn GeomCurve> {
    let mut current = curve.clone();
    loop {
        let trimmed = current.cast::<GeomTrimmedCurve>();
        match trimmed.get() {
            Some(live) => current = live.basis_curve(),
            None => return current,
        }
    }
}

impl GeomToIgesGeomSurface {
    /// The tabulated cylinder over the whole U range of the surface; the end of the generatrix is
    /// the point at the last V, and the directrix starts at the first V;
    /// `tool.transfer_extrusion(&surface, range)?`.
    ///
    /// # Errors
    /// A failure of the evaluation of the surface or of the transfer of the directrix.
    pub fn transfer_extrusion(
        &mut self,
        start: &Handle<GeomSurfaceOfLinearExtrusion>,
        (_, _, first_v, last_v): SurfaceRange,
    ) -> EntityResult {
        self.length = 1.0;
        let Some(live) = start.get() else {
            return Ok(Handle::default());
        };
        let (v1, v2) = bounded_range(first_v, last_v);
        let bounds = live.bounds();
        let u1 = bounds.0;
        let end = self.in_unit(live.value(u1, v2)?.xyz());
        let curve = live.basis_curve();
        let directrix_curve = if v1.abs() > precision::confusion() {
            let moved = iges_live(&curve, WHERE)?
                .translated_between(live.value(u1, 0.0)?, live.value(u1, v1)?)?;
            moved.cast()
        } else {
            curve
        };
        let directrix = GeomToIgesGeomCurve::with_entity(&self.base).transfer_curve(
            &directrix_curve,
            u1,
            bounds.1,
        )?;
        let entity = IgesGeomTabulatedCylinder::new();
        entity.init(&directrix, end);
        Ok(Handle::new(entity).cast())
    }
    /// The surface of revolution, with an IGES axis against the axis of the surface;
    /// `tool.transfer_revolution(&surface, range)?`.
    ///
    /// # Errors
    /// A failure of the evaluation of the surface or of the transfer of the generatrix.
    pub fn transfer_revolution(
        &mut self,
        start: &Handle<GeomSurfaceOfRevolution>,
        (u1, u2, first_v, last_v): SurfaceRange,
    ) -> EntityResult {
        self.length = 1.0;
        let Some(live) = start.get() else {
            return Ok(Handle::default());
        };
        let (v1, v2) = bounded_range(first_v, last_v);
        let curve = live.basis_curve();
        let generatrix =
            GeomToIgesGeomCurve::with_entity(&self.base).transfer_curve(&curve, v1, v2)?;
        self.record_line_length(&untrimmed(&curve), v1, v2)?;
        let entity = IgesGeomSurfaceOfRevolution::new();
        entity.init(
            &self.revolution_axis(live),
            &generatrix,
            2.0 * PI - u2,
            2.0 * PI - u1,
        );
        Ok(Handle::new(entity).cast())
    }
    fn record_line_length(
        &mut self,
        curve: &Handle<dyn GeomCurve>,
        v1: f64,
        v2: f64,
    ) -> Result<(), Failure> {
        let line = curve.cast::<GeomLine>();
        if let Some(live) = line.get() {
            self.length = live.value(v1)?.distance(&live.value(v2)?);
        }
        Ok(())
    }
    fn revolution_axis(&self, surface: &GeomSurfaceOfRevolution) -> Handle<IgesGeomLine> {
        let axis = surface.axis();
        let location = axis.location().xyz();
        let direction = axis.direction().xyz();
        // The IGES axis ends behind the location. Only the direction part is out of scale, as written.
        let end = Xyz::new(
            location.x() - direction.x(),
            location.y() - direction.y(),
            location.z() - direction.z(),
        );
        let entity = IgesGeomLine::new();
        entity.init(self.in_unit(location), self.in_unit(end));
        Handle::new(entity)
    }
}
