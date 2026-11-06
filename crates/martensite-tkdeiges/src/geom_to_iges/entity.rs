//! `GeomToIgesGeomEntity`: the target model and the unit shared by the geometry translators.
use crate::iges_data::{IgesDataIgesModel, iges_live};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkmath::gp::Xyz;

/// The model the entities are for, and the length of one model unit in the units of the geometry.
///
/// ```
/// use martensite_tkdeiges::geom_to_iges::GeomToIgesGeomEntity;
/// let mut tool = GeomToIgesGeomEntity::new();
/// tool.set_unit(25.4);
/// assert_eq!(tool.get_unit(), 25.4);
/// ```
#[derive(Clone, Default)]
pub struct GeomToIgesGeomEntity {
    model: Handle<IgesDataIgesModel>,
    unit_factor: f64,
}

impl GeomToIgesGeomEntity {
    /// A tool without a model, with a null unit; `GeomToIgesGeomEntity::new()`.
    pub fn new() -> Self {
        Self::default()
    }
    /// Sets the model and takes the unit of its global section; `tool.set_model(&model)?`.
    ///
    /// # Errors
    /// A null object failure for a null model.
    pub fn set_model(&mut self, model: &Handle<IgesDataIgesModel>) -> Result<(), Failure> {
        self.unit_factor = iges_live(model, "GeomToIgesGeomEntity::set_model")?
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
    /// Coordinates in the unit of the model.
    pub(super) fn in_unit(&self, coordinates: Xyz) -> Xyz {
        let unit = self.get_unit();
        Xyz::new(
            coordinates.x() / unit,
            coordinates.y() / unit,
            coordinates.z() / unit,
        )
    }
}
