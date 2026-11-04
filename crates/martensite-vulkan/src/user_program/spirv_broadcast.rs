//! GL's `gl_FragColor` broadcast for a user program's fragment module: a GLSL 1.10 shader that
//! writes `gl_FragColor` writes that one colour into every enabled draw buffer, so in a
//! multi-attachment pass (the depth-peeling targets) native writes the program's colour into
//! each of them. The Vulkan module writes `occFragColor` at location 0 only; this rewrite adds
//! one output per further attachment and copies the final location-0 colour into each before
//! every return of the entry point.
use crate::RenderError;
use crate::error::rejected;
#[path = "spirv_broadcast_validation.rs"]
mod validation;

const MAGIC: u32 = 0x0723_0203;
const HEADER_WORDS: usize = 5;
const OP_NAME: u16 = 5;
const OP_ENTRY_POINT: u16 = 15;
const OP_TYPE_VECTOR: u16 = 23;
const OP_TYPE_POINTER: u16 = 32;
const OP_FUNCTION: u16 = 54;
const OP_FUNCTION_END: u16 = 56;
const OP_VARIABLE: u16 = 59;
const OP_LOAD: u16 = 61;
const OP_STORE: u16 = 62;
const OP_DECORATE: u16 = 71;
const OP_RETURN: u16 = 253;
const DECORATION_LOCATION: u32 = 30;
const STORAGE_OUTPUT: u32 = 3;
const EXECUTION_MODEL_FRAGMENT: u32 = 4;

/// The ids the rewrite reads from the module: the location-0 output variable, its pointer and
/// vector types, and the fragment entry point.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ColorOutput {
    variable: u32,
    pointer: u32,
    vector: u32,
    entry: u32,
}

/// `words` with the location-0 colour also written to locations `1..attachments`; the module
/// is returned unchanged for one attachment. Example:
/// `broadcast_frag_color(&fragment_words, 3)?` for the three depth-peeling targets.
pub fn broadcast_frag_color(words: &[u32], attachments: u32) -> Result<Vec<u32>, RenderError> {
    if attachments <= 1 {
        return Ok(words.to_vec());
    }
    let spans = instruction_spans(words)?;
    validation::validate_instructions(words, &spans)?;
    let color = color_output(words, &spans)?;
    let final_bound = validation::expanded_bound(words, &spans, color.entry, attachments)?;
    let mut rewrite = Rewrite::new(words, color, attachments);
    for &(start, end) in &spans {
        rewrite.copy_instruction(&words[start..end]);
    }
    debug_assert_eq!(rewrite.next_id, final_bound);
    rewrite.out[3] = rewrite.next_id;
    Ok(rewrite.out)
}

/// `(start, end)` word ranges of every instruction after the header.
fn instruction_spans(words: &[u32]) -> Result<Vec<(usize, usize)>, RenderError> {
    if words.len() < HEADER_WORDS || words[0] != MAGIC {
        return Err(rejected(
            words.len(),
            "a SPIR-V module with its 5-word header",
        ));
    }
    let mut spans = Vec::new();
    let mut start = HEADER_WORDS;
    while start < words.len() {
        let end = instruction_end(words, start)?;
        spans.push((start, end));
        start = end;
    }
    Ok(spans)
}

fn instruction_end(words: &[u32], start: usize) -> Result<usize, RenderError> {
    let count = (words[start] >> 16) as usize;
    let end = start.checked_add(count);
    if count == 0 || end.is_none_or(|end| end > words.len()) {
        return Err(rejected(
            (start, count),
            "a SPIR-V instruction inside the module",
        ));
    }
    Ok(end.expect("validated instruction end"))
}

fn opcode(instruction: &[u32]) -> u16 {
    (instruction[0] & 0xffff) as u16
}

/// The location-0 output variable of the fragment entry point and its types.
fn color_output(words: &[u32], spans: &[(usize, usize)]) -> Result<ColorOutput, RenderError> {
    let variable = color_variable(words, spans)?;
    Ok(ColorOutput {
        variable: variable[2],
        pointer: variable[1],
        vector: color_vector(words, spans, variable[1])?,
        entry: fragment_entry(words, spans)?,
    })
}

fn color_variable<'a>(
    words: &'a [u32],
    spans: &[(usize, usize)],
) -> Result<&'a [u32], RenderError> {
    let all = || spans.iter().map(|&(start, end)| &words[start..end]);
    let at_zero: Vec<u32> = all()
        .filter(|i| opcode(i) == OP_DECORATE && i.len() == 4)
        .filter(|i| i[2] == DECORATION_LOCATION && i[3] == 0)
        .map(|i| i[1])
        .collect();
    all()
        .find(|i| opcode(i) == OP_VARIABLE && i[3] == STORAGE_OUTPUT && at_zero.contains(&i[2]))
        .ok_or_else(|| rejected("no location-0 output", "a fragment colour at location 0"))
}

fn color_vector(words: &[u32], spans: &[(usize, usize)], pointer: u32) -> Result<u32, RenderError> {
    let all = || spans.iter().map(|&(start, end)| &words[start..end]);
    let pointee = all()
        .find(|i| opcode(i) == OP_TYPE_POINTER && i[1] == pointer)
        .map(|i| i[3]);
    pointee
        .filter(|&ty| all().any(|i| opcode(i) == OP_TYPE_VECTOR && i[1] == ty))
        .ok_or_else(|| rejected(pointee, "a vector fragment colour at location 0"))
}

fn fragment_entry(words: &[u32], spans: &[(usize, usize)]) -> Result<u32, RenderError> {
    spans
        .iter()
        .map(|&(start, end)| &words[start..end])
        .find(|i| opcode(i) == OP_ENTRY_POINT && i[1] == EXECUTION_MODEL_FRAGMENT)
        .map(|i| i[2])
        .ok_or_else(|| rejected("no fragment entry point", "a fragment-stage module"))
}

/// The rewritten module under construction.
struct Rewrite {
    color: ColorOutput,
    /// The new output variables, for locations 1, 2, ...
    copies: Vec<u32>,
    next_id: u32,
    out: Vec<u32>,
    in_entry: bool,
}

impl Rewrite {
    fn new(words: &[u32], color: ColorOutput, attachments: u32) -> Self {
        Self {
            color,
            copies: (0..attachments - 1).map(|n| words[3] + n).collect(),
            next_id: words[3] + attachments - 1,
            out: words[..HEADER_WORDS].to_vec(),
            in_entry: false,
        }
    }

    /// Copy `instruction`, adding the broadcast declarations and stores it implies.
    fn copy_instruction(&mut self, instruction: &[u32]) {
        let op = opcode(instruction);
        if op == OP_RETURN && self.in_entry {
            self.store_copies();
        }
        if op == OP_ENTRY_POINT && instruction[2] == self.color.entry {
            self.entry_point(instruction);
            return;
        }
        self.out.extend_from_slice(instruction);
        self.follow(op, instruction);
    }
    /// The declarations that follow the colour variable's name, location and variable.
    fn follow(&mut self, op: u16, instruction: &[u32]) {
        let colour = instruction.get(1) == Some(&self.color.variable);
        match op {
            OP_NAME if colour => self.names(),
            OP_DECORATE if colour && instruction[2..] == [DECORATION_LOCATION, 0] => {
                self.locations()
            }
            OP_VARIABLE if instruction[2] == self.color.variable => self.variables(),
            OP_FUNCTION => self.in_entry = instruction[2] == self.color.entry,
            OP_FUNCTION_END => self.in_entry = false,
            _ => {}
        }
    }
    fn entry_point(&mut self, instruction: &[u32]) {
        let count = instruction.len() + self.copies.len();
        self.out
            .push(((count as u32) << 16) | u32::from(OP_ENTRY_POINT));
        self.out.extend_from_slice(&instruction[1..]);
        self.out.extend_from_slice(&self.copies);
    }
    fn names(&mut self) {
        for (index, &id) in self.copies.iter().enumerate() {
            let name = string_words(&format!("occFragColor{}", index + 1));
            self.out
                .push((((2 + name.len()) as u32) << 16) | u32::from(OP_NAME));
            self.out.push(id);
            self.out.extend(name);
        }
    }
    fn locations(&mut self) {
        for (index, &id) in self.copies.iter().enumerate() {
            let location = index as u32 + 1;
            self.out.extend([
                (4 << 16) | u32::from(OP_DECORATE),
                id,
                DECORATION_LOCATION,
                location,
            ]);
        }
    }
    fn variables(&mut self) {
        for &id in &self.copies {
            let pointer = self.color.pointer;
            self.out.extend([
                (4 << 16) | u32::from(OP_VARIABLE),
                pointer,
                id,
                STORAGE_OUTPUT,
            ]);
        }
    }
    /// `%c = OpLoad %vec4 %occFragColor; OpStore %copy %c` per copy, before a return.
    fn store_copies(&mut self) {
        let value = self.next_id;
        self.next_id += 1;
        let (vector, colour) = (self.color.vector, self.color.variable);
        self.out
            .extend([(4 << 16) | u32::from(OP_LOAD), vector, value, colour]);
        for &id in &self.copies {
            self.out
                .extend([(3 << 16) | u32::from(OP_STORE), id, value]);
        }
    }
}

/// A SPIR-V literal string: UTF-8, nul-terminated, padded to whole little-endian words.
fn string_words(text: &str) -> Vec<u32> {
    let mut bytes = text.as_bytes().to_vec();
    bytes.push(0);
    bytes.resize(bytes.len().div_ceil(4) * 4, 0);
    let (words, _) = bytes.as_chunks::<4>();
    words.iter().map(|&word| u32::from_le_bytes(word)).collect()
}
