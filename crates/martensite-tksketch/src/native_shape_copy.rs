// SPDX-License-Identifier: GPL-2.0-only
//! One topology/geometry copy and distinct exact image membership in its completed root.
use martensite_tkbrep::{topexp, topods::Shape, toptools::IndexedMapOfShape};
use martensite_tkernel::standard::Failure;
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPICopy, BRepBuilderAPIMakeShape};

pub(crate) enum CopyFailure {
    Native(Failure),
    SharedRoot,
    MissingMember,
}

pub(crate) struct ShapeCopy {
    copy: BRepBuilderAPICopy,
    root: Shape,
    members: IndexedMapOfShape,
    visited: IndexedMapOfShape,
}

impl ShapeCopy {
    pub(crate) fn new(source: &Shape) -> Result<Self, CopyFailure> {
        let mut copy =
            BRepBuilderAPICopy::with_shape(source, true, false).map_err(CopyFailure::Native)?;
        let root = copy.shape().map_err(CopyFailure::Native)?.clone();
        if root.is_same(source) {
            return Err(CopyFailure::SharedRoot);
        }
        let mut members = IndexedMapOfShape::default();
        topexp::map_shapes(&root, &mut members, true, true);
        Ok(Self {
            copy,
            root,
            members,
            visited: IndexedMapOfShape::default(),
        })
    }

    pub(crate) fn bind(&mut self, original: &Shape) -> Result<Shape, CopyFailure> {
        let copied = self
            .copy
            .modified_shape(original)
            .map_err(CopyFailure::Native)?;
        if copied.is_same(original)
            || !self.members.contains(&copied)
            || self.visited.contains(&copied)
        {
            return Err(CopyFailure::MissingMember);
        }
        self.visited.add(copied.clone());
        Ok(copied)
    }

    pub(crate) fn into_root(self) -> Shape {
        self.root
    }
}
