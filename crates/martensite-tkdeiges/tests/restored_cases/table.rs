//! The table types of the restored cases: data only, no API call.

/// What the reference run ended in.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RecordedVerdict {
    Ok,
    /// `BAD (known problem)`: every problem is declared by a `TODO`.
    Known,
    Failed,
}

/// A `de/iges_*` script: data for the one procedure the grid's `end` is.
pub struct DeRow {
    pub source: &'static str,
    pub file: &'static str,
    /// The case's own `puts "TODO ..."` lines, in order.
    pub todos: &'static [&'static str],
    /// `ref_data`, line by line, as `de/end` compares it: the first line is the empty one after the brace.
    pub reference: &'static [&'static str],
    pub recorded: RecordedVerdict,
}

/// What the grid's `end` adds to the first document before it writes it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Added {
    /// `set AddToDocument "ASSEMBLY_C_L"`: an assembly of boxes with its colours and layers.
    AssemblyColorsLayers,
    /// `set AddToDocument "COLORS_LAYERS"`: colours and layers on what the document holds.
    ColorsLayers,
}

/// An `xcaf/*_igs*` script: data for the one procedure `xcaf/end` is, on `TypeOfFile igs`.
pub struct XcafRow {
    pub source: &'static str,
    /// `restore [locate_data_file file] Shape1`, added to the document; none for the assembly grid.
    pub restored: Option<&'static str>,
    pub added: Added,
    pub recorded: RecordedVerdict,
}
