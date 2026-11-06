//! Optional causal observations resolve actual dimension owners, never nearby draw indices.
#[cfg(test)]
#[path = "angle_role_observer_controls.rs"]
mod controls;
use super::presentation_role_scope as role_scope;
#[path = "angle_text_wire_role.rs"]
mod text_wire;
#[cfg(test)]
#[path = "angle_text_wire_controls.rs"]
mod text_wire_controls;
use super::capture_witness::{CaseWitnessDirectory, DumpWitnessWriter};
use crate::case_view::viewer_session::ViewerSession;
use crate::source_output::control_capture::control_dump;
use martensite_tkernel::standard::Handle;
use martensite_tkservice::graphic3d::*;
use martensite_tkv3d::{
    ais::AisInteractiveObject,
    prs_dim::{PrsDimAngleDimension, PrsDimDimensionMembers, PrsDimDimensionOwner},
    prs_mgr::{PrsMgrPresentableObjectMembers, PrsMgrPresentationManager},
    select_mgr::SelectMgrEntityOwnerSource,
};
use std::io;

const FLAG: &str = "MARTENSITE_ANGLE_DIMENSION_ROLE_OBSERVER";
#[derive(Clone, Copy, Debug, PartialEq)]
enum RoleKind {
    DimensionLine,
    TextWire,
}
struct ResolvedRole {
    kind: RoleKind,
    object: Handle<dyn AisInteractiveObject>,
    owner: Handle<dyn SelectMgrEntityOwnerSource>,
    mode: i32,
    groups: Vec<Handle<dyn Graphic3dGroup>>,
    aspect: Handle<dyn Graphic3dAspectsSource>,
}
#[derive(Debug, PartialEq)]
struct Identity {
    object: usize,
    owner: usize,
    mode: i32,
    camera: Vec<u64>,
    drawers: [usize; 3],
    groups: Vec<(i32, usize, usize)>,
}
fn refused(value: impl std::fmt::Display) -> io::Error {
    io::Error::other(format!(
        "angle role {value}; expected detected dim2 angle presentation and exact bindings"
    ))
}
fn owner_mode(
    object: &Handle<dyn AisInteractiveObject>,
    owner: &Handle<dyn SelectMgrEntityOwnerSource>,
) -> io::Result<i32> {
    if object.cast::<PrsDimAngleDimension>().is_null()
        || owner.is_null()
        || owner.selectable().object_address() != object.object_address()
    {
        return Err(refused("wrong object/owner identity or type"));
    }
    let part = owner.cast::<PrsDimDimensionOwner>();
    if !part.is_null() {
        return Ok(part.selection_mode() as i32);
    }
    if *owner != object.global_sel_owner() {
        return Err(refused("generic owner is not actual neutral owner"));
    }
    Ok(0)
}
fn resolved_groups(
    kind: RoleKind,
    object: &Handle<dyn AisInteractiveObject>,
    mode: i32,
    manager: &PrsMgrPresentationManager,
    aspect: &Handle<dyn Graphic3dAspectsSource>,
) -> io::Result<Vec<Handle<dyn Graphic3dGroup>>> {
    let prs = manager
        .presentation(&object.cast(), mode, false, &Handle::default())
        .map_err(refused)?;
    if prs.is_null() {
        return Err(refused(format!("missing computed mode {mode}")));
    }
    let groups: Vec<_> = prs
        .groups()
        .into_iter()
        .filter(|g| g.aspects().object_address() == aspect.object_address())
        .collect();
    if groups.is_empty() {
        if kind == RoleKind::TextWire {
            return Err(text_wire::missing_binding());
        }
        return Err(refused("zero dimension-line aspect bindings"));
    }
    Ok(groups)
}
fn resolve_owner_for(
    kind: RoleKind,
    object: Handle<dyn AisInteractiveObject>,
    owner: Handle<dyn SelectMgrEntityOwnerSource>,
    manager: &PrsMgrPresentationManager,
) -> io::Result<ResolvedRole> {
    let mode = owner_mode(&object, &owner)?;
    let dimension = object.cast::<PrsDimAngleDimension>();
    let aspect = match kind {
        RoleKind::DimensionLine => dimension.dimension_aspect().line_aspect().aspect().cast(),
        RoleKind::TextWire => text_wire::aspect(&dimension)?,
    };
    let groups = resolved_groups(kind, &object, mode, manager, &aspect)?;
    Ok(ResolvedRole {
        kind,
        object,
        owner,
        mode,
        groups,
        aspect,
    })
}
fn resolve_owner(
    object: Handle<dyn AisInteractiveObject>,
    owner: Handle<dyn SelectMgrEntityOwnerSource>,
    manager: &PrsMgrPresentationManager,
) -> io::Result<ResolvedRole> {
    resolve_owner_for(RoleKind::DimensionLine, object, owner, manager)
}
fn resolve(session: &ViewerSession) -> io::Result<ResolvedRole> {
    resolve_for(session, RoleKind::DimensionLine)
}
fn resolve_for(session: &ViewerSession, kind: RoleKind) -> io::Result<ResolvedRole> {
    let object = session
        .object("dim2")
        .cloned()
        .ok_or_else(|| refused("dim2 absent"))?;
    if session.context().detected_interactive() != object {
        return Err(refused("detected object is not actual dim2"));
    }
    let owner = session.context().detected_owner().clone();
    let style = session.context().get_hi_style(&object, &owner).read();
    if owner.cast::<PrsDimDimensionOwner>().is_null()
        && session.context().get_hilight_mode(&object, &style, -1) != 0
    {
        return Err(refused("neutral highlight mode is not zero"));
    }
    resolve_owner_for(
        kind,
        object,
        owner,
        session.context().main_prs_mgr().get().unwrap(),
    )
}
fn identity(session: &ViewerSession, role: &ResolvedRole) -> Identity {
    let camera = session.view().camera();
    let mut bits = camera
        .orientation_matrix()
        .data()
        .map(f64::to_bits)
        .to_vec();
    bits.extend(camera.projection_matrix().data().map(f64::to_bits));
    let groups = role
        .object
        .presentations()
        .into_iter()
        .flat_map(|p| {
            let mode = p.mode();
            p.groups().into_iter().map(move |g| {
                (
                    mode,
                    g.object_address() as usize,
                    g.aspects().object_address() as usize,
                )
            })
        })
        .collect();
    Identity {
        object: role.object.object_address() as usize,
        owner: role.owner.object_address() as usize,
        mode: role.mode,
        camera: bits,
        drawers: [
            role.object.attributes().object_address() as usize,
            role.object.hilight_attributes().object_address() as usize,
            role.object.dynamic_hilight_attributes().object_address() as usize,
        ],
        groups,
    }
}
fn require_identity(
    session: &ViewerSession,
    role: &ResolvedRole,
    before: &Identity,
) -> io::Result<()> {
    let current = resolve_for(session, role.kind)?;
    if identity(session, &current) != *before || current.aspect != role.aspect {
        return Err(refused("restored owner/camera/presentation/aspect changed"));
    }
    Ok(())
}
fn requested(flag: Option<&std::ffi::OsStr>, image: &str) -> io::Result<bool> {
    let Some(flag) = flag else {
        return Ok(false);
    };
    if flag != "1" {
        return Err(refused(format!("flag {flag:?}; expected 1 or unset")));
    }
    if image != "bug24133_2.png" {
        return Err(refused(format!("unsupported image {image}")));
    }
    Ok(true)
}
fn observe_role(session: &ViewerSession, image: &str, role: ResolvedRole) -> io::Result<()> {
    let before = identity(session, &role);
    let mut scope =
        role_scope::PresentationLineRoleScope::new(role.groups.clone(), role.aspect.clone())?;
    let capture =
        control_dump(session, image, role.kind.empty_suffix()).map_err(|e| refused(e.to_string()));
    scope.restore()?;
    require_identity(session, &role, &before)?;
    capture?;
    control_dump(session, image, role.kind.restored_suffix())
        .map_err(|e| refused(e.to_string()))?;
    require_identity(session, &role, &before)?;
    write_role(image, &role, &before)
}
fn write_role_to(
    writer: &mut impl DumpWitnessWriter,
    role: &ResolvedRole,
    before: &Identity,
) -> io::Result<()> {
    let metadata = format!(
        "role={}\nmatched_groups={}\nrestored=true\n{before:#?}\n",
        role.kind.name(),
        role.groups.len()
    );
    writer.write_piece(role.kind.metadata_name(), metadata.as_bytes())
}
fn write_role(image: &str, role: &ResolvedRole, before: &Identity) -> io::Result<()> {
    write_role_to(&mut CaseWitnessDirectory::new(image), role, before)
}
/// Observe the one approved causal target; `observe(session, "bug24133_2.png")?`.
pub(super) fn observe(session: &ViewerSession, image: &str) -> io::Result<()> {
    let line = requested(std::env::var_os(FLAG).as_deref(), image)?;
    let text = requested(
        std::env::var_os("MARTENSITE_ANGLE_TEXT_WIRE_ROLE_OBSERVER").as_deref(),
        image,
    )?;
    let Some(kind) = text_wire::requested_kind(line, text)? else {
        return Ok(());
    };
    observe_role(session, image, resolve_for(session, kind)?)
}
