//! The list of the generated tables.
use super::protocol::ProtocolRow;

/// All parts of the table.
pub(super) const PROTOCOL_TYPES_PARTS: &[&[ProtocolRow]] = &[
    super::protocol_types_01::PROTOCOL_TYPES_01,
    super::protocol_types_02::PROTOCOL_TYPES_02,
    super::protocol_types_03::PROTOCOL_TYPES_03,
];
