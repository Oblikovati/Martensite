//! `StepDataField`: the values, lists and copies of a field.
use martensite_tkdestep::step_data::{
    StepDataDescribed, StepDataESDescr, StepDataField, StepDataFieldKind as Kind,
    StepDataFieldList, StepDataFieldListN, StepDataLogical, StepDataPDescr, StepDataSelectArrReal,
    StepDataSelectInt, StepDataSelectMember, StepDataSelectNamed, StepDataSelectReal,
    StepDataSimple,
};
use martensite_tkernel::{
    ncollection::{HArray1, HArray2},
    standard::{Handle, StandardTransient},
    tcollection::HAsciiString,
};
use martensite_tkxsbase::interface::{InterfaceEntityIterator, InterfaceParamType};

type Transients = HArray1<Handle<StandardTransient>>;
type TransientGrid = HArray2<Handle<StandardTransient>>;

fn entity() -> Handle<StandardTransient> {
    Handle::new(StandardTransient::default())
}

fn text(value: &str) -> Handle<StandardTransient> {
    Handle::new(HAsciiString::new(value)).cast()
}

#[test]
fn scalar_values_keep_their_kinds() {
    let mut field = StepDataField::new();
    field.set_integer(42);
    assert_eq!(field.kind(true), Kind::INTEGER);
    assert_eq!(field.integer(1, 1).unwrap(), 42);
    field.set_boolean(true);
    assert_eq!(field.kind(true), Kind::BOOLEAN);
    assert!(field.boolean(1, 1).unwrap());
    field.set_logical(StepDataLogical::LUnknown);
    assert_eq!(field.kind(true), Kind::LOGICAL);
    assert_eq!(field.logical(1, 1).unwrap(), StepDataLogical::LUnknown);
    field.set_enum(4, b".FOUR.");
    assert_eq!(field.kind(true), Kind::ENUM);
    assert_eq!(field.enum_value(1, 1).unwrap(), 4);
    assert_eq!(field.enum_text(1, 1).unwrap().as_bytes(), b".FOUR.");
    field.set_string(b".OTHER.");
    assert_eq!(field.enum_value(1, 1).unwrap(), 4);
    assert_eq!(field.enum_text(1, 1).unwrap().as_bytes(), b".OTHER.");
    assert!(field.transient().cast::<HAsciiString>().is_some());
    field.set_string(b"text");
    assert_eq!(field.string(1, 1).unwrap().as_bytes(), b"text");
    assert!(field.transient().cast::<HAsciiString>().is_some());
}

#[test]
fn aggregate_bounds_are_preserved() {
    let array = HArray1::<i32>::with_bounds(-2, 0).unwrap();
    array.change_array1().set_value(-2, 10).unwrap();
    array.change_array1().set_value(-1, 20).unwrap();
    array.change_array1().set_value(0, 30).unwrap();
    let mut field = StepDataField::new();
    field.set(&Handle::new(array).cast());
    assert_eq!(field.kind(true), Kind::INTEGER);
    assert_eq!(field.arity(), 1);
    assert_eq!(field.lower(1), -2);
    assert_eq!(field.length(1), 3);
    assert_eq!(field.item_kind(-2, 1).unwrap(), Kind::INTEGER);
    assert_eq!(field.integer(-2, 1).unwrap(), 10);
    assert_eq!(field.integer(0, 1).unwrap(), 30);
}

#[test]
fn item_kind_reads_one_dimensional_any_list() {
    let mut field = StepDataField::new();
    field.set_list(4, -2).unwrap();
    let any = field.transient().cast::<Transients>();
    let array = any.get().expect("a list of transients");
    let select = StepDataSelectInt::default();
    select.set_kind(Kind::INTEGER.to_int());
    array.change_array1().set_value(-1, text("text")).unwrap();
    array
        .change_array1()
        .set_value(0, Handle::new(select).cast())
        .unwrap();
    array.change_array1().set_value(1, entity()).unwrap();
    assert_eq!(field.item_kind(-2, 1).unwrap(), Kind::UNDEFINED);
    assert_eq!(field.item_kind(-1, 1).unwrap(), Kind::STRING);
    assert_eq!(field.item_kind(0, 1).unwrap(), Kind::INTEGER);
    assert_eq!(field.item_kind(1, 1).unwrap(), Kind::ENTITY);
}

#[test]
fn item_kind_reads_two_dimensional_any_list() {
    let mut field = StepDataField::new();
    field.set_list2(1, 3, -1, 2).unwrap();
    let any = field.transient().cast::<TransientGrid>();
    let array = any.get().expect("a grid of transients");
    array
        .change_array2()
        .set_value(-1, 3, text("text"))
        .unwrap();
    array.change_array2().set_value(-1, 4, entity()).unwrap();
    assert_eq!(field.item_kind(-1, 2).unwrap(), Kind::UNDEFINED);
    assert_eq!(field.item_kind(-1, 3).unwrap(), Kind::STRING);
    assert_eq!(field.item_kind(-1, 4).unwrap(), Kind::ENTITY);
}

#[test]
fn two_dimensional_entity_array_uses_row_and_column_bounds() {
    let array = TransientGrid::with_bounds(-1, 0, 3, 5).unwrap();
    let held = entity();
    array.change_array2().set_value(0, 5, held.clone()).unwrap();
    let mut field = StepDataField::new();
    field.set(&Handle::new(array).cast());
    assert_eq!(field.lower(1), -1);
    assert_eq!(field.lower(2), 3);
    assert_eq!(field.length(1), 2);
    assert_eq!(field.length(2), 3);
    assert_eq!(field.item_kind(0, 5).unwrap(), Kind::ENTITY);
    assert!(field.entity(0, 5).unwrap() == held);
}

#[test]
fn item_kind_handles_any_kind_with_integer_arrays() {
    let mut list = StepDataField::new();
    list.clear(Kind::ANY);
    list.set(&Handle::new(HArray1::<i32>::with_bounds(1, 1).unwrap()).cast());
    assert_eq!(list.item_kind(1, 1).unwrap(), Kind::ANY);
    let mut grid = StepDataField::new();
    grid.clear(Kind::ANY);
    grid.set(&Handle::new(HArray2::<i32>::with_bounds(1, 1, 1, 1).unwrap()).cast());
    assert_eq!(grid.item_kind(1, 1).unwrap(), Kind::ANY);
}

#[test]
fn two_dimensional_aggregate_uses_row_and_column_bounds() {
    let array = HArray2::<f64>::with_bounds(-1, 0, 3, 5).unwrap();
    array.change_array2().set_value(-1, 3, 1.5).unwrap();
    array.change_array2().set_value(0, 5, 2.5).unwrap();
    let mut field = StepDataField::new();
    field.set(&Handle::new(array).cast());
    assert_eq!(field.kind(true), Kind::REAL);
    assert_eq!(field.arity(), 2);
    assert_eq!(field.lower(1), -1);
    assert_eq!(field.lower(2), 3);
    assert_eq!(field.length(1), 2);
    assert_eq!(field.length(2), 3);
    assert_eq!(field.real(-1, 3).unwrap(), 1.5);
    assert_eq!(field.real(0, 5).unwrap(), 2.5);
}

#[test]
fn two_dimensional_integer_array_preserves_values() {
    let array = HArray2::<i32>::with_bounds(-2, -1, 4, 5).unwrap();
    array.change_array2().set_value(-2, 4, 10).unwrap();
    array.change_array2().set_value(-1, 5, 20).unwrap();
    let mut field = StepDataField::new();
    field.set(&Handle::new(array).cast());
    assert_eq!(field.kind(true), Kind::INTEGER);
    assert_eq!(field.lower(1), -2);
    assert_eq!(field.lower(2), 4);
    assert_eq!(field.length(1), 2);
    assert_eq!(field.length(2), 2);
    assert_eq!(field.integer(-2, 4).unwrap(), 10);
    assert_eq!(field.integer(-1, 5).unwrap(), 20);
}

#[test]
fn two_dimensional_string_list_keeps_transient_representation() {
    let mut field = StepDataField::new();
    field.set_string(b"");
    field.set_list2(2, 3, -1, 4).unwrap();
    assert_eq!(field.kind(true), Kind::STRING);
    assert_eq!(field.lower(1), -1);
    assert_eq!(field.lower(2), 4);
    assert!(field.transient().cast::<TransientGrid>().is_some());
}

#[test]
fn copy_preserves_value_ownership() {
    let mut source = StepDataField::new();
    source.set_string(b"text");
    let mut copy = StepDataField::new();
    copy.copy_from(&source).unwrap();
    assert_eq!(copy.string(1, 1).unwrap().as_bytes(), b"text");
    assert!(copy.transient() != source.transient());
    let held = entity();
    let mut entity_field = StepDataField::new();
    entity_field.set_entity(&held);
    let mut entity_copy = StepDataField::new();
    entity_copy.copy_from(&entity_field).unwrap();
    assert!(entity_copy.entity(1, 1).unwrap() == held);
}

#[test]
fn copy_preserves_list_ownership() {
    let mut source = StepDataField::new();
    source.set_integer(0);
    source.set_list(2, -1).unwrap();
    source.set_integer_at(-1, 10).unwrap();
    source.set_integer_at(0, 20).unwrap();
    let mut copy = StepDataField::new();
    copy.copy_from(&source).unwrap();
    assert!(copy.transient() != source.transient());
    assert_eq!(copy.lower(1), -1);
    assert_eq!(copy.integer(-1, 1).unwrap(), 10);
    assert_eq!(copy.integer(0, 1).unwrap(), 20);
    source.set_integer_at(-1, 30).unwrap();
    assert_eq!(copy.integer(-1, 1).unwrap(), 10);
}

#[test]
fn copy_preserves_sparse_string_list() {
    let mut source = StepDataField::new();
    source.set_string(b"");
    source.set_list(2, -1).unwrap();
    source.set_string_at(0, b"text").unwrap();
    let mut copy = StepDataField::new();
    copy.copy_from(&source).unwrap();
    assert!(!copy.is_set(-1, 1).unwrap());
    assert!(copy.is_set(0, 1).unwrap());
    assert_eq!(copy.string(0, 1).unwrap().as_bytes(), b"text");
    assert!(copy.transient() != source.transient());
    source.set_string_at(0, b"changed").unwrap();
    assert_eq!(copy.string(0, 1).unwrap().as_bytes(), b"text");
}

#[test]
fn copy_preserves_select_values() {
    let select = Handle::new(StepDataSelectInt::default());
    select.set_kind(Kind::LOGICAL.to_int());
    select.set_int(StepDataLogical::LUnknown as i32);
    let mut source = StepDataField::new();
    source.set_select_member(&select.cast());
    let mut copy = StepDataField::new();
    copy.copy_from(&source).unwrap();
    let copied = copy.transient().cast::<StepDataSelectInt>();
    assert!(copied.is_some() && copied != select);
    assert_eq!(copied.kind(), Kind::LOGICAL.to_int());
    assert_eq!(copied.logical(), StepDataLogical::LUnknown);

    let real = Handle::new(StepDataSelectReal::default());
    real.set_real(4.5);
    let mut real_source = StepDataField::new();
    real_source.set_select_member(&real.cast());
    let mut real_copy = StepDataField::new();
    real_copy.copy_from(&real_source).unwrap();
    let copied = real_copy.transient().cast::<StepDataSelectReal>();
    assert!(copied.is_some() && copied != real);
    assert_eq!(copied.real(), 4.5);
}

#[test]
fn clear_item_clears_string_and_entity_lists() {
    let mut strings = StepDataField::new();
    strings.set_string(b"");
    strings.set_list(1, 1).unwrap();
    strings.set_string_at(1, b"text").unwrap();
    assert!(strings.is_set(1, 1).unwrap());
    strings.clear_item(1).unwrap();
    assert!(!strings.is_set(1, 1).unwrap());

    let mut entities = StepDataField::new();
    entities.set_entity(&Handle::default());
    entities.set_list(1, 1).unwrap();
    entities.set_entity_at(1, &entity()).unwrap();
    assert!(entities.is_set(1, 1).unwrap());
    entities.clear_item(1).unwrap();
    assert!(!entities.is_set(1, 1).unwrap());
}

#[test]
fn logical_select_preserves_value() {
    let select = Handle::new(StepDataSelectInt::default());
    let mut field = StepDataField::new();
    field.set_select_member(&select.cast());
    field.set_logical(StepDataLogical::LFalse);
    assert_eq!(select.kind(), Kind::LOGICAL.to_int());
    assert_eq!(select.logical(), StepDataLogical::LFalse);
    field.set_logical(StepDataLogical::LTrue);
    assert_eq!(select.logical(), StepDataLogical::LTrue);
    field.set_logical(StepDataLogical::LUnknown);
    assert_eq!(select.logical(), StepDataLogical::LUnknown);
}

#[test]
fn logical_list_preserves_all_values() {
    let mut field = StepDataField::new();
    field.set_logical(StepDataLogical::LFalse);
    field.set_list(3, -1).unwrap();
    field.set_logical_at(-1, StepDataLogical::LFalse).unwrap();
    field.set_logical_at(0, StepDataLogical::LTrue).unwrap();
    field.set_logical_at(1, StepDataLogical::LUnknown).unwrap();
    assert_eq!(field.logical(-1, 1).unwrap(), StepDataLogical::LFalse);
    assert_eq!(field.logical(0, 1).unwrap(), StepDataLogical::LTrue);
    assert_eq!(field.logical(1, 1).unwrap(), StepDataLogical::LUnknown);
}

#[test]
fn field_list_fill_shared_traverses_entity_aggregates() {
    let entities = [entity(), entity(), entity(), entity()];
    let mut fields = StepDataFieldListN::new(3).unwrap();
    fields.c_field(1).unwrap().set_entity(&entities[0]);
    let second = fields.c_field(2).unwrap();
    second.set_entity(&Handle::default());
    second.set_list(2, -1).unwrap();
    second.set_entity_at(-1, &entities[1]).unwrap();
    second.set_entity_at(0, &entities[2]).unwrap();
    let third = fields.c_field(3).unwrap();
    third.set_entity(&Handle::default());
    third.set_list2(1, 1, 2, -3).unwrap();
    let any = third.transient().cast::<TransientGrid>();
    let grid = any.get().expect("a grid of transients");
    grid.change_array2()
        .set_value(2, -3, entities[3].clone())
        .unwrap();
    let mut iterator = InterfaceEntityIterator::new();
    fields.fill_shared(&mut iterator).unwrap();
    assert_eq!(iterator.nb_entities(), 4);
    iterator.start();
    for held in &entities {
        assert!(iterator.value().unwrap() == *held);
        iterator.next();
    }
}

#[test]
fn field_list_n_validates_field_count() {
    assert_eq!(StepDataFieldListN::new(0).unwrap().nb_fields(), 0);
    assert_eq!(StepDataFieldListN::new(3).unwrap().nb_fields(), 3);
    let failure = StepDataFieldListN::new(-1).err().expect("a failure");
    assert_eq!(failure.exception_type(), "Standard_RangeError");
}

#[test]
fn simple_shared_traverses_fields_and_skips_null_entities() {
    let description = StepDataESDescr::new("TEST_ENTITY");
    description.set_nb_fields(2);
    let simple = StepDataSimple::new(&description);
    let scalar = entity();
    let in_array = entity();
    simple
        .change_field_num(1, |field| field.set_entity(&scalar))
        .unwrap();
    let array = Transients::with_bounds(-1, 0).unwrap();
    array
        .change_array1()
        .set_value(-1, in_array.clone())
        .unwrap();
    let array = Handle::new(array).cast();
    simple
        .change_field_num(2, |field| field.set(&array))
        .unwrap();
    let mut iterator = InterfaceEntityIterator::new();
    simple.shared(&mut iterator).unwrap();
    assert_eq!(iterator.nb_entities(), 2);
    iterator.start();
    assert!(iterator.value().unwrap() == scalar);
    iterator.next();
    assert!(iterator.value().unwrap() == in_array);
}

#[test]
fn p_descr_set_from_preserves_field_definition() {
    let source = StepDataPDescr::new();
    source.set_enum();
    source.add_enum_def(".FIRST.");
    source.add_enum_def(".SECOND.");
    source.set_arity(2);
    source.set_optional(true);
    source.set_derived(true);
    source.set_field("value", 3);
    let copy = StepDataPDescr::new();
    copy.set_from(&source);
    assert!(copy.is_enum());
    assert_eq!(copy.enum_max(), 1);
    assert_eq!(copy.enum_text(0).as_bytes(), b".FIRST.");
    assert_eq!(copy.enum_text(1).as_bytes(), b".SECOND.");
    assert_eq!(copy.arity(), 2);
    assert!(copy.is_optional());
    assert!(copy.is_derived());
    assert!(copy.is_field());
    assert_eq!(copy.field_name(), "value");
    assert_eq!(copy.field_rank(), 3);
}

#[test]
fn select_member_unsupported_kind_has_misc_parameter_type() {
    let select = StepDataSelectInt::default();
    select.set_kind(257);
    assert_eq!(select.param_type(), InterfaceParamType::ParamMisc);
}

#[test]
fn select_real_stores_real_value() {
    let select = StepDataSelectReal::default();
    select.set_real(3.25);
    assert_eq!(select.kind(), Kind::REAL.to_int());
    assert_eq!(select.real(), 3.25);
}

#[test]
fn select_named_delegates_value_to_field() {
    let select = StepDataSelectNamed::default();
    select.set_name("VALUE");
    select.set_kind(Kind::INTEGER.to_int());
    select.set_int(7);
    assert!(select.has_name());
    assert_eq!(select.name(), "VALUE");
    assert_eq!(select.kind(), Kind::INTEGER.to_int());
    assert_eq!(select.int(), 7);
    select.set_kind(261);
    assert_eq!(select.kind(), Kind::UNDEFINED.to_int());
}

#[test]
fn select_arr_real_preserves_array_bounds() {
    let array = HArray1::<f64>::with_bounds(-2, 0).unwrap();
    array.change_array1().set_value(-2, 1.0).unwrap();
    let select = StepDataSelectArrReal::default();
    select.set_arr_real(&Handle::new(array));
    assert_eq!(select.kind(), Kind::ANY.to_int());
    let held = select.arr_real();
    let live = held.get().expect("the array");
    assert_eq!(live.array1().lower(), -2);
    assert_eq!(*live.array1().value(-2).unwrap(), 1.0);
}
