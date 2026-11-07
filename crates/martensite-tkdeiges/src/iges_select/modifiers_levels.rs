//! The modifiers of the level of entities.
use super::IgesSelectModelModifier;
use super::modifier_bases::{add_fail, iges_model_modifier, modifier_base};
use crate::iges_data::{
    IgesDataDefList, IgesDataIgesEntity, IgesDataIgesModel, IgesDataLevelListEntity,
};
use martensite_tkernel::{
    standard::{Failure, Handle, SharedState, StandardTransient},
    tcollection::AsciiString,
};
use martensite_tkxsbase::{
    if_select::{IFSelectContextModif, IFSelectIntParam},
    interface::InterfaceCopyTool,
};

/// The value of a parameter, 0 for null.
fn value_of(param: &Handle<IFSelectIntParam>) -> i32 {
    param.get().map_or(0, IFSelectIntParam::value)
}

iges_model_modifier! {
    /// Replaces a level list by a level number: for the lists of a given level, or all, by a given
    /// number, or the first of the list.
    ///
    /// ```
    /// use martensite_tkdeiges::iges_select::IgesSelectChangeLevelList;
    /// use martensite_tkxsbase::if_select::IFSelectGeneralModifier;
    /// let label = IgesSelectChangeLevelList::new().label();
    /// assert!(label.as_bytes().starts_with(b"Changes all Level Lists"));
    /// ```
    IgesSelectChangeLevelList, "IgesSelectChangeLevelList",
    { theold: Handle<IFSelectIntParam>, thenew: Handle<IFSelectIntParam> },
    self => self.list_label()
}

impl Default for IgesSelectChangeLevelList {
    fn default() -> Self {
        Self::new()
    }
}

impl IgesSelectChangeLevelList {
    /// A modifier of all the lists, to the first of each.
    pub fn new() -> Self {
        Self {
            transient: StandardTransient::default(),
            base: modifier_base(true),
            theold: SharedState::new(Handle::default()),
            thenew: SharedState::new(Handle::default()),
        }
    }
    /// Whether the old number is set.
    pub fn has_old_number(&self) -> bool {
        self.theold.read().is_some()
    }
    /// The old number.
    pub fn old_number(&self) -> Handle<IFSelectIntParam> {
        self.theold.read().clone()
    }
    /// Sets the old number.
    pub fn set_old_number(&self, param: &Handle<IFSelectIntParam>) {
        *self.theold.write() = param.clone();
    }
    /// Whether the new number is set.
    pub fn has_new_number(&self) -> bool {
        self.thenew.read().is_some()
    }
    /// The new number.
    pub fn new_number(&self) -> Handle<IFSelectIntParam> {
        self.thenew.read().clone()
    }
    /// Sets the new number.
    pub fn set_new_number(&self, param: &Handle<IFSelectIntParam>) {
        *self.thenew.write() = param.clone();
    }
    fn list_label(&self) -> AsciiString {
        let oldl = value_of(&self.old_number());
        let head = if self.has_old_number() {
            format!("Changes Level Lists containing {oldl}")
        } else {
            format!("Changes all Level Lists in D.E. {oldl}")
        };
        // The new number is written when the old one is set: that decides, not the new one.
        let tail = if self.has_old_number() {
            format!(" to Number {}", value_of(&self.new_number()))
        } else {
            " to Number = first value in List".to_owned()
        };
        AsciiString::new(&format!("{head}{tail}"))
    }
}

impl IgesSelectModelModifier for IgesSelectChangeLevelList {
    fn performing(
        &self,
        ctx: &mut IFSelectContextModif,
        _target: &Handle<IgesDataIgesModel>,
        _tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let yaold = self.has_old_number();
        let oldl = value_of(&self.old_number());
        let mut newl = value_of(&self.new_number());
        if oldl < 0 {
            add_fail(ctx, "ChangeLevelList : OldNumber negative")?;
        }
        if newl < 0 {
            add_fail(ctx, "ChangeLevelList : NewNumber negative")?;
        }
        if oldl < 0 || newl < 0 {
            return Ok(());
        }
        ctx.start();
        while ctx.more() {
            let entity = ctx.value_result()?.cast::<IgesDataIgesEntity>();
            if let Some(ent) = entity.get()
                && ent.def_level() == IgesDataDefList::DefSeveral
                && (!yaold || ent.level() == oldl)
            {
                // The new number stays what the old one decided: it is taken from the list only
                // when no old number is set.
                if !yaold {
                    let Some(first) = first_level(ent)? else {
                        ctx.next();
                        continue;
                    };
                    newl = first;
                }
                ent.init_level(&Handle::default(), newl);
                ctx.trace("")?;
            }
            ctx.next();
        }
        Ok(())
    }
}

/// The first level of the list of an entity, 0 for an empty list or a negative number; `None` when
/// there is no list.
fn first_level(ent: &IgesDataIgesEntity) -> Result<Option<i32>, Failure> {
    let list = ent.level_list();
    let Some(live): Option<&dyn IgesDataLevelListEntity> = list.get() else {
        return Ok(None);
    };
    let level = if live.nb_level_numbers()? > 0 {
        live.level_number(1)?
    } else {
        0
    };
    Ok(Some(level.max(0)))
}

iges_model_modifier! {
    /// Changes the level numbers of entities that have one: a given one, or all.
    ///
    /// ```
    /// use martensite_tkdeiges::iges_select::IgesSelectChangeLevelNumber;
    /// use martensite_tkxsbase::if_select::IFSelectGeneralModifier;
    /// let label = IgesSelectChangeLevelNumber::new().label();
    /// assert_eq!(label.as_bytes(), b"Changes all Level Numbers positive and zero to 0");
    /// ```
    IgesSelectChangeLevelNumber, "IgesSelectChangeLevelNumber",
    { theold: Handle<IFSelectIntParam>, thenew: Handle<IFSelectIntParam> },
    self => self.number_label()
}

impl Default for IgesSelectChangeLevelNumber {
    fn default() -> Self {
        Self::new()
    }
}

impl IgesSelectChangeLevelNumber {
    /// A modifier of all the numbers, to 0.
    pub fn new() -> Self {
        Self {
            transient: StandardTransient::default(),
            base: modifier_base(false),
            theold: SharedState::new(Handle::default()),
            thenew: SharedState::new(Handle::default()),
        }
    }
    /// Whether the old number is set.
    pub fn has_old_number(&self) -> bool {
        self.theold.read().is_some()
    }
    /// The old number.
    pub fn old_number(&self) -> Handle<IFSelectIntParam> {
        self.theold.read().clone()
    }
    /// Sets the old number.
    pub fn set_old_number(&self, param: &Handle<IFSelectIntParam>) {
        *self.theold.write() = param.clone();
    }
    /// The new number.
    pub fn new_number(&self) -> Handle<IFSelectIntParam> {
        self.thenew.read().clone()
    }
    /// Sets the new number.
    pub fn set_new_number(&self, param: &Handle<IFSelectIntParam>) {
        *self.thenew.write() = param.clone();
    }
    fn number_label(&self) -> AsciiString {
        let (oldl, newl) = (value_of(&self.old_number()), value_of(&self.new_number()));
        AsciiString::new(&if self.has_old_number() {
            format!("Changes Level Number {oldl} to {newl}")
        } else {
            format!("Changes all Level Numbers positive and zero to {newl}")
        })
    }
}

impl IgesSelectModelModifier for IgesSelectChangeLevelNumber {
    fn performing(
        &self,
        ctx: &mut IFSelectContextModif,
        _target: &Handle<IgesDataIgesModel>,
        _tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let yaold = self.has_old_number();
        let (oldl, newl) = (value_of(&self.old_number()), value_of(&self.new_number()));
        if oldl < 0 {
            add_fail(ctx, "ChangeLevelNumber : OldNumber negative")?;
        }
        if newl < 0 {
            add_fail(ctx, "ChangeLevelNumber : NewNumber negative")?;
        }
        if oldl < 0 || newl < 0 {
            return Ok(());
        }
        ctx.start();
        while ctx.more() {
            let entity = ctx.value_result()?.cast::<IgesDataIgesEntity>();
            if let Some(ent) = entity.get()
                && ent.def_level() != IgesDataDefList::DefSeveral
                && (!yaold || ent.level() == oldl)
            {
                ent.init_level(&Handle::default(), newl);
                ctx.trace("")?;
            }
            ctx.next();
        }
        Ok(())
    }
}
