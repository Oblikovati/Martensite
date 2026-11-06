//! The select members of `StepFEA` and the selects holding them. The tensor selects drop
//! their arrays, with the code commented out; `StepFEASymmetricTensor43d` declares
//! `SetFeaIsotropicSymmetricTensor43d` with no definition, so that member does not exist here.
#[allow(
    unused_imports,
    reason = "the classes of the package reference each other"
)]
use super::*;
use crate::step_data::{StepDataSelectArrReal, StepDataSelectNamed, StepDataSelectType};
use crate::step_entity::*;
use crate::step_members::{step_case_name_member, step_immediate_select, step_member_accessors};

step_case_name_member!(
    /// Degree of freedom member.
    StepFEADegreeOfFreedomMember,
    "StepFEADegreeOfFreedomMember",
    StepDataSelectNamed,
    ["ENUMERATED_DEGREE_OF_FREEDOM", "APPLICATION_DEFINED_DEGREE_OF_FREEDOM"]
);

step_case_name_member!(
    /// Symmetric tensor23d member.
    StepFEASymmetricTensor23dMember,
    "StepFEASymmetricTensor23dMember",
    StepDataSelectArrReal,
    [
        "ISOTROPIC_SYMMETRIC_TENSOR2_3D",
        "ORTHOTROPIC_SYMMETRIC_TENSOR2_3D",
        "ANISOTROPIC_SYMMETRIC_TENSOR2_3D",
    ]
);

step_case_name_member!(
    /// Symmetric tensor43d member.
    StepFEASymmetricTensor43dMember,
    "StepFEASymmetricTensor43dMember",
    StepDataSelectArrReal,
    [
        "ANISOTROPIC_SYMMETRIC_TENSOR4_3D",
        "FEA_ISOTROPIC_SYMMETRIC_TENSOR4_3D",
        "FEA_ISO_ORTHOTROPIC_SYMMETRIC_TENSOR4_3D",
        "FEA_TRANSVERSE_ISOTROPIC_SYMMETRIC_TENSOR4_3D",
        "FEA_COLUMN_NORMALISED_ORTHOTROPIC_SYMMETRIC_TENSOR4_3D",
        "FEA_COLUMN_NORMALISED_MONOCLINIC_SYMMETRIC_TENSOR4_3D",
    ]
);

step_immediate_select!(
    /// Degree of freedom.
    StepFEADegreeOfFreedom,
    StepFEADegreeOfFreedomMember,
    ["ENUMERATED_DEGREE_OF_FREEDOM", "APPLICATION_DEFINED_DEGREE_OF_FREEDOM"]
);

impl StepFEADegreeOfFreedom {
    // Only number 1 is decoded, the other cases are commented out.
    step_member_accessors! {
        StepFEADegreeOfFreedomMember;
        enum set_enumerated_degree_of_freedom, enumerated_degree_of_freedom: StepFEAEnumeratedDegreeOfFreedom =
            "ENUMERATED_DEGREE_OF_FREEDOM", [XTranslation], default XTranslation;
        string set_application_defined_degree_of_freedom, application_defined_degree_of_freedom =
            "APPLICATION_DEFINED_DEGREE_OF_FREEDOM";
    }
}

step_immediate_select!(
    /// Symmetric tensor23d.
    StepFEASymmetricTensor23d,
    StepFEASymmetricTensor23dMember,
    [
        "ISOTROPIC_SYMMETRIC_TENSOR2_3D",
        "ORTHOTROPIC_SYMMETRIC_TENSOR2_3D",
        "ANISOTROPIC_SYMMETRIC_TENSOR2_3D",
    ]
);

impl StepFEASymmetricTensor23d {
    step_member_accessors! {
        StepFEASymmetricTensor23dMember;
        real set_isotropic_symmetric_tensor23d, isotropic_symmetric_tensor23d = "ISOTROPIC_SYMMETRIC_TENSOR2_3D";
        array set_orthotropic_symmetric_tensor23d, orthotropic_symmetric_tensor23d = "ORTHOTROPIC_SYMMETRIC_TENSOR2_3D";
        array set_anisotropic_symmetric_tensor23d, anisotropic_symmetric_tensor23d = "ANISOTROPIC_SYMMETRIC_TENSOR2_3D";
    }
}

step_immediate_select!(
    /// Symmetric tensor43d.
    StepFEASymmetricTensor43d,
    StepFEASymmetricTensor43dMember,
    [
        "ANISOTROPIC_SYMMETRIC_TENSOR4_3D",
        "FEA_ISOTROPIC_SYMMETRIC_TENSOR4_3D",
        "FEA_ISO_ORTHOTROPIC_SYMMETRIC_TENSOR4_3D",
        "FEA_TRANSVERSE_ISOTROPIC_SYMMETRIC_TENSOR4_3D",
        "FEA_COLUMN_NORMALISED_ORTHOTROPIC_SYMMETRIC_TENSOR4_3D",
        "FEA_COLUMN_NORMALISED_MONOCLINIC_SYMMETRIC_TENSOR4_3D",
    ]
);

impl StepFEASymmetricTensor43d {
    /// `anisotropic_symmetric_tensor43d`: null.
    pub fn anisotropic_symmetric_tensor43d(&self) -> Handle<HArray1<f64>> {
        Handle::default()
    }
    /// `fea_isotropic_symmetric_tensor43d`: null.
    pub fn fea_isotropic_symmetric_tensor43d(&self) -> Handle<HArray1<f64>> {
        Handle::default()
    }
    /// `fea_iso_orthotropic_symmetric_tensor43d`: null.
    pub fn fea_iso_orthotropic_symmetric_tensor43d(&self) -> Handle<HArray1<f64>> {
        Handle::default()
    }
    /// `fea_transverse_isotropic_symmetric_tensor43d`: null.
    pub fn fea_transverse_isotropic_symmetric_tensor43d(&self) -> Handle<HArray1<f64>> {
        Handle::default()
    }
    /// `fea_column_normalised_orthotropic_symmetric_tensor43d`: null.
    pub fn fea_column_normalised_orthotropic_symmetric_tensor43d(&self) -> Handle<HArray1<f64>> {
        Handle::default()
    }
    /// `fea_column_normalised_monoclinic_symmetric_tensor43d`: null.
    pub fn fea_column_normalised_monoclinic_symmetric_tensor43d(&self) -> Handle<HArray1<f64>> {
        Handle::default()
    }
}

/// The array of a stored `StepDataSelectArrReal`, or a new array of `size` when another value is
/// held. The array is uninitialized; here it reads zero.
fn stored_or_new_array(
    value: &Handle<StandardTransient>,
    size: i32,
) -> Result<Handle<HArray1<f64>>, Failure> {
    let member = value.cast::<StepDataSelectArrReal>();
    if let Some(live) = member.get() {
        return Ok(live.arr_real());
    }
    let fresh =
        HArray1::with_value(1, size, 0.0).map_err(|error| error.into_failure("SymmetricTensor"))?;
    Ok(Handle::new(fresh))
}

step_select! {
    /// Symmetric tensor22d.
    StepFEASymmetricTensor22d, [StepDataSelectArrReal], {
    }
}

impl StepFEASymmetricTensor22d {
    /// `anisotropic_symmetric_tensor22d`: the stored array, or a new array of 3 reals.
    ///
    /// # Errors
    /// Only from the allocation of the bounds, which are fixed here and valid.
    pub fn anisotropic_symmetric_tensor22d(&self) -> Result<Handle<HArray1<f64>>, Failure> {
        stored_or_new_array(self.select_value(), 3)
    }
}

step_select! {
    /// Symmetric tensor42d.
    StepFEASymmetricTensor42d, [StepDataSelectArrReal], {
    }
}

impl StepFEASymmetricTensor42d {
    /// `anisotropic_symmetric_tensor42d`: the stored array, or a new array of 6 reals.
    ///
    /// # Errors
    /// Only from the allocation of the bounds, which are fixed here and valid.
    pub fn anisotropic_symmetric_tensor42d(&self) -> Result<Handle<HArray1<f64>>, Failure> {
        stored_or_new_array(self.select_value(), 6)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::step_data::StepDataSelectMember;

    #[test]
    fn case_name_member_matches_only_its_current_case() {
        let member = StepFEASymmetricTensor23dMember::new();
        assert!(!member.has_name());
        assert!(
            member.matches("UNKNOWN"),
            "recorded: no case compares equal to an unknown name"
        );
        assert!(member.set_name("ORTHOTROPIC_SYMMETRIC_TENSOR2_3D"));
        assert_eq!(member.name(), "ORTHOTROPIC_SYMMETRIC_TENSOR2_3D");
        assert!(member.matches("ORTHOTROPIC_SYMMETRIC_TENSOR2_3D"));
        assert!(!member.matches("ISOTROPIC_SYMMETRIC_TENSOR2_3D"));
        assert!(!member.set_name("UNKNOWN"));
        assert!(!member.has_name());
    }

    #[test]
    fn degree_of_freedom_decodes_only_number_one() {
        let mut select = StepFEADegreeOfFreedom::new();
        let member = select.new_member();
        assert_eq!(select.case_mem(&member), 0);
        member
            .get()
            .unwrap()
            .set_name("ENUMERATED_DEGREE_OF_FREEDOM");
        assert_eq!(select.case_mem(&member), 1);
        select.set_value(&member.cast()).unwrap();
        select.set_enumerated_degree_of_freedom(StepFEAEnumeratedDegreeOfFreedom::ZTranslation);
        assert_eq!(
            select.enumerated_degree_of_freedom(),
            StepFEAEnumeratedDegreeOfFreedom::XTranslation
        );
        assert!(select.application_defined_degree_of_freedom().is_null());
    }

    #[test]
    fn degree_of_freedom_keeps_an_application_string_and_refuses_a_null_one() {
        let mut select = StepFEADegreeOfFreedom::new();
        select
            .set_application_defined_degree_of_freedom(&Handle::default())
            .unwrap();
        let member = Handle::new(StepFEADegreeOfFreedomMember::new());
        member.set_name("APPLICATION_DEFINED_DEGREE_OF_FREEDOM");
        select.set_value(&member.cast()).unwrap();
        assert!(
            select
                .set_application_defined_degree_of_freedom(&Handle::default())
                .is_err()
        );
        select
            .set_application_defined_degree_of_freedom(&Handle::new(HAsciiString::new("twist")))
            .unwrap();
        let text = select.application_defined_degree_of_freedom();
        assert_eq!(text.get().unwrap().string().as_bytes(), b"twist");
    }

    #[test]
    fn tensor_selects_answer_their_real_and_drop_their_arrays() {
        let mut select = StepFEASymmetricTensor23d::new();
        let member = Handle::new(StepFEASymmetricTensor23dMember::new());
        member.set_name("ISOTROPIC_SYMMETRIC_TENSOR2_3D");
        select.set_value(&member.cast()).unwrap();
        select.set_isotropic_symmetric_tensor23d(2.5);
        assert_eq!(select.isotropic_symmetric_tensor23d(), 2.5);
        select.set_orthotropic_symmetric_tensor23d(&Handle::default());
        assert_eq!(member.name(), "ORTHOTROPIC_SYMMETRIC_TENSOR2_3D");
        assert!(select.orthotropic_symmetric_tensor23d().is_null());
        assert!(
            StepFEASymmetricTensor43d::new()
                .anisotropic_symmetric_tensor43d()
                .is_null()
        );
    }

    #[test]
    fn anisotropic_tensor_without_a_member_is_a_new_array_of_the_size() {
        let two = StepFEASymmetricTensor22d::new()
            .anisotropic_symmetric_tensor22d()
            .unwrap();
        assert_eq!(two.get().unwrap().array1().length(), 3);
        let four = StepFEASymmetricTensor42d::new()
            .anisotropic_symmetric_tensor42d()
            .unwrap();
        assert_eq!(four.get().unwrap().array1().length(), 6);
    }
}
