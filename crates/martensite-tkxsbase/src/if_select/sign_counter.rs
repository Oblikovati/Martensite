//! `IFSelectSignCounter`: counts the entities by the value of a signature;
//! `IFSelectGraphCounter`: by the size of the result of a deduction on each entity;
//! `IFSelectCheckCounter`: counts the messages of a check list.
use super::{
    IFSelectSelectDeduct, IFSelectSelection, IFSelectSignature, IFSelectSignatureListData,
    IFSelectSignatureListOps,
};
use crate::interface::{
    InterfaceCheckIterator, InterfaceGraph, InterfaceInterfaceModel, InterfaceModelEntities,
    InterfaceSignTypeBase,
};
use crate::moni_tool::MoniToolSignText;
use martensite_tkernel::{
    impl_transient, impl_transient_interface_target,
    ncollection::IndexedMap,
    standard::{
        Failure, Handle, HandleUpcast, RuntimeType, RuntimeTyped, SharedState, Transient,
        exceptions,
    },
    tcollection::{AsciiString, HAsciiString},
};
use std::ops::Deref;
use std::sync::Arc;

/// The fields of `IFSelectSignCounter`.
#[derive(Default)]
struct CounterState {
    themapstat: bool,
    themap: IndexedMap<Handle<dyn Transient>>,
    thematcher: Handle<dyn IFSelectSignature>,
    theselect: Handle<dyn IFSelectSelection>,
    theselmode: i32,
    thenbcomp1: i32,
    thenbcomp2: i32,
}

/// A counter that groups entities by the value of a signature.
///
/// ```
/// use martensite_tkxsbase::if_select::{IFSelectSignCounter, IFSelectSignCounterOps};
/// use martensite_tkernel::standard::Handle;
/// let counter = IFSelectSignCounter::new(true, false);
/// counter.add_sign(&Handle::default(), &Handle::default()).unwrap();
/// assert_eq!(counter.nb_nulls(), 1);
/// ```
pub struct IFSelectSignCounter {
    counter: IFSelectSignCounterData,
}
impl_transient!(IFSelectSignCounter, "IFSelectSignCounter", dyn IFSelectSignatureListOps, interfaces: [dyn IFSelectSignatureListOps, dyn IFSelectSignCounterOps]);

impl Deref for IFSelectSignCounter {
    type Target = IFSelectSignCounterData;

    fn deref(&self) -> &Self::Target {
        &self.counter
    }
}

impl IFSelectSignCounter {
    /// Construct a counter without a signature, e.g. `IFSelectSignCounter::new(true, false)`.
    pub fn new(withmap: bool, withlist: bool) -> Self {
        Self {
            counter: IFSelectSignCounterData::new(withmap, withlist),
        }
    }

    /// `IFSelectSignCounter(matcher, withmap = true, withlist = false)`: named after the
    /// signature; `StandardNullObject` for a null one (a native null dereference).
    pub fn with_signature(
        matcher: &Handle<dyn IFSelectSignature>,
        withmap: bool,
        withlist: bool,
    ) -> Result<Self, Failure> {
        let live = matcher.get().ok_or_else(|| {
            exceptions::null_object("IFSelectSignCounter: null signature; expected a signature")
        })?;
        let counter = Self::new(withmap, withlist);
        counter.state.write().thematcher = matcher.clone();
        counter.set_name(&live.name());
        Ok(counter)
    }
}

/// The accumulator shared by signature counters and graph counters.
pub struct IFSelectSignCounterData {
    list: IFSelectSignatureListData,
    state: SharedState<CounterState>,
}

impl Deref for IFSelectSignCounterData {
    type Target = IFSelectSignatureListData;

    fn deref(&self) -> &Self::Target {
        &self.list
    }
}

impl IFSelectSignCounterData {
    /// Construct the accumulator for a signature counter, e.g.
    /// `IFSelectSignCounterData::new(true, false)` counts each entity once.
    pub fn new(withmap: bool, withlist: bool) -> Self {
        let state = CounterState {
            themapstat: withmap,
            ..CounterState::default()
        };
        Self {
            list: IFSelectSignatureListData::new(withlist),
            state: SharedState::new(state),
        }
    }

    /// `Signature`.
    pub fn signature(&self) -> Handle<dyn IFSelectSignature> {
        self.state.read().thematcher.clone()
    }
    /// `SetMap(withmap)`.
    pub fn set_map(&self, withmap: bool) {
        self.state.write().themapstat = withmap;
    }
    /// `SetSelection(sel)`: the selection mode 2 (0 for null).
    pub fn set_selection(&self, sel: &Handle<dyn IFSelectSelection>) {
        self.state.write().theselect = sel.clone();
        self.set_sel_mode(-1);
        self.set_sel_mode(if sel.is_null() { 0 } else { 2 });
    }
    /// `Selection`.
    pub fn selection(&self) -> Handle<dyn IFSelectSelection> {
        self.state.read().theselect.clone()
    }
    /// `SetSelMode(selmode)`: a negative mode resets the computed sizes; 0 drops the
    /// selection.
    pub fn set_sel_mode(&self, selmode: i32) {
        let mut st = self.state.write();
        if selmode < 0 {
            st.thenbcomp1 = 0;
            st.thenbcomp2 = 0;
        } else {
            st.theselmode = selmode;
        }
        if selmode == 0 {
            st.theselect = Handle::default();
        }
    }
    /// `SelMode`.
    pub fn sel_mode(&self) -> i32 {
        self.state.read().theselmode
    }
    /// The map of `AddEntity`: false when `ent` is already counted (and records it).
    fn record(&self, ent: &Handle<dyn Transient>) -> bool {
        let mut st = self.state.write();
        if !st.themapstat || ent.is_null() {
            return true;
        }
        if st.themap.contains(ent) {
            return false;
        }
        st.themap.add(ent.clone());
        true
    }
}

impl IFSelectSignatureListOps for IFSelectSignCounter {
    fn signature_list(&self) -> &IFSelectSignatureListData {
        &self.counter.list
    }
}

/// The virtual members of `IFSelectSignCounter` and the members that call them.
pub trait IFSelectSignCounterOps: IFSelectSignatureListOps {
    /// Borrow the accumulator, e.g. `counter.sign_counter().set_map(false)`.
    fn sign_counter(&self) -> &IFSelectSignCounterData;
    /// `AddEntity(ent, model)`: counts `ent` once (with the map); false when already
    /// counted.
    fn add_entity(
        &self,
        ent: &Handle<dyn Transient>,
        model: &Handle<dyn InterfaceInterfaceModel>,
    ) -> Result<bool, Failure> {
        if !self.sign_counter().record(ent) {
            return Ok(false);
        }
        self.add_sign(ent, model)?;
        Ok(true)
    }
    /// `AddSign(ent, model)`: counts the value of the signature for `ent` (a null).
    fn add_sign(
        &self,
        ent: &Handle<dyn Transient>,
        model: &Handle<dyn InterfaceInterfaceModel>,
    ) -> Result<(), Failure> {
        let counter = self.sign_counter();
        let matcher = counter.signature();
        match matcher.get().filter(|_| ent.is_some()) {
            Some(live) => counter.add(ent, &live.value(ent, model)),
            None => counter.add(ent, ""),
        }
        Ok(())
    }
    /// `AddList(list, model)`: `AddEntity` of each entity.
    fn add_list(
        &self,
        list: Option<&[Handle<dyn Transient>]>,
        model: &Handle<dyn InterfaceInterfaceModel>,
    ) -> Result<(), Failure> {
        for ent in list.unwrap_or_default() {
            self.add_entity(ent, model)?;
        }
        Ok(())
    }
    /// `AddWithGraph(list, graph)`: `AddList` with the model of the graph.
    fn add_with_graph(
        &self,
        list: Option<&[Handle<dyn Transient>]>,
        graph: &InterfaceGraph,
    ) -> Result<(), Failure> {
        self.add_list(list, graph.model())
    }
    /// `AddModel(model)`: every entity of the model (without the map when it is empty).
    fn add_model(&self, model: &Handle<dyn InterfaceInterfaceModel>) -> Result<(), Failure> {
        let Some(live) = model.get() else {
            return Ok(());
        };
        let counter = self.sign_counter();
        let mapstat = counter.state.read().themapstat;
        if counter.state.read().themap.is_empty() {
            counter.state.write().themapstat = false;
        }
        let result = (1..=live.nb_entities())
            .try_for_each(|i| self.add_entity(&live.value(i)?, model).map(|_| ()));
        counter.state.write().themapstat = mapstat;
        result
    }
    /// `AddFromSelection(sel, G)`: the root result of `sel`.
    fn add_from_selection(
        &self,
        sel: &Handle<dyn IFSelectSelection>,
        g: &InterfaceGraph,
    ) -> Result<(), Failure> {
        let live = sel.get().ok_or_else(|| {
            exceptions::null_object("IFSelectSignCounter::AddFromSelection: null selection")
        })?;
        self.add_with_graph(Some(&live.root_result(g)?.content()), g)
    }
    /// `ComputeSelected(G, forced = false)`: counts the root result of the selection when
    /// the graph or the result size changed (or when forced); false without selection.
    fn compute_selected(&self, g: &InterfaceGraph, forced: bool) -> Result<bool, Failure> {
        let counter = self.sign_counter();
        let (mode, select) = (counter.sel_mode(), counter.selection());
        let Some(live) = select.get().filter(|_| mode >= 2) else {
            return Ok(false);
        };
        let iter = live.root_result(g)?;
        let (nb1, nb2) = (g.size(), iter.nb_entities());
        let afaire = {
            let mut st = counter.state.write();
            let changed = forced || nb1 != st.thenbcomp1 || nb2 != st.thenbcomp2;
            (st.thenbcomp1, st.thenbcomp2) = (nb1, nb2);
            changed
        };
        if afaire {
            self.add_with_graph(Some(&iter.content()), g)?;
        }
        Ok(true)
    }
    /// `Sign(ent, model)`: the value of the signature (null without signature or entity).
    fn sign(
        &self,
        ent: &Handle<dyn Transient>,
        model: &Handle<dyn InterfaceInterfaceModel>,
    ) -> Handle<HAsciiString> {
        let matcher = self.sign_counter().signature();
        match matcher.get().filter(|_| ent.is_some()) {
            Some(live) => Handle::new(HAsciiString::new(live.value(ent, model))),
            None => Handle::default(),
        }
    }
    /// `ComputedSign(ent, G)`: the value `AddWithGraph` gives for `ent` alone.
    fn computed_sign(
        &self,
        ent: &Handle<dyn Transient>,
        g: &InterfaceGraph,
    ) -> Result<AsciiString, Failure> {
        let counter = self.sign_counter();
        counter.set_mode_sign_only(true);
        let result = self.add_with_graph(Some(std::slice::from_ref(ent)), g);
        let val = counter.last_value();
        counter.set_mode_sign_only(false);
        result.map(|_| val)
    }
}
impl RuntimeTyped for dyn IFSelectSignCounterOps {
    const TYPE_NAME: &'static str = IFSelectSignCounter::TYPE_NAME;

    fn static_type() -> &'static RuntimeType {
        IFSelectSignCounter::static_type()
    }
}
impl_transient_interface_target!(dyn IFSelectSignCounterOps);

impl HandleUpcast<dyn IFSelectSignatureListOps> for dyn IFSelectSignCounterOps {
    fn upcast_arc(owner: Arc<Self>) -> Arc<dyn IFSelectSignatureListOps> {
        owner
    }
}

impl IFSelectSignCounterOps for IFSelectSignCounter {
    fn sign_counter(&self) -> &IFSelectSignCounterData {
        &self.counter
    }
}

impl Deref for dyn IFSelectSignCounterOps {
    type Target = IFSelectSignCounterData;

    fn deref(&self) -> &Self::Target {
        self.sign_counter()
    }
}

/// Counts by the size of the result of a deduction applied to each entity.
pub struct IFSelectGraphCounter {
    base: IFSelectSignCounterData,
    theapplied: SharedState<Handle<dyn IFSelectSelectDeduct>>,
}
impl_transient!(IFSelectGraphCounter, "IFSelectGraphCounter", dyn IFSelectSignCounterOps, interfaces: [dyn IFSelectSignatureListOps, dyn IFSelectSignCounterOps]);

impl Deref for IFSelectGraphCounter {
    type Target = IFSelectSignCounterData;
    fn deref(&self) -> &IFSelectSignCounterData {
        &self.base
    }
}

impl IFSelectGraphCounter {
    /// `IFSelectGraphCounter(withmap = true, withlist = false)`.
    pub fn new(withmap: bool, withlist: bool) -> Self {
        Self {
            base: IFSelectSignCounterData::new(withmap, withlist),
            theapplied: SharedState::new(Handle::default()),
        }
    }
    /// `Applied`.
    pub fn applied(&self) -> Handle<dyn IFSelectSelectDeduct> {
        self.theapplied.read().clone()
    }
    /// `SetApplied(sel)`.
    pub fn set_applied(&self, sel: &Handle<dyn IFSelectSelectDeduct>) {
        *self.theapplied.write() = sel.clone();
    }
}

impl IFSelectSignatureListOps for IFSelectGraphCounter {
    fn signature_list(&self) -> &IFSelectSignatureListData {
        &self.base.list
    }
}

impl IFSelectSignCounterOps for IFSelectGraphCounter {
    fn sign_counter(&self) -> &IFSelectSignCounterData {
        &self.base
    }
    /// `AddWithGraph(list, graph)`: without deduction, `AddList`; else each entity counted
    /// by the size of the unique result of the deduction from it.
    fn add_with_graph(
        &self,
        list: Option<&[Handle<dyn Transient>]>,
        graph: &InterfaceGraph,
    ) -> Result<(), Failure> {
        let applied = self.applied();
        let Some(live) = applied.get() else {
            return self.add_list(list, graph.model());
        };
        for ent in list.unwrap_or_default() {
            live.alternate().set_entity(ent);
            let n = live.unique_result(graph)?.nb_entities();
            self.base.add(ent, &n.to_string());
        }
        Ok(())
    }
}

/// `IFSelectCheckCounter`: counts the messages of a check list ("F:" for fails, "W:"
/// for warnings, with the type of the entity).
pub struct IFSelectCheckCounter {
    base: IFSelectSignatureListData,
    thesign: SharedState<Handle<dyn MoniToolSignText>>,
}
impl_transient!(IFSelectCheckCounter, "IFSelectCheckCounter", dyn IFSelectSignatureListOps, interfaces: [dyn IFSelectSignatureListOps]);

impl Deref for IFSelectCheckCounter {
    type Target = IFSelectSignatureListData;
    fn deref(&self) -> &IFSelectSignatureListData {
        &self.base
    }
}

impl IFSelectSignatureListOps for IFSelectCheckCounter {
    fn signature_list(&self) -> &IFSelectSignatureListData {
        &self.base
    }
}

impl IFSelectCheckCounter {
    /// `IFSelectCheckCounter(withlist = false)`: named "Check".
    pub fn new(withlist: bool) -> Self {
        let counter = Self {
            base: IFSelectSignatureListData::new(withlist),
            thesign: SharedState::new(Handle::default()),
        };
        counter.set_name("Check");
        counter
    }
    /// `SetSignature(sign)`: the signature giving the type text.
    pub fn set_signature(&self, sign: &Handle<dyn MoniToolSignText>) {
        *self.thesign.write() = sign.clone();
    }
    /// `Signature`.
    pub fn signature(&self) -> Handle<dyn MoniToolSignText> {
        self.thesign.read().clone()
    }
    /// The type text of an entity: the signature text, else the type name in the model,
    /// else the class name.
    fn type_text(
        &self,
        ent: &Handle<dyn Transient>,
        model: &Handle<dyn InterfaceInterfaceModel>,
    ) -> String {
        let sign = self.signature();
        if let Some(live) = sign.get() {
            return String::from_utf8_lossy(live.text(ent, &model.cast()).as_bytes()).into_owned();
        }
        if let Some(live) = model.get() {
            return live.type_name(ent, true);
        }
        ent.dynamic_type()
            .map(|k| InterfaceSignTypeBase::class_name(k.name()).to_owned())
            .unwrap_or_default()
    }
    /// `Analyse(list, model, original = false, failsonly = false)`: counts the fails (and
    /// the warnings unless `failsonly`) of each check, by type of entity.
    pub fn analyse(
        &self,
        list: &InterfaceCheckIterator,
        model: &Handle<dyn InterfaceInterfaceModel>,
        original: bool,
        failsonly: bool,
    ) -> Result<(), Failure> {
        let nbe = model.get().map_or(0, |m| m.nb_entities());
        self.set_name(&format!(
            "Check {}",
            String::from_utf8_lossy(list.name().as_bytes())
        ));
        for (num, check) in list.items() {
            let mut ent = check.entity();
            if ent.is_null() && num > 0 && num <= nbe {
                ent = model
                    .get()
                    .map(|m| m.value(num))
                    .transpose()?
                    .unwrap_or_default();
            }
            let tystr = if ent.is_some() {
                Some(self.type_text(&ent, model))
            } else {
                None
            };
            let prefix = |kind: &str| match &tystr {
                Some(t) => format!("{kind}:{t}: "),
                None => format!("{kind}: "),
            };
            for i in 1..=check.nb_fails() {
                self.add(
                    &ent,
                    &format!(
                        "{}{}",
                        prefix("F"),
                        String::from_utf8_lossy(check.c_fail(i, original)?.as_bytes())
                    ),
                );
            }
            let nbw = if failsonly { 0 } else { check.nb_warnings() };
            for i in 1..=nbw {
                self.add(
                    &ent,
                    &format!(
                        "{}{}",
                        prefix("W"),
                        String::from_utf8_lossy(check.c_warning(i, original)?.as_bytes())
                    ),
                );
            }
        }
        Ok(())
    }
}
