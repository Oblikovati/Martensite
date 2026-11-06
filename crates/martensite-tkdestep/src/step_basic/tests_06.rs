#[allow(
    unused_imports,
    reason = "the classes of the package reference each other across its files"
)]
use super::*;
#[allow(unused_imports)]
use crate::step_data::*;
use crate::step_entity::*;

#[test]
fn runtime_type_step_basic_personal_address() {
    let object = Handle::new(StepBasicPersonalAddress::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicPersonalAddress"
    );
    assert!(object.is_kind_name("StepBasicAddress"));
}

#[test]
fn init_round_trip_step_basic_personal_address() {
    let has_ainternal_location = true;
    let a_internal_location = Handle::new(HAsciiString::new("sample"));
    let has_astreet_number = true;
    let a_street_number = Handle::new(HAsciiString::new("sample"));
    let has_astreet = true;
    let a_street = Handle::new(HAsciiString::new("sample"));
    let has_apostal_box = true;
    let a_postal_box = Handle::new(HAsciiString::new("sample"));
    let has_atown = true;
    let a_town = Handle::new(HAsciiString::new("sample"));
    let has_aregion = true;
    let a_region = Handle::new(HAsciiString::new("sample"));
    let has_apostal_code = true;
    let a_postal_code = Handle::new(HAsciiString::new("sample"));
    let has_acountry = true;
    let a_country = Handle::new(HAsciiString::new("sample"));
    let has_afacsimile_number = true;
    let a_facsimile_number = Handle::new(HAsciiString::new("sample"));
    let has_atelephone_number = true;
    let a_telephone_number = Handle::new(HAsciiString::new("sample"));
    let has_aelectronic_mail_address = true;
    let a_electronic_mail_address = Handle::new(HAsciiString::new("sample"));
    let has_atelex_number = true;
    let a_telex_number = Handle::new(HAsciiString::new("sample"));
    let a_people = Handle::new(HArray1::<Handle<StepBasicPerson>>::with_bounds(1, 2).unwrap());
    let a_description = Handle::new(HAsciiString::new("sample"));
    let object = StepBasicPersonalAddress::new();
    object.init(
        has_ainternal_location,
        &a_internal_location,
        has_astreet_number,
        &a_street_number,
        has_astreet,
        &a_street,
        has_apostal_box,
        &a_postal_box,
        has_atown,
        &a_town,
        has_aregion,
        &a_region,
        has_apostal_code,
        &a_postal_code,
        has_acountry,
        &a_country,
        has_afacsimile_number,
        &a_facsimile_number,
        has_atelephone_number,
        &a_telephone_number,
        has_aelectronic_mail_address,
        &a_electronic_mail_address,
        has_atelex_number,
        &a_telex_number,
        &a_people,
        &a_description,
    );
    assert!(object.people() == a_people);
    assert!(object.description() == a_description);
}

#[test]
fn runtime_type_step_basic_physically_modeled_product_definition() {
    let object = Handle::new(StepBasicPhysicallyModeledProductDefinition::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicPhysicallyModeledProductDefinition"
    );
    assert!(object.is_kind_name("StepBasicProductDefinition"));
}

#[test]
fn runtime_type_step_basic_plane_angle_measure_with_unit() {
    let object = Handle::new(StepBasicPlaneAngleMeasureWithUnit::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicPlaneAngleMeasureWithUnit"
    );
    assert!(object.is_kind_name("StepBasicMeasureWithUnit"));
}

#[test]
fn runtime_type_step_basic_plane_angle_unit() {
    let object = Handle::new(StepBasicPlaneAngleUnit::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicPlaneAngleUnit"
    );
    assert!(object.is_kind_name("StepBasicNamedUnit"));
}

#[test]
fn runtime_type_step_basic_product() {
    let object = Handle::new(StepBasicProduct::new());
    assert_eq!(object.dynamic_type().unwrap().name(), "StepBasicProduct");
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_basic_product() {
    let a_id = Handle::new(HAsciiString::new("sample"));
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_description = Handle::new(HAsciiString::new("sample"));
    let a_frame_of_reference =
        Handle::new(HArray1::<Handle<StepBasicProductContext>>::with_bounds(1, 2).unwrap());
    let object = StepBasicProduct::new();
    object.init(&a_id, &a_name, &a_description, &a_frame_of_reference);
    assert!(object.id() == a_id);
    assert!(object.name() == a_name);
    assert!(object.description() == a_description);
    assert!(object.frame_of_reference() == a_frame_of_reference);
}

#[test]
fn runtime_type_step_basic_product_category() {
    let object = Handle::new(StepBasicProductCategory::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicProductCategory"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_basic_product_category() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let has_adescription = true;
    let a_description = Handle::new(HAsciiString::new("sample"));
    let object = StepBasicProductCategory::new();
    object.init(&a_name, has_adescription, &a_description);
    assert!(object.name() == a_name);
    assert!(object.has_description() == has_adescription);
    assert!(object.description() == a_description);
}

#[test]
fn runtime_type_step_basic_product_category_relationship() {
    let object = Handle::new(StepBasicProductCategoryRelationship::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicProductCategoryRelationship"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_basic_product_category_relationship() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let has_description = true;
    let a_description = Handle::new(HAsciiString::new("sample"));
    let a_category = Handle::new(StepBasicProductCategory::new());
    let a_sub_category = Handle::new(StepBasicProductCategory::new());
    let object = StepBasicProductCategoryRelationship::new();
    object.init(
        &a_name,
        has_description,
        &a_description,
        &a_category,
        &a_sub_category,
    );
    assert!(object.name() == a_name);
    assert!(object.has_description() == has_description);
    assert!(object.description() == a_description);
    assert!(object.category() == a_category);
    assert!(object.sub_category() == a_sub_category);
}

#[test]
fn runtime_type_step_basic_product_concept_context() {
    let object = Handle::new(StepBasicProductConceptContext::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicProductConceptContext"
    );
    assert!(object.is_kind_name("StepBasicApplicationContextElement"));
}

#[test]
fn init_round_trip_step_basic_product_concept_context() {
    let a_application_context_element_name = Handle::new(HAsciiString::new("sample"));
    let a_application_context_element_frame_of_reference =
        Handle::new(StepBasicApplicationContext::new());
    let a_market_segment_type = Handle::new(HAsciiString::new("sample"));
    let object = StepBasicProductConceptContext::new();
    object.init(
        &a_application_context_element_name,
        &a_application_context_element_frame_of_reference,
        &a_market_segment_type,
    );
    assert!(object.market_segment_type() == a_market_segment_type);
}

#[test]
fn runtime_type_step_basic_product_context() {
    let object = Handle::new(StepBasicProductContext::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicProductContext"
    );
    assert!(object.is_kind_name("StepBasicApplicationContextElement"));
}

#[test]
fn init_round_trip_step_basic_product_context() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_frame_of_reference = Handle::new(StepBasicApplicationContext::new());
    let a_discipline_type = Handle::new(HAsciiString::new("sample"));
    let object = StepBasicProductContext::new();
    object.init(&a_name, &a_frame_of_reference, &a_discipline_type);
    assert!(object.discipline_type() == a_discipline_type);
}

#[test]
fn runtime_type_step_basic_product_definition() {
    let object = Handle::new(StepBasicProductDefinition::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicProductDefinition"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_basic_product_definition() {
    let a_id = Handle::new(HAsciiString::new("sample"));
    let a_description = Handle::new(HAsciiString::new("sample"));
    let a_formation = Handle::new(StepBasicProductDefinitionFormation::new());
    let a_frame_of_reference = Handle::new(StepBasicProductDefinitionContext::new());
    let object = StepBasicProductDefinition::new();
    object.init(&a_id, &a_description, &a_formation, &a_frame_of_reference);
    assert!(object.id() == a_id);
    assert!(object.description() == a_description);
    assert!(object.formation() == a_formation);
    assert!(object.frame_of_reference() == a_frame_of_reference);
}

#[test]
fn runtime_type_step_basic_product_definition_context() {
    let object = Handle::new(StepBasicProductDefinitionContext::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicProductDefinitionContext"
    );
    assert!(object.is_kind_name("StepBasicApplicationContextElement"));
}

#[test]
fn init_round_trip_step_basic_product_definition_context() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_frame_of_reference = Handle::new(StepBasicApplicationContext::new());
    let a_life_cycle_stage = Handle::new(HAsciiString::new("sample"));
    let object = StepBasicProductDefinitionContext::new();
    object.init(&a_name, &a_frame_of_reference, &a_life_cycle_stage);
    assert!(object.life_cycle_stage() == a_life_cycle_stage);
}

#[test]
fn runtime_type_step_basic_product_definition_effectivity() {
    let object = Handle::new(StepBasicProductDefinitionEffectivity::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicProductDefinitionEffectivity"
    );
    assert!(object.is_kind_name("StepBasicEffectivity"));
}

#[test]
fn init_round_trip_step_basic_product_definition_effectivity() {
    let a_id = Handle::new(HAsciiString::new("sample"));
    let a_usage = Handle::new(StepBasicProductDefinitionRelationship::new());
    let object = StepBasicProductDefinitionEffectivity::new();
    object.init(&a_id, &a_usage);
    assert!(object.usage() == a_usage);
}

#[test]
fn runtime_type_step_basic_product_definition_formation() {
    let object = Handle::new(StepBasicProductDefinitionFormation::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicProductDefinitionFormation"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_basic_product_definition_formation() {
    let a_id = Handle::new(HAsciiString::new("sample"));
    let a_description = Handle::new(HAsciiString::new("sample"));
    let a_of_product = Handle::new(StepBasicProduct::new());
    let object = StepBasicProductDefinitionFormation::new();
    object.init(&a_id, &a_description, &a_of_product);
    assert!(object.id() == a_id);
    assert!(object.description() == a_description);
    assert!(object.of_product() == a_of_product);
}

#[test]
fn runtime_type_step_basic_product_definition_formation_relationship() {
    let object = Handle::new(StepBasicProductDefinitionFormationRelationship::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicProductDefinitionFormationRelationship"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_basic_product_definition_formation_relationship() {
    let a_id = Handle::new(HAsciiString::new("sample"));
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_description = Handle::new(HAsciiString::new("sample"));
    let a_relating_product_definition_formation =
        Handle::new(StepBasicProductDefinitionFormation::new());
    let a_related_product_definition_formation =
        Handle::new(StepBasicProductDefinitionFormation::new());
    let object = StepBasicProductDefinitionFormationRelationship::new();
    object.init(
        &a_id,
        &a_name,
        &a_description,
        &a_relating_product_definition_formation,
        &a_related_product_definition_formation,
    );
    assert!(object.id() == a_id);
    assert!(object.name() == a_name);
    assert!(object.description() == a_description);
    assert!(
        object.relating_product_definition_formation() == a_relating_product_definition_formation
    );
    assert!(
        object.related_product_definition_formation() == a_related_product_definition_formation
    );
}

#[test]
fn runtime_type_step_basic_product_definition_formation_with_specified_source() {
    let object = Handle::new(StepBasicProductDefinitionFormationWithSpecifiedSource::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicProductDefinitionFormationWithSpecifiedSource"
    );
    assert!(object.is_kind_name("StepBasicProductDefinitionFormation"));
}

#[test]
fn init_round_trip_step_basic_product_definition_formation_with_specified_source() {
    let a_id = Handle::new(HAsciiString::new("sample"));
    let a_description = Handle::new(HAsciiString::new("sample"));
    let a_of_product = Handle::new(StepBasicProduct::new());
    let a_make_or_buy = StepBasicSource::default();
    let object = StepBasicProductDefinitionFormationWithSpecifiedSource::new();
    object.init(&a_id, &a_description, &a_of_product, a_make_or_buy);
    assert!(object.make_or_buy() == a_make_or_buy);
}

#[test]
fn runtime_type_step_basic_product_definition_reference() {
    let object = Handle::new(StepBasicProductDefinitionReference::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicProductDefinitionReference"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}
