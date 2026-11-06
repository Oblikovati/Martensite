//! The grouping statements of an OBJ file and the memory check of its reader.
use super::{RwObjMtlReader, RwObjReader, RwObjSubMeshReason, RwObjTools};
use martensite_tkernel::message::Message;
use martensite_tkernel::standard::Failure;
use martensite_tkernel::tcollection::AsciiString;

fn send_warning(text: &str) {
    Message::send_warning_text(&AsciiString::new(text));
}

/// Starts a sub-mesh; vertices may be duplicated after this point when one starts.
fn start_mesh<R: RwObjReader + ?Sized>(
    reader: &mut R,
    reason: RwObjSubMeshReason,
) -> Result<(), Failure> {
    let active = reader.obj_base().active_sub_mesh.clone();
    if reader.add_mesh(&active, reason)? {
        reader.obj_base_mut().packed_indices.clear();
    }
    Ok(())
}

pub(super) fn push_object<R: RwObjReader + ?Sized>(
    reader: &mut R,
    text: &[u8],
) -> Result<(), Failure> {
    let mut name = AsciiString::default();
    RwObjTools::read_name(text, &mut name);
    start_mesh(reader, RwObjSubMeshReason::NewObject)?;
    reader.obj_base_mut().active_sub_mesh.object = name;
    Ok(())
}

pub(super) fn push_group<R: RwObjReader + ?Sized>(
    reader: &mut R,
    text: &[u8],
) -> Result<(), Failure> {
    let mut name = AsciiString::default();
    RwObjTools::read_name(text, &mut name);
    start_mesh(reader, RwObjSubMeshReason::NewGroup)?;
    reader.obj_base_mut().active_sub_mesh.group = name;
    Ok(())
}

pub(super) fn push_smooth_group<R: RwObjReader + ?Sized>(
    reader: &mut R,
    text: &[u8],
) -> Result<(), Failure> {
    let mut name = AsciiString::default();
    RwObjTools::read_name(text, &mut name);
    if name == AsciiString::new("off") || name == AsciiString::new("0") {
        name = AsciiString::default();
    }
    // Smooth groups repeat in some files, so unlike groups and objects an equal name is ignored.
    if reader.obj_base().active_sub_mesh.smooth_group == name {
        return Ok(());
    }
    start_mesh(reader, RwObjSubMeshReason::NewSmoothGroup)?;
    reader.obj_base_mut().active_sub_mesh.smooth_group = name;
    Ok(())
}

pub(super) fn push_material<R: RwObjReader + ?Sized>(
    reader: &mut R,
    text: &[u8],
) -> Result<(), Failure> {
    let mut name = AsciiString::default();
    // An empty material name is allowed by the format.
    if RwObjTools::read_name(text, &mut name) && !reader.obj_base().materials.is_bound(&name) {
        let lines = reader.obj_base().nb_lines;
        send_warning(&format!(
            "Warning: use of undefined OBJ material at line {lines}"
        ));
        return Ok(());
    }
    if reader.obj_base().active_sub_mesh.material == name {
        return Ok(());
    }
    // A new group is created implicitly, to split materials.
    start_mesh(reader, RwObjSubMeshReason::NewMaterial)?;
    reader.obj_base_mut().active_sub_mesh.material = name;
    Ok(())
}

pub(super) fn read_material_lib<R: RwObjReader + ?Sized>(reader: &mut R, text: &[u8]) {
    let mut path = AsciiString::default();
    if !RwObjTools::read_name(text, &mut path) {
        let lines = reader.obj_base().nb_lines;
        send_warning(&format!("Warning: invalid OBJ syntax at line {lines}"));
        return;
    }
    let base = reader.obj_base_mut();
    let folder = base.folder.clone();
    if RwObjMtlReader::new(&mut base.materials).read(&folder, &path) {
        let full = AsciiString::from_raw(&[folder.as_bytes(), path.as_bytes()].concat());
        base.external_files.add(full);
    }
}

/// False once the memory limit is reached, which stops the import.
pub(super) fn check_memory<R: RwObjReader + ?Sized>(reader: &mut R) -> bool {
    let base = reader.obj_base_mut();
    if base.mem_estim < base.mem_limit_bytes || base.to_abort {
        return true;
    }
    let limit = base.mem_limit_bytes / (1024 * 1024);
    Message::send_fail_text(&AsciiString::new(&format!(
        "Error: OBJ file content does not fit into {limit} MiB limit.\nMesh data will be truncated."
    )));
    base.to_abort = true;
    false
}
