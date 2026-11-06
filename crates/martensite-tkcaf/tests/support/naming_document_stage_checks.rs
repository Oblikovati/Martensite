//! Independent lifecycle controls for the ordered document preparation boundary.
use super::naming_draw_document::NamingDocumentCase;

#[test]
fn prepared_document_has_no_undo_command_until_begin_stage() {
    let prepared = NamingDocumentCase::prepare_driver();
    let identity = prepared.document.clone();
    assert_eq!(prepared.document.get_undo_limit(), 0);
    assert!(!prepared.document.has_open_command());
    let active = prepared.begin_driver();
    assert!(active.document == identity);
    assert_eq!(active.document.get_undo_limit(), 100);
    assert!(active.document.has_open_command());
    active.close();
    assert!(!identity.has_open_command());
}

#[test]
fn legacy_driver_and_standalone_keep_distinct_transaction_settings() {
    let driver = NamingDocumentCase::driver();
    let standalone = NamingDocumentCase::standalone();
    assert_eq!(driver.document.get_undo_limit(), 100);
    assert!(driver.document.has_open_command());
    assert_eq!(standalone.document.get_undo_limit(), 0);
    assert!(!standalone.document.has_open_command());
    assert!(driver.document != standalone.document);
    driver.close();
    standalone.close();
}
