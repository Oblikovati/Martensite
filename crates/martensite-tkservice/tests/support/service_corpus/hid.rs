//! WNT_HIDSpaceMouse decoding.
use super::common::{Reader, end, flag, int, real};
use super::print::Output;
use martensite_tkernel::ncollection::Vec3;
use martensite_tkservice::wnt::WntHidSpaceMouse;

fn print_vector(output: &Output, (vector, idle): (Vec3<f64>, bool)) {
    real(output, vector.x);
    real(output, vector.y);
    real(output, vector.z);
    flag(output, idle);
}

/// `sv_hid ProductId Quadric N byte...`.
pub fn hid_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let product_id = r.real() as u64;
    let quadric = r.flag();
    let count = r.integer();
    let bytes: Vec<u8> = (0..count).map(|_| r.integer() as u8).collect();
    let mouse = WntHidSpaceMouse::new(product_id, &bytes);
    flag(output, WntHidSpaceMouse::is_known_product(product_id));
    flag(output, mouse.is_translation());
    flag(output, mouse.is_rotation());
    flag(output, mouse.is_key_state());
    print_vector(output, mouse.translation(quadric));
    print_vector(output, mouse.rotation(quadric));
    int(output, i32::from(mouse.raw_value_range()));
    if mouse.is_key_state() && count >= 5 {
        real(output, f64::from(mouse.key_state()));
    }
    (0..34u16).for_each(|bit| int(output, mouse.hid_to_space_key(bit) as i32));
    end(output);
}
