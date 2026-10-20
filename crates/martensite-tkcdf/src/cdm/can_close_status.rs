/// `CDM_CanCloseStatus`: why a document can or cannot be closed, in native order;
/// `CdmCanCloseStatus::Ok as i32 == 0`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CdmCanCloseStatus {
    /// `CDM_CCS_OK`.
    Ok,
    /// `CDM_CCS_NotOpen`.
    NotOpen,
    /// `CDM_CCS_UnstoredReferenced`.
    UnstoredReferenced,
    /// `CDM_CCS_ModifiedReferenced`.
    ModifiedReferenced,
    /// `CDM_CCS_ReferenceRejection`.
    ReferenceRejection,
}
