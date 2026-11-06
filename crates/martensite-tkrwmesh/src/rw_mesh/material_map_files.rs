//! The texture files of a material map.
use super::RwMeshMaterialMapBase;
use martensite_tkernel::message::Message;
use martensite_tkernel::osd::CadPath;
use martensite_tkernel::standard::Handle;
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkservice::image::{ImageTextureMembers as _, ImageTextureSource};
use std::path::Path;

fn text_of(value: &AsciiString) -> String {
    String::from_utf8_lossy(value.as_bytes()).into_owned()
}

fn send_fail(text: &str) {
    Message::send_fail_text(&AsciiString::new(text));
}

fn concatenated(parts: &[&[u8]]) -> AsciiString {
    AsciiString::from_raw(&parts.concat())
}

/// Copies a file; the same path counts as copied and a missing source is reported.
fn copy_file_to(source: &AsciiString, target: &AsciiString) -> bool {
    if source.is_empty() || target.is_empty() {
        return false;
    }
    if source == target {
        return true;
    }
    let (from, to) = (text_of(source), text_of(target));
    if !Path::new(&from).exists() {
        send_fail(&format!(
            "Failed to copy file - source file '{from}' does not exist"
        ));
        return false;
    }
    match std::fs::copy(&from, &to) {
        Ok(_) => true,
        Err(error) => {
            send_fail(&format!("Failed to copy file\n{error}"));
            false
        }
    }
}

/// Makes the texture folder; a failure falls back to the model's folder and still answers true.
pub(super) fn create_texture_folder(base: &mut RwMeshMaterialMapBase) -> bool {
    if !base.tex_folder.is_empty() {
        return true;
    }
    base.tex_folder_short = concatenated(&[base.short_file_name_base.as_bytes(), b"_textures/"]);
    base.tex_folder = concatenated(&[
        base.folder.as_bytes(),
        b"/",
        base.tex_folder_short.as_bytes(),
    ]);
    let folder = text_of(&base.tex_folder);
    if Path::new(&folder).exists() {
        return true;
    }
    if !Path::new(&text_of(&base.folder)).exists() {
        send_fail(&format!(
            "Failed to create textures folder '{}'",
            text_of(&base.folder)
        ));
        return false;
    }
    if std::fs::create_dir(&folder).is_err() {
        send_fail(&format!("Failed to create textures folder '{folder}'"));
        base.tex_folder = base.folder.clone();
        base.tex_folder_short = AsciiString::default();
    }
    true
}

/// Copies a file texture under its own name, or writes an embedded one under the key.
pub(super) fn copy_texture(
    base: &RwMeshMaterialMapBase,
    result: &mut AsciiString,
    texture: &Handle<dyn ImageTextureSource>,
    key: &AsciiString,
) -> bool {
    let Some(source) = texture.get() else {
        return false;
    };
    let path = source.file_path();
    if !path.is_empty() && source.file_offset() <= 0 && source.file_length() <= 0 {
        let (_, file_name) = CadPath::folder_and_file_from_path(path.as_bytes());
        let target = concatenated(&[base.tex_folder.as_bytes(), file_name.as_bytes()]);
        *result = concatenated(&[base.tex_folder_short.as_bytes(), file_name.as_bytes()]);
        return copy_file_to(&path, &target);
    }
    let mut extension = source.probe_image_file_format();
    if extension.is_empty() {
        extension = AsciiString::new("bin");
    }
    let file_name = concatenated(&[key.as_bytes(), b".", extension.as_bytes()]);
    let target = concatenated(&[base.tex_folder.as_bytes(), file_name.as_bytes()]);
    *result = concatenated(&[base.tex_folder_short.as_bytes(), file_name.as_bytes()]);
    source.write_image(&target)
}
