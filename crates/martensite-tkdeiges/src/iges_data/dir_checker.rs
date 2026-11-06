//! The criteria a directory entry must meet for a kind of entity: checked, and corrected.
use super::param_reader::iges_msg;
use super::{IgesDataDefType, IgesDataIgesEntity};
use martensite_tkernel::standard::{Handle, RuntimeTyped};
use martensite_tkxsbase::interface::InterfaceCheck;

const NOT_TESTED: i32 = -100;
const IGNORED: i32 = -10;

/// The criteria on a directory entry; each one is inhibited until it is set.
///
/// ```
/// use martensite_tkdeiges::iges_data::{IgesDataDefType, IgesDataDirChecker};
/// let mut checker = IgesDataDirChecker::with_forms(110, 0, 2);
/// checker.structure(IgesDataDefType::DefVoid);
/// assert!(checker.is_set());
/// ```
#[derive(Clone, Copy, Debug)]
pub struct IgesDataDirChecker {
    isitset: bool,
    thetype: i32,
    theform1: i32,
    theform2: i32,
    thestructure: IgesDataDefType,
    thelinefont: IgesDataDefType,
    thelineweig: IgesDataDefType,
    thecolor: IgesDataDefType,
    thegraphier: i32,
    theblankst: i32,
    thesubordst: i32,
    theuseflag: i32,
    thehierst: i32,
}

impl Default for IgesDataDirChecker {
    fn default() -> Self {
        Self::with_forms(0, 0, 0)
    }
}

impl IgesDataDirChecker {
    /// No criterion at all.
    pub fn new() -> Self {
        Self::default()
    }
    /// A required type, any form.
    pub fn with_type(atype: i32) -> Self {
        Self::with_forms(atype, 0, -1)
    }
    /// A required type and form.
    pub fn with_form(atype: i32, aform: i32) -> Self {
        Self::with_forms(atype, aform, aform)
    }
    /// A required type, and a range of forms.
    pub fn with_forms(atype: i32, aform1: i32, aform2: i32) -> Self {
        Self {
            isitset: false,
            thetype: atype,
            theform1: aform1,
            theform2: aform2,
            thestructure: IgesDataDefType::ErrorRef,
            thelinefont: IgesDataDefType::ErrorRef,
            thelineweig: IgesDataDefType::ErrorRef,
            thecolor: IgesDataDefType::ErrorRef,
            thegraphier: NOT_TESTED,
            theblankst: NOT_TESTED,
            thesubordst: NOT_TESTED,
            theuseflag: NOT_TESTED,
            thehierst: NOT_TESTED,
        }
    }
    /// Whether a criterion was set.
    pub fn is_set(&self) -> bool {
        self.isitset
    }
    /// Sets the default criterion: no structure.
    pub fn set_default(&mut self) {
        self.structure(IgesDataDefType::DefVoid);
    }
    /// Sets the criterion on the structure.
    pub fn structure(&mut self, crit: IgesDataDefType) {
        self.isitset = true;
        self.thestructure = crit;
    }
    /// Sets the criterion on the line font.
    pub fn line_font(&mut self, crit: IgesDataDefType) {
        self.isitset = true;
        self.thelinefont = crit;
    }
    /// Sets the criterion on the line weight.
    pub fn line_weight(&mut self, crit: IgesDataDefType) {
        self.isitset = true;
        self.thelineweig = crit;
    }
    /// Sets the criterion on the color.
    pub fn color(&mut self, crit: IgesDataDefType) {
        self.isitset = true;
        self.thecolor = crit;
    }
    /// The graphic fields are ignored: always when `hierarchy` is -1, else for that hierarchy.
    pub fn graphics_ignored(&mut self, hierarchy: i32) {
        self.isitset = true;
        self.thegraphier = hierarchy;
    }
    /// The blank status is ignored.
    pub fn blank_status_ignored(&mut self) {
        self.isitset = true;
        self.theblankst = IGNORED;
    }
    /// The blank status must be `val`.
    pub fn blank_status_required(&mut self, val: i32) {
        self.isitset = true;
        self.theblankst = val;
    }
    /// The subordinate status is ignored.
    pub fn subordinate_status_ignored(&mut self) {
        self.isitset = true;
        self.thesubordst = IGNORED;
    }
    /// The subordinate status must be `val`.
    pub fn subordinate_status_required(&mut self, val: i32) {
        self.isitset = true;
        self.thesubordst = val;
    }
    /// The use flag is ignored.
    pub fn use_flag_ignored(&mut self) {
        self.isitset = true;
        self.theuseflag = IGNORED;
    }
    /// The use flag must be `val`.
    pub fn use_flag_required(&mut self, val: i32) {
        self.isitset = true;
        self.theuseflag = val;
    }
    /// The hierarchy status is ignored.
    pub fn hierarchy_status_ignored(&mut self) {
        self.isitset = true;
        self.thehierst = IGNORED;
    }
    /// The hierarchy status must be `val`.
    pub fn hierarchy_status_required(&mut self, val: i32) {
        self.isitset = true;
        self.thehierst = val;
    }
    /// Checks a whole directory entry; what is wrong goes to `ach`.
    pub fn check<T: ?Sized + RuntimeTyped>(&self, ach: &Handle<InterfaceCheck>, ent: &Handle<T>) {
        let entity = ent.cast::<IgesDataIgesEntity>();
        let (Some(check), Some(live)) = (ach.get(), entity.get()) else {
            return;
        };
        if self.thetype != 0 {
            self.check_identity(check, live);
        }
        if self.thestructure == IgesDataDefType::DefReference && !live.has_structure() {
            check.send_fail(&iges_msg("XSTEP_59"));
        }
        if self.thegraphier != -1 && self.thegraphier != live.hierarchy_status() {
            self.check_graphics(check, live);
        }
        check_range(check, live.blank_status(), 1, "XSTEP_65");
        check_range(check, live.subordinate_status(), 3, "XSTEP_66");
        check_range(check, live.use_flag(), 5, "XSTEP_67");
        check_range(check, live.hierarchy_status(), 2, "XSTEP_68");
    }
    fn check_identity(&self, check: &InterfaceCheck, live: &IgesDataIgesEntity) {
        if live.type_number() != self.thetype {
            check.send_fail(&iges_msg("XSTEP_58"));
        }
        if self.form_out_of_range(live) {
            check.send_fail(&iges_msg("XSTEP_71"));
        }
    }
    fn form_out_of_range(&self, live: &IgesDataIgesEntity) -> bool {
        self.theform1 <= self.theform2
            && (live.form_number() < self.theform1 || live.form_number() > self.theform2)
    }
    fn check_graphics(&self, check: &InterfaceCheck, live: &IgesDataIgesEntity) {
        let font = live.def_line_font();
        if is_error(font) {
            check.send_fail(&iges_msg("XSTEP_60"));
        } else if self.thelinefont == IgesDataDefType::DefValue && font != IgesDataDefType::DefValue
        {
            check.send_warning(&iges_msg("XSTEP_60"));
        }
        let weight = iges_msg("XSTEP_69");
        if self.thelineweig == IgesDataDefType::DefValue && live.line_weight_number() == 0 {
            check.send_warning(&weight);
        }
        let color = live.def_color();
        if is_error(color) {
            check.send_fail(&weight);
        } else if self.thecolor == IgesDataDefType::DefValue && color != IgesDataDefType::DefValue {
            check.send_warning(&weight);
        }
    }
    /// Checks the type and form only, by warnings: this is called on reading, where a failure
    /// would mean that the entity could not be loaded.
    pub fn check_type_and_form<T: ?Sized + RuntimeTyped>(
        &self,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<T>,
    ) {
        let entity = ent.cast::<IgesDataIgesEntity>();
        let (Some(check), Some(live)) = (ach.get(), entity.get()) else {
            return;
        };
        if self.thetype == 0 {
            return;
        }
        if live.type_number() != self.thetype {
            check.send_warning(&iges_msg("XSTEP_58"));
        }
        if self.form_out_of_range(live) {
            check.send_warning(&iges_msg("XSTEP_71"));
        }
    }
    /// Brings a directory entry to the criteria; true when something changed.
    pub fn correct<T: ?Sized + RuntimeTyped>(&self, ent: &Handle<T>) -> bool {
        let entity = ent.cast::<IgesDataIgesEntity>();
        let Some(live) = entity.get() else {
            return false;
        };
        let mut done = self.correct_identity(live);
        done |= self.correct_graphics(live);
        done |= self.correct_misc(live);
        done |= self.correct_status(live);
        done
    }
    fn correct_identity(&self, live: &IgesDataIgesEntity) -> bool {
        if self.thetype == 0 {
            return false;
        }
        let form = live.form_number();
        if self.theform1 >= 0 && self.theform1 == self.theform2 && self.theform1 != form {
            live.init_type_and_form(self.thetype, self.theform1);
            return true;
        }
        if self.thetype != live.type_number() {
            live.init_type_and_form(self.thetype, form);
            return true;
        }
        false
    }
    fn correct_graphics(&self, live: &IgesDataIgesEntity) -> bool {
        let mut done = false;
        let no_graphics = self.thegraphier == -1;
        if no_graphics
            || (live.rank_line_font() != 0 && self.thelinefont == IgesDataDefType::DefVoid)
        {
            live.init_line_font(&Handle::default(), 0);
            done = true;
        }
        if no_graphics || (live.rank_color() != 0 && self.thecolor == IgesDataDefType::DefVoid) {
            live.init_color(&Handle::default(), 0);
            done = true;
        }
        if no_graphics && (live.view().is_some() || live.level() != 0) {
            live.init_view(&Handle::default());
            live.init_level(&Handle::default(), 0);
            done = true;
        }
        done
    }
    fn correct_misc(&self, live: &IgesDataIgesEntity) -> bool {
        let no_graphics = self.thegraphier == -1;
        let no_structure = self.thestructure == IgesDataDefType::DefVoid;
        let structure = if no_structure {
            Handle::default()
        } else {
            live.structure()
        };
        let label = if no_graphics {
            Handle::default()
        } else {
            live.label_display()
        };
        let linew = if !no_graphics && self.thelineweig != IgesDataDefType::DefVoid {
            live.line_weight_number()
        } else {
            0
        };
        let graphics_to_clear =
            no_graphics && (live.label_display().is_some() || live.line_weight_number() != 0);
        if graphics_to_clear || (live.has_structure() && no_structure) {
            live.init_misc(&structure, &label, linew);
            return true;
        }
        false
    }
    fn correct_status(&self, live: &IgesDataIgesEntity) -> bool {
        let required = [
            (self.theblankst, live.blank_status()),
            (self.thesubordst, live.subordinate_status()),
            (self.theuseflag, live.use_flag()),
            (self.thehierst, live.hierarchy_status()),
        ];
        let force = required
            .iter()
            .any(|(want, have)| *want >= 0 && want != have);
        if force {
            let value = |rank: usize| {
                let (want, have) = required[rank];
                if want >= 0 { want } else { have }
            };
            live.init_status(value(0), value(1), value(2), value(3));
        }
        force
    }
}

fn is_error(def: IgesDataDefType) -> bool {
    matches!(def, IgesDataDefType::ErrorVal | IgesDataDefType::ErrorRef)
}

fn check_range(check: &InterfaceCheck, value: i32, max: i32, key: &str) {
    if value < 0 || value > max {
        check.send_fail(&iges_msg(key));
    }
}

#[cfg(test)]
mod tests {
    use super::{Handle, IgesDataDirChecker, IgesDataIgesEntity, InterfaceCheck};

    fn entity_of(type_number: i32, form: i32) -> Handle<IgesDataIgesEntity> {
        let entity = Handle::new(IgesDataIgesEntity::default());
        entity.get().unwrap().init_type_and_form(type_number, form);
        entity
    }

    #[test]
    fn wrong_form_is_a_warning_on_reading_and_a_failure_on_checking() {
        let checker = IgesDataDirChecker::with_forms(110, 0, 2);
        let entity = entity_of(110, 5);
        let check = Handle::new(InterfaceCheck::new());
        checker.check_type_and_form(&check, &entity);
        assert_eq!(check.get().unwrap().nb_warnings(), 1);
        assert_eq!(check.get().unwrap().nb_fails(), 0);
        checker.check(&check, &entity);
        assert_eq!(check.get().unwrap().nb_fails(), 1);
    }

    #[test]
    fn correct_sets_the_required_form_and_status() {
        let mut checker = IgesDataDirChecker::with_form(110, 3);
        checker.blank_status_required(1);
        let entity = entity_of(110, 0);
        assert!(checker.correct(&entity));
        let live = entity.get().unwrap();
        assert_eq!((live.form_number(), live.blank_status()), (3, 1));
        assert!(!checker.correct(&entity));
    }

    #[test]
    fn nothing_is_set_by_default() {
        assert!(!IgesDataDirChecker::new().is_set());
    }
}
