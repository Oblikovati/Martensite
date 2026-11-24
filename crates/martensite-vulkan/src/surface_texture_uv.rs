//! Native shader texture transform preserves projective Q and packed live UV buffers.
use crate::{
    RenderError, primitive::PrimitiveSubmission, primitive_buffer_read::read_vector_attribute,
    primitive_cache::UvStamp, surface_texture::SurfaceUvStream,
};
#[cfg(test)]
use martensite_tkernel::ncollection::Vec2;
use martensite_tkernel::standard::Handle;
use martensite_tkservice::graphic3d::*;
mod cache;
pub(crate) use cache::RetainedTextureStreams;
use cache::UvKey;

pub(crate) fn surface_texture_coordinates(
    primitive: &PrimitiveSubmission,
    parameters: &Handle<Graphic3dTextureParams>,
    top_down: bool,
) -> Result<SurfaceUvStream, RenderError> {
    let transform = SurfaceUvTransform::read(parameters, top_down);
    let key = |source| UvKey::Transformed {
        owner: parameters.downgrade(),
        source,
        bits: transform.bits(),
    };
    retained_uv(primitive, key, |uv| transform.programmable(uv))
}
/// Points skip all texture transforms, including image-row flipping and Q division.
pub(crate) fn point_texture_coordinates(
    primitive: &PrimitiveSubmission,
) -> Result<SurfaceUvStream, RenderError> {
    // Both GPU upload paths retain memory row order; no compensating UV transform is valid.
    retained_uv(primitive, UvKey::Raw, |uv| uv)
}

pub(crate) fn environment_coordinates(
    primitive: &PrimitiveSubmission,
    count: usize,
) -> SurfaceUvStream {
    let key = UvKey::Environment(count);
    if let Some(cache) = &primitive.retained {
        let cache = &cache.texture_streams;
        if let Some(uv) = cache.uv(&key) {
            return uv;
        }
        return cache.remember_uv(key, vec![[0., 0., 0., 1.]; count].into());
    }
    vec![[0., 0., 0., 1.]; count].into()
}

fn retained_uv(
    primitive: &PrimitiveSubmission,
    key: impl FnOnce(UvStamp) -> UvKey,
    map: impl Fn([f32; 4]) -> [f32; 4],
) -> Result<SurfaceUvStream, RenderError> {
    let read = primitive.attributes.read_buffer();
    let cache = primitive
        .retained
        .as_ref()
        .map(|retained| &retained.texture_streams);
    let tracked = cache.and_then(|cache| Some((cache, key(UvStamp::capture(&read)?))));
    if let Some((cache, key)) = &tracked
        && let Some(uv) = cache.uv(key)
    {
        return Ok(uv);
    }
    let uv = read_uv(&read, map)?.into();
    drop(read);
    Ok(match tracked {
        Some((cache, key)) => cache.remember_uv(key, uv),
        None => uv,
    })
}

/// Preserve the actual UV buffer before choosing a coordinate pipeline; `surface_input_uv(primitive)?`.
pub(crate) fn surface_input_uv(
    primitive: &PrimitiveSubmission,
) -> Result<Vec<[f32; 4]>, RenderError> {
    let read = primitive.attributes.read_buffer();
    read_uv(&read, |uv| uv)
}

fn read_uv(
    read: &Graphic3dBufferRead<'_>,
    map: impl Fn([f32; 4]) -> [f32; 4],
) -> Result<Vec<[f32; 4]>, RenderError> {
    let count = read.element_count().max(0) as usize;
    let Some(attribute) = read.attribute(Graphic3dTypeOfAttribute::Uv) else {
        return Ok(vec![map([0., 0., 0., 1.]); count]);
    };
    read_vector_attribute(
        &attribute,
        count,
        "Vec2, Vec3 or Vec4 texture coordinates",
        "allocated UV bytes",
        map,
    )
}
pub(crate) fn transform_surface_uv(
    uv: [f32; 4],
    parameters: &Graphic3dTextureParams,
    top_down: bool,
) -> [f32; 4] {
    SurfaceUvTransform::read(parameters, top_down).programmable(uv)
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct SurfaceUvTransform {
    sine: f32,
    cosine: f32,
    scale: [f32; 2],
    shift: [f32; 2],
}
impl SurfaceUvTransform {
    fn bits(self) -> [u32; 6] {
        let [scale_x, scale_y] = self.scale;
        let [shift_x, shift_y] = self.shift;
        [self.sine, self.cosine, scale_x, scale_y, shift_x, shift_y].map(f32::to_bits)
    }
    pub fn read(parameters: &Graphic3dTextureParams, top_down: bool) -> Self {
        let (sine, cosine) = {
            let (sine, cosine) =
                martensite_cmath::sincos(f64::from((-parameters.rotation()).to_radians()));
            (sine as f32, cosine as f32)
        };
        let scale = parameters.scale();
        let shift = parameters.translation();
        // Recorded references use a bottom-up UV convention; Vulkan image rows retain source order.
        let ty = -shift.y() + if top_down { 1. / scale.y() } else { 0. };
        Self {
            sine,
            cosine,
            scale: [scale.x(), if top_down { -scale.y() } else { scale.y() }],
            shift: [-shift.x(), ty],
        }
    }
    pub fn programmable(self, uv: [f32; 4]) -> [f32; 4] {
        self.apply(uv, 1.)
    }
    pub fn generated(self, uv: [f32; 4]) -> [f32; 4] {
        // The fixed-function texture matrix multiplies Q; the programmable transform does not.
        self.apply(uv, uv[3])
    }
    fn apply(self, uv: [f32; 4], weight: f32) -> [f32; 4] {
        let x = uv[0] * self.cosine - uv[1] * self.sine;
        let y = uv[0] * self.sine + uv[1] * self.cosine;
        [
            (x + self.shift[0] * weight) * self.scale[0],
            (y + self.shift[1] * weight) * self.scale[1],
            uv[2],
            uv[3],
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use martensite_tkernel::ncollection::{Vec3, Vec4};

    fn retained_source() -> (PrimitiveSubmission, Handle<Graphic3dBuffer>) {
        let attributes =
            Handle::new(Graphic3dBuffer::new(Graphic3dBufferBase::default_allocator()).unwrap());
        let layout = [
            Graphic3dAttribute {
                id: Graphic3dTypeOfAttribute::Pos,
                data_type: Graphic3dTypeOfData::Vec3,
            },
            Graphic3dAttribute {
                id: Graphic3dTypeOfAttribute::Uv,
                data_type: Graphic3dTypeOfData::Vec4,
            },
        ];
        assert!(attributes.init(2, &layout).unwrap());
        assert!(attributes.write_at(12, Vec4::new(0.25_f32, 0.5, 0.75, 2.)));
        let mut primitive = PrimitiveSubmission {
            kind: Graphic3dTypeOfPrimitiveArray::Triangles,
            attributes: attributes.cast(),
            indices: Handle::default(),
            bounds: Handle::default(),
            upload: Default::default(),
            uid: 1,
            retained: None,
            mesh_levels: None,
        };
        primitive.retained =
            crate::primitive_cache::RetainedPrimitives::default().entry(&primitive);
        (primitive, attributes)
    }

    #[test]
    fn texture_transforms_are_live_without_sampler_changes_detaching_uv() {
        let (primitive, _) = retained_source();
        let params = Handle::new(Graphic3dTextureParams::default());
        let read = || surface_texture_coordinates(&primitive, &params, false).unwrap();
        let first = read();
        params.set_repeat(true);
        assert_eq!(first.identity(), read().identity());
        params.set_scale(Vec2::new(2., 4.));
        let mapped = read();
        assert_ne!(first.identity(), mapped.identity());
        assert_eq!(mapped[0], [0.5, 2., 0.75, 2.]);
        let flipped = surface_texture_coordinates(&primitive, &params, true).unwrap();
        assert_ne!(mapped.identity(), flipped.identity());
        assert_eq!(flipped[0], [0.5, -3., 0.75, 2.]);
    }

    #[test]
    fn matching_texture_transforms_share_uv_and_signed_zero_changes_identity() {
        let (primitive, _) = retained_source();
        let first = Handle::new(Graphic3dTextureParams::default());
        let second = Handle::new(Graphic3dTextureParams::default());
        let read = |params| surface_texture_coordinates(&primitive, params, false).unwrap();
        assert_eq!(read(&first).identity(), read(&second).identity());
        second.set_scale(Vec2::new(0., 1.));
        let positive = read(&second);
        second.set_scale(Vec2::new(-0., 1.));
        let negative = read(&second);
        assert_ne!(positive.identity(), negative.identity());
        assert_eq!(positive[0][0].to_bits(), 0_f32.to_bits());
        assert_eq!(negative[0][0].to_bits(), (-0_f32).to_bits());
    }

    #[test]
    fn uv_cache_checks_buffer_writes_and_layout_without_explicit_invalidation() {
        let (primitive, attributes) = retained_source();
        let read = || point_texture_coordinates(&primitive).unwrap();
        let first = read();
        assert!(attributes.write_at(12, Vec4::new(0.5_f32, 0.25, 0.75, 2.)));
        let changed = read();
        assert_eq!(changed[0], [0.5, 0.25, 0.75, 2.]);
        assert_ne!(first.identity(), changed.identity());
        let mut layout = attributes.buffer_base().state.write();
        layout.attributes[1].data_type = Graphic3dTypeOfData::Vec3;
        drop(layout);
        let changed = read();
        assert_eq!(changed[0], [0.5, 0.25, 0.75, 1.]);
        assert_eq!(first[0], [0.25, 0.5, 0.75, 2.]);
    }

    #[test]
    fn sphere_map_normals_reuse_defaults_and_track_authored_values() {
        let (primitive, attributes) = retained_source();
        let first = primitive.shared_sphere_normals().unwrap();
        assert_eq!(&*first, [[0.; 3]; 2]);
        assert_eq!(
            first.identity(),
            primitive.shared_sphere_normals().unwrap().identity()
        );
        let mut layout = attributes.buffer_base().state.write();
        layout.attributes[1].id = Graphic3dTypeOfAttribute::Norm;
        layout.attributes[1].data_type = Graphic3dTypeOfData::Vec3;
        drop(layout);
        assert!(attributes.write_at(12, Vec3::new(1_f32, 0., 0.)));
        let authored = primitive.shared_sphere_normals().unwrap();
        assert_ne!(authored.identity(), first.identity());
        assert_eq!(authored[0], [1., 0., 0.]);
        assert_eq!(
            authored.identity(),
            primitive.shared_sphere_normals().unwrap().identity()
        );
    }

    #[test]
    fn native_packed_transform_rotates_before_translating_and_scaling() {
        let parameters = Graphic3dTextureParams::default();
        parameters.set_rotation(90.);
        parameters.set_scale(Vec2::new(2., 3.));
        parameters.set_translation(Vec2::new(0.25, 0.5));
        let result = transform_surface_uv([1., 2., 4., 5.], &parameters, false);
        for (actual, expected) in result.into_iter().zip([3.5, -4.5, 4., 5.]) {
            assert!((actual - expected).abs() < 1e-6);
        }
    }
    #[test]
    fn native_top_down_pack_preserves_projective_q() {
        let parameters = Graphic3dTextureParams::default();
        parameters.set_scale(Vec2::new(2., 4.));
        parameters.set_translation(Vec2::new(0.25, 0.5));
        assert_eq!(
            transform_surface_uv([1., 2., 3., 7.], &parameters, true),
            [1.5, -7., 3., 7.]
        );
        assert_eq!(
            transform_surface_uv([1., 2., 3., 7.], &parameters, false),
            [1.5, 6., 3., 7.]
        );
    }
    #[test]
    fn compatibility_texture_matrix_weights_translation_by_projective_q() {
        let parameters = Graphic3dTextureParams::default();
        parameters.set_translation(Vec2::new(0.25, 0.5));
        let transform = SurfaceUvTransform::read(&parameters, true);
        assert_eq!(
            transform.programmable([1., 2., 3., 2.]),
            [0.75, -2.5, 3., 2.]
        );
        assert_eq!(transform.generated([1., 2., 3., 2.]), [0.5, -3., 3., 2.]);
        assert_eq!(transform.generated([1., 2., 3., 0.]), [1., -2., 3., 0.]);
    }
}
