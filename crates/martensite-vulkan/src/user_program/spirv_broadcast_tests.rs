//! The gl_FragColor broadcast rewrite of the command-host uniforms program's fragment module
//! (tests/fixtures/user_programs/spirv, compiled by the recording tool).
use super::spirv_broadcast::broadcast_frag_color;
use super::spirv_reflect::reflect_stage;

const FRAGMENT: &[u8] =
    include_bytes!("../../tests/fixtures/user_programs/spirv/ced4d8ee5ebda03a.frag.spv");
const VERTEX: &[u8] =
    include_bytes!("../../tests/fixtures/user_programs/spirv/ced4d8ee5ebda03a.vert.spv");

fn words(bytes: &[u8]) -> Vec<u32> {
    let (words, _) = bytes.as_chunks::<4>();
    words.iter().map(|&word| u32::from_le_bytes(word)).collect()
}

/// Every `(opcode, word count)` of `module` after the header.
fn opcodes(module: &[u32]) -> Vec<u32> {
    let mut found = Vec::new();
    let mut at = 5;
    while at < module.len() {
        found.push(module[at] & 0xffff);
        at += (module[at] >> 16) as usize;
    }
    found
}

#[test]
fn three_attachments_add_two_named_outputs_at_locations_one_and_two() {
    let original = words(FRAGMENT);
    let rewritten = broadcast_frag_color(&original, 3).unwrap();
    let outputs = reflect_stage(&rewritten).unwrap().outputs;
    let expected = [
        ("occFragColor", 0),
        ("occFragColor1", 1),
        ("occFragColor2", 2),
    ];
    let expected = expected.map(|(name, location)| (name.to_owned(), location));
    assert_eq!(outputs.into_iter().collect::<Vec<_>>(), expected.to_vec());
    // Two new variables and one load per return of main.
    assert_eq!(rewritten[3], original[3] + 3);
}

#[test]
fn every_return_of_main_stores_the_final_colour_into_each_copy() {
    let rewritten = broadcast_frag_color(&words(FRAGMENT), 3).unwrap();
    let ops = opcodes(&rewritten);
    // main's one return follows a load and two stores; occSetFragColor's return does not.
    let copied = ops.windows(4).filter(|w| *w == [61, 62, 62, 253]).count();
    assert_eq!(copied, 1);
    assert_eq!(ops.iter().filter(|&&op| op == 253).count(), 2);
    let stores = ops.iter().filter(|&&op| op == 62).count();
    let original_stores = opcodes(&words(FRAGMENT))
        .iter()
        .filter(|&&op| op == 62)
        .count();
    assert_eq!(stores, original_stores + 2);
}

#[test]
fn one_attachment_keeps_the_module_and_a_vertex_module_is_refused() {
    let original = words(FRAGMENT);
    assert_eq!(broadcast_frag_color(&original, 1).unwrap(), original);
    let error = broadcast_frag_color(&words(VERTEX), 3).unwrap_err();
    assert!(format!("{error:?}").contains("fragment"), "{error:?}");
    let error = broadcast_frag_color(&[1, 2, 3], 2).unwrap_err();
    assert!(format!("{error:?}").contains("5-word header"), "{error:?}");
}

fn shorten_instruction(module: &mut Vec<u32>, opcode: u32, count: usize) {
    let mut at = 5;
    while at < module.len() {
        let previous = (module[at] >> 16) as usize;
        if module[at] & 0xffff == opcode {
            module[at] = ((count as u32) << 16) | opcode;
            module.drain(at + count..at + previous);
            return;
        }
        at += previous;
    }
    panic!("opcode {opcode}; expected an instruction in the fragment fixture");
}

#[test]
fn short_function_operands_return_an_error_without_indexing_the_missing_id() {
    let mut module = words(FRAGMENT);
    shorten_instruction(&mut module, 54, 1);
    assert!(reflect_stage(&module).is_ok());
    assert!(broadcast_frag_color(&module, 3).is_err());
}

#[test]
fn short_entry_point_operands_return_a_typed_error() {
    let mut module = words(FRAGMENT);
    shorten_instruction(&mut module, 15, 2);
    assert!(broadcast_frag_color(&module, 3).is_err());
}

#[test]
fn short_output_variable_operands_return_a_typed_error() {
    let mut module = words(FRAGMENT);
    shorten_instruction(&mut module, 59, 3);
    assert!(broadcast_frag_color(&module, 3).is_err());
}

#[test]
fn short_pointer_and_vector_types_return_a_typed_error() {
    for opcode in [32, 23] {
        let mut module = words(FRAGMENT);
        shorten_instruction(&mut module, opcode, 2);
        assert!(broadcast_frag_color(&module, 3).is_err());
    }
}

#[test]
fn exhausted_id_bounds_and_attachment_counts_are_rejected() {
    let mut module = words(FRAGMENT);
    for bound in [0, 1, u32::MAX] {
        module[3] = bound;
        assert!(broadcast_frag_color(&module, 3).is_err(), "bound={bound}");
    }
    assert!(broadcast_frag_color(&words(FRAGMENT), u32::MAX).is_err());
}
