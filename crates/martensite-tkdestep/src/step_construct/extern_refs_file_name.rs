//! `StepConstructExternRefs::file_name`: the name of an external file, by its entities and the disk.
use super::extern_refs::{StepConstructExternRefs, WHERE, find_pdwad_and_exclude_excess, seq_item};
use crate::step_ap214::{
    StepAP214AppliedDocumentReference, StepAP214AppliedExternalIdentificationAssignment,
};
use crate::step_basic::{StepBasicDocumentFile, StepBasicProductDefinitionWithAssociatedDocuments};
use crate::step_data::{StepDataSelectMember, StepDataSelectNamed, StepDataSelectType};
use crate::step_entity::step_live;
use martensite_tkernel::osd::{CadPath, File};
use martensite_tkernel::standard::{Failure, Handle, StandardTransient, exceptions};
use martensite_tkernel::tcollection::HAsciiString;
use martensite_tkxsbase::interface::InterfaceGraph;

/// The text of a non-empty string, `None` for a null or an empty one.
fn non_empty_text(value: &Handle<HAsciiString>) -> Option<String> {
    let live = value.get().filter(|text| !text.is_empty())?;
    Some(String::from_utf8_lossy(live.string().as_bytes()).into_owned())
}

/// The name of a document of a PDWAD, for a reference of AP203.
fn name_in_pdwad(shape: &Handle<StandardTransient>) -> Result<String, Failure> {
    let pdwad = shape.cast::<StepBasicProductDefinitionWithAssociatedDocuments>();
    let Some(pdwad) = pdwad.get().filter(|live| live.doc_ids().is_some()) else {
        return Ok(String::new());
    };
    for i in 1..=pdwad.nb_doc_ids() {
        let doc = pdwad.doc_ids_value(i)?;
        let name = non_empty_text(&step_live(&doc, WHERE)?.name());
        if let Some(name) = name {
            return Ok(name);
        }
    }
    Ok(String::new())
}

/// The first AEIA among the entities sharing a document file, null without one.
fn aeia_of_file(
    graph: &InterfaceGraph,
    doc_file: &Handle<StepBasicDocumentFile>,
) -> Result<Handle<StepAP214AppliedExternalIdentificationAssignment>, Failure> {
    if doc_file.is_null() {
        return Ok(Handle::default());
    }
    for sharing in graph.sharings(&doc_file.cast())?.content() {
        let aeia = sharing.cast::<StepAP214AppliedExternalIdentificationAssignment>();
        if aeia.is_some() {
            return Ok(aeia);
        }
    }
    Ok(Handle::default())
}

/// The name an AEIA gives: its assigned id, or the string of its external source.
fn name_of_aeia(
    aeia: &StepAP214AppliedExternalIdentificationAssignment,
) -> Result<Option<String>, Failure> {
    let mut name = non_empty_text(&aeia.assigned_id());
    // ptv 29.01.2003 file trj4_xr1-tc-214.stp entity #71 have id "#71"
    if name.as_deref().is_some_and(|id| id.starts_with('#')) {
        name = None;
    }
    if name.is_some() {
        return Ok(name);
    }
    // try to take name from external source
    let source = aeia.source();
    let Some(source) = source.get() else {
        return Ok(None);
    };
    let source_id = source.source_id();
    if source_id.is_null() {
        return Ok(None);
    }
    let named = source_id.value().cast::<StepDataSelectNamed>();
    match named.get() {
        // nothing to do, hope could take name later.
        Some(named) if named.kind() == 6 => Ok(Some(
            String::from_utf8_lossy(named.string().as_bytes()).into_owned(),
        )),
        _ => Ok(None),
    }
}

/// The name a document file gives: its id, or else its name.
fn name_of_document_file(doc_file: &StepBasicDocumentFile) -> Option<String> {
    non_empty_text(&doc_file.id()).or_else(|| non_empty_text(&doc_file.name()))
}

/// `file path::absolute_path(dpath, name)` or the name itself: does that file exist?
fn file_exists(dpath: &str, name: &str) -> Result<bool, Failure> {
    let mut fullname = CadPath::absolute_path(dpath, name)?;
    if fullname.is_empty() {
        fullname = name.into();
    }
    Ok(File::new(fullname.as_bytes()).exists())
}

impl StepConstructExternRefs {
    /// The document file of an AP214 reference, by the long chain ADR->D<-DPE->PDF<-PDWAD->DF
    /// (PTV 28.01.2003 CAX-IF TRJ11, file ext_ref_master.stp): the last one found.
    fn document_file_by_chain(
        &self,
        num: i32,
        graph: &InterfaceGraph,
    ) -> Result<Handle<StepBasicDocumentFile>, Failure> {
        let adr: Handle<StepAP214AppliedDocumentReference> = seq_item(&self.my_aeias, num)?.cast();
        let mut pdwad = Handle::default();
        // we do not need to exclude, just find PDWAD
        find_pdwad_and_exclude_excess(&adr, &mut Vec::new(), graph, &mut pdwad)?;
        let mut doc_file = Handle::default();
        for candidate in Self::document_file_candidates(graph, &adr, &pdwad)? {
            let found = candidate.cast::<StepBasicDocumentFile>();
            if found.is_some() {
                doc_file = found;
            }
        }
        Ok(doc_file)
    }
    /// `file_name`: the file name of a reference; records in the transient process a failure when
    /// the file cannot be read, and a warning when the name of the AEIA was replaced.
    pub fn file_name(&self, num: i32) -> Result<String, Failure> {
        let hgraph = self.hgraph()?;
        let hgraph = step_live(&hgraph, WHERE)?;
        let graph = hgraph.graph();
        let in_doc_files = num <= self.my_doc_files.len() as i32;
        let doc_file: Handle<StepBasicDocumentFile> =
            if in_doc_files && seq_item(&self.my_doc_files, num)?.is_some() {
                seq_item(&self.my_doc_files, num)?.cast()
            } else if seq_item(&self.my_is_ap214, num)? == 1 {
                self.document_file_by_chain(num, &graph)?
            } else {
                return name_in_pdwad(&seq_item(&self.my_shapes, num)?);
            };
        // take name from AEIA and from DF
        let aeia = aeia_of_file(&graph, &doc_file)?;
        let name = match aeia.get() {
            Some(live) => name_of_aeia(live)?,
            None => None,
        };
        drop(graph);
        self.resolve_on_disk(name, &doc_file)
    }
    /// The directory of the loaded file.
    fn loaded_directory(&self) -> Result<String, Failure> {
        let ws = step_live(self.ws(), WHERE)?;
        let mut mainfile = CadPath::new(ws.loaded_file().as_bytes());
        mainfile.set_name("");
        mainfile.set_extension("");
        Ok(String::from_utf8_lossy(mainfile.system_name().as_bytes()).into_owned())
    }
    /// The last part of `file_name`: compute true path to the extern file.
    fn resolve_on_disk(
        &self,
        mut name: Option<String>,
        doc_file: &Handle<StepBasicDocumentFile>,
    ) -> Result<String, Failure> {
        let dpath = self.loaded_directory()?;
        let mut old_file_name = None;
        if let Some(current) = name.as_deref().filter(|text| !text.is_empty())
            && !file_exists(&dpath, current)?
        {
            old_file_name = name.take();
        }
        if name.as_deref().is_none_or(str::is_empty)
            && let Some(file) = doc_file.get()
        {
            // try to find name of the directory from DocFile
            name = name_of_document_file(file).or_else(|| old_file_name.clone());
            if name.is_none() {
                return Ok(String::new());
            }
        }
        // A null name here goes to the string constructor, which raises.
        let Some(mut name) = name else {
            return Err(exceptions::null_object(&format!(
                "{WHERE}::FileName: no name for the reference, expected one from an AEIA or a document file"
            )));
        };
        let process = || -> Result<_, Failure> {
            let ws = step_live(self.ws(), WHERE)?;
            Ok(ws.transfer_reader())
        };
        if !file_exists(&dpath, &name)? {
            if let Some(old) = &old_file_name {
                name.clone_from(old);
            }
            let reader = process()?;
            let tp = step_live(&reader, WHERE)?.transient_process();
            step_live(&tp, WHERE)?.add_fail_text(
                &doc_file.cast(),
                &format!("Can not read external file {name}"),
                "",
            );
        } else if let Some(old) = old_file_name.as_deref().filter(|old| *old != name) {
            let reader = process()?;
            let tp = step_live(&reader, WHERE)?.transient_process();
            step_live(&tp, WHERE)?.add_warning_text(
                &doc_file.cast(),
                &format!(
                    "External file with name from entity AEIA ({old}) not existed => use file name from DocumentFile entity - {name}"
                ),
                "",
            );
        }
        Ok(name)
    }
}
