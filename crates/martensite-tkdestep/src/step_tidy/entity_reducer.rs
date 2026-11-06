//! `StepTidyEntityReducer`: finds the duplicates of one entity class and replaces them where they
//! are shared.
use crate::step_entity::step_live;
use martensite_tkernel::ncollection::{DataMap, HashPolicy};
use martensite_tkernel::standard::{Failure, Handle, RuntimeType, RuntimeTyped, StandardTransient};
use martensite_tkxsbase::xs_control::XSControlWorkSession;
use std::collections::HashSet;
use std::fmt;

/// `ReplacerFunction`: puts the new entity in the sharing entity; false when it did not hold the
/// old one.
pub type StepTidyReplacer<T> =
    fn(&Handle<T>, &Handle<T>, &Handle<StandardTransient>) -> Result<bool, Failure>;

/// The first entity of a class of duplicates, with the map's key type's `debug` satisfied.
pub struct DuplicateKey<T: RuntimeTyped>(pub(super) Handle<T>);

impl<T: RuntimeTyped> Clone for DuplicateKey<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: RuntimeTyped> fmt::Debug for DuplicateKey<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}@{:?}", T::TYPE_NAME, self.0.object_address())
    }
}

/// The hasher of the class, on the key.
#[derive(Clone, Copy, Debug, Default)]
pub struct KeyPolicy<H>(H);

impl<T: RuntimeTyped, H: HashPolicy<Handle<T>>> HashPolicy<DuplicateKey<T>> for KeyPolicy<H> {
    fn hash(&self, key: &DuplicateKey<T>) -> usize {
        self.0.hash(&key.0)
    }
    fn equal(&self, first: &DuplicateKey<T>, second: &DuplicateKey<T>) -> bool {
        self.0.equal(&first.0, &second.0)
    }
}

/// `StepTidyEntityReducer<ProcessedType, ProcessedTypeHasher>`.
pub struct StepTidyEntityReducer<T: RuntimeTyped, H: HashPolicy<Handle<T>> + Default> {
    my_ws: Handle<XSControlWorkSession>,
    my_replacers: Vec<(&'static RuntimeType, StepTidyReplacer<T>)>,
    my_duplicates: DataMap<DuplicateKey<T>, Vec<Handle<T>>, KeyPolicy<H>>,
}

impl<T: RuntimeTyped, H: HashPolicy<Handle<T>> + Default> StepTidyEntityReducer<T, H> {
    /// `StepTidyEntityReducer(theWS)`.
    pub fn new(ws: &Handle<XSControlWorkSession>) -> Self {
        Self {
            my_ws: ws.clone(),
            my_replacers: Vec::new(),
            my_duplicates: DataMap::default(),
        }
    }
    /// `registerReplacer(theType, theReplacer)`.
    pub fn register_replacer(&mut self, kind: &'static RuntimeType, replacer: StepTidyReplacer<T>) {
        self.my_replacers.push((kind, replacer));
    }
    fn replacer_of(&self, ent: &Handle<StandardTransient>) -> Option<StepTidyReplacer<T>> {
        let kind = ent.dynamic_type()?;
        self.my_replacers
            .iter()
            .find(|(known, _)| std::ptr::eq(*known, kind))
            .map(|(_, replacer)| *replacer)
    }
    /// `get_sharings(ent)` of the session's graph; `None` when the graph has no such entity.
    fn sharings_of(
        &self,
        ent: &Handle<T>,
    ) -> Result<Option<Vec<Handle<StandardTransient>>>, Failure> {
        let session = step_live(&self.my_ws, "StepTidyEntityReducer")?;
        let hgraph = session.graph()?;
        let holder = step_live(&hgraph, "StepTidyEntityReducer")?.graph();
        holder.get_sharings(&ent.cast())
    }
    /// `process_entity(entity)`: true when the entity is of the class. It is a duplicate
    /// candidate when every entity that shares it has a replacer.
    pub fn process_entity(&mut self, entity: &Handle<StandardTransient>) -> Result<bool, Failure> {
        let typed: Handle<T> = entity.cast();
        if typed.is_null() {
            return Ok(false);
        }
        let sharings = self.sharings_of(&typed)?;
        let all_known =
            sharings.is_some_and(|list| list.iter().all(|ent| self.replacer_of(ent).is_some()));
        if all_known {
            let key = DuplicateKey(typed.clone());
            match self.my_duplicates.change_seek(&key) {
                Some(list) => list.push(typed),
                None => {
                    self.my_duplicates.bind(key, Vec::new());
                }
            }
        }
        Ok(true)
    }
    /// `perform(replaced_entities)`: the addresses of the replaced entities are added.
    pub fn perform(
        &self,
        replaced: &mut Vec<Handle<StandardTransient>>,
        seen: &mut HashSet<usize>,
    ) -> Result<(), Failure> {
        for (key, duplicates) in self.my_duplicates.items() {
            for duplicate in duplicates {
                let Some(sharings) = self.sharings_of(duplicate)? else {
                    continue;
                };
                if self.replace_in_sharings(duplicate, &key.0, &sharings)?
                    && seen.insert(duplicate.object_address() as usize)
                {
                    replaced.push(duplicate.cast());
                }
            }
        }
        Ok(())
    }
    /// `replaceInSharings(theOldEntity, theNewEntity, theSharings)`.
    fn replace_in_sharings(
        &self,
        old: &Handle<T>,
        new: &Handle<T>,
        sharings: &[Handle<StandardTransient>],
    ) -> Result<bool, Failure> {
        let mut all_replaced = true;
        for sharing in sharings {
            if sharing.is_null() {
                continue;
            }
            let replacer = self.replacer_of(sharing).ok_or_else(|| {
                martensite_tkernel::standard::exceptions::no_such_object(
                    "StepTidyEntityReducer: no replacer for the type of a sharing entity",
                )
            })?;
            if !replacer(old, new, sharing)? {
                all_replaced = false;
            }
        }
        Ok(all_replaced)
    }
}
