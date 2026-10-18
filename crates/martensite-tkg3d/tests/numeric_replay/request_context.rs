//! Surface replay parameters belong to the requesting operation, including interleaving.
use super::operations::{common::surface_summary, output::Output, surfaces::show_surface};
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::geom::{GeomGeometry, GeomPlane};
use martensite_tkmath::gp::Axis3;

fn expected_summary(parameters: &[f64]) -> String {
    let output = Output::captured();
    surface_summary(&output, &GeomPlane::new(Axis3::default()), parameters);
    output.finish()
}

#[test]
fn interleaved_surface_replays_keep_their_own_parameter_pairs() {
    let first = Output::captured();
    let second = Output::captured();
    first.set_parameters(vec![0.25, 0.5]);
    second.set_parameters(vec![-2., 4., 3., -1.]);
    let plane: Handle<dyn GeomGeometry> = Handle::new(GeomPlane::new(Axis3::default())).cast();
    show_surface(&second, &plane);
    show_surface(&first, &plane);
    assert_eq!(first.finish(), expected_summary(&[0.25, 0.5]));
    assert_eq!(second.finish(), expected_summary(&[-2., 4., 3., -1.]));
}
