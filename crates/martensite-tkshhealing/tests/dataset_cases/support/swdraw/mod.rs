//! The SWscenario shape healing commands the TKShHealing scripts call, on the Rust API
//! Each operation returns
//! what the command sets and the text it prints (`di << ...`), which the callers log.
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::{message::ProgressRange, standard::Handle};
use martensite_tkshhealing::shape_extend::{
    BasicMsgRegistrator, ShapeExtendMsgRegistrator, ShapeExtendStatus,
};
use martensite_tkshhealing::shape_fix::{FixRoot, ShapeFix, ShapeFixShape};
use std::f64::consts::PI;

mod analysis;
mod fix;
mod misc;
mod process;
mod unify;
mod upgrade;
pub use analysis::{getanasurf, statshape, statshape_count};
pub use fix::{directfaces, fixsmall, fixwgaps, settolerance};
pub use misc::{checkoverlapedges, removeloc, scaleshape};
pub use process::{dt_apply_seq, heal_data_directory, set_resource_defaults};
pub use unify::unifysamedom;
pub use upgrade::{
    dt_closed_split, dt_shape_convert, dt_shape_convert_rev, dt_shape_divide, dt_split_angle,
    dt_split_by_number, dt_to_bspl,
};

/// Apply non-forced same-parameter repair before publishing the result;
/// `same_parameter_fix(&res)`.
pub fn same_parameter_fix(shape: &Shape) {
    ShapeFix::same_parameter(
        shape,
        false,
        0.0,
        &ProgressRange::default(),
        &Handle::default(),
    )
    .unwrap_or_else(|failure| panic!("ShapeFix::SameParameter: {failure:?}"));
}

/// The `Status: ...` lines the DT_* commands print for a tool's status, in their order:
/// `status_lines(|s| tool.status(s))`.
pub fn status_lines(status: impl Fn(ShapeExtendStatus) -> bool) -> String {
    use ShapeExtendStatus::*;
    const ORDER: [(ShapeExtendStatus, &str); 17] = [
        (Ok, "OK"),
        (Done1, "DONE1"),
        (Done2, "DONE2"),
        (Done3, "DONE3"),
        (Done4, "DONE4"),
        (Done5, "DONE5"),
        (Done6, "DONE6"),
        (Done7, "DONE7"),
        (Done8, "DONE8"),
        (Fail1, "FAIL1"),
        (Fail2, "FAIL2"),
        (Fail3, "FAIL3"),
        (Fail4, "FAIL4"),
        (Fail5, "FAIL5"),
        (Fail6, "FAIL6"),
        (Fail7, "FAIL7"),
        (Fail8, "FAIL8"),
    ];
    ORDER
        .iter()
        .filter(|(value, _)| status(*value))
        .map(|(_, word)| format!("Status: {word}\n"))
        .collect()
}

/// `fixshape result shape [prec [maxtol]] [switches]
/// [-maxtaila deg] [-maxtailw w]`, `args` being the words after `shape`: a ShapeFixShape
/// with a message registrator, the `-x`/`+x`/`*x` mode switches (l o h m d s i n), the
/// tail options, Init, SetPrecision, SetMaxTolerance and Perform; without `+?` it prints
/// nothing. `fixshape(&a, &["0.001", "0.005"])`.
pub fn fixshape(shape: &Shape, args: &[&str]) -> Shape {
    let fixer = ShapeFixShape::empty();
    let registrator: Handle<dyn BasicMsgRegistrator> =
        Handle::new(ShapeExtendMsgRegistrator::new()).cast();
    fixer.set_msg_registrator(&registrator);
    if !shape.is_null() {
        fixer.init(shape.clone());
    }
    let mut par = 2;
    let mut words = args.iter();
    while let Some(word) = words.next() {
        let bytes = word.as_bytes();
        if bytes.len() == 2 && matches!(bytes[0], b'-' | b'+' | b'*') {
            fixshape_switch(&fixer, bytes);
            continue;
        }
        match *word {
            "-maxtaila" => {
                let Some(degrees) = words.next() else { break };
                let wire = fixer.fix_wire_tool();
                wire.get()
                    .unwrap()
                    .set_max_tail_angle(atof(degrees) * (PI / 180.));
                continue;
            }
            "-maxtailw" => {
                let Some(width) = words.next() else { break };
                let wire = fixer.fix_wire_tool();
                wire.get().unwrap().set_max_tail_width(atof(width));
                wire.get().unwrap().set_fix_tail_mode(1);
                continue;
            }
            _ => {}
        }
        match par {
            2 => fixer.set_precision(atof(word)),
            3 => fixer.set_max_tolerance(atof(word)),
            _ => {}
        }
        par += 1;
    }
    fixer
        .perform()
        .unwrap_or_else(|failure| panic!("fixshape: {failure:?}"));
    fixer.shape()
}

/// One `<symbol><letter>` switch of `fixshape`: `-` off (0), `+` on (1), `*` default (-1).
fn fixshape_switch(fixer: &ShapeFixShape, bytes: &[u8]) {
    let value = match bytes[0] {
        b'-' => 0,
        b'+' => 1,
        _ => -1,
    };
    let wire = fixer.fix_wire_tool();
    let wire = wire.get().unwrap();
    let face = fixer.fix_face_tool();
    let face = face.get().unwrap();
    match bytes[1] {
        b'l' => wire.set_fix_lacking_mode(value),
        b'o' => face.set_fix_orientation_mode(value),
        b'h' => wire.set_fix_shifted_mode(value),
        b'm' => face.set_fix_missing_seam_mode(value),
        b'd' => wire.set_fix_degenerated_mode(value),
        b's' => wire.set_fix_small_mode(value),
        b'i' => wire.set_fix_self_intersection_mode(value),
        b'n' => wire.set_fix_notched_edges_mode(value),
        b'?' => panic!("fixshape +?: the message summary is unsupported"),
        _ => {}
    }
}

/// Parses a plain numeric word.
pub fn atof(word: &str) -> f64 {
    word.parse()
        .unwrap_or_else(|_| panic!("numeric token {word:?}: expected a number"))
}
