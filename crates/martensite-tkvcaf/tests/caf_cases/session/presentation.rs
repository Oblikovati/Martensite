//! The presentation commands: each finds the attribute as the command does, calls the library and
//! writes what the command writes.
use super::commands::{CafSession, entry_of};
use super::pixel_check::significant_17;
use super::viewer::ViewerHost;
use crate::support::script_error::script_error;
use martensite_tkcaf::tdata_xtd::TDataXtdAxis;
use martensite_tkcaf::tnaming::TNamingNamedShape;
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkernel::standard::{Guid, Handle};
use martensite_tklcaf::tdf::TdfLabel;
use martensite_tkservice::graphic3d::Graphic3dNameOfMaterial;
use martensite_tkvcaf::tprs_std::{TPrsStdAISPresentation, TPrsStdAISViewer};

/// The material a script's number selects, of the ranks the scripts use.
fn material_of_rank(rank: i32) -> Graphic3dNameOfMaterial {
    match rank {
        0 => Graphic3dNameOfMaterial::Brass,
        18 => Graphic3dNameOfMaterial::Jade,
        other => panic!("material rank {other}; expected 0 or 18"),
    }
}

fn driver_guid(kind: &str) -> Guid {
    match kind {
        "NS" => TNamingNamedShape::get_id(),
        "A" => TDataXtdAxis::get_id(),
        other => panic!("driver {other:?}; expected NS or A"),
    }
}

impl CafSession {
    /// `AISInitViewer D`: the root's viewer attribute over a newly opened view.
    /// Example: `session.ais_init_viewer();`.
    pub fn ais_init_viewer(&mut self) {
        println!("AISInitViewer D ");
        let root = self.document().get_data().root();
        let mut viewer = Handle::default();
        if !TPrsStdAISViewer::find(&root, &mut viewer).unwrap() {
            println!("pload OPENGL ");
            let host = ViewerHost::open();
            TPrsStdAISViewer::new_with_context(&root, &host.context).unwrap();
            self.host = Some(host);
        }
        // The attribute lives on the root, whose entry the command answers.
        println!("{}", entry_of(&root));
    }

    /// `AISRepaint D`: updates the viewer of the document's root.
    /// Example: `session.ais_repaint();`.
    pub fn ais_repaint(&self) {
        println!("AISRepaint D ");
        TPrsStdAISViewer::update_label(&self.document().get_data().root()).unwrap();
    }

    /// `vsetdispmode mode`; `session.vsetdispmode(1);`.
    pub fn vsetdispmode(&self, mode: i32) {
        println!("vsetdispmode {mode} ");
        self.open_host().set_display_mode(mode);
    }

    /// `vfit`; `session.vfit();`.
    pub fn vfit(&self) {
        println!("vfit ");
        self.open_host().fit();
    }

    pub(super) fn open_host(&self) -> &ViewerHost {
        self.host.as_ref().expect("the script opened a viewer")
    }

    /// The presentation of a label; a script error when it has none.
    fn presentation(&self, label: &TdfLabel) -> Handle<TPrsStdAISPresentation> {
        presentation_on(label)
            .unwrap_or_else(|| script_error("label: expected a presentation attribute"))
    }

    /// The label and its presentation, for the commands that also ask for a viewer.
    fn in_viewer(&self, entry: &str) -> (TdfLabel, Handle<TPrsStdAISPresentation>) {
        let label = self.find_label(entry);
        let mut viewer = Handle::default();
        if !TPrsStdAISViewer::find(&label, &mut viewer).unwrap() {
            script_error(&format!("label {entry}: expected a viewer on its document"));
        }
        let presentation = self.presentation(&label);
        (label, presentation)
    }

    /// `AISSet D entry kind`; answers the driver's id.
    /// Example: `session.ais_set("0:2", "NS");`.
    pub fn ais_set(&self, entry: &str, kind: &str) -> String {
        println!("AISSet D {entry} {kind} ");
        let label = self.find_label(entry);
        let guid = driver_guid(kind);
        TPrsStdAISPresentation::set(&label, &guid).unwrap();
        println!("{guid}");
        guid.to_string()
    }

    /// `AISDriver D entry`; answers the driver's id.
    /// Example: `session.ais_driver("0:2");`.
    pub fn ais_driver(&self, entry: &str) -> String {
        println!("AISDriver D {entry} ");
        let presentation = self.presentation(&self.find_label(entry));
        let guid = presentation.get_driver_guid().unwrap();
        println!("{guid}");
        guid.to_string()
    }

    /// `AISDriver D entry kind`; answers the new driver's id.
    /// Example: `session.ais_set_driver("0:2", "A");`.
    pub fn ais_set_driver(&self, entry: &str, kind: &str) -> String {
        println!("AISDriver D {entry} {kind} ");
        let presentation = self.presentation(&self.find_label(entry));
        let guid = driver_guid(kind);
        presentation.set_driver_guid(&guid).unwrap();
        println!("{guid}");
        guid.to_string()
    }

    /// `AISDisplay D entry`: displays and updates the viewer.
    /// Example: `session.ais_display("0:2");`.
    pub fn ais_display(&self, entry: &str) {
        println!("AISDisplay D {entry} ");
        self.display(entry, true);
    }

    /// `AISDisplay D entry 0`: displays without an update of the presentation.
    /// Example: `session.ais_display_lazily("0:1");`.
    pub fn ais_display_lazily(&self, entry: &str) {
        println!("AISDisplay D {entry} 0 ");
        self.display(entry, false);
    }

    fn display(&self, entry: &str, update: bool) {
        let label = self.find_label(entry);
        self.presentation(&label).display(update).unwrap();
        TPrsStdAISViewer::update_label(&label).unwrap();
    }

    /// `AISUpdate D entry`.
    /// Example: `session.ais_update("0:2");`.
    pub fn ais_update(&self, entry: &str) {
        println!("AISUpdate D {entry} ");
        let label = self.find_label(entry);
        self.presentation(&label).update().unwrap();
        TPrsStdAISViewer::update_label(&label).unwrap();
    }

    /// `AISErase D entry`.
    /// Example: `session.ais_erase("0:2");`.
    pub fn ais_erase(&self, entry: &str) {
        println!("AISErase D {entry} ");
        let label = self.find_label(entry);
        self.presentation(&label).erase(false).unwrap();
        TPrsStdAISViewer::update_label(&label).unwrap();
    }

    /// `AISRemove D entry`.
    /// Example: `session.ais_remove("0:2");`.
    pub fn ais_remove(&self, entry: &str) {
        println!("AISRemove D {entry} ");
        let label = self.find_label(entry);
        self.presentation(&label).erase(true).unwrap();
        TPrsStdAISViewer::update_label(&label).unwrap();
    }

    /// `AISUnset D entry`.
    /// Example: `session.ais_unset("0:2");`.
    pub fn ais_unset(&self, entry: &str) {
        println!("AISUnset D {entry} ");
        let label = self.find_label(entry);
        TPrsStdAISPresentation::unset(&label).unwrap();
        TPrsStdAISViewer::update_label(&label).unwrap();
    }

    /// `AISColor D entry name`.
    /// Example: `session.ais_set_color("0:2", "GREEN");`.
    pub fn ais_set_color(&self, entry: &str, name: &str) {
        println!("AISColor D {entry} {name} ");
        let (label, presentation) = self.in_viewer(entry);
        let color = NamedColor::from_name(name)
            .unwrap_or_else(|| script_error(&format!("color {name}: expected a named colour")));
        presentation.set_color(color).unwrap();
        TPrsStdAISViewer::update_label(&label).unwrap();
    }

    /// `AISColor D entry`: the own colour's name, or -1 with the command's warning; `None`, with
    /// the command's refusal, on a label without a presentation.
    /// Example: `session.ais_color("0:2")`.
    pub fn ais_color(&self, entry: &str) -> Option<String> {
        println!("AISColor D {entry} ");
        let label = self.find_label(entry);
        let mut viewer = Handle::default();
        let found = TPrsStdAISViewer::find(&label, &mut viewer).unwrap();
        let Some(presentation) = presentation_on(&label).filter(|_| found) else {
            println!("Syntax error: '{entry}' label has no presentation");
            return None;
        };
        if presentation.has_own_color().unwrap() {
            let name = Color::string_name(presentation.color().unwrap());
            println!("Color = {name}\n{name}");
            return Some(name.to_owned());
        }
        println!("DPrsStd_AISColor: Warning : Color wasn't set\n-1");
        Some("-1".to_owned())
    }

    /// `AISDefaultColor D entry`.
    /// Example: `session.ais_default_color("0:2");`.
    pub fn ais_default_color(&self, entry: &str) {
        println!("AISDefaultColor D {entry} ");
        let (label, presentation) = self.in_viewer(entry);
        presentation.unset_color().unwrap();
        TPrsStdAISViewer::update_label(&label).unwrap();
    }

    /// `AISHasOwnColor D entry`.
    /// Example: `session.ais_has_own_color("0:2")`.
    pub fn ais_has_own_color(&self, entry: &str) -> i32 {
        println!("AISHasOwnColor D {entry} ");
        let (_, presentation) = self.in_viewer(entry);
        answered(presentation.has_own_color().unwrap())
    }

    /// `AISMaterial D entry rank`, where the text a script passes reads as `rank`.
    /// Example: `session.ais_set_material("0:2", "18", 18);`.
    pub fn ais_set_material(&self, entry: &str, written: &str, rank: i32) {
        println!("AISMaterial D {entry} {written} ");
        let (label, presentation) = self.in_viewer(entry);
        presentation.set_material(material_of_rank(rank)).unwrap();
        TPrsStdAISViewer::update_label(&label).unwrap();
    }

    /// `AISMaterial D entry`: the own material's rank, or -1 with the command's warning.
    /// Example: `session.ais_material("0:2")`.
    pub fn ais_material(&self, entry: &str) -> i32 {
        println!("AISMaterial D {entry} ");
        let (_, presentation) = self.in_viewer(entry);
        if presentation.has_own_material().unwrap() {
            let rank = presentation.material().unwrap() as i32;
            println!("Material = {rank}\n{rank}");
            return rank;
        }
        println!("DPrsStd_AISMaterial: Warning : Material wasn't set\n-1");
        -1
    }

    /// `AISDefaultMaterial D entry`.
    /// Example: `session.ais_default_material("0:2");`.
    pub fn ais_default_material(&self, entry: &str) {
        println!("AISDefaultMaterial D {entry} ");
        let (label, presentation) = self.in_viewer(entry);
        presentation.unset_material().unwrap();
        TPrsStdAISViewer::update_label(&label).unwrap();
    }

    /// `AISHasOwnMaterial D entry`.
    /// Example: `session.ais_has_own_material("0:2")`.
    pub fn ais_has_own_material(&self, entry: &str) -> i32 {
        println!("AISHasOwnMaterial D {entry} ");
        let (_, presentation) = self.in_viewer(entry);
        answered(presentation.has_own_material().unwrap())
    }

    /// `AISTransparency D entry value`.
    /// Example: `session.ais_set_transparency("0:3", 0.9);`.
    pub fn ais_set_transparency(&self, entry: &str, value: f64) {
        println!("AISTransparency D {entry} {value} ");
        let (label, presentation) = self.in_viewer(entry);
        presentation.set_transparency(value).unwrap();
        TPrsStdAISViewer::update_label(&label).unwrap();
    }

    /// `AISTransparency D entry`: the own value, or -1 with the command's warning.
    /// Example: `session.ais_transparency("0:3")`.
    pub fn ais_transparency(&self, entry: &str) -> f64 {
        println!("AISTransparency D {entry} ");
        let (_, presentation) = self.in_viewer(entry);
        if presentation.has_own_transparency().unwrap() {
            let value = presentation.transparency().unwrap();
            let written = significant_17(value);
            println!("Transparency = {written}\n{written}");
            return value;
        }
        println!("DPrsStd_AISTransparency: Warning : Transparency wasn't set\n-1");
        -1.
    }

    /// `AISDefaultTransparency D entry`.
    /// Example: `session.ais_default_transparency("0:3");`.
    pub fn ais_default_transparency(&self, entry: &str) {
        println!("AISDefaultTransparency D {entry} ");
        let (label, presentation) = self.in_viewer(entry);
        presentation.unset_transparency().unwrap();
        TPrsStdAISViewer::update_label(&label).unwrap();
    }

    /// `AISHasOwnTransparency D entry`.
    /// Example: `session.ais_has_own_transparency("0:3")`.
    pub fn ais_has_own_transparency(&self, entry: &str) -> i32 {
        println!("AISHasOwnTransparency D {entry} ");
        let (_, presentation) = self.in_viewer(entry);
        answered(presentation.has_own_transparency().unwrap())
    }

    /// `AISSelMode D entry first [more...]`: sets the first mode and adds the others.
    /// Example: `session.ais_set_sel_mode("0:2", &[2, 4]);`.
    pub fn ais_set_sel_mode(&self, entry: &str, modes: &[i32]) {
        let written: Vec<String> = modes.iter().map(i32::to_string).collect();
        println!("AISSelMode D {entry} {} ", written.join(" "));
        let label = self.find_label(entry);
        let presentation = self.presentation(&label);
        presentation.set_selection_mode(modes[0], true).unwrap();
        for mode in &modes[1..] {
            presentation.add_selection_mode(*mode, true).unwrap();
        }
        TPrsStdAISViewer::update_label(&label).unwrap();
    }

    /// `AISSelMode D entry`: the selection modes, space separated; nothing without an own mode.
    /// Example: `session.ais_sel_mode("0:2") == "2 4"`.
    pub fn ais_sel_mode(&self, entry: &str) -> String {
        println!("AISSelMode D {entry} ");
        let presentation = self.presentation(&self.find_label(entry));
        let count = presentation.get_nb_selection_modes().unwrap();
        let modes: Vec<String> = (1..=count)
            .map(|index| presentation.selection_mode(index).unwrap().to_string())
            .collect();
        let answer = modes.join(" ");
        println!("{answer}");
        answer
    }

    /// `MartensiteSelectionMode D entry`: the first selection mode.
    /// Example: `session.selection_mode_query("0:2")`.
    pub fn selection_mode_query(&self, entry: &str) -> i32 {
        println!("MartensiteSelectionMode D {entry} ");
        let (_, presentation) = self.in_viewer(entry);
        let mode = presentation.selection_mode(1).unwrap();
        println!("{mode}");
        mode
    }

    /// `MartensiteSelectionMode D entry mode`.
    /// Example: `session.selection_mode_set("0:2", 1);`.
    pub fn selection_mode_set(&self, entry: &str, mode: i32) {
        println!("MartensiteSelectionMode D {entry} {mode} ");
        let (label, presentation) = self.in_viewer(entry);
        presentation.set_selection_mode(mode, true).unwrap();
        TPrsStdAISViewer::update_label(&label).unwrap();
    }

    /// `AISWidth D entry width`, the width as the script wrote it.
    /// Example: `session.ais_set_width("0:1:1", "1.0", 1.);`.
    pub fn ais_set_width(&self, entry: &str, written: &str, width: f64) {
        println!("AISWidth D {entry} {written} ");
        let (label, presentation) = self.in_viewer(entry);
        presentation.set_width(width).unwrap();
        TPrsStdAISViewer::update_label(&label).unwrap();
    }

    /// `AISWidth D entry`: the own width, or -4 with the command's warning.
    /// Example: `session.ais_width("0:1:1")`.
    pub fn ais_width(&self, entry: &str) -> f64 {
        println!("AISWidth D {entry} ");
        let (_, presentation) = self.in_viewer(entry);
        if presentation.has_own_width().unwrap() {
            let width = presentation.width().unwrap();
            println!("{}", significant_17(width));
            return width;
        }
        println!("AISWidth: Warning : Width wasn't set\n-4");
        -4.
    }
}

/// The attribute a label holds, if any; `presentation_on(&label).is_some()`.
fn presentation_on(label: &TdfLabel) -> Option<Handle<TPrsStdAISPresentation>> {
    let mut found = Handle::default();
    let has = label
        .find_attribute_as(&TPrsStdAISPresentation::get_id(), &mut found)
        .unwrap();
    has.then_some(found)
}

/// Writes a boolean the way the commands answer one, and answers it as the script reads it.
fn answered(flag: bool) -> i32 {
    let value = i32::from(flag);
    println!("{value}");
    value
}
