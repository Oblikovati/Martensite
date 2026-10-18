use super::fakes::FailingBufferedInput;
use martensite_tkernel::fsd::{BinaryFile, MemoryFileSystem};
use martensite_tkernel::storage::checked_input::{InputContext, PositionedInput, UnformattedInput};
use std::rc::Rc;
pub(super) fn driver() -> BinaryFile {
    BinaryFile::new(Rc::new(MemoryFileSystem::default()))
}
pub(super) fn source(bytes: &[u8]) -> FailingBufferedInput {
    let mut source = FailingBufferedInput::new(bytes);
    source.cursor = 7;
    source
}
pub(super) fn with_input<R>(
    source: &mut FailingBufferedInput,
    call: impl FnOnce(&mut dyn UnformattedInput) -> R,
) -> R {
    let mut positioned = PositionedInput(source);
    let mut input = InputContext::new(&mut positioned);
    call(&mut input)
}
