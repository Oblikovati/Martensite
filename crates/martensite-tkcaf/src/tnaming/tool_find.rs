//! `TNamingTool::FindShape`: re-finds a selection among the shapes of the external
//! arguments its label references.
use super::tool_shapes::sub_shape_type;
use super::{TNamingNamedShape, TNamingNaming, TNamingTool};
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::Shape;
use martensite_tkbrep::toptools::MapOfShape;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tklcaf::live;
use martensite_tklcaf::tdf::{TdfAttributeMap, TdfAttributeState, TdfLabelMap, TdfTool};

impl TNamingTool {
    /// `FindShape(Valid, Forbiden, Arg, S)`: the sub-shape of the context of an external
    /// argument whose sub-shapes (one level down) are those of `Arg`'s shape; `S` stays as
    /// it is when none matches. `Forbiden` is unused, as natively.
    pub fn find_shape(
        valid: &TdfLabelMap,
        _forbiden: &TdfLabelMap,
        argument: &Handle<TNamingNamedShape>,
        shape: &mut Shape,
    ) -> Result<(), Failure> {
        let named = live(argument, "TNamingTool::FindShape")?;
        if !valid.is_empty() && !valid.contains(&named.label()) {
            return Ok(());
        }
        if named.is_empty() {
            return Ok(());
        }
        let mut naming: Handle<TNamingNaming> = Handle::default();
        if !named.find_attribute_as(&TNamingNaming::get_id(), &mut naming)? {
            return Ok(());
        }
        let shape_type = live(&naming, "TNamingTool::FindShape")?
            .get_name()
            .shape_type();
        let sub_shapes = sub_shapes_of(&named.get()?, shape_type)?;
        for external in external_arguments(named)? {
            if let Some(found) = coinciding_shape(&external, shape_type, &sub_shapes)? {
                *shape = found;
                return Ok(());
            }
        }
        Ok(())
    }
}

/// The sub-shapes of the type after `shape_type` (none past `TopAbsSHAPE`).
fn sub_shapes_of(shape: &Shape, shape_type: ShapeEnum) -> Result<MapOfShape, Failure> {
    let mut sub_shapes = MapOfShape::default();
    let Some(sub_type) = sub_shape_type(shape_type) else {
        return Ok(sub_shapes);
    };
    let mut explorer = Explorer::new(shape, sub_type, ShapeEnum::Shape)?;
    while explorer.more() {
        sub_shapes.add(explorer.current().clone());
        explorer.next()?;
    }
    Ok(sub_shapes)
}

/// The current named shapes (not empty) of the named shapes the label refers to, in the
/// order of the native reference map.
fn external_arguments(
    named: &TNamingNamedShape,
) -> Result<Vec<Handle<TNamingNamedShape>>, Failure> {
    let mut references = TdfAttributeMap::default();
    TdfTool::out_references(&named.label(), &mut references)?;
    let mut arguments = Vec::new();
    for attribute in references.iter() {
        if attribute.dynamic_type().map(|t| t.name()) != Some("TNamingNamedShape") {
            continue;
        }
        let current = TNamingTool::current_named_shape(&attribute.cast())?;
        if current.get().is_some_and(|current| !current.is_empty()) {
            arguments.push(current);
        }
    }
    Ok(arguments)
}

/// The context of an external argument (the named shape of its father label, else the
/// argument itself) and its sub-shape of `shape_type` whose sub-shapes coincide with
/// `sub_shapes`.
fn coinciding_shape(
    external: &Handle<TNamingNamedShape>,
    shape_type: ShapeEnum,
    sub_shapes: &MapOfShape,
) -> Result<Option<Shape>, Failure> {
    let external_ref = live(external, "TNamingTool::FindShape")?;
    let father = external_ref.label().father();
    let mut context: Handle<TNamingNamedShape> = Handle::default();
    if father.is_null() || !father.find_attribute_as(&TNamingNamedShape::get_id(), &mut context)? {
        context = external.clone();
    }
    let context_shape = live(&context, "TNamingTool::FindShape")?.get()?;
    let mut explorer = Explorer::new(&context_shape, shape_type, ShapeEnum::Shape)?;
    while explorer.more() {
        let candidate = explorer.current().clone();
        if coincides(&candidate, shape_type, sub_shapes)? {
            return Ok(Some(candidate));
        }
        explorer.next()?;
    }
    Ok(None)
}

/// The same number of sub-shapes, each one the same as one of `sub_shapes`.
fn coincides(
    candidate: &Shape,
    shape_type: ShapeEnum,
    sub_shapes: &MapOfShape,
) -> Result<bool, Failure> {
    let own = sub_shapes_of(candidate, shape_type)?;
    if own.extent() != sub_shapes.extent() {
        return Ok(false);
    }
    let matching = own
        .iter()
        .filter(|shape| sub_shapes.iter().any(|other| shape.is_same(other)))
        .count();
    Ok(matching == sub_shapes.extent())
}
