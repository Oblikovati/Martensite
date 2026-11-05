use super::{BOPAlgoPaveFiller, SectionAttribute};
use std::fmt::Write;

#[test]
fn copied_section_attributes_survive_source_mutation_and_clear() {
    let mut filler = BOPAlgoPaveFiller::new();
    let mut rows = format!("filler-default {}\n", mask(&filler));
    let mut flags = SectionAttribute::with_flags(true, false, true);
    filler.set_section_attribute(&flags);
    flags.set_approximation(false);
    writeln!(rows, "filler-copy {}", mask(&filler)).unwrap();
    filler.clear();
    writeln!(rows, "filler-clear {}", mask(&filler)).unwrap();
    let native = include_str!("../../../tests/fixtures/recorded/tkbo_section_attribute.expected");
    assert_eq!(rows, &native[native.find("filler-default").unwrap()..]);
}

use super::owned_progress_fixture::section_progress_attribute_mask as mask;
