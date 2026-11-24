// Array and vector contracts run with access to TKernel-private methods.
#[path = "../../tests/array1_assignment_pointer_contracts.rs"]
mod array1_assignment_pointer_contracts;

#[path = "../../tests/array1_borrowed_init.rs"]
mod array1_borrowed_init;

#[path = "../../tests/array2_assignment_pointer_contracts.rs"]
mod array2_assignment_pointer_contracts;

#[path = "../../tests/array2_failure_mapping.rs"]
mod array2_failure_mapping;

#[path = "../../tests/array2_index_limits.rs"]
mod array2_index_limits;

#[path = "../../tests/array2_resize_assignment.rs"]
mod array2_resize_assignment;

#[path = "../../tests/array2_resize_validation.rs"]
mod array2_resize_validation;

#[allow(clippy::module_inception)]
#[path = "../../tests/dynamic_array_borrowed_methods.rs"]
mod dynamic_array_borrowed_methods;

#[allow(clippy::module_inception)]
#[path = "../../tests/dynamic_array_cached_assignment.rs"]
mod dynamic_array_cached_assignment;

#[allow(clippy::module_inception)]
#[path = "../../tests/dynamic_array_cached_iterator.rs"]
mod dynamic_array_cached_iterator;

#[allow(clippy::module_inception)]
#[path = "../../tests/dynamic_array_cached_operations.rs"]
mod dynamic_array_cached_operations;

#[path = "../../tests/dynamic_array_clear_state.rs"]
mod dynamic_array_clear_state;

#[allow(clippy::module_inception)]
#[path = "../../tests/dynamic_array_copy_date.rs"]
mod dynamic_array_copy_date;

#[path = "../../tests/dynamic_array_default_constructor_order.rs"]
mod dynamic_array_default_constructor_order;

#[path = "../../tests/dynamic_array_emplace_allocation_order.rs"]
mod dynamic_array_emplace_allocation_order;

#[path = "../../tests/dynamic_array_emplace_value_order.rs"]
mod dynamic_array_emplace_value_order;

#[path = "../../tests/dynamic_array_erase_last_phase.rs"]
mod dynamic_array_erase_last_phase;

#[path = "../../tests/dynamic_array_increment_overflow.rs"]
mod dynamic_array_increment_overflow;

#[path = "../../tests/dynamic_array_internal_reconstruction.rs"]
mod dynamic_array_internal_reconstruction;

#[path = "../../tests/dynamic_array_iterator_tail.rs"]
mod dynamic_array_iterator_tail;

#[path = "../../tests/dynamic_array_legacy_cursor_contracts.rs"]
mod dynamic_array_legacy_cursor_contracts;

#[allow(clippy::module_inception)]
#[path = "../../tests/dynamic_array_method_tail.rs"]
mod dynamic_array_method_tail;

#[path = "../../tests/dynamic_array_move_constructor.rs"]
mod dynamic_array_move_constructor;

#[allow(clippy::module_inception)]
#[path = "../../tests/dynamic_array_native_insertion.rs"]
mod dynamic_array_native_insertion;

#[allow(clippy::module_inception)]
#[path = "../../tests/dynamic_array_protected_storage.rs"]
mod dynamic_array_protected_storage;

#[allow(clippy::module_inception)]
#[path = "../../tests/dynamic_array_public_copy.rs"]
mod dynamic_array_public_copy;

#[allow(clippy::module_inception)]
#[path = "../../tests/dynamic_array_raw_lifetime.rs"]
mod dynamic_array_raw_lifetime;

#[allow(clippy::module_inception)]
#[path = "../../tests/dynamic_array_replacement_policy.rs"]
mod dynamic_array_replacement_policy;

#[allow(clippy::module_inception)]
#[path = "../../tests/dynamic_array_retained_parameters.rs"]
mod dynamic_array_retained_parameters;

#[allow(clippy::module_inception)]
#[path = "../../tests/dynamic_array_retained_zero_stride.rs"]
mod dynamic_array_retained_zero_stride;

#[path = "../../tests/dynamic_array_storage_wrapping.rs"]
mod dynamic_array_storage_wrapping;

#[path = "../../tests/dynamic_array_zero_first_replacement.rs"]
mod dynamic_array_zero_first_replacement;

#[allow(clippy::duplicate_mod)]
#[path = "../../tests/harray1_borrowed_initialization.rs"]
mod harray1_borrowed_initialization;

#[allow(clippy::duplicate_mod)]
#[path = "../../tests/harray1_trivial_copy.rs"]
mod harray1_trivial_copy;

#[allow(clippy::duplicate_mod)]
#[path = "../../tests/harray2_borrowed_initialization.rs"]
mod harray2_borrowed_initialization;

#[allow(clippy::duplicate_mod)]
#[path = "../../tests/harray2_trivial_copy.rs"]
mod harray2_trivial_copy;

#[path = "../../tests/linear_vector_append_policy.rs"]
mod linear_vector_append_policy;

#[path = "../../tests/linear_vector_assignment_pointer_contracts.rs"]
mod linear_vector_assignment_pointer_contracts;

#[path = "../../tests/linear_vector_assignment_tail.rs"]
mod linear_vector_assignment_tail;

#[path = "../../tests/linear_vector_copy_constructor.rs"]
mod linear_vector_copy_constructor;

#[path = "../../tests/linear_vector_erase_last_methods.rs"]
mod linear_vector_erase_last_methods;

#[path = "../../tests/linear_vector_erase_methods.rs"]
mod linear_vector_erase_methods;

#[path = "../../tests/linear_vector_fill_constructor.rs"]
mod linear_vector_fill_constructor;

#[path = "../../tests/linear_vector_growth_ownership.rs"]
mod linear_vector_growth_ownership;

#[path = "../../tests/linear_vector_insert_methods.rs"]
mod linear_vector_insert_methods;

#[path = "../../tests/linear_vector_move_methods.rs"]
mod linear_vector_move_methods;

#[path = "../../tests/linear_vector_native_tail.rs"]
mod linear_vector_native_tail;

#[path = "../../tests/linear_vector_relocation_methods.rs"]
mod linear_vector_relocation_methods;

#[allow(clippy::duplicate_mod)]
#[path = "../../tests/linear_vector_resize_policy.rs"]
mod linear_vector_resize_policy;

#[allow(clippy::duplicate_mod)]
#[path = "../../tests/linear_vector_set_value_policy.rs"]
mod linear_vector_set_value_policy;

#[path = "../../tests/local_array_nontrivial_methods.rs"]
mod local_array_nontrivial_methods;

#[path = "../../tests/local_array_shared_view.rs"]
mod local_array_shared_view;

#[path = "../../tests/local_array_trivial_move_contracts.rs"]
mod local_array_trivial_move_contracts;

#[path = "../../tests/ncollection_array1.rs"]
mod ncollection_array1;

#[path = "../../tests/ncollection_array2.rs"]
mod ncollection_array2;

#[path = "../../tests/ncollection_array2_buffer_prefix.rs"]
mod ncollection_array2_buffer_prefix;

#[path = "../../tests/ncollection_dynamic_array.rs"]
mod ncollection_dynamic_array;

#[path = "../../tests/ncollection_dynamic_array_assignment_behavior.rs"]
mod ncollection_dynamic_array_assignment_behavior;

#[path = "../../tests/ncollection_linear_vector.rs"]
mod ncollection_linear_vector;

#[allow(clippy::duplicate_mod)]
#[path = "../../tests/ncollection_linear_vector_exception_state.rs"]
mod ncollection_linear_vector_exception_state;

#[path = "../../tests/ncollection_local_array.rs"]
mod ncollection_local_array;
