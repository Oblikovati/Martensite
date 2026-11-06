//! The table types of the dataset cases: data only, no API call.

/// What the reference run ended in.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RecordedVerdict {
    Ok,
    /// `BAD (known problem)`: every problem is declared by a `TODO`.
    Known,
    Failed,
    Improvement,
}

/// A `de/step_*` script: data for the one procedure the grid's `end` is.
pub struct DeRow {
    pub source: &'static str,
    pub file: &'static str,
    /// `set ProductMode OFF`: `read.step.product.mode` is off for this read.
    pub product_mode_off: bool,
    /// `set lengthunit_start`: the document's length unit before the read.
    pub length_unit: Option<&'static str>,
    /// `de_before_script`: keys the script changes in the STEP resource file for its reads.
    pub healing: &'static [(&'static str, &'static str)],
    /// The case's own `puts "TODO ..."` lines, in order.
    pub todos: &'static [&'static str],
    /// `ref_data`, line by line, as `de/end` compares it: the first line is the empty one after the brace.
    pub reference: &'static [&'static str],
    pub recorded: RecordedVerdict,
}

/// The `gdt/*` grids whose `end` reads a STEP file and compares a dump with `ref_data`.
#[derive(Clone, Copy, Debug)]
pub enum GdtGrid {
    Export,
    Import,
    Presentation,
    View,
}

pub struct GdtRow {
    pub source: &'static str,
    pub grid: GdtGrid,
    pub file: &'static str,
    pub todos: &'static [&'static str],
    pub reference: &'static [&'static str],
    pub recorded: RecordedVerdict,
}

/// A `metadata/step` script.
pub struct MetadataRow {
    pub source: &'static str,
    pub file: &'static str,
    pub ref_size: i64,
    pub check_metadata: bool,
    pub reference: &'static [&'static str],
    pub recorded: RecordedVerdict,
}

/// A path word of a script.
#[derive(Clone, Copy, Debug)]
pub enum Place {
    /// `[locate_data_file name]`.
    Data(&'static str),
    /// `${imagedir}/name`.
    Image(&'static str),
}

/// A script with its own commands; the group's `begin` and `end` come from `group` and `end`.
pub struct StepCase {
    pub source: &'static str,
    pub group: &'static str,
    /// The limit the chain sets last; `de_wrapper` sets none.
    pub cpu_seconds: Option<i64>,
    pub recorded: RecordedVerdict,
    pub steps: &'static [Step],
}

/// One script command. Arguments are words the generator has already substituted.
#[derive(Debug)]
#[allow(
    clippy::enum_variant_names,
    reason = "the variants are the commands of a case"
)]
pub enum Step {
    Puts(&'static str),
    Param(&'static str, &'static str),
    NewModel,
    /// `ReadStep doc file`.
    ReadStep(&'static str, Place),
    /// `WriteStep doc file`.
    WriteStep(&'static str, Place),
    /// `stepread file name *`.
    StepRead(Place, &'static str),
    /// `stepwrite mode shape file`.
    StepWrite(&'static str, &'static str, Place),
    /// `testreadstep file name`.
    TestReadStep(Place, &'static str),
    /// `testwritestep file shape`.
    TestWriteStep(Place, &'static str),
    /// `testreadstep file name -stream`.
    TestReadStream(Place, &'static str),
    /// `testwritestep file shape -stream`.
    TestWriteStream(Place, &'static str),
    /// `ReadFile doc file`, `readfile shape file`, through the STEP provider.
    ReadFile(&'static str, Place),
    WriteFile(&'static str, Place),
    ReadShapeFile(&'static str, Place),
    WriteShapeFile(&'static str, Place),
    Restore(Place, &'static str),
    FileDelete(Place),
    TpCompound(&'static str),
    RenameVar(&'static str, &'static str),
    Copy(&'static str, &'static str),
    Explode(&'static str, &'static str),
    XGetOneShape(&'static str, &'static str),
    XGetShape(&'static str, &'static str, &'static str),
    Close(&'static str),
    /// `checkshape`, `checknbshapes`, `checkprops`, `checktrinfo`, `tricheck`, `incmesh`, `checkmaxtol`.
    Command(&'static str, &'static [&'static str]),
    /// `checknbshapes a -ref [nbshapes b]`.
    CheckNbShapesLike(&'static str, &'static str),
    /// `checkview -display name -2d`, on the memory raster.
    CheckView(&'static str),
    /// `checkview -screenshot -2d -path image`.
    Screenshot(&'static str),
    /// `smallview`, `donly`, `fit`, `isos`, `clear`, `dall`.
    Draw2d(&'static str, &'static [&'static str]),
    /// A 3D viewer command on the real renderer: `vinit`, `vfit`, `vdump`, `XShow`, `XDisplay`.
    Viewer(&'static str, &'static [&'static str]),
    /// `catch {step}`: a failure of the step does not stop the script.
    Catch(&'static Step),
}
