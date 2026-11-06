//! `SelectMgrFilter` and the composition filters (`SelectMgrCompositionFilter`,
//! `SelectMgrAndFilter`, `SelectMgrOrFilter`, `SelectMgrAndOrFilter`).
use super::{SelectMgrEntityOwnerSource, SelectMgrFilterType, SelectMgrStoredFilters};
use martensite_tkernel::ncollection::Shared;
use martensite_tkernel::standard::{Failure, Handle, SharedState, StandardTransient, Transient};
use martensite_tkg3d::top_abs::ShapeEnum;
use std::collections::BTreeSet;

/// A filter of the detected owners.
pub trait SelectMgrFilter: Transient {
    /// `IsOk` (pure virtual): whether the owner may be detected; `filter.is_ok(&owner)?`.
    fn is_ok(&self, owner: &Handle<dyn SelectMgrEntityOwnerSource>) -> Result<bool, Failure>;
    /// `ActsOn`: false by default; `filter.acts_on(ShapeEnum::Face)`.
    fn acts_on(&self, standard_mode: ShapeEnum) -> bool {
        let _ = standard_mode;
        false
    }
}
martensite_tkernel::impl_transient_interface!(
    dyn SelectMgrFilter,
    "SelectMgrFilter",
    StandardTransient
);

/// The filters of a composition filter (`myFilters`).
#[derive(Default)]
pub struct SelectMgrCompositionFilterBase {
    /// `myFilters`.
    pub filters: SelectMgrStoredFilters,
}

/// A filter combining other filters.
pub trait SelectMgrCompositionFilter: SelectMgrFilter {
    /// The embedded filter list; `filter.composition_base()`.
    fn composition_base(&self) -> &SelectMgrCompositionFilterBase;
    /// `Add`; `composition.add(&filter)`.
    fn add(&self, filter: &Handle<dyn SelectMgrFilter>) {
        self.composition_base().filters.add(filter);
    }
    /// `Remove`: the first occurrence of the filter; `composition.remove(&filter)`.
    fn remove(&self, filter: &Handle<dyn SelectMgrFilter>) {
        self.composition_base().filters.remove(filter);
    }
    /// `IsEmpty`; `composition.is_empty()`.
    fn is_empty(&self) -> bool {
        self.composition_base().filters.is_empty()
    }
    /// `IsIn`; `composition.is_in(&filter)`.
    fn is_in(&self, filter: &Handle<dyn SelectMgrFilter>) -> bool {
        self.composition_base().filters.contains(filter)
    }
    /// Retain a live read-only list alias; `let filters = composition.stored_filters()`.
    fn stored_filters(&self) -> SelectMgrStoredFilters {
        self.composition_base().filters.clone()
    }
    /// `Clear`; `composition.clear()`.
    fn clear(&self) {
        self.composition_base().filters.clear();
    }
}

// Declare this real intermediate parent; callbacks alone do not register its handle view.
martensite_tkernel::impl_transient_interface!(
    dyn SelectMgrCompositionFilter,
    "SelectMgrCompositionFilter",
    dyn SelectMgrFilter
);

/// `SelectMgrCompositionFilter::ActsOn`: one stored filter acts on the mode.
/// Example: `select_mgr_composition_acts_on(&*filter, ShapeEnum::Edge)`.
pub fn select_mgr_composition_acts_on<F: SelectMgrCompositionFilter + ?Sized>(
    filter: &F,
    standard_mode: ShapeEnum,
) -> bool {
    filter.stored_filters().iter().any(|filter| {
        filter
            .get()
            .is_some_and(|value| value.acts_on(standard_mode))
    })
}

macro_rules! composition_filter {
    ($name:ident, $native:literal, $doc:literal) => {
        #[doc = $doc]
        #[derive(Default)]
        pub struct $name {
            transient: StandardTransient,
            base: SelectMgrCompositionFilterBase,
        }
        martensite_tkernel::impl_transient!($name, $native, dyn SelectMgrCompositionFilter, transient,
            interfaces: [dyn SelectMgrCompositionFilter, dyn SelectMgrFilter]);
        impl SelectMgrCompositionFilter for $name {
            fn composition_base(&self) -> &SelectMgrCompositionFilterBase {
                &self.base
            }
        }
    };
}

composition_filter!(
    SelectMgrAndFilter,
    "SelectMgrAndFilter",
    "`SelectMgrAndFilter`: every stored filter must accept; `SelectMgrAndFilter::default()`."
);
composition_filter!(
    SelectMgrOrFilter,
    "SelectMgrOrFilter",
    "`SelectMgrOrFilter`: one stored filter (or none stored) must accept; `SelectMgrOrFilter::default()`."
);

impl SelectMgrFilter for SelectMgrAndFilter {
    fn is_ok(&self, owner: &Handle<dyn SelectMgrEntityOwnerSource>) -> Result<bool, Failure> {
        selection_filters_accept(&self.stored_filters(), owner, || SelectMgrFilterType::And)
    }
    fn acts_on(&self, standard_mode: ShapeEnum) -> bool {
        select_mgr_composition_acts_on(self, standard_mode)
    }
}

impl SelectMgrFilter for SelectMgrOrFilter {
    fn is_ok(&self, owner: &Handle<dyn SelectMgrEntityOwnerSource>) -> Result<bool, Failure> {
        selection_filters_accept(&self.stored_filters(), owner, || SelectMgrFilterType::Or)
    }
    fn acts_on(&self, standard_mode: ShapeEnum) -> bool {
        select_mgr_composition_acts_on(self, standard_mode)
    }
}

/// The disabled objects of an `SelectMgrAndOrFilter`, by object address
/// (`NCollectionShared<NCollectionMap<const Standard_Transient*>>`).
pub type SelectMgrDisabledObjects = Shared<SharedState<BTreeSet<usize>>>;

/// `SelectMgrAndOrFilter`: the stored filters combined by AND or OR, refusing the disabled
/// objects. Example: `SelectMgrAndOrFilter::new(SelectMgrFilterType::Or)`.
pub struct SelectMgrAndOrFilter {
    transient: StandardTransient,
    base: SelectMgrCompositionFilterBase,
    disabled_objects: SharedState<Handle<SelectMgrDisabledObjects>>,
    filter_type: SharedState<SelectMgrFilterType>,
}
martensite_tkernel::impl_transient!(SelectMgrAndOrFilter, "SelectMgrAndOrFilter", dyn SelectMgrCompositionFilter, transient,
    interfaces: [dyn SelectMgrCompositionFilter, dyn SelectMgrFilter]);

impl SelectMgrAndOrFilter {
    /// A filter of a type without disabled objects; `SelectMgrAndOrFilter::new(SelectMgrFilterType::And)`.
    pub fn new(filter_type: SelectMgrFilterType) -> Self {
        Self {
            transient: StandardTransient::default(),
            base: SelectMgrCompositionFilterBase::default(),
            disabled_objects: SharedState::new(Handle::default()),
            filter_type: SharedState::new(filter_type),
        }
    }
    /// `SetDisabledObjects`; `filter.set_disabled_objects(&objects)`.
    pub fn set_disabled_objects(&self, objects: &Handle<SelectMgrDisabledObjects>) {
        *self.disabled_objects.write() = objects.clone();
    }
    /// `FilterType`; `filter.filter_type()`.
    pub fn filter_type(&self) -> SelectMgrFilterType {
        *self.filter_type.read()
    }
    /// `SetFilterType`; `filter.set_filter_type(SelectMgrFilterType::Or)`.
    pub fn set_filter_type(&self, filter_type: SelectMgrFilterType) {
        *self.filter_type.write() = filter_type;
    }
    /// Whether the selectable object of the owner is disabled.
    fn is_disabled(&self, owner: &Handle<dyn SelectMgrEntityOwnerSource>) -> bool {
        let address = owner
            .get()
            .map_or(std::ptr::null(), |o| o.selectable().object_address())
            as usize;
        self.disabled_objects
            .read()
            .get()
            .is_some_and(|set| set.read().contains(&address))
    }
}

impl SelectMgrCompositionFilter for SelectMgrAndOrFilter {
    fn composition_base(&self) -> &SelectMgrCompositionFilterBase {
        &self.base
    }
}

impl SelectMgrFilter for SelectMgrAndOrFilter {
    fn is_ok(&self, owner: &Handle<dyn SelectMgrEntityOwnerSource>) -> Result<bool, Failure> {
        if self.is_disabled(owner) {
            return Ok(false);
        }
        selection_filters_accept(&self.stored_filters(), owner, || self.filter_type())
    }
    fn acts_on(&self, standard_mode: ShapeEnum) -> bool {
        select_mgr_composition_acts_on(self, standard_mode)
    }
}

// Keep evaluation order and stop before later callbacks after a decision or failure.
fn selection_filters_accept(
    filters: &SelectMgrStoredFilters,
    owner: &Handle<dyn SelectMgrEntityOwnerSource>,
    kind: impl Fn() -> SelectMgrFilterType,
) -> Result<bool, Failure> {
    let mut cursor = filters.iter();
    while let Some(lease) = cursor.try_next()? {
        let Some(filter) = lease.get() else {
            continue;
        };
        let accepted = filter.is_ok(owner)?;
        // A callback may change AND/OR; source semantics read the mode after dispatch.
        let Some(decision) = selection_decision(accepted, kind()) else {
            continue;
        };
        return Ok(decision);
    }
    Ok(kind() != SelectMgrFilterType::Or || filters.is_empty())
}

fn selection_decision(accepted: bool, kind: SelectMgrFilterType) -> Option<bool> {
    match (accepted, kind) {
        (true, SelectMgrFilterType::Or) => Some(true),
        (false, SelectMgrFilterType::And) => Some(false),
        _ => None,
    }
}
