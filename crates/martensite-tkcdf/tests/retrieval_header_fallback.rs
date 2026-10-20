//! Unknown-format fallback values are independent constants, not decoded native headers.
use martensite_tkcdf::pcdm::{PcdmReadWriterBase, PcdmRetrievalDriverBase};
use martensite_tkernel::{message::Messenger, tcollection::ExtendedString};
use std::sync::Arc;

#[test]
fn unknown_format_fallback_has_no_format_or_references_and_no_document_version() {
    let messenger = Arc::new(Messenger::new());
    for path in ["", "not-a-native-document.unknown", "owned/nothing"] {
        let file_name = ExtendedString::new(path);
        assert!(PcdmReadWriterBase::file_format(&file_name).is_empty());
        assert_eq!(
            PcdmRetrievalDriverBase::reference_counter(&file_name, &messenger),
            0
        );
        assert_eq!(
            PcdmRetrievalDriverBase::document_version(&file_name, &messenger),
            -1
        );
    }
}
