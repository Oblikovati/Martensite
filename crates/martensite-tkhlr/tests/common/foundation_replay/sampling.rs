//! Original controlled-adaptor sampling report over request-local tokens.
use super::geometry_records::common::Reader;
use super::{
    common::{Sink, end, int, real, show},
    sampling_fixture::sampling_surface,
    surface_setup::SurfaceSetup,
};
use martensite_tkg3d::adaptor3d::h_surface_tool::Adaptor3dSurfaceHandle;
use martensite_tkhlr::contap::ContapHContTool as Tool;
const INDICES: [i32; 10] = [-1, 0, 1, 2, 3, 4, 5, 6, 17, 40];

fn sample_pair(
    tokens: &mut Sink,
    surface: &Adaptor3dSurfaceHandle,
    index: i32,
    setup: &SurfaceSetup,
) {
    show(
        tokens,
        setup.sample_point(surface, index),
        |tokens, (u, v)| {
            real(tokens, u);
            real(tokens, v);
        },
    );
}

/// Replay historical range sharing and partial refreshes; `sampling_case(tokens, values)`.
pub fn sampling_case(tokens: &mut Sink, values: &[f64]) {
    sampling_replay(tokens, values, &mut SurfaceSetup::Historical);
}

fn sampling_replay(tokens: &mut Sink, values: &[f64], setup: &mut SurfaceSetup) {
    let mut reader = Reader::new(values);
    let baseline = sampling_surface(&mut reader);
    let changed = sampling_surface(&mut reader);
    show(tokens, setup.sample_count(&baseline), int);
    show(tokens, Tool::nb_samples_u(&changed, -1., 2.), int);
    show(tokens, Tool::nb_samples_v(&changed, -1., 2.), int);
    for index in INDICES {
        sample_pair(tokens, &changed, index, setup);
    }
    show(tokens, setup.sample_count(&changed), int);
    for index in INDICES {
        sample_pair(tokens, &baseline, index, setup);
        sample_pair(tokens, &changed, index, setup);
    }
    end(tokens);
}

/// Replay the same controlled sampling report with explicitly owned bounds.
/// `case_with_context(tokens, values, context)`.
pub fn case_with_context(
    tokens: &mut Sink,
    values: &[f64],
    context: &mut martensite_tkhlr::contap::ContapSamplingContext,
) {
    sampling_replay(tokens, values, &mut SurfaceSetup::Local(context));
}
