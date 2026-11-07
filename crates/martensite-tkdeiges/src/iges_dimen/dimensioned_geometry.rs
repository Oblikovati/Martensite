//! IgesDimenDimensionedGeometry.
#![allow(
    unused_mut,
    unused_variables,
    unused_assignments,
    clippy::needless_late_init,
    clippy::collapsible_if,
    clippy::collapsible_match,
    clippy::manual_range_contains,
    clippy::too_many_arguments,
    clippy::collapsible_else_if,
    clippy::needless_return,
    reason = "the members keep the order and the locals of the record they read"
)]
use crate::iges_basic::entity_shape::{iges_array1_length, iges_array1_lower};
use crate::iges_data::{IgesDataIgesEntity, iges_array1_item};
use crate::{iges_entity, iges_methods};
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, exceptions};

iges_entity! {
/// Defines IGES Dimensioned Geometry, Type `402` Form `13`,
/// This entity has been replaced by the new form of Dimensioned
/// Geometry Associativity Entity (Type 402, Form 21) and should no
/// longer be used by preprocessors.
IgesDimenDimensionedGeometry, IgesDimenDimensionedGeometryState, "IgesDimenDimensionedGeometry", parent IgesDataIgesEntity, {
the_nb_dimensions: i32,
the_dimension: Handle<IgesDataIgesEntity>,
the_geometry_entities: Handle<HArray1<Handle<IgesDataIgesEntity>>>,
}
}

impl IgesDimenDimensionedGeometry {
    iges_methods! {
    get nb_dimensions = the_nb_dimensions: i32;
    get dimension_entity = the_dimension: Handle<IgesDataIgesEntity>;
    }
    /// `init`.
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn init(
        &self,
        nb_dims: i32,
        a_dimension: &Handle<IgesDataIgesEntity>,
        entities: &Handle<HArray1<Handle<IgesDataIgesEntity>>>,
    ) -> Result<(), Failure> {
        if iges_array1_lower(entities, "IgesDimenDimensionedGeometry::init")? != 1 {
            return Err(exceptions::dimension_mismatch(
                "IgesDimenDimensionedGeometry : Init",
            ));
        }
        {
            let mut state = self.state.write();
            state.the_nb_dimensions = nb_dims;
            state.the_dimension = a_dimension.clone();
            state.the_geometry_entities = entities.clone();
        }
        self.init_type_and_form(402, 13);
        Ok(())
    }
    /// returns the number of associated geometry entities
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn nb_geometry_entities(&self) -> Result<i32, Failure> {
        iges_array1_length(
            &self.state.read().the_geometry_entities,
            "IgesDimenDimensionedGeometry::nb_geometry_entities",
        )
    }
    /// returns the num'th Geometry entity
    /// raises exception if Index at most 0 or Index &gt; NbGeometryEntities()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn geometry_entity(&self, num: i32) -> Result<Handle<IgesDataIgesEntity>, Failure> {
        iges_array1_item(
            &self.state.read().the_geometry_entities,
            num,
            "IgesDimenDimensionedGeometry::geometry_entity",
        )
    }
}
