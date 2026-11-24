//! Native indices are local lookup references; semantic keys remain authored labels.
use super::*;
use martensite_tkbrep::toptools::IndexedMapOfShape;
pub(super) struct Registry<'a> {
    by_key: BTreeMap<BindingKey, &'a Shape>,
    shapes: IndexedMapOfShape,
    owners: Vec<BindingKey>,
}
impl<'a> Registry<'a> {
    pub fn build(
        request: &DevelopedRegionRequest,
        bindings: &'a [(BindingKey, Shape)],
    ) -> Checked<Self> {
        let owner = expected::region(request, HistoryRole::Solid);
        let result = Self::register(bindings)?;
        result.validate(request, owner)?;
        Ok(result)
    }
    fn register(bindings: &'a [(BindingKey, Shape)]) -> Checked<Self> {
        let mut result = Self {
            by_key: BTreeMap::new(),
            shapes: IndexedMapOfShape::default(),
            owners: Vec::new(),
        };
        for (key, shape) in bindings {
            result.insert(*key, shape)?;
        }
        Ok(result)
    }
    pub fn build_many<R: AsRef<DevelopedRegionRequest>>(
        requests: &[R],
        bindings: &'a [(BindingKey, Shape)],
    ) -> Checked<Self> {
        let result = Self::register(bindings)?;
        let definitions = family_definitions(requests)?;
        result.validate_definitions(CoverageOwner::Family, &definitions)?;
        Ok(result)
    }
    fn validate(&self, request: &DevelopedRegionRequest, owner: BindingKey) -> Checked<()> {
        let definitions = expected::definitions(request)?;
        self.validate_definitions(owner.into(), &definitions)
    }
    fn validate_definitions(
        &self,
        owner: CoverageOwner,
        definitions: &BTreeMap<BindingKey, ShapeEnum>,
    ) -> Checked<()> {
        count(
            owner,
            "semantic bindings",
            self.by_key.len(),
            definitions.len(),
        )?;
        for (key, kind) in definitions {
            kind_check(*key, self.shape(*key)?, *kind)?;
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
    pub fn resolve(
        &self,
        owner: impl Into<CoverageOwner> + Copy,
        shape: &Shape,
    ) -> Checked<BindingKey> {
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
fn family_definitions<R: AsRef<DevelopedRegionRequest>>(
    requests: &[R],
) -> Checked<BTreeMap<BindingKey, ShapeEnum>> {
    let mut result = BTreeMap::new();
    for request in requests {
        extend_definitions(&mut result, expected::definitions(request.as_ref())?)?;
    }
    Ok(result)
}

fn extend_definitions(
    target: &mut BTreeMap<BindingKey, ShapeEnum>,
    definitions: BTreeMap<BindingKey, ShapeEnum>,
) -> Checked<()> {
    for (key, kind) in definitions {
        if target.insert(key, kind).is_some() {
            return Err(duplicate_definition(key));
        }
    }
    Ok(())
}
fn duplicate_definition(key: BindingKey) -> CoverageFailure {
    fail(
        key,
        CoverageCause::DuplicateBinding {
            key,
            role: "declared family",
        },
    )
}

pub(super) fn canonical(owner: impl Into<CoverageOwner> + Copy, shape: &Shape) -> Checked<()> {
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
pub(super) fn kind_check(
    owner: impl Into<CoverageOwner> + Copy,
    shape: &Shape,
    expected: ShapeEnum,
) -> Checked<()> {
    let actual = native(owner, shape.shape_type())?;
    if actual != expected {
        return Err(fail(owner, CoverageCause::Kind { actual, expected }));
    }
    Ok(())
}
