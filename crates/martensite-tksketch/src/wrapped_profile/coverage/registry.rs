//! Native indices are local lookup references; semantic keys remain authored labels.
use super::*;
use martensite_tkbrep::toptools::IndexedMapOfShape;
pub(super) struct Registry<'a> {
    by_key: BTreeMap<BindingKey, &'a Shape>,
    shapes: IndexedMapOfShape,
    owners: Vec<BindingKey>,
}
impl<'a> Registry<'a> {
    pub fn build(request: &Request, bindings: &'a [(BindingKey, Shape)]) -> Checked<Self> {
        let owner = expected::region(request, HistoryRole::Solid);
        let mut result = Self {
            by_key: BTreeMap::new(),
            shapes: IndexedMapOfShape::default(),
            owners: Vec::new(),
        };
        for (key, shape) in bindings {
            result.insert(*key, shape)?;
        }
        result.validate(request, owner)?;
        Ok(result)
    }
    fn validate(&self, request: &Request, owner: BindingKey) -> Checked<()> {
        let definitions = expected::definitions(request)?;
        count(
            owner,
            "semantic bindings",
            self.by_key.len(),
            definitions.len(),
        )?;
        for (key, kind) in definitions {
            kind_check(key, self.shape(key)?, kind)?;
        }
        Ok(())
    }
    fn insert(&mut self, key: BindingKey, shape: &'a Shape) -> Checked<()> {
        canonical(key, shape)?;
        if self.by_key.insert(key, shape).is_some() {
            return Err(fail(
                key,
                CoverageCause::DuplicateBinding {
                    key,
                    role: "decoded",
                },
            ));
        }
        let prior = self.shapes.find_index(shape);
        if prior != 0 {
            return Err(self.alias(key, prior));
        }
        self.shapes.add(shape.clone());
        self.owners.push(key);
        Ok(())
    }
    fn alias(&self, key: BindingKey, index: usize) -> CoverageFailure {
        let first = self.owners.get(index - 1).copied().unwrap_or(key);
        fail(key, CoverageCause::AliasedBindings { first, second: key })
    }
    pub fn shape(&self, key: BindingKey) -> Checked<&'a Shape> {
        self.by_key
            .get(&key)
            .copied()
            .ok_or_else(|| fail(key, CoverageCause::MissingBinding(key)))
    }
    pub fn resolve(&self, owner: BindingKey, shape: &Shape) -> Checked<BindingKey> {
        let index = self.shapes.find_index(shape);
        self.owners
            .get(index.wrapping_sub(1))
            .copied()
            .ok_or_else(|| {
                let actual = shape.shape_type().unwrap_or(ShapeEnum::Shape);
                fail(owner, CoverageCause::UnboundUse { actual })
            })
    }
    pub fn count_kind(&self, kind: ShapeEnum) -> Checked<usize> {
        let mut count = 0;
        for (key, shape) in &self.by_key {
            if native(*key, shape.shape_type())? == kind {
                count += 1;
            }
        }
        Ok(count)
    }
    pub fn len(&self) -> usize {
        self.by_key.len()
    }
}
pub(super) fn canonical(owner: BindingKey, shape: &Shape) -> Checked<()> {
    if shape.orientation() != Orientation::Forward || !shape.location().is_identity() {
        return Err(fail(
            owner,
            CoverageCause::Placement {
                orientation: shape.orientation(),
                identity_location: shape.location().is_identity(),
            },
        ));
    }
    Ok(())
}
pub(super) fn kind_check(owner: BindingKey, shape: &Shape, expected: ShapeEnum) -> Checked<()> {
    let actual = native(owner, shape.shape_type())?;
    if actual != expected {
        return Err(fail(owner, CoverageCause::Kind { actual, expected }));
    }
    Ok(())
}
