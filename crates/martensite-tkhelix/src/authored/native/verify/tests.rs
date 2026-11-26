// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use crate::authored::{
    ConstantHelixRequest, FittedHelix, HelixLead, HelixRadius, HelixRotation, HelixTraceLimits,
};
use martensite_tkmath::gp::Axis2;
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeWire};

fn fixture() -> FittedHelix {
    let law = ConstantHelix::new(ConstantHelixRequest {
        placement: Axis2::default(),
        radius: HelixRadius::Cone {
            opening: 4.,
            closing: 3.,
        },
        lead: HelixLead::PitchAndTurns {
            pitch: 2.,
            turns: 0.5,
        },
        rotation: HelixRotation::Negative,
    })
    .unwrap();
    law.fit(
        HelixFitOptions {
            max_turns: 1.,
            approximation_tolerance: 1e-8,
            max_segments: 16,
            max_evaluations: 2048,
            max_topology_tolerance: 1e-6,
            trace: HelixTraceLimits {
                max_position_error: 1e-6,
                max_native_spans: 16,
                max_cells: 64,
                max_operations: 100_000,
            },
        },
        || false,
    )
    .unwrap()
}

#[test]
fn valid_native_shorter_trim_cannot_be_published_as_the_authored_full_span() {
    let fit = fixture();
    let archived = fit.export_brep(|| false).unwrap();
    let same_carrier = BRepTool::curve_with_location(&archived.edge).unwrap();
    let shortened = BRepBuilderAPIMakeEdge::from_curve_params(
        &same_carrier.curve,
        same_carrier.first,
        0.5 * same_carrier.last,
    )
    .unwrap()
    .edge()
    .unwrap();
    let altered = BRepBuilderAPIMakeWire::from_edge(&shortened)
        .unwrap()
        .wire()
        .unwrap();
    let shortened = edge(&altered).unwrap();
    assert!(topology(&altered, &shortened, &mut || false).is_ok());
    assert!(matches!(
        retained(fit.law(), &altered, &shortened, fit.options(), &mut || false),
        Err(HelixFailure::Trace(cause)) if matches!(*cause, HelixTraceFailure::InvalidSpline)
    ));
    assert_eq!(
        BRepTool::curve_with_location(&archived.edge).unwrap().last,
        fit.history().native_parameter[1]
    );
}

#[test]
fn multi_coordinate_endpoint_gap_uses_outward_l1_and_refuses_nan() {
    assert_eq!(l1_gap([1., -0., 4.], [1., 0., 4.]), 0.);
    assert!(l1_gap([1., 2., 4.], [2., 4., 1.]) >= 6.);
    assert!(l1_gap([0., f64::NAN, 0.], [0.; 3]).is_nan());
}
