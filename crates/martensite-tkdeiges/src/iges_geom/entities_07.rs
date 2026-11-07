#![allow(
    unused_assignments,
    clippy::if_same_then_else,
    clippy::collapsible_if,
    clippy::collapsible_else_if,
    clippy::manual_range_contains,
    clippy::assign_op_pattern,
    reason = "the members keep the form of the rules they state, and values a reader then ignores"
)]
use super::{IgesGeomCurveOnSurface, IgesGeomLine};
use crate::iges_basic::entity_shape::{
    iges_array1_lower, iges_array2_col_length, iges_array2_fill, iges_array2_row_length,
    iges_placement_methods,
};
use crate::iges_data::{IgesDataIgesEntity, IgesDataTransfEntity};
use crate::iges_data::{iges_array2_item, iges_array2_set, iges_new_array2};
use crate::{iges_entity, iges_methods};
use martensite_tkernel::ncollection::{HArray1, HArray2};
use martensite_tkernel::standard::{Failure, Handle, exceptions};
use martensite_tkmath::gp::{GeneralTransform3, Xyz};

iges_entity! {
/// defines IGESSurfaceOfRevolution, Type `120` Form `0`
/// A surface of revolution is defined by an axis of rotation
/// a generatrix, and start and terminate rotation angles. The
/// surface is created by rotating the generatrix about the axis
/// of rotation through the start and terminate rotation angles.
IgesGeomSurfaceOfRevolution, IgesGeomSurfaceOfRevolutionState, "IgesGeomSurfaceOfRevolution", parent IgesDataIgesEntity, { line: Handle<IgesGeomLine>, generatrix: Handle<IgesDataIgesEntity>, start_angle: f64, end_angle: f64 }
}
impl IgesGeomSurfaceOfRevolution {
    iges_methods! {
    get axis_of_revolution = line: Handle<IgesGeomLine>;
    get generatrix = generatrix: Handle<IgesDataIgesEntity>;
    get start_angle = start_angle: f64;
    get end_angle = end_angle: f64;
    }
    /// This method is used to set the fields of the class Line
    /// - anAxis      : Axis of revolution
    /// - aGeneratrix : The curve which is revolved about the axis
    /// - aStartAngle : Start angle of the surface of revolution
    /// - anEndAngle  : End angle of the surface of revolution
    pub fn init(
        &self,
        an_axis: &Handle<IgesGeomLine>,
        a_generatrix: &Handle<IgesDataIgesEntity>,
        a_start_angle: f64,
        an_end_angle: f64,
    ) {
        {
            let mut state = self.state.write();
            state.line = an_axis.clone();
            state.generatrix = a_generatrix.clone();
            state.start_angle = a_start_angle;
            state.end_angle = an_end_angle;
        }
        self.init_type_and_form(120, 0);
    }
}
iges_entity! {
/// defines IGESTabulatedCylinder, Type `122` Form `0`
/// A tabulated cylinder is a surface formed by moving a line
/// segment called generatrix parallel to itself along a curve
/// called directrix. The curve may be a line, circular arc,
/// conic arc, parametric spline curve, rational B-spline
/// curve or composite curve.
IgesGeomTabulatedCylinder, IgesGeomTabulatedCylinderState, "IgesGeomTabulatedCylinder", parent IgesDataIgesEntity, { directrix: Handle<IgesDataIgesEntity>, end: Xyz }
}
impl IgesGeomTabulatedCylinder {
    iges_methods! {
    get directrix = directrix: Handle<IgesDataIgesEntity>;
    }
    iges_placement_methods! {
    point end_point = end;
    located_point transformed_end_point = end;
    }
    /// This method is used to set the fields of the class
    /// TabulatedCylinder
    /// - aDirectrix : Directrix Curve of the tabulated cylinder
    /// - anEnd      : Coordinates of the terminate point of the
    ///   generatrix
    ///   The start point of the directrix is identical to the start
    ///   point of the generatrix
    pub fn init(&self, a_directrix: &Handle<IgesDataIgesEntity>, an_end: Xyz) {
        {
            let mut state = self.state.write();
            state.directrix = a_directrix.clone();
            state.end = an_end;
        }
        self.init_type_and_form(122, 0);
    }
}
iges_entity! {
/// defines IGESTransformationMatrix, Type `124` Form `0`
/// The transformation matrix entity transforms three-row column
/// vectors by means of matrix multiplication and then a vector
/// addition. This entity can be considered as an "operator"
/// entity in that it starts with the input vector, operates on
/// it as described above, and produces the output vector.
IgesGeomTransformationMatrix, IgesGeomTransformationMatrixState, "IgesGeomTransformationMatrix", kind dyn IgesDataTransfEntity, { data: Handle<HArray2<f64>> }
}
impl IgesGeomTransformationMatrix {
    iges_placement_methods! {
    item2 data = data: f64;
    }
    /// This method is used to set the fields of the class
    /// TransformationMatrix
    /// - aMatrix : 3 x 4 array containing elements of the
    ///   transformation matrix
    ///   raises exception if aMatrix is not 3 x 4 array
    pub fn init(&self, a_matrix: &Handle<HArray2<f64>>) -> Result<(), Failure> {
        if a_matrix.is_null() {
            self.state.write().data = iges_new_array2::<f64>(1, 3, 1, 4)?;
            iges_array2_fill(
                &self.state.read().data.clone(),
                0.0,
                "IgesGeomTransformationMatrix::init",
            )?;
            let mut i: i32 = 1;
            while i <= 3 {
                iges_array2_set(
                    &self.state.read().data.clone(),
                    i,
                    i,
                    1.0,
                    "IgesGeomTransformationMatrix::init",
                )?;
                i += 1;
            }
        }
        if iges_array2_row_length(a_matrix, "IgesGeomTransformationMatrix::init")? != 4
            || iges_array2_col_length(a_matrix, "IgesGeomTransformationMatrix::init")? != 3
        {
            return Err(exceptions::dimension_mismatch(
                "IgesGeomTransformationMatrix : Init",
            ));
        }
        self.state.write().data = a_matrix.clone();
        if self.state.read().data.clone().is_null() {
            return Ok(());
        }
        self.init_type_and_form(124, self.form_number());
        Ok(())
    }
    /// Changes FormNumber (indicates the Type of Transf :
    /// Transformation 0-1 or Coordinate System 10-11-12)
    /// Error if not in ranges [0-1] or [10-12]
    pub fn set_form_number(&self, fm: i32) -> Result<(), Failure> {
        if (fm < 0 || fm > 1) && (fm < 10 || fm > 12) {
            return Err(exceptions::out_of_range(
                "IgesGeomTransformationMatrix : SetFormNumber",
            ));
        }
        self.init_type_and_form(124, fm);
        Ok(())
    }
    /// returns the transformation matrix
    /// 4th row elements of GTrsf will always be 0, 0, 0, 1 (not defined)
    pub fn value(&self) -> Result<GeneralTransform3, Failure> {
        let the_data = self.state.read().data.clone();
        let mut matrix: GeneralTransform3 = GeneralTransform3::default();
        if !the_data.is_null() {
            for i in 1..=3 {
                for j in 1..=4 {
                    matrix.set_value(
                        i,
                        j,
                        iges_array2_item(&the_data, i, j, "IgesGeomTransformationMatrix::value")?,
                    )?;
                }
            }
        }
        Ok(matrix)
    }
}
impl IgesDataTransfEntity for IgesGeomTransformationMatrix {
    fn value(&self) -> Result<GeneralTransform3, Failure> {
        IgesGeomTransformationMatrix::value(self)
    }
}
iges_entity! {
/// defines IGESTrimmedSurface, Type `144` Form `0`
/// A simple closed curve in Euclidean plane divides the
/// plane in to two disjoint, open connected components; one
/// bounded, one unbounded. The bounded one is called the
/// interior region to the curve. Unbounded component is called
/// exterior region to the curve. The domain of the trimmed
/// surface is defined as the interior of the outer boundaries
/// and exterior of the inner boundaries and includes the
/// boundary curves.
IgesGeomTrimmedSurface, IgesGeomTrimmedSurfaceState, "IgesGeomTrimmedSurface", parent IgesDataIgesEntity, { surface: Handle<IgesDataIgesEntity>, flag: i32, outer_curve: Handle<IgesGeomCurveOnSurface>, inner_curves: Handle<HArray1<Handle<IgesGeomCurveOnSurface>>> }
}
impl IgesGeomTrimmedSurface {
    iges_methods! {
    get surface = surface: Handle<IgesDataIgesEntity>;
    get outer_contour = outer_curve: Handle<IgesGeomCurveOnSurface>;
    get outer_boundary_type = flag: i32;
    len nb_inner_contours = inner_curves;
    item inner_contour = inner_curves: Handle<IgesGeomCurveOnSurface>;
    }
    iges_placement_methods! {
    has has_outer_contour = outer_curve;
    }
    /// This method is used to set the fields of the class
    /// TrimmedSurface
    /// - aSurface  : Surface to be trimmed
    /// - aFlag     : Outer boundary type
    ///   False = The outer boundary is the boundary of
    ///   rectangle D which is the domain of the
    ///   surface to be trimmed
    ///   True  = otherwise
    /// - anOuter   : Closed curve which constitutes outer boundary
    /// - allInners : Array of closed curves which constitute the
    ///   inner boundary
    pub fn init(
        &self,
        a_surface: &Handle<IgesDataIgesEntity>,
        a_flag: i32,
        an_outer: &Handle<IgesGeomCurveOnSurface>,
        an_inner: &Handle<HArray1<Handle<IgesGeomCurveOnSurface>>>,
    ) -> Result<(), Failure> {
        if !an_inner.is_null() {
            if iges_array1_lower(an_inner, "IgesGeomTrimmedSurface::init")? != 1 {
                return Err(exceptions::dimension_mismatch(
                    "IgesGeomTrimmedSurface : Init",
                ));
            }
        }
        {
            let mut state = self.state.write();
            state.surface = a_surface.clone();
            state.flag = a_flag;
            state.outer_curve = an_outer.clone();
            state.inner_curves = an_inner.clone();
        }
        self.init_type_and_form(144, 0);
        Ok(())
    }
}
