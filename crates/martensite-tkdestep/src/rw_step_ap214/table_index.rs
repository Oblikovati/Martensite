//! The list of the generated tables.
use super::table_category_01;
use super::table_category_02;
use super::table_category_03;
use super::table_check_01;
use super::table_new_01;
use super::table_new_02;
use super::table_new_03;
use super::table_read_01;
use super::table_read_02;
use super::table_read_03;
use super::table_share_01;
use super::table_share_02;
use super::table_share_03;
use super::table_step_type_01;
use super::table_step_type_02;
use super::table_step_type_03;
use super::table_typenums_01;
use super::table_typenums_02;
use super::table_typenums_03;
use super::table_typeshor_01;
use super::table_typeshor_02;
use super::table_write_01;
use super::table_write_02;
use super::table_write_03;
use super::tables::*;
/// All parts of the table.
pub(super) const TABLE_READ_PARTS: &[&[ReadRow]] = &[
    table_read_01::TABLE_READ_01,
    table_read_02::TABLE_READ_02,
    table_read_03::TABLE_READ_03,
];

/// All parts of the table.
pub(super) const TABLE_WRITE_PARTS: &[&[WriteRow]] = &[
    table_write_01::TABLE_WRITE_01,
    table_write_02::TABLE_WRITE_02,
    table_write_03::TABLE_WRITE_03,
];

/// All parts of the table.
pub(super) const TABLE_SHARE_PARTS: &[&[ShareRow]] = &[
    table_share_01::TABLE_SHARE_01,
    table_share_02::TABLE_SHARE_02,
    table_share_03::TABLE_SHARE_03,
];

/// All parts of the table.
pub(super) const TABLE_CHECK_PARTS: &[&[CheckRow]] = &[table_check_01::TABLE_CHECK_01];

/// All parts of the table.
pub(super) const TABLE_NEW_PARTS: &[&[NewRow]] = &[
    table_new_01::TABLE_NEW_01,
    table_new_02::TABLE_NEW_02,
    table_new_03::TABLE_NEW_03,
];

/// All parts of the table.
pub(super) const TABLE_CATEGORY_PARTS: &[&[CategoryRow]] = &[
    table_category_01::TABLE_CATEGORY_01,
    table_category_02::TABLE_CATEGORY_02,
    table_category_03::TABLE_CATEGORY_03,
];

/// All parts of the table.
pub(super) const TABLE_STEP_TYPE_PARTS: &[&[(i32, &str)]] = &[
    table_step_type_01::TABLE_STEP_TYPE_01,
    table_step_type_02::TABLE_STEP_TYPE_02,
    table_step_type_03::TABLE_STEP_TYPE_03,
];

/// All parts of the table.
pub(super) const TABLE_TYPENUMS_PARTS: &[&[(&str, i32)]] = &[
    table_typenums_01::TABLE_TYPENUMS_01,
    table_typenums_02::TABLE_TYPENUMS_02,
    table_typenums_03::TABLE_TYPENUMS_03,
];

/// All parts of the table.
pub(super) const TABLE_TYPESHOR_PARTS: &[&[(&str, i32)]] = &[
    table_typeshor_01::TABLE_TYPESHOR_01,
    table_typeshor_02::TABLE_TYPESHOR_02,
];
