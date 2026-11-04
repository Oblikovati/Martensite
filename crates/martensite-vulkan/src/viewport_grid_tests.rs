use super::*;

const PRECISIONS: [RenderViewportGridPrecision; 3] = [
    RenderViewportGridPrecision::Fractional8,
    RenderViewportGridPrecision::Fractional10,
    RenderViewportGridPrecision::Fractional12,
];

#[test]
fn all_declared_precisions_round_half_cells_to_even_on_both_sides() {
    for precision in PRECISIONS {
        let scale = f64::from(1_u32 << precision.fractional_bits());
        for (input, expected) in [(0.5, 0.), (1.5, 2.), (2.5, 2.), (-0.5, -0.), (-1.5, -2.)] {
            assert_eq!(
                precision.snap(input / scale).unwrap().to_bits(),
                (expected / scale).to_bits()
            );
        }
    }
}

#[test]
fn adjacent_half_cell_neighbors_keep_their_distinct_ownership() {
    for precision in PRECISIONS {
        let half = 0.5 / f64::from(1_u32 << precision.fractional_bits());
        let below = f64::from_bits(half.to_bits() - 1);
        let above = f64::from_bits(half.to_bits() + 1);
        assert_eq!(precision.snap(below).unwrap(), 0.);
        assert_eq!(precision.snap(above).unwrap(), 2. * half);
    }
}

#[test]
fn every_grid_preserves_exact_coordinates_and_signed_zero() {
    for precision in PRECISIONS {
        for coordinate in [-409., -1., -0., 0., 1., 409.] {
            assert_eq!(
                precision.snap(coordinate).unwrap().to_bits(),
                coordinate.to_bits()
            );
        }
    }
}

#[test]
fn nonfinite_and_scaled_overflow_retain_offending_words() {
    for precision in PRECISIONS {
        for coordinate in [f64::INFINITY, f64::NEG_INFINITY, f64::NAN] {
            let error = precision.snap(coordinate).unwrap_err();
            assert_eq!(
                error,
                RenderViewportGridFailure::NonFiniteCoordinate {
                    bits: coordinate.to_bits()
                }
            );
            assert!(error.to_string().contains("expected finite"));
        }
        assert!(matches!(
            precision.snap(f64::MAX),
            Err(RenderViewportGridFailure::ScaledCoordinateOverflow { .. })
        ));
    }
}

#[test]
fn published_grid_contract_is_send_and_sync() {
    fn require_traits<T: Send + Sync>() {}
    require_traits::<RenderViewportGridPrecision>();
    require_traits::<RenderViewportGridFailure>();
}
