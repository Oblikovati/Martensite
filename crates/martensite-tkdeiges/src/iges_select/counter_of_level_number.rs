//! The count of entities per level.
use crate::iges_data::IgesDataIgesEntity;
use martensite_tkernel::{
    impl_transient,
    ncollection::HSequence,
    standard::{Failure, Handle, SharedState, StandardTransient},
    tcollection::HAsciiString,
};
use martensite_tkxsbase::{
    if_select::{
        IFSelectSignCounter, IFSelectSignCounterOps, IFSelectSignatureList,
        IFSelectSignatureListOps,
    },
    interface::InterfaceInterfaceModel,
};
use std::io::{self, Write};

#[derive(Default)]
struct LevelCounts {
    thehigh: i32,
    thenblists: i32,
    thelevels: Vec<i32>,
}

/// A counter of the entities by level, an entity with a list counting for each of its levels.
///
/// ```
/// use martensite_tkdeiges::{iges_data::IgesDataIgesEntity, iges_select::IgesSelectCounterOfLevelNumber};
/// use martensite_tkernel::standard::Handle;
/// let counter = IgesSelectCounterOfLevelNumber::new(true, false);
/// let entity = Handle::new(IgesDataIgesEntity::default());
/// counter.add_level(&entity.cast(), 7);
/// assert_eq!((counter.highest_level(), counter.nb_times_level(7)), (7, 1));
/// ```
pub struct IgesSelectCounterOfLevelNumber {
    transient: StandardTransient,
    base: IFSelectSignCounter,
    counts: SharedState<LevelCounts>,
}
impl_transient!(
    IgesSelectCounterOfLevelNumber,
    "IgesSelectCounterOfLevelNumber",
    dyn IFSelectSignCounterOps,
    transient,
    interfaces: [dyn IFSelectSignatureListOps, dyn IFSelectSignCounterOps]
);

impl IgesSelectCounterOfLevelNumber {
    /// A counter; `withmap` counts an entity once, `withlist` keeps the entities.
    pub fn new(withmap: bool, withlist: bool) -> Self {
        let base = IFSelectSignCounter::new(withmap, withlist);
        base.set_name("IGES Level Number");
        Self {
            transient: StandardTransient::default(),
            base,
            counts: SharedState::new(LevelCounts::default()),
        }
    }
    /// Counts an entity for a level; a negative level stands for a list.
    pub fn add_level(&self, ent: &Handle<StandardTransient>, level: i32) {
        if level < 0 {
            self.counts.write().thenblists += 1;
            self.base.add(ent, "LEVEL LIST");
            return;
        }
        {
            let mut counts = self.counts.write();
            let slot = level as usize;
            if counts.thelevels.len() <= slot {
                counts.thelevels.resize(slot.max(100) + 1, 0);
            }
            counts.thelevels[slot] += 1;
            counts.thehigh = counts.thehigh.max(level);
        }
        self.base.add(ent, &format!("{level:7}"));
    }
    /// The highest level counted.
    pub fn highest_level(&self) -> i32 {
        self.counts.read().thehigh
    }
    /// The count of a level; of lists for a negative one.
    pub fn nb_times_level(&self, level: i32) -> i32 {
        let counts = self.counts.read();
        if level < 0 {
            return counts.thenblists;
        }
        if level > counts.thehigh {
            return 0;
        }
        counts.thelevels.get(level as usize).copied().unwrap_or(0)
    }
    /// The positive levels counted.
    pub fn levels(&self) -> Handle<HSequence<i32>> {
        let list = Handle::new(HSequence::default());
        let counts = self.counts.read();
        if let Some(live) = list.get() {
            for level in 1..=counts.thehigh {
                if counts
                    .thelevels
                    .get(level as usize)
                    .is_some_and(|count| *count > 0)
                {
                    live.append(level);
                }
            }
        }
        list
    }
}

impl IFSelectSignatureListOps for IgesSelectCounterOfLevelNumber {
    fn signature_list(&self) -> &IFSelectSignatureList {
        &self.base
    }
    fn clear(&self) {
        self.base.clear_base();
        *self.counts.write() = LevelCounts::default();
    }
    fn print_count(&self, s: &mut dyn Write) -> io::Result<()> {
        IFSelectSignatureListOps::print_count(&self.base, s)?;
        let counts = self.counts.read();
        writeln!(s, " Highest value : {}", counts.thehigh)?;
        if counts.thenblists > 0 {
            writeln!(
                s,
                "REMARK for LEVEL LIST : Entities are counted in <LEVEL LIST>\n, and in each Level value of their list"
            )?;
        }
        Ok(())
    }
}

impl IFSelectSignCounterOps for IgesSelectCounterOfLevelNumber {
    fn sign_counter(&self) -> &IFSelectSignCounter {
        &self.base
    }
    fn add_sign(
        &self,
        ent: &Handle<StandardTransient>,
        _model: &Handle<dyn InterfaceInterfaceModel>,
    ) -> Result<(), Failure> {
        let entity = ent.cast::<IgesDataIgesEntity>();
        let Some(live) = entity.get() else {
            return Ok(());
        };
        let levels = live.level_list();
        let Some(list) = levels.get() else {
            if live.level() >= 0 {
                self.add_level(ent, live.level());
            }
            return Ok(());
        };
        for rank in 1..=list.nb_level_numbers()? {
            self.add_level(ent, list.level_number(rank)?);
        }
        self.add_level(ent, -1);
        Ok(())
    }
    fn sign(
        &self,
        ent: &Handle<StandardTransient>,
        _model: &Handle<dyn InterfaceInterfaceModel>,
    ) -> Handle<HAsciiString> {
        let entity = ent.cast::<IgesDataIgesEntity>();
        let Some(live) = entity.get() else {
            return Handle::default();
        };
        let level = live.level();
        if live.level_list().is_null() && level < 0 {
            return Handle::default();
        }
        if level < 0 {
            return Handle::new(HAsciiString::new("LEVEL LIST"));
        }
        Handle::new(HAsciiString::new(format!("{level:7}")))
    }
}
