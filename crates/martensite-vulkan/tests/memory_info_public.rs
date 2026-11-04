//! Public driver calls preserve caller outputs when no memory provider exists.
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkservice::graphic3d::Graphic3dGraphicDriver;
use martensite_vulkan::{
    FrameCapture, Graphic3dRenderDriver, RenderBackend, RenderError, RenderFrame,
};

struct UnavailableMemoryDevice;
impl RenderBackend for UnavailableMemoryDevice {
    fn render(&mut self, _frame: &RenderFrame) -> Result<FrameCapture, RenderError> {
        // Memory queries must not submit a frame or consume a rendering failure.
        panic!("memory query submitted geometry; expected a provider-only operation")
    }
}

#[test]
fn unavailable_memory_info_preserves_both_caller_outputs() {
    let driver = Graphic3dRenderDriver::with_backend(UnavailableMemoryDevice, 64).unwrap();
    let mut free_bytes = 0xA55;
    let mut description = AsciiString::from("sentinel");
    assert!(!driver.memory_info(&mut free_bytes, &mut description));
    assert_eq!(free_bytes, 0xA55);
    assert_eq!(description, AsciiString::from("sentinel"));
}
