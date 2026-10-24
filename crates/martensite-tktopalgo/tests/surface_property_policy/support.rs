use martensite_tkgeombase::gprop::GProps;

pub(super) fn property_bits(props: &GProps) -> Vec<u64> {
    let mut numbers = vec![
        props.dim,
        props.loc.x(),
        props.loc.y(),
        props.loc.z(),
        props.g.x(),
        props.g.y(),
        props.g.z(),
    ];
    for row in 1..=3 {
        for column in 1..=3 {
            numbers.push(props.inertia.value(row, column).unwrap());
        }
    }
    numbers.into_iter().map(f64::to_bits).collect()
}

pub(super) fn assert_inertia(props: &GProps, expected: [[f64; 3]; 3]) {
    let actual = props.matrix_of_inertia();
    for (row, columns) in expected.iter().enumerate() {
        for (column, expected) in columns.iter().enumerate() {
            let observed = actual.value(row as i32 + 1, column as i32 + 1).unwrap();
            assert!(
                (observed - expected).abs() < 1e-12,
                "({row},{column}): {observed} versus {expected}"
            );
        }
    }
}
