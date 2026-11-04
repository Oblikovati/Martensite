//! Validate every operand the rewrite reads before allocating IDs or indexing instructions.
use super::*;

// The installed SPIRV-Tools validator uses this ID budget (libspirv.h kDefaultMaxIdBound).
const MAX_ID_BOUND: u32 = 0x3f_ffff;

pub(super) fn validate_instructions(
    words: &[u32],
    spans: &[(usize, usize)],
) -> Result<(), RenderError> {
    let bound = words[3];
    if bound == 0 || bound > MAX_ID_BOUND {
        return Err(rejected(
            bound,
            "a nonzero SPIR-V ID bound within the validator ID budget",
        ));
    }
    for &(start, end) in spans {
        let instruction = &words[start..end];
        validate_arity(instruction)?;
        validate_ids(instruction, bound)?;
    }
    Ok(())
}

fn validate_arity(instruction: &[u32]) -> Result<(), RenderError> {
    let count = instruction.len();
    let valid = match opcode(instruction) {
        OP_NAME => count >= 3,
        OP_ENTRY_POINT => count >= 4,
        OP_TYPE_VECTOR | OP_TYPE_POINTER => count == 4,
        OP_FUNCTION => count == 5,
        OP_VARIABLE => matches!(count, 4 | 5),
        OP_DECORATE => count >= 3 && (instruction[2] != DECORATION_LOCATION || count == 4),
        OP_FUNCTION_END | OP_RETURN => count == 1,
        _ => true,
    };
    if valid {
        return Ok(());
    }
    Err(rejected(
        instruction,
        "complete operands for the SPIR-V instruction opcode",
    ))
}

fn validate_ids(instruction: &[u32], bound: u32) -> Result<(), RenderError> {
    let positions: &[usize] = match opcode(instruction) {
        OP_NAME | OP_DECORATE => &[1],
        OP_ENTRY_POINT => &[2],
        OP_TYPE_VECTOR => &[1, 2],
        OP_TYPE_POINTER => &[1, 3],
        OP_FUNCTION => &[1, 2, 4],
        OP_VARIABLE if instruction.len() == 5 => &[1, 2, 4],
        OP_VARIABLE => &[1, 2],
        _ => &[],
    };
    for &position in positions {
        let id = instruction[position];
        if id == 0 || id >= bound {
            return Err(rejected(
                (opcode(instruction), position, id, bound),
                "a SPIR-V operand ID in 1..bound",
            ));
        }
    }
    Ok(())
}

pub(super) fn expanded_bound(
    words: &[u32],
    spans: &[(usize, usize)],
    entry: u32,
    attachments: u32,
) -> Result<u32, RenderError> {
    let copies = attachments - 1;
    let returns = entry_return_count(words, spans, entry);
    let bound = words[3]
        .checked_add(copies)
        .and_then(|id| id.checked_add(returns));
    let valid = bound.filter(|&id| id <= MAX_ID_BOUND);
    validate_entry_lengths(words, spans, entry, copies)?;
    valid.ok_or_else(|| {
        rejected(
            (words[3], copies, returns),
            "new IDs within the validator ID budget",
        )
    })
}

fn validate_entry_lengths(
    words: &[u32],
    spans: &[(usize, usize)],
    entry: u32,
    copies: u32,
) -> Result<(), RenderError> {
    for &(start, end) in spans {
        let instruction = &words[start..end];
        if opcode(instruction) != OP_ENTRY_POINT || instruction[2] != entry {
            continue;
        }
        let size = instruction.len().checked_add(copies as usize);
        if size.is_none_or(|size| size > u16::MAX as usize) {
            return Err(rejected(
                copies,
                "broadcast outputs fitting an OpEntryPoint word count",
            ));
        }
    }
    Ok(())
}

fn entry_return_count(words: &[u32], spans: &[(usize, usize)], entry: u32) -> u32 {
    let mut in_entry = false;
    let mut count = 0_u32;
    for &(start, end) in spans {
        let instruction = &words[start..end];
        match opcode(instruction) {
            OP_FUNCTION => in_entry = instruction[2] == entry,
            OP_FUNCTION_END => in_entry = false,
            OP_RETURN if in_entry => count = count.saturating_add(1),
            _ => {}
        }
    }
    count
}
