//! The sorting of entities by the view or the drawing they are attached to.
use crate::iges_data::{IgesDataIgesEntity, IgesDataIgesModel, IgesDataViewKindEntity};
use martensite_tkernel::{
    impl_transient,
    ncollection::{HSequence, IndexedMap},
    standard::{Failure, Handle, RuntimeTyped, SharedState, StandardTransient},
};
use martensite_tkxsbase::{
    if_select::IFSelectPacketList,
    interface::{InterfaceGraph, InterfaceInterfaceModel, InterfaceModelEntities},
};

const TYPE_FOR_DRAWING: i32 = 404;

#[derive(Default)]
struct SorterFields {
    themodel: Handle<IgesDataIgesModel>,
    themap: IndexedMap<Handle<StandardTransient>>,
    theitems: IndexedMap<Handle<StandardTransient>>,
    thefinals: IndexedMap<Handle<StandardTransient>>,
    theinditem: Vec<i32>,
    theindfin: Vec<i32>,
}

/// The entities of a model, sorted by their view or drawing.
///
/// ```
/// use martensite_tkdeiges::{iges_data::IgesDataIgesEntity, iges_select::IgesSelectViewSorter};
/// use martensite_tkernel::standard::Handle;
/// let sorter = IgesSelectViewSorter::default();
/// let entity = Handle::new(IgesDataIgesEntity::default());
/// assert!(sorter.add_entity(&entity));
/// assert!(!sorter.add_entity(&entity));
/// assert_eq!(sorter.nb_entities(), 1);
/// ```
#[derive(Default)]
pub struct IgesSelectViewSorter {
    transient: StandardTransient,
    state: SharedState<SorterFields>,
}
impl_transient!(
    IgesSelectViewSorter,
    "IgesSelectViewSorter",
    StandardTransient,
    transient
);

impl IgesSelectViewSorter {
    /// Sets the model the sets are made from.
    pub fn set_model(&self, model: &Handle<IgesDataIgesModel>) {
        self.state.write().themodel = model.clone();
    }
    /// Forgets the entities added and the sets.
    pub fn clear(&self) {
        let model = self.state.read().themodel.clone();
        *self.state.write() = SorterFields {
            themodel: model,
            ..SorterFields::default()
        };
    }
    /// Adds an entity, a list or a model, as `ent` is one; false when it is none or nothing changed.
    pub fn add(&self, ent: &Handle<StandardTransient>) -> bool {
        let entity = ent.cast::<IgesDataIgesEntity>();
        if entity.is_some() {
            return self.add_entity(&entity);
        }
        let list = ent.cast::<HSequence<Handle<StandardTransient>>>();
        if list.is_some() {
            self.add_list(&list);
            return true;
        }
        let model = ent.cast::<dyn InterfaceInterfaceModel>();
        if model.is_some() {
            self.add_model(&model);
            return true;
        }
        false
    }
    /// Adds an entity under its view, itself for a view or a drawing; false when it is there.
    pub fn add_entity(&self, igesent: &Handle<IgesDataIgesEntity>) -> bool {
        let Some(live) = igesent.get() else {
            return false;
        };
        let key: Handle<StandardTransient> = igesent.cast();
        let is_own_set = live.type_number() == TYPE_FOR_DRAWING
            || igesent.is_kind(<dyn IgesDataViewKindEntity>::static_type());
        let view: Handle<StandardTransient> = if is_own_set {
            key.clone()
        } else {
            live.view().cast()
        };
        let mut state = self.state.write();
        if state.themap.find_index(&key) != 0 {
            return false;
        }
        state.themap.add(key);
        let viewindex = if view.is_null() {
            0
        } else {
            match state.theitems.find_index(&view) {
                0 => state.theitems.add(view) as i32,
                found => found as i32,
            }
        };
        state.theinditem.push(viewindex);
        state.theindfin.push(0);
        true
    }
    /// Adds the entities of a list.
    pub fn add_list(&self, list: &Handle<HSequence<Handle<StandardTransient>>>) {
        let items: Vec<Handle<StandardTransient>> = list.get().map_or_else(Vec::new, |live| {
            live.sequence(|items| items.iter().cloned().collect())
        });
        for item in &items {
            self.add(item);
        }
    }
    /// Adds the entities of a model of the norm.
    pub fn add_model(&self, model: &Handle<dyn InterfaceInterfaceModel>) {
        let igesmod = model.cast::<IgesDataIgesModel>();
        let Some(live) = igesmod.get() else {
            return;
        };
        for i in 1..=live.nb_entities() {
            self.add_entity(&live.entity(i));
        }
    }
    /// The count of entities added.
    pub fn nb_entities(&self) -> i32 {
        self.state.read().themap.extent() as i32
    }
    /// The sets are the single views, and the drawings when `alsoframes`; the rest goes to none.
    pub fn sort_single_views(&self, alsoframes: bool) {
        let mut state = self.state.write();
        let SorterFields {
            theitems,
            thefinals,
            theinditem,
            theindfin,
            ..
        } = &mut *state;
        *thefinals = IndexedMap::default();
        for (rank, numitem) in theinditem.iter().enumerate() {
            let mut finalindex = 0;
            if *numitem > 0
                && let Ok(item) = theitems.find_key(*numitem as usize)
            {
                let entity = item.cast::<IgesDataIgesEntity>();
                let is_frame = alsoframes
                    && entity
                        .get()
                        .is_some_and(|live| live.type_number() == TYPE_FOR_DRAWING);
                let is_single = item
                    .cast::<dyn IgesDataViewKindEntity>()
                    .get()
                    .is_some_and(IgesDataViewKindEntity::is_single);
                if is_frame || is_single {
                    finalindex = final_index(thefinals, item);
                }
            }
            theindfin[rank] = finalindex;
        }
    }
    /// The sets are the drawings that hold the views.
    ///
    /// # Errors
    /// A failure of the graph.
    pub fn sort_drawings(&self, g: &InterfaceGraph) -> Result<(), Failure> {
        let mut state = self.state.write();
        let SorterFields {
            theitems,
            thefinals,
            theinditem,
            theindfin,
            ..
        } = &mut *state;
        *thefinals = IndexedMap::default();
        for (rank, numitem) in theinditem.iter().enumerate() {
            if *numitem <= 0 {
                theindfin[rank] = 0;
                continue;
            }
            let Ok(item) = theitems.find_key(*numitem as usize) else {
                continue;
            };
            let drawing = drawing_of(item, g)?;
            theindfin[rank] = if drawing.is_null() {
                0
            } else {
                final_index(thefinals, &drawing)
            };
        }
        Ok(())
    }
    /// The count of sets: of the sort made when `final_sets`, else of the views.
    pub fn nb_sets(&self, final_sets: bool) -> i32 {
        let state = self.state.read();
        let sets = if final_sets {
            &state.thefinals
        } else {
            &state.theitems
        };
        sets.extent() as i32
    }
    /// The entity of a set, from rank 1.
    ///
    /// # Errors
    /// An out of range failure for a bad rank.
    pub fn set_item(
        &self,
        num: i32,
        final_sets: bool,
    ) -> Result<Handle<IgesDataIgesEntity>, Failure> {
        let state = self.state.read();
        let sets = if final_sets {
            &state.thefinals
        } else {
            &state.theitems
        };
        let key = sets.find_key(usize::try_from(num).unwrap_or(0))?;
        Ok(key.cast())
    }
    /// The sets as packets of entities.
    ///
    /// # Errors
    /// A failure of the packet list.
    pub fn sets(&self, final_sets: bool) -> Result<Handle<IFSelectPacketList>, Failure> {
        let state = self.state.read();
        let list = IFSelectPacketList::new(&state.themodel.cast())?;
        let (indices, sets) = if final_sets {
            (&state.theindfin, &state.thefinals)
        } else {
            (&state.theinditem, &state.theitems)
        };
        for num in 1..=sets.extent() as i32 {
            list.add_packet()?;
            for (rank, index) in indices.iter().enumerate() {
                if *index == num {
                    list.add(state.themap.find_key(rank + 1)?)?;
                }
            }
        }
        Ok(Handle::new(list))
    }
}

fn final_index(
    finals: &mut IndexedMap<Handle<StandardTransient>>,
    item: &Handle<StandardTransient>,
) -> i32 {
    match finals.find_index(item) {
        0 => finals.add(item.clone()) as i32,
        found => found as i32,
    }
}

/// The drawing of an item: itself, or the last one sharing it, null when none.
fn drawing_of(
    item: &Handle<StandardTransient>,
    g: &InterfaceGraph,
) -> Result<Handle<StandardTransient>, Failure> {
    let entity = item.cast::<IgesDataIgesEntity>();
    let Some(live) = entity.get() else {
        return Ok(Handle::default());
    };
    if live.type_number() == TYPE_FOR_DRAWING {
        return Ok(item.clone());
    }
    let mut drawing = Handle::default();
    let list = g.sharings(item)?;
    list.start();
    while list.more() {
        let sharing = list.value()?;
        let is_drawing = sharing
            .cast::<IgesDataIgesEntity>()
            .get()
            .is_some_and(|draw| draw.type_number() == TYPE_FOR_DRAWING);
        if is_drawing {
            drawing = sharing;
        }
        list.next();
    }
    Ok(drawing)
}
