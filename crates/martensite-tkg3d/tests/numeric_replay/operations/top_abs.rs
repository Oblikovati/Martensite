//! Orientation, state and shape-category request encoding.
use super::common::{flag, int};
use super::output::Output;
use super::print::Reader;
use martensite_tkg3d::top_abs::{self, Orientation, ShapeEnum, State};

fn printed(write: impl FnOnce(&mut Vec<u8>) -> std::io::Result<()>) -> String {
    let mut stream: Vec<u8> = Vec::new();
    write(&mut stream).unwrap();
    String::from_utf8(stream).unwrap()
}

/// g3_topabs: the orientation algebra, then every name through ToString and Print.
pub fn topabs_case(output: &Output, _values: &[f64]) {
    for a in Orientation::ALL {
        for b in Orientation::ALL {
            int(output, top_abs::compose(a, b) as i32);
        }
    }
    for a in Orientation::ALL {
        int(output, top_abs::reverse(a) as i32);
        int(output, top_abs::complement(a) as i32);
    }
    for t in ShapeEnum::ALL {
        let text: String = printed(|s| top_abs::print_shape_type(t, s));
        replay_print!(output, "{} {text} ", top_abs::shape_type_to_string(t));
    }
    for o in Orientation::ALL {
        let text: String = printed(|s| top_abs::print_orientation(o, s));
        replay_print!(
            output,
            "{} {text} ",
            top_abs::shape_orientation_to_string(o)
        );
    }
    for st in State::ALL {
        replay_print!(output, "{} ", printed(|s| top_abs::print_state(st, s)));
    }
    replay_println!(output);
}

/// g3_topabs_parse n byte*n: both FromString overloads of shape types and orientations.
pub fn topabs_parse_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let count: i32 = r.integer();
    let bytes: Vec<u8> = (0..count).map(|_| r.integer() as u8).collect();
    let text: String = String::from_utf8(bytes).expect("probe text is valid UTF-8");
    int(output, top_abs::shape_type_from_string(&text) as i32);
    let found: Option<ShapeEnum> = top_abs::try_shape_type_from_string(&text);
    flag(output, found.is_some());
    int(output, found.unwrap_or(ShapeEnum::Wire) as i32);
    int(output, top_abs::shape_orientation_from_string(&text) as i32);
    let found: Option<Orientation> = top_abs::try_shape_orientation_from_string(&text);
    flag(output, found.is_some());
    int(output, found.unwrap_or(Orientation::Reversed) as i32);
    replay_println!(output);
}
