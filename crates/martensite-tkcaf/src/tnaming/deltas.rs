//! The undo records of a named shape: they keep the evolutions as shape arrays and replay
//! them through a `TNamingBuilder`.
use super::{TNamingBuilder, TNamingEvolution, TNamingIterator, TNamingNamedShape};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::{
    impl_transient,
    standard::{Failure, Handle, StandardTransient},
};
use martensite_tklcaf::live;
use martensite_tklcaf::tdf::{
    TdfAttribute, TdfAttributeDelta, TdfAttributeDeltaBase, TdfDeltaOnModification,
    TdfDeltaOnRemoval,
};

/// The saved evolutions: `myOld` and `myNew` (`None` is a null array).
#[derive(Clone, Default)]
struct SavedShapes {
    old: Option<Vec<Shape>>,
    new: Option<Vec<Shape>>,
}

impl SavedShapes {
    /// The constructor body: the old and new shapes of every evolution, by kind.
    fn of(named: &TNamingNamedShape) -> Result<Self, Failure> {
        let mut pairs: Vec<(Shape, Shape)> = Vec::new();
        let mut iterator = TNamingIterator::of(named);
        while iterator.more() {
            pairs.push((iterator.old_shape()?, iterator.new_shape()?));
            iterator.next()?;
        }
        if pairs.is_empty() {
            return Ok(Self::default());
        }
        let olds = || pairs.iter().map(|(old, _)| old.clone()).collect();
        let news = || pairs.iter().map(|(_, new)| new.clone()).collect();
        Ok(match named.evolution() {
            TNamingEvolution::Primitive => Self {
                old: None,
                new: Some(news()),
            },
            TNamingEvolution::Delete => Self {
                old: Some(olds()),
                new: None,
            },
            _ => Self {
                old: Some(olds()),
                new: Some(news()),
            },
        })
    }
}

/// The file-static `LoadNamedShape` of TNamingDeltaOnModification (`REPLACE` loads as
/// `GENERATED` for compatibility).
fn load_named_shape(
    builder: &TNamingBuilder,
    evolution: TNamingEvolution,
    old: &Shape,
    new: &Shape,
) -> Result<(), Failure> {
    match evolution {
        TNamingEvolution::Primitive => builder.generated(new),
        TNamingEvolution::Replace | TNamingEvolution::Generated => builder.generated_from(old, new),
        TNamingEvolution::Modify => builder.modify(old, new),
        TNamingEvolution::Delete => builder.delete(old),
        TNamingEvolution::Selected => builder.select(new, old),
    }
}

/// `TNamingDeltaOnModification`: restores a modified named shape by rebuilding its saved
/// evolutions.
pub struct TNamingDeltaOnModification {
    transient: StandardTransient,
    base: TdfAttributeDeltaBase,
    saved: SavedShapes,
}
impl_transient!(
    TNamingDeltaOnModification,
    "TNamingDeltaOnModification",
    dyn TdfDeltaOnModification,
    transient,
    interfaces: [dyn TdfAttributeDelta, dyn TdfDeltaOnModification]
);

impl TNamingDeltaOnModification {
    /// `TNamingDeltaOnModification(NS)`: saves the evolutions of `NS`
    /// (`StandardNullObject` for a null handle, a crash natively).
    pub fn new(named: &Handle<TNamingNamedShape>) -> Result<Handle<Self>, Failure> {
        let saved = SavedShapes::of(live(named, "TNamingDeltaOnModification")?)?;
        Ok(TdfAttributeDeltaBase::share(Self {
            transient: StandardTransient::default(),
            base: TdfAttributeDeltaBase::new(&named.cast()),
            saved,
        }))
    }
    /// The saved arrays replayed into a builder on the delta's label.
    fn replay(&self, named: &TNamingNamedShape) -> Result<(), Failure> {
        let (old, new) = (&self.saved.old, &self.saved.new);
        let length = match (old, new) {
            (None, None) => return Ok(()),
            (Some(shapes), _) | (None, Some(shapes)) => shapes.len(),
        };
        let builder = TNamingBuilder::new(&self.label())?;
        let null = Shape::default();
        for index in 0..length {
            let old_shape = old.as_ref().map_or(&null, |shapes| &shapes[index]);
            let new_shape = new.as_ref().map_or(&null, |shapes| &shapes[index]);
            load_named_shape(&builder, named.evolution(), old_shape, new_shape)?;
        }
        Ok(())
    }
}

impl TdfAttributeDelta for TNamingDeltaOnModification {
    fn attribute_delta_base(&self) -> &TdfAttributeDeltaBase {
        &self.base
    }
    /// `Apply`: puts the saved attribute back on the label when it has none of its ID (so
    /// the references to it stay valid), then rebuilds the saved evolutions.
    fn apply(&self) -> Result<(), Failure> {
        let attribute = self.attribute();
        let named_handle = attribute.cast::<TNamingNamedShape>();
        let named = live(&named_handle, "TNamingDeltaOnModification::Apply")?;
        let mut existing: Handle<dyn TdfAttribute> = Handle::default();
        if !self.label().find_attribute(&named.id(), &mut existing)? {
            self.label().add_attribute(&attribute, true)?;
        }
        self.replay(named)
    }
}
impl TdfDeltaOnModification for TNamingDeltaOnModification {}

/// `TNamingDeltaOnRemoval`: restores a removed named shape through a
/// `TNamingDeltaOnModification` made when it was removed.
pub struct TNamingDeltaOnRemoval {
    transient: StandardTransient,
    base: TdfAttributeDeltaBase,
    delta: Handle<TNamingDeltaOnModification>,
}
impl_transient!(
    TNamingDeltaOnRemoval,
    "TNamingDeltaOnRemoval",
    dyn TdfDeltaOnRemoval,
    transient,
    interfaces: [dyn TdfAttributeDelta, dyn TdfDeltaOnRemoval]
);

impl TNamingDeltaOnRemoval {
    /// `TNamingDeltaOnRemoval(NS)`.
    pub fn new(named: &Handle<TNamingNamedShape>) -> Result<Handle<Self>, Failure> {
        let delta = TNamingDeltaOnModification::new(named)?;
        Ok(TdfAttributeDeltaBase::share(Self {
            transient: StandardTransient::default(),
            base: TdfAttributeDeltaBase::new(&named.cast()),
            delta,
        }))
    }
}

impl TdfAttributeDelta for TNamingDeltaOnRemoval {
    fn attribute_delta_base(&self) -> &TdfAttributeDeltaBase {
        &self.base
    }
    /// `Apply`: the saved modification delta applies.
    fn apply(&self) -> Result<(), Failure> {
        live(&self.delta, "TNamingDeltaOnRemoval::Apply")?.apply()
    }
}
impl TdfDeltaOnRemoval for TNamingDeltaOnRemoval {}
