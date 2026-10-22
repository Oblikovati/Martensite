//! The message keys of `martensite_object_model_DocumentApplication::SetError` for the store and reader statuses.
use martensite_tkcdf::pcdm::{PcdmReaderStatus, PcdmStoreStatus};

/// The key of `SetError(PCDM_StoreStatus, ...)`.
pub(crate) fn store_error_key(status: PcdmStoreStatus) -> &'static str {
    match status {
        PcdmStoreStatus::DriverFailure => "ObjectModel_Appl_SDriverFailure",
        PcdmStoreStatus::WriteFailure => "ObjectModel_Appl_SWriteFailure",
        PcdmStoreStatus::Failure => "ObjectModel_Appl_SFailure",
        PcdmStoreStatus::DocIsNull => "ObjectModel_Appl_SDocIsNull",
        PcdmStoreStatus::NoObj => "ObjectModel_Appl_SNoObj",
        PcdmStoreStatus::InfoSectionError => "ObjectModel_Appl_SInfoSectionError",
        _ => "ObjectModel_Appl_SUnknownFailure",
    }
}

/// The key of `SetError(PCDM_ReaderStatus, ...)`.
pub(crate) fn reader_error_key(status: PcdmReaderStatus) -> &'static str {
    match status {
        PcdmReaderStatus::UnknownDocument => "ObjectModel_Appl_RUnknownDocument",
        PcdmReaderStatus::AlreadyRetrieved => "ObjectModel_Appl_RAlreadyRetrieved",
        PcdmReaderStatus::AlreadyRetrievedAndModified => {
            "ObjectModel_Appl_RAlreadyRetrievedAndModified"
        }
        PcdmReaderStatus::NoDriver | PcdmReaderStatus::UnknownFileDriver => {
            "ObjectModel_Appl_RNoDriver"
        }
        PcdmReaderStatus::OpenError => "ObjectModel_Appl_ROpenError",
        PcdmReaderStatus::NoVersion => "ObjectModel_Appl_RNoVersion",
        PcdmReaderStatus::NoModel => "ObjectModel_Appl_RNoModel",
        PcdmReaderStatus::NoDocument => "ObjectModel_Appl_RNoDocument",
        PcdmReaderStatus::FormatFailure => "ObjectModel_Appl_RFormatFailure",
        PcdmReaderStatus::TypeNotFoundInSchema => "ObjectModel_Appl_RTypeNotFound",
        PcdmReaderStatus::UnrecognizedFileFormat => "ObjectModel_Appl_RBadFileFormat",
        PcdmReaderStatus::MakeFailure => "ObjectModel_Appl_RMakeFailure",
        PcdmReaderStatus::PermissionDenied => "ObjectModel_Appl_RPermissionDenied",
        PcdmReaderStatus::DriverFailure => "ObjectModel_Appl_RDriverFailure",
        PcdmReaderStatus::ReaderException => "ObjectModel_Appl_RException",
        _ => "ObjectModel_Appl_RUnknownFail",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_status_has_the_runtime_key() {
        assert_eq!(
            store_error_key(PcdmStoreStatus::UserBreak),
            "ObjectModel_Appl_SUnknownFailure"
        );
        assert_eq!(
            store_error_key(PcdmStoreStatus::NoObj),
            "ObjectModel_Appl_SNoObj"
        );
        assert_eq!(
            reader_error_key(PcdmReaderStatus::UnknownFileDriver),
            "ObjectModel_Appl_RNoDriver"
        );
        assert_eq!(
            reader_error_key(PcdmReaderStatus::NoSchema),
            "ObjectModel_Appl_RUnknownFail"
        );
        assert_eq!(
            reader_error_key(PcdmReaderStatus::ReaderException),
            "ObjectModel_Appl_RException"
        );
    }
}
