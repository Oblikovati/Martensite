#![allow(
    unused_assignments,
    clippy::if_same_then_else,
    clippy::collapsible_if,
    clippy::collapsible_else_if,
    clippy::manual_range_contains,
    clippy::assign_op_pattern,
    reason = "the members keep the form of the rules they state, and values a reader then ignores"
)]
use super::{IgesSolidBooleanTree, IgesSolidFace};
use crate::iges_basic::entity_shape::{
    iges_array1_length, iges_array1_lower, iges_placement_methods,
};
use crate::iges_basic::{
    IgesBasicHArray1OfHArray1OfIgesEntity, IgesBasicHArray1OfHArray1OfInteger,
};
use crate::iges_data::IgesDataIgesEntity;
use crate::iges_data::{iges_array1_item, iges_live};
use crate::iges_geom::{IgesGeomDirection, IgesGeomPoint, IgesGeomTransformationMatrix};
use crate::{iges_entity, iges_methods};
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, exceptions};
use martensite_tkmath::gp::Xyz;

iges_entity! {
/// defines Loop, Type `508` Form Number `1`
/// A Loop entity specifies a bound of a face. It represents
/// a connected collection of face boundaries, seams, and
/// poles of a single face.
/// From IGES-5.3, a Loop can be free with Form Number 0,
/// else it is a bound of a face (it is the default)
IgesSolidLoop, IgesSolidLoopState, "IgesSolidLoop", parent IgesDataIgesEntity, { types: Handle<HArray1<i32>>, edges: Handle<HArray1<Handle<IgesDataIgesEntity>>>, index: Handle<HArray1<i32>>, orientation_flags: Handle<HArray1<i32>>, nb_parameter_curves: Handle<HArray1<i32>>, isoparametric_flags: Handle<IgesBasicHArray1OfHArray1OfInteger>, curves: Handle<IgesBasicHArray1OfHArray1OfIgesEntity> }
}
impl IgesSolidLoop {
    iges_methods! {
    len nb_edges = edges;
    item edge_type = types: i32;
    item edge = edges: Handle<IgesDataIgesEntity>;
    item nb_parameter_curves = nb_parameter_curves: i32;
    item list_index = index: i32;
    }
    /// This method is used to set the fields of the class Loop
    /// - types              : 0 = Edge; 1 = Vertex
    /// - edges              : Pointer to the EdgeList or VertexList
    /// - index              : Index of the edge into the EdgeList
    ///   VertexList entity
    /// - orient             : Orientation flag of the edge
    /// - nbParameterCurves  : the number of parameter space curves
    ///   for each edge
    /// - isoparametricFlags : the isoparametric flag of the
    ///   parameter space curve
    /// - curves             : the parameter space curves
    ///   raises exception if length of types, edges, index, orient and
    ///   nbParameterCurves do not match or the length of
    ///   isoparametricFlags and curves do not match
    #[allow(clippy::too_many_arguments, reason = "the member takes every field")]
    pub fn init(
        &self,
        types: &Handle<HArray1<i32>>,
        edges: &Handle<HArray1<Handle<IgesDataIgesEntity>>>,
        index: &Handle<HArray1<i32>>,
        orient: &Handle<HArray1<i32>>,
        nb_parameter_curves: &Handle<HArray1<i32>>,
        isoparametric_flags: &Handle<IgesBasicHArray1OfHArray1OfInteger>,
        curves: &Handle<IgesBasicHArray1OfHArray1OfIgesEntity>,
    ) -> Result<(), Failure> {
        let isoparametric_flags_live = iges_live(isoparametric_flags, "IgesSolidLoop::init")?;
        let curves_live = iges_live(curves, "IgesSolidLoop::init")?;
        let nb: i32 = iges_array1_length(types, "IgesSolidLoop::init")?;
        if iges_array1_lower(types, "IgesSolidLoop::init")? != 1
            || iges_array1_lower(edges, "IgesSolidLoop::init")? != 1
            || nb != iges_array1_length(edges, "IgesSolidLoop::init")?
            || iges_array1_lower(index, "IgesSolidLoop::init")? != 1
            || nb != iges_array1_length(index, "IgesSolidLoop::init")?
            || iges_array1_lower(orient, "IgesSolidLoop::init")? != 1
            || nb != iges_array1_length(orient, "IgesSolidLoop::init")?
            || iges_array1_lower(nb_parameter_curves, "IgesSolidLoop::init")? != 1
            || nb != iges_array1_length(nb_parameter_curves, "IgesSolidLoop::init")?
            || isoparametric_flags_live.lower() != 1
            || nb != isoparametric_flags_live.length()
            || curves_live.lower() != 1
            || nb != curves_live.length()
        {
            return Err(exceptions::dimension_error("IgesSolidLoop : Init"));
        }
        {
            let mut state = self.state.write();
            state.types = types.clone();
            state.edges = edges.clone();
            state.index = index.clone();
            state.orientation_flags = orient.clone();
            state.nb_parameter_curves = nb_parameter_curves.clone();
            state.isoparametric_flags = isoparametric_flags.clone();
            state.curves = curves.clone();
        }
        self.init_type_and_form(508, 1);
        Ok(())
    }
    /// Tells if a Loop is a Bound (FN 1) else it is free (FN 0)
    pub fn is_bound(&self) -> bool {
        self.form_number() == 1
    }
    /// Sets or Unset the Bound Status (from Form Number)
    /// Default is True
    pub fn set_bound(&self, bound: bool) {
        self.init_type_and_form(508, if bound { 1 } else { 0 });
    }
    /// returns the orientation flag corresponding to Index'th edge
    /// raises exception if Index `= 0 or Index ` NbEdges()
    pub fn orientation(&self, index: i32) -> Result<bool, Failure> {
        let the_orientation_flags = self.state.read().orientation_flags.clone();
        Ok(iges_array1_item(&the_orientation_flags, index, "IgesSolidLoop::orientation")? != 0)
    }
    /// `is_isoparametric`.
    pub fn is_isoparametric(&self, edge_index: i32, curve_index: i32) -> Result<bool, Failure> {
        let the_isoparametric_flags = self.state.read().isoparametric_flags.clone();
        if !iges_live(&the_isoparametric_flags, "IgesSolidLoop::is_isoparametric")?
            .value(edge_index)?
            .is_null()
        {
            Ok(iges_array1_item(
                &iges_live(&the_isoparametric_flags, "IgesSolidLoop::is_isoparametric")?
                    .value(edge_index)?,
                curve_index,
                "IgesSolidLoop::is_isoparametric",
            )? != 0)
        } else {
            Ok(false)
        }
    }
    /// returns the CurveIndex'th parameter space curve associated with
    /// EdgeIndex'th edge
    /// raises exception if EdgeIndex `= 0 or EdgeIndex ` NbEdges() or
    /// if CurveIndex `= 0 or CurveIndex ` NbParameterCurves(EdgeIndex)
    pub fn parametric_curve(
        &self,
        edge_index: i32,
        curve_index: i32,
    ) -> Result<Handle<IgesDataIgesEntity>, Failure> {
        let the_curves = self.state.read().curves.clone();
        let mut acurve: Handle<IgesDataIgesEntity> = Handle::default();
        if !iges_live(&the_curves, "IgesSolidLoop::parametric_curve")?
            .value(edge_index)?
            .is_null()
        {
            acurve = iges_array1_item(
                &iges_live(&the_curves, "IgesSolidLoop::parametric_curve")?.value(edge_index)?,
                curve_index,
                "IgesSolidLoop::parametric_curve",
            )?;
        }
        Ok(acurve)
    }
}
iges_entity! {
/// defines ManifoldSolid, Type `186` Form Number `0`
/// A manifold solid is a bounded, closed, and finite volume
/// in three dimensional Euclidean space
IgesSolidManifoldSolid, IgesSolidManifoldSolidState, "IgesSolidManifoldSolid", parent IgesDataIgesEntity, { shell: Handle<IgesSolidShell>, orientation_flag: bool, void_shells: Handle<HArray1<Handle<IgesSolidShell>>>, orient_flags: Handle<HArray1<i32>> }
}
impl IgesSolidManifoldSolid {
    iges_methods! {
    get shell = shell: Handle<IgesSolidShell>;
    get orientation_flag = orientation_flag: bool;
    len nb_void_shells = void_shells;
    }
    /// This method is used to set the fields of the class
    /// ManifoldSolid
    /// - aShell         : pointer to the shell
    /// - shellflag      : orientation flag of shell
    /// - voidShells     : the void shells
    /// - voidShellFlags : orientation of the void shells
    ///   raises exception if length of voidShells and voidShellFlags
    ///   do not match
    pub fn init(
        &self,
        a_shell: &Handle<IgesSolidShell>,
        shellflag: bool,
        void_shells: &Handle<HArray1<Handle<IgesSolidShell>>>,
        void_shell_flags: &Handle<HArray1<i32>>,
    ) -> Result<(), Failure> {
        if !void_shells.is_null() {
            if iges_array1_lower(void_shells, "IgesSolidManifoldSolid::init")? != 1
                || iges_array1_lower(void_shell_flags, "IgesSolidManifoldSolid::init")? != 1
                || iges_array1_length(void_shells, "IgesSolidManifoldSolid::init")?
                    != iges_array1_length(void_shell_flags, "IgesSolidManifoldSolid::init")?
            {
                return Err(exceptions::dimension_error("IgesSolidManifoldSolid : Init"));
            }
        }
        {
            let mut state = self.state.write();
            state.shell = a_shell.clone();
            state.orientation_flag = shellflag;
            state.void_shells = void_shells.clone();
            state.orient_flags = void_shell_flags.clone();
        }
        self.init_type_and_form(186, 0);
        Ok(())
    }
    /// returns Index'th void shell.
    /// raises exception if Index `= 0 or Index ` NbVoidShells()
    pub fn void_shell(&self, index: i32) -> Result<Handle<IgesSolidShell>, Failure> {
        let the_void_shells = self.state.read().void_shells.clone();
        let mut ashell: Handle<IgesSolidShell> = Handle::default();
        if !the_void_shells.is_null() {
            ashell = iges_array1_item(
                &the_void_shells,
                index,
                "IgesSolidManifoldSolid::void_shell",
            )?;
        }
        Ok(ashell)
    }
    /// returns Index'th orientation flag.
    /// raises exception if Index `= 0 or Index ` NbVoidShells()
    pub fn void_orientation_flag(&self, index: i32) -> Result<bool, Failure> {
        let the_orient_flags = self.state.read().orient_flags.clone();
        if !the_orient_flags.is_null() {
            Ok(iges_array1_item(
                &the_orient_flags,
                index,
                "IgesSolidManifoldSolid::void_orientation_flag",
            )? != 0)
        } else {
            Ok(false)
        }
    }
}
iges_entity! {
/// defines PlaneSurface, Type `190` Form Number `0,1`
/// A plane surface entity is defined by a point on the
/// surface and a normal to it.
IgesSolidPlaneSurface, IgesSolidPlaneSurfaceState, "IgesSolidPlaneSurface", parent IgesDataIgesEntity, { location_point: Handle<IgesGeomPoint>, normal: Handle<IgesGeomDirection>, ref_dir: Handle<IgesGeomDirection> }
}
impl IgesSolidPlaneSurface {
    iges_methods! {
    get location_point = location_point: Handle<IgesGeomPoint>;
    get normal = normal: Handle<IgesGeomDirection>;
    get reference_dir = ref_dir: Handle<IgesGeomDirection>;
    }
    iges_placement_methods! {
    has is_parametrised = ref_dir;
    }
    /// This method is used to set the fields of the class
    /// PlaneSurface
    /// - aLocation : the point on the surface
    /// - aNormal   : the surface normal direction
    /// - refdir    : the reference direction (default NULL) for
    ///   unparameterised curves
    pub fn init(
        &self,
        a_location: &Handle<IgesGeomPoint>,
        a_normal: &Handle<IgesGeomDirection>,
        refdir: &Handle<IgesGeomDirection>,
    ) {
        {
            let mut state = self.state.write();
            state.location_point = a_location.clone();
            state.normal = a_normal.clone();
            state.ref_dir = refdir.clone();
        }
        self.init_type_and_form(
            190,
            if self.state.read().ref_dir.clone().is_null() {
                0
            } else {
                1
            },
        );
    }
}
iges_entity! {
/// defines RightAngularWedge, Type `152` Form Number `0`
/// A right angular wedge is a triangular/trapezoidal prism
IgesSolidRightAngularWedge, IgesSolidRightAngularWedgeState, "IgesSolidRightAngularWedge", parent IgesDataIgesEntity, { size: Xyz, x_small_length: f64, corner: Xyz, x_axis: Xyz, z_axis: Xyz }
}
impl IgesSolidRightAngularWedge {
    iges_methods! {
    get size = size: Xyz;
    get x_small_length = x_small_length: f64;
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
    /// This method is used to set the fields of the class
    /// RightAngularWedge
    /// - aSize    : the lengths along the local axes
    /// - lowX     : the length at the smaller X-side
    /// - aCorner  : the corner point coordinates
    ///   default (0,0,0)
    /// - anXAxis  : the unit vector defining local X-axis
    ///   default (1,0,0)
    /// - anZAxis  : the unit vector defining local Z-axis
    ///   default (0,0,1)
    pub fn init(&self, a_size: Xyz, low_x: f64, a_corner: Xyz, an_x_axis: Xyz, an_z_axis: Xyz) {
        {
            let mut state = self.state.write();
            state.size = a_size;
            state.x_small_length = low_x;
            state.corner = a_corner;
            state.x_axis = an_x_axis;
            state.z_axis = an_z_axis;
        }
        self.init_type_and_form(152, 0);
    }
    /// returns the length along the local X-axis
    pub fn x_big_length(&self) -> f64 {
        let the_size = self.state.read().size;
        the_size.x()
    }
    /// returns the length along the local Y-axis
    pub fn y_length(&self) -> f64 {
        let the_size = self.state.read().size;
        the_size.y()
    }
    /// returns the length along the local Z-axis
    pub fn z_length(&self) -> f64 {
        let the_size = self.state.read().size;
        the_size.z()
    }
}
iges_entity! {
/// defines SelectedComponent, Type `182` Form Number `0`
/// The Selected Component entity provides a means of
/// selecting one component of a disjoint CSG solid
IgesSolidSelectedComponent, IgesSolidSelectedComponentState, "IgesSolidSelectedComponent", parent IgesDataIgesEntity, { entity: Handle<IgesSolidBooleanTree>, select_point: Xyz }
}
impl IgesSolidSelectedComponent {
    iges_methods! {
    get component = entity: Handle<IgesSolidBooleanTree>;
    }
    iges_placement_methods! {
    point select_point = select_point;
    located_point transformed_select_point = select_point;
    }
    /// This method is used to set the fields of the class
    /// SelectedComponent
    /// - anEntity  : the Boolean tree entity
    /// - selectPnt : Point in or on the desired component
    pub fn init(&self, an_entity: &Handle<IgesSolidBooleanTree>, select_pnt: Xyz) {
        {
            let mut state = self.state.write();
            state.entity = an_entity.clone();
            state.select_point = select_pnt;
        }
        self.init_type_and_form(182, 0);
    }
}
iges_entity! {
/// defines Shell, Type `514` Form Number `1`
/// Shell entity is a connected entity of dimensionality 2
/// which divides R3 into two arcwise connected open subsets,
/// one of which is finite. Inside of the shell is defined to
/// be the finite region.
/// From IGES-5.3, Form can be `1` for Closed or `2` for Open
IgesSolidShell, IgesSolidShellState, "IgesSolidShell", parent IgesDataIgesEntity, { faces: Handle<HArray1<Handle<IgesSolidFace>>>, orientation: Handle<HArray1<i32>> }
}
impl IgesSolidShell {
    iges_methods! {
    len_checked nb_faces = faces;
    item face = faces: Handle<IgesSolidFace>;
    }
    /// This method is used to set the fields of the class Shell
    /// - allFaces  : the faces comprising the shell
    /// - allOrient : the orientation flags of the shell
    ///   raises exception if length of allFaces & allOrient do not match
    pub fn init(
        &self,
        faces: &Handle<HArray1<Handle<IgesSolidFace>>>,
        orient: &Handle<HArray1<i32>>,
    ) -> Result<(), Failure> {
        if iges_array1_lower(faces, "IgesSolidShell::init")? != 1
            || iges_array1_lower(orient, "IgesSolidShell::init")? != 1
            || iges_array1_length(faces, "IgesSolidShell::init")?
                != iges_array1_length(orient, "IgesSolidShell::init")?
        {
            return Err(exceptions::dimension_error("IgesSolidShell : Init"));
        }
        {
            let mut state = self.state.write();
            state.faces = faces.clone();
            state.orientation = orient.clone();
        }
        self.init_type_and_form(514, 1);
        Ok(())
    }
    /// Tells if a Shell is Closed, i.e. if its FormNumber is 1
    /// (this is the default)
    pub fn is_closed(&self) -> bool {
        self.form_number() == 1
    }
    /// Sets or Unsets the Closed status (FormNumber = 1 else 2)
    pub fn set_closed(&self, closed: bool) {
        self.init_type_and_form(514, if closed { 1 } else { 2 });
    }
    /// returns the orientation of Index'th face w.r.t the direction of
    /// the underlying surface
    /// raises exception if Index `= 0 or Index ` NbFaces()
    pub fn orientation(&self, index: i32) -> Result<bool, Failure> {
        let the_orientation = self.state.read().orientation.clone();
        Ok(iges_array1_item(&the_orientation, index, "IgesSolidShell::orientation")? != 0)
    }
}
iges_entity! {
/// defines SolidAssembly, Type `184` Form `0`
/// Solid assembly is a collection of items which possess a
/// shared fixed geometric relationship.
/// From IGES-5.3, From 1 says that at least one item is a Brep
/// else all are Primitives, Boolean Trees, Solid Instances or
/// other Assemblies
IgesSolidSolidAssembly, IgesSolidSolidAssemblyState, "IgesSolidSolidAssembly", parent IgesDataIgesEntity, { items: Handle<HArray1<Handle<IgesDataIgesEntity>>>, matrices: Handle<HArray1<Handle<IgesGeomTransformationMatrix>>> }
}
impl IgesSolidSolidAssembly {
    iges_methods! {
    len_checked nb_items = items;
    item item = items: Handle<IgesDataIgesEntity>;
    item transf_matrix = matrices: Handle<IgesGeomTransformationMatrix>;
    }
    /// This method is used to set the fields of the class
    /// SolidAssembly
    /// - allItems    : the collection of items
    /// - allMatrices : transformation matrices corresponding to each
    ///   item
    ///   raises exception if the length of allItems & allMatrices
    ///   do not match
    pub fn init(
        &self,
        items: &Handle<HArray1<Handle<IgesDataIgesEntity>>>,
        matrices: &Handle<HArray1<Handle<IgesGeomTransformationMatrix>>>,
    ) -> Result<(), Failure> {
        if iges_array1_lower(items, "IgesSolidSolidAssembly::init")? != 1
            || iges_array1_lower(matrices, "IgesSolidSolidAssembly::init")? != 1
            || iges_array1_length(items, "IgesSolidSolidAssembly::init")?
                != iges_array1_length(matrices, "IgesSolidSolidAssembly::init")?
        {
            return Err(exceptions::dimension_error("IgesSolidSolidAssembly : Init"));
        }
        {
            let mut state = self.state.write();
            state.items = items.clone();
            state.matrices = matrices.clone();
        }
        self.init_type_and_form(184, 0);
        Ok(())
    }
    /// Tells if at least one item is a Brep, from FormNumber
    pub fn has_brep(&self) -> bool {
        self.form_number() == 1
    }
    /// Sets or Unsets the status "HasBrep" from FormNumber
    /// Default is False
    pub fn set_brep(&self, hasbrep: bool) {
        self.init_type_and_form(184, if hasbrep { 1 } else { 0 });
    }
}
