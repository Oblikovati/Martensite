//! The writing part of `StepConstructExternRefs`: new references and their entities.
use super::extern_refs::{StepConstructExternRefs, WHERE, seq_item, text};
use crate::step_ap203::{
    StepAP203ApprovedItem, StepAP203CcDesignApproval, StepAP203CcDesignDateAndTimeAssignment,
    StepAP203CcDesignPersonAndOrganizationAssignment, StepAP203DateTimeItem,
    StepAP203PersonOrganizationItem,
};
use crate::step_ap214::{
    StepAP214AppliedDocumentReference, StepAP214AppliedExternalIdentificationAssignment,
    StepAP214DocumentReferenceItem, StepAP214ExternalIdentificationItem,
};
use crate::step_basic::{
    StepBasicDocument, StepBasicDocumentFile, StepBasicDocumentProductEquivalence,
    StepBasicDocumentRepresentationType, StepBasicDocumentType, StepBasicExternalSource,
    StepBasicIdentificationRole, StepBasicObjectRole, StepBasicProduct, StepBasicProductContext,
    StepBasicProductDefinition, StepBasicProductDefinitionContext,
    StepBasicProductDefinitionFormation, StepBasicProductDefinitionWithAssociatedDocuments,
    StepBasicProductOrFormationOrDefinition, StepBasicProductRelatedProductCategory,
    StepBasicRoleAssociation, StepBasicRoleSelect, StepBasicSourceItem,
};
use crate::step_data::{StepDataSelectMember, StepDataSelectNamed, StepDataSelectType};
use crate::step_entity::{HArray1, step_array1_item, step_array1_set, step_live, step_new_array1};
use crate::step_repr::{
    StepReprCharacterizedDefinition, StepReprDescriptiveRepresentationItem,
    StepReprNextAssemblyUsageOccurrence, StepReprProductDefinitionShape,
    StepReprPropertyDefinition, StepReprPropertyDefinitionRepresentation, StepReprRepresentation,
    StepReprRepresentationContext, StepReprRepresentationItem, StepReprRepresentedDefinition,
};
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkernel::tcollection::HAsciiString;
use martensite_tkxsbase::interface::{InterfaceInterfaceModel, InterfaceModelEntities};

/// An array of one handle, from 1.
fn single<T>(value: T) -> Result<Handle<HArray1<T>>, Failure>
where
    T: Default + Clone + Send + Sync + 'static,
{
    let array = step_new_array1::<T>(1, 1)?;
    step_array1_set(&array, 1, value, WHERE)?;
    Ok(array)
}

/// The AEIA of a document file, naming the file (PTV 30.01.2003 TRJ11: stored in the AEIA).
fn new_external_identification(
    filename: &str,
    doc_file: &Handle<StepBasicDocumentFile>,
) -> Result<Handle<StepAP214AppliedExternalIdentificationAssignment>, Failure> {
    // PTV 30.01.2003 TRJ11
    // - set the ("external document id and location", $) without unmeaning description
    let role = Handle::new(StepBasicIdentificationRole::new());
    role.set_name(&text("external document id and location"));
    let source_name = Handle::new(StepDataSelectNamed::default());
    source_name.set_string(filename.as_bytes());
    source_name.set_name("IDENTIFIER");
    let mut source_item = StepBasicSourceItem::default();
    source_item.set_value(&source_name.cast())?;
    let source = Handle::new(StepBasicExternalSource::new());
    source.init(&source_item);
    let mut item = StepAP214ExternalIdentificationItem::default();
    item.set_value(&doc_file.cast())?;
    let identification = Handle::new(StepAP214AppliedExternalIdentificationAssignment::new());
    identification.init(&text(filename), &role, &source, &single(item)?);
    Ok(identification)
}

/// The PDR defining the format of a document, for a format that is given.
fn new_format_representation(
    format: &str,
    doc_file: &Handle<StepBasicDocumentFile>,
) -> Result<Handle<StepReprPropertyDefinitionRepresentation>, Failure> {
    let context = Handle::new(StepReprRepresentationContext::new());
    context.init(&text(""), &text("document parameters"));
    let description = Handle::new(StepReprDescriptiveRepresentationItem::new());
    description.init(&text("data format"), &text(format));
    let representation = Handle::new(StepReprRepresentation::new());
    let items = single::<Handle<StepReprRepresentationItem>>(description.cast())?;
    representation.init(&text("document format"), &items, &context);
    let mut characterized = StepReprCharacterizedDefinition::default();
    characterized.set_value(&doc_file.cast())?;
    let property = Handle::new(StepReprPropertyDefinition::new());
    property.init(&text("document property"), true, &text(""), &characterized);
    let mut represented = StepReprRepresentedDefinition::default();
    represented.set_value(&property.cast())?;
    let pdr = Handle::new(StepReprPropertyDefinitionRepresentation::new());
    pdr.init(&represented, &representation);
    Ok(pdr)
}

/// A select equal to `product_definition` is replaced by `pdwad`, in place, in one array.
fn replace_in_items<S>(
    items: &Handle<HArray1<S>>,
    product_definition_of: impl Fn(&S) -> Handle<StepBasicProductDefinition>,
    model: &Handle<dyn InterfaceInterfaceModel>,
    num_prod_def: i32,
    pdwad: &Handle<StepBasicProductDefinitionWithAssociatedDocuments>,
) -> Result<(), Failure>
where
    S: StepDataSelectType + Clone + Send + Sync + 'static,
{
    let live = step_live(items, WHERE)?;
    let length = live.array1().length();
    for i in 1..=length as i32 {
        let mut item = step_array1_item(items, i, WHERE)?;
        if model.number(&product_definition_of(&item).cast()) == num_prod_def {
            item.set_value(&pdwad.cast())?;
            step_array1_set(items, i, item, WHERE)?;
        }
    }
    Ok(())
}

impl StepConstructExternRefs {
    /// The reference of a schema other than AP203.
    fn add_ap214_reference(
        &mut self,
        filename: &str,
        pd: &Handle<StepBasicProductDefinition>,
        format: &str,
    ) -> Result<(), Failure> {
        let empty = text("");
        // create core entity DocumentFile
        let doc_type = Handle::new(StepBasicDocumentType::new());
        doc_type.init(&empty);
        // PTV 30.01.2003 TRJ11 - copy external filename as is
        let doc_file = Handle::new(StepBasicDocumentFile::new());
        doc_file.init(
            &text(filename),
            &empty,
            false,
            &empty,
            &doc_type,
            &empty,
            false,
            &empty,
        );
        let identification = new_external_identification(filename, &doc_file)?;
        // create DocumentRepresentationType
        let representation_type = Handle::new(StepBasicDocumentRepresentationType::new());
        representation_type.init(&text("digital"), &doc_file.cast());
        // create AppliedDocumentReference,
        let adr = Handle::new(StepAP214AppliedDocumentReference::new());
        // PTV 30.01.2003 TRJ11 - create additional entities for AP214
        self.add_ap214_exter_ref(&adr, pd, &doc_file, filename)?;
        // create RoleAssociation etc.
        let object_role = Handle::new(StepBasicObjectRole::new());
        object_role.init(&text("mandatory"), false, &empty);
        let mut role_select = StepBasicRoleSelect::default();
        role_select.set_value(&adr.cast())?;
        let role = Handle::new(StepBasicRoleAssociation::new());
        role.init(&object_role, &role_select);
        // create PDR for definition of document format (if defined)
        let pdr_format = if format.is_empty() {
            Handle::default()
        } else {
            new_format_representation(format, &doc_file)?
        };
        // add all the created root entities to sequences
        self.my_aeias.push(identification.cast());
        self.my_roles.push(role.cast());
        self.my_formats.push(pdr_format.cast());
        self.my_shapes.push(pd.cast());
        self.my_types.push(representation_type.cast());
        Ok(())
    }
    /// Replaces the references to `pd` by `pdwad` in the entities of AP203 sharing it, and in its
    /// PDS and NAUOs.
    fn replace_by_pdwad(
        &self,
        pd: &Handle<StepBasicProductDefinition>,
        pdwad: &Handle<StepBasicProductDefinitionWithAssociatedDocuments>,
        num_prod_def: i32,
    ) -> Result<(), Failure> {
        let model = self.tool.model()?;
        let hgraph = self.hgraph()?;
        let hgraph = step_live(&hgraph, WHERE)?;
        let graph = hgraph.graph();
        for sub in graph.sharings(&pd.cast())?.content() {
            if let Some(shape) = sub.cast::<StepReprProductDefinitionShape>().get() {
                let mut characterized = StepReprCharacterizedDefinition::default();
                characterized.set_value(&pdwad.cast())?;
                shape.set_definition(&characterized);
            }
        }
        for sub in graph.sharings(&pd.cast())?.content() {
            if let Some(nauo) = sub.cast::<StepReprNextAssemblyUsageOccurrence>().get() {
                nauo.set_related_product_definition(&pdwad.cast());
            }
            if let Some(assignment) = sub
                .cast::<StepAP203CcDesignPersonAndOrganizationAssignment>()
                .get()
            {
                replace_in_items(
                    &assignment.items(),
                    StepAP203PersonOrganizationItem::product_definition,
                    &model,
                    num_prod_def,
                    pdwad,
                )?;
            }
            if let Some(assignment) = sub.cast::<StepAP203CcDesignDateAndTimeAssignment>().get() {
                replace_in_items(
                    &assignment.items(),
                    StepAP203DateTimeItem::product_definition,
                    &model,
                    num_prod_def,
                    pdwad,
                )?;
            }
            if let Some(approval) = sub.cast::<StepAP203CcDesignApproval>().get() {
                replace_in_items(
                    &approval.items(),
                    StepAP203ApprovedItem::product_definition,
                    &model,
                    num_prod_def,
                    pdwad,
                )?;
            }
        }
        Ok(())
    }
    /// The reference of AP203: a document, and a PDWAD replacing the product definition.
    fn add_ap203_reference(
        &mut self,
        filename: &str,
        pd: &Handle<StepBasicProductDefinition>,
    ) -> Result<(), Failure> {
        let empty = text("");
        let definition = step_live(pd, WHERE)?;
        // create document
        let doc_type = Handle::new(StepBasicDocumentType::new());
        doc_type.init(&text("cad_filename"));
        let document = Handle::new(StepBasicDocument::new());
        document.init(
            &empty,
            &text(filename),
            true,
            &text("CAD Model associated to the part"),
            &doc_type,
        );
        // create ProductDefinitionWithAssociatedDocuments
        let pdwad = Handle::new(StepBasicProductDefinitionWithAssociatedDocuments::new());
        pdwad.init(
            &empty,
            &definition.description(),
            &definition.formation(),
            &definition.frame_of_reference(),
            &single(document.clone())?,
        );
        let num_prod_def = self.tool.model()?.number(&pd.cast());
        self.replace_by_pdwad(pd, &pdwad, num_prod_def)?;
        self.my_aeias.push(pdwad.cast());
        self.my_replace_num.push(num_prod_def);
        self.my_roles.push(document.cast());
        self.my_types.push(doc_type.cast());
        Ok(())
    }
    /// `add_extern_ref`: a new external reference to `filename` attached to `pd`; `format` may be
    /// empty, when it is not written, else "STEP AP214" or "STEP AP203". Returns the index of the
    /// new reference.
    pub fn add_extern_ref(
        &mut self,
        filename: &str,
        pd: &Handle<StepBasicProductDefinition>,
        format: &str,
    ) -> Result<i32, Failure> {
        if format.contains("203") {
            self.add_ap203_reference(filename, pd)?;
        } else {
            self.add_ap214_reference(filename, pd, format)?;
        }
        Ok(self.my_aeias.len() as i32)
    }
    /// `add_with_refs` of one optional root.
    fn add_root(&self, root: &Handle<StandardTransient>) -> Result<(), Failure> {
        if root.is_some() {
            self.tool.model()?.add_with_refs(root, 0, false)?;
        }
        Ok(())
    }
    /// `write_extern_refs`: adds the references to the model, with the schema `num` (3 is AP203);
    /// returns their number.
    pub fn write_extern_refs(&self, num: i32) -> Result<i32, Failure> {
        let model = self.tool.model()?;
        for i in 1..=self.my_aeias.len() as i32 {
            if num == 3 {
                model.replace_entity(
                    seq_item(&self.my_replace_num, i)?,
                    &seq_item(&self.my_aeias, i)?,
                )?;
            } else {
                model.add_with_refs(&seq_item(&self.my_aeias, i)?, 0, false)?;
            }
            self.add_root(&seq_item(&self.my_roles, i)?)?;
            if num != 3 {
                self.add_root(&seq_item(&self.my_formats, i)?)?;
                self.add_root(&seq_item(&self.my_shapes, i)?)?;
            }
            self.add_root(&seq_item(&self.my_types, i)?)?;
        }
        // PTV 30.01.2003 TRJ11
        if self.my_apd.is_some() {
            model.add_with_refs(&self.my_apd.cast(), 0, false)?;
        }
        if self.my_shared_prpc.is_some() {
            model.add_with_refs(&self.my_shared_prpc.cast(), 0, false)?;
        }
        Ok(self.my_aeias.len() as i32)
    }
    /// `addAP214ExterRef`: a new structure of additional entities and the references to it. Do not
    /// refer from ADR to DF directly in AP214 (TRJ11).
    pub fn add_ap214_exter_ref(
        &mut self,
        adr: &Handle<StepAP214AppliedDocumentReference>,
        pd: &Handle<StepBasicProductDefinition>,
        df: &Handle<StepBasicDocumentFile>,
        filename: &str,
    ) -> Result<bool, Failure> {
        let mut reference_item = StepAP214DocumentReferenceItem::default();
        reference_item.set_value(&pd.cast())?;
        let empty = text("");
        // create/get created shared entities:
        // DocumentType, ProductDefinitionContext, ProductRelatedProductCategory, ProductContext
        self.check_ap214_shared()?;
        // create document
        let document = Handle::new(StepBasicDocument::new());
        document.init(&empty, &empty, false, &empty, &self.my_shared_doc_type);
        step_live(adr, WHERE)?.init(&document, &empty, &single(reference_item)?);
        // create new product
        let prpc = step_live(&self.my_shared_prpc, WHERE)?;
        let old_products = prpc.products();
        let nb_products = old_products
            .get()
            .map_or(0, |products| products.array1().length() as i32);
        let product = Handle::new(StepBasicProduct::new());
        let product_id = Handle::new(HAsciiString::from_integer(20001 + nb_products));
        let product_name = text(&format!("{filename}-Doc"));
        product.init(
            &product_id,
            &product_name,
            &empty,
            &single(self.my_shared_pc.clone())?,
        );
        // create new product definition formation
        let formation = Handle::new(StepBasicProductDefinitionFormation::new());
        // name id taken from example ext_ref_master.stp
        formation.init(&text("1"), &empty, &product);
        let equivalence = Handle::new(StepBasicDocumentProductEquivalence::new());
        let mut equivalent = StepBasicProductOrFormationOrDefinition::default();
        equivalent.set_value(&formation.cast())?;
        equivalence.init(&text("equivalence"), false, &empty, &document, &equivalent);
        // add to the model with references
        let model = self.tool.model()?;
        model.add_with_refs(&equivalence.cast(), 0, false)?;
        // add products to shared PRPC
        let products = step_new_array1::<Handle<StepBasicProduct>>(1, nb_products + 1)?;
        for index in 1..=nb_products {
            step_array1_set(
                &products,
                index,
                step_array1_item(&old_products, index, WHERE)?,
                WHERE,
            )?;
        }
        step_array1_set(&products, nb_products + 1, product, WHERE)?;
        // set the hArray to the PRPC
        prpc.set_products(&products);
        // create new PDWAD
        let pdwad = Handle::new(StepBasicProductDefinitionWithAssociatedDocuments::new());
        let documents = single::<Handle<StepBasicDocument>>(df.cast())?;
        pdwad.init(
            &text("1"),
            &empty,
            &formation,
            &self.my_shared_pdc,
            &documents,
        );
        // add to the model with references
        model.add_with_refs(&pdwad.cast(), 0, false)?;
        Ok(true)
    }
    /// `checkAP214Shared`: checks, creating it when null, all shared entities for the model.
    pub fn check_ap214_shared(&mut self) -> Result<(), Failure> {
        let empty = text("");
        if self.my_shared_prpc.is_null() {
            // create new ProductRelatedProductCategory for all extern files.
            let prpc = Handle::new(StepBasicProductRelatedProductCategory::new());
            prpc.init(&text("document"), false, &empty, &Handle::default());
            self.my_shared_prpc = prpc;
        }
        if self.my_shared_doc_type.is_null() {
            // create new shared Document Type
            let doc_type = Handle::new(StepBasicDocumentType::new());
            doc_type.init(&text("configuration controlled document version"));
            self.my_shared_doc_type = doc_type;
        }
        if self.my_shared_pdc.is_null() {
            // create new shared Product Definition Context
            let application = step_live(&self.get_ap214_apd()?, WHERE)?.application();
            let pdc = Handle::new(StepBasicProductDefinitionContext::new());
            pdc.init(&text("digital document definition"), &application, &empty);
            self.my_shared_pdc = pdc;
        }
        if self.my_shared_pc.is_null() {
            // create new shared ProductContext
            let application = step_live(&self.get_ap214_apd()?, WHERE)?.application();
            let pc = Handle::new(StepBasicProductContext::new());
            pc.init(&empty, &application, &empty);
            self.my_shared_pc = pc;
        }
        Ok(())
    }
}
