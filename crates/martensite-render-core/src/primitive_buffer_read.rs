//! Batched Graphic3d reads preserve the native byte order and homogeneous values.
use crate::{RenderError, error::rejected};
use martensite_tkernel::ncollection::{Vec2, Vec3, Vec4};
use martensite_tkservice::graphic3d::{
    Graphic3dAttributeRead, Graphic3dBufferValue, Graphic3dTypeOfData,
};

pub(crate) fn read_vector_attribute<T>(
    attribute: &Graphic3dAttributeRead<'_>,
    count: usize,
    expected_kind: &str,
    expected_bytes: &str,
    convert: impl Fn([f32; 4]) -> T,
) -> Result<Vec<T>, RenderError> {
    if count == 0 {
        return Ok(Vec::new());
    }
    // The shader multiplies the full occVertex, including a non-unit homogeneous weight.
    match attribute.kind {
        Graphic3dTypeOfData::Vec2 => {
            decode_vectors(attribute, count, expected_bytes, |v: Vec2<f32>| {
                convert([v.x(), v.y(), 0., 1.])
            })
        }
        Graphic3dTypeOfData::Vec3 => {
            decode_vectors(attribute, count, expected_bytes, |v: Vec3<f32>| {
                convert([v.x(), v.y(), v.z(), 1.])
            })
        }
        Graphic3dTypeOfData::Vec4 => {
            decode_vectors(attribute, count, expected_bytes, |v: Vec4<f32>| {
                convert(v.data())
            })
        }
        kind => Err(rejected(kind, expected_kind)),
    }
}

fn decode_vectors<V: Graphic3dBufferValue, T>(
    attribute: &Graphic3dAttributeRead<'_>,
    count: usize,
    expected_bytes: &str,
    convert: impl Fn(V) -> T,
) -> Result<Vec<T>, RenderError> {
    (0..count)
        .map(|index| decode_attribute(attribute, index, expected_bytes).map(&convert))
        .collect()
}

pub(crate) fn read_color(
    attribute: &Graphic3dAttributeRead<'_>,
    index: usize,
) -> Result<[f32; 4], RenderError> {
    let expected = "allocated color bytes";
    match attribute.kind {
        Graphic3dTypeOfData::Vec4ub => decode_attribute::<Vec4<u8>>(attribute, index, expected)
            .map(|v| v.data().map(|c| f32::from(c) / 255.)),
        Graphic3dTypeOfData::Vec4 => {
            decode_attribute::<Vec4<f32>>(attribute, index, expected).map(|v| v.data())
        }
        kind => Err(rejected(kind, "Vec4ub or Vec4 vertex colors")),
    }
}

fn decode_attribute<T: Graphic3dBufferValue>(
    attribute: &Graphic3dAttributeRead<'_>,
    index: usize,
    expected: &str,
) -> Result<T, RenderError> {
    attribute
        .value(index)
        .ok_or_else(|| rejected(attribute.byte_offset(index).unwrap_or(usize::MAX), expected))
}

pub(crate) fn read_index_words(
    stride: i32,
    bytes: &[u8],
    count: usize,
) -> Result<Vec<u32>, RenderError> {
    if stride == 2 {
        let words = bytes.as_chunks::<2>().0;
        let words = words
            .get(..count)
            .ok_or_else(|| invalid_index(words.len()))?;
        return Ok(words
            .iter()
            .map(|word| u32::from(u16::from_le_bytes(*word)))
            .collect());
    }
    let words = bytes.as_chunks::<4>().0;
    let indices: Vec<_> = words
        .iter()
        .take(count)
        .map(|word| i32::from_le_bytes(*word))
        .enumerate()
        .map(|(index, value)| u32::try_from(value).map_err(|_| invalid_index(index)))
        .collect::<Result<_, _>>()?;
    if count > words.len() {
        return Err(invalid_index(words.len()));
    }
    Ok(indices)
}

fn invalid_index(index: usize) -> RenderError {
    rejected(index, "allocated nonnegative vertex index")
}
