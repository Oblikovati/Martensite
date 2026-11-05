//! Actual DNamingFilletDriver fixture, using the native fillet and validity/history contracts.
use super::naming_function_values as values;
use super::{
    naming_box_driver::{function_result, real_argument, result_label},
    naming_function_graph::add_function,
    naming_history::{load_deleted, load_generated, load_modified, orientation_map},
    naming_transform_arguments::{argument_value, previous_function, set_object_argument},
};
use martensite_tkbrep::{
    topexp::Explorer,
    topods::{self, Shape, ShapeIterator},
    toptools::MapOfShape,
};
use martensite_tkcaf::tnaming::TNamingBuilder;
use martensite_tkernel::{
    impl_transient,
    message::ProgressRange,
    ncollection::List,
    precision,
    standard::{Failure, Handle, StandardTransient},
};
use martensite_tkfillet::{brep_fillet_api::BRepFilletAPIMakeFillet, chfi3d::FilletShape};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tklcaf::{
    guid_from_text,
    tdata_std::TDataStdUAttribute,
    tdf::TdfAttributeState,
    tfunction::{TFunctionDriver, TFunctionDriverBase, TFunctionFunction, TFunctionLogbook},
};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
use values::integer_argument;
use values::optional_failure as failed;

/// Native DNaming fillet driver; `NamingFilletDriver::default()`.
#[derive(Default)]
pub struct NamingFilletDriver {
    transient: StandardTransient,
    base: TFunctionDriverBase,
}
impl_transient!(NamingFilletDriver, "DNamingFilletDriver", dyn TFunctionDriver, transient, interfaces: [dyn TFunctionDriver]);
fn section_shape(raw: i32) -> Result<FilletShape, Failure> {
    match raw {
        0 => Ok(FilletShape::Rational),
        1 => Ok(FilletShape::QuasiAngular),
        2 => Ok(FilletShape::Polynomial),
        _ => Err(Failure::new(&format!(
            "fillet section value {raw}; expected declared 0, 1 or 2"
        ))),
    }
}
fn valid_inputs(
    function: &Handle<TFunctionFunction>,
) -> Result<Option<(Shape, Shape, f64, FilletShape)>, Failure> {
    let Some(source) = previous_source(function)? else {
        return Ok(None);
    };
    let radius = real_argument(function, 1)?.get().unwrap().get();
    let section = section_shape(integer_argument(function, 2)?.get().unwrap().get())?;
    if radius < precision::confusion() {
        return wrong_argument(function);
    }
    let Some((context, path)) = input_shapes(function, &source)? else {
        return Ok(None);
    };
    Ok(Some((context, path, radius, section)))
}
fn previous_source(
    function: &Handle<TFunctionFunction>,
) -> Result<Option<Handle<martensite_tkcaf::tnaming::TNamingNamedShape>>, Failure> {
    let previous = previous_function(function)?;
    if previous.is_null() {
        return Ok(None);
    }
    let source = function_result(&previous)?;
    if source.is_null() || source.is_empty() {
        return wrong_argument(function);
    }
    Ok(Some(source))
}

fn input_shapes(
    function: &Handle<TFunctionFunction>,
    source: &Handle<martensite_tkcaf::tnaming::TNamingNamedShape>,
) -> Result<Option<(Shape, Shape)>, Failure> {
    let named_path = argument_value(function, 3)?;
    if named_path.is_null() || named_path.is_empty() {
        return wrong_argument(function);
    }
    let path = named_path.get().unwrap().get()?;
    let context = source.get().unwrap().get()?;
    if path.is_null() || context.is_null() {
        return wrong_argument(function);
    }
    Ok(Some((context, path)))
}

fn wrong_argument<T>(function: &Handle<TFunctionFunction>) -> Result<Option<T>, Failure> {
    failed(function, 14)
}

fn add_path(
    maker: &mut BRepFilletAPIMakeFillet,
    path: &Shape,
    radius: f64,
) -> Result<bool, Failure> {
    if path.shape_type()? == ShapeEnum::Edge {
        maker.add_with_radius(radius, &topods::edge(path)?)?;
        return Ok(true);
    }
    if path.shape_type()? != ShapeEnum::Face {
        return Ok(false);
    }
    let mut seen = MapOfShape::default();
    for edge in Explorer::new(path, ShapeEnum::Edge, ShapeEnum::Shape)? {
        if seen.add(edge.clone()) {
            maker.add_with_radius(radius, &topods::edge(&edge)?)?;
        }
    }
    Ok(true)
}
fn checked_fillet(
    function: &Handle<TFunctionFunction>,
    context: &Shape,
    path: &Shape,
    radius: f64,
    section: FilletShape,
) -> Result<Option<BRepFilletAPIMakeFillet>, Failure> {
    let mut maker = BRepFilletAPIMakeFillet::new(context, section)?;
    if !add_path(&mut maker, path, radius)? {
        return wrong_argument(function);
    }
    maker.build(&ProgressRange::default())?;
    if !maker.is_done() {
        return failed(function, 11);
    }
    if !valid_fillet_result(&mut maker, context)? {
        return failed(function, 12);
    }
    Ok(Some(maker))
}
fn valid_fillet_result(
    maker: &mut BRepFilletAPIMakeFillet,
    context: &Shape,
) -> Result<bool, Failure> {
    let mut arguments = List::default();
    arguments.append(context.clone());
    martensite_tkbool::brep_algo::is_valid_result(&arguments, maker.shape()?, false, false)
}

fn load_result_shape(
    result: &martensite_tklcaf::tdf::TdfLabel,
    maker: &mut BRepFilletAPIMakeFillet,
    context: &Shape,
) -> Result<Shape, Failure> {
    let builder = TNamingBuilder::new(result)?;
    let mut shape = maker.shape()?.clone();
    if shape.shape_type()? == ShapeEnum::Compound
        && shape.nb_children() == 1
        && let Some(child) = Iterator::next(&mut ShapeIterator::new(&shape, true, true))
    {
        shape = child;
    }
    if shape.is_null() {
        builder.generated(&shape)?;
    } else {
        builder.modify(context, &shape)?;
    }
    Ok(shape)
}
struct FilletHistoryLabels<'a> {
    result: &'a martensite_tklcaf::tdf::TdfLabel,
    context: &'a Shape,
    orientations: martensite_tkcaf::tnaming::TNamingShapeShapeMap,
}
impl FilletHistoryLabels<'_> {
    fn generated(
        &self,
        maker: &mut BRepFilletAPIMakeFillet,
        tag: i32,
        kind: ShapeEnum,
    ) -> Result<(), Failure> {
        let builder = TNamingBuilder::new(&self.result.find_child(tag, true)?)?;
        load_generated(maker, self.context, kind, &builder, &self.orientations)
    }
    fn modified(&self, maker: &mut BRepFilletAPIMakeFillet) -> Result<(), Failure> {
        let builder = TNamingBuilder::new(&self.result.find_child(2, true)?)?;
        load_modified(
            maker,
            self.context,
            ShapeEnum::Face,
            &builder,
            &self.orientations,
        )
    }
    fn deleted(&self, maker: &mut BRepFilletAPIMakeFillet) -> Result<(), Failure> {
        let builder = TNamingBuilder::new(&self.result.find_child(4, true)?)?;
        load_deleted(maker, self.context, ShapeEnum::Face, &builder)
    }
}
fn load_fillet_names(
    result: &martensite_tklcaf::tdf::TdfLabel,
    maker: &mut BRepFilletAPIMakeFillet,
    context: &Shape,
) -> Result<(), Failure> {
    let shape = load_result_shape(result, maker, context)?;
    let labels = FilletHistoryLabels {
        result,
        context,
        orientations: orientation_map(&shape, &[ShapeEnum::Face])?,
    };
    labels.generated(maker, 1, ShapeEnum::Edge)?;
    labels.modified(maker)?;
    labels.generated(maker, 3, ShapeEnum::Vertex)?;
    labels.deleted(maker)
}

impl TFunctionDriver for NamingFilletDriver {
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
        let Some((context, path, radius, section)) = valid_inputs(&function)? else {
            return Ok(-1);
        };
        let Some(mut maker) = checked_fillet(&function, &context, &path, radius, section)? else {
            return Ok(-1);
        };
        let result = result_label(&function)?;
        load_fillet_names(&result, &mut maker, &context)?;
        log.set_valid(&result, true)?;
        function.set_failure(0)?;
        Ok(0)
    }
}
/// Original MakeNamingFillet uses radius25, section0, path argument3; `make_fillet(&object,&path)?`.
pub fn make_fillet(
    object: &Handle<TDataStdUAttribute>,
    path: &Handle<TDataStdUAttribute>,
) -> Result<Handle<TFunctionFunction>, Failure> {
    let function = add_function(
        object,
        &guid_from_text("12e94553-6dbc-11d4-b9c8-0060b0ee281b"),
    )?;
    real_argument(&function, 1)?.set_value(25.0)?;
    integer_argument(&function, 2)?.set_value(0)?;
    set_object_argument(&function, 3, path)?;
    assert_eq!(execute_fillet(&function)?, 0);
    Ok(function)
}
/// Execute the actual native fillet driver; `execute_fillet(&function)?`.
pub fn execute_fillet(function: &Handle<TFunctionFunction>) -> Result<i32, Failure> {
    let driver = NamingFilletDriver::default();
    driver.init(&function.label());
    let mut log = TFunctionLogbook::set(&function.label())?;
    log.clear()?;
    driver.execute(&mut log)
}
