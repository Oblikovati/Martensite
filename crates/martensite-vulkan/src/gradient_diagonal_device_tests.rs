//! Diagonal GPU workloads share the existing foreground and lifecycle verification path.
use super::*;
use martensite_tkservice::aspect::AspectGradientFillMethod::{Diagonal1, Diagonal2};

const DIAGONAL_DEVICE_CASES: [matrix::GradientMatrixCase; 5] = [
    matrix::GradientMatrixCase {
        label: "diagonal1-97x43-0",
        extent: [97, 43],
        method: Diagonal1,
        foreground: false,
    },
    matrix::GradientMatrixCase {
        label: "diagonal2-43x97-0",
        extent: [43, 97],
        method: Diagonal2,
        foreground: false,
    },
    matrix::GradientMatrixCase {
        label: "diagonal1-97x43-1",
        extent: [97, 43],
        method: Diagonal1,
        foreground: true,
    },
    matrix::GradientMatrixCase {
        label: "diagonal2-43x97-1",
        extent: [43, 97],
        method: Diagonal2,
        foreground: true,
    },
    matrix::GradientMatrixCase {
        label: "return-diagonal1-97x43-0",
        extent: [97, 43],
        method: Diagonal1,
        foreground: false,
    },
];

#[test]
fn diagonal_device_matrix_preserves_foreground_depth_and_resized_solid_transitions() {
    let mut renderer = gradient_device_renderer();
    for case in DIAGONAL_DEVICE_CASES {
        let gradient = renderer.render(&matrix::matrix_frame(case, true)).unwrap();
        let solid = renderer.render(&matrix::matrix_frame(case, false)).unwrap();
        matrix_checks::assert_matrix_pair(case, &gradient, &solid);
        retain_gradient_device_capture(&gradient, &format!("matrix-{}", case.label));
        retain_gradient_device_capture(&solid, &format!("matrix-{}-solid", case.label));
    }
    assert_diagonal_device_validation(&renderer);
}

fn assert_diagonal_device_validation(renderer: &VulkanRenderer) {
    println!(
        "{{\"device\":{:?},\"diagonal_matrix_pairs\":5}}",
        renderer.device_name()
    );
    assert!(
        renderer.validation_errors().unwrap().is_empty(),
        "{:?}",
        renderer.validation_errors().unwrap()
    );
}

/// Reuse the exact submitted cases; `policy_diagonal_cases()` preserves authored odd extents.
pub(super) fn policy_diagonal_cases() -> [matrix::GradientMatrixCase; 5] {
    // Keep both policy paths on the same authored odd-sized workload.
    DIAGONAL_DEVICE_CASES
}
