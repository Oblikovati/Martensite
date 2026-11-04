//! A named CPU fake writes only buffers owned by the real bounded query reader.
use super::api::ExecutableApi;
use ash::vk;

#[derive(Clone, Copy, Debug)]
pub(super) enum Fault {
    None,
    LargeSuccess,
    PropertiesError,
    PropertiesIncomplete,
    ZeroProperties,
    TooManyProperties,
    NoFragment,
    MixedStages,
    DuplicateFragments,
    UnterminatedName,
    IrIncomplete,
    ZeroIr,
    TooManyIr,
    ZeroBytes,
    OversizedBytes,
    OverflowBytes,
    ChangedCount,
    ChangedMetadata,
    ChangedSize,
    ByteReadError,
    InvalidText,
    UnterminatedText,
    InvalidTextFlag,
    NoAssembly,
    BinaryExtra,
}
pub(super) struct FakeExecutableApi {
    pub fault: Fault,
    pub ir_calls: usize,
}
impl FakeExecutableApi {
    /// Start an independent fake protocol sequence; `FakeExecutableApi::new(Fault::None)`.
    pub fn new(fault: Fault) -> Self {
        // Each control starts at the first real descriptor phase.
        Self { fault, ir_calls: 0 }
    }
}
impl ExecutableApi for FakeExecutableApi {
    fn properties(
        &mut self,
        _: vk::Pipeline,
        rows: &mut [vk::PipelineExecutablePropertiesKHR<'_>],
    ) -> (vk::Result, u32) {
        let (status, count) = property_response(self.fault);
        if matches!(self.fault, Fault::PropertiesError) {
            fill_property(&mut rows[0], Fault::None);
            c_field(&mut rows[0].name, b"retained-before-error");
            rows[0].subgroup_size = 77;
        }
        if status != vk::Result::SUCCESS || count == 0 || count as usize > rows.len() {
            return (status, count);
        }
        for row in &mut rows[..count as usize] {
            fill_property(row, self.fault);
        }
        (status, count)
    }
    fn representations(
        &mut self,
        _: vk::Pipeline,
        _: u32,
        rows: &mut [vk::PipelineExecutableInternalRepresentationKHR<'_>],
    ) -> (vk::Result, u32) {
        self.ir_calls += 1;
        if self.ir_calls == 2 && matches!(self.fault, Fault::ByteReadError) {
            return partial_byte_failure(&mut rows[0]);
        }
        if let Some(response) = ir_refusal(self.fault, self.ir_calls) {
            return response;
        }
        let count = ir_count(self.fault);
        for (index, row) in rows[..count].iter_mut().enumerate() {
            fill_ir(row, self.fault, self.ir_calls, index);
        }
        let written = ir_written(self.fault, self.ir_calls, count);
        (vk::Result::SUCCESS, written)
    }
}
fn c_field(target: &mut [std::ffi::c_char], text: &[u8]) {
    assert!(
        text.len() < target.len(),
        "bytes={} expected less than field={}",
        text.len(),
        target.len()
    );
    target.fill(0);
    for (slot, byte) in target.iter_mut().zip(text) {
        *slot = *byte as std::ffi::c_char;
    }
}
fn fill_property(row: &mut vk::PipelineExecutablePropertiesKHR<'_>, fault: Fault) {
    row.stages = vk::ShaderStageFlags::FRAGMENT;
    if matches!(fault, Fault::NoFragment) {
        row.stages = vk::ShaderStageFlags::VERTEX;
    }
    if matches!(fault, Fault::MixedStages) {
        row.stages |= vk::ShaderStageFlags::VERTEX;
    }
    c_field(&mut row.name, b"Fragment");
    c_field(&mut row.description, b"owned fake executable");
    if matches!(fault, Fault::UnterminatedName) {
        row.name.fill(65);
    }
    row.subgroup_size = 64;
}
fn fill_ir(
    row: &mut vk::PipelineExecutableInternalRepresentationKHR<'_>,
    fault: Fault,
    call: usize,
    index: usize,
) {
    let binary = index == 1 && matches!(fault, Fault::BinaryExtra);
    ir_labels(row, fault, binary);
    if call == 2 {
        write_ir_bytes(row, fault, binary);
    }
    row.data_size = initial_size(fault);
    if call == 2 && matches!(fault, Fault::ChangedMetadata) {
        row.is_text = vk::FALSE;
    }
    if call == 2 && matches!(fault, Fault::ChangedSize) {
        row.data_size = 9;
    }
}
fn ir_labels(
    row: &mut vk::PipelineExecutableInternalRepresentationKHR<'_>,
    fault: Fault,
    binary: bool,
) {
    c_field(
        &mut row.name,
        if matches!(fault, Fault::NoAssembly) {
            b"NIR"
        } else {
            b"Assembly"
        },
    );
    c_field(&mut row.description, b"owned test output");
    row.is_text = if binary { vk::FALSE } else { vk::TRUE };
    if matches!(fault, Fault::InvalidTextFlag) {
        row.is_text = 3;
    }
}
fn initial_size(fault: Fault) -> usize {
    match fault {
        Fault::LargeSuccess => 8 * 1024 * 1024,
        Fault::ZeroBytes => 0,
        Fault::OversizedBytes => 8 * 1024 * 1024 + 1,
        Fault::OverflowBytes => usize::MAX,
        _ => 8,
    }
}
fn write_ir_bytes(
    row: &mut vk::PipelineExecutableInternalRepresentationKHR<'_>,
    fault: Fault,
    binary: bool,
) {
    assert!(
        !row.p_data.is_null() && row.data_size >= 8,
        "pointer={:?} size={} expected owned8byte buffer",
        row.p_data,
        row.data_size
    );
    if matches!(fault, Fault::LargeSuccess) {
        write_large_payload(row);
        return;
    }
    let bytes = ir_payload(fault, binary);
    // The real reader supplies this initialized, stable allocation for exactly this call.
    // SAFETY: the destination is a live mapped allocation whose size was checked above, and it does not overlap the source.
    unsafe { std::ptr::copy_nonoverlapping(bytes.as_ptr(), row.p_data.cast(), bytes.len()) };
}

fn property_response(fault: Fault) -> (vk::Result, u32) {
    match fault {
        Fault::PropertiesError => (vk::Result::ERROR_DEVICE_LOST, 1),
        Fault::PropertiesIncomplete => (vk::Result::INCOMPLETE, 16),
        Fault::ZeroProperties => (vk::Result::SUCCESS, 0),
        Fault::TooManyProperties => (vk::Result::SUCCESS, 17),
        Fault::DuplicateFragments => (vk::Result::SUCCESS, 2),
        _ => (vk::Result::SUCCESS, 1),
    }
}
fn ir_refusal(fault: Fault, call: usize) -> Option<(vk::Result, u32)> {
    match (fault, call) {
        (Fault::IrIncomplete, _) => Some((vk::Result::INCOMPLETE, 16)),
        (Fault::ZeroIr, _) => Some((vk::Result::SUCCESS, 0)),
        (Fault::TooManyIr, _) => Some((vk::Result::SUCCESS, 17)),
        (Fault::ByteReadError, 2) => Some((vk::Result::ERROR_DEVICE_LOST, 0)),
        _ => None,
    }
}

fn ir_count(fault: Fault) -> usize {
    // The two-row cases exercise both size overflow and binary preservation.
    if matches!(
        fault,
        Fault::OverflowBytes | Fault::BinaryExtra | Fault::ChangedCount
    ) {
        return 2;
    }
    1
}
fn ir_written(fault: Fault, call: usize, count: usize) -> u32 {
    if call == 2 && matches!(fault, Fault::ChangedCount) {
        return 1;
    }
    // A stable count confirms the second call did not silently select a prefix.
    count as u32
}

fn ir_payload(fault: Fault, binary: bool) -> [u8; 8] {
    if binary || matches!(fault, Fault::InvalidText) {
        return [255, 1, 2, 3, 4, 5, 6, 0];
    }
    if matches!(fault, Fault::UnterminatedText) {
        return *b"ps code!";
    }
    *b"ps code\0"
}

fn partial_byte_failure(
    row: &mut vk::PipelineExecutableInternalRepresentationKHR<'_>,
) -> (vk::Result, u32) {
    assert!(!row.p_data.is_null() && row.data_size >= 3);
    // Only three actual bytes are written; the reader must retain the untouched sentinel tail.
    // SAFETY: the destination is a live mapped allocation whose size was checked above, and it does not overlap the source.
    unsafe { std::ptr::copy_nonoverlapping(b"ERR".as_ptr(), row.p_data.cast(), 3) };
    (vk::Result::ERROR_DEVICE_LOST, 0)
}
fn write_large_payload(row: &mut vk::PipelineExecutableInternalRepresentationKHR<'_>) {
    assert!(!row.p_data.is_null() && row.data_size == 8 * 1024 * 1024);
    // This fake fills exactly the real reader-owned allocation; no fabricated byte-call counter.
    // SAFETY: the destination is a live mapped allocation whose size was checked above, and it does not overlap the source.
    unsafe {
        std::ptr::write_bytes(row.p_data.cast::<u8>(), b'x', row.data_size);
        row.p_data.cast::<u8>().add(row.data_size - 1).write(0);
    }
}
