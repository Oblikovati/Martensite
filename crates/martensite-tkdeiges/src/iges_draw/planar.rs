//! IgesDrawPlanar.
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
use crate::iges_geom::IgesGeomTransformationMatrix;
use crate::{iges_entity, iges_methods};
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, exceptions};

iges_entity! {
/// defines IGESPlanar, Type `402` Form `16`
///
/// Indicates that a collection of entities is coplanar.The
/// entities may be geometric, annotative, and/or structural.
IgesDrawPlanar, IgesDrawPlanarState, "IgesDrawPlanar", parent IgesDataIgesEntity, {
the_nb_matrices: i32,
the_transformation_matrix: Handle<IgesGeomTransformationMatrix>,
the_entities: Handle<HArray1<Handle<IgesDataIgesEntity>>>,
}
}

impl IgesDrawPlanar {
    iges_methods! {
    get nb_matrices = the_nb_matrices: i32;
    get transform_matrix = the_transformation_matrix: Handle<IgesGeomTransformationMatrix>;
    }
    /// This method is used to set the fields of the class Planar
    /// - nbMats                : Number of Transformation matrices
    /// - aTransformationMatrix : Pointer to the Transformation matrix
    /// - allEntities           : Pointers to the entities specified
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn init(
        &self,
        nb_mats: i32,
        a_transformation_matrix: &Handle<IgesGeomTransformationMatrix>,
        all_entities: &Handle<HArray1<Handle<IgesDataIgesEntity>>>,
    ) -> Result<(), Failure> {
        if !all_entities.is_null() {
            if iges_array1_lower(all_entities, "IgesDrawPlanar::init")? != 1 {
                return Err(exceptions::dimension_mismatch("IgesDrawPlanar : Init"));
            }
        }
        {
            let mut state = self.state.write();
            state.the_nb_matrices = nb_mats;
            state.the_transformation_matrix = a_transformation_matrix.clone();
            state.the_entities = all_entities.clone();
        }
        self.init_type_and_form(402, 16);
        Ok(())
    }
    /// returns the number of Entities in the plane pointed to by this
    /// associativity
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn nb_entities(&self) -> Result<i32, Failure> {
        Ok(if self.state.read().the_entities.is_null() {
            0
        } else {
            iges_array1_length(
                &self.state.read().the_entities,
                "IgesDrawPlanar::nb_entities",
            )?
        })
    }
    /// returns True if TransformationMatrix is Identity Matrix,
    /// i.e:- No Matrix defined.
    pub fn is_identity_matrix(&self) -> bool {
        self.state.read().the_transformation_matrix.is_null()
    }
    /// returns the Entity on the specified plane, indicated by EntityIndex
    /// raises an exception if EntityIndex at most 0 or
    /// EntityIndex &gt; NbEntities()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn entity(&self, entity_index: i32) -> Result<Handle<IgesDataIgesEntity>, Failure> {
        iges_array1_item(
            &self.state.read().the_entities,
            entity_index,
            "IgesDrawPlanar::entity",
        )
    }
}
