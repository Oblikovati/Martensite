//! The model of an IGES file: its entities, start lines and global section.
use super::model_dump::dump_global;
use super::model_verify::verify_header;
use super::{IgesDataGlobalSection, IgesDataIgesEntity, IgesDataTextLines};
use martensite_tkernel::{
    impl_transient,
    standard::{Failure, Handle, RuntimeTyped, SharedState, StandardTransient},
    tcollection::HAsciiString,
};
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceInterfaceModel, InterfaceModelBase, InterfaceModelEntities,
    InterfaceStatic,
};
use std::io::{self, Write};

struct ModelFields {
    thestart: Handle<IgesDataTextLines>,
    theheader: IgesDataGlobalSection,
}

impl Default for ModelFields {
    fn default() -> Self {
        Self {
            thestart: Handle::new(IgesDataTextLines::default()),
            theheader: IgesDataGlobalSection::new(),
        }
    }
}

/// The entities of an IGES file, with its start lines and global section.
/// [`IgesDataIgesModel::new`] gives the bound handle the general interface needs.
///
/// ```
/// use martensite_tkdeiges::iges_data::IgesDataIgesModel;
/// use martensite_tkxsbase::interface::InterfaceModelEntities;
/// let model = IgesDataIgesModel::new();
/// assert_eq!(model.nb_entities(), 0);
/// assert_eq!(model.nb_start_lines(), 0);
/// ```
#[derive(Default)]
pub struct IgesDataIgesModel {
    transient: StandardTransient,
    base: InterfaceModelBase,
    state: SharedState<ModelFields>,
}
impl_transient!(
    IgesDataIgesModel,
    "IgesDataIgesModel",
    dyn InterfaceInterfaceModel,
    transient,
    interfaces: [dyn InterfaceInterfaceModel]
);

fn line_text(lines: &Handle<IgesDataTextLines>, num: i32) -> Vec<u8> {
    lines
        .get()
        .and_then(|live| {
            live.sequence(|items| {
                items
                    .value(usize::try_from(num).ok()?)
                    .ok()
                    .map(|line| line.get().map(|t| t.string().as_bytes().to_vec()))
            })
        })
        .flatten()
        .unwrap_or_default()
}

fn nb_of(lines: &Handle<IgesDataTextLines>) -> i32 {
    lines
        .get()
        .map_or(0, |live| live.sequence(|items| items.length()))
}

impl IgesDataIgesModel {
    /// An empty model, as a bound handle.
    pub fn new() -> Handle<Self> {
        let model = Handle::new(Self::default());
        if let Some(live) = model.get() {
            live.base.bind_self(&model.cast());
        }
        model
    }
    /// The start lines.
    pub fn start_section(&self) -> Handle<IgesDataTextLines> {
        self.state.read().thestart.clone()
    }
    /// The count of start lines.
    pub fn nb_start_lines(&self) -> i32 {
        nb_of(&self.state.read().thestart)
    }
    /// A start line, by its rank from 1; empty for a rank out of range.
    pub fn start_line(&self, num: i32) -> Vec<u8> {
        line_text(&self.state.read().thestart, num)
    }
    /// Forgets the start lines.
    pub fn clear_start_section(&self) {
        if let Some(lines) = self.state.read().thestart.get() {
            lines.change_sequence(|items| items.clear(None));
        }
    }
    /// Takes the start lines of `list`: copies when `copy`, else `list` itself; empty for null.
    pub fn set_start_section(&self, list: &Handle<IgesDataTextLines>, copy: bool) {
        let lines = if list.is_null() {
            Handle::new(IgesDataTextLines::default())
        } else if copy {
            let copied = Handle::new(IgesDataTextLines::default());
            for num in 1..=nb_of(list) {
                if let Some(target) = copied.get() {
                    target.append(Handle::new(HAsciiString::new(line_text(list, num))));
                }
            }
            copied
        } else {
            list.clone()
        };
        self.state.write().thestart = lines;
    }
    /// Adds a start line before the rank `atnum`, at the end for a rank out of range.
    pub fn add_start_line(&self, line: impl AsRef<[u8]>, atnum: i32) {
        let text = Handle::new(HAsciiString::new(line));
        let state = self.state.read();
        let Some(lines) = state.thestart.get() else {
            return;
        };
        lines.change_sequence(|items| {
            if atnum <= 0 || atnum > items.length() {
                items.append(text);
            } else {
                // The rank is checked above.
                let _inserted = items.insert_before(atnum as usize, text);
            }
        });
    }
    /// A copy of the global section.
    pub fn global_section(&self) -> IgesDataGlobalSection {
        self.state.read().theheader.clone()
    }
    /// Lends the global section to `edit`; nothing else of the model may be called in it.
    pub fn change_global_section<R>(
        &self,
        edit: impl FnOnce(&mut IgesDataGlobalSection) -> R,
    ) -> R {
        edit(&mut self.state.write().theheader)
    }
    /// Sets the global section.
    pub fn set_global_section(&self, header: &IgesDataGlobalSection) {
        self.state.write().theheader = header.clone();
    }
    /// Takes the fields of the header from the session's parameters: `r` receiver, `a` author,
    /// `c` company, all of them for an empty text. False when the parameter is empty.
    pub fn apply_static(&self, param: &str) -> bool {
        if param.is_empty() {
            self.apply_static("receiver");
            self.apply_static("author");
            self.apply_static("company");
            return true;
        }
        let (name, field): (&str, fn(&mut IgesDataGlobalSection, &Handle<HAsciiString>)) =
            match param.as_bytes()[0] {
                b'r' => (
                    "write.iges.header.receiver",
                    IgesDataGlobalSection::set_receive_name,
                ),
                b'a' => (
                    "write.iges.header.author",
                    IgesDataGlobalSection::set_author_name,
                ),
                b'c' => (
                    "write.iges.header.company",
                    IgesDataGlobalSection::set_company_name,
                ),
                _ => return true,
            };
        let value = InterfaceStatic::c_val(name);
        if value.is_empty() {
            return false;
        }
        let text = Handle::new(HAsciiString::new(value.as_bytes()));
        field(&mut self.state.write().theheader, &text);
        true
    }
    /// The entity at a rank from 1; null when out of range or not an IGES entity.
    pub fn entity(&self, num: i32) -> Handle<IgesDataIgesEntity> {
        self.value(num)
            .map_or_else(|_| Handle::default(), |ent| ent.cast())
    }
    /// The directory number of an entity: `2 * rank - 1`, 0 when it is not in the model.
    pub fn d_num<T: ?Sized + RuntimeTyped>(&self, ent: &Handle<T>) -> i32 {
        match self.number(&ent.cast()) {
            0 => 0,
            num => 2 * num - 1,
        }
    }
    /// Computes the line weight value of every entity.
    pub fn set_line_weights(&self, defw: f64) {
        let header = self.global_section();
        let mut lwg = header.line_weight_grad();
        let mut maxw = header.max_line_weight();
        if lwg > 0 {
            maxw /= f64::from(lwg);
            lwg = 1;
        }
        for i in 1..=self.nb_entities() {
            if let Some(entity) = self.entity(i).get() {
                entity.set_line_weight(defw, maxw, lwg);
            }
        }
    }
    /// Writes a directory number and the class of an entity.
    ///
    /// # Errors
    /// A failure of the stream.
    pub fn print_info(&self, ent: &Handle<StandardTransient>, s: &mut dyn Write) -> io::Result<()> {
        if ent.cast::<IgesDataIgesEntity>().is_null() {
            return s.write_all(b"(NOT IGES)");
        }
        match self.number(ent) {
            0 => s.write_all(b"??"),
            num => write!(s, "{}type {}", 2 * num - 1, self.entity_type(ent, 1).name()),
        }
    }
    fn dump_start(&self, s: &mut dyn Write) -> io::Result<()> {
        let ns = self.nb_start_lines();
        writeln!(
            s,
            "****    Dump of IGES Model , Start and Global Sections   ****"
        )?;
        if ns > 0 {
            writeln!(s, "****    Start Section : {ns} Line(s)   ****")?;
            for i in 1..=ns {
                let pad = if i < 10 { " " } else { "" };
                write!(s, "[{pad}{i}]:")?;
                s.write_all(&self.start_line(i))?;
                writeln!(s)?;
            }
        }
        Ok(())
    }
}

impl InterfaceInterfaceModel for IgesDataIgesModel {
    fn model_base(&self) -> &InterfaceModelBase {
        &self.base
    }
    fn clear_labels(&self) {}
    fn clear_header(&self) {
        *self.state.write() = ModelFields::default();
    }
    fn get_from_another(&self, other: &Handle<dyn InterfaceInterfaceModel>) -> Result<(), Failure> {
        let another = other.cast::<IgesDataIgesModel>();
        let source = super::iges_live(&another, "IGES model: get from another")?;
        let mut header = source.global_section();
        header.copy_refs();
        self.set_global_section(&header);
        self.set_start_section(&source.start_section(), true);
        Ok(())
    }
    fn new_empty_model(&self) -> Handle<dyn InterfaceInterfaceModel> {
        IgesDataIgesModel::new().cast()
    }
    fn verify_check(&self, ach: &mut Handle<InterfaceCheck>) {
        if let Some(check) = ach.get() {
            verify_header(&self.global_section(), check);
        }
    }
    fn dump_header(&self, s: &mut dyn Write, _level: i32) -> io::Result<()> {
        self.dump_start(s)?;
        dump_global(s, &self.global_section())
    }
    fn print_label(&self, ent: &Handle<StandardTransient>, s: &mut dyn Write) -> io::Result<()> {
        if ent.cast::<IgesDataIgesEntity>().is_null() {
            return s.write_all(b"Null");
        }
        match self.number(ent) {
            0 => s.write_all(b"??"),
            num => write!(s, "D{}", 2 * num - 1),
        }
    }
    fn print_to_log(&self, ent: &Handle<StandardTransient>, s: &mut dyn Write) -> io::Result<()> {
        let iges = ent.cast::<IgesDataIgesEntity>();
        let Some(live) = iges.get() else {
            return Ok(());
        };
        match self.number(ent) {
            0 => s.write_all(b"??"),
            num => write!(s, " DE : {} type : {}", 2 * num - 1, live.type_number()),
        }
    }
    fn string_label(&self, ent: &Handle<StandardTransient>) -> Handle<HAsciiString> {
        if ent.cast::<IgesDataIgesEntity>().is_null() {
            return Handle::new(HAsciiString::new("(NOT IGES)"));
        }
        let text = match self.number(ent) {
            0 => "D0...".to_owned(),
            num => format!("D{}", 2 * num - 1),
        };
        Handle::new(HAsciiString::new(text))
    }
}

#[cfg(test)]
mod tests {
    use super::{Handle, IgesDataIgesEntity, IgesDataIgesModel, InterfaceInterfaceModel};

    #[test]
    fn start_section_keeps_its_order() {
        let model = IgesDataIgesModel::new();
        let live = model.get().unwrap();
        live.add_start_line("second", 0);
        live.add_start_line("first", 1);
        assert_eq!(live.start_line(1), b"first");
        assert_eq!(live.start_line(2), b"second");
        assert_eq!(live.start_line(3), b"");
    }

    #[test]
    fn labels_of_entities_outside_the_model() {
        let model = IgesDataIgesModel::new();
        let live = model.get().unwrap();
        let entity = Handle::new(IgesDataIgesEntity::default());
        let label = live.string_label(&entity.cast());
        assert_eq!(label.get().unwrap().string().as_bytes(), b"D0...");
        assert_eq!(live.d_num(&entity), 0);
    }
}
