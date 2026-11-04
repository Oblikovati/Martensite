//! Each explode enumerates every immediate child with accumulated location/orientation.
use crate::primitive_shapes::{cone, translated};
use martensite_tkbrep::topods::{Shape, ShapeIterator};
use martensite_tkernel::standard::{Failure, exceptions::program_error};
use std::collections::BTreeMap;
type ConeBindings = BTreeMap<String, Shape>;
pub(super) fn cone_explode_bindings() -> Result<ConeBindings, Failure> {
    let mut registry = ConeBindings::new();
    for (name, first, last, x, z) in [
        ("c1", 50., 100., 100., 100.),
        ("c2", 100., 50., -100., 100.),
        ("c3", 0., 100., 100., -100.),
        ("c4", 100., 0., -100., -100.),
    ] {
        let shape = cone(first, last, 100.)?;
        registry.insert(name.to_owned(), translated(&shape, [x, 0., z])?);
        bind_children(&mut registry, name)?;
        bind_children(&mut registry, &format!("{name}_1"))?;
    }
    Ok(registry)
}
fn bind_children(registry: &mut ConeBindings, name: &str) -> Result<(), Failure> {
    let parent = registry
        .get(name)
        .ok_or_else(|| {
            program_error(&format!(
                "explode name={name:?}; expected existing parent binding"
            ))
        })?
        .clone();
    for (index, child) in ShapeIterator::new(&parent, true, true).enumerate() {
        registry.insert(format!("{name}_{}", index + 1), child);
    }
    Ok(())
}
