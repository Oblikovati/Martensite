//! The dispatches and the selections by view and drawing.
use super::IgesSelectViewSorter;
use crate::iges_data::{IgesDataIgesEntity, IgesDataIgesModel, iges_live};
use martensite_tkernel::{
    impl_transient,
    ncollection::HSequence,
    standard::{Failure, Handle, StandardTransient},
    tcollection::AsciiString,
};
use martensite_tkxsbase::{
    if_graph::IFGraphSubPartsIterator,
    if_select::{
        IFSelectDispatch, IFSelectDispatchBase, IFSelectSelectDeduct, IFSelectSelectDeductBase,
        IFSelectSelection, IFSelectSelectionIterator, deduct_fill_iterator,
    },
    interface::{InterfaceEntityIterator, InterfaceGraph},
};

/// The entities of an iterator as the list a sorter takes.
fn list_of(iter: &InterfaceEntityIterator) -> Handle<HSequence<Handle<StandardTransient>>> {
    let list = Handle::new(HSequence::default());
    if let Some(live) = list.get() {
        for ent in iter.content() {
            live.append(ent);
        }
    }
    list
}

/// How a sorter makes its sets.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ViewSort {
    Drawings,
    SingleViews,
}

/// Fills a sorter with the entities of `list`, sorted.
fn sorted(
    sorter: &IgesSelectViewSorter,
    list: &InterfaceEntityIterator,
    sort: ViewSort,
    g: &InterfaceGraph,
) -> Result<(), Failure> {
    sorter.set_model(&g.model().cast::<IgesDataIgesModel>());
    sorter.clear();
    sorter.add_list(&list_of(list));
    match sort {
        ViewSort::Drawings => sorter.sort_drawings(g),
        ViewSort::SingleViews => {
            sorter.sort_single_views(true);
            Ok(())
        }
    }
}

/// Declares a dispatch over a view sorter.
macro_rules! view_dispatch {
    ($(#[$meta:meta])* $class:ident, $recorded:literal, $sort:expr, $label:literal) => {
        $(#[$meta])*
        #[derive(Default)]
        pub struct $class {
            transient: StandardTransient,
            base: IFSelectDispatchBase,
            thesorter: IgesSelectViewSorter,
        }
        impl_transient!(
            $class,
            $recorded,
            dyn IFSelectDispatch,
            transient,
            interfaces: [dyn IFSelectDispatch]
        );
        impl IFSelectDispatch for $class {
            fn dispatch_base(&self) -> &IFSelectDispatchBase {
                &self.base
            }
            fn label(&self) -> AsciiString {
                AsciiString::new($label)
            }
            fn packets(
                &self,
                g: &InterfaceGraph,
                packs: &mut IFGraphSubPartsIterator,
            ) -> Result<(), Failure> {
                let selection = self.final_selection();
                let Some(live) = selection.get() else {
                    return Ok(());
                };
                sorted(&self.thesorter, &live.unique_result(g)?, $sort, g)?;
                let sets = self.thesorter.sets(true)?;
                let list = iges_live(&sets, "view dispatch")?;
                packs.set_load();
                for i in 1..=list.nb_packets() {
                    packs.add_part();
                    packs.get_from_iter(&list.entities(i)?)?;
                }
                Ok(())
            }
            fn can_have_remainder(&self) -> bool {
                true
            }
            fn remainder(&self, g: &InterfaceGraph) -> Result<InterfaceEntityIterator, Failure> {
                if self.thesorter.nb_entities() == 0 {
                    let selection = self.final_selection();
                    let Some(live) = selection.get() else {
                        return Ok(InterfaceEntityIterator::new());
                    };
                    sorted(&self.thesorter, &live.unique_result(g)?, $sort, g)?;
                }
                let sets = self.thesorter.sets(true)?;
                iges_live(&sets, "view dispatch")?.duplicated(0, false)
            }
        }
    };
}

view_dispatch! {
    /// One file per drawing.
    ///
    /// ```
    /// use martensite_tkdeiges::iges_select::IgesSelectDispPerDrawing;
    /// use martensite_tkxsbase::if_select::IFSelectDispatch;
    /// assert_eq!(IgesSelectDispPerDrawing::default().label().as_bytes(), b"One File per Drawing");
    /// ```
    IgesSelectDispPerDrawing, "IgesSelectDispPerDrawing", ViewSort::Drawings, "One File per Drawing"
}

view_dispatch! {
    /// One file per single view or drawing frame.
    ///
    /// ```
    /// use martensite_tkdeiges::iges_select::IgesSelectDispPerSingleView;
    /// use martensite_tkxsbase::if_select::IFSelectDispatch;
    /// assert!(IgesSelectDispPerSingleView::default().can_have_remainder());
    /// ```
    IgesSelectDispPerSingleView, "IgesSelectDispPerSingleView", ViewSort::SingleViews,
    "One File per single View or Drawing Frame"
}

/// Declares a selection deducted from its input, with its root result by `$root`.
macro_rules! view_selection {
    ($(#[$meta:meta])* $class:ident, $recorded:literal, $label:literal, $unique:literal, $self_:ident, $g:ident => $root:expr) => {
        $(#[$meta])*
        #[derive(Default)]
        pub struct $class {
            transient: StandardTransient,
            base: IFSelectSelectDeductBase,
        }
        impl_transient!(
            $class,
            $recorded,
            dyn IFSelectSelectDeduct,
            transient,
            interfaces: [dyn IFSelectSelection, dyn IFSelectSelectDeduct]
        );
        impl IFSelectSelectDeduct for $class {
            fn deduct_base(&self) -> &IFSelectSelectDeductBase {
                &self.base
            }
        }
        impl IFSelectSelection for $class {
            fn root_result(&$self_, $g: &InterfaceGraph) -> Result<InterfaceEntityIterator, Failure> {
                $root
            }
            fn fill_iterator(&self, iter: &mut IFSelectSelectionIterator) {
                deduct_fill_iterator(self, iter)
            }
            fn label(&self) -> AsciiString {
                AsciiString::new($label)
            }
            fn has_unique_result(&self) -> bool {
                $unique
            }
        }
    };
}

/// The sets of a sort of the input of a selection.
fn sets_of_input<S: IFSelectSelectDeduct + ?Sized>(
    selection: &S,
    sort: ViewSort,
    g: &InterfaceGraph,
) -> Result<InterfaceEntityIterator, Failure> {
    let sorter = IgesSelectViewSorter::default();
    sorted(&sorter, &selection.input_result(g)?, sort, g)?;
    let mut list = InterfaceEntityIterator::new();
    for i in 1..=sorter.nb_sets(true) {
        list.get_one_item(&sorter.set_item(i, true)?.cast());
    }
    Ok(list)
}

/// Adds the entities of the graph attached to a view of the graph.
fn add_attached_to_views(
    g: &InterfaceGraph,
    list: &mut InterfaceEntityIterator,
) -> Result<(), Failure> {
    let nb = g.size();
    for i in 1..=nb {
        let ent = g.entity(i)?;
        let entity = ent.cast::<IgesDataIgesEntity>();
        if let Some(live) = entity.get() {
            let nv = g.entity_number(&live.view().cast());
            if nv > 0 && nv <= nb {
                list.get_one_item(&ent);
            }
        }
    }
    Ok(())
}

view_selection! {
    /// The drawings the entities of the input are attached to.
    ///
    /// ```
    /// use martensite_tkdeiges::iges_select::IgesSelectSelectDrawingFrom;
    /// use martensite_tkxsbase::if_select::IFSelectSelection;
    /// assert!(IgesSelectSelectDrawingFrom::default().has_unique_result());
    /// ```
    IgesSelectSelectDrawingFrom, "IgesSelectSelectDrawingFrom", "Drawings attached", true,
    self, g => sets_of_input(self, ViewSort::Drawings, g)
}

view_selection! {
    /// The single views the entities of the input are attached to.
    ///
    /// ```
    /// use martensite_tkdeiges::iges_select::IgesSelectSelectSingleViewFrom;
    /// use martensite_tkxsbase::if_select::IFSelectSelection;
    /// assert_eq!(IgesSelectSelectSingleViewFrom::default().label().as_bytes(), b"Single Views attached");
    /// ```
    IgesSelectSelectSingleViewFrom, "IgesSelectSelectSingleViewFrom", "Single Views attached", true,
    self, g => sets_of_input(self, ViewSort::SingleViews, g)
}

view_selection! {
    /// The entities attached to the drawings of the input: their content, and what their views hold.
    ///
    /// ```
    /// use martensite_tkdeiges::iges_select::IgesSelectSelectFromDrawing;
    /// use martensite_tkxsbase::if_select::IFSelectSelection;
    /// assert!(!IgesSelectSelectFromDrawing::default().has_unique_result());
    /// ```
    IgesSelectSelectFromDrawing, "IgesSelectSelectFromDrawing", "Entities attached to a Drawing", false,
    self, g => {
        let mut list = InterfaceEntityIterator::new();
        let draws = self.input_result(g)?;
        if draws.nb_entities() == 0 {
            return Ok(list);
        }
        for ent in draws.content() {
            let is_drawing = ent
                .cast::<IgesDataIgesEntity>()
                .get()
                .is_some_and(|live| live.type_number() == 404);
            if is_drawing {
                list.get_one_item(&ent);
                list.add_list(&g.shareds(&ent)?.content());
            }
        }
        add_attached_to_views(g, &mut list)?;
        Ok(list)
    }
}

view_selection! {
    /// The entities attached to the single views of the input.
    ///
    /// ```
    /// use martensite_tkdeiges::iges_select::IgesSelectSelectFromSingleView;
    /// use martensite_tkxsbase::if_select::IFSelectSelection;
    /// assert_eq!(IgesSelectSelectFromSingleView::default().label().as_bytes(), b"Entities attached to a single View");
    /// ```
    IgesSelectSelectFromSingleView, "IgesSelectSelectFromSingleView", "Entities attached to a single View", false,
    self, g => {
        let mut list = InterfaceEntityIterator::new();
        if self.input_result(g)?.nb_entities() == 0 {
            return Ok(list);
        }
        add_attached_to_views(g, &mut list)?;
        Ok(list)
    }
}
