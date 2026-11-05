//! `IFSelectSelectCombine`: a selection combining a list of inputs (union,
//! intersection); `IFSelectSelectControl`: a selection with a main and a second input
//! (difference).
use super::{IFSelectSelection, IFSelectSelectionIterator};
use crate::if_graph::{IFGraphCompare, IFGraphCumulate};
use crate::interface::{InterfaceEntityIterator, InterfaceGraph, InterfaceGraphContentEvaluate};
use martensite_tkernel::{
    impl_transient, impl_transient_interface,
    standard::{Failure, Handle, SharedState, StandardTransient, exceptions},
    tcollection::AsciiString,
};

/// The fields of `IFSelectSelectCombine`: the inputs.
#[derive(Default)]
pub struct IFSelectSelectCombineBase {
    thelist: SharedState<Vec<Handle<dyn IFSelectSelection>>>,
}

/// `IFSelectSelectCombine`, abstract: the inputs; its `FillIterator` is the list and its
/// result is unique.
pub trait IFSelectSelectCombine: IFSelectSelection {
    /// The embedded fields.
    fn combine_base(&self) -> &IFSelectSelectCombineBase;
    /// `NbInputs`.
    fn nb_inputs(&self) -> i32 {
        self.combine_base().thelist.read().len() as i32
    }
    /// `Input(num)`: `StandardOutOfRange` out of 1..NbInputs.
    fn input_at(&self, num: i32) -> Result<Handle<dyn IFSelectSelection>, Failure> {
        let list = self.combine_base().thelist.read();
        usize::try_from(num - 1)
            .ok()
            .and_then(|i| list.get(i))
            .cloned()
            .ok_or_else(|| {
                exceptions::out_of_range(&format!(
                    "IFSelectSelectCombine::Input: {num}; expected [1, {}]",
                    list.len()
                ))
            })
    }
    /// `InputRank(sel)`: the rank of `sel` (0 when absent or null).
    fn input_rank(&self, sel: &Handle<dyn IFSelectSelection>) -> i32 {
        if sel.is_null() {
            return 0;
        }
        self.combine_base()
            .thelist
            .read()
            .iter()
            .position(|s| s == sel)
            .map_or(0, |p| p as i32 + 1)
    }
    /// `Add(sel, atnum = 0)`: inserts before `atnum`, appends out of 1..NbInputs.
    fn add(&self, sel: &Handle<dyn IFSelectSelection>, atnum: i32) {
        let mut list = self.combine_base().thelist.write();
        match usize::try_from(atnum - 1).ok().filter(|i| *i < list.len()) {
            Some(index) => list.insert(index, sel.clone()),
            None => list.push(sel.clone()),
        }
    }
    /// `Remove(sel)`.
    fn remove(&self, sel: &Handle<dyn IFSelectSelection>) -> bool {
        self.remove_at(self.input_rank(sel))
    }
    /// `Remove(num)`: false out of 1..NbInputs.
    fn remove_at(&self, num: i32) -> bool {
        let mut list = self.combine_base().thelist.write();
        let Some(index) = usize::try_from(num - 1).ok().filter(|i| *i < list.len()) else {
            return false;
        };
        list.remove(index);
        true
    }
    /// The inputs (a copy).
    fn inputs(&self) -> Vec<Handle<dyn IFSelectSelection>> {
        self.combine_base().thelist.read().clone()
    }
}
impl_transient_interface!(
    dyn IFSelectSelectCombine,
    "IFSelectSelectCombine",
    dyn IFSelectSelection
);

/// `IFSelectSelectCombine::FillIterator`: the inputs (null ones included).
pub fn combine_fill_iterator<S: IFSelectSelectCombine + ?Sized>(
    sel: &S,
    iter: &mut IFSelectSelectionIterator,
) {
    iter.add_list(&sel.inputs());
}

/// The fields of `IFSelectSelectControl`: the main and the second input.
#[derive(Default)]
pub struct IFSelectSelectControlBase {
    themain: SharedState<Handle<dyn IFSelectSelection>>,
    thesecond: SharedState<Handle<dyn IFSelectSelection>>,
}

/// `IFSelectSelectControl`, abstract: a main and a second input.
pub trait IFSelectSelectControl: IFSelectSelection {
    /// The embedded fields.
    fn control_base(&self) -> &IFSelectSelectControlBase;
    /// `MainInput`.
    fn main_input(&self) -> Handle<dyn IFSelectSelection> {
        self.control_base().themain.read().clone()
    }
    /// `HasSecondInput`.
    fn has_second_input(&self) -> bool {
        self.second_input().is_some()
    }
    /// `SecondInput`.
    fn second_input(&self) -> Handle<dyn IFSelectSelection> {
        self.control_base().thesecond.read().clone()
    }
    /// `SetMainInput(sel)`.
    fn set_main_input(&self, sel: &Handle<dyn IFSelectSelection>) {
        *self.control_base().themain.write() = sel.clone();
    }
    /// `SetSecondInput(sel)`.
    fn set_second_input(&self, sel: &Handle<dyn IFSelectSelection>) {
        *self.control_base().thesecond.write() = sel.clone();
    }
}
impl_transient_interface!(
    dyn IFSelectSelectControl,
    "IFSelectSelectControl",
    dyn IFSelectSelection
);

/// `IFSelectSelectControl::FillIterator`: the main input, then the second one.
pub fn control_fill_iterator<S: IFSelectSelectControl + ?Sized>(
    sel: &S,
    iter: &mut IFSelectSelectionIterator,
) {
    iter.add_item(&sel.main_input());
    let second = sel.second_input();
    if second.is_some() {
        iter.add_item(&second);
    }
}

/// The root result of an input; `StandardNullObject` for a null one (a native null
/// dereference).
fn root_of(
    sel: &Handle<dyn IFSelectSelection>,
    g: &InterfaceGraph,
) -> Result<InterfaceEntityIterator, Failure> {
    let live = sel.get().ok_or_else(|| {
        exceptions::null_object("IFSelect: null input selection; expected a selection")
    })?;
    live.root_result(g)
}

/// `IFSelectSelectUnion`: the entities of any input.
///
/// ```
/// use martensite_tkxsbase::if_select::{IFSelectSelectUnion, IFSelectSelection};
/// assert_eq!(IFSelectSelectUnion::new().label().as_bytes(), b"Union (OR)");
/// ```
#[derive(Default)]
pub struct IFSelectSelectUnion {
    transient: StandardTransient,
    base: IFSelectSelectCombineBase,
}
impl_transient!(IFSelectSelectUnion, "IFSelectSelectUnion", dyn IFSelectSelectCombine, transient, interfaces: [dyn IFSelectSelection, dyn IFSelectSelectCombine]);

impl IFSelectSelectUnion {
    /// `IFSelectSelectUnion()`: no input.
    pub fn new() -> Self {
        Self::default()
    }
}

impl IFSelectSelectCombine for IFSelectSelectUnion {
    fn combine_base(&self) -> &IFSelectSelectCombineBase {
        &self.base
    }
}

impl IFSelectSelection for IFSelectSelectUnion {
    /// `RootResult(G)`: the entities of the root results of the inputs, cumulated
    /// (`IFGraphCumulate`).
    fn root_result(&self, g: &InterfaceGraph) -> Result<InterfaceEntityIterator, Failure> {
        let mut cumulate = IFGraphCumulate::new(g)?;
        for sel in self.inputs() {
            cumulate.get_from_iter(&root_of(&sel, g)?)?;
        }
        cumulate.result()
    }
    fn fill_iterator(&self, iter: &mut IFSelectSelectionIterator) {
        combine_fill_iterator(self, iter)
    }
    /// `Label`: "Union (OR)".
    fn label(&self) -> AsciiString {
        AsciiString::new("Union (OR)")
    }
    fn has_unique_result(&self) -> bool {
        true
    }
}

/// `IFSelectSelectIntersection`: the entities of every input.
#[derive(Default)]
pub struct IFSelectSelectIntersection {
    transient: StandardTransient,
    base: IFSelectSelectCombineBase,
}
impl_transient!(IFSelectSelectIntersection, "IFSelectSelectIntersection", dyn IFSelectSelectCombine, transient, interfaces: [dyn IFSelectSelection, dyn IFSelectSelectCombine]);

impl IFSelectSelectIntersection {
    /// `IFSelectSelectIntersection()`: no input.
    pub fn new() -> Self {
        Self::default()
    }
}

impl IFSelectSelectCombine for IFSelectSelectIntersection {
    fn combine_base(&self) -> &IFSelectSelectCombineBase {
        &self.base
    }
}

impl IFSelectSelection for IFSelectSelectIntersection {
    /// `RootResult(G)`: the entities common to the root results of the inputs
    /// (`IFGraphCompare`). Native quirk kept: the comparison is reset after the second
    /// input of three or more, and a reset graph keeps no status, so the result is then
    /// empty.
    fn root_result(&self, g: &InterfaceGraph) -> Result<InterfaceEntityIterator, Failure> {
        let inputs = self.inputs();
        let nb = inputs.len();
        let mut compare = IFGraphCompare::new(g)?;
        for (i, sel) in inputs.iter().enumerate() {
            compare.get_from_iter(&root_of(sel, g)?, i == 0)?;
            if i > 0 && i + 1 < nb {
                let common = compare.common()?;
                compare.reset_data()?;
                compare.get_from_iter(&common, true)?;
            }
        }
        compare.common()
    }
    fn fill_iterator(&self, iter: &mut IFSelectSelectionIterator) {
        combine_fill_iterator(self, iter)
    }
    /// `Label`: "Intersection (AND)".
    fn label(&self) -> AsciiString {
        AsciiString::new("Intersection (AND)")
    }
    fn has_unique_result(&self) -> bool {
        true
    }
}

/// `IFSelectSelectDiff`: the entities of the main input not in the second one.
#[derive(Default)]
pub struct IFSelectSelectDiff {
    transient: StandardTransient,
    base: IFSelectSelectControlBase,
}
impl_transient!(IFSelectSelectDiff, "IFSelectSelectDiff", dyn IFSelectSelectControl, transient, interfaces: [dyn IFSelectSelection, dyn IFSelectSelectControl]);

impl IFSelectSelectDiff {
    /// `IFSelectSelectDiff()`: no input.
    pub fn new() -> Self {
        Self::default()
    }
}

impl IFSelectSelectControl for IFSelectSelectDiff {
    fn control_base(&self) -> &IFSelectSelectControlBase {
        &self.base
    }
}

impl IFSelectSelection for IFSelectSelectDiff {
    /// `RootResult(G)`: the entities of the main input not in the second one
    /// (`IFGraphCompare`).
    fn root_result(&self, g: &InterfaceGraph) -> Result<InterfaceEntityIterator, Failure> {
        let mut compare = IFGraphCompare::new(g)?;
        compare.get_from_iter(&root_of(&self.main_input(), g)?, true)?;
        let second = self.second_input();
        if second.is_some() {
            compare.get_from_iter(&root_of(&second, g)?, false)?;
        }
        compare.first_only()
    }
    fn fill_iterator(&self, iter: &mut IFSelectSelectionIterator) {
        control_fill_iterator(self, iter)
    }
    /// `Label`: "Differences".
    fn label(&self) -> AsciiString {
        AsciiString::new("Differences")
    }
    fn has_unique_result(&self) -> bool {
        true
    }
}
