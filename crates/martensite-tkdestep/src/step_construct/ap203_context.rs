//! `StepConstructAP203Context`: the management data AP203 requires (persons, dates, security,
//! approval) for each part and assembly link.
use super::StepConstructPart;
use super::ap203_defaults::{new_date_and_time, new_person_and_organization};
use crate::step_ap203::{
    StepAP203ApprovedItem, StepAP203CcDesignApproval, StepAP203CcDesignDateAndTimeAssignment,
    StepAP203CcDesignPersonAndOrganizationAssignment, StepAP203CcDesignSecurityClassification,
    StepAP203ClassifiedItem, StepAP203DateTimeItem, StepAP203PersonOrganizationItem,
};
use crate::step_basic::{
    StepBasicApproval, StepBasicApprovalDateTime, StepBasicApprovalPersonOrganization,
    StepBasicApprovalRole, StepBasicApprovalStatus, StepBasicDateAndTime, StepBasicDateTimeRole,
    StepBasicDateTimeSelect, StepBasicPersonAndOrganization, StepBasicPersonAndOrganizationRole,
    StepBasicPersonOrganizationSelect, StepBasicProductCategory,
    StepBasicProductCategoryRelationship, StepBasicSecurityClassification,
    StepBasicSecurityClassificationLevel,
};
use crate::step_data::StepDataSelectType;
use crate::step_entity::{HArray1, step_array1_set, step_live, step_new_array1};
use crate::step_repr::StepReprNextAssemblyUsageOccurrence;
use crate::step_shape::StepShapeShapeDefinitionRepresentation;
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkernel::tcollection::HAsciiString;

const WHERE: &str = "StepConstructAP203Context";

fn text(value: &str) -> Handle<HAsciiString> {
    Handle::new(HAsciiString::new(value))
}

/// An array of select values, from 1, each set from one entity.
fn select_items<S>(values: &[Handle<StandardTransient>]) -> Result<Handle<HArray1<S>>, Failure>
where
    S: StepDataSelectType + Default + Clone + Send + Sync + 'static,
{
    let items = step_new_array1::<S>(1, values.len() as i32)?;
    for (index, value) in values.iter().enumerate() {
        let mut item = S::default();
        item.set_value(value)?;
        step_array1_set(&items, index as i32 + 1, item, WHERE)?;
    }
    Ok(items)
}

/// A person role with a name.
fn person_role(name: &str) -> Handle<StepBasicPersonAndOrganizationRole> {
    let role = Handle::new(StepBasicPersonAndOrganizationRole::new());
    role.init(&text(name));
    role
}

/// A date role with a name.
fn date_role(name: &str) -> Handle<StepBasicDateTimeRole> {
    let role = Handle::new(StepBasicDateTimeRole::new());
    role.init(&text(name));
    role
}

/// ```
/// use martensite_tkdestep::step_construct::StepConstructAP203Context;
/// let mut context = StepConstructAP203Context::new();
/// assert_eq!(context.role_creator().name().string().as_bytes(), b"creator");
/// assert_eq!(context.default_approval().unwrap().status().name().string().as_bytes(), b"not_yet_approved");
/// assert!(context.get_creator().is_null());
/// ```
#[derive(Clone)]
pub struct StepConstructAP203Context {
    def_approval: Handle<StepBasicApproval>,
    def_date_and_time: Handle<StepBasicDateAndTime>,
    def_person_and_organization: Handle<StepBasicPersonAndOrganization>,
    def_security_classification_level: Handle<StepBasicSecurityClassificationLevel>,
    role_creator: Handle<StepBasicPersonAndOrganizationRole>,
    role_design_owner: Handle<StepBasicPersonAndOrganizationRole>,
    role_design_supplier: Handle<StepBasicPersonAndOrganizationRole>,
    role_classification_officer: Handle<StepBasicPersonAndOrganizationRole>,
    role_creation_date: Handle<StepBasicDateTimeRole>,
    role_classification_date: Handle<StepBasicDateTimeRole>,
    role_approver: Handle<StepBasicApprovalRole>,
    my_creator: Handle<StepAP203CcDesignPersonAndOrganizationAssignment>,
    my_design_owner: Handle<StepAP203CcDesignPersonAndOrganizationAssignment>,
    my_design_supplier: Handle<StepAP203CcDesignPersonAndOrganizationAssignment>,
    my_classification_officer: Handle<StepAP203CcDesignPersonAndOrganizationAssignment>,
    my_security: Handle<StepAP203CcDesignSecurityClassification>,
    my_creation_date: Handle<StepAP203CcDesignDateAndTimeAssignment>,
    my_classification_date: Handle<StepAP203CcDesignDateAndTimeAssignment>,
    my_approval: Handle<StepAP203CcDesignApproval>,
    my_approver: Handle<StepBasicApprovalPersonOrganization>,
    my_approval_date_time: Handle<StepBasicApprovalDateTime>,
    my_product_category_relationship: Handle<StepBasicProductCategoryRelationship>,
}

impl Default for StepConstructAP203Context {
    fn default() -> Self {
        let approver = Handle::new(StepBasicApprovalRole::new());
        approver.init(&text("approver"));
        Self {
            def_approval: Handle::default(),
            def_date_and_time: Handle::default(),
            def_person_and_organization: Handle::default(),
            def_security_classification_level: Handle::default(),
            role_creator: person_role("creator"),
            role_design_owner: person_role("design_owner"),
            role_design_supplier: person_role("design_supplier"),
            role_classification_officer: person_role("classification_officer"),
            role_creation_date: date_role("creation_date"),
            role_classification_date: date_role("classification_date"),
            role_approver: approver,
            my_creator: Handle::default(),
            my_design_owner: Handle::default(),
            my_design_supplier: Handle::default(),
            my_classification_officer: Handle::default(),
            my_security: Handle::default(),
            my_creation_date: Handle::default(),
            my_classification_date: Handle::default(),
            my_approval: Handle::default(),
            my_approver: Handle::default(),
            my_approval_date_time: Handle::default(),
            my_product_category_relationship: Handle::default(),
        }
    }
}

/// A getter of a field that holds a handle.
macro_rules! context_getters {
    ($($(#[$meta:meta])* $name:ident = $field:ident: $target:ty;)+) => {
        $($(#[$meta])* pub fn $name(&self) -> Handle<$target> { self.$field.clone() })+
    };
}

impl StepConstructAP203Context {
    /// `StepConstructAP203Context()`: creates the roles.
    pub fn new() -> Self {
        Self::default()
    }
    /// `default_approval`.
    pub fn default_approval(&mut self) -> Result<Handle<StepBasicApproval>, Failure> {
        if self.def_approval.is_null() {
            let status = Handle::new(StepBasicApprovalStatus::new());
            status.init(&text("not_yet_approved"));
            let approval = Handle::new(StepBasicApproval::new());
            approval.init(&status, &text(""));
            self.def_approval = approval;
        }
        Ok(self.def_approval.clone())
    }
    /// `set_default_approval`.
    pub fn set_default_approval(&mut self, app: &Handle<StepBasicApproval>) {
        self.def_approval = app.clone();
    }
    /// `default_date_and_time`.
    pub fn default_date_and_time(&mut self) -> Result<Handle<StepBasicDateAndTime>, Failure> {
        if self.def_date_and_time.is_null() {
            self.def_date_and_time = new_date_and_time()?;
        }
        Ok(self.def_date_and_time.clone())
    }
    /// `set_default_date_and_time`.
    pub fn set_default_date_and_time(&mut self, dt: &Handle<StepBasicDateAndTime>) {
        self.def_date_and_time = dt.clone();
    }
    /// `default_person_and_organization`.
    pub fn default_person_and_organization(
        &mut self,
    ) -> Result<Handle<StepBasicPersonAndOrganization>, Failure> {
        if self.def_person_and_organization.is_null() {
            self.def_person_and_organization = new_person_and_organization();
        }
        Ok(self.def_person_and_organization.clone())
    }
    /// `set_default_person_and_organization`.
    pub fn set_default_person_and_organization(
        &mut self,
        po: &Handle<StepBasicPersonAndOrganization>,
    ) {
        self.def_person_and_organization = po.clone();
    }
    /// `default_security_classification_level`.
    pub fn default_security_classification_level(
        &mut self,
    ) -> Handle<StepBasicSecurityClassificationLevel> {
        if self.def_security_classification_level.is_null() {
            let level = Handle::new(StepBasicSecurityClassificationLevel::new());
            level.init(&text("unclassified"));
            self.def_security_classification_level = level;
        }
        self.def_security_classification_level.clone()
    }
    /// `set_default_security_classification_level`.
    pub fn set_default_security_classification_level(
        &mut self,
        scl: &Handle<StepBasicSecurityClassificationLevel>,
    ) {
        self.def_security_classification_level = scl.clone();
    }
    context_getters! {
        /// `RoleCreator`.
        role_creator = role_creator: StepBasicPersonAndOrganizationRole;
        /// `RoleDesignOwner`.
        role_design_owner = role_design_owner: StepBasicPersonAndOrganizationRole;
        /// `RoleDesignSupplier`.
        role_design_supplier = role_design_supplier: StepBasicPersonAndOrganizationRole;
        /// `RoleClassificationOfficer`.
        role_classification_officer = role_classification_officer: StepBasicPersonAndOrganizationRole;
        /// `RoleCreationDate`.
        role_creation_date = role_creation_date: StepBasicDateTimeRole;
        /// `RoleClassificationDate`.
        role_classification_date = role_classification_date: StepBasicDateTimeRole;
        /// `RoleApprover`.
        role_approver = role_approver: StepBasicApprovalRole;
        /// `GetCreator`.
        get_creator = my_creator: StepAP203CcDesignPersonAndOrganizationAssignment;
        /// `GetDesignOwner`.
        get_design_owner = my_design_owner: StepAP203CcDesignPersonAndOrganizationAssignment;
        /// `GetDesignSupplier`.
        get_design_supplier = my_design_supplier: StepAP203CcDesignPersonAndOrganizationAssignment;
        /// `GetClassificationOfficer`.
        get_classification_officer = my_classification_officer: StepAP203CcDesignPersonAndOrganizationAssignment;
        /// `GetSecurity`.
        get_security = my_security: StepAP203CcDesignSecurityClassification;
        /// `GetCreationDate`.
        get_creation_date = my_creation_date: StepAP203CcDesignDateAndTimeAssignment;
        /// `GetClassificationDate`.
        get_classification_date = my_classification_date: StepAP203CcDesignDateAndTimeAssignment;
        /// `GetApproval`.
        get_approval = my_approval: StepAP203CcDesignApproval;
        /// `GetApprover`.
        get_approver = my_approver: StepBasicApprovalPersonOrganization;
        /// `GetApprovalDateTime`.
        get_approval_date_time = my_approval_date_time: StepBasicApprovalDateTime;
        /// `GetProductCategoryRelationship`.
        get_product_category_relationship = my_product_category_relationship: StepBasicProductCategoryRelationship;
    }
    /// `init(sdr)`: takes the SDR and the data of the part it defines; existing data is cleared.
    pub fn init_sdr(
        &mut self,
        sdr: &Handle<StepShapeShapeDefinitionRepresentation>,
    ) -> Result<(), Failure> {
        self.clear();
        let mut sdr_tool = StepConstructPart::new();
        sdr_tool.read_sdr(sdr);
        self.init_part(&sdr_tool)
    }
    /// `init(SDRTool)`.
    pub fn init(&mut self, sdr_tool: &StepConstructPart) -> Result<(), Failure> {
        self.clear();
        self.init_part(sdr_tool)
    }
    /// `init(nauo)`.
    pub fn init_nauo(
        &mut self,
        nauo: &Handle<StepReprNextAssemblyUsageOccurrence>,
    ) -> Result<(), Failure> {
        self.clear();
        self.init_assembly(nauo)
    }
    /// `clear`: the approver and its date stay.
    pub fn clear(&mut self) {
        self.my_creator = Handle::default();
        self.my_design_owner = Handle::default();
        self.my_design_supplier = Handle::default();
        self.my_classification_officer = Handle::default();
        self.my_security = Handle::default();
        self.my_creation_date = Handle::default();
        self.my_classification_date = Handle::default();
        self.my_approval = Handle::default();
        self.my_product_category_relationship = Handle::default();
    }
    /// `init_roles`: fresh roles (`new` already made them).
    pub fn init_roles(&mut self) {
        let fresh = Self::default();
        self.role_creator = fresh.role_creator;
        self.role_design_owner = fresh.role_design_owner;
        self.role_design_supplier = fresh.role_design_supplier;
        self.role_classification_officer = fresh.role_classification_officer;
        self.role_creation_date = fresh.role_creation_date;
        self.role_classification_date = fresh.role_classification_date;
        self.role_approver = fresh.role_approver;
    }
    /// A new assignment of the default person and organization.
    fn person_assignment(
        &mut self,
        role: Handle<StepBasicPersonAndOrganizationRole>,
        values: &[Handle<StandardTransient>],
    ) -> Result<Handle<StepAP203CcDesignPersonAndOrganizationAssignment>, Failure> {
        let assignment = Handle::new(StepAP203CcDesignPersonAndOrganizationAssignment::new());
        let items = select_items::<StepAP203PersonOrganizationItem>(values)?;
        assignment.init(&self.default_person_and_organization()?, &role, &items);
        Ok(assignment)
    }
    /// A new assignment of the default date.
    fn date_assignment(
        &mut self,
        role: Handle<StepBasicDateTimeRole>,
        value: &Handle<StandardTransient>,
    ) -> Result<Handle<StepAP203CcDesignDateAndTimeAssignment>, Failure> {
        let assignment = Handle::new(StepAP203CcDesignDateAndTimeAssignment::new());
        let items = select_items::<StepAP203DateTimeItem>(std::slice::from_ref(value))?;
        assignment.init(&self.default_date_and_time()?, &role, &items);
        Ok(assignment)
    }
    /// The classification of `item`, when there is none yet.
    fn ensure_security(&mut self, item: &Handle<StandardTransient>) -> Result<(), Failure> {
        if self.my_security.is_some() {
            return Ok(());
        }
        let classification = Handle::new(StepBasicSecurityClassification::new());
        classification.init(
            &text(""),
            &text(""),
            &self.default_security_classification_level(),
        );
        let security = Handle::new(StepAP203CcDesignSecurityClassification::new());
        let items = select_items::<StepAP203ClassifiedItem>(std::slice::from_ref(item))?;
        security.init(&classification, &items);
        self.my_security = security;
        Ok(())
    }
    /// The approval of `values`, when there is none yet.
    fn ensure_approval(&mut self, values: &[Handle<StandardTransient>]) -> Result<(), Failure> {
        if self.my_approval.is_some() {
            return Ok(());
        }
        let approval = Handle::new(StepAP203CcDesignApproval::new());
        let items = select_items::<StepAP203ApprovedItem>(values)?;
        approval.init(&self.default_approval()?, &items);
        self.my_approval = approval;
        Ok(())
    }
    /// The assigned classification, as the value selects hold it.
    fn assigned_security(&self) -> Result<Handle<StandardTransient>, Failure> {
        Ok(step_live(&self.my_security, WHERE)?
            .assigned_security_classification()
            .cast())
    }
    /// `init_part`: all missing data required for a part.
    fn init_part(&mut self, sdr_tool: &StepConstructPart) -> Result<(), Failure> {
        let pdf: Handle<StandardTransient> = sdr_tool.pdf()?.cast();
        let pd: Handle<StandardTransient> = sdr_tool.pd()?.cast();
        if self.my_creator.is_null() {
            self.my_creator =
                self.person_assignment(self.role_creator(), &[pdf.clone(), pd.clone()])?;
        }
        if self.my_design_owner.is_null() {
            self.my_design_owner =
                self.person_assignment(self.role_design_owner(), &[sdr_tool.product()?.cast()])?;
        }
        if self.my_design_supplier.is_null() {
            self.my_design_supplier =
                self.person_assignment(self.role_design_supplier(), std::slice::from_ref(&pdf))?;
        }
        if self.my_creation_date.is_null() {
            self.my_creation_date = self.date_assignment(self.role_creation_date(), &pd)?;
        }
        self.ensure_security(&pdf)?;
        self.init_security_requisites()?;
        if self.my_approval.is_null() {
            self.ensure_approval(&[pdf, pd, self.assigned_security()?])?;
        }
        self.init_approval_requisites()?;
        if self.my_product_category_relationship.is_null() {
            let category = Handle::new(StepBasicProductCategory::new());
            category.init(&text("part"), false, &Handle::default());
            let relationship = Handle::new(StepBasicProductCategoryRelationship::new());
            relationship.init(
                &text(""),
                true,
                &text(""),
                &category,
                &sdr_tool.prpc().cast(),
            );
            self.my_product_category_relationship = relationship;
        }
        Ok(())
    }
    /// `init_assembly`: all missing data required for an assembly link.
    pub fn init_assembly(
        &mut self,
        nauo: &Handle<StepReprNextAssemblyUsageOccurrence>,
    ) -> Result<(), Failure> {
        self.ensure_security(&nauo.cast())?;
        self.init_security_requisites()?;
        if self.my_approval.is_null() {
            self.ensure_approval(&[self.assigned_security()?])?;
        }
        self.init_approval_requisites()
    }
    /// `init_security_requisites`: the officer and the date of the current classification.
    pub fn init_security_requisites(&mut self) -> Result<(), Failure> {
        let assigned = self.assigned_security()?;
        let officer_current = match self.my_classification_officer.get() {
            None => false,
            Some(officer) => first_item_value(&officer.items(), WHERE)? == assigned,
        };
        if !officer_current {
            self.my_classification_officer = self.person_assignment(
                self.role_classification_officer(),
                std::slice::from_ref(&assigned),
            )?;
        }
        let date_current = match self.my_classification_date.get() {
            None => false,
            Some(date) => first_item_value(&date.items(), WHERE)? == assigned,
        };
        if !date_current {
            self.my_classification_date =
                self.date_assignment(self.role_classification_date(), &assigned)?;
        }
        Ok(())
    }
    /// `init_approval_requisites`: the approver and the date of the current approval.
    pub fn init_approval_requisites(&mut self) -> Result<(), Failure> {
        let assigned = step_live(&self.my_approval, WHERE)?.assigned_approval();
        if self
            .my_approver
            .get()
            .is_none_or(|approver| approver.authorized_approval() != assigned)
        {
            let mut po = StepBasicPersonOrganizationSelect::default();
            po.set_value(&self.default_person_and_organization()?.cast())?;
            let approver = Handle::new(StepBasicApprovalPersonOrganization::new());
            approver.init(&po, &assigned, &self.role_approver());
            self.my_approver = approver;
        }
        if self
            .my_approval_date_time
            .get()
            .is_none_or(|date| date.dated_approval() != assigned)
        {
            let mut dt = StepBasicDateTimeSelect::default();
            dt.set_value(&self.default_date_and_time()?.cast())?;
            let date = Handle::new(StepBasicApprovalDateTime::new());
            date.init(&dt, &assigned);
            self.my_approval_date_time = date;
        }
        Ok(())
    }
}

/// `items->Value(1).Value()`.
fn first_item_value<S>(
    items: &Handle<HArray1<S>>,
    where_called: &str,
) -> Result<Handle<StandardTransient>, Failure>
where
    S: StepDataSelectType + Clone + Send + Sync + 'static,
{
    Ok(crate::step_entity::step_array1_item(items, 1, where_called)?.value())
}
