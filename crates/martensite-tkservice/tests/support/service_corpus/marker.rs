//! Graphic3d_MarkerImage.
use super::common::{Reader, end, flag, int, text};
use super::print::Output;
use martensite_tkernel::ncollection::{HArray1, Vec4};
use martensite_tkernel::standard::Handle;
use martensite_tkservice::aspect::AspectTypeOfMarker;
use martensite_tkservice::graphic3d::Graphic3dMarkerImage;
use martensite_tkservice::image::ImagePixMap;

/// The marker type of a corpus value (-1..=13).
fn marker_type(value: i32) -> AspectTypeOfMarker {
    use AspectTypeOfMarker as T;
    let all = [
        T::Empty,
        T::Point,
        T::Plus,
        T::Star,
        T::X,
        T::O,
        T::OPoint,
        T::OPlus,
        T::OStar,
        T::OX,
        T::Ring1,
        T::Ring2,
        T::Ring3,
        T::Ball,
        T::Userdefined,
    ];
    all[(value + 1) as usize]
}

fn pixmap(output: &Output, image: &Handle<ImagePixMap>) {
    flag(output, image.is_some());
    let Some(image) = image.get() else { return };
    int(output, image.width() as i32);
    int(output, image.height() as i32);
    int(output, image.format() as i32);
    flag(output, image.is_top_down());
    let state = image.data();
    let length = image.width() * image.size_pixel_bytes();
    for row in 0..image.height() {
        let bytes = state.data.row(row).unwrap_or_default();
        bytes[..length]
            .iter()
            .for_each(|byte| int(output, i32::from(*byte)));
    }
}

fn bits(output: &Output, array: &Handle<HArray1<u8>>) {
    flag(output, array.is_some());
    let Some(array) = array.get() else { return };
    let values = array.array1();
    int(output, values.lower());
    int(output, values.length());
    values
        .as_slice()
        .iter()
        .for_each(|byte| int(output, i32::from(*byte)));
}

/// `sv_marker Type Scale R G B`.
pub fn marker_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let kind = marker_type(r.integer());
    let scale = r.real() as f32;
    let c: Vec<f32> = (0..3).map(|_| r.real() as f32).collect();
    let marker =
        Graphic3dMarkerImage::standard_marker(kind, scale, Vec4::new(c[0], c[1], c[2], 1.0))
            .unwrap_or_default();
    flag(output, marker.is_some());
    if let Some(m) = marker.get() {
        text(output, m.get_image_id().as_bytes());
        text(output, m.get_image_alpha_id().as_bytes());
        let (w, h) = m.get_texture_size();
        int(output, w);
        int(output, h);
        flag(output, m.is_colored_image());
        bits(output, &m.get_bit_map_array(0.5, false));
        bits(output, &m.get_bit_map_array(0.5, true));
        pixmap(output, &m.get_image());
        pixmap(output, &m.get_image_alpha());
        flag(output, m.is_colored_image());
    }
    end(output);
}
