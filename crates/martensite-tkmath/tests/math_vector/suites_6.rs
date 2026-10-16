#[test]
fn math_vector_multiply_row_vector_by_matrix_all_components_equal() {
    let aLow: i32 = 1;
    let aHigh: i32 = 6;
    let mut aVec = Vector::filled(aLow, aHigh, 5.0).unwrap();
    let mut aMat = Matrix::filled(aLow, aHigh, aLow, aHigh, 4.0).unwrap();
    let mut aResult = Vector::with_bounds(aLow, aHigh).unwrap();

    aResult.set_vector_matrix(&aVec, &aMat).unwrap();

    for i in aLow..=aHigh {
        assert_eq!(aResult[i], 120.0);
    }
}

#[test]
fn math_vector_t_multiply_row_vector_by_transposed_matrix_modified_component() {
    let aLow: i32 = 1;
    let aHigh: i32 = 6;
    let mut aVec = Vector::filled(aLow, aHigh, 5.0).unwrap();
    let mut aMat = Matrix::filled(aLow, aHigh, aLow, aHigh, 4.0).unwrap();
    let mut aResult = Vector::with_bounds(aLow, aHigh).unwrap();

    // Modify one element to create an asymmetry in the transposed multiply
    aMat[(aLow + 1, aLow)] += 1.0;

    aResult.set_vector_transpose_matrix(&aVec, &aMat).unwrap();

    assert_eq!(aResult[1], 120.0);
    assert_eq!(aResult[2], 125.0);
    for i in 3..=aHigh {
        assert_eq!(aResult[i], 120.0);
    }
}
