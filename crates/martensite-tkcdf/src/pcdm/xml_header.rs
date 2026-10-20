//! The XML format probe stops at the first document start tag, before its body.
use crate::ldom::{LdomElement, LdomParser, LdomParserHooks, LdomString};
use martensite_tkernel::tcollection::ExtendedString;
use std::{
    io::Read,
    sync::{Arc, OnceLock},
};

struct DocumentStart(Arc<OnceLock<ExtendedString>>);

impl LdomParserHooks for DocumentStart {
    fn start_element(&mut self, current: &LdomElement) -> bool {
        if !current.get_tag_name().equals(&xml_name(b"document")) {
            return false;
        }
        // Direct LDOMString assignment widens bytes/decodes ##feff; it does not
        // impose UTF-8 conversion or publish the thread-confined DOM arena.
        let _ = self.0.set(
            current
                .get_attribute(&xml_name(b"format"))
                .to_extended_string(),
        );
        true
    }
}

fn xml_name(name: &[u8]) -> LdomString {
    LdomString::from_c_str(Some(name))
}

pub(super) fn file_format(file_name: &ExtendedString) -> ExtendedString {
    document_format(|parser| parser.parse_file(file_name.to_utf8().as_bytes()))
}

/// `TryXmlDriverType(theIStream)` after the declaration: one tag per read step, so the
/// input is consumed only up to the document start tag.
pub(super) fn stream_format(input: &mut dyn Read) -> ExtendedString {
    document_format(|parser| parser.parse(input, true, false))
}

/// Only an aborted parse (true) can have stopped at the document start tag.
fn document_format(parse: impl FnOnce(&mut LdomParser) -> bool) -> ExtendedString {
    let found = Arc::new(OnceLock::new());
    let mut parser = LdomParser::with_hooks(Box::new(DocumentStart(found.clone())));
    if !parse(&mut parser) {
        return ExtendedString::default();
    }
    found.get().cloned().unwrap_or_default()
}
