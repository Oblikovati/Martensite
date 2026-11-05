//! Cylinder document driver, preserving finite-axis and fixed face-label semantics.
use super::naming_function_values as values;
use super::naming_transform_arguments::argument_value;
use martensite_tkbrep::{
    brep::BRepTool,
    brep_adaptor::BRepAdaptorCurve,
    topexp::{self, Explorer},
    topods,
};
use martensite_tkcaf::tnaming::TNaming;
use martensite_tkernel::{
    impl_transient,
    message::ProgressRange,
    standard::{Failure, Handle, StandardTransient},
};
use martensite_tkg3d::{adaptor3d::Adaptor3dCurve, top_abs::ShapeEnum};
use martensite_tklcaf::{
    tdf::TdfLabel,
    tfunction::{TFunctionDriver, TFunctionDriverBase, TFunctionFunction, TFunctionLogbook},
};
use martensite_tkmath::{geom_abs::CurveType, gp::Axis2};
use martensite_tkprim::{
    brep_prim::BRepPrimOneAxis,
    brep_prim_api::{BRepPrimAPIMakeCylinder, BRepPrimAPIMakeOneAxis},
};
use martensite_tktopalgo::{
    brep_builder_api::BRepBuilderAPIMakeShape, brep_check::BRepCheckAnalyzer,
};

/// Computes a cylinder from an edge or wire argument; `NamingCylinderDriver::default()`.
#[derive(Default)]
pub struct NamingCylinderDriver {
    transient: StandardTransient,
    base: TFunctionDriverBase,
}
impl_transient!(NamingCylinderDriver, "DNamingCylinderDriver", dyn TFunctionDriver, transient,
    interfaces: [dyn TFunctionDriver]);

fn first_axis_edge(shape: &topods::Shape) -> Result<Option<topods::Edge>, Failure> {
    match shape.shape_type()? {
        ShapeEnum::Edge => Ok(Some(topods::edge(shape)?)),
        ShapeEnum::Wire => {
            let explorer = Explorer::new(shape, ShapeEnum::Edge, ShapeEnum::Shape)?;
            Ok(Some(topods::edge(explorer.current())?))
        }
        _ => Ok(None),
    }
}

fn cylinder_axis(shape: &topods::Shape) -> Result<Option<Axis2>, Failure> {
    let Some(edge) = first_axis_edge(shape)? else {
        return Ok(None);
    };
    let curve = BRepAdaptorCurve::with_edge(&edge)?;
    if curve.get_type()? != CurveType::Line {
        return Ok(None);
    }
    let line = curve.line()?;
    let origin = if edge.infinite()? {
        line.location()
    } else {
        BRepTool::pnt(&topexp::vertices(&edge, false).0)?
    };
    Ok(Some(Axis2::new(origin, line.direction())?))
}

fn named_axis(function: &Handle<TFunctionFunction>) -> Result<Option<Axis2>, Failure> {
    let named = argument_value(function, 3)?;
    // A null native handle has no defined dereference behavior; keep a typed boundary here.
    if named.is_null() {
        return Err(Failure::new("null cylinder axis; expected named shape"));
    }
    if named.is_empty() {
        return Ok(None);
    }
    let shape = named.get().unwrap().get()?;
    if shape.is_null() {
        return Ok(None);
    }
    cylinder_axis(&shape)
}

fn generated_face(label: &TdfLabel, tag: i32, face: &topods::Face) -> Result<(), Failure> {
    values::record_primitive_shape(&label.find_child(tag, true)?, face)
}

/// Preserve fixed bottom/top/lateral/start/end tags; `load_cylinder_names(&label, &mut maker)?`.
pub(super) fn load_cylinder_names(
    label: &TdfLabel,
    maker: &mut BRepPrimAPIMakeCylinder,
) -> Result<(), Failure> {
    values::record_primitive_shape(label, &maker.solid()?.into_shape())?;
    let cylinder = maker.cylinder();
    if cylinder.has_bottom()? {
        generated_face(label, 1, &cylinder.bottom_face()?)?;
    }
    if cylinder.has_top()? {
        generated_face(label, 2, &cylinder.top_face()?)?;
    }
    generated_face(label, 3, &cylinder.lateral_face()?)?;
    if cylinder.has_sides() {
        generated_face(label, 4, &cylinder.start_face()?)?;
        generated_face(label, 5, &cylinder.end_face()?)?;
    }
    Ok(())
}

fn checked_cylinder(
    function: &Handle<TFunctionFunction>,
    axis: Axis2,
    radius: f64,
    height: f64,
) -> Result<Option<BRepPrimAPIMakeCylinder>, Failure> {
    let mut maker = BRepPrimAPIMakeCylinder::with_axes(axis, radius, height);
    maker.build(&ProgressRange::default())?;
    if !maker.is_done() {
        values::primitive_failure(function, 11)?;
        return Ok(None);
    }
    let solid = maker.solid()?;
    if !BRepCheckAnalyzer::new(&solid, true, false, false)?.is_valid_shape(&solid)? {
        values::primitive_failure(function, 12)?;
        return Ok(None);
    }
    Ok(Some(maker))
}

fn execute_cylinder(
    function: &Handle<TFunctionFunction>,
    log: &mut Handle<TFunctionLogbook>,
) -> Result<i32, Failure> {
    let radius = values::real_argument(function, 1)?.get().unwrap().get();
    let height = values::real_argument(function, 2)?.get().unwrap().get();
    let Some(axis) = named_axis(function)? else {
        return values::primitive_failure(function, 13);
    };
    let location = values::previous_location(function)?;
    let Some(mut maker) = checked_cylinder(function, axis, radius, height)? else {
        return Ok(-1);
    };
    publish_cylinder(function, log, &mut maker, &location)
}

fn publish_cylinder(
    function: &Handle<TFunctionFunction>,
    log: &mut Handle<TFunctionLogbook>,
    maker: &mut BRepPrimAPIMakeCylinder,
    location: &martensite_tkmath::toploc::Location,
) -> Result<i32, Failure> {
    let result = values::result_label(function)?;
    load_cylinder_names(&result, maker)?;
    if !location.is_identity() {
        TNaming::displace(&result, location, true)?;
    }
    log.set_valid(&result, true)?;
    function.set_failure(0)?;
    Ok(0)
}

impl TFunctionDriver for NamingCylinderDriver {
    fn driver_base(&self) -> &TFunctionDriverBase {
        &self.base
    }
    fn validate(&self, _log: &Handle<TFunctionLogbook>) -> Result<(), Failure> {
        Ok(())
    }
    fn must_execute(&self, _log: &Handle<TFunctionLogbook>) -> Result<bool, Failure> {
        Ok(true)
    }
    fn execute(&self, log: &mut Handle<TFunctionLogbook>) -> Result<i32, Failure> {
        let Some(function) = values::function_on(&self.label())? else {
            return Ok(-1);
        };
        execute_cylinder(&function, log)
    }
}
