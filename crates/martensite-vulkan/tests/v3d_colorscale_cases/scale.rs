//! One colour-scale command: the options applied to a new or bound scale, then its redisplay.
use crate::session_run::SessionRun;
use martensite_tkernel::ncollection::Vec4;
use martensite_tkernel::quantity::{Color, ColorRgba};
use martensite_tkservice::aspect::{
    AspectTypeOfColorScaleData as Data, AspectTypeOfColorScalePosition as Pos,
};
use martensite_tkv3d::ais::AisColorScale;
use std::error::Error;

/// The options of one colour-scale command, applied in command-line order.
pub(crate) type ColorScaleSetting<'a> = &'a dyn Fn(&AisColorScale);

/// Apply `setting` to the scale `name` (created when unbound) and redisplay it sized to the
/// active view; `color_scale(&mut run, "cs", &|s| demo(s))?`.
pub(crate) fn color_scale(
    run: &mut SessionRun,
    name: &str,
    setting: ColorScaleSetting<'_>,
) -> Result<(), Box<dyn Error>> {
    let scale = run.session().color_scale(name)?;
    setting(scale.get().expect("a session colour scale is never null"));
    run.session().show_color_scale(name, &scale)?;
    Ok(())
}

/// The demonstration settings; `demo(scale)`.
pub(crate) fn demo(scale: &AisColorScale) {
    scale.set_position(0, 0);
    scale.set_text_height(16);
    scale.set_range(0.0, 100.0);
    scale.set_number_of_intervals(10);
    scale.set_breadth(0);
    scale.set_height(0);
    scale.set_label_position(Pos::Right);
    scale.set_color_type(Data::Auto);
    scale.set_label_type(Data::Auto);
}

/// A colour given as three real components in 0..1, read as single-precision linear RGB the
/// way the command line parses them; `linear_rgb(0.42, 0.35, 0.8)`.
pub(crate) fn linear_rgb(red: f32, green: f32, blue: f32) -> Color {
    let rgba = ColorRgba::from_vec4(Vec4::new(red, green, blue, 1.0))
        .expect("components in 0..1 form a valid colour");
    *rgba.rgb()
}

/// Read the named colour at (x, y) for every (y, name) and print the original's error line for
/// each mismatch; `expect_pixels(&mut run, 15, &[(20, "RED")])?`.
pub(crate) fn expect_pixels(
    run: &mut SessionRun,
    x: i32,
    expected: &[(i32, &str)],
) -> Result<(), Box<dyn Error>> {
    for &(y, name) in expected {
        if run.session().pixel_color_name(x, y)? != name {
            println!("Error: wrong color at {y}");
        }
    }
    Ok(())
}
