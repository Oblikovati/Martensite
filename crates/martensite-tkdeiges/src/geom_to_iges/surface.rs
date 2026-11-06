//! `GeomToIgesGeomSurface`: surfaces into IGES surfaces. This file holds the choice among the kinds.
use super::GeomToIgesGeomEntity;
use super::curve::EntityResult;
use crate::iges_data::iges_live;
use crate::iges_geom::IgesGeomOffsetSurface;
use martensite_tkernel::precision;
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::geom::{
    GeomBSplineSurface, GeomBezierSurface, GeomConicalSurface, GeomCylindricalSurface,
    GeomOffsetSurface, GeomPlane, GeomRectangularTrimmedSurface, GeomSphericalSurface, GeomSurface,
    GeomSurfaceOfLinearExtrusion, GeomSurfaceOfRevolution, GeomToroidalSurface,
};
use martensite_tkgeombase::geom_lprop::SLProps;
use std::ops::{Deref, DerefMut};

/// A range in both parameters: first and last U, first and last V.
pub(super) type SurfaceRange = (f64, f64, f64, f64);

/// Translates surfaces.
///
/// ```
/// use martensite_tkdeiges::geom_to_iges::GeomToIgesGeomSurface;
/// let mut tool = GeomToIgesGeomSurface::new();
/// tool.set_b_rep_mode(true);
/// assert!(tool.get_b_rep_mode() && !tool.get_analytic_mode());
/// ```
#[derive(Clone, Default)]
pub struct GeomToIgesGeomSurface {
    pub(super) base: GeomToIgesGeomEntity,
    pub(super) length: f64,
    pub(super) b_rep_mode: bool,
    pub(super) analytic: bool,
}

impl Deref for GeomToIgesGeomSurface {
    type Target = GeomToIgesGeomEntity;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for GeomToIgesGeomSurface {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl GeomToIgesGeomSurface {
    /// A tool at the initial state, in neither mode; `GeomToIgesGeomSurface::new()`.
    pub fn new() -> Self {
        Self::default()
    }
    /// A tool with the model and the unit of `base`; `GeomToIgesGeomSurface::with_entity(&base)`.
    pub fn with_entity(base: &GeomToIgesGeomEntity) -> Self {
        Self {
            base: base.clone(),
            ..Self::default()
        }
    }
    /// The length of the generatrix of the last translation; `tool.length()`.
    pub fn length(&self) -> f64 {
        self.length
    }
    /// Whether planes and analytic surfaces are written as solid surfaces; `tool.get_b_rep_mode()`.
    pub fn get_b_rep_mode(&self) -> bool {
        self.b_rep_mode
    }
    /// Sets that mode; `tool.set_b_rep_mode(true)`.
    pub fn set_b_rep_mode(&mut self, flag: bool) {
        self.b_rep_mode = flag;
    }
    /// Whether elementary surfaces are written as analytic ones; `tool.get_analytic_mode()`.
    pub fn get_analytic_mode(&self) -> bool {
        self.analytic
    }
    /// Sets that mode; `tool.set_analytic_mode(true)`.
    pub fn set_analytic_mode(&mut self, flag: bool) {
        self.analytic = flag;
    }
    /// The entity of any surface on a range; null for a null surface or an unknown kind;
    /// `tool.transfer_surface(&surface, (0.0, 1.0, 0.0, 1.0))?`.
    ///
    /// # Errors
    /// A failure of the evaluation or the conversion of the surface.
    pub fn transfer_surface(
        &mut self,
        start: &Handle<dyn GeomSurface>,
        range: SurfaceRange,
    ) -> EntityResult {
        let bspline = start.cast::<GeomBSplineSurface>();
        if bspline.is_some() {
            return self.transfer_bspline_surface(&bspline, range);
        }
        let bezier = start.cast::<GeomBezierSurface>();
        if bezier.is_some() {
            return self.transfer_bezier_surface(&bezier);
        }
        let trimmed = start.cast::<GeomRectangularTrimmedSurface>();
        if trimmed.is_some() {
            return self.transfer_trimmed_surface(&trimmed, range);
        }
        if let Some(entity) = self.transfer_elementary(start, range) {
            return entity;
        }
        self.transfer_swept_or_offset(start, range)
    }
    fn transfer_elementary(
        &mut self,
        start: &Handle<dyn GeomSurface>,
        range: SurfaceRange,
    ) -> Option<EntityResult> {
        let plane = start.cast::<GeomPlane>();
        if plane.is_some() {
            return Some(if self.b_rep_mode {
                self.transfer_plane_surface(&plane)
            } else {
                self.transfer_plane(&plane, range)
            });
        }
        let as_solid = self.b_rep_mode && self.analytic;
        let cylinder = start.cast::<GeomCylindricalSurface>();
        if cylinder.is_some() {
            return Some(if as_solid {
                self.transfer_cylindrical_surface(&cylinder)
            } else {
                self.transfer_cylinder(&cylinder, range)
            });
        }
        let cone = start.cast::<GeomConicalSurface>();
        if cone.is_some() {
            return Some(if as_solid {
                self.transfer_conical_surface(&cone)
            } else {
                self.transfer_cone(&cone, range)
            });
        }
        self.transfer_round_elementary(start, range, as_solid)
    }
    fn transfer_round_elementary(
        &mut self,
        start: &Handle<dyn GeomSurface>,
        range: SurfaceRange,
        as_solid: bool,
    ) -> Option<EntityResult> {
        let sphere = start.cast::<GeomSphericalSurface>();
        if sphere.is_some() {
            return Some(if as_solid {
                self.transfer_spherical_surface(&sphere)
            } else {
                self.transfer_sphere(&sphere, range)
            });
        }
        let torus = start.cast::<GeomToroidalSurface>();
        if torus.is_some() {
            return Some(if as_solid {
                self.transfer_toroidal_surface(&torus)
            } else {
                self.transfer_torus(&torus, range)
            });
        }
        None
    }
    fn transfer_swept_or_offset(
        &mut self,
        start: &Handle<dyn GeomSurface>,
        range: SurfaceRange,
    ) -> EntityResult {
        let extrusion = start.cast::<GeomSurfaceOfLinearExtrusion>();
        if extrusion.is_some() {
            return self.transfer_extrusion(&extrusion, range);
        }
        let revolution = start.cast::<GeomSurfaceOfRevolution>();
        if revolution.is_some() {
            return self.transfer_revolution(&revolution, range);
        }
        self.transfer_offset_surface(&start.cast(), range)
    }
    /// The entity of the basis of a trimmed surface on the given range, null for a trimmed basis,
    /// which has no translation; `tool.transfer_trimmed_surface(&surface, range)?`.
    ///
    /// # Errors
    /// A failure of the transfer of the basis.
    pub fn transfer_trimmed_surface(
        &mut self,
        start: &Handle<GeomRectangularTrimmedSurface>,
        range: SurfaceRange,
    ) -> EntityResult {
        let Some(live) = start.get() else {
            return Ok(Handle::default());
        };
        let basis = live.basis_surface();
        if basis.cast::<GeomRectangularTrimmedSurface>().is_some() {
            return Ok(Handle::default());
        }
        self.transfer_surface(&basis, range)
    }
    /// The offset surface (type 140) with the normal in the middle of the basis as indicator;
    /// `tool.transfer_offset_surface(&surface, range)?`.
    ///
    /// # Errors
    /// A failure of the transfer of the basis or of its normal.
    pub fn transfer_offset_surface(
        &mut self,
        start: &Handle<GeomOffsetSurface>,
        range: SurfaceRange,
    ) -> EntityResult {
        let Some(live) = start.get() else {
            return Ok(Handle::default());
        };
        let basis = live.basis_surface();
        let bounds = live.bounds();
        let surface = self.transfer_surface(&basis, range)?;
        let middle = ((bounds.0 + bounds.1) / 2.0, (bounds.2 + bounds.3) / 2.0);
        iges_live(&basis, "GeomToIgesGeomSurface")?;
        let mut props =
            SLProps::with_parameters(basis, middle.0, middle.1, 1, precision::confusion())?;
        let indicator = self.in_unit(props.normal()?.xyz());
        let entity = IgesGeomOffsetSurface::new();
        entity.init(indicator, live.offset() / self.get_unit(), &surface);
        Ok(Handle::new(entity).cast())
    }
}
