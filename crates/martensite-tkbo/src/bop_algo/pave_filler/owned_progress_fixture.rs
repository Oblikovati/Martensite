//! One checked reader reuses native-owned shape bytes across private progress controls.
use martensite_tkbrep::{brep::BRepBuilder, brep_tools, topods::Shape};
use martensite_tkernel::{
    message::{ProgressRange, ProgressScope},
    storage::IoStream,
};
use martensite_tkg3d::top_abs::ShapeEnum;

/// Reads an owned fixture and refuses an unexpected topology kind before stage execution.
/// Example: `read_owned_progress_shape(bytes, ShapeEnum::Face)`.
pub(in crate::bop_algo) fn read_owned_progress_shape(bytes: &[u8], expected: ShapeEnum) -> Shape {
    let mut stream = IoStream::from_bytes(bytes.to_vec());
    let mut shape = Shape::default();
    brep_tools::read(
        &mut shape,
        &mut stream,
        &BRepBuilder::default(),
        &ProgressRange::default(),
    )
    .unwrap_or_else(|cause| panic!("fixture {cause:?}, expected readable {expected:?} bytes"));
    assert_eq!(shape.shape_type().unwrap(), expected);
    shape
}

#[test]
#[should_panic(expected = "Compound")]
fn owned_progress_fixture_refuses_a_face_where_a_compound_is_required() {
    let bytes = include_bytes!("face_face_store_fixture/plane.brep");
    read_owned_progress_shape(bytes, ShapeEnum::Compound);
}

/// Keeps the complete native scope-name chain in owned progress observations.
/// Example: `let names = progress_scope_names(scope);`.
pub(super) fn progress_scope_names(scope: &ProgressScope<'_>) -> Vec<String> {
    let mut current = Some(scope);
    let mut names = Vec::new();
    while let Some(scope) = current {
        if let Some(name) = scope.name() {
            names.push(String::from_utf8_lossy(name).into_owned());
        }
        current = scope.parent();
    }
    names
}

/// Retains unnamed scopes and every actual max/value in a native callback chain.
/// Example: `let chain = progress_scope_ancestry(scope);`.
pub(super) fn progress_scope_ancestry(scope: &ProgressScope<'_>) -> Vec<(String, f64, f64)> {
    native_scope_frames(scope)
        .into_iter()
        .map(ancestry_view)
        .collect()
}

/// Own public SDK observations only; `native_scope_frames(scope)[0].maximum`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(in crate::bop_algo) struct NativeScopeFrame {
    pub name: Option<Vec<u8>>,
    pub maximum: u64,
    pub value: u64,
    pub portion: u64,
    pub infinite: bool,
    pub active: bool,
}

static_assertions::assert_impl_all!(NativeScopeFrame: Send, Sync);

/// Snapshot full ancestry before the callback returns; `let frames = native_scope_frames(scope);`.
pub(in crate::bop_algo) fn native_scope_frames(scope: &ProgressScope<'_>) -> Vec<NativeScopeFrame> {
    let mut frames = Vec::new();
    let mut current = Some(scope);
    while let Some(item) = current {
        frames.push(native_scope_frame(item));
        current = item.parent();
    }
    frames
}

fn native_scope_frame(scope: &ProgressScope<'_>) -> NativeScopeFrame {
    NativeScopeFrame {
        name: scope.name().map(<[u8]>::to_vec),
        maximum: scope.max_value().to_bits(),
        value: scope.value().to_bits(),
        portion: scope.get_portion().to_bits(),
        infinite: scope.is_infinite(),
        active: scope.is_active(),
    }
}

fn ancestry_view(frame: NativeScopeFrame) -> (String, f64, f64) {
    let name = frame.name.map_or_else(
        || "<null>".to_owned(),
        |bytes| {
            std::str::from_utf8(&bytes)
                .unwrap_or_else(|cause| {
                    panic!("scope name bytes {bytes:?} ({cause}), expected valid UTF-8")
                })
                .to_owned()
        },
    );
    (
        name,
        f64::from_bits(frame.maximum),
        f64::from_bits(frame.value),
    )
}

/// Encodes the pinned plain ASCII native scope traces as complete JSON arrays.
/// Example: `let json = native_scope_json(&progress_scope_ancestry(scope));`.
pub(super) fn native_scope_json(chain: &[(String, f64, f64)]) -> String {
    let fields: Vec<String> = chain
        .iter()
        .map(|(name, maximum, value)| {
            assert!(
                name.is_ascii() && !name.contains(['\"', '\\']),
                "scope name {name:?}, expected the pinned plain ASCII native scope literals"
            );
            format!("[\"{name}\",{maximum},{value}]")
        })
        .collect();
    format!("[{}]", fields.join(","))
}

/// Read actual copied flags for progress qualification; `section_progress_attribute_mask(&filler)`.
pub(in crate::bop_algo) fn section_progress_attribute_mask(
    filler: &super::BOPAlgoPaveFiller,
) -> i32 {
    let flags = filler.section_attribute;
    i32::from(flags.approximation())
        | (i32::from(flags.pcurve_on_s1()) << 1)
        | (i32::from(flags.pcurve_on_s2()) << 2)
}
