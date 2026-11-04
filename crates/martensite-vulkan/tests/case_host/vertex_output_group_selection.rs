//! Explicit semantic part selection joins existing handles, never draw order or geometry rank.
use super::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum GroupSelection {
    WholeActor,
    ManipulatorRotation(i32),
}
impl GroupSelection {
    pub(super) fn parse(value: &str) -> io::Result<(martensite_vulkan::VertexOutputFormat, Self)> {
        use martensite_vulkan::VertexOutputFormat as Format;
        match value {
            "auto" => Ok((Format::TrianglesClipNormalId32, Self::WholeActor)),
            "points-clip-id20" => Ok((Format::PointsClipId20, Self::WholeActor)),
            "text-triangles-clip-uv-id28" => Ok((Format::TextClipUvId28, Self::WholeActor)),
            other => Ok((
                Format::TrianglesClipNormalId32,
                Self::ManipulatorRotation(rotation_axis(other)?),
            )),
        }
    }
    pub(super) fn groups(
        self,
        actor: &Handle<dyn AisInteractiveObject>,
        presentation: &Handle<PrsMgrPresentation>,
    ) -> Result<Vec<Handle<dyn Graphic3dGroup>>, Box<dyn Error>> {
        let Self::ManipulatorRotation(axis) = self else {
            return Ok(presentation.groups());
        };
        let selected = rotation_group(actor, axis)?;
        let members = presentation
            .groups()
            .into_iter()
            .filter(|group| {
                !selected.is_null() && group.object_address() == selected.object_address()
            })
            .collect();
        require_member(members, axis)
    }
}
fn rotation_axis(value: &str) -> io::Result<i32> {
    match value {
        "manipulator-rotation-0" => Ok(0),
        "manipulator-rotation-1" => Ok(1),
        "manipulator-rotation-2" => Ok(2),
        actual => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!(
                "vertex selector={actual:?}; expected auto, points-clip-id20, text-triangles-clip-uv-id28 or manipulator-rotation-0|1|2"
            ),
        )),
    }
}
fn rotation_group(
    actor: &Handle<dyn AisInteractiveObject>,
    axis: i32,
) -> Result<Handle<dyn Graphic3dGroup>, Box<dyn Error>> {
    let manipulator = actor.cast::<martensite_tkv3d::ais::AisManipulator>();
    if manipulator.is_null() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!(
                "actor={:?}; expected actual AisManipulator for rotation axis{axis}",
                actor.object_address()
            ),
        )
        .into());
    }
    Ok(manipulator.get_group(axis, martensite_tkv3d::ais::AisManipulatorMode::Rotation))
}
fn require_member(
    groups: Vec<Handle<dyn Graphic3dGroup>>,
    axis: i32,
) -> Result<Vec<Handle<dyn Graphic3dGroup>>, Box<dyn Error>> {
    let source = format!("actual manipulator rotation axis{axis} presentation membership");
    if groups.is_empty() {
        return Err(VertexOutputFailure::MissingSource { source }.into());
    }
    if groups.len() != 1 {
        return Err(VertexOutputFailure::AmbiguousSource {
            source,
            matches: groups.len(),
        }
        .into());
    }
    Ok(groups)
}
