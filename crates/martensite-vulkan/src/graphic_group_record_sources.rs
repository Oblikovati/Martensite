//! The sources a group draw record reads besides its scalars: the revisions of its aspects
//! and the stamps of its geometry, compared in place on every replay (#594).
use super::super::commands::{GroupCommand, GroupGeometry};
use super::RecordKey;
use crate::primitive_cache::GeometryStamp;
use martensite_tkernel::standard::Handle;
use martensite_tkservice::graphic3d::*;

/// The aspects addresses with their revisions, and the geometry stamps, of a group.
type RecordedSources = (Vec<(usize, u64)>, Vec<GeometryStamp>);

/// Whether the base aspects and the commands still have the record's aspects revisions and
/// geometry stamps, compared in order without building new lists.
pub(super) fn sources_match(
    key: &RecordKey,
    base: &Handle<dyn Graphic3dAspectsSource>,
    commands: &[GroupCommand],
) -> bool {
    let (mut aspects, mut geometry) = (key.aspects.iter(), key.geometry.iter());
    if aspects.next() != aspects_revision(base).as_ref() {
        return false;
    }
    for command in commands {
        let same = match command {
            GroupCommand::Aspects(style) => aspects.next() == aspects_revision(style).as_ref(),
            GroupCommand::Stencil(_) => true,
            GroupCommand::Geometry(GroupGeometry::Primitives(primitive)) => geometry
                .next()
                .is_some_and(|stamp| stamp.matches(primitive)),
            _ => false,
        };
        if !same {
            return false;
        }
    }
    aspects.next().is_none() && geometry.next().is_none()
}

/// The aspects revisions and geometry stamps of commands that hold only aspects, stencil
/// switches and primitive arrays.
pub(super) fn recorded_sources(
    base: &Handle<dyn Graphic3dAspectsSource>,
    commands: &[GroupCommand],
) -> Option<RecordedSources> {
    let mut aspects = vec![aspects_revision(base)?];
    let mut geometry = Vec::new();
    for command in commands {
        match command {
            GroupCommand::Aspects(style) => aspects.push(aspects_revision(style)?),
            GroupCommand::Stencil(_) => {}
            GroupCommand::Geometry(GroupGeometry::Primitives(primitive)) => {
                geometry.push(GeometryStamp::capture(primitive)?);
            }
            _ => return None,
        }
    }
    Some((aspects, geometry))
}

/// The address and write revision of aspects. A write that changes their fields advances the
/// revision, so `plain_aspects` needs to read the fields only when a record is kept.
fn aspects_revision(style: &Handle<dyn Graphic3dAspectsSource>) -> Option<(usize, u64)> {
    let revision = style.get()?.aspects_base().fields.revision();
    Some((style.object_address().addr(), revision))
}

pub(crate) fn observe_command_inputs(
    scope: &GraphicInputRevision,
    base: &Handle<dyn Graphic3dAspectsSource>,
    commands: &[GroupCommand],
) -> bool {
    observe_aspects(scope, base)
        && commands.iter().all(|command| match command {
            GroupCommand::Aspects(style) => observe_aspects(scope, style),
            GroupCommand::Stencil(_) => true,
            GroupCommand::Geometry(GroupGeometry::Primitives(primitive)) => {
                primitive.observe_inputs(scope)
            }
            _ => false,
        })
}

fn observe_aspects(
    scope: &GraphicInputRevision,
    style: &Handle<dyn Graphic3dAspectsSource>,
) -> bool {
    let Some(source) = style.get() else {
        return false;
    };
    source.aspects_base().fields.observe(scope);
    true
}

/// Whether no aspects of the group read a shader program or textures, whose own contents
/// change without an aspects revision.
pub(super) fn plain_aspects(
    base: &Handle<dyn Graphic3dAspectsSource>,
    commands: &[GroupCommand],
) -> bool {
    let styles = commands.iter().filter_map(|command| match command {
        GroupCommand::Aspects(style) => Some(style),
        _ => None,
    });
    std::iter::once(base).chain(styles).all(|style| {
        style.get().is_some_and(|source| {
            let fields = source.aspects_base().fields.read();
            fields.program.is_null() && fields.texture_set.is_null() && !fields.to_map_texture
        })
    })
}
