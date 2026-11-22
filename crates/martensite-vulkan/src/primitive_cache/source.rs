//! Exact source keys include live public layout fields that can change without a byte edit.
use super::PrimitiveSubmission;
use martensite_tkernel::ncollection::BufferRevision;
use martensite_tkservice::graphic3d::{
    Graphic3dBoundBufferState, Graphic3dBufferMembers, Graphic3dBufferRead,
    Graphic3dTypeOfAttribute, Graphic3dTypeOfData,
};

#[derive(PartialEq, Eq)]
pub(crate) struct GeometryStamp {
    pub(crate) attributes: AttributeStamp,
    pub(crate) indices: Option<IndexStamp>,
    pub(crate) bounds: Option<BoundsStamp>,
}

impl GeometryStamp {
    pub(super) fn has_vertex_colors(&self) -> bool {
        self.attributes.layout[1].is_some()
    }

    pub(crate) fn capture(primitive: &PrimitiveSubmission) -> Option<Self> {
        let attributes = AttributeStamp::capture(&primitive.attributes.get()?.read_buffer())?;
        let indices = match primitive.indices.get() {
            Some(buffer) => Some(IndexStamp::capture(&buffer.read_buffer())?),
            None => None,
        };
        let bounds = match primitive.bounds.get() {
            Some(buffer) => Some(BoundsStamp::capture(&buffer.state())?),
            None => None,
        };
        Some(Self {
            attributes,
            indices,
            bounds,
        })
    }
}

#[derive(PartialEq, Eq)]
pub(crate) struct AttributeStamp {
    revision: BufferRevision,
    count: i32,
    layout: [Option<AttributePlacement>; 3],
}

#[derive(PartialEq, Eq)]
struct AttributePlacement {
    kind: Graphic3dTypeOfData,
    first: Option<usize>,
    second: Option<usize>,
}

impl AttributeStamp {
    pub(crate) fn capture(read: &Graphic3dBufferRead<'_>) -> Option<Self> {
        use Graphic3dTypeOfAttribute::{Color, Norm, Pos};
        Some(Self {
            revision: read.content_revision()?,
            count: read.element_count(),
            layout: [Pos, Color, Norm].map(|role| AttributePlacement::capture(read, role)),
        })
    }
}

impl AttributePlacement {
    fn capture(read: &Graphic3dBufferRead<'_>, role: Graphic3dTypeOfAttribute) -> Option<Self> {
        let stream = read.attribute(role)?;
        Some(Self {
            kind: stream.kind,
            first: stream.byte_offset(0),
            second: stream.byte_offset(1),
        })
    }
}

#[derive(PartialEq, Eq)]
pub(crate) struct UvStamp {
    revision: BufferRevision,
    count: i32,
    placement: Option<AttributePlacement>,
}

impl UvStamp {
    pub(crate) fn capture(read: &Graphic3dBufferRead<'_>) -> Option<Self> {
        Some(Self {
            revision: read.content_revision()?,
            count: read.element_count(),
            placement: AttributePlacement::capture(read, Graphic3dTypeOfAttribute::Uv),
        })
    }
}

#[derive(PartialEq, Eq)]
pub(crate) struct IndexStamp {
    revision: BufferRevision,
    count: i32,
    stride: i32,
}

impl IndexStamp {
    pub(crate) fn capture(read: &Graphic3dBufferRead<'_>) -> Option<Self> {
        Some(Self {
            revision: read.content_revision()?,
            count: read.element_count(),
            stride: read.element_stride(),
        })
    }
}

#[derive(PartialEq, Eq)]
pub(crate) struct BoundsStamp {
    revision: BufferRevision,
    count: i32,
    capacity: i32,
    colors: bool,
}

impl BoundsStamp {
    pub(crate) fn capture(read: &Graphic3dBoundBufferState) -> Option<Self> {
        Some(Self {
            revision: read.data.content_revision()?,
            count: read.nb_bounds,
            capacity: read.nb_max_bounds,
            colors: read.has_colors,
        })
    }
}
