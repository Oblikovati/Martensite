//! IgesAppliNode.
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
use crate::iges_data::{IgesDataIgesEntity, IgesDataTransfEntity, iges_live};
use crate::iges_entity;
use crate::iges_geom::IgesGeomTransformationMatrix;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkmath::gp::{Point3, Xyz};

iges_entity! {
/// defines Node, Type `134` Form `0`
/// Geometric point used in the definition of a finite element.
IgesAppliNode, IgesAppliNodeState, "IgesAppliNode", parent IgesDataIgesEntity, {
the_coord: Xyz,
the_system: Handle<IgesGeomTransformationMatrix>,
}
}

impl IgesAppliNode {
    /// This method is used to set the fields of the class Node
    /// - aCoord       : Nodal Coordinates
    /// - aCoordSystem : the Nodal Displacement Coordinate
    ///   System Entity (default 0 is Global
    ///   Cartesian Coordinate system)
    pub fn init(&self, a_coord: Xyz, a_coord_system: &Handle<IgesGeomTransformationMatrix>) {
        {
            let mut state = self.state.write();
            state.the_coord = a_coord;
            state.the_system = a_coord_system.clone();
        }
        self.init_type_and_form(134, 0);
    }
    /// returns the nodal coordinates
    pub fn coord(&self) -> Point3 {
        Point3::from(self.state.read().the_coord)
    }
    /// returns TransfEntity if a Nodal Displacement Coordinate
    /// System Entity is defined
    /// else (for Global Cartesien) returns Null Handle
    pub fn system(&self) -> Handle<dyn IgesDataTransfEntity> {
        self.state.read().the_system.cast()
    }
    /// Computes & returns the Type of Coordinate System :
    /// 0 GlobalCartesian, 1 Cartesian, 2 Cylindrical, 3 Spherical
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn system_type(&self) -> Result<i32, Failure> {
        if self.state.read().the_system.is_null() {
            return Ok(0);
        }
        return Ok(
            (iges_live(&self.state.read().the_system, "IgesAppliNode::system_type")?.form_number())
                - 9,
        );
    }
    /// returns the Nodal coordinates after transformation
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn transformed_nodal_coord(&self) -> Result<Point3, Failure> {
        let mut temp_coord: Xyz = self.coord().xyz();
        let mut temp: Handle<dyn IgesDataTransfEntity> = self.system();
        if !temp.is_null() {
            iges_live(&temp, "IgesAppliNode::transformed_nodal_coord")?
                .value()?
                .transforms(&mut temp_coord);
        }
        Ok(Point3::from(temp_coord))
    }
}
