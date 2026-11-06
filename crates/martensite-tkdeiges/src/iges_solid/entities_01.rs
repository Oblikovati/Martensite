#![allow(
    unused_assignments,
    clippy::if_same_then_else,
    clippy::collapsible_if,
    clippy::collapsible_else_if,
    clippy::manual_range_contains,
    clippy::assign_op_pattern,
    reason = "the members keep the form of the rules they state, and values a reader then ignores"
)]
use super::{IgesSolidLoop, IgesSolidVertexList};
use crate::iges_basic::entity_shape::{
    iges_array1_length, iges_array1_lower, iges_placement_methods,
};
use crate::iges_data::IgesDataIgesEntity;
use crate::iges_data::iges_array1_item;
use crate::iges_geom::{IgesGeomDirection, IgesGeomPoint};
use crate::{iges_entity, iges_methods};
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, exceptions};
use martensite_tkmath::gp::Xyz;

iges_entity! {
/// defines Block, Type `150` Form Number `0`
/// The Block is a rectangular parallelopiped, defined with
/// one vertex at (X1, Y1, Z1) and three edges lying along
/// the local +X, +Y, +Z axes.
IgesSolidBlock, IgesSolidBlockState, "IgesSolidBlock", parent IgesDataIgesEntity, { size: Xyz, corner: Xyz, x_axis: Xyz, z_axis: Xyz }
}
impl IgesSolidBlock {
    iges_methods! {
    get size = size: Xyz;
    }
    iges_placement_methods! {
    point corner = corner;
    located_point transformed_corner = corner;
    direction x_axis = x_axis;
    located_direction transformed_x_axis = x_axis;
    crossed y_axis = x_axis, z_axis;
    located_crossed transformed_y_axis = x_axis, z_axis;
    direction z_axis = z_axis;
    located_direction transformed_z_axis = z_axis;
    }
    /// This method is used to set the fields of the class Block
    /// - aSize   : Length in each local directions
    /// - aCorner : Corner point coordinates. Default (0,0,0)
    /// - aXAxis  : Unit vector defining local X-axis
    ///   default (1,0,0)
    /// - aZAxis  : Unit vector defining local Z-axis
    ///   default (0,0,1)
    pub fn init(&self, a_size: Xyz, a_corner: Xyz, a_x_axis: Xyz, a_z_axis: Xyz) {
        {
            let mut state = self.state.write();
            state.size = a_size;
            state.corner = a_corner;
            state.x_axis = a_x_axis;
            state.z_axis = a_z_axis;
        }
        self.init_type_and_form(150, 0);
    }
    /// returns the length of the Block along the local X-direction
    pub fn x_length(&self) -> f64 {
        let the_size = self.state.read().size;
        the_size.x()
    }
    /// returns the length of the Block along the local Y-direction
    pub fn y_length(&self) -> f64 {
        let the_size = self.state.read().size;
        the_size.y()
    }
    /// returns the length of the Block along the local Z-direction
    pub fn z_length(&self) -> f64 {
        let the_size = self.state.read().size;
        the_size.z()
    }
}
iges_entity! {
/// defines BooleanTree, Type `180` Form Number `0`
/// The Boolean tree describes a binary tree structure
/// composed of regularized Boolean operations and operands,
/// in post-order notation.
IgesSolidBooleanTree, IgesSolidBooleanTreeState, "IgesSolidBooleanTree", parent IgesDataIgesEntity, { operands: Handle<HArray1<Handle<IgesDataIgesEntity>>>, operations: Handle<HArray1<i32>> }
}
impl IgesSolidBooleanTree {
    iges_methods! {
    len_checked length = operands;
    item operand = operands: Handle<IgesDataIgesEntity>;
    }
    /// This method is used to set the fields of the class
    /// BooleanTree
    /// - operands   : Array containing pointer to DE of operands
    /// - operations : Array containing integer type for operations
    pub fn init(
        &self,
        operands: &Handle<HArray1<Handle<IgesDataIgesEntity>>>,
        operations: &Handle<HArray1<i32>>,
    ) -> Result<(), Failure> {
        if iges_array1_lower(operands, "IgesSolidBooleanTree::init")? != 1
            || iges_array1_lower(operations, "IgesSolidBooleanTree::init")? != 1
            || iges_array1_length(operands, "IgesSolidBooleanTree::init")?
                != iges_array1_length(operations, "IgesSolidBooleanTree::init")?
        {
            return Err(exceptions::dimension_error("IgesSolidBooleanTree : Init"));
        }
        {
            let mut state = self.state.write();
            state.operations = operations.clone();
            state.operands = operands.clone();
        }
        self.init_type_and_form(180, 0);
        Ok(())
    }
    /// returns True if Index'th value in the post-order list is an Operand;
    /// else returns False if it is an Integer Operations
    /// raises exception if Index ` 1 or Index ` Length()
    pub fn is_operand(&self, index: i32) -> Result<bool, Failure> {
        let the_operands = self.state.read().operands.clone();
        Ok(!iges_array1_item(&the_operands, index, "IgesSolidBooleanTree::is_operand")?.is_null())
    }
    /// returns the Index'th value in the post-order list only if it is
    /// an operation else returns 0
    /// raises exception if Index ` 1 or Index ` Length()
    pub fn operation(&self, index: i32) -> Result<i32, Failure> {
        let the_operands = self.state.read().operands.clone();
        let the_operations = self.state.read().operations.clone();
        if iges_array1_item(&the_operands, index, "IgesSolidBooleanTree::operation")?.is_null() {
            Ok(iges_array1_item(
                &the_operations,
                index,
                "IgesSolidBooleanTree::operation",
            )?)
        } else {
            Ok(0)
        }
    }
}
iges_entity! {
/// defines ConeFrustum, Type `156` Form Number `0`
/// The Cone Frustum is defined by the center of the
/// larger circular face of the frustum, its radius, a unit
/// vector in the axis direction, a height in this direction
/// and a second circular face with radius which is lesser
/// than the first face.
IgesSolidConeFrustum, IgesSolidConeFrustumState, "IgesSolidConeFrustum", parent IgesDataIgesEntity, { height: f64, r1: f64, r2: f64, face_center: Xyz, axis: Xyz }
}
impl IgesSolidConeFrustum {
    iges_methods! {
    get height = height: f64;
    get larger_radius = r1: f64;
    get smaller_radius = r2: f64;
    }
    iges_placement_methods! {
    point face_center = face_center;
    located_point transformed_face_center = face_center;
    direction axis = axis;
    located_direction transformed_axis = axis;
    }
    /// This method is used to set the fields of the class
    /// ConeFrustum
    /// - Ht     : the Height of cone
    /// - R1     : Radius of the larger face
    /// - R2     : Radius of the smaller face (default 0)
    /// - Center : Center of the larger face (default (0,0,0))
    /// - anAxis : Unit vector in axis direction (default (0,0,1))
    pub fn init(&self, ht: f64, r1: f64, r2: f64, center: Xyz, an_axis: Xyz) {
        {
            let mut state = self.state.write();
            state.height = ht;
            state.r1 = r1;
            state.r2 = r2;
            state.face_center = center;
            state.axis = an_axis;
        }
        self.init_type_and_form(156, 0);
    }
}
iges_entity! {
/// defines ConicalSurface, Type `194` Form Number `0,1`
/// The right circular conical surface is defined by a
/// point on the axis on the cone, the direction of the axis
/// of the cone, the radius of the cone at the axis point and
/// the cone semi-angle.
IgesSolidConicalSurface, IgesSolidConicalSurfaceState, "IgesSolidConicalSurface", parent IgesDataIgesEntity, { location_point: Handle<IgesGeomPoint>, axis: Handle<IgesGeomDirection>, radius: f64, angle: f64, ref_dir: Handle<IgesGeomDirection> }
}
impl IgesSolidConicalSurface {
    iges_methods! {
    get location_point = location_point: Handle<IgesGeomPoint>;
    get axis = axis: Handle<IgesGeomDirection>;
    get radius = radius: f64;
    get semi_angle = angle: f64;
    get reference_dir = ref_dir: Handle<IgesGeomDirection>;
    }
    iges_placement_methods! {
    has is_parametrised = ref_dir;
    }
    /// This method is used to set the fields of the class
    /// ConicalSurface
    /// - aLocation : Location of the point on axis
    /// - anAxis    : Direction of the axis
    /// - aRadius   : Radius at axis point
    /// - anAngle   : Value of semi-angle in degrees (0`angle`90)
    /// - aRefdir   : Reference direction (parametrised surface)
    ///   Null if unparametrised surface.
    pub fn init(
        &self,
        a_location: &Handle<IgesGeomPoint>,
        an_axis: &Handle<IgesGeomDirection>,
        a_radius: f64,
        an_angle: f64,
        a_refdir: &Handle<IgesGeomDirection>,
    ) {
        {
            let mut state = self.state.write();
            state.location_point = a_location.clone();
            state.axis = an_axis.clone();
            state.radius = a_radius;
            state.angle = an_angle;
            state.ref_dir = a_refdir.clone();
        }
        self.init_type_and_form(
            194,
            if self.state.read().ref_dir.clone().is_null() {
                0
            } else {
                1
            },
        );
    }
}
iges_entity! {
/// defines Cylinder, Type `154` Form Number `0`
/// This defines a solid cylinder
IgesSolidCylinder, IgesSolidCylinderState, "IgesSolidCylinder", parent IgesDataIgesEntity, { height: f64, radius: f64, face_center: Xyz, axis: Xyz }
}
impl IgesSolidCylinder {
    iges_methods! {
    get height = height: f64;
    get radius = radius: f64;
    }
    iges_placement_methods! {
    point face_center = face_center;
    located_point transformed_face_center = face_center;
    direction axis = axis;
    located_direction transformed_axis = axis;
    }
    /// This method is used to set the fields of the class
    /// Cylinder
    /// - aHeight : Cylinder height
    /// - aRadius : Cylinder radius
    /// - aCenter : First face center coordinates (default (0,0,0))
    /// - anAxis  : Unit vector in axis direction (default (0,0,1))
    pub fn init(&self, a_height: f64, a_radius: f64, a_center: Xyz, an_axis: Xyz) {
        {
            let mut state = self.state.write();
            state.height = a_height;
            state.radius = a_radius;
            state.face_center = a_center;
            state.axis = an_axis;
        }
        self.init_type_and_form(154, 0);
    }
}
iges_entity! {
/// defines CylindricalSurface, Type `192` Form Number `0,1`
IgesSolidCylindricalSurface, IgesSolidCylindricalSurfaceState, "IgesSolidCylindricalSurface", parent IgesDataIgesEntity, { location_point: Handle<IgesGeomPoint>, axis: Handle<IgesGeomDirection>, radius: f64, ref_dir: Handle<IgesGeomDirection> }
}
impl IgesSolidCylindricalSurface {
    iges_methods! {
    get location_point = location_point: Handle<IgesGeomPoint>;
    get axis = axis: Handle<IgesGeomDirection>;
    get radius = radius: f64;
    get reference_dir = ref_dir: Handle<IgesGeomDirection>;
    }
    iges_placement_methods! {
    has is_parametrised = ref_dir;
    }
    /// This method is used to set the fields of the class
    /// CylindricalSurface
    /// - aLocation : the location of the point on axis
    /// - anAxis    : the direction of the axis
    /// - aRadius   : the radius at the axis point
    /// - aRefdir   : the reference direction (parametrised surface)
    ///   default NULL (unparametrised surface)
    pub fn init(
        &self,
        a_location: &Handle<IgesGeomPoint>,
        an_axis: &Handle<IgesGeomDirection>,
        a_radius: f64,
        a_refdir: &Handle<IgesGeomDirection>,
    ) {
        {
            let mut state = self.state.write();
            state.location_point = a_location.clone();
            state.axis = an_axis.clone();
            state.radius = a_radius;
            state.ref_dir = a_refdir.clone();
        }
        self.init_type_and_form(
            192,
            if self.state.read().ref_dir.clone().is_null() {
                0
            } else {
                1
            },
        );
    }
}
iges_entity! {
/// defines EdgeList, Type `504` Form `1`
/// EdgeList is defined as a segment joining two vertices
/// It contains one or more edge tuples.
IgesSolidEdgeList, IgesSolidEdgeListState, "IgesSolidEdgeList", parent IgesDataIgesEntity, { curves: Handle<HArray1<Handle<IgesDataIgesEntity>>>, start_vertex_list: Handle<HArray1<Handle<IgesSolidVertexList>>>, start_vertex_index: Handle<HArray1<i32>>, end_vertex_list: Handle<HArray1<Handle<IgesSolidVertexList>>>, end_vertex_index: Handle<HArray1<i32>> }
}
impl IgesSolidEdgeList {
    iges_methods! {
    len nb_edges = curves;
    item curve = curves: Handle<IgesDataIgesEntity>;
    item start_vertex_list = start_vertex_list: Handle<IgesSolidVertexList>;
    item start_vertex_index = start_vertex_index: i32;
    item end_vertex_list = end_vertex_list: Handle<IgesSolidVertexList>;
    item end_vertex_index = end_vertex_index: i32;
    }
    /// This method is used to set the fields of the class
    /// EdgeList
    /// - curves           : the model space curves
    /// - startVertexList  : the vertex list that contains the
    ///   start vertices
    /// - startVertexIndex : the index of the vertex in the
    ///   corresponding vertex list
    /// - endVertexList    : the vertex list that contains the
    ///   end vertices
    /// - endVertexIndex   : the index of the vertex in the
    ///   corresponding vertex list
    ///   raises exception if size of curves,startVertexList,startVertexIndex,
    ///   endVertexList and endVertexIndex do no match
    pub fn init(
        &self,
        curves: &Handle<HArray1<Handle<IgesDataIgesEntity>>>,
        start_vertex_list: &Handle<HArray1<Handle<IgesSolidVertexList>>>,
        start_vertex_index: &Handle<HArray1<i32>>,
        end_vertex_list: &Handle<HArray1<Handle<IgesSolidVertexList>>>,
        end_vertex_index: &Handle<HArray1<i32>>,
    ) -> Result<(), Failure> {
        let nb: i32 = if curves.is_null() {
            0
        } else {
            iges_array1_length(curves, "IgesSolidEdgeList::init")?
        };
        if nb == 0
            || iges_array1_lower(curves, "IgesSolidEdgeList::init")? != 1
            || iges_array1_lower(start_vertex_list, "IgesSolidEdgeList::init")? != 1
            || iges_array1_length(start_vertex_list, "IgesSolidEdgeList::init")? != nb
            || iges_array1_lower(start_vertex_index, "IgesSolidEdgeList::init")? != 1
            || iges_array1_length(start_vertex_index, "IgesSolidEdgeList::init")? != nb
            || iges_array1_lower(end_vertex_list, "IgesSolidEdgeList::init")? != 1
            || iges_array1_length(end_vertex_list, "IgesSolidEdgeList::init")? != nb
            || iges_array1_lower(end_vertex_index, "IgesSolidEdgeList::init")? != 1
            || iges_array1_length(end_vertex_index, "IgesSolidEdgeList::init")? != nb
        {
            return Err(exceptions::dimension_error("IgesSolidEdgeList : Init"));
        }
        {
            let mut state = self.state.write();
            state.curves = curves.clone();
            state.start_vertex_list = start_vertex_list.clone();
            state.start_vertex_index = start_vertex_index.clone();
            state.end_vertex_list = end_vertex_list.clone();
            state.end_vertex_index = end_vertex_index.clone();
        }
        self.init_type_and_form(504, 1);
        Ok(())
    }
}
iges_entity! {
/// defines Ellipsoid, Type `168` Form Number `0`
/// The ellipsoid is a solid bounded by the surface defined
/// by:
/// X^2       Y^2       Z^2
/// -----  +  -----  +  -----  =  1
/// LX^2      LY^2      LZ^2
IgesSolidEllipsoid, IgesSolidEllipsoidState, "IgesSolidEllipsoid", parent IgesDataIgesEntity, { size: Xyz, center: Xyz, x_axis: Xyz, z_axis: Xyz }
}
impl IgesSolidEllipsoid {
    iges_methods! {
    get size = size: Xyz;
    }
    iges_placement_methods! {
    point center = center;
    located_point transformed_center = center;
    direction x_axis = x_axis;
    located_direction transformed_x_axis = x_axis;
    crossed y_axis = x_axis, z_axis;
    located_crossed transformed_y_axis = x_axis, z_axis;
    direction z_axis = z_axis;
    located_direction transformed_z_axis = z_axis;
    }
    /// This method is used to set the fields of the class
    /// Ellipsoid
    /// - aSize      : Lengths in the local X,Y,Z directions
    /// - aCenter    : Center point of ellipsoid (default (0,0,0))
    /// - anXAxis    : Unit vector defining local X-axis
    ///   default (1,0,0)
    /// - anZAxis    : Unit vector defining local Z-axis
    ///   default (0,0,1)
    pub fn init(&self, a_size: Xyz, a_center: Xyz, an_x_axis: Xyz, an_z_axis: Xyz) {
        {
            let mut state = self.state.write();
            state.size = a_size;
            state.center = a_center;
            state.x_axis = an_x_axis;
            state.z_axis = an_z_axis;
        }
        self.init_type_and_form(168, 0);
    }
    /// returns the length in the local X-direction
    pub fn x_length(&self) -> f64 {
        let the_size = self.state.read().size;
        the_size.x()
    }
    /// returns the length in the local Y-direction
    pub fn y_length(&self) -> f64 {
        let the_size = self.state.read().size;
        the_size.y()
    }
    /// returns the length in the local Z-direction
    pub fn z_length(&self) -> f64 {
        let the_size = self.state.read().size;
        the_size.z()
    }
}
iges_entity! {
/// defines Face, Type `510` Form Number `1`
/// Face entity is a bound (partial) which has finite area
IgesSolidFace, IgesSolidFaceState, "IgesSolidFace", parent IgesDataIgesEntity, { surface: Handle<IgesDataIgesEntity>, has_outer_loop: bool, loops: Handle<HArray1<Handle<IgesSolidLoop>>> }
}
impl IgesSolidFace {
    iges_methods! {
    get surface = surface: Handle<IgesDataIgesEntity>;
    len_checked nb_loops = loops;
    item loop_ = loops: Handle<IgesSolidLoop>;
    }
    /// This method is used to set the fields of the class Face
    /// - aSurface      : Pointer to the underlying surface
    /// - outerLoopFlag : True means the first loop is the outer loop
    /// - loops         : Array of loops bounding the face
    pub fn init(
        &self,
        a_surface: &Handle<IgesDataIgesEntity>,
        outer_loop_flag: bool,
        loops: &Handle<HArray1<Handle<IgesSolidLoop>>>,
    ) -> Result<(), Failure> {
        if iges_array1_lower(loops, "IgesSolidFace::init")? != 1 {
            return Err(exceptions::dimension_mismatch("IgesSolidFace : Init"));
        }
        {
            let mut state = self.state.write();
            state.surface = a_surface.clone();
            state.has_outer_loop = outer_loop_flag;
            state.loops = loops.clone();
        }
        self.init_type_and_form(510, 1);
        Ok(())
    }
    /// checks whether there is an outer loop or not
    pub fn has_outer_loop(&self) -> bool {
        self.state.read().has_outer_loop
    }
}
