//! Translation of curves in the plane into IGES curves.
use crate::geom_to_iges::GeomToIgesGeomCurve;
use crate::iges_data::{IgesDataIgesEntity, IgesDataIgesModel, iges_live};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg2d::geom2d::Geom2dCurve;
use martensite_tkgeomalgo::geom_api;
use martensite_tkmath::gp::{Axis3, Plane3};
use std::ops::{Deref, DerefMut};

/// The model the entities are for, and the unit of its global section.
///
/// ```
/// use martensite_tkdeiges::geom2d_to_iges::Geom2dToIgesGeom2dEntity;
/// let tool = Geom2dToIgesGeom2dEntity::new();
/// assert_eq!(tool.get_unit(), 0.0);
/// ```
#[derive(Clone, Default)]
pub struct Geom2dToIgesGeom2dEntity {
    model: Handle<IgesDataIgesModel>,
    unit_factor: f64,
}

impl Geom2dToIgesGeom2dEntity {
    /// A tool without a model and with a null unit; `Geom2dToIgesGeom2dEntity::new()`.
    pub fn new() -> Self {
        Self::default()
    }
    /// Sets the model and takes the unit of its global section; `tool.set_model(&model)?`.
    ///
    /// # Errors
    /// A null object failure for a null model.
    pub fn set_model(&mut self, model: &Handle<IgesDataIgesModel>) -> Result<(), Failure> {
        self.unit_factor = iges_live(model, "Geom2dToIgesGeom2dEntity::set_model")?
            .global_section()
            .unit_value();
        self.model = model.clone();
        Ok(())
    }
    /// The model; `tool.get_model()`.
    pub fn get_model(&self) -> Handle<IgesDataIgesModel> {
        self.model.clone()
    }
    /// Sets the unit; `tool.set_unit(1.0)`.
    pub fn set_unit(&mut self, unit: f64) {
        self.unit_factor = unit;
    }
    /// The unit; `tool.get_unit()`.
    pub fn get_unit(&self) -> f64 {
        self.unit_factor
    }
}

/// Translates curves in the plane through the translation of their image in the plane Z = 0.
///
/// ```
/// use martensite_tkdeiges::geom2d_to_iges::Geom2dToIgesGeom2dCurve;
/// use martensite_tkernel::standard::Handle;
/// use martensite_tkg2d::geom2d::Geom2dCurve;
/// let tool = Geom2dToIgesGeom2dCurve::new();
/// let none: Handle<dyn Geom2dCurve> = Handle::default();
/// assert!(tool.transfer_2d_curve(&none, 0.0, 1.0).unwrap().is_null());
/// ```
#[derive(Clone, Default)]
pub struct Geom2dToIgesGeom2dCurve {
    base: Geom2dToIgesGeom2dEntity,
}

impl Deref for Geom2dToIgesGeom2dCurve {
    type Target = Geom2dToIgesGeom2dEntity;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for Geom2dToIgesGeom2dCurve {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl Geom2dToIgesGeom2dCurve {
    /// A tool at the initial state; `Geom2dToIgesGeom2dCurve::new()`.
    pub fn new() -> Self {
        Self::default()
    }
    /// A tool with the model and the unit of `base`; `Geom2dToIgesGeom2dCurve::with_entity(&base)`.
    pub fn with_entity(base: &Geom2dToIgesGeom2dEntity) -> Self {
        Self { base: base.clone() }
    }
    /// The entity of a curve between two parameters, not scaled: the plane has no length unit;
    /// `tool.transfer_2d_curve(&curve, 0.0, 1.0)?`.
    ///
    /// # Errors
    /// A failure of the transfer of the image of the curve.
    pub fn transfer_2d_curve(
        &self,
        start: &Handle<dyn Geom2dCurve>,
        first: f64,
        last: f64,
    ) -> Result<Handle<IgesDataIgesEntity>, Failure> {
        if start.is_null() {
            return Ok(Handle::default());
        }
        let mut tool = GeomToIgesGeomCurve::new();
        tool.set_model(&self.get_model())?;
        tool.set_unit(1.0);
        let plane = Plane3::from_position(Axis3::default());
        tool.transfer_curve(&geom_api::to_3d(start, &plane)?, first, last)
    }
}
