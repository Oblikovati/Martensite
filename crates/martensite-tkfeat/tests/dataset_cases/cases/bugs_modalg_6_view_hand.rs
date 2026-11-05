//! The `bugs/modalg_6` feature scripts that check their result on the real renderer.
use super::bugs_modalg_5_hand::finish;
use crate::support::{
    checkshape::checkshape_named,
    construction::wire,
    feat_definitions::{featprism, featrevol},
    feat_perform::{Kas, featperform_from_until},
    feat_script::FeatScript,
    surface_variables::{circle, mkedge, mkface, plane_with_x},
    viewer_host::ViewerHost,
};

/// The disc of the circle at `center`, and the limiting faces at the heights `from` and `to`.
fn disc_and_limits(s: &mut FeatScript<'_>, center: [f64; 3], from: f64, to: f64) {
    let (normal, x_axis) = ([0., 0., 1.], [1., 0., 0.]);
    circle(s, "aCircle", center, normal, x_axis, 10.);
    mkedge(s, "anEdge", "aCircle");
    wire(s, "aWire", &["anEdge"]);
    // `mkplane aCircle aWire 0`: the plane is not forced.
    crate::support::construction::mkplane(s, "aCircle", "aWire");
    plane_with_x(s, "aFromPlane", [0., 0., from], normal, x_axis);
    mkface(s, "aFromPlane", "aFromPlane");
    plane_with_x(s, "aToPlane", [0., 0., to], normal, x_axis);
    mkface(s, "aToPlane", "aToPlane");
}

/// `vinit`, `vdisplay aResult`, `vsetdispmode aResult 1`, `vfit`.
fn shown_result(s: &FeatScript<'_>) -> ViewerHost {
    let mut host = ViewerHost::open();
    host.display("aResult", &s.shape_or_null("aResult"));
    host.set_display_mode("aResult", 1);
    host.fit();
    host
}

/// `vreadpixel x y name`, and the script's check that it is not the background.
fn pixel_check(host: &ViewerHost, x: i32, y: i32) {
    println!("vreadpixel {x} {y} name");
    let answer = host.read_pixel_name(x, y);
    println!("{answer}");
    if answer == "BLACK 0" {
        println!("ERROR: 26305 is reproduced. Prism is incorrect.");
    }
}

fn prism_case(
    s: &mut FeatScript<'_>,
    center: [f64; 3],
    heights: [f64; 2],
    from: &str,
    to: &str,
    pixel: [i32; 2],
    image: &str,
) {
    println!("========");
    println!("26305");
    println!("========");
    println!();
    disc_and_limits(s, center, heights[0], heights[1]);
    featprism(s, "aCircle", "aCircle", "aCircle", [0., 0., 1.], 2, 1);
    featperform_from_until(s, Kas::Prism, "aResult", from, to);
    checkshape_named(s, "aResult");
    let host = shown_result(s);
    pixel_check(&host, pixel[0], pixel[1]);
    finish(host, image);
}

pub fn bug26305_1(s: &mut FeatScript<'_>) {
    prism_case(
        s,
        [-9., -9., 0.],
        [-10., 10.],
        "aFromPlane",
        "aToPlane",
        [300, 340],
        "bug26305_1",
    );
}

pub fn bug26305_2(s: &mut FeatScript<'_>) {
    prism_case(
        s,
        [-10., -10., 0.],
        [-10., 10.],
        "aFromPlane",
        "aToPlane",
        [300, 340],
        "bug26305_2",
    );
}

pub fn bug26305_3(s: &mut FeatScript<'_>) {
    prism_case(
        s,
        [0., 0., 0.],
        [10., 20.],
        "aToPlane",
        "aFromPlane",
        [300, 300],
        "bug26305_3",
    );
}

pub fn bug26315(s: &mut FeatScript<'_>) {
    println!("========");
    println!("26315");
    println!("========");
    println!();
    disc_and_limits(s, [0., 0., 0.], -10., 10.);
    featrevol(
        s,
        "aCircle",
        "aCircle",
        "aCircle",
        [20., 0., 0.],
        [0., 1., 0.],
        2,
        1,
    );
    featperform_from_until(s, Kas::Revol, "aResult", "aFromPlane", "aToPlane");
    checkshape_named(s, "aResult");
    let host = shown_result(s);
    finish(host, "bug26315");
}
