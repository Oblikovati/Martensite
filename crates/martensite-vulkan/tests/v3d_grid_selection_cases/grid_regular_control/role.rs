//! The actual viewer grid supplies the dump identity; public handles supply ownership.
use super::*;
use martensite_tkservice::aspect::AspectGrid;
use martensite_tkv3d::v3d::V3dViewer;
type GridGroupBinding = (
    Handle<dyn Graphic3dStructureSource>,
    Handle<dyn Graphic3dGroup>,
);
pub(super) struct GridRole {
    pub grid: Handle<dyn AspectGrid>,
    pub structure: Handle<dyn Graphic3dStructureSource>,
    pub group: Handle<dyn Graphic3dGroup>,
    pub aspect: Handle<dyn Graphic3dAspectsSource>,
}
impl GridRole {
    /// Resolve the actual displayed regular role; `GridRole::resolve(session)?`.
    pub(super) fn resolve(session: &ViewerSession) -> io::Result<Self> {
        Self::resolve_current(session, None)
    }
    /// Recheck a known active swap; `GridRole::resolve_current(session,Some(replacement))?`.
    pub(super) fn resolve_current(
        session: &ViewerSession,
        replacement: Option<&Handle<dyn Graphic3dAspectsSource>>,
    ) -> io::Result<Self> {
        let viewer = session.view().viewer();
        let grid = actual_grid(&viewer)?;
        let stream = grid_dump(&grid)?;
        let target = dump::unique(&stream, b"Group")?;
        let group_id = dump::unique(&target, b"this")?;
        let structure_id = dump::unique(&target, b"Structure")?;
        let (structure, group) = unique_group(&viewer, &group_id, &structure_id)?;
        require_single_group(&structure)?;
        let aspect = actual_regular(&group, replacement)?;
        Ok(Self {
            grid,
            structure,
            group,
            aspect,
        })
    }
}
fn require_single_group(structure: &Handle<dyn Graphic3dStructureSource>) -> io::Result<()> {
    let count = structure.groups().len();
    if count != 1 {
        return Err(refused(format!(
            "grid structure group count {count}; expected one owned group"
        )));
    }
    Ok(())
}

fn actual_grid(viewer: &Handle<V3dViewer>) -> io::Result<Handle<dyn AspectGrid>> {
    let grid = viewer.grid(false).map_err(|e| refused(e.to_string()))?;
    if !grid
        .dynamic_type()
        .is_some_and(|t| t.name() == "V3dRectangularGrid")
        || !grid.is_active()
        || !grid.is_displayed()
    {
        return Err(refused(
            "grid type/activity/display differs; expected existing displayed rectangular grid",
        ));
    }
    Ok(grid)
}
/// Read the actual public grid fields; `grid_dump(&role.grid)?`.
pub(super) fn grid_dump(grid: &Handle<dyn AspectGrid>) -> io::Result<Vec<u8>> {
    let mut stream = martensite_tkernel::standard::dump::DumpStream::default();
    grid.get()
        .ok_or_else(|| refused("null grid; expected actual viewer grid"))?
        .dump_json(&mut stream, 4)?;
    Ok(martensite_tkernel::standard::dump::text(&stream)
        .as_bytes()
        .to_vec())
}
fn unique_group(
    viewer: &Handle<V3dViewer>,
    group_id: &[u8],
    structure_id: &[u8],
) -> io::Result<GridGroupBinding> {
    unique_binding(displayed_groups(viewer), group_id, structure_id)
}
/// Refuse missing or ambiguous owned handles; `unique_binding(pairs,group,structure)?`.
pub(super) fn unique_binding(
    pairs: Vec<GridGroupBinding>,
    group_id: &[u8],
    structure_id: &[u8],
) -> io::Result<GridGroupBinding> {
    let mut matched = Vec::new();
    for (structure, group) in pairs {
        if binding_matches(&structure, &group, group_id, structure_id)? {
            matched.push((structure, group));
        }
    }
    if matched.len() != 1 {
        return Err(refused(format!(
            "public grid binding count {}; expected exactly one",
            matched.len()
        )));
    }
    Ok(matched.remove(0))
}
fn binding_matches(
    structure: &Handle<dyn Graphic3dStructureSource>,
    group: &Handle<dyn Graphic3dGroup>,
    group_id: &[u8],
    structure_id: &[u8],
) -> io::Result<bool> {
    let owned_group = dump::pointer(group.object_address() as usize)?;
    let owned_structure = dump::pointer(structure.object_address() as usize)?;
    Ok(owned_group == group_id && owned_structure == structure_id)
}

fn displayed_groups(
    viewer: &Handle<V3dViewer>,
) -> Vec<(
    Handle<dyn Graphic3dStructureSource>,
    Handle<dyn Graphic3dGroup>,
)> {
    let structures = viewer.structure_manager().displayed_structures();
    structures
        .into_iter()
        .filter(|structure| !structure.is_null())
        .flat_map(|structure| {
            structure
                .groups()
                .into_iter()
                .filter(|group| !group.is_null())
                .map(move |group| (structure.clone(), group))
        })
        .collect()
}

fn actual_regular(
    group: &Handle<dyn Graphic3dGroup>,
    replacement: Option<&Handle<dyn Graphic3dAspectsSource>>,
) -> io::Result<Handle<dyn Graphic3dAspectsSource>> {
    let aspect = group.aspects();
    if replacement.is_some_and(|expected| *expected == aspect) {
        return Ok(aspect);
    }
    if !aspect
        .dynamic_type()
        .is_some_and(|t| t.name() == "Graphic3d_AspectLine3d")
        || aspect.line_type() != martensite_tkservice::aspect::AspectTypeOfLine::Solid
        || aspect.line_width() != 1.
    {
        return Err(refused(
            "default grid aspect differs; expected actual unit solid line aspect",
        ));
    }
    Ok(aspect)
}
