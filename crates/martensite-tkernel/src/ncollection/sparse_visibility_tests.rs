// Sparse array and helper contracts run with access to TKernel-private methods.
#[path = "../../tests/aliased_array_exception_contracts.rs"]
mod aliased_array_exception_contracts;

#[path = "../../tests/aliased_array_method_contracts.rs"]
mod aliased_array_method_contracts;

#[path = "../../tests/aliased_array_stride_narrowing.rs"]
mod aliased_array_stride_narrowing;

#[path = "../../tests/aliased_array_supplemental.rs"]
mod aliased_array_supplemental;

#[path = "../../tests/cell_filter_cell_methods.rs"]
mod cell_filter_cell_methods;

#[path = "../../tests/cell_filter_fallible.rs"]
mod cell_filter_fallible;

#[path = "../../tests/cell_filter_method_contracts.rs"]
mod cell_filter_method_contracts;

#[path = "../../tests/cell_filter_protected_helpers.rs"]
mod cell_filter_protected_helpers;

#[path = "../../tests/cell_filter_supplemental.rs"]
mod cell_filter_supplemental;

#[path = "../../tests/forward_range_method_contracts.rs"]
mod forward_range_method_contracts;

#[path = "../../tests/forward_range_supplemental.rs"]
mod forward_range_supplemental;

#[path = "../../tests/items_view_method_contracts.rs"]
mod items_view_method_contracts;

#[path = "../../tests/ncollection_aliased_array.rs"]
mod ncollection_aliased_array;

#[path = "../../tests/ncollection_cell_filter.rs"]
mod ncollection_cell_filter;

#[path = "../../tests/ncollection_forward_range.rs"]
mod ncollection_forward_range;

#[path = "../../tests/ncollection_sparse_array.rs"]
mod ncollection_sparse_array;

#[path = "../../tests/sparse_array_method_contracts.rs"]
mod sparse_array_method_contracts;

#[path = "../../tests/forward_native_access_contracts.rs"]
mod forward_native_access_contracts;

#[path = "../../tests/forward_native_arrow_contracts.rs"]
mod forward_native_arrow_contracts;

#[allow(clippy::module_inception)]
#[path = "../../tests/sparse_typed_storage_contracts.rs"]
mod sparse_typed_storage_contracts;
