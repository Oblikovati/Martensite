//! Retain the latest authored streams for each texture while the source primitive lives.
use crate::{
    draw::DrawStream,
    primitive_cache::{AttributeStamp, UvStamp},
    surface_texture::SurfaceUvStream,
};
use martensite_tkernel::standard::{SharedState, WeakHandle};
use martensite_tkservice::graphic3d::Graphic3dTextureParams;
use std::collections::VecDeque;

const TEXTURE_STREAM_LIMIT: usize = 4;

#[derive(Default)]
pub(crate) struct RetainedTextureStreams(SharedState<Option<Box<TextureStreams>>>);

#[derive(Default)]
struct TextureStreams {
    uv: VecDeque<(UvKey, SurfaceUvStream)>,
    zero_normals: Option<(AttributeStamp, DrawStream<[f32; 3]>)>,
}

#[derive(PartialEq, Eq)]
pub(super) enum UvKey {
    Raw(UvStamp),
    Transformed {
        owner: WeakHandle<Graphic3dTextureParams>,
        source: UvStamp,
        bits: [u32; 6],
    },
    Environment(usize),
}

impl UvKey {
    fn same_coordinates(&self, requested: &Self) -> bool {
        match (self, requested) {
            (
                Self::Transformed { source, bits, .. },
                Self::Transformed {
                    source: other,
                    bits: other_bits,
                    ..
                },
            ) => source == other && bits == other_bits,
            _ => self == requested,
        }
    }

    fn same_slot(&self, requested: &Self) -> bool {
        match (self, requested) {
            (Self::Raw(_), Self::Raw(_)) | (Self::Environment(_), Self::Environment(_)) => true,
            (Self::Transformed { owner, .. }, Self::Transformed { owner: other, .. }) => {
                owner == other
            }
            _ => false,
        }
    }
}

impl RetainedTextureStreams {
    pub(super) fn uv(&self, key: &UvKey) -> Option<SurfaceUvStream> {
        let saved = self.0.read();
        let (_, uv) = saved
            .as_ref()?
            .uv
            .iter()
            .find(|(candidate, _)| candidate.same_coordinates(key))?;
        Some(uv.clone())
    }

    pub(super) fn remember_uv(&self, key: UvKey, uv: SurfaceUvStream) -> SurfaceUvStream {
        let mut streams = self.0.write();
        let variants = &mut streams.get_or_insert_with(Box::default).uv;
        if let Some((_, saved)) = variants
            .iter()
            .find(|(saved, _)| saved.same_coordinates(&key))
        {
            return saved.clone();
        }
        if let Some(index) = variants.iter().position(|(saved, _)| saved.same_slot(&key)) {
            variants.remove(index);
        }
        if variants.len() == TEXTURE_STREAM_LIMIT {
            variants.pop_front();
        }
        variants.push_back((key, uv.clone()));
        uv
    }

    pub(crate) fn zero_normals(&self, stamp: &AttributeStamp) -> Option<DrawStream<[f32; 3]>> {
        let saved = self.0.read();
        let (source, normals) = saved.as_ref()?.zero_normals.as_ref()?;
        (source == stamp).then(|| normals.clone())
    }

    pub(crate) fn remember_zero_normals(
        &self,
        stamp: AttributeStamp,
        normals: DrawStream<[f32; 3]>,
    ) -> DrawStream<[f32; 3]> {
        let mut saved = self.0.write();
        let slot = &mut saved.get_or_insert_with(Box::default).zero_normals;
        if let Some((source, saved)) = slot.as_ref()
            && *source == stamp
        {
            return saved.clone();
        }
        *slot = Some((stamp, normals.clone()));
        normals
    }
}
