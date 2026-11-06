//! Exact public dump/state snapshots exclude only the intentionally swapped default aspect.
use super::*;
use crate::case_view::SourceViewOutput;
use martensite_tkernel::standard::dump::{DumpStream, text};
use martensite_tkv3d::ais::AisSelectionMembers;
#[derive(PartialEq)]
struct StableGrid {
    grid_fields: dump::Fields,
    group_fields: dump::Fields,
    structure_fields: dump::Fields,
    owners: [usize; 6],
    camera: [u64; 32],
    grid_bits: [u64; 8],
    plane_bits: [u64; 12],
    context: String,
}
pub(super) struct GridSnapshot {
    stable: StableGrid,
    original: Handle<Graphic3dAspects>,
    default_line: Handle<dyn Graphic3dAspectsSource>,
    default_line_values: Handle<Graphic3dAspects>,
}
impl GridSnapshot {
    /// Freeze owned diagnostic inputs; `GridSnapshot::capture(session,role)?`.
    pub(super) fn capture(session: &ViewerSession, role: &role::GridRole) -> io::Result<Self> {
        let default_line = defaults::default_line(session)?;
        Ok(Self {
            default_line_values: copy_aspect(&default_line),
            default_line,
            stable: StableGrid::capture(session, role)?,
            original: copy_aspect(&role.aspect),
        })
    }
    /// Refuse state interference during or after a swap; `saved.require(session,role,None)?`.
    pub(super) fn require(
        &self,
        session: &ViewerSession,
        role: &role::GridRole,
        replacement: Option<&Handle<dyn Graphic3dAspectsSource>>,
    ) -> io::Result<()> {
        let current = role::GridRole::resolve_current(session, replacement)?;
        self.require_default_line(session)?;
        self.require_stable(session, role, &current)?;
        let expected = replacement.unwrap_or(&role.aspect);
        if current.aspect != *expected || !role.aspect.is_equal(&*self.original) {
            return Err(refused(
                "regular role changed; expected exact recorded aspect and original fields",
            ));
        }
        require_replacement(&self.original, expected, replacement.is_some())
    }
    fn require_default_line(&self, session: &ViewerSession) -> io::Result<()> {
        let current = defaults::default_line(session)?;
        if self.default_line != current || !self.default_line.is_equal(&*self.default_line_values) {
            return Err(refused(
                "default drawer line changed; expected exact original handle and fields",
            ));
        }
        Ok(())
    }

    fn require_stable(
        &self,
        session: &ViewerSession,
        role: &role::GridRole,
        current: &role::GridRole,
    ) -> io::Result<()> {
        if self.stable != StableGrid::capture(session, current)?
            || current.grid != role.grid
            || current.group != role.group
            || current.structure != role.structure
        {
            return Err(refused(
                "grid scene/geometry/inlineaspect changed; expected original owned state",
            ));
        }
        Ok(())
    }
}
impl StableGrid {
    fn capture(session: &ViewerSession, role: &role::GridRole) -> io::Result<Self> {
        let source = role::grid_dump(&role.grid)?;
        let group = dump_group(&role.group)?;
        let structure = dump::unique(&source, b"Structure")?;
        let cstructure = dump::unique(&structure, b"CStructure")?;
        Ok(Self {
            grid_fields: dump::without(&source, &[b"Group", b"Structure"])?,
            group_fields: dump::without(&group, &[b"Aspects"])?,
            structure_fields: dump::without(&cstructure, &[b"Group"])?,
            owners: owner_handles(session, role),
            camera: camera_bits(session),
            grid_bits: numeric::grid_bits(role),
            plane_bits: numeric::plane_bits(session),
            context: context_values(session)?,
        })
    }
}
/// Observe group ownership and primitive metadata; `dump_group(&role.group)?`.
pub(super) fn dump_group(group: &Handle<dyn Graphic3dGroup>) -> io::Result<Vec<u8>> {
    let mut stream = DumpStream::default();
    group.dump_json(&mut stream, 4)?;
    Ok(text(&stream).as_bytes().to_vec())
}
fn owner_handles(session: &ViewerSession, role: &role::GridRole) -> [usize; 6] {
    [
        role.grid.object_address() as usize,
        role.structure.object_address() as usize,
        role.group.object_address() as usize,
        session.view().object_address() as usize,
        session.view().camera().object_address() as usize,
        session.view().window().object_address() as usize,
    ]
}
fn camera_bits(session: &ViewerSession) -> [u64; 32] {
    let camera = session.view().camera();
    let mut values = camera
        .orientation_matrix()
        .data()
        .map(f64::to_bits)
        .to_vec();
    values.extend(camera.projection_matrix().data().map(f64::to_bits));
    values.try_into().unwrap()
}
fn context_values(session: &ViewerSession) -> io::Result<String> {
    let displayed = displayed_handles(session)?;
    let selected = selected_handles(session);
    let mut defaults = DumpStream::default();
    session
        .context()
        .default_drawer()
        .dump_json(&mut defaults, -1)?;
    Ok(format!(
        "{:?}",
        (
            session.source_window_size(),
            displayed,
            selected,
            session.context().detected_owner().object_address() as usize,
            text(&defaults)
        )
    ))
}
fn selected_handles(session: &ViewerSession) -> Vec<usize> {
    let selection = session.context().selection();
    let owners = selection.objects();
    owners
        .iter()
        .map(|owner| owner.object_address() as usize)
        .collect()
}

fn displayed_handles(session: &ViewerSession) -> io::Result<Vec<usize>> {
    let mut displayed = Default::default();
    session
        .context()
        .displayed_objects(&mut displayed)
        .map_err(|error| refused(error.to_string()))?;
    Ok(displayed
        .iter()
        .map(|object| object.object_address() as usize)
        .collect())
}

fn copy_aspect(aspect: &Handle<dyn Graphic3dAspectsSource>) -> Handle<Graphic3dAspects> {
    let copy = Handle::new(Graphic3dAspects::default());
    *copy.aspects_base().fields.write() = aspect.aspects_base().fields.read().clone();
    copy
}
fn require_replacement(
    original: &Handle<Graphic3dAspects>,
    actual: &Handle<dyn Graphic3dAspectsSource>,
    empty: bool,
) -> io::Result<()> {
    let expected = copy_aspect(&original.cast());
    if empty {
        expected.set_line_type(martensite_tkservice::aspect::AspectTypeOfLine::Empty);
    }
    if !actual.is_equal(&*expected) {
        return Err(refused(
            "regular replacement fields changed; expected complete original or Empty clone",
        ));
    }
    Ok(())
}
