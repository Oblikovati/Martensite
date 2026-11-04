//! User-defined colours and labels: interval colours, labels, titles, smooth transitions,
//! hue and colour ranges, free labels, and finding the colour of a value.
use super::prelude::*;

/// The colours of the six red-to-blue demonstration intervals read at rows 20 to 370.
const DEMO_ROWS: [(i32, &str); 9] = [
    (20, "RED"),
    (60, "DARKORANGE1"),
    (100, "GOLD"),
    (140, "GREENYELLOW"),
    (220, "GREEN"),
    (250, "MEDIUMSPRINGGREEN"),
    (290, "CYAN"),
    (330, "DODGERBLUE1"),
    (370, "BLUE"),
];

/// The automatic colours of a five-interval scale read at rows 40 to 350.
const AUTO_ROWS: [(i32, &str); 5] = [
    (40, "RED"),
    (120, "YELLOW"),
    (200, "GREEN"),
    (280, "CYAN2"),
    (350, "BLUE"),
];

/// The user colours of a five-interval scale read at rows 40 to 350.
const USER_ROWS: [(i32, &str); 5] = [
    (40, "WHITE"),
    (120, "RED"),
    (200, "GREEN"),
    (280, "BLUE"),
    (350, "WHITE"),
];

/// Five intervals over 0..20.
fn five_intervals(scale: &AisColorScale) {
    scale.set_range(0., 20.);
    scale.set_number_of_intervals(5);
}

/// Custom colours and labels over the five intervals.
fn user_colors_and_labels(scale: &AisColorScale) {
    let colors = [
        Color::named(NamedColor::WHITE),
        linear_rgb(0., 0., 1.),
        Color::named(NamedColor::GREEN),
        linear_rgb(1., 0., 0.),
        linear_rgb(1., 1., 1.),
    ];
    scale.set_colors(&colors);
    scale.set_color_type(Data::User);
    let labels = ["start", "1", "2", "3", "4", "end"].map(ExtendedString::from_utf8);
    scale.set_labels(&labels);
    scale.set_label_type(Data::User);
}

/// `bug25136`: the demonstration scale, a reduced range, user colours and labels, and changed
/// interval colours, label and title.
pub(crate) fn customized_scales() -> Outcome {
    banner(&[
        "============",
        "OCC25136: Display customized colorscale.",
        "============",
        "",
    ]);
    let mut run = SessionRun::new()?;
    run.session().set_axonometric()?;
    color_scale(&mut run, "cs1", &|s| {
        demo(s);
        s.set_position(0, 0);
    })?;
    expect_pixels(&mut run, 15, &DEMO_ROWS)?;
    color_scale(&mut run, "cs2", &|s| {
        five_intervals(s);
        s.set_position(60, 0);
    })?;
    expect_pixels(&mut run, 75, &AUTO_ROWS)?;
    user_scale(&mut run, "cs3", 120)?;
    expect_pixels(&mut run, 135, &USER_ROWS)?;
    user_scale(&mut run, "cs4", 200)?;
    recolor_and_title(&mut run)?;
    let changed = [
        (60, "PINK"),
        (120, "RED"),
        (200, "GREEN"),
        (280, "BLUE"),
        (350, "LIGHTSTEELBLUE3"),
    ];
    expect_pixels(&mut run, 215, &changed)?;
    run.dump("bug25136.png")?;
    run.finish()
}

/// A five-interval scale with user colours and labels at `x`.
fn user_scale(run: &mut SessionRun, name: &str, x: i32) -> Result<(), Box<dyn Error>> {
    color_scale(run, name, &|s| {
        five_intervals(s);
        user_colors_and_labels(s);
        s.set_position(x, 0);
    })
}

/// `cs4`: first and last interval colours, the last label and a title, one command each.
fn recolor_and_title(run: &mut SessionRun) -> Result<(), Box<dyn Error>> {
    color_scale(run, "cs4", &|s| {
        s.set_interval_color(linear_rgb(0.42, 0.35, 0.8), 1);
        s.set_color_type(Data::User);
    })?;
    color_scale(run, "cs4", &|s| {
        s.set_interval_color(Color::named(NamedColor::PINK), 5);
        s.set_color_type(Data::User);
    })?;
    color_scale(run, "cs4", &|s| {
        s.set_label(&ExtendedString::from_utf8("last"), 6);
        s.set_label_type(Data::User);
    })?;
    color_scale(run, "cs4", &|s| {
        s.set_title(&ExtendedString::from_utf8("My color scale"))
    })
}

/// Print `message` when the colour found for `value` is not `expected`.
fn expect_found(
    run: &mut SessionRun,
    value: f64,
    expected: &str,
    message: &str,
) -> Result<(), Box<dyn Error>> {
    let scale = run.session().color_scale("cs")?;
    let scale = scale.get().expect("a session colour scale is never null");
    let mut color = Color::default();
    scale.find_color(value, &mut color)?;
    if Color::string_name(color.name()) != expected {
        println!("{message}");
    }
    Ok(())
}

/// `bug27573`: the colour of a value follows user colours, for positive and negative ranges.
pub(crate) fn find_color_custom() -> Outcome {
    banner(&[
        "============",
        "0027573: AisColorScale::FindColor does not take into account custom colors.",
        "============",
        "",
    ]);
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    run.session().set_axonometric()?;
    color_scale(&mut run, "cs", &demo)?;
    color_scale(&mut run, "cs", &five_intervals)?;
    color_scale(&mut run, "cs", &|s| {
        let names = [
            NamedColor::WHITE,
            NamedColor::RED,
            NamedColor::GREEN,
            NamedColor::BLUE,
            NamedColor::GRAY,
        ];
        s.set_colors(&names.map(Color::named));
        s.set_color_type(Data::User);
    })?;
    run.dump("bug27573_pos.png")?;
    let positive = [
        (0., "WHITE", "ERROR: wrong 1st segment \u{0441}olor"),
        (
            4.,
            "RED",
            "ERROR: wrong color on border between 1st and 2nd segments",
        ),
        (5., "RED", "ERROR: wrong 2nd segment color"),
        (9., "GREEN", "ERROR: wrong 3rd segment color"),
        (20., "GRAY", "ERROR: wrong last segment color at border"),
    ];
    for (value, expected, message) in positive {
        expect_found(&mut run, value, expected, message)?;
    }
    color_scale(&mut run, "cs", &|s| {
        s.set_range(-5., 5.);
        s.set_number_of_intervals(5);
    })?;
    run.dump("bug27573_neg.png")?;
    let negative = [
        (-5., "WHITE", "ERROR: wrong first  segment color"),
        (0., "GREEN", "ERROR: wrong middle segment color"),
        (5., "GRAY", "ERROR: wrong last   segment color"),
    ];
    for (value, expected, message) in negative {
        expect_found(&mut run, value, expected, message)?;
    }
    run.finish()
}

/// A demonstration scale at `x` with the given smooth transition and optional hue range.
fn smooth_demo(
    run: &mut SessionRun,
    name: &str,
    x: i32,
    smooth: bool,
    hue: Option<(f64, f64)>,
) -> Result<(), Box<dyn Error>> {
    color_scale(run, name, &|s| {
        demo(s);
        s.set_smooth_transition(smooth);
        if let Some((min, max)) = hue {
            s.set_hue_range(min, max);
        }
        s.set_position(x, 0);
    })
}

/// A user-coloured five-interval scale with 16-pixel text at `x`.
fn smooth_user(
    run: &mut SessionRun,
    name: &str,
    x: i32,
    smooth: bool,
) -> Result<(), Box<dyn Error>> {
    color_scale(run, name, &|s| {
        five_intervals(s);
        s.set_text_height(16);
        user_colors_and_labels(s);
        s.set_smooth_transition(smooth);
        s.set_position(x, 0);
    })
}

/// `bug28004`: smooth and stepped demonstration, user and hue-range scales, and free labels.
pub(crate) fn hue_and_free_labels() -> Outcome {
    banner(&[
        "============",
        "OCC28004: Visualization, AisColorScale - allow defining labels list not equal to intervals list",
        "============",
        "",
    ]);
    let mut run = SessionRun::new()?;
    run.session().set_axonometric()?;
    smooth_demo(&mut run, "cs1", 0, false, None)?;
    smooth_demo(&mut run, "cs2", 60, true, None)?;
    expect_pixels(&mut run, 15, &DEMO_ROWS)?;
    expect_pixels(&mut run, 15, &DEMO_ROWS)?;
    smooth_user(&mut run, "cs3", 120, false)?;
    smooth_user(&mut run, "cs4", 180, true)?;
    expect_pixels(&mut run, 135, &USER_ROWS)?;
    smooth_demo(&mut run, "cs5", 240, false, Some((300., 130.)))?;
    smooth_demo(&mut run, "cs6", 300, true, Some((300., 130.)))?;
    expect_pixels(&mut run, 245, &HUE_STEPPED)?;
    expect_pixels(&mut run, 315, &HUE_SMOOTH)?;
    color_scale(&mut run, "cs7", &|s| {
        demo(s);
        s.set_smooth_transition(false);
        s.set_color_range(
            Color::named(NamedColor::BLACK),
            Color::named(NamedColor::WHITE),
        );
        s.set_position(360, 0);
        s.set_labels(&["l1", "l2", "l3"].map(ExtendedString::from_utf8));
        s.set_label_type(Data::User);
    })?;
    run.dump("bug28004.png")?;
    run.finish()
}

/// The stepped hue-range colours at rows 20 to 370.
const HUE_STEPPED: [(i32, &str); 10] = [
    (20, "GREEN"),
    (60, "SPRINGGREEN"),
    (100, "CYAN2"),
    (140, "TURQUOISE2"),
    (180, "DODGERBLUE1"),
    (220, "BLUE"),
    (250, "BLUE"),
    (290, "BLUEVIOLET"),
    (330, "PURPLE"),
    (370, "MAGENTA"),
];

/// The smooth hue-range colours at rows 20 to 370.
const HUE_SMOOTH: [(i32, &str); 10] = [
    (20, "GREEN"),
    (60, "MEDIUMSPRINGGREEN"),
    (100, "CYAN2"),
    (140, "TURQUOISE2"),
    (180, "DEEPSKYBLUE2"),
    (220, "DODGERBLUE2"),
    (250, "ROYALBLUE2"),
    (290, "PURPLE3"),
    (330, "PURPLE"),
    (370, "MAGENTA2"),
];
