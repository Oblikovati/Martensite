//! IgesDimenNewDimensionedGeometry.
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
use martensite_tkmath::gp::{Point3, Xyz};

iges_entity! {
/// defines New Dimensioned Geometry, Type `402`, Form `21`
/// Links a dimension entity with the geometry entities it
/// is dimensioning, so that later, in the receiving
/// database, the dimension can be automatically recalculated
/// and redrawn should the geometry be changed.
IgesDimenNewDimensionedGeometry, IgesDimenNewDimensionedGeometryState, "IgesDimenNewDimensionedGeometry", parent IgesDataIgesEntity, {
the_nb_dimensions: i32,
the_dimension_entity: Handle<IgesDataIgesEntity>,
the_dimension_orientation_flag: i32,
the_angle_value: f64,
the_geometry_entities: Handle<HArray1<Handle<IgesDataIgesEntity>>>,
the_dimension_location_flags: Handle<HArray1<i32>>,
the_points: Handle<HArray1<Xyz>>,
}
}

impl IgesDimenNewDimensionedGeometry {
    iges_methods! {
    get nb_dimensions = the_nb_dimensions: i32;
    get dimension_entity = the_dimension_entity: Handle<IgesDataIgesEntity>;
    get dimension_orientation_flag = the_dimension_orientation_flag: i32;
    get angle_value = the_angle_value: f64;
    }
    /// This method is used to set the fields of the class
    /// NewDimensionedGeometry
    /// - nbDimen       : Number of Dimensions, default = 1
    /// - aDimen        : Dimension Entity
    /// - anOrientation : Dimension Orientation Flag
    /// - anAngle       : Angle Value
    /// - allEntities   : Geometric Entities
    /// - allLocations  : Dimension Location Flags
    /// - allPoints     : Points on the Geometry Entities
    ///   exception raised if lengths of entities, locations, points
    ///   are not the same
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn init(
        &self,
        nb_dimens: i32,
        a_dimen: &Handle<IgesDataIgesEntity>,
        an_orientation: i32,
        an_angle: f64,
        all_entities: &Handle<HArray1<Handle<IgesDataIgesEntity>>>,
        all_locations: &Handle<HArray1<i32>>,
        all_points: &Handle<HArray1<Xyz>>,
    ) -> Result<(), Failure> {
        let mut num: i32 =
            iges_array1_length(all_entities, "IgesDimenNewDimensionedGeometry::init")?;
        if ((((iges_array1_lower(all_entities, "IgesDimenNewDimensionedGeometry::init")? != 1)
            || (iges_array1_lower(all_locations, "IgesDimenNewDimensionedGeometry::init")? != 1))
            || (iges_array1_length(all_locations, "IgesDimenNewDimensionedGeometry::init")?
                != num))
            || (iges_array1_lower(all_points, "IgesDimenNewDimensionedGeometry::init")? != 1))
            || (iges_array1_length(all_points, "IgesDimenNewDimensionedGeometry::init")? != num)
        {
            return Err(exceptions::dimension_mismatch(
                "IgesDimenNewDimensionedGeometry: Init",
            ));
        }
        {
            let mut state = self.state.write();
            state.the_nb_dimensions = nb_dimens;
            state.the_dimension_entity = a_dimen.clone();
            state.the_dimension_orientation_flag = an_orientation;
            state.the_angle_value = an_angle;
            state.the_geometry_entities = all_entities.clone();
            state.the_dimension_location_flags = all_locations.clone();
            state.the_points = all_points.clone();
        }
        self.init_type_and_form(402, 21);
        Ok(())
    }
    /// returns the number of associated geometry entities
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn nb_geometries(&self) -> Result<i32, Failure> {
        iges_array1_length(
            &self.state.read().the_geometry_entities,
            "IgesDimenNewDimensionedGeometry::nb_geometries",
        )
    }
    /// returns the Index'th geometry entity
    /// raises exception if Index at most 0 or Index &gt; NbGeometries()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn geometry_entity(&self, index: i32) -> Result<Handle<IgesDataIgesEntity>, Failure> {
        iges_array1_item(
            &self.state.read().the_geometry_entities,
            index,
            "IgesDimenNewDimensionedGeometry::geometry_entity",
        )
    }
    /// returns the Index'th geometry entity's dimension location flag
    /// raises exception if Index at most 0 or Index &gt; NbGeometries()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn dimension_location_flag(&self, index: i32) -> Result<i32, Failure> {
        iges_array1_item(
            &self.state.read().the_dimension_location_flags,
            index,
            "IgesDimenNewDimensionedGeometry::dimension_location_flag",
        )
    }
    /// coordinate of point on Index'th geometry entity
    /// raises exception if Index at most 0 or Index &gt; NbGeometries()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn point(&self, index: i32) -> Result<Point3, Failure> {
        Ok(Point3::from(iges_array1_item(
            &self.state.read().the_points,
            index,
            "IgesDimenNewDimensionedGeometry::point",
        )?))
    }
    /// coordinate of point on Index'th geometry entity after Transformation
    /// raises exception if Index at most 0 or Index &gt; NbGeometries()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn transformed_point(&self, index: i32) -> Result<Point3, Failure> {
        let mut point: Xyz = iges_array1_item(
            &self.state.read().the_points,
            index,
            "IgesDimenNewDimensionedGeometry::transformed_point",
        )?;
        if self.has_transf() {
            self.location()?.transforms(&mut point);
        }
        Ok(Point3::from(point))
    }
}
