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
    iges_array1_length, iges_array1_lower, iges_array2_col_length, iges_array2_lower_col,
    iges_array2_lower_row, iges_array2_row_length, iges_placement_methods,
};
use crate::iges_data::IgesDataIgesEntity;
use crate::iges_data::{iges_array1_item, iges_array2_item};
use crate::{iges_entity, iges_methods};
use martensite_tkernel::ncollection::{HArray1, HArray2};
use martensite_tkernel::standard::{Failure, Handle, exceptions};

iges_entity! {
/// Defines IGESSplineCurve, Type `112` Form `0`
/// The parametric spline is a sequence of parametric
/// polynomial segments. The curve could be of the type
/// Linear, Quadratic, Cubic, Wilson-Fowler, Modified
/// Wilson-Fowler, B-Spline. The N polynomial segments
/// are delimited by the break points:
/// T(1), T(2), T(3), ..., T(N+1).
IgesGeomSplineCurve, IgesGeomSplineCurveState, "IgesGeomSplineCurve", parent IgesDataIgesEntity, { type_: i32, degree: i32, nb_dimensions: i32, break_points: Handle<HArray1<f64>>, x_coords_polynomial: Handle<HArray2<f64>>, y_coords_polynomial: Handle<HArray2<f64>>, z_coords_polynomial: Handle<HArray2<f64>>, xvalues: Handle<HArray1<f64>>, yvalues: Handle<HArray1<f64>>, zvalues: Handle<HArray1<f64>> }
}
impl IgesGeomSplineCurve {
    iges_methods! {
    get spline_type = type_: i32;
    get degree = degree: i32;
    get nb_dimensions = nb_dimensions: i32;
    item break_point = break_points: f64;
    }
    /// This method is used to set the fields of the class
    /// SplineCurve
    /// - aType           : Spline Type
    ///   1 = Linear
    ///   2 = Quadratic
    ///   3 = Cubic
    ///   4 = Wilson-Fowler
    ///   5 = Modified Wilson-Fowler
    ///   6 = B Spline
    /// - aDegree         : Degree of continuity w.r.t. arc length
    /// - nbDimensions    : Number of dimensions
    ///   2 = Planar
    ///   3 = Non-planar
    /// - allBreakPoints  : Array of break points
    /// - allXPolynomials : X coordinate polynomials of segments
    /// - allYPolynomials : Y coordinate polynomials of segments
    /// - allZPolynomials : Z coordinate polynomials of segments
    /// - allXValues      : Values of 1st, 2nd, 3rd derivatives of
    ///   X polynomials at the terminate point
    ///   and values of X at terminate point
    /// - allYValues      : Values of 1st, 2nd, 3rd derivatives of
    ///   Y polynomials at the terminate point
    ///   and values of Y at terminate point
    /// - allZvalues      : Values of 1st, 2nd, 3rd derivatives of
    ///   Z polynomials at the terminate point
    ///   and values of Z at terminate point
    ///   raises exception if allXPolynomials, allYPolynomials
    ///   & allZPolynomials are not of same size OR allXValues, allYValues
    ///   & allZValues are not of size 4
    #[allow(clippy::too_many_arguments, reason = "the member takes every field")]
    pub fn init(
        &self,
        a_type: i32,
        a_degree: i32,
        nb_dimensions: i32,
        all_break_points: &Handle<HArray1<f64>>,
        all_x_polynomials: &Handle<HArray2<f64>>,
        all_y_polynomials: &Handle<HArray2<f64>>,
        all_z_polynomials: &Handle<HArray2<f64>>,
        all_xvalues: &Handle<HArray1<f64>>,
        all_yvalues: &Handle<HArray1<f64>>,
        all_zvalues: &Handle<HArray1<f64>>,
    ) -> Result<(), Failure> {
        let mut len: i32 = iges_array2_col_length(all_x_polynomials, "IgesGeomSplineCurve::init")?;
        if len != iges_array2_col_length(all_y_polynomials, "IgesGeomSplineCurve::init")?
            || len != iges_array2_col_length(all_z_polynomials, "IgesGeomSplineCurve::init")?
        {
            return Err(exceptions::dimension_mismatch(
                "IgesGeomSplineCurve : Column Length of HArray2s in Init",
            ));
        }
        if iges_array1_lower(all_break_points, "IgesGeomSplineCurve::init")? != 1
            || iges_array1_lower(all_xvalues, "IgesGeomSplineCurve::init")? != 1
            || iges_array1_lower(all_yvalues, "IgesGeomSplineCurve::init")? != 1
            || iges_array1_lower(all_zvalues, "IgesGeomSplineCurve::init")? != 1
            || iges_array2_lower_col(all_x_polynomials, "IgesGeomSplineCurve::init")? != 1
            || iges_array2_lower_row(all_x_polynomials, "IgesGeomSplineCurve::init")? != 1
            || iges_array2_lower_col(all_y_polynomials, "IgesGeomSplineCurve::init")? != 1
            || iges_array2_lower_row(all_y_polynomials, "IgesGeomSplineCurve::init")? != 1
            || iges_array2_lower_col(all_z_polynomials, "IgesGeomSplineCurve::init")? != 1
            || iges_array2_lower_row(all_z_polynomials, "IgesGeomSplineCurve::init")? != 1
        {
            return Err(exceptions::dimension_mismatch(
                "IgesGeomSplineCurve : Lower Indices of HArrays in Init",
            ));
        }
        len = iges_array2_row_length(all_x_polynomials, "IgesGeomSplineCurve::init")?;
        if len != iges_array2_row_length(all_y_polynomials, "IgesGeomSplineCurve::init")?
            || len != iges_array2_row_length(all_z_polynomials, "IgesGeomSplineCurve::init")?
        {
            return Err(exceptions::dimension_mismatch(
                "IgesGeomSplineCurve : Row Length of HArray2s in Init",
            ));
        }
        len = iges_array1_length(all_xvalues, "IgesGeomSplineCurve::init")?;
        if len != iges_array1_length(all_yvalues, "IgesGeomSplineCurve::init")?
            || len != iges_array1_length(all_zvalues, "IgesGeomSplineCurve::init")?
        {
            return Err(exceptions::dimension_mismatch(
                "IgesGeomSplineCurve : Length of HArray1s in Init",
            ));
        }
        {
            let mut state = self.state.write();
            state.type_ = a_type;
            state.degree = a_degree;
            state.nb_dimensions = nb_dimensions;
            state.break_points = all_break_points.clone();
            state.x_coords_polynomial = all_x_polynomials.clone();
            state.y_coords_polynomial = all_y_polynomials.clone();
            state.z_coords_polynomial = all_z_polynomials.clone();
            state.xvalues = all_xvalues.clone();
            state.yvalues = all_yvalues.clone();
            state.zvalues = all_zvalues.clone();
        }
        self.init_type_and_form(112, 0);
        Ok(())
    }
    /// returns the number of segments
    pub fn nb_segments(&self) -> Result<i32, Failure> {
        let the_break_points = self.state.read().break_points.clone();
        if the_break_points.is_null() {
            Ok(0)
        } else {
            Ok(iges_array1_length(&the_break_points, "IgesGeomSplineCurve::nb_segments")? - 1)
        }
    }
    /// returns X coordinate polynomial for segment referred to by Index
    /// raises exception if Index `= 0 or Index ` NbSegments()
    pub fn x_coord_polynomial(
        &self,
        index: i32,
        ax: &mut f64,
        bx: &mut f64,
        cx: &mut f64,
        dx: &mut f64,
    ) -> Result<(), Failure> {
        let the_x_coords_polynomial = self.state.read().x_coords_polynomial.clone();
        *ax = iges_array2_item(
            &the_x_coords_polynomial,
            index,
            1,
            "IgesGeomSplineCurve::x_coord_polynomial",
        )?;
        *bx = iges_array2_item(
            &the_x_coords_polynomial,
            index,
            2,
            "IgesGeomSplineCurve::x_coord_polynomial",
        )?;
        *cx = iges_array2_item(
            &the_x_coords_polynomial,
            index,
            3,
            "IgesGeomSplineCurve::x_coord_polynomial",
        )?;
        *dx = iges_array2_item(
            &the_x_coords_polynomial,
            index,
            4,
            "IgesGeomSplineCurve::x_coord_polynomial",
        )?;
        Ok(())
    }
    /// returns Y coordinate polynomial for segment referred to by Index
    /// raises exception if Index `= 0 or Index ` NbSegments()
    pub fn y_coord_polynomial(
        &self,
        index: i32,
        ay: &mut f64,
        by: &mut f64,
        cy: &mut f64,
        dy: &mut f64,
    ) -> Result<(), Failure> {
        let the_y_coords_polynomial = self.state.read().y_coords_polynomial.clone();
        *ay = iges_array2_item(
            &the_y_coords_polynomial,
            index,
            1,
            "IgesGeomSplineCurve::y_coord_polynomial",
        )?;
        *by = iges_array2_item(
            &the_y_coords_polynomial,
            index,
            2,
            "IgesGeomSplineCurve::y_coord_polynomial",
        )?;
        *cy = iges_array2_item(
            &the_y_coords_polynomial,
            index,
            3,
            "IgesGeomSplineCurve::y_coord_polynomial",
        )?;
        *dy = iges_array2_item(
            &the_y_coords_polynomial,
            index,
            4,
            "IgesGeomSplineCurve::y_coord_polynomial",
        )?;
        Ok(())
    }
    /// returns Z coordinate polynomial for segment referred to by Index
    /// raises exception if Index `= 0 or Index ` NbSegments()
    pub fn z_coord_polynomial(
        &self,
        index: i32,
        az: &mut f64,
        bz: &mut f64,
        cz: &mut f64,
        dz: &mut f64,
    ) -> Result<(), Failure> {
        let the_z_coords_polynomial = self.state.read().z_coords_polynomial.clone();
        *az = iges_array2_item(
            &the_z_coords_polynomial,
            index,
            1,
            "IgesGeomSplineCurve::z_coord_polynomial",
        )?;
        *bz = iges_array2_item(
            &the_z_coords_polynomial,
            index,
            2,
            "IgesGeomSplineCurve::z_coord_polynomial",
        )?;
        *cz = iges_array2_item(
            &the_z_coords_polynomial,
            index,
            3,
            "IgesGeomSplineCurve::z_coord_polynomial",
        )?;
        *dz = iges_array2_item(
            &the_z_coords_polynomial,
            index,
            4,
            "IgesGeomSplineCurve::z_coord_polynomial",
        )?;
        Ok(())
    }
    /// returns the value of X polynomial, the values of 1st, 2nd and
    /// 3rd derivatives of the X polynomial at the terminate point
    pub fn x_values(
        &self,
        tpx0: &mut f64,
        tpx1: &mut f64,
        tpx2: &mut f64,
        tpx3: &mut f64,
    ) -> Result<(), Failure> {
        let the_xvalues = self.state.read().xvalues.clone();
        *tpx0 = iges_array1_item(&the_xvalues, 1, "IgesGeomSplineCurve::x_values")?;
        *tpx1 = iges_array1_item(&the_xvalues, 2, "IgesGeomSplineCurve::x_values")?;
        *tpx2 = iges_array1_item(&the_xvalues, 3, "IgesGeomSplineCurve::x_values")?;
        *tpx3 = iges_array1_item(&the_xvalues, 4, "IgesGeomSplineCurve::x_values")?;
        Ok(())
    }
    /// returns the value of Y polynomial, the values of 1st, 2nd and
    /// 3rd derivatives of the Y polynomial at the termminate point
    pub fn y_values(
        &self,
        tpy0: &mut f64,
        tpy1: &mut f64,
        tpy2: &mut f64,
        tpy3: &mut f64,
    ) -> Result<(), Failure> {
        let the_yvalues = self.state.read().yvalues.clone();
        *tpy0 = iges_array1_item(&the_yvalues, 1, "IgesGeomSplineCurve::y_values")?;
        *tpy1 = iges_array1_item(&the_yvalues, 2, "IgesGeomSplineCurve::y_values")?;
        *tpy2 = iges_array1_item(&the_yvalues, 3, "IgesGeomSplineCurve::y_values")?;
        *tpy3 = iges_array1_item(&the_yvalues, 4, "IgesGeomSplineCurve::y_values")?;
        Ok(())
    }
    /// returns the value of Z polynomial, the values of 1st, 2nd and
    /// 3rd derivatives of the Z polynomial at the termminate point
    pub fn z_values(
        &self,
        tpz0: &mut f64,
        tpz1: &mut f64,
        tpz2: &mut f64,
        tpz3: &mut f64,
    ) -> Result<(), Failure> {
        let the_zvalues = self.state.read().zvalues.clone();
        *tpz0 = iges_array1_item(&the_zvalues, 1, "IgesGeomSplineCurve::z_values")?;
        *tpz1 = iges_array1_item(&the_zvalues, 2, "IgesGeomSplineCurve::z_values")?;
        *tpz2 = iges_array1_item(&the_zvalues, 3, "IgesGeomSplineCurve::z_values")?;
        *tpz3 = iges_array1_item(&the_zvalues, 4, "IgesGeomSplineCurve::z_values")?;
        Ok(())
    }
}
iges_entity! {
/// defines IGESSplineSurface, Type `114` Form `0`
/// A parametric spline surface is a grid of polynomial
/// patches. Patch could be of the type Linear, Quadratic,
/// Cubic, Wilson-Fowler, Modified Wilson-Fowler, B-Spline
/// The M * N grid of patches is defined by the 'u' break
/// points TU(1), TU(2), ..., TU(M+1) and the 'v' break
/// points TV(1), TV(2), TV(3) ..., TV(N+1).
IgesGeomSplineSurface, IgesGeomSplineSurfaceState, "IgesGeomSplineSurface", parent IgesDataIgesEntity, { boundary_type: i32, patch_type: i32, u_break_points: Handle<HArray1<f64>>, v_break_points: Handle<HArray1<f64>>, x_coeffs: Handle<HArray2<Handle<HArray1<f64>>>>, y_coeffs: Handle<HArray2<Handle<HArray1<f64>>>>, z_coeffs: Handle<HArray2<Handle<HArray1<f64>>>> }
}
impl IgesGeomSplineSurface {
    iges_methods! {
    get boundary_type = boundary_type: i32;
    get patch_type = patch_type: i32;
    item u_break_point = u_break_points: f64;
    item v_break_point = v_break_points: f64;
    }
    iges_placement_methods! {
    item2 x_polynomial = x_coeffs: Handle<HArray1<f64>>;
    item2 y_polynomial = y_coeffs: Handle<HArray1<f64>>;
    item2 z_polynomial = z_coeffs: Handle<HArray1<f64>>;
    }
    /// This method is used to set the fields of the class
    /// SplineSurface
    /// - aBoundaryType   : Type of Spline boundary
    ///   1 = Linear
    ///   2 = Quadratic
    ///   3 = Cubic
    ///   4 = Wilson-Fowler
    ///   5 = Modified Wilson-Fowler
    ///   6 = B-spline
    /// - aPatchType      : Type of patch contained in the grid
    ///   1 = Cartesian Product
    ///   0 = Unspecified
    /// - allUBreakpoints : u values of grid lines
    /// - allVBreakpoints : v values of grid lines
    /// - allXCoeffs      : X coefficients of M x N patches
    /// - allYCoeffs      : Y coefficients of M x N patches
    /// - allZCoeffs      : Z coefficients of M x N patches
    ///   raises exception if allXCoeffs, allYCoeffs & allZCoeffs are not
    ///   of the same size.
    ///   or if the size of each element of the double array is not 16
    #[allow(clippy::too_many_arguments, reason = "the member takes every field")]
    pub fn init(
        &self,
        a_boundary_type: i32,
        a_patch_type: i32,
        all_u_break_points: &Handle<HArray1<f64>>,
        all_v_break_points: &Handle<HArray1<f64>>,
        all_x_coeffs: &Handle<HArray2<Handle<HArray1<f64>>>>,
        all_y_coeffs: &Handle<HArray2<Handle<HArray1<f64>>>>,
        all_z_coeffs: &Handle<HArray2<Handle<HArray1<f64>>>>,
    ) -> Result<(), Failure> {
        if iges_array1_lower(all_u_break_points, "IgesGeomSplineSurface::init")? != 1
            || iges_array1_lower(all_v_break_points, "IgesGeomSplineSurface::init")? != 1
        {
            return Err(exceptions::dimension_mismatch(
                "IgesGeomSplineSurface: Lower Indices of BreakPoints in Init",
            ));
        }
        let nb_u_segs: i32 =
            iges_array1_length(all_u_break_points, "IgesGeomSplineSurface::init")? - 1;
        let nb_v_segs: i32 =
            iges_array1_length(all_v_break_points, "IgesGeomSplineSurface::init")? - 1;
        let mut len: i32 = iges_array2_row_length(all_x_coeffs, "IgesGeomSplineSurface::init")?;
        if len != iges_array2_row_length(all_y_coeffs, "IgesGeomSplineSurface::init")?
            || len != iges_array2_row_length(all_z_coeffs, "IgesGeomSplineSurface::init")?
        {
            return Err(exceptions::dimension_mismatch(
                "IgesGeomSplineSurface: Row Length of HArray2s in Init",
            ));
        }
        if iges_array2_lower_col(all_x_coeffs, "IgesGeomSplineSurface::init")? != 1
            || iges_array2_lower_row(all_x_coeffs, "IgesGeomSplineSurface::init")? != 1
            || iges_array2_lower_col(all_y_coeffs, "IgesGeomSplineSurface::init")? != 1
            || iges_array2_lower_row(all_y_coeffs, "IgesGeomSplineSurface::init")? != 1
            || iges_array2_lower_col(all_z_coeffs, "IgesGeomSplineSurface::init")? != 1
            || iges_array2_lower_row(all_z_coeffs, "IgesGeomSplineSurface::init")? != 1
        {
            return Err(exceptions::dimension_mismatch(
                "IgesGeomSplineSurface: Lower Col-Row Indices of HArray2s in Init",
            ));
        }
        len = iges_array2_col_length(all_x_coeffs, "IgesGeomSplineSurface::init")?;
        if len != iges_array2_col_length(all_y_coeffs, "IgesGeomSplineSurface::init")?
            || len != iges_array2_col_length(all_z_coeffs, "IgesGeomSplineSurface::init")?
        {
            return Err(exceptions::dimension_mismatch(
                "IgesGeomSplineSurface: Column Length of HArray2s in Init",
            ));
        }
        let mut temp1: Handle<HArray1<f64>> = Handle::default();
        let mut temp2: Handle<HArray1<f64>> = Handle::default();
        let mut temp3: Handle<HArray1<f64>> = Handle::default();
        for i in 1..=nb_u_segs {
            for j in 1..=nb_v_segs {
                temp1 = iges_array2_item(all_x_coeffs, i, j, "IgesGeomSplineSurface::init")?;
                temp2 = iges_array2_item(all_y_coeffs, i, j, "IgesGeomSplineSurface::init")?;
                temp3 = iges_array2_item(all_z_coeffs, i, j, "IgesGeomSplineSurface::init")?;
                if temp1.is_null()
                    || iges_array1_length(&temp1, "IgesGeomSplineSurface::init")? != 16
                    || (temp2.is_null()
                        || iges_array1_length(&temp2, "IgesGeomSplineSurface::init")? != 16)
                    || (temp3.is_null()
                        || iges_array1_length(&temp3, "IgesGeomSplineSurface::init")? != 16)
                {
                    return Err(exceptions::dimension_mismatch(
                        "IgesGeomSplineSurface: Lengths of elements of HArray2s in Init",
                    ));
                }
            }
        }
        {
            let mut state = self.state.write();
            state.boundary_type = a_boundary_type;
            state.patch_type = a_patch_type;
            state.u_break_points = all_u_break_points.clone();
            state.v_break_points = all_v_break_points.clone();
            state.x_coeffs = all_x_coeffs.clone();
            state.y_coeffs = all_y_coeffs.clone();
            state.z_coeffs = all_z_coeffs.clone();
        }
        self.init_type_and_form(114, 0);
        Ok(())
    }
    /// returns the number of U segments
    pub fn nb_u_segments(&self) -> Result<i32, Failure> {
        let the_u_break_points = self.state.read().u_break_points.clone();
        Ok(iges_array1_length(&the_u_break_points, "IgesGeomSplineSurface::nb_u_segments")? - 1)
    }
    /// returns the number of V segments
    pub fn nb_v_segments(&self) -> Result<i32, Failure> {
        let the_v_break_points = self.state.read().v_break_points.clone();
        Ok(iges_array1_length(&the_v_break_points, "IgesGeomSplineSurface::nb_v_segments")? - 1)
    }
    /// returns in one all the polynomial values "in bulk"
    /// useful for massive treatments
    pub fn polynomials(
        &self,
        all_x_coeffs: &mut Handle<HArray2<Handle<HArray1<f64>>>>,
        all_y_coeffs: &mut Handle<HArray2<Handle<HArray1<f64>>>>,
        all_z_coeffs: &mut Handle<HArray2<Handle<HArray1<f64>>>>,
    ) {
        let the_x_coeffs = self.state.read().x_coeffs.clone();
        let the_y_coeffs = self.state.read().y_coeffs.clone();
        let the_z_coeffs = self.state.read().z_coeffs.clone();
        *all_x_coeffs = the_x_coeffs.clone();
        *all_y_coeffs = the_y_coeffs.clone();
        *all_z_coeffs = the_z_coeffs.clone();
    }
}
