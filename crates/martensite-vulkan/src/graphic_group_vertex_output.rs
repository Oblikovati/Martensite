//! Selection is local to the actual retained group and its driver-issued primitive UID.
use super::*;
use std::num::NonZeroU64;
impl Graphic3dDrawGroup {
    /// Resolve actual retained text-object identities; `group.vertex_output_text_sources()`.
    pub fn vertex_output_text_sources(&self) -> Vec<usize> {
        self.submissions
            .read()
            .commands
            .iter()
            .filter_map(|command| match command {
                GroupCommand::Geometry(GroupGeometry::Text(text, _, _)) => {
                    Some(text.object_address() as usize)
                }
                _ => None,
            })
            .collect()
    }
    /// Bind one actual retained text object; `group.bind_vertex_output_text(address, token)?`.
    pub fn bind_vertex_output_text(
        &self,
        address: usize,
        token: NonZeroU64,
    ) -> Result<(), RenderError> {
        require_text_matches(
            address,
            self.vertex_output_text_sources()
                .into_iter()
                .filter(|actual| *actual == address)
                .count(),
        )?;
        self.submissions.write().vertex_output_text = Some((address, token));
        Ok(())
    }
    pub(super) fn tag_vertex_output_text(
        &self,
        text: &Handle<Graphic3dText>,
        draws: Vec<PrimitiveDraw>,
    ) -> Result<Vec<PrimitiveDraw>, RenderError> {
        let Some((address, token)) = self.submissions.read().vertex_output_text else {
            return Ok(draws);
        };
        if text.object_address() as usize != address {
            return Ok(draws);
        }
        require_text_matches(
            address,
            self.vertex_output_text_sources()
                .into_iter()
                .filter(|actual| *actual == address)
                .count(),
        )?;
        Ok(draws
            .into_iter()
            .map(|draw| draw.with_vertex_output_target(token))
            .collect())
    }
    /// Read actual retained source IDs, never guessed draw ordinals; `group.vertex_output_primitive_uids()`.
    pub fn vertex_output_primitive_uids(&self) -> Vec<usize> {
        self.submissions
            .read()
            .commands
            .iter()
            .filter_map(|command| match command {
                GroupCommand::Geometry(GroupGeometry::Primitives(primitive)) => Some(primitive.uid),
                _ => None,
            })
            .collect()
    }
    /// Resolve all actual triangle-array IDs in this source group; `group.vertex_output_triangle_primitive_uids()`.
    pub fn vertex_output_triangle_primitive_uids(&self) -> Vec<usize> {
        self.vertex_output_kind_primitive_uids(Graphic3dTypeOfPrimitiveArray::Triangles)
    }
    /// Resolve actual point-array IDs; `group.vertex_output_point_primitive_uids()`.
    pub fn vertex_output_point_primitive_uids(&self) -> Vec<usize> {
        self.vertex_output_kind_primitive_uids(Graphic3dTypeOfPrimitiveArray::Points)
    }
    fn vertex_output_kind_primitive_uids(&self, kind: Graphic3dTypeOfPrimitiveArray) -> Vec<usize> {
        self.submissions
            .read()
            .commands
            .iter()
            .filter_map(|command| match command {
                GroupCommand::Geometry(GroupGeometry::Primitives(primitive))
                    if primitive.kind == kind =>
                {
                    Some(primitive.uid)
                }
                _ => None,
            })
            .collect()
    }
    /// Bind an explicitly resolved source primitive; `group.bind_vertex_output_primitive(uid, token)?`.
    pub fn bind_vertex_output_primitive(
        &self,
        uid: usize,
        token: NonZeroU64,
    ) -> Result<(), RenderError> {
        let matches = self
            .vertex_output_primitive_uids()
            .into_iter()
            .filter(|actual| *actual == uid)
            .count();
        require_uid_matches(uid, matches)?;
        self.submissions.write().vertex_output = Some((uid, token));
        Ok(())
    }
    pub(super) fn tag_vertex_output_primitive(
        &self,
        primitive: &PrimitiveSubmission,
        draws: Vec<PrimitiveDraw>,
    ) -> Result<Vec<PrimitiveDraw>, RenderError> {
        let Some((uid, token)) = self.submissions.read().vertex_output else {
            return Ok(draws);
        };
        if primitive.uid != uid {
            return Ok(draws);
        }
        require_uid_matches(
            uid,
            self.vertex_output_primitive_uids()
                .into_iter()
                .filter(|actual| *actual == uid)
                .count(),
        )?;
        eprintln!(
            "{{\"kind\":\"vertex-output-source-binding\",\"source_token\":{token},\"group\":{},\"structure\":{},\"structure_c\":{},\"primitive_uid\":{uid},\"attributes\":{},\"indices\":{},\"draw_indices_input\":{:?},\"post_vertex_observed\":false}}",
            self as *const Self as usize,
            self.structure().object_address() as usize,
            self.structure()
                .get()
                .map_or(0, |structure| structure.cstructure().object_address()
                    as usize),
            primitive.attributes.object_address() as usize,
            primitive.indices.object_address() as usize,
            draws.iter().map(|draw| &draw.indices).collect::<Vec<_>>()
        );
        Ok(draws
            .into_iter()
            .map(|draw| draw.with_vertex_output_target(token))
            .collect())
    }
}
fn require_text_matches(address: usize, matches: usize) -> Result<(), RenderError> {
    if address != 0 && matches == 1 {
        return Ok(());
    }
    let source = format!("retained text object address={address}");
    if matches > 1 {
        return Err(crate::VertexOutputFailure::AmbiguousSource { source, matches }.into());
    }
    Err(crate::VertexOutputFailure::MissingSource { source }.into())
}
fn require_uid_matches(uid: usize, matches: usize) -> Result<(), RenderError> {
    if uid != 0 && matches == 1 {
        return Ok(());
    }
    let source = format!("retained primitive uid={uid}");
    if uid == 0 {
        return Err(crate::VertexOutputFailure::MissingSource { source }.into());
    }
    if matches == 0 {
        return Err(crate::VertexOutputFailure::MissingSource { source }.into());
    }
    Err(crate::VertexOutputFailure::AmbiguousSource { source, matches }.into())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn vertex_output_source_uid_never_falls_back_to_rank_or_geometry() {
        assert!(require_uid_matches(173, 1).is_ok());
        assert!(require_uid_matches(0, 1).is_err());
        assert!(require_uid_matches(173, 0).is_err());
        assert!(require_uid_matches(173, 2).is_err());
    }
}
#[cfg(test)]
mod retained_tests {
    use super::*;
    use crate::vertex_output_fixture::RetainedPhongFixture as RetainedSourceFixture;
    #[test]
    fn vertex_output_actual_group_uid_tags_only_its_original_primitive() {
        let source = RetainedSourceFixture::new();
        assert_eq!(source.group.vertex_output_primitive_uids(), [1]);
        assert_eq!(source.group.vertex_output_triangle_primitive_uids(), [1]);
        let token = NonZeroU64::new(173).unwrap();
        assert!(source.group.bind_vertex_output_primitive(2, token).is_err());
        source.group.bind_vertex_output_primitive(1, token).unwrap();
        let draws = source
            .group
            .primitive_draws(&Graphic3dCamera::default())
            .unwrap();
        assert_eq!(draws.len(), 1);
        assert_eq!(draws[0].vertex_output_target, Some(token));
        assert_eq!(draws[0].indices(), [0, 1, 2]);
    }
}
