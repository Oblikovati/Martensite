#![allow(
    unused_assignments,
    clippy::if_same_then_else,
    clippy::collapsible_if,
    clippy::collapsible_else_if,
    clippy::manual_range_contains,
    clippy::assign_op_pattern,
    reason = "the members keep the form of the rules they state, and values a reader then ignores"
)]
use crate::iges_basic::entity_shape::{iges_array2_fill, spline_coefficients};
use crate::iges_basic::nested_arrays::nested_entity_lists;
use crate::iges_basic::samples_01::sample_iges_basic_subfigure_def;
use crate::iges_basic::{IgesBasicHArray1OfHArray1OfIgesEntity, IgesBasicSubfigureDef};
use crate::iges_data::IgesDataIgesEntity;
use crate::iges_data::{iges_array1_set, iges_new_array1, iges_new_array2};
use crate::iges_geom::{
    IgesGeomBSplineCurve, IgesGeomBSplineSurface, IgesGeomBoundary, IgesGeomBoundedSurface,
    IgesGeomCircularArc, IgesGeomCompositeCurve, IgesGeomConicArc, IgesGeomCopiousData,
    IgesGeomCurveOnSurface, IgesGeomDirection, IgesGeomFlash, IgesGeomLine, IgesGeomOffsetCurve,
    IgesGeomOffsetSurface, IgesGeomPlane, IgesGeomPoint, IgesGeomRuledSurface, IgesGeomSplineCurve,
    IgesGeomSplineSurface, IgesGeomSurfaceOfRevolution, IgesGeomTabulatedCylinder,
    IgesGeomTransformationMatrix, IgesGeomTrimmedSurface,
};
use crate::iges_test_samples::sample_list;
use martensite_tkernel::ncollection::{HArray1, HArray2};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkmath::gp::{Xy, Xyz};

pub(crate) fn sample_iges_geom_b_spline_curve() -> Result<Handle<IgesGeomBSplineCurve>, Failure> {
    let object = Handle::new(IgesGeomBSplineCurve::new());
    let an_index: i32 = 2;
    let a_degree: i32 = 1;
    let a_planar: bool = true;
    let a_closed: bool = true;
    let a_polynom: bool = true;
    let a_periodic: bool = true;
    let all_knots: Handle<HArray1<f64>> = iges_new_array1::<f64>(-1, 3)?;
    let all_weights: Handle<HArray1<f64>> = iges_new_array1::<f64>(0, 2)?;
    let all_poles: Handle<HArray1<Xyz>> = iges_new_array1::<Xyz>(0, 2)?;
    let a_umin: f64 = 10.5;
    let a_umax: f64 = 11.5;
    let a_norm: Xyz = Xyz::new(12.5, 13.5, 14.5);
    object.init(
        an_index,
        a_degree,
        a_planar,
        a_closed,
        a_polynom,
        a_periodic,
        &all_knots,
        &all_weights,
        &all_poles,
        a_umin,
        a_umax,
        a_norm,
    )?;
    Ok(object)
}
pub(crate) fn sample_iges_geom_b_spline_surface() -> Result<Handle<IgesGeomBSplineSurface>, Failure>
{
    let object = Handle::new(IgesGeomBSplineSurface::new());
    let an_index_u: i32 = 1;
    let an_index_v: i32 = 1;
    let a_deg_u: i32 = 1;
    let a_deg_v: i32 = 1;
    let a_close_u: bool = true;
    let a_close_v: bool = true;
    let a_polynom: bool = true;
    let a_period_u: bool = true;
    let a_period_v: bool = true;
    let all_knots_u: Handle<HArray1<f64>> = iges_new_array1::<f64>(-1, 2)?;
    let all_knots_v: Handle<HArray1<f64>> = iges_new_array1::<f64>(-1, 2)?;
    let all_weights: Handle<HArray2<f64>> =
        iges_new_array2::<f64>(0, 1, 0, 1).and_then(|weights| {
            iges_array2_fill(&weights, 1.0, "sample")?;
            Ok(weights)
        })?;
    let all_poles: Handle<HArray2<Xyz>> = iges_new_array2::<Xyz>(0, 1, 0, 1)?;
    let a_umin: f64 = 14.5;
    let a_umax: f64 = 15.5;
    let a_vmin: f64 = 16.5;
    let a_vmax: f64 = 17.5;
    object.init(
        an_index_u,
        an_index_v,
        a_deg_u,
        a_deg_v,
        a_close_u,
        a_close_v,
        a_polynom,
        a_period_u,
        a_period_v,
        &all_knots_u,
        &all_knots_v,
        &all_weights,
        &all_poles,
        a_umin,
        a_umax,
        a_vmin,
        a_vmax,
    )?;
    Ok(object)
}
pub(crate) fn sample_iges_geom_boundary() -> Result<Handle<IgesGeomBoundary>, Failure> {
    let object = Handle::new(IgesGeomBoundary::new());
    let a_type: i32 = 2;
    let a_preference: i32 = 3;
    let a_surface: Handle<IgesDataIgesEntity> =
        sample_iges_geom_line()?.cast::<IgesDataIgesEntity>();
    let all_model_curves: Handle<HArray1<Handle<IgesDataIgesEntity>>> = sample_list([
        sample_iges_geom_line()?.cast::<IgesDataIgesEntity>(),
        sample_iges_geom_line()?.cast::<IgesDataIgesEntity>(),
    ])?;
    let all_senses: Handle<HArray1<i32>> = sample_list([3, 4])?;
    let all_parameter_curves: Handle<IgesBasicHArray1OfHArray1OfIgesEntity> =
        nested_entity_lists()?;
    object.init(
        a_type,
        a_preference,
        &a_surface,
        &all_model_curves,
        &all_senses,
        &all_parameter_curves,
    )?;
    Ok(object)
}
pub(crate) fn sample_iges_geom_bounded_surface() -> Result<Handle<IgesGeomBoundedSurface>, Failure>
{
    let object = Handle::new(IgesGeomBoundedSurface::new());
    let a_type: i32 = 2;
    let a_surface: Handle<IgesDataIgesEntity> =
        sample_iges_geom_line()?.cast::<IgesDataIgesEntity>();
    let a_boundary: Handle<HArray1<Handle<IgesGeomBoundary>>> =
        sample_list([sample_iges_geom_boundary()?, sample_iges_geom_boundary()?])?;
    object.init(a_type, &a_surface, &a_boundary)?;
    Ok(object)
}
pub(crate) fn sample_iges_geom_circular_arc() -> Result<Handle<IgesGeomCircularArc>, Failure> {
    let object = Handle::new(IgesGeomCircularArc::new());
    let a_zt: f64 = 1.5;
    let a_center: Xy = Xy::new(2.5, 3.5);
    let a_start: Xy = Xy::new(3.5, 4.5);
    let an_end: Xy = Xy::new(4.5, 5.5);
    object.init(a_zt, a_center, a_start, an_end);
    Ok(object)
}
pub(crate) fn sample_iges_geom_composite_curve() -> Result<Handle<IgesGeomCompositeCurve>, Failure>
{
    let object = Handle::new(IgesGeomCompositeCurve::new());
    let all_entities: Handle<HArray1<Handle<IgesDataIgesEntity>>> = sample_list([
        sample_iges_geom_line()?.cast::<IgesDataIgesEntity>(),
        sample_iges_geom_line()?.cast::<IgesDataIgesEntity>(),
    ])?;
    object.init(&all_entities)?;
    Ok(object)
}
pub(crate) fn sample_iges_geom_conic_arc() -> Result<Handle<IgesGeomConicArc>, Failure> {
    let object = Handle::new(IgesGeomConicArc::new());
    let a: f64 = 1.5;
    let b: f64 = 2.5;
    let c: f64 = 3.5;
    let d: f64 = 4.5;
    let e: f64 = 5.5;
    let f: f64 = 6.5;
    let zt: f64 = 7.5;
    let a_start: Xy = Xy::new(8.5, 9.5);
    let an_end: Xy = Xy::new(9.5, 10.5);
    object.init(a, b, c, d, e, f, zt, a_start, an_end);
    Ok(object)
}
pub(crate) fn sample_iges_geom_copious_data() -> Result<Handle<IgesGeomCopiousData>, Failure> {
    let object = Handle::new(IgesGeomCopiousData::new());
    let a_data_type: i32 = 2;
    let a_z_plane: f64 = 2.5;
    let all_data: Handle<HArray1<f64>> = iges_new_array1::<f64>(1, 3)?;
    for (rank, value) in [1.5, 2.5, 3.5].into_iter().enumerate() {
        iges_array1_set(&all_data, rank as i32 + 1, value, "sample")?;
    }
    object.init(a_data_type, a_z_plane, &all_data)?;
    object.set_polyline(false);
    Ok(object)
}
pub(crate) fn sample_iges_geom_curve_on_surface() -> Result<Handle<IgesGeomCurveOnSurface>, Failure>
{
    let object = Handle::new(IgesGeomCurveOnSurface::new());
    let a_mode: i32 = 2;
    let a_surface: Handle<IgesDataIgesEntity> =
        sample_iges_geom_line()?.cast::<IgesDataIgesEntity>();
    let a_curve_uv: Handle<IgesDataIgesEntity> =
        sample_iges_geom_line()?.cast::<IgesDataIgesEntity>();
    let a_curve3d: Handle<IgesDataIgesEntity> =
        sample_iges_geom_line()?.cast::<IgesDataIgesEntity>();
    let a_preference: i32 = 6;
    object.init(a_mode, &a_surface, &a_curve_uv, &a_curve3d, a_preference);
    Ok(object)
}
pub(crate) fn sample_iges_geom_direction() -> Result<Handle<IgesGeomDirection>, Failure> {
    let object = Handle::new(IgesGeomDirection::new());
    let a_direction: Xyz = Xyz::new(1.5, 2.5, 3.5);
    object.init(a_direction);
    Ok(object)
}
pub(crate) fn sample_iges_geom_flash() -> Result<Handle<IgesGeomFlash>, Failure> {
    let object = Handle::new(IgesGeomFlash::new());
    let a_point: Xy = Xy::new(1.5, 2.5);
    let a_dim1: f64 = 2.5;
    let a_dim2: f64 = 3.5;
    let a_rotation: f64 = 4.5;
    let a_reference: Handle<IgesDataIgesEntity> =
        sample_iges_geom_line()?.cast::<IgesDataIgesEntity>();
    object.init(a_point, a_dim1, a_dim2, a_rotation, &a_reference);
    Ok(object)
}
pub(crate) fn sample_iges_geom_line() -> Result<Handle<IgesGeomLine>, Failure> {
    let object = Handle::new(IgesGeomLine::new());
    let a_start: Xyz = Xyz::new(1.5, 2.5, 3.5);
    let an_end: Xyz = Xyz::new(2.5, 3.5, 4.5);
    object.init(a_start, an_end);
    Ok(object)
}
pub(crate) fn sample_iges_geom_offset_curve() -> Result<Handle<IgesGeomOffsetCurve>, Failure> {
    let object = Handle::new(IgesGeomOffsetCurve::new());
    let a_base_curve: Handle<IgesDataIgesEntity> =
        sample_iges_geom_line()?.cast::<IgesDataIgesEntity>();
    let an_offset_type: i32 = 3;
    let a_function: Handle<IgesDataIgesEntity> =
        sample_iges_geom_line()?.cast::<IgesDataIgesEntity>();
    let a_function_coord: i32 = 5;
    let a_tapered_offset_type: i32 = 6;
    let off_distance1: f64 = 6.5;
    let an_arc_length1: f64 = 7.5;
    let off_distance2: f64 = 8.5;
    let an_arc_length2: f64 = 9.5;
    let a_normal_vec: Xyz = Xyz::new(10.5, 11.5, 12.5);
    let an_offset_param: f64 = 11.5;
    let another_offset_param: f64 = 12.5;
    object.init(
        &a_base_curve,
        an_offset_type,
        &a_function,
        a_function_coord,
        a_tapered_offset_type,
        off_distance1,
        an_arc_length1,
        off_distance2,
        an_arc_length2,
        a_normal_vec,
        an_offset_param,
        another_offset_param,
    );
    Ok(object)
}
pub(crate) fn sample_iges_geom_offset_surface() -> Result<Handle<IgesGeomOffsetSurface>, Failure> {
    let object = Handle::new(IgesGeomOffsetSurface::new());
    let an_indicator: Xyz = Xyz::new(1.5, 2.5, 3.5);
    let a_distance: f64 = 2.5;
    let a_surface: Handle<IgesDataIgesEntity> =
        sample_iges_geom_line()?.cast::<IgesDataIgesEntity>();
    object.init(an_indicator, a_distance, &a_surface);
    Ok(object)
}
pub(crate) fn sample_iges_geom_plane() -> Result<Handle<IgesGeomPlane>, Failure> {
    let object = Handle::new(IgesGeomPlane::new());
    let a: f64 = 1.5;
    let b: f64 = 2.5;
    let c: f64 = 3.5;
    let d: f64 = 4.5;
    let a_curve: Handle<IgesDataIgesEntity> = sample_iges_geom_line()?.cast::<IgesDataIgesEntity>();
    let attach: Xyz = Xyz::new(6.5, 7.5, 8.5);
    let a_size: f64 = 7.5;
    object.init(a, b, c, d, &a_curve, attach, a_size);
    Ok(object)
}
pub(crate) fn sample_iges_geom_point() -> Result<Handle<IgesGeomPoint>, Failure> {
    let object = Handle::new(IgesGeomPoint::new());
    let a_point: Xyz = Xyz::new(1.5, 2.5, 3.5);
    let a_symbol: Handle<IgesBasicSubfigureDef> = sample_iges_basic_subfigure_def()?;
    object.init(a_point, &a_symbol);
    Ok(object)
}
pub(crate) fn sample_iges_geom_ruled_surface() -> Result<Handle<IgesGeomRuledSurface>, Failure> {
    let object = Handle::new(IgesGeomRuledSurface::new());
    let a_curve: Handle<IgesDataIgesEntity> = sample_iges_geom_line()?.cast::<IgesDataIgesEntity>();
    let another_curve: Handle<IgesDataIgesEntity> =
        sample_iges_geom_line()?.cast::<IgesDataIgesEntity>();
    let a_dir_flag: i32 = 4;
    let a_dev_flag: i32 = 5;
    object.init(&a_curve, &another_curve, a_dir_flag, a_dev_flag);
    Ok(object)
}
pub(crate) fn sample_iges_geom_spline_curve() -> Result<Handle<IgesGeomSplineCurve>, Failure> {
    let object = Handle::new(IgesGeomSplineCurve::new());
    let a_type: i32 = 2;
    let a_degree: i32 = 3;
    let nb_dimensions: i32 = 4;
    let all_break_points: Handle<HArray1<f64>> = iges_new_array1::<f64>(1, 2)?;
    let all_x_polynomials: Handle<HArray2<f64>> = iges_new_array2::<f64>(1, 1, 1, 4)?;
    let all_y_polynomials: Handle<HArray2<f64>> = iges_new_array2::<f64>(1, 1, 1, 4)?;
    let all_z_polynomials: Handle<HArray2<f64>> = iges_new_array2::<f64>(1, 1, 1, 4)?;
    let all_xvalues: Handle<HArray1<f64>> = iges_new_array1::<f64>(1, 4)?;
    let all_yvalues: Handle<HArray1<f64>> = iges_new_array1::<f64>(1, 4)?;
    let all_zvalues: Handle<HArray1<f64>> = iges_new_array1::<f64>(1, 4)?;
    object.init(
        a_type,
        a_degree,
        nb_dimensions,
        &all_break_points,
        &all_x_polynomials,
        &all_y_polynomials,
        &all_z_polynomials,
        &all_xvalues,
        &all_yvalues,
        &all_zvalues,
    )?;
    Ok(object)
}
pub(crate) fn sample_iges_geom_spline_surface() -> Result<Handle<IgesGeomSplineSurface>, Failure> {
    let object = Handle::new(IgesGeomSplineSurface::new());
    let a_boundary_type: i32 = 2;
    let a_patch_type: i32 = 3;
    let all_u_break_points: Handle<HArray1<f64>> = iges_new_array1::<f64>(1, 2)?;
    let all_v_break_points: Handle<HArray1<f64>> = iges_new_array1::<f64>(1, 2)?;
    let all_x_coeffs: Handle<HArray2<Handle<HArray1<f64>>>> = spline_coefficients()?;
    let all_y_coeffs: Handle<HArray2<Handle<HArray1<f64>>>> = spline_coefficients()?;
    let all_z_coeffs: Handle<HArray2<Handle<HArray1<f64>>>> = spline_coefficients()?;
    object.init(
        a_boundary_type,
        a_patch_type,
        &all_u_break_points,
        &all_v_break_points,
        &all_x_coeffs,
        &all_y_coeffs,
        &all_z_coeffs,
    )?;
    Ok(object)
}
pub(crate) fn sample_iges_geom_surface_of_revolution()
-> Result<Handle<IgesGeomSurfaceOfRevolution>, Failure> {
    let object = Handle::new(IgesGeomSurfaceOfRevolution::new());
    let an_axis: Handle<IgesGeomLine> = sample_iges_geom_line()?;
    let a_generatrix: Handle<IgesDataIgesEntity> =
        sample_iges_geom_line()?.cast::<IgesDataIgesEntity>();
    let a_start_angle: f64 = 3.5;
    let an_end_angle: f64 = 4.5;
    object.init(&an_axis, &a_generatrix, a_start_angle, an_end_angle);
    Ok(object)
}
pub(crate) fn sample_iges_geom_tabulated_cylinder()
-> Result<Handle<IgesGeomTabulatedCylinder>, Failure> {
    let object = Handle::new(IgesGeomTabulatedCylinder::new());
    let a_directrix: Handle<IgesDataIgesEntity> =
        sample_iges_geom_line()?.cast::<IgesDataIgesEntity>();
    let an_end: Xyz = Xyz::new(2.5, 3.5, 4.5);
    object.init(&a_directrix, an_end);
    Ok(object)
}
pub(crate) fn sample_iges_geom_transformation_matrix()
-> Result<Handle<IgesGeomTransformationMatrix>, Failure> {
    let object = Handle::new(IgesGeomTransformationMatrix::new());
    let a_matrix: Handle<HArray2<f64>> = iges_new_array2::<f64>(1, 3, 1, 4)?;
    object.init(&a_matrix)?;
    Ok(object)
}
pub(crate) fn sample_iges_geom_trimmed_surface() -> Result<Handle<IgesGeomTrimmedSurface>, Failure>
{
    let object = Handle::new(IgesGeomTrimmedSurface::new());
    let a_surface: Handle<IgesDataIgesEntity> =
        sample_iges_geom_line()?.cast::<IgesDataIgesEntity>();
    let a_flag: i32 = 3;
    let an_outer: Handle<IgesGeomCurveOnSurface> = sample_iges_geom_curve_on_surface()?;
    let an_inner: Handle<HArray1<Handle<IgesGeomCurveOnSurface>>> = sample_list([
        sample_iges_geom_curve_on_surface()?,
        sample_iges_geom_curve_on_surface()?,
    ])?;
    object.init(&a_surface, a_flag, &an_outer, &an_inner)?;
    Ok(object)
}
