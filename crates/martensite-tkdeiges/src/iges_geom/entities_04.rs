#![allow(
    unused_assignments,
    clippy::if_same_then_else,
    clippy::collapsible_if,
    clippy::collapsible_else_if,
    clippy::manual_range_contains,
    clippy::assign_op_pattern,
    reason = "the members keep the form of the rules they state, and values a reader then ignores"
)]
use crate::iges_basic::entity_shape::{
    iges_array1_length, iges_array1_lower, iges_placement_methods,
};
use crate::iges_data::IgesDataIgesEntity;
use crate::iges_data::iges_array1_item;
use crate::{iges_entity, iges_methods};
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, exceptions};
use martensite_tkmath::gp::{GeneralTransform3, Point2, Point3, Vector3, Xy, Xyz};

iges_entity! {
/// defines IGESCopiousData, Type `106` Form `1-3,11-13,63`
/// This entity stores data points in the form of pairs,
/// triples, or sextuples. An interpretation flag value
/// signifies which of these forms is being used.
IgesGeomCopiousData, IgesGeomCopiousDataState, "IgesGeomCopiousData", parent IgesDataIgesEntity, { data_type: i32, z_plane: f64, data: Handle<HArray1<f64>> }
}
impl IgesGeomCopiousData {
    iges_methods! {
    get data_type = data_type: i32;
    get z_plane = z_plane: f64;
    }
    /// This method is used to set the fields of the class
    /// CopiousData
    /// - aDataType : Specifies whether data is a pair or a triple
    ///   or a sextuple.
    /// - aZPlane   : Common Z value for all points if datatype = 1
    /// - allData   : Data to be read in groups of 2, 3 or 6
    pub fn init(
        &self,
        a_data_type: i32,
        a_z_plane: f64,
        all_data: &Handle<HArray1<f64>>,
    ) -> Result<(), Failure> {
        if all_data.is_null() {
            return Err(exceptions::null_object(
                "IgesGeomCopiousData : Init with null data",
            ));
        }
        if iges_array1_lower(all_data, "IgesGeomCopiousData::init")? != 1 {
            return Err(exceptions::dimension_mismatch("IgesGeomCopiousData : Init"));
        }
        {
            let mut state = self.state.write();
            state.data_type = a_data_type;
            state.z_plane = a_z_plane;
            state.data = all_data.clone();
        }
        self.init_type_and_form(106, self.form_number());
        Ok(())
    }
    /// Sets Copious Data to be a Polyline if `mode` is True
    /// (Form = 11-12-13) or a Set of Points else (Form 1-2-3)
    pub fn set_polyline(&self, f: bool) {
        let the_data_type = self.state.read().data_type;
        let mut newfn: i32 = the_data_type;
        if f {
            newfn += 10;
        }
        self.init_type_and_form(106, newfn);
    }
    /// Sets Copious Data to be a Closed Path 2D (Form 63)
    /// Warning : DataType is not checked and must be set to ONE by Init
    pub fn set_closed_path2d(&self) {
        self.init_type_and_form(106, 63);
    }
    /// Returns True if `me` is a Set of Points (Form 1-2-3)
    pub fn is_point_set(&self) -> bool {
        self.form_number() < 10
    }
    /// Returns True if `me` is a Polyline (Form 11-12-13)
    pub fn is_polyline(&self) -> bool {
        self.form_number() / 10 == 1
    }
    /// Returns True if `me` is a Closed Path 2D (Form 63)
    pub fn is_closed_path2d(&self) -> bool {
        self.form_number() == 63
    }
    /// returns the number of tuples
    pub fn nb_points(&self) -> Result<i32, Failure> {
        let the_data = self.state.read().data.clone();
        let the_data_type = self.state.read().data_type;
        let mut nbtuples: i32 = 0;
        if the_data.is_null() {
            nbtuples = 0;
        } else {
            nbtuples = iges_array1_length(&the_data, "IgesGeomCopiousData::nb_points")?;
        }
        if the_data_type == 1 {
            nbtuples /= 2;
        } else if the_data_type == 2 {
            nbtuples /= 3;
        } else if the_data_type == 3 {
            nbtuples /= 6;
        }
        Ok(nbtuples)
    }
    /// Returns an individual Data, given the N0 of the Point
    /// and the B0 of the Coordinate (according DataType)
    pub fn data(&self, nump: i32, numdata: i32) -> Result<f64, Failure> {
        let the_data_type = self.state.read().data_type;
        let the_data = self.state.read().data.clone();
        let mut numd: i32 = 0;
        if the_data_type == 1 {
            numd = 2 * (nump - 1) + numdata;
        } else if the_data_type == 2 {
            numd = 3 * (nump - 1) + numdata;
        } else if the_data_type == 3 {
            numd = 6 * (nump - 1) + numdata;
        }
        iges_array1_item(&the_data, numd, "IgesGeomCopiousData::data")
    }
    /// returns the coordinates of the point specified by the anIndex
    /// raises exception if anIndex `= 0 or anIndex ` NbPoints()
    pub fn point(&self, an_index: i32) -> Result<Point3, Failure> {
        let the_data = self.state.read().data.clone();
        let the_data_type = self.state.read().data_type;
        let the_z_plane = self.state.read().z_plane;
        let lower: i32 = iges_array1_lower(&the_data, "IgesGeomCopiousData::point")?;
        let mut real_index: i32 = 0;
        let mut x: f64 = 0.0;
        let mut y: f64 = 0.0;
        let mut z: f64 = 0.0;
        if the_data_type == 1 {
            real_index = lower + 2 * (an_index - 1);
            x = iges_array1_item(&the_data, real_index, "IgesGeomCopiousData::point")?;
            y = iges_array1_item(&the_data, real_index + 1, "IgesGeomCopiousData::point")?;
            z = the_z_plane;
        }
        if the_data_type == 2 {
            real_index = lower + 3 * (an_index - 1);
            x = iges_array1_item(&the_data, real_index, "IgesGeomCopiousData::point")?;
            y = iges_array1_item(&the_data, real_index + 1, "IgesGeomCopiousData::point")?;
            z = iges_array1_item(&the_data, real_index + 2, "IgesGeomCopiousData::point")?;
        }
        if the_data_type == 3 {
            real_index = lower + 6 * (an_index - 1);
            x = iges_array1_item(&the_data, real_index, "IgesGeomCopiousData::point")?;
            y = iges_array1_item(&the_data, real_index + 1, "IgesGeomCopiousData::point")?;
            z = iges_array1_item(&the_data, real_index + 2, "IgesGeomCopiousData::point")?;
        }
        Ok(Point3::new(x, y, z))
    }
    /// returns the coordinates of the point specified by the anIndex
    /// after applying Transf. Matrix
    /// raises exception if anIndex `= 0 or anIndex ` NbPoints()
    pub fn transformed_point(&self, an_index: i32) -> Result<Point3, Failure> {
        if !self.has_transf() {
            return self.point(an_index);
        }
        let mut xyz: Xyz = Xyz::from(self.point(an_index)?);
        self.location()?.transforms(&mut xyz);
        Ok(Point3::from(xyz))
    }
    /// returns i, j, k values if 3-tuple else returns (0, 0, 0)
    /// raises exception if anIndex `= 0 or anIndex ` NbPoints()
    pub fn vector(&self, an_index: i32) -> Result<Vector3, Failure> {
        let the_data = self.state.read().data.clone();
        let the_data_type = self.state.read().data_type;
        let lower: i32 = iges_array1_lower(&the_data, "IgesGeomCopiousData::vector")?;
        let mut real_index: i32 = 0;
        if the_data_type != 3 {
            return Ok(Vector3::new(0.0, 0.0, 0.0));
        }
        real_index = lower + 6 * (an_index - 1) + 3;
        let i: f64 = iges_array1_item(&the_data, real_index, "IgesGeomCopiousData::vector")?;
        let j: f64 = iges_array1_item(&the_data, real_index + 1, "IgesGeomCopiousData::vector")?;
        let k: f64 = iges_array1_item(&the_data, real_index + 2, "IgesGeomCopiousData::vector")?;
        Ok(Vector3::new(i, j, k))
    }
    /// returns transformed vector if 3-tuple else returns (0, 0, 0)
    /// raises exception if anIndex `= 0 or anIndex ` NbPoints()
    pub fn transformed_vector(&self, an_index: i32) -> Result<Vector3, Failure> {
        if !self.has_transf() {
            return self.vector(an_index);
        }
        let mut xyz: Xyz = *self.vector(an_index)?.xyz();
        let mut loc: GeneralTransform3 = self.location()?;
        loc.set_translation_part(Xyz::new(0.0, 0.0, 0.0));
        loc.transforms(&mut xyz);
        Ok(Vector3::from(xyz))
    }
}
iges_entity! {
/// defines IGESCurveOnSurface, Type `142` Form `0`
/// A curve on a parametric surface entity associates a given
/// curve with a surface and identifies the curve as lying on
/// the surface.
IgesGeomCurveOnSurface, IgesGeomCurveOnSurfaceState, "IgesGeomCurveOnSurface", parent IgesDataIgesEntity, { creation_mode: i32, surface: Handle<IgesDataIgesEntity>, curve_uv: Handle<IgesDataIgesEntity>, curve3d: Handle<IgesDataIgesEntity>, preference_mode: i32 }
}
impl IgesGeomCurveOnSurface {
    iges_methods! {
    get creation_mode = creation_mode: i32;
    get surface = surface: Handle<IgesDataIgesEntity>;
    get curve_uv = curve_uv: Handle<IgesDataIgesEntity>;
    get curve3d = curve3d: Handle<IgesDataIgesEntity>;
    get preference_mode = preference_mode: i32;
    }
    /// This method is used to set the fields of the class
    /// CurveOnSurface
    /// - aMode       : Way the curve on the surface has been created
    /// - aSurface    : Surface on which the curve lies
    /// - aCurveUV    : Curve S (UV)
    /// - aCurve3D    : Curve C (3D)
    /// - aPreference : 0 = Unspecified
    ///   1 = S o B is preferred
    ///   2 = C is preferred
    ///   3 = C and S o B are equally preferred
    pub fn init(
        &self,
        a_mode: i32,
        a_surface: &Handle<IgesDataIgesEntity>,
        a_curve_uv: &Handle<IgesDataIgesEntity>,
        a_curve3d: &Handle<IgesDataIgesEntity>,
        a_preference: i32,
    ) {
        {
            let mut state = self.state.write();
            state.creation_mode = a_mode;
            state.surface = a_surface.clone();
            state.curve_uv = a_curve_uv.clone();
            state.curve3d = a_curve3d.clone();
            state.preference_mode = a_preference;
        }
        self.init_type_and_form(142, 0);
    }
}
iges_entity! {
/// defines IGESDirection, Type `123` Form `0`
/// A direction entity is a non-zero vector in Euclidean 3-space
/// that is defined by its three components (direction ratios)
/// with respect to the coordinate axes. If x, y, z are the
/// direction ratios then (x^2 + y^2 + z^2) ` 0
IgesGeomDirection, IgesGeomDirectionState, "IgesGeomDirection", parent IgesDataIgesEntity, { direction: Xyz }
}
impl IgesGeomDirection {
    iges_placement_methods! {
    vector value = direction;
    located_vector transformed_value = direction;
    }
    /// This method is used to set the fields of the class
    /// Direction
    /// - aDirection : Direction ratios, Z is 0 by default
    pub fn init(&self, a_direction: Xyz) {
        self.state.write().direction = a_direction;
        self.init_type_and_form(123, 0);
    }
}
iges_entity! {
/// defines IGESFlash, Type `125` Form `0 - 4`
/// A flash entity is a point in the ZT=0 plane that locates
/// a particular closed area. That closed area can be defined
/// in one of two ways. First, it can be an arbitrary closed
/// area defined by any entity capable of defining a closed
/// area. The points of this entity must all lie in the ZT=0
/// plane. Second, it can be a member of a predefined set of
/// flash shapes.
IgesGeomFlash, IgesGeomFlashState, "IgesGeomFlash", parent IgesDataIgesEntity, { point: Xy, dim1: f64, dim2: f64, rotation: f64, reference: Handle<IgesDataIgesEntity> }
}
impl IgesGeomFlash {
    iges_methods! {
    get dimension1 = dim1: f64;
    get dimension2 = dim2: f64;
    get rotation = rotation: f64;
    get reference_entity = reference: Handle<IgesDataIgesEntity>;
    }
    iges_placement_methods! {
    has has_reference_entity = reference;
    }
    /// This method is used to set the fields of the class Flash
    /// - aPoint     : Reference of flash
    /// - aDim       : First flash sizing parameter
    /// - anotherDim : Second flash sizing parameter
    /// - aRotation  : Rotation of flash about reference point
    ///   in radians
    /// - aReference : Pointer to the referenced entity or Null
    pub fn init(
        &self,
        a_point: Xy,
        a_dim1: f64,
        a_dim2: f64,
        a_rotation: f64,
        a_reference: &Handle<IgesDataIgesEntity>,
    ) {
        {
            let mut state = self.state.write();
            state.point = a_point;
            state.dim1 = a_dim1;
            state.dim2 = a_dim2;
            state.rotation = a_rotation;
            state.reference = a_reference.clone();
        }
        self.init_type_and_form(125, self.form_number());
    }
    /// Changes FormNumber (indicates the Nature of the Flash :
    /// 0 Unspecified, then given by Reference, 1-`4 various
    /// Specialisations (Circle,Rectangle, etc...) )
    /// Error if not in range [0-4]
    pub fn set_form_number(&self, form: i32) -> Result<(), Failure> {
        if form < 0 || form > 4 {
            return Err(exceptions::out_of_range("IgesGeomFlash : SetFormNumber"));
        }
        self.init_type_and_form(125, form);
        Ok(())
    }
    /// returns the referenced point, Z = 0 always
    pub fn reference_point(&self) -> Point2 {
        let the_point = self.state.read().point;
        Point2::from(the_point)
    }
    /// returns the referenced point after applying Transf. Matrix
    pub fn transformed_reference_point(&self) -> Result<Point3, Failure> {
        let the_point = self.state.read().point;
        let mut point: Xyz = Xyz::new(the_point.x(), the_point.y(), 0.0);
        if self.has_transf() {
            self.location()?.transforms(&mut point);
        }
        Ok(Point3::from(point))
    }
}
iges_entity! {
/// defines IGESLine, Type `110` Form `0`
/// A line is a bounded, connected portion of a parent straight
/// line which consists of more than one point. A line is
/// defined by its end points.
/// From IGES-5.3, two other Forms are admitted (same params) :
/// 0 remains for standard limited line (the default)
/// 1 for semi-infinite line (End is just a passing point)
/// 2 for full infinite Line (both Start and End are arbitrary)
IgesGeomLine, IgesGeomLineState, "IgesGeomLine", parent IgesDataIgesEntity, { start: Xyz, end: Xyz }
}
impl IgesGeomLine {
    iges_placement_methods! {
    point start_point = start;
    located_point transformed_start_point = start;
    point end_point = end;
    located_point transformed_end_point = end;
    }
    /// This method is used to set the fields of the class Line
    /// - aStart : Start point of the line
    /// - anEnd  : End point of the line
    pub fn init(&self, a_start: Xyz, an_end: Xyz) {
        {
            let mut state = self.state.write();
            state.start = a_start;
            state.end = an_end;
        }
        self.init_type_and_form(110, 0);
    }
    /// Returns the Infinite status i.e. the Form Number : 0 1 2
    pub fn infinite(&self) -> i32 {
        self.form_number()
    }
    /// Sets the Infinite status
    /// Does nothing if `status` is not 0 1 or 2
    pub fn set_infinite(&self, status: i32) {
        if status >= 0 && status <= 2 {
            self.init_type_and_form(110, status);
        }
    }
}
iges_entity! {
/// defines IGESOffsetCurve, Type `130` Form `0`
/// An OffsetCurve entity contains the data necessary to
/// determine the offset of a given curve C. This entity
/// points to the base curve to be offset and contains
/// offset distance and other pertinent information.
IgesGeomOffsetCurve, IgesGeomOffsetCurveState, "IgesGeomOffsetCurve", parent IgesDataIgesEntity, { base_curve: Handle<IgesDataIgesEntity>, offset_type: i32, function: Handle<IgesDataIgesEntity>, function_coord: i32, tapered_offset_type: i32, offset_distance1: f64, arc_length1: f64, offset_distance2: f64, arc_length2: f64, normal_vector: Xyz, offset_param1: f64, offset_param2: f64 }
}
impl IgesGeomOffsetCurve {
    iges_methods! {
    get base_curve = base_curve: Handle<IgesDataIgesEntity>;
    get offset_type = offset_type: i32;
    get function = function: Handle<IgesDataIgesEntity>;
    get function_parameter = function_coord: i32;
    get tapered_offset_type = tapered_offset_type: i32;
    get first_offset_distance = offset_distance1: f64;
    get arc_length1 = arc_length1: f64;
    get second_offset_distance = offset_distance2: f64;
    get arc_length2 = arc_length2: f64;
    get start_parameter = offset_param1: f64;
    get end_parameter = offset_param2: f64;
    }
    iges_placement_methods! {
    has has_function = function;
    vector normal_vector = normal_vector;
    located_vector transformed_normal_vector = normal_vector;
    }
    /// This method is used to set the fields of the class
    /// OffsetCurve
    /// - aBaseCurve         : The curve entity to be offset
    /// - anOffsetType       : Offset distance flag
    ///   1 = Single value, uniform distance
    ///   2 = Varying linearly
    ///   3 = As a specified function
    /// - aFunction          : Curve entity, one coordinate of which
    ///   describes offset as a function of its
    ///   parameter (0 unless OffsetType = 3)
    /// - aFunctionCoord     : Particular coordinate of curve
    ///   describing offset as function of its
    ///   parameters. (used if OffsetType = 3)
    /// - aTaperedOffsetType : Tapered offset type flag
    ///   1 = Function of arc length
    ///   2 = Function of parameter
    ///   (Only used if OffsetType = 2 or 3)
    /// - offDistance1       : First offset distance
    ///   (Only used if OffsetType = 1 or 2)
    /// - arcLength1         : Arc length or parameter value of
    ///   first offset distance
    ///   (Only used if OffsetType = 2)
    /// - offDistance2       : Second offset distance
    /// - arcLength2         : Arc length or parameter value of
    ///   second offset distance
    ///   (Only used if OffsetType = 2)
    /// - aNormalVec         : Unit vector normal to plane containing
    ///   curve to be offset
    /// - anOffsetParam      : Start parameter value of offset curve
    /// - anotherOffsetParam : End parameter value of offset curve
    #[allow(clippy::too_many_arguments, reason = "the member takes every field")]
    pub fn init(
        &self,
        a_base_curve: &Handle<IgesDataIgesEntity>,
        an_offset_type: i32,
        a_function: &Handle<IgesDataIgesEntity>,
        a_function_coord: i32,
        a_tapered_offset_type: i32,
        off_distance1: f64,
        an_arc_length1: f64,
        off_distance2: f64,
        an_arc_length2: f64,
        a_normal_vec: Xyz,
        an_offset_param: f64,
        another_offset_param: f64,
    ) {
        {
            let mut state = self.state.write();
            state.base_curve = a_base_curve.clone();
            state.offset_type = an_offset_type;
            state.function = a_function.clone();
            state.function_coord = a_function_coord;
            state.tapered_offset_type = a_tapered_offset_type;
            state.offset_distance1 = off_distance1;
            state.arc_length1 = an_arc_length1;
            state.offset_distance2 = off_distance2;
            state.arc_length2 = an_arc_length2;
            state.normal_vector = a_normal_vec;
            state.offset_param1 = an_offset_param;
            state.offset_param2 = another_offset_param;
        }
        self.init_type_and_form(130, 0);
    }
    /// `parameters`.
    pub fn parameters(&self, tt1: &mut f64, tt2: &mut f64) {
        let the_offset_param1 = self.state.read().offset_param1;
        let the_offset_param2 = self.state.read().offset_param2;
        *tt1 = the_offset_param1;
        *tt2 = the_offset_param2;
    }
}
