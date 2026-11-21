use super::{
    budget::{CaptureBudget, CaptureResult, native_failure},
    types::*,
};
use martensite_tkernel::{
    ncollection::{BaseAllocator, Buffer, String8},
    standard::Handle,
};
use martensite_tkservice::font::{FontFTFont, FontFTFontParams, FontFTLibrary, FontTextFormatter};

pub(super) fn open_font(
    asset: &OwnedFontTextAsset,
    options: &FontTextCaptureOptions,
    budget: &mut CaptureBudget<'_>,
) -> CaptureResult<FontFTFont> {
    let font = private_font(budget)?;
    let buffer = font_buffer(asset, budget)?;
    initialize_face(&font, &buffer, asset, budget)?;
    font.set_use_unicode_subset_fallback(false);
    if !font.set_width_scaling(options.width_scaling) {
        return Err(face_rejected(asset));
    }
    budget.checkpoint()?;
    Ok(font)
}
fn initialize_face(
    font: &FontFTFont,
    buffer: &Handle<Buffer>,
    asset: &OwnedFontTextAsset,
    budget: &mut CaptureBudget<'_>,
) -> CaptureResult<()> {
    let params = FontFTFontParams {
        point_size: 72,
        resolution: 4800,
        ..Default::default()
    };
    let accepted = font.init_from(buffer, &"owned-text-capture".into(), &params, asset.face);
    budget.checkpoint()?;
    if !accepted {
        return Err(face_rejected(asset));
    }
    Ok(())
}

fn private_font(budget: &mut CaptureBudget<'_>) -> CaptureResult<FontFTFont> {
    budget.checkpoint()?;
    let library = FontFTLibrary::new();
    budget.checkpoint()?;
    if !library.is_valid() {
        return Err(FontTextCaptureFailure::BackendUnavailable);
    }
    Ok(FontFTFont::new(Handle::new(library)))
}
fn face_rejected(asset: &OwnedFontTextAsset) -> FontTextCaptureFailure {
    FontTextCaptureFailure::FaceRejected {
        face: asset.face,
        bytes: asset.bytes.len(),
    }
}

fn font_buffer(
    asset: &OwnedFontTextAsset,
    budget: &mut CaptureBudget<'_>,
) -> CaptureResult<Handle<Buffer>> {
    budget.checkpoint()?;
    let buffer = Buffer::new(Some(BaseAllocator::common()), asset.bytes.len())
        .map_err(|cause| native_failure("font buffer allocation", cause));
    let mut buffer = budget.after_call(buffer)?;
    buffer.as_mut_slice().copy_from_slice(&asset.bytes);
    budget.checkpoint()?;
    Ok(Handle::new(buffer))
}
pub(super) fn format_text(
    font: &FontFTFont,
    request: &FontTextCaptureRequest,
    budget: &mut CaptureBudget<'_>,
) -> CaptureResult<FontTextFormatter> {
    budget.checkpoint()?;
    let formatter = FontTextFormatter::default();
    formatter.setup_alignment(request.options.horizontal, request.options.vertical);
    append_and_format(&formatter, font, request, budget)?;
    Ok(formatter)
}
fn append_and_format(
    formatter: &FontTextFormatter,
    font: &FontFTFont,
    request: &FontTextCaptureRequest,
    budget: &mut CaptureBudget<'_>,
) -> CaptureResult<()> {
    if request.text.is_empty() {
        return Ok(());
    }
    let text = String8::from_unicode_units(request.text.as_bytes(), -1);
    let appended = formatter
        .append(&text, font)
        .map_err(|cause| native_failure("layout append", cause));
    budget.after_call(appended)?;
    let formatted = formatter
        .format()
        .map_err(|cause| native_failure("layout format", cause));
    budget.after_call(formatted)
}
