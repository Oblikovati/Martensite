//! The commands of the attribute and transaction scripts.
use super::commands::{CafSession, entry_of};
use crate::support::script_error::script_error;
use martensite_tkernel::standard::Handle;
use martensite_tklcaf::tdata_std::{TDataStdInteger, TDataStdReal};
use martensite_tklcaf::tdf::{TdfAttributeDelta, TdfAttributeIterator};

/// The attribute kinds the transaction scripts name in their commands.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Attr {
    Integer,
    Real,
}

impl Attr {
    /// The kind of a script's attribute number: 0 is the integer; `Attr::of(0)`.
    pub fn of(id: i32) -> Self {
        if id == 0 { Self::Integer } else { Self::Real }
    }

    /// The name the commands and the type name are built from; `Attr::Real.name() == "Real"`.
    pub fn name(self) -> &'static str {
        match self {
            Self::Integer => "Integer",
            Self::Real => "Real",
        }
    }

    fn guid_text(self) -> &'static str {
        match self {
            Self::Integer => "2a96b606-ec8b-11d0-bee7-080009dc3333",
            Self::Real => "2a96b60f-ec8b-11d0-bee7-080009dc3333",
        }
    }
}

impl CafSession {
    /// `Label D entry`: makes the label and answers its entry.
    /// Example: `session.label("0:2")`.
    pub fn label(&self, entry: &str) -> String {
        println!("Label D {entry} ");
        let answer = entry_of(&self.add_label(entry));
        println!("{answer}");
        answer
    }

    /// `ForgetAll D entry`.
    /// Example: `session.forget_all("0:2");`.
    pub fn forget_all(&self, entry: &str) {
        println!("ForgetAll D {entry} ");
        self.find_label(entry).forget_all_attributes(true).unwrap();
    }

    /// `ForgetAtt D entry guid`.
    /// Example: `session.forget_attribute("0:2", Attr::Real);`.
    pub fn forget_attribute(&self, entry: &str, attr: Attr) {
        println!("ForgetAtt D {entry} {} ", attr.guid_text());
        let guid = attr.guid_text().parse().unwrap();
        self.find_label(entry).forget_attribute_id(&guid).unwrap();
    }

    /// `SetInteger` or `SetReal D entry value`.
    /// Example: `session.set_value("0:2", Attr::Real, 1);`.
    pub fn set_value(&self, entry: &str, attr: Attr, value: i32) {
        println!("Set{} D {entry} {value} ", attr.name());
        let label = self.add_label(entry);
        match attr {
            Attr::Integer => drop(TDataStdInteger::set(&label, value).unwrap()),
            Attr::Real => drop(TDataStdReal::set(&label, f64::from(value)).unwrap()),
        }
    }

    /// `SetReal D entry value` with a value in the script's own spelling.
    /// Example: `session.set_real("0:2", "300.", 300.);`.
    pub fn set_real(&self, entry: &str, written: &str, value: f64) {
        println!("SetReal D {entry} {written} ");
        TDataStdReal::set(&self.add_label(entry), value).unwrap();
    }

    /// `GetInteger` or `GetReal D entry`; a script error, as the command's, without the attribute.
    /// Example: `session.get_value("0:2", Attr::Real)`.
    pub fn get_value(&self, entry: &str, attr: Attr) -> i32 {
        println!("Get{} D {entry} ", attr.name());
        let label = self.find_label(entry);
        let value = match attr {
            Attr::Integer => {
                let mut found: Handle<TDataStdInteger> = Handle::default();
                let has = label.find_attribute_as(&TDataStdInteger::get_id(), &mut found);
                has.unwrap().then(|| f64::from(found.get().unwrap().get()))
            }
            Attr::Real => {
                let mut found: Handle<TDataStdReal> = Handle::default();
                let has = label.find_attribute_as(&TDataStdReal::get_id(), &mut found);
                has.unwrap().then(|| found.get().unwrap().get())
            }
        };
        let Some(value) = value else {
            script_error(&format!("Get{} {entry}: no such attribute", attr.name()));
        };
        println!("{value}");
        value as i32
    }

    /// `Attributes D entry`: the type names, each followed by a space, on one line.
    /// Example: `session.attributes("0:2")`.
    pub fn attributes(&self, entry: &str) -> Vec<String> {
        println!("Attributes D {entry} ");
        let label = self.find_label(entry);
        let mut names = Vec::new();
        let mut iterator = TdfAttributeIterator::new(&label, true).unwrap();
        while iterator.more() {
            let attribute = iterator.value();
            names.push(attribute.dynamic_type().unwrap().name().to_owned());
            iterator.next();
        }
        let answer: String = names.iter().map(|name| format!("{name} ")).collect();
        println!("{answer}");
        names
    }

    /// `DumpCommand D`: the last undo's attribute changes, sorted by kind; its words as a list.
    /// Example: `session.dump_command().len()`.
    pub fn dump_command(&self) -> Vec<String> {
        println!("DumpCommand D ");
        let undos = self.document().get_undos();
        let Some(delta) = undos.last() else {
            println!("no UNDO available");
            return vec!["no".into(), "UNDO".into(), "available".into()];
        };
        let deltas = delta.attribute_deltas();
        let mut text = String::new();
        for (title, kind) in DELTA_KINDS {
            text += &delta_section(title, kind, &deltas);
        }
        println!("{text}");
        text.split_whitespace().map(str::to_owned).collect()
    }
}

/// The sections of the dump, each with the runtime name of its delta's kind.
const DELTA_KINDS: [(&str, &str); 5] = [
    ("ADDED    :", "TDF_DeltaOnAddition"),
    ("FORGOTEN :", "TDF_DeltaOnForget"),
    ("RESUMED  :", "TDF_DeltaOnResume"),
    ("REMOVED  :", "TDF_DeltaOnRemoval"),
    ("MODIFIED :", "TDF_DeltaOnModification"),
];

fn delta_section(title: &str, kind: &str, deltas: &[Handle<dyn TdfAttributeDelta>]) -> String {
    let mut lines = String::new();
    for delta in deltas.iter().filter(|delta| delta.is_kind_name(kind)) {
        let attribute = delta.attribute();
        let name = attribute.dynamic_type().unwrap().name();
        lines += &format!("- {} {name}\n", entry_of(&delta.label()));
    }
    if lines.is_empty() {
        return format!("{title} empty\n");
    }
    format!("{title}\n{lines}")
}
