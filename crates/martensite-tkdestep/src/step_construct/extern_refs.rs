//! `StepConstructExternRefs`: analyzes (reading) and creates (writing) references to external files
//! in STEP, as sequences of the relevant root entities.
use super::StepConstructTool;
use crate::step_ap214::StepAP214AppliedDocumentReference;
use crate::step_basic::{
    StepBasicApplicationContext, StepBasicApplicationProtocolDefinition, StepBasicDocument,
    StepBasicDocumentFile, StepBasicDocumentProductEquivalence,
    StepBasicDocumentRepresentationType, StepBasicDocumentType, StepBasicProductContext,
    StepBasicProductDefinition, StepBasicProductDefinitionContext,
    StepBasicProductDefinitionFormation, StepBasicProductDefinitionWithAssociatedDocuments,
    StepBasicProductRelatedProductCategory, StepBasicRoleAssociation,
};
use crate::step_entity::step_live;
use crate::step_repr::{
    StepReprDescriptiveRepresentationItem, StepReprPropertyDefinition,
    StepReprPropertyDefinitionRepresentation,
};
use crate::step_shape::StepShapeShapeRepresentation;
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped, StandardTransient, exceptions};
use martensite_tkernel::tcollection::HAsciiString;
use martensite_tkxsbase::interface::{InterfaceGraph, InterfaceHGraph, InterfaceModelEntities};
use martensite_tkxsbase::xs_control::XSControlWorkSession;
use std::ops::Deref;

pub(super) const WHERE: &str = "StepConstructExternRefs";

pub(super) fn text(value: &str) -> Handle<HAsciiString> {
    Handle::new(HAsciiString::new(value))
}

/// `sequence(num)`: an out of range failure out of 1..Length.
pub(super) fn seq_item<T: Clone>(sequence: &[T], num: i32) -> Result<T, Failure> {
    usize::try_from(num - 1)
        .ok()
        .and_then(|index| sequence.get(index))
        .cloned()
        .ok_or_else(|| {
            exceptions::out_of_range(&format!(
                "{WHERE}: index {num} out of the sequence, expected 1..{}",
                sequence.len()
            ))
        })
}

/// The exact type test of the `dynamic_type() == STANDARD_TYPE(T)`.
fn is_exactly<T: RuntimeTyped>(entity: &Handle<StandardTransient>) -> bool {
    entity
        .dynamic_type()
        .is_some_and(|kind| std::ptr::eq(kind, T::static_type()))
}

/// The PDWAD found through a DPE of a document of the ADR (ADR->D<-DPE->PDF<-PDWAD): removes it from
/// `seq_of_pdwad`, and true, when it is in that sequence. Called with an empty one too, just to find
/// the PDWAD by ADR.
pub(super) fn find_pdwad_and_exclude_excess(
    adr: &Handle<StepAP214AppliedDocumentReference>,
    seq_of_pdwad: &mut Vec<Handle<StandardTransient>>,
    graph: &InterfaceGraph,
    pdwad: &mut Handle<StepBasicProductDefinitionWithAssociatedDocuments>,
) -> Result<bool, Failure> {
    for doc in graph.shareds(&adr.cast())?.content() {
        if doc.cast::<StepBasicDocument>().is_null() {
            continue;
        }
        // looking for Document Product Equivalence
        for dpe in graph.sharings(&doc)?.content() {
            if dpe.cast::<StepBasicDocumentProductEquivalence>().is_null() {
                continue;
            }
            // take PDF and search the same PDF by PDWAD chain
            for pdf in graph.shareds(&dpe)?.content() {
                if pdf.cast::<StepBasicProductDefinitionFormation>().is_null() {
                    continue;
                }
                if exclude_pdwad_of_formation(&pdf, seq_of_pdwad, graph, pdwad)? {
                    return Ok(true);
                }
            }
        }
    }
    Ok(false)
}

/// One formation of `find_pdwad_and_exclude_excess`: the last PDWAD sharing it stays in `pdwad`.
fn exclude_pdwad_of_formation(
    pdf: &Handle<StandardTransient>,
    seq_of_pdwad: &mut Vec<Handle<StandardTransient>>,
    graph: &InterfaceGraph,
    pdwad: &mut Handle<StepBasicProductDefinitionWithAssociatedDocuments>,
) -> Result<bool, Failure> {
    for sharing in graph.sharings(pdf)?.content() {
        let found = sharing.cast::<StepBasicProductDefinitionWithAssociatedDocuments>();
        if found.is_some() {
            *pdwad = found;
        }
    }
    // now searching for PDWAD that refer to the same PDF
    let same = seq_of_pdwad.iter().position(|item| {
        let current = item.cast::<StepBasicProductDefinitionWithAssociatedDocuments>();
        current.is_some() && current == *pdwad
    });
    let Some(position) = same else {
        return Ok(false);
    };
    // found the same Product Definition Formation
    seq_of_pdwad.remove(position);
    Ok(true)
}

/// ```
/// use martensite_tkdestep::step_construct::StepConstructExternRefs;
/// let mut refs = StepConstructExternRefs::new();
/// assert_eq!(refs.nb_extern_refs(), 0);
/// assert_eq!(refs.get_ap214_apd().unwrap().status().string().as_bytes(), b"version 1.1");
/// assert!(refs.prod_def(1).is_err());
/// ```
#[derive(Clone, Default)]
pub struct StepConstructExternRefs {
    pub(super) tool: StepConstructTool,
    pub(super) my_aeias: Vec<Handle<StandardTransient>>,
    pub(super) my_roles: Vec<Handle<StandardTransient>>,
    pub(super) my_formats: Vec<Handle<StandardTransient>>,
    pub(super) my_shapes: Vec<Handle<StandardTransient>>,
    pub(super) my_types: Vec<Handle<StandardTransient>>,
    pub(super) my_is_ap214: Vec<i32>,
    pub(super) my_replace_num: Vec<i32>,
    pub(super) my_doc_files: Vec<Handle<StandardTransient>>,
    pub(super) my_shared_prpc: Handle<StepBasicProductRelatedProductCategory>,
    pub(super) my_shared_doc_type: Handle<StepBasicDocumentType>,
    pub(super) my_shared_pdc: Handle<StepBasicProductDefinitionContext>,
    pub(super) my_shared_pc: Handle<StepBasicProductContext>,
    pub(super) my_apd: Handle<StepBasicApplicationProtocolDefinition>,
}

impl Deref for StepConstructExternRefs {
    type Target = StepConstructTool;
    fn deref(&self) -> &StepConstructTool {
        &self.tool
    }
}

/// What `load_extern_refs` collects around one DocumentFile.
#[derive(Default)]
struct DocumentFileData {
    format: Handle<StepReprPropertyDefinitionRepresentation>,
    doc_type: Handle<StepBasicDocumentRepresentationType>,
}

impl DocumentFileData {
    fn complete(&self) -> bool {
        self.doc_type.is_some() && self.format.is_some()
    }
    /// The format and the type of a DocumentFile, from the entities sharing it.
    fn collect(
        &mut self,
        graph: &InterfaceGraph,
        file: &Handle<StandardTransient>,
    ) -> Result<(), Failure> {
        for sub in graph.sharings(file)?.content() {
            // PDRs of a shape and of a file format
            if sub.cast::<StepReprPropertyDefinition>().is_some() {
                self.collect_format(graph, &sub)?;
            }
            // DocumentRepresentationType
            let doc_type = sub.cast::<StepBasicDocumentRepresentationType>();
            if doc_type.is_some() {
                self.doc_type = doc_type;
            }
            if self.complete() {
                break;
            }
        }
        Ok(())
    }
    fn collect_format(
        &mut self,
        graph: &InterfaceGraph,
        definition: &Handle<StandardTransient>,
    ) -> Result<(), Failure> {
        for sharing in graph.sharings(definition)?.content() {
            let pdr = sharing.cast::<StepReprPropertyDefinitionRepresentation>();
            let Some(live) = pdr.get() else {
                continue;
            };
            let used = live.used_representation();
            step_live(&used, WHERE)?;
            if used.cast::<StepShapeShapeRepresentation>().is_some() {
                self.format = pdr;
            }
        }
        Ok(())
    }
}

impl StepConstructExternRefs {
    /// `StepConstructExternRefs()`: an empty tool.
    pub fn new() -> Self {
        Self::default()
    }
    /// `StepConstructExternRefs(WS)`: a tool loaded with a session.
    pub fn with_ws(ws: &Handle<XSControlWorkSession>) -> Result<Self, Failure> {
        Ok(Self {
            tool: StepConstructTool::with_ws(ws)?,
            ..Self::default()
        })
    }
    /// `init`: true on success.
    pub fn init(&mut self, ws: &Handle<XSControlWorkSession>) -> Result<bool, Failure> {
        self.clear();
        self.tool.set_ws(ws)
    }
    /// `clear`: the list of defined extern refs; the replacement numbers stay.
    pub fn clear(&mut self) {
        self.my_aeias.clear();
        self.my_roles.clear();
        self.my_formats.clear();
        self.my_shapes.clear();
        self.my_types.clear();
        self.my_is_ap214.clear();
        // PTV 30.01.2003 TRJ11
        self.my_doc_files.clear();
        self.my_shared_prpc = Handle::default();
        self.my_shared_doc_type = Handle::default();
        self.my_shared_pdc = Handle::default();
        self.my_shared_pc = Handle::default();
        self.my_apd = Handle::default();
    }
    /// The graph of the tool, read through its handle.
    pub(super) fn hgraph(&self) -> Result<Handle<InterfaceHGraph>, Failure> {
        self.tool.graph(false)
    }
    /// The entities a DocumentFile is searched in: of the PDWAD, or of the ADR when there is none
    /// (which shouldn't begin from TRJ11).
    pub(super) fn document_file_candidates(
        graph: &InterfaceGraph,
        adr: &Handle<StepAP214AppliedDocumentReference>,
        pdwad: &Handle<StepBasicProductDefinitionWithAssociatedDocuments>,
    ) -> Result<Vec<Handle<StandardTransient>>, Failure> {
        if pdwad.is_null() {
            return Ok(graph.shareds(&adr.cast())?.content());
        }
        Ok(graph.shareds(&pdwad.cast())?.content())
    }
    /// One ADR of `load_extern_refs`: its reference is recorded when it has a DocumentFile.
    fn load_document_reference(
        &mut self,
        graph: &InterfaceGraph,
        adr: &Handle<StepAP214AppliedDocumentReference>,
        seq_of_pdwad: &mut Vec<Handle<StandardTransient>>,
    ) -> Result<(), Failure> {
        // looking for Product Definition Formation and exclude excess PDWAD from aSeqOfPDWAD
        let mut pdwad = Handle::default();
        find_pdwad_and_exclude_excess(adr, seq_of_pdwad, graph, &mut pdwad)?;
        // AppliedDocumentReference with RoleAssociation...
        let mut role = Handle::<StepBasicRoleAssociation>::default();
        for sharing in graph.sharings(&adr.cast())?.content() {
            let found = sharing.cast::<StepBasicRoleAssociation>();
            if found.is_some() {
                role = found;
            }
        }
        let mut shape = Handle::<StepBasicProductDefinition>::default();
        for shared in graph.shareds(&adr.cast())?.content() {
            let found = shared.cast::<StepBasicProductDefinition>();
            if found.is_some() {
                shape = found;
            }
        }
        // search for Document file
        let mut doc_file = Handle::<StandardTransient>::default();
        let mut file_data = DocumentFileData::default();
        for candidate in Self::document_file_candidates(graph, adr, &pdwad)? {
            if candidate.cast::<StepBasicDocumentFile>().is_null() {
                continue;
            }
            doc_file = candidate;
            // for each DocumentFile, find associated with it data:
            file_data.collect(graph, &doc_file)?;
            if file_data.complete() {
                break;
            }
        }
        if doc_file.is_null() {
            return Ok(());
        }
        self.my_aeias.push(adr.cast());
        self.my_roles.push(role.cast());
        self.my_formats.push(file_data.format.cast());
        self.my_shapes.push(shape.cast());
        self.my_types.push(file_data.doc_type.cast());
        self.my_is_ap214.push(1);
        self.my_doc_files.push(doc_file);
        Ok(())
    }
    /// `load_extern_refs`: loads the external references of the model, without clearing first; true
    /// when any is defined.
    pub fn load_extern_refs(&mut self) -> Result<bool, Failure> {
        // iterate on entities in the model and find AEIAs or PDWADs (for AP203)
        let model = self.tool.model()?;
        // PTV 28.01.2003 CAX-IF TRJ11, file ext_ref_master.stp
        // search all ADR and PDWAD and exclude excess PDWADs
        let (mut seq_of_adr, mut seq_of_pdwad) = (Vec::new(), Vec::new());
        for ient in 1..=model.nb_entities() {
            let entity = model.value(ient)?;
            if is_exactly::<StepBasicProductDefinitionWithAssociatedDocuments>(&entity) {
                seq_of_pdwad.push(entity);
            } else if is_exactly::<StepAP214AppliedDocumentReference>(&entity) {
                seq_of_adr.push(entity);
            }
        }
        // run on sequence of ADR and remove excess PDWAD from its sequence
        let hgraph = self.hgraph()?;
        let hgraph = step_live(&hgraph, WHERE)?;
        let graph = hgraph.graph();
        for adr in &seq_of_adr {
            self.load_document_reference(&graph, &adr.cast(), &mut seq_of_pdwad)?;
        }
        // now iterates on sequence of Product Definition With Associated Documents
        for pdwad in seq_of_pdwad {
            self.my_shapes.push(pdwad);
            self.my_is_ap214.push(0);
            self.my_aeias.push(Handle::default());
            self.my_roles.push(Handle::default());
            self.my_formats.push(Handle::default());
            self.my_types.push(Handle::default());
            self.my_doc_files.push(Handle::default());
        }
        Ok(!self.my_shapes.is_empty())
    }
    /// `nb_extern_refs`.
    pub fn nb_extern_refs(&self) -> i32 {
        self.my_shapes.len() as i32
    }
    /// `prod_def`: the product definition of a reference, null when it cannot be detected.
    pub fn prod_def(&self, num: i32) -> Result<Handle<StepBasicProductDefinition>, Failure> {
        Ok(seq_item(&self.my_shapes, num)?.cast())
    }
    /// `doc_file`: the document file of a reference, null when it cannot be detected.
    pub fn doc_file(&self, num: i32) -> Result<Handle<StepBasicDocumentFile>, Failure> {
        Ok(seq_item(&self.my_doc_files, num)?.cast())
    }
    /// `format`: the format identification of a document, null when not defined.
    pub fn format(&self, num: i32) -> Result<Handle<HAsciiString>, Failure> {
        if seq_item(&self.my_is_ap214, num)? == 0 {
            return Ok(Handle::default());
        }
        let pdr =
            seq_item(&self.my_formats, num)?.cast::<StepReprPropertyDefinitionRepresentation>();
        let Some(pdr) = pdr.get() else {
            return Ok(Handle::default());
        };
        let rep = pdr.used_representation();
        let rep = step_live(&rep, WHERE)?;
        for i in 1..=rep.nb_items() {
            let item = rep.items_value(i)?;
            step_live(&item, WHERE)?;
            if let Some(description) = item.cast::<StepReprDescriptiveRepresentationItem>().get() {
                return Ok(description.description());
            }
        }
        Ok(Handle::default())
    }
    /// `set_ap214_apd`: the ApplicationProtocolDefinition of the PDM schema.
    pub fn set_ap214_apd(&mut self, apd: &Handle<StepBasicApplicationProtocolDefinition>) {
        self.my_apd = apd.clone();
    }
    /// `get_ap214_apd`: the ApplicationProtocolDefinition of the PDM schema, a new one with a new
    /// application context when not defined.
    pub fn get_ap214_apd(
        &mut self,
    ) -> Result<Handle<StepBasicApplicationProtocolDefinition>, Failure> {
        if self.my_apd.is_null() {
            // examples of the values taken from ext_ref_master.stp
            let application = Handle::new(StepBasicApplicationContext::new());
            application.init(&text(""));
            let apd = Handle::new(StepBasicApplicationProtocolDefinition::new());
            apd.init(
                &text("version 1.1"),
                &text("pdm_schema"),
                1999,
                &application,
            );
            self.my_apd = apd;
        }
        Ok(self.my_apd.clone())
    }
}
