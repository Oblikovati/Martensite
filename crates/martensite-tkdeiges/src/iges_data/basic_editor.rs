//! The global edits of a model: its unit, the status of its entities, their corrections.
use super::{
    IgesDataGeneralModule, IgesDataIgesEntity, IgesDataIgesModel, IgesDataProtocolOps,
    IgesDataSpecificLib, iges_live,
};
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkxsbase::interface::{
    InterfaceEntityIterator, InterfaceGeneralLib, InterfaceGraph, InterfaceInterfaceModel,
    InterfaceModelBase, InterfaceModelEntities, InterfaceProtocol,
};

/// The edits of a model.
///
/// ```
/// use martensite_tkdeiges::iges_data::IgesDataBasicEditor;
/// assert_eq!(IgesDataBasicEditor::unit_name_flag(b"2HMM"), 2);
/// assert_eq!(IgesDataBasicEditor::unit_flag_value(1), 25.4);
/// assert_eq!(IgesDataBasicEditor::iges_version_name(11), "5.3");
/// ```
pub struct IgesDataBasicEditor {
    theunit: bool,
    theproto: Handle<dyn IgesDataProtocolOps>,
    themodel: Handle<IgesDataIgesModel>,
    theglib: InterfaceGeneralLib,
    theslib: IgesDataSpecificLib,
}

impl Default for IgesDataBasicEditor {
    fn default() -> Self {
        Self {
            theunit: false,
            theproto: Handle::default(),
            themodel: Handle::default(),
            theglib: InterfaceGeneralLib::empty(),
            theslib: IgesDataSpecificLib::empty(),
        }
    }
}

impl IgesDataBasicEditor {
    /// An editor without model.
    pub fn new() -> Self {
        Self::default()
    }
    /// An editor on the template model of the session.
    pub fn with_protocol(protocol: &Handle<dyn IgesDataProtocolOps>) -> Self {
        let mut editor = Self::new();
        editor.init(protocol);
        editor
    }
    /// An editor on a model.
    pub fn with_model(
        model: &Handle<IgesDataIgesModel>,
        protocol: &Handle<dyn IgesDataProtocolOps>,
    ) -> Self {
        let mut editor = Self::new();
        editor.init_model(model, protocol);
        editor
    }
    /// Sets the template model of the session, or null when there is none, and the libraries.
    pub fn init(&mut self, protocol: &Handle<dyn IgesDataProtocolOps>) {
        let template = InterfaceModelBase::template("iges")
            .map_or_else(|_| Handle::default(), |model| model.cast());
        self.init_model(&template, protocol);
    }
    /// Sets the model and the libraries.
    pub fn init_model(
        &mut self,
        model: &Handle<IgesDataIgesModel>,
        protocol: &Handle<dyn IgesDataProtocolOps>,
    ) {
        self.theunit = false;
        self.theproto = protocol.clone();
        self.themodel = model.clone();
        let general: Handle<dyn InterfaceProtocol> = protocol.cast();
        self.theglib = InterfaceGeneralLib::new(&general);
        self.theslib = IgesDataSpecificLib::new(protocol);
    }
    /// The model.
    pub fn model(&self) -> Handle<IgesDataIgesModel> {
        self.themodel.clone()
    }
    /// Sets the unit by its flag, and its name; false for a wrong flag or without model.
    pub fn set_unit_flag(&mut self, flag: i32) -> bool {
        let Some(model) = self.themodel.get() else {
            return false;
        };
        if !(1..=11).contains(&flag) {
            return false;
        }
        let mut header = model.global_section();
        let known = Self::unit_flag_name(flag);
        let name = if known.is_empty() {
            header.unit_name()
        } else {
            Handle::new(martensite_tkernel::tcollection::HAsciiString::new(known))
        };
        header.set_unit_flag(flag);
        header.set_unit_name(&name);
        model.set_global_section(&header);
        self.theunit = true;
        true
    }
    /// Sets the unit by its value; false when no flag has that value.
    pub fn set_unit_value(&mut self, val: f64) -> bool {
        if val <= 0.0 {
            return false;
        }
        let Some(model) = self.themodel.get() else {
            return false;
        };
        let flag = Self::get_flag_by_value(val * model.global_section().cascade_unit());
        flag > 0 && self.set_unit_flag(flag)
    }
    /// Sets the unit by its name. With the flag of a free unit name, the name is just taken.
    pub fn set_unit_name(&mut self, name: &[u8]) -> bool {
        let Some(model) = self.themodel.get() else {
            return false;
        };
        let flag = Self::unit_name_flag(name);
        let mut header = model.global_section();
        if header.unit_flag() == 3 {
            let text = if name.get(1) == Some(&b'H') {
                &name[2..]
            } else {
                name
            };
            header.set_unit_name(&Handle::new(
                martensite_tkernel::tcollection::HAsciiString::new(text),
            ));
            model.set_global_section(&header);
            return true;
        }
        flag > 0 && self.set_unit_flag(flag)
    }
    /// Converts the values of the header to the unit set, or when `enforce` to the unit it holds.
    pub fn apply_unit(&mut self, enforce: bool) {
        let Some(model) = self.themodel.get() else {
            return;
        };
        if !enforce && !self.theunit {
            return;
        }
        let mut header = model.global_section();
        let unit = header.unit_value();
        if unit <= 0.0 {
            return;
        }
        if unit != 1.0 {
            header.set_max_line_weight(header.max_line_weight() / unit);
            header.set_resolution(header.resolution() / unit);
            header.set_max_coord(header.max_coord() / unit);
            model.set_global_section(&header);
        }
        self.theunit = false;
    }
    /// Computes the subordinate status and the use flag of every entity from the sharings.
    ///
    /// # Errors
    /// A failure of the graph or of a module.
    pub fn compute_status(&mut self) -> Result<(), Failure> {
        let Some(model) = self.themodel.get() else {
            return Ok(());
        };
        let nb = model.nb_entities();
        if nb == 0 {
            return Ok(());
        }
        let general: Handle<dyn InterfaceInterfaceModel> = self.themodel.cast();
        let mut subs = vec![0_i32; nb as usize + 1];
        let mut graph = InterfaceGraph::with_lib(&general, &self.theglib, true)?;
        graph.reset_status()?;
        for i in 1..=nb {
            self.status_of_entity(model, i, &mut subs, &mut graph)?;
        }
        for i in 1..=nb {
            if let Some(ent) = model.entity(i).get() {
                let flag = match ent.use_flag() {
                    0 => graph.status(i)?,
                    set => set,
                };
                ent.init_status(
                    ent.blank_status(),
                    subs[i as usize],
                    flag,
                    ent.hierarchy_status(),
                );
            }
        }
        Ok(())
    }
    /// The contribution of one entity to its descendants: drawings and the annotations of type
    /// 402 and 404 make them logical, the rest physical; the use flag comes down from annotations
    /// and meshes.
    fn status_of_entity(
        &self,
        model: &IgesDataIgesModel,
        i: i32,
        subs: &mut [i32],
        graph: &mut InterfaceGraph,
    ) -> Result<(), Failure> {
        let ent = model.entity(i);
        let live = iges_live(&ent, "compute status")?;
        let igt = live.type_number();
        let transient: Handle<StandardTransient> = ent.cast();
        if let Some(shared) = self.own_shared(&ent)? {
            shared.start();
            while shared.more() {
                let nums = model.number(&shared.value()?) as usize;
                subs[nums] |= if igt == 402 || igt == 404 { 2 } else { 1 };
                shared.next();
            }
        }
        if igt / 100 == 2 {
            graph.get_from_entity(&transient, true, 1)?;
            graph.get_from_entity(&transient, false, live.use_flag())?;
        } else if (igt == 134 || igt == 116 || igt == 132)
            && graph.sharings(&transient)?.nb_entities() > 0
        {
            graph.get_from_entity(&transient, true, 4)?;
        }
        Ok(())
    }
    fn own_shared(
        &self,
        ent: &Handle<IgesDataIgesEntity>,
    ) -> Result<Option<InterfaceEntityIterator>, Failure> {
        let Some((gmodule, cn)) = self.theglib.select(&ent.cast()) else {
            return Ok(None);
        };
        let module = gmodule.cast::<dyn IgesDataGeneralModule>();
        let mut list = InterfaceEntityIterator::new();
        iges_live(&module, "own shared")?.own_shared_case(cn, ent, &mut list)?;
        Ok(Some(list))
    }
    /// Corrects an entity: what its associativities say of the model, its directory entry, its own
    /// parameters. True when something changed.
    ///
    /// # Errors
    /// A failure of a module.
    pub fn auto_correct<T: ?Sized + martensite_tkernel::standard::RuntimeTyped>(
        &self,
        ent: &Handle<T>,
    ) -> Result<bool, Failure> {
        let entity = ent.cast::<IgesDataIgesEntity>();
        let (Some(model), Some(live)) = (self.themodel.get(), entity.get()) else {
            return Ok(false);
        };
        let mut done = false;
        let associativities = live.associativities();
        associativities.start();
        while associativities.more() {
            let sub = associativities.value()?.cast::<IgesDataIgesEntity>();
            if sub.is_some() && model.number(&sub.cast()) == 0 {
                IgesDataIgesEntity::dissociate(&sub, &entity)?;
                done = true;
            }
            associativities.next();
        }
        let transient: Handle<StandardTransient> = entity.cast();
        if let Some((gmodule, cn)) = self.theglib.select(&transient) {
            let module = gmodule.cast::<dyn IgesDataGeneralModule>();
            if let Some(general) = module.get() {
                done |= general.dir_checker(cn, &entity).correct(&entity);
            }
        }
        if let Some((smodule, cn)) = self.theslib.select(&transient)
            && let Some(specific) = smodule.get()
        {
            done |= specific.own_correct(cn, &entity)?;
        }
        Ok(done)
    }
    /// Corrects every entity of the model; the count of those that changed.
    ///
    /// # Errors
    /// A failure of a module.
    pub fn auto_correct_model(&self) -> Result<i32, Failure> {
        let model = iges_live(&self.themodel, "auto correct model")?;
        let mut res = 0;
        for i in 1..=model.nb_entities() {
            if self.auto_correct(&model.entity(i))? {
                res += 1;
            }
        }
        Ok(res)
    }
    /// The flag of a unit name, in counted form or not; 0 when it is not known.
    pub fn unit_name_flag(name: &[u8]) -> i32 {
        let text = if name.get(1) == Some(&b'H') {
            &name[2..]
        } else {
            name
        };
        match text {
            b"INCH" | b"IN" => 1,
            b"MM" => 2,
            b"FT" => 4,
            b"MI" => 5,
            b"M" => 6,
            b"KM" => 7,
            b"MIL" => 8,
            b"UM" => 9,
            b"CM" => 10,
            b"UIN" => 11,
            _ => 0,
        }
    }
    /// The value of a unit flag in millimeters; 0 when it is not known.
    pub fn unit_flag_value(flag: i32) -> f64 {
        match flag {
            1 => 25.4,
            2 | 3 => 1.0,
            4 => 304.8,
            5 => 1_609_344.0,
            6 => 1000.0,
            7 => 1_000_000.0,
            8 => 0.0254,
            9 => 0.001,
            10 => 10.0,
            11 => 0.000_025_4,
            _ => 0.0,
        }
    }
    /// The name of a unit flag; empty when it is not known.
    pub fn unit_flag_name(flag: i32) -> &'static str {
        match flag {
            1 => "INCH",
            2 => "MM",
            4 => "FT",
            5 => "MI",
            6 => "M",
            7 => "KM",
            8 => "MIL",
            9 => "UM",
            10 => "CM",
            11 => "UIN",
            _ => "",
        }
    }
    /// The name of a version of the norm; empty when it is not known.
    pub fn iges_version_name(flag: i32) -> &'static str {
        match flag {
            1 => "1.0",
            2 => "ANSI Y14.26M-1981",
            3 => "2.0",
            4 => "3.0",
            5 => "ANSI Y14.26M-1987",
            6 => "4.0",
            7 => "ANSI Y14.26M-1989",
            8 => "5.0",
            9 => "5.1",
            10 => "5.2",
            11 => "5.3",
            _ => "",
        }
    }
    /// The last version of the norm.
    pub fn iges_version_max() -> i32 {
        11
    }
    /// The name of a drafting standard; empty when it is not known.
    pub fn drafting_name(flag: i32) -> &'static str {
        match flag {
            0 => "(None)",
            1 => "ISO",
            2 => "AFNOR",
            3 => "ANSI",
            4 => "BSI",
            5 => "CSA",
            6 => "DIN",
            7 => "JIS",
            _ => "",
        }
    }
    /// The last drafting standard.
    pub fn drafting_max() -> i32 {
        7
    }
    /// The unit flag whose value is close to `value`, in millimeters; 0 when none is.
    pub fn get_flag_by_value(value: f64) -> i32 {
        const RANGES: [(f64, f64, i32); 10] = [
            (25.0, 26.0, 1),
            (0.9, 1.1, 2),
            (300.0, 310.0, 4),
            (1_600_000.0, 1_620_000.0, 5),
            (990.0, 1010.0, 6),
            (990_000.0, 1_010_000.0, 7),
            (0.025, 0.026, 8),
            (0.0009, 0.0011, 9),
            (9.0, 11.0, 10),
            (0.000_025, 0.000_026, 11),
        ];
        RANGES
            .iter()
            .find(|(low, high, _)| value >= *low && value <= *high)
            .map_or(0, |(_, _, flag)| *flag)
    }
}
