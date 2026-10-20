//! Typed pixel access: the types that `Image_PixMap::Value<ColorType_t>` and
//! `Image_PixMapTypedData<PixelType_t>` reinterpret pixel bytes as.
//!
//! The native code reinterprets a pixel pointer; the port copies the native-endian bytes
//! into a value and back, which reads and writes the same bits; an [`ImagePixelPod`] type
//! can also be borrowed in place (the `typed_ref` family of `ImagePixMapData`).
// ImagePixelPod is an unsafe marker trait for the pixel types that may be reinterpreted.
#![allow(unsafe_code)]
use super::colors::*;
use martensite_tkernel::ncollection::{Vec2, Vec3, Vec4};

/// A plain-old-data pixel value of `SIZE` bytes.
pub trait ImagePixel: Copy {
    /// `sizeof(PixelType_t)`.
    const SIZE: usize;
    /// Reads a value from its native-endian bytes; `u16::read_pixel(&[1, 0]) == 1`.
    fn read_pixel(bytes: &[u8]) -> Self;
    /// Writes the value as native-endian bytes; `1u16.write_pixel(&mut bytes)`.
    fn write_pixel(&self, bytes: &mut [u8]);
}

/// Implements [`ImagePixel`] for a scalar through its native-endian byte conversions.
macro_rules! scalar_pixel {
    ($($scalar:ty),*) => {$(
        impl ImagePixel for $scalar {
            const SIZE: usize = size_of::<$scalar>();
            fn read_pixel(bytes: &[u8]) -> Self {
                <$scalar>::from_ne_bytes(bytes[..Self::SIZE].try_into().expect("pixel bytes"))
            }
            fn write_pixel(&self, bytes: &mut [u8]) {
                bytes[..Self::SIZE].copy_from_slice(&self.to_ne_bytes());
            }
        }
    )*};
}
scalar_pixel!(u8, u16, f32);

/// Implements [`ImagePixel`] for an array-backed pixel of `$count` scalars.
macro_rules! array_pixel {
    ($name:ty, $scalar:ty, $count:literal, |$value:ident| $components:expr, |$array:ident| $build:expr) => {
        impl ImagePixel for $name {
            const SIZE: usize = $count * size_of::<$scalar>();
            fn read_pixel(bytes: &[u8]) -> Self {
                let step = size_of::<$scalar>();
                let $array: [$scalar; $count] =
                    std::array::from_fn(|i| <$scalar>::read_pixel(&bytes[i * step..]));
                $build
            }
            fn write_pixel(&self, bytes: &mut [u8]) {
                let step = size_of::<$scalar>();
                let $value = self;
                let components: [$scalar; $count] = $components;
                for (i, component) in components.iter().enumerate() {
                    component.write_pixel(&mut bytes[i * step..]);
                }
            }
        }
    };
}
array_pixel!(ImageColorRgb, u8, 3, |p| p.v, |v| Self { v });
array_pixel!(ImageColorRgb32, u8, 4, |p| p.v, |v| Self { v });
array_pixel!(ImageColorRgba, u8, 4, |p| p.v, |v| Self { v });
array_pixel!(ImageColorBgr, u8, 3, |p| p.v, |v| Self { v });
array_pixel!(ImageColorBgr32, u8, 4, |p| p.v, |v| Self { v });
array_pixel!(ImageColorBgra, u8, 4, |p| p.v, |v| Self { v });
array_pixel!(ImageColorRgf, f32, 2, |p| p.v, |v| Self { v });
array_pixel!(ImageColorRgbf, f32, 3, |p| p.v, |v| Self { v });
array_pixel!(ImageColorBgrf, f32, 3, |p| p.v, |v| Self { v });
array_pixel!(ImageColorRgbaf, f32, 4, |p| p.v, |v| Self { v });
array_pixel!(ImageColorBgraf, f32, 4, |p| p.v, |v| Self { v });
array_pixel!(Vec2<u16>, u16, 2, |p| [p.x, p.y], |v| Vec2::new(v[0], v[1]));
array_pixel!(Vec3<u8>, u8, 3, |p| [p.x, p.y, p.z], |v| Vec3::new(
    v[0], v[1], v[2]
));
array_pixel!(Vec4<u16>, u16, 4, |p| [p.x, p.y, p.z, p.w], |v| Vec4::new(
    v[0], v[1], v[2], v[3]
));

/// A pixel type whose values may be read and written in place in the image bytes, as the
/// native `reinterpret_cast<T*>` accessors do.
///
/// # Safety
///
/// An implementor is a primitive or a `#[repr(C)]` structure of one primitive component type
/// without padding, `size_of::<Self>() == Self::SIZE`, and every bit pattern of that size is a
/// valid value.
pub unsafe trait ImagePixelPod: ImagePixel {}

/// Implements [`ImagePixelPod`] for pixel types that satisfy its contract.
macro_rules! pod_pixel {
    ($($pixel:ty),*) => {$(
        // SAFETY: a primitive, a #[repr(C)] Image_Color structure of a component array, or a
        // #[repr(C)] NCollection vector of scalars: no padding, every bit pattern valid
        // (u8, u16 and f32 components), and the size equals SIZE (checked by the tests).
        unsafe impl ImagePixelPod for $pixel {}
    )*};
}
pod_pixel!(
    u8,
    u16,
    f32,
    ImageColorRgb,
    ImageColorRgb32,
    ImageColorRgba,
    ImageColorBgr,
    ImageColorBgr32,
    ImageColorBgra,
    ImageColorRgf,
    ImageColorRgbf,
    ImageColorBgrf,
    ImageColorRgbaf,
    ImageColorBgraf,
    Vec2<u16>,
    Vec3<u8>,
    Vec4<u16>
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pixels_round_trip_through_their_bytes() {
        let mut bytes = [0u8; 16];
        ImageColorRgbaf {
            v: [0.5, 1.0, 0.0, 0.25],
        }
        .write_pixel(&mut bytes);
        assert_eq!(ImageColorRgbaf::read_pixel(&bytes).v, [0.5, 1.0, 0.0, 0.25]);
        Vec4::new(1u16, 2, 3, 4).write_pixel(&mut bytes);
        assert_eq!(<Vec2<u16>>::read_pixel(&bytes[4..]), Vec2::new(3, 4));
        assert_eq!(ImageColorBgra::SIZE, 4);
    }

    /// The ImagePixelPod contract: the in-memory size is the pixel size, with no padding.
    #[test]
    fn pod_pixels_have_no_padding() {
        fn check<P: ImagePixelPod>() {
            assert_eq!(size_of::<P>(), P::SIZE, "{}", std::any::type_name::<P>());
        }
        check::<u8>();
        check::<u16>();
        check::<f32>();
        check::<ImageColorRgb>();
        check::<ImageColorRgb32>();
        check::<ImageColorRgba>();
        check::<ImageColorBgr>();
        check::<ImageColorBgr32>();
        check::<ImageColorBgra>();
        check::<ImageColorRgf>();
        check::<ImageColorRgbf>();
        check::<ImageColorBgrf>();
        check::<ImageColorRgbaf>();
        check::<ImageColorBgraf>();
        check::<Vec2<u16>>();
        check::<Vec3<u8>>();
        check::<Vec4<u16>>();
    }
}
