//! The typed pixel references of `Image_PixMap::Value<ColorType_t>` and
//! `Image_PixMapTypedData<PixelType_t>`: `const T&` and `T&` into the image storage.
//!
//! The native accessors reinterpret the pixel address as a `T*`. Here a reference borrows the
//! buffer, so it sees the stored bits and writes through to them, and the borrow keeps the
//! buffer from changing or moving while the reference lives. A pixel outside the buffer, or an
//! address not aligned for `T` (a row size that is not a multiple of the component size, which
//! natively is undefined), gives `None`.
// The in-place references reinterpret aligned pixel bytes (ImagePixelPod); Miri runs the tests.
#![allow(unsafe_code)]
use super::ImagePixMapData;
use super::pixel::ImagePixelPod;

/// `bytes` as a `P` when it is long enough and aligned for `P`.
fn pod_ref<P: ImagePixelPod>(bytes: &[u8]) -> Option<&P> {
    let fits = bytes.len() >= size_of::<P>() && bytes.as_ptr().align_offset(align_of::<P>()) == 0;
    // SAFETY: the span holds size_of::<P>() initialized bytes at an address aligned for P, and
    // ImagePixelPod promises that every bit pattern of that size is a valid P without padding.
    fits.then(|| unsafe { &*bytes.as_ptr().cast::<P>() })
}

/// `bytes` as a mutable `P` when it is long enough and aligned for `P`.
fn pod_mut<P: ImagePixelPod>(bytes: &mut [u8]) -> Option<&mut P> {
    let fits = bytes.len() >= size_of::<P>() && bytes.as_ptr().align_offset(align_of::<P>()) == 0;
    // SAFETY: as in pod_ref; the exclusive borrow of the span makes the reference unique.
    fits.then(|| unsafe { &mut *bytes.as_mut_ptr().cast::<P>() })
}

/// `bytes` as the `P` values that fit in it, when it is aligned for `P`.
fn pod_slice<P: ImagePixelPod>(bytes: &[u8]) -> Option<&[P]> {
    let count = bytes.len() / size_of::<P>();
    let aligned = bytes.as_ptr().align_offset(align_of::<P>()) == 0;
    // SAFETY: count values of P cover initialized bytes of the span at an aligned address, and
    // every bit pattern is a valid P (ImagePixelPod).
    aligned.then(|| unsafe { std::slice::from_raw_parts(bytes.as_ptr().cast::<P>(), count) })
}

/// `bytes` as the mutable `P` values that fit in it, when it is aligned for `P`.
fn pod_slice_mut<P: ImagePixelPod>(bytes: &mut [u8]) -> Option<&mut [P]> {
    let count = bytes.len() / size_of::<P>();
    let aligned = bytes.as_ptr().align_offset(align_of::<P>()) == 0;
    // SAFETY: as in pod_slice; the exclusive borrow of the span makes the slice unique.
    aligned
        .then(|| unsafe { std::slice::from_raw_parts_mut(bytes.as_mut_ptr().cast::<P>(), count) })
}

impl ImagePixMapData {
    /// `Value<T>(theRow, theCol)`: the `const T&` of a pixel; `data.typed_ref::<u8>(0, 1)`.
    pub fn typed_ref<P: ImagePixelPod>(&self, row: usize, col: usize) -> Option<&P> {
        pod_ref(self.value(row, col)?)
    }
    /// `ChangeValue<T>(theRow, theCol)`: the `T&` of a pixel; `*data.typed_mut::<u8>(0, 1)? = 7`.
    pub fn typed_mut<P: ImagePixelPod>(&mut self, row: usize, col: usize) -> Option<&mut P> {
        pod_mut(self.change_value(row, col)?)
    }
    /// `ValueXY<T>(theX, theY)`: the `const T&` of the pixel at column `x`, row `y`.
    /// Example: `data.typed_ref_xy::<u16>(1, 0)`.
    pub fn typed_ref_xy<P: ImagePixelPod>(&self, x: usize, y: usize) -> Option<&P> {
        self.typed_ref(y, x)
    }
    /// `ChangeValueXY<T>(theX, theY)`: the `T&` of the pixel at column `x`, row `y`.
    /// Example: `*data.typed_mut_xy::<u16>(1, 0)? = 3`.
    pub fn typed_mut_xy<P: ImagePixelPod>(&mut self, x: usize, y: usize) -> Option<&mut P> {
        self.typed_mut(y, x)
    }
    /// `ValueXYZ<T>(theX, theY, theZ)`: the `const T&` of a voxel; `data.typed_ref_xyz::<u8>(0, 0, 1)`.
    pub fn typed_ref_xyz<P: ImagePixelPod>(&self, x: usize, y: usize, z: usize) -> Option<&P> {
        pod_ref(self.value_xyz(x, y, z)?)
    }
    /// `ChangeValueXYZ<T>(theX, theY, theZ)`: the `T&` of a voxel.
    /// Example: `*data.typed_mut_xyz::<u8>(0, 0, 1)? = 9`.
    pub fn typed_mut_xyz<P: ImagePixelPod>(
        &mut self,
        x: usize,
        y: usize,
        z: usize,
    ) -> Option<&mut P> {
        pod_mut(self.change_value_xyz(x, y, z)?)
    }
    /// `Image_PixMapTypedData<T>::Row(theRow)`: the row as the `T` values its bytes hold (the
    /// native `const T*` to the first pixel); `data.typed_row::<u16>(0)`.
    pub fn typed_row<P: ImagePixelPod>(&self, row: usize) -> Option<&[P]> {
        pod_slice(self.row(row)?)
    }
    /// `Image_PixMapTypedData<T>::ChangeRow(theRow)`: the row as mutable `T` values.
    /// Example: `data.typed_row_mut::<u16>(0)?[1] = 4`.
    pub fn typed_row_mut<P: ImagePixelPod>(&mut self, row: usize) -> Option<&mut [P]> {
        pod_slice_mut(self.change_row(row)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::image::ImageColorRgbf;
    use martensite_tkernel::ncollection::Vec3;

    fn rgbf_image() -> ImagePixMapData {
        let mut data = ImagePixMapData::default();
        data.set_top_down(true);
        assert!(data.init(12, 2, 2, 0, None));
        data
    }

    /// A `T&` writes through to the stored bits and a later `const T&` reads them back.
    #[test]
    fn typed_references_alias_the_pixel_storage() {
        let mut data = rgbf_image();
        let pixel = data.typed_mut::<ImageColorRgbf>(1, 1).unwrap();
        pixel.v = [0.25, 0.5, 1.0];
        *pixel.r_mut() = 0.75;
        assert_eq!(
            data.typed_ref::<ImageColorRgbf>(1, 1).unwrap().v,
            [0.75, 0.5, 1.0]
        );
        assert_eq!(
            data.typed::<ImageColorRgbf>(1, 1).unwrap().v,
            [0.75, 0.5, 1.0]
        );
        let stored = std::ptr::from_ref(data.typed_ref_xy::<ImageColorRgbf>(1, 1).unwrap());
        let bytes = data.value(1, 1).unwrap().as_ptr();
        assert_eq!(stored.cast::<u8>(), bytes);
        assert!(data.typed_ref::<ImageColorRgbf>(2, 0).is_none());
    }

    /// The typed row covers the whole row and writes through; a voxel reference uses the slice.
    #[test]
    fn typed_rows_and_voxels_alias_the_storage() {
        let mut data = ImagePixMapData::default();
        data.set_top_down(true);
        assert!(data.init_3d(2, Vec3::new(3, 2, 2), 0, None));
        data.typed_row_mut::<u16>(1).unwrap()[2] = 0x0102;
        assert_eq!(data.typed_row::<u16>(1).unwrap(), &[0, 0, 0x0102]);
        assert_eq!(data.typed_ref::<u16>(1, 2), Some(&0x0102));
        *data.typed_mut_xyz::<u16>(0, 1, 1).unwrap() = 7;
        assert_eq!(data.typed_ref_xyz::<u16>(0, 1, 1), Some(&7));
        assert_eq!(data.typed_ref_xyz::<u16>(0, 1, 0), Some(&0));
        *data.typed_mut_xy::<u16>(0, 1).unwrap() = 5;
        assert_eq!(data.typed_ref::<u16>(1, 0), Some(&5));
    }

    /// A row size that breaks the component alignment is undefined natively; it gives None.
    #[test]
    fn misaligned_pixels_give_none() {
        let mut data = ImagePixMapData::default();
        data.set_top_down(true);
        assert!(data.init(2, 2, 2, 5, None));
        assert!(data.typed_ref::<u16>(0, 1).is_some());
        assert!(data.typed_ref::<u16>(1, 0).is_none());
        assert!(data.typed_row::<u16>(1).is_none());
    }
}
