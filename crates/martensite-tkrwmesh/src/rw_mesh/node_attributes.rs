//! `RwMeshNodeAttributes`: what a reader knows about a node.
use martensite_tkernel::standard::Handle;
use martensite_tkernel::tcollection::AsciiString;
use martensite_tklcaf::tdata_std::TDataStdNamedData;
use martensite_tkxcaf::xcaf_prs::XCAFPrsStyle;

/// The attributes of a shape; `RwMeshNodeAttributes::default()`.
#[derive(Clone, Default)]
pub struct RwMeshNodeAttributes {
    /// The name for the user.
    pub name: AsciiString,
    /// The name within the low-level format structure.
    pub raw_name: AsciiString,
    /// Optional metadata.
    pub named_data: Handle<TDataStdNamedData>,
    /// The presentation style.
    pub style: XCAFPrsStyle,
}
