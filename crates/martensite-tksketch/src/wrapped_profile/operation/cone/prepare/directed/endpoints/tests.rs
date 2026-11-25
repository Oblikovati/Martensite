use super::*;
fn span() -> CurveSpan {
    CurveSpan {
        curve: SurfaceCurveId(3),
        index: 0,
        level: 0,
    }
}
fn corner(numerator: u32, level: u8) -> CornerSource {
    CornerSource::Split {
        curve: SurfaceCurveId(3),
        numerator,
        level,
    }
}
fn rows() -> [[f64; 3]; 2] {
    [[2., 0., -1.], [2., 0., 1.]]
}
#[test]
fn final_table_uses_canonical_source_incidence_for_both_physical_rows() {
    let table = table();
    assert_eq!(
        table
            .lookup(SurfaceRegionId(1), span(), corner(2, 2), [4., 0.])
            .unwrap(),
        rows()
    );
    assert!(
        table
            .lookup(SurfaceRegionId(1), span(), corner(3, 2), [4., 0.])
            .is_err()
    );
}
#[test]
fn final_table_refuses_source_and_shared_xyz_bit_mutations() {
    let mut table = table();
    assert!(
        table
            .lookup(SurfaceRegionId(1), span(), corner(1, 1), [4., -0.])
            .is_err()
    );
    let mut changed = rows();
    changed[1][2] = changed[1][2].next_up();
    let failure = insert(&mut table, rows(), changed).unwrap_err();
    assert_eq!(failure.region, Some(SurfaceRegionId(1)));
    assert_eq!(failure.span, Some(span()));
    assert!(matches!(
        failure.cause,
        ConeCause::Binding {
            field: "final shared incidence",
            ..
        }
    ));
}

fn table() -> FinalEndpoints {
    let mut table = FinalEndpoints::default();
    table
        .insert(
            SurfaceRegionId(1),
            span(),
            corner(1, 1),
            [4., 0.],
            rows(),
            rows(),
        )
        .unwrap();
    table
}

fn insert(
    table: &mut FinalEndpoints,
    nearest: [[f64; 3]; 2],
    points: [[f64; 3]; 2],
) -> ConeChecked<()> {
    table.insert(
        SurfaceRegionId(1),
        span(),
        corner(2, 2),
        [4., 0.],
        nearest,
        points,
    )
}
#[test]
fn identical_final_rows_do_not_hide_different_nearest_source_incidence() {
    let mut table = table();
    let mut nearest = rows();
    nearest[0][2] = nearest[0][2].next_down();
    assert!(insert(&mut table, nearest, rows()).is_err());
}
