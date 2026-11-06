//! Ranges and interval colours: linear and logarithmic ranges, uniform-lightness colours.
use super::prelude::*;

/// Set the range and the number of intervals.
fn range(scale: &AisColorScale, (min, max): (f64, f64), intervals: i32) {
    scale.set_range(min, max);
    scale.set_number_of_intervals(intervals);
}

/// `bug22632`: a linear scale, then logarithmic scales over several ranges and intervals.
pub(crate) fn logarithmic_ranges() -> Outcome {
    banner(&[
        "============",
        "OCC22632: Display logarithmic colorscale.",
        "============",
        "",
    ]);
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    run.session().set_axonometric()?;
    color_scale(&mut run, "cs", &|s| range(s, (0., 1000.), 3))?;
    run.dump("bug22632_1.png")?;
    color_scale(&mut run, "cs", &|s| {
        range(s, (0., 1000.), 3);
        s.set_logarithmic(true);
    })?;
    run.dump("bug22632_2.png")?;
    let later = [
        ((5., 200.), 4, "bug22632_3.png"),
        ((1., 1568.), 8, "bug22632_4.png"),
        ((3., 500.), 5, "bug22632_5.png"),
        ((1., 1000.), 6, "bug22632_6.png"),
    ];
    for (bounds, intervals, image) in later {
        color_scale(&mut run, "cs", &|s| range(s, bounds, intervals))?;
        run.dump(image)?;
    }
    run.finish()
}

/// A 10-interval scale over 0..1 at `x` with a title, optionally with uniform-lightness colours.
fn titled_scale(
    run: &mut SessionRun,
    name: &str,
    x: i32,
    uniform_lightness: Option<f64>,
    title: &str,
) -> Result<(), Box<dyn Error>> {
    color_scale(run, name, &|s| {
        range(s, (0., 1.), 10);
        s.set_position(x, 0);
        if let Some(lightness) = uniform_lightness {
            s.set_uniform_colors(lightness, 300., 40.)
                .expect("uniform colours over a valid hue range");
            s.set_color_type(Data::User);
        }
        s.set_title(&ExtendedString::from_utf8(title));
    })
}

/// `bug31454`: an HSL scale next to five scales of uniform lightness 30 to 70.
pub(crate) fn uniform_lightness() -> Outcome {
    banner(&[
        "============",
        "0031454: Visualization - perceptually uniform color scale",
        "============",
        "",
    ]);
    let mut run = SessionRun::with_view_size(600, 409)?;
    run.session().set_axonometric()?;
    titled_scale(&mut run, "hsl", 0, None, "HSL")?;
    for (index, lightness) in [30., 40., 50., 60., 70.].into_iter().enumerate() {
        let name = format!("lch{lightness}");
        let title = format!("L={lightness}");
        titled_scale(
            &mut run,
            &name,
            100 * (index as i32 + 1),
            Some(lightness),
            &title,
        )?;
    }
    run.dump("bug31454.png")?;
    run.finish()
}
