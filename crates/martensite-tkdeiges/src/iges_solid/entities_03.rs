#![allow(
    unused_assignments,
    clippy::if_same_then_else,
    clippy::collapsible_if,
    clippy::collapsible_else_if,
    clippy::manual_range_contains,
    clippy::assign_op_pattern,
    reason = "the members keep the form of the rules they state, and values a reader then ignores"
)]
use crate::iges_basic::entity_shape::{iges_array1_lower, iges_placement_methods};
use crate::iges_data::IgesDataIgesEntity;
use crate::iges_data::{iges_array1_item, iges_live};
use crate::iges_geom::{IgesGeomDirection, IgesGeomPoint};
use crate::{iges_entity, iges_methods};
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, exceptions};
use martensite_tkmath::gp::{Point3, Xyz};

iges_entity! {
/// defines SolidInstance, Type `430` Form Number `0`
/// This provides a mechanism for replicating a solid
/// representation.
/// From IGES-5.3, Form may be `1` for a BREP
/// Else it is for a Boolean Tree, Primitive, other Solid Inst.
IgesSolidSolidInstance, IgesSolidSolidInstanceState, "IgesSolidSolidInstance", parent IgesDataIgesEntity, { entity: Handle<IgesDataIgesEntity> }
}
impl IgesSolidSolidInstance {
    iges_methods! {
    get entity = entity: Handle<IgesDataIgesEntity>;
    }
    /// This method is used to set the fields of the class
    /// SolidInstance
    /// - anEntity : the entity corresponding to the solid
    pub fn init(&self, an_entity: &Handle<IgesDataIgesEntity>) {
        self.state.write().entity = an_entity.clone();
        self.init_type_and_form(430, 0);
    }
    /// Tells if a SolidInstance is for a BREP
    /// Default is False
    pub fn is_brep(&self) -> bool {
        self.form_number() == 1
    }
    /// Sets or unsets the Brep status (FormNumber = 1 else 0)
    pub fn set_brep(&self, brep: bool) {
        self.init_type_and_form(430, if brep { 1 } else { 0 });
    }
}
iges_entity! {
/// defines SolidOfLinearExtrusion, Type `164` Form Number `0`
/// Solid of linear extrusion is defined by translating an
/// area determined by a planar curve
IgesSolidSolidOfLinearExtrusion, IgesSolidSolidOfLinearExtrusionState, "IgesSolidSolidOfLinearExtrusion", parent IgesDataIgesEntity, { curve: Handle<IgesDataIgesEntity>, length: f64, direction: Xyz }
}
impl IgesSolidSolidOfLinearExtrusion {
    iges_methods! {
    get curve = curve: Handle<IgesDataIgesEntity>;
    get extrusion_length = length: f64;
    }
    iges_placement_methods! {
    direction extrusion_direction = direction;
    located_direction transformed_extrusion_direction = direction;
    }
    /// This method is used to set the fields of the class
    /// SolidOfLinearExtrusion
    /// - aCurve     : the planar curve that is to be translated
    /// - aLength    : the length of extrusion
    /// - aDirection : the vector specifying the direction of extrusion
    ///   default (0,0,1)
    pub fn init(&self, a_curve: &Handle<IgesDataIgesEntity>, length: f64, direction: Xyz) {
        {
            let mut state = self.state.write();
            state.curve = a_curve.clone();
            state.length = length;
            state.direction = direction;
        }
        self.init_type_and_form(164, 0);
    }
}
iges_entity! {
/// defines SolidOfRevolution, Type `162` Form Number `0,1`
/// This entity is defined by revolving the area determined
/// by a planar curve about a specified axis through a given
/// fraction of full rotation.
IgesSolidSolidOfRevolution, IgesSolidSolidOfRevolutionState, "IgesSolidSolidOfRevolution", parent IgesDataIgesEntity, { curve: Handle<IgesDataIgesEntity>, fraction: f64, axis_point: Xyz, axis: Xyz }
}
impl IgesSolidSolidOfRevolution {
    iges_methods! {
    get curve = curve: Handle<IgesDataIgesEntity>;
    get fraction = fraction: f64;
    }
    iges_placement_methods! {
    point axis_point = axis_point;
    located_point transformed_axis_point = axis_point;
    direction axis = axis;
    located_direction transformed_axis = axis;
    }
    /// This method is used to set the fields of the class
    /// SolidOfRevolution
    /// - aCurve     : the curve entity that is to be revolved
    /// - aFract     : the fraction of full rotation (default 1.0)
    /// - aAxisPnt   : the point on the axis
    /// - aDirection : the direction of the axis
    pub fn init(
        &self,
        a_curve: &Handle<IgesDataIgesEntity>,
        fract: f64,
        axis_pnt: Xyz,
        direction: Xyz,
    ) {
        {
            let mut state = self.state.write();
            state.curve = a_curve.clone();
            state.fraction = fract;
            state.axis_point = axis_pnt;
            state.axis = direction;
        }
        self.init_type_and_form(162, self.form_number());
    }
    /// Sets the Curve to be by default, Closed to Axis (Form 0)
    /// if `mode` is True, Closed to Itself (Form 1) else
    pub fn set_closed_to_axis(&self, f: bool) {
        self.init_type_and_form(162, if f { 0 } else { 1 });
    }
    /// Returns True if Form Number = 0
    /// if Form no is 0, then the curve is closed to axis
    /// if 1, the curve is closed to itself.
    pub fn is_closed_to_axis(&self) -> bool {
        self.form_number() == 0
    }
}
iges_entity! {
/// defines Sphere, Type `158` Form Number `0`
/// This defines a sphere with a center and radius
IgesSolidSphere, IgesSolidSphereState, "IgesSolidSphere", parent IgesDataIgesEntity, { radius: f64, center: Xyz }
}
impl IgesSolidSphere {
    iges_methods! {
    get radius = radius: f64;
    }
    iges_placement_methods! {
    point center = center;
    located_point transformed_center = center;
    }
    /// This method is used to set the fields of the class Sphere
    /// - aRadius : the radius of the sphere
    /// - aCenter : the center point coordinates (default (0,0,0))
    pub fn init(&self, a_radius: f64, a_center: Xyz) {
        {
            let mut state = self.state.write();
            state.radius = a_radius;
            state.center = a_center;
        }
        self.init_type_and_form(158, 0);
    }
}
iges_entity! {
/// defines SphericalSurface, Type `196` Form Number `0,1`
/// Spherical surface is defined by a center and radius.
/// In case of parametrised surface an axis and a
/// reference direction is provided.
IgesSolidSphericalSurface, IgesSolidSphericalSurfaceState, "IgesSolidSphericalSurface", parent IgesDataIgesEntity, { center: Handle<IgesGeomPoint>, radius: f64, axis: Handle<IgesGeomDirection>, ref_dir: Handle<IgesGeomDirection> }
}
impl IgesSolidSphericalSurface {
    iges_methods! {
    get center = center: Handle<IgesGeomPoint>;
    get radius = radius: f64;
    get axis = axis: Handle<IgesGeomDirection>;
    get reference_dir = ref_dir: Handle<IgesGeomDirection>;
    }
    iges_placement_methods! {
    has is_parametrised = ref_dir;
    }
    /// This method is used to set the fields of the class
    /// SphericalSurface
    /// - aCenter : the coordinates of the center point
    /// - aRadius : value of radius
    /// - anAxis  : the direction of the axis
    ///   Null in case of Unparametrised surface
    /// - aRefdir : the reference direction
    ///   Null in case of Unparametrised surface
    pub fn init(
        &self,
        a_center: &Handle<IgesGeomPoint>,
        a_radius: f64,
        an_axis: &Handle<IgesGeomDirection>,
        a_refdir: &Handle<IgesGeomDirection>,
    ) {
        {
            let mut state = self.state.write();
            state.center = a_center.clone();
            state.radius = a_radius;
            state.axis = an_axis.clone();
            state.ref_dir = a_refdir.clone();
        }
        self.init_type_and_form(
            196,
            if self.state.read().ref_dir.clone().is_null() {
                0
            } else {
                1
            },
        );
    }
    /// returns the center of the spherical surface after applying
    /// TransformationMatrix
    pub fn transformed_center(&self) -> Result<Point3, Failure> {
        let the_center = self.state.read().center.clone();
        if !self.has_transf() {
            Ok(iges_live(&the_center, "IgesSolidSphericalSurface::transformed_center")?.value())
        } else {
            let mut tmp: Xyz = Xyz::from(
                iges_live(&the_center, "IgesSolidSphericalSurface::transformed_center")?.value(),
            );
            self.location()?.transforms(&mut tmp);
            Ok(Point3::from(tmp))
        }
    }
}
iges_entity! {
/// defines ToroidalSurface, Type `198` Form Number `0,1`
/// This entity is defined by the center point, the axis
/// direction and the major and minor radii. In case of
/// parametrised surface a reference direction is provided.
IgesSolidToroidalSurface, IgesSolidToroidalSurfaceState, "IgesSolidToroidalSurface", parent IgesDataIgesEntity, { center: Handle<IgesGeomPoint>, axis: Handle<IgesGeomDirection>, major_radius: f64, minor_radius: f64, ref_dir: Handle<IgesGeomDirection> }
}
impl IgesSolidToroidalSurface {
    iges_methods! {
    get center = center: Handle<IgesGeomPoint>;
    get axis = axis: Handle<IgesGeomDirection>;
    get major_radius = major_radius: f64;
    get minor_radius = minor_radius: f64;
    get reference_dir = ref_dir: Handle<IgesGeomDirection>;
    }
    iges_placement_methods! {
    has is_parametrised = ref_dir;
    }
    /// This method is used to set the fields of the class
    /// ToroidalSurface
    /// - aCenter   : the center point coordinates
    /// - anAxis    : the direction of the axis
    /// - majRadius : the major radius
    /// - minRadius : the minor radius
    /// - Refdir    : the reference direction (parametrised)
    ///   default Null for unparametrised surface
    pub fn init(
        &self,
        a_center: &Handle<IgesGeomPoint>,
        an_axis: &Handle<IgesGeomDirection>,
        maj_radius: f64,
        min_radius: f64,
        refdir: &Handle<IgesGeomDirection>,
    ) {
        {
            let mut state = self.state.write();
            state.center = a_center.clone();
            state.axis = an_axis.clone();
            state.major_radius = maj_radius;
            state.minor_radius = min_radius;
            state.ref_dir = refdir.clone();
        }
        self.init_type_and_form(
            198,
            if self.state.read().ref_dir.clone().is_null() {
                0
            } else {
                1
            },
        );
    }
    /// returns the center point coordinates of the surface
    /// after applying TransformationMatrix
    pub fn transformed_center(&self) -> Result<Point3, Failure> {
        let the_center = self.state.read().center.clone();
        if !self.has_transf() {
            Ok(iges_live(&the_center, "IgesSolidToroidalSurface::transformed_center")?.value())
        } else {
            let mut tmp: Xyz = Xyz::from(
                iges_live(&the_center, "IgesSolidToroidalSurface::transformed_center")?.value(),
            );
            self.location()?.transforms(&mut tmp);
            Ok(Point3::from(tmp))
        }
    }
}
iges_entity! {
/// defines Torus, Type `160` Form Number `0`
/// A Torus is a solid formed by revolving a circular disc
/// about a specified coplanar axis.
IgesSolidTorus, IgesSolidTorusState, "IgesSolidTorus", parent IgesDataIgesEntity, { r1: f64, r2: f64, point: Xyz, axis: Xyz }
}
impl IgesSolidTorus {
    iges_methods! {
    get major_radius = r1: f64;
    get disc_radius = r2: f64;
    }
    iges_placement_methods! {
    point axis_point = point;
    located_point transformed_axis_point = point;
    direction axis = axis;
    located_direction transformed_axis = axis;
    }
    /// This method is used to set the fields of the class Torus
    /// - R1     : distance from center of torus to center
    ///   of circular disc to be revolved
    /// - R2     : radius of circular disc
    /// - aPoint : center point coordinates (default (0,0,0))
    /// - anAxis : unit vector in axis direction (default (0,0,1))
    pub fn init(&self, r1: f64, r2: f64, point: Xyz, axisdir: Xyz) {
        {
            let mut state = self.state.write();
            state.r1 = r1;
            state.r2 = r2;
            state.point = point;
            state.axis = axisdir;
        }
        self.init_type_and_form(160, 0);
    }
}
iges_entity! {
/// defines VertexList, Type `502` Form Number `1`
/// A vertex is a point in R3. A vertex is the bound of an
/// edge and can participate in the bounds of a face.
/// It contains one or more vertices.
IgesSolidVertexList, IgesSolidVertexListState, "IgesSolidVertexList", parent IgesDataIgesEntity, { vertices: Handle<HArray1<Xyz>> }
}
impl IgesSolidVertexList {
    iges_methods! {
    len nb_vertices = vertices;
    }
    /// This method is used to set the fields of the class
    /// VertexList
    /// - vertices : the vertices in the list
    pub fn init(&self, vertices: &Handle<HArray1<Xyz>>) -> Result<(), Failure> {
        if vertices.is_null() || iges_array1_lower(vertices, "IgesSolidVertexList::init")? != 1 {
            return Err(exceptions::dimension_mismatch("IgesSolidVertexList : Init"));
        }
        self.state.write().vertices = vertices.clone();
        self.init_type_and_form(502, 1);
        Ok(())
    }
    /// returns the num'th vertex in the list
    /// raises exception if num `= 0 or num ` NbVertices()
    pub fn vertex(&self, index: i32) -> Result<Point3, Failure> {
        let the_vertices = self.state.read().vertices.clone();
        Ok(Point3::from(iges_array1_item(
            &the_vertices,
            index,
            "IgesSolidVertexList::vertex",
        )?))
    }
}
