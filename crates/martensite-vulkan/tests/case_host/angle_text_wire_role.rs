//! The glyph wire role is an owned drawer aspect, never a nearby line primitive.
use super::*;
use std::fmt;

#[derive(Debug, PartialEq)]
pub(super) enum TextWireRefusal {
    Not3d,
    Shaded,
    MissingOwnedAspect,
    MissingAspect,
    AliasedLine,
    ConflictingFlags,
    MissingBinding,
}
impl fmt::Display for TextWireRefusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "text wire role {self:?}; expected exclusive 3D unshaded glyph wire with an owned distinct aspect"
        )
    }
}
impl std::error::Error for TextWireRefusal {}
fn reject(reason: TextWireRefusal) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, reason)
}
pub(super) fn aspect(
    dimension: &Handle<PrsDimAngleDimension>,
) -> io::Result<Handle<dyn Graphic3dAspectsSource>> {
    let style = dimension.dimension_aspect();
    if style.is_null() {
        return Err(reject(TextWireRefusal::MissingAspect));
    }
    if !style.is_text_3d() {
        return Err(reject(TextWireRefusal::Not3d));
    }
    if style.is_text_shaded() {
        return Err(reject(TextWireRefusal::Shaded));
    }
    let drawer = dimension.attributes();
    if drawer.is_null() {
        return Err(reject(TextWireRefusal::MissingOwnedAspect));
    }
    if !drawer.has_own_free_boundary_aspect() {
        return Err(reject(TextWireRefusal::MissingOwnedAspect));
    }
    let line = drawer.free_boundary_aspect();
    if line.is_null() || line.aspect().is_null() {
        return Err(reject(TextWireRefusal::MissingAspect));
    }
    let dimension_line = style.line_aspect();
    if dimension_line.is_null() || dimension_line.aspect().is_null() {
        return Err(reject(TextWireRefusal::MissingAspect));
    }
    let aspect = line.aspect().cast::<dyn Graphic3dAspectsSource>();
    if aspect == dimension_line.aspect().cast::<dyn Graphic3dAspectsSource>() {
        return Err(reject(TextWireRefusal::AliasedLine));
    }
    Ok(aspect)
}
pub(super) fn requested_kind(line: bool, text: bool) -> io::Result<Option<RoleKind>> {
    if line && text {
        return Err(reject(TextWireRefusal::ConflictingFlags));
    }
    Ok(if text {
        Some(RoleKind::TextWire)
    } else if line {
        Some(RoleKind::DimensionLine)
    } else {
        None
    })
}
impl RoleKind {
    pub(super) fn empty_suffix(self) -> &'static str {
        match self {
            Self::DimensionLine => "dimension-lines-empty",
            Self::TextWire => "dimension-text-wire-empty",
        }
    }
    pub(super) fn restored_suffix(self) -> &'static str {
        match self {
            Self::DimensionLine => "dimension-lines-restored",
            Self::TextWire => "dimension-text-wire-restored",
        }
    }
    pub(super) fn metadata_name(self) -> &'static str {
        match self {
            Self::DimensionLine => "angle-role.txt",
            Self::TextWire => "angle-text-wire-role.txt",
        }
    }
    pub(super) fn name(self) -> &'static str {
        match self {
            Self::DimensionLine => "actual-dim2-dimension-line-aspect",
            Self::TextWire => "actual-dim2-owned-text-free-boundary-aspect",
        }
    }
}

pub(super) fn missing_binding() -> io::Error {
    reject(TextWireRefusal::MissingBinding)
}
