//! Downstream compatibility fixtures for the safe-storage migration (#81, #88).
use martensite_tkernel::ncollection::{Mat3, Mat4, Vec2, Vec3, Vec4};

// Deliberately neither Copy nor Clone, and not restricted to 'static.
struct BorrowedComponent<'a>(&'a str);

#[test]
fn named_vector_fields_and_const_array_borrows_remain_available() {
    const TWO: [i32; 2] = *Vec2 { x: 1, y: 2 }.as_components();
    const THREE: [i32; 3] = *Vec3 { x: 1, y: 2, z: 3 }.as_components();
    const FOUR: [i32; 4] = {
        let mut vector = Vec4 {
            x: 1,
            y: 2,
            z: 3,
            w: 4,
        };
        vector.as_components_mut()[3] = 5;
        *vector.as_components()
    };
    assert_eq!(TWO, [1, 2]);
    assert_eq!(THREE, [1, 2, 3]);
    assert_eq!(FOUR, [1, 2, 3, 5]);
}

#[test]
fn vector_array_view_retains_borrowed_noncopy_fields_and_identity() {
    let text = String::from("borrowed");
    let mut vector = Vec3 {
        x: BorrowedComponent(&text),
        y: BorrowedComponent("second"),
        z: BorrowedComponent("third"),
    };
    assert_eq!(vector.as_ptr(), std::ptr::from_ref(&vector.x));
    assert_eq!(vector.as_components()[0].0, "borrowed");
    vector.as_components_mut()[1] = BorrowedComponent(&text);
    assert_eq!(vector.y.0, "borrowed");
    assert_eq!(vector.as_mut_ptr(), std::ptr::from_mut(&mut vector.x));
}

macro_rules! matrix_borrow_fixture {
    ($test:ident, $matrix:ident, $count:literal) => {
        #[test]
        fn $test() {
            let text = String::from("borrowed");
            let mut elements = std::array::from_fn::<_, $count, _>(|_| BorrowedComponent(&text));
            let pointer = elements.as_ptr();
            let matrix: &$matrix<BorrowedComponent<'_>> = $matrix::map_ref(&elements);
            assert_eq!(matrix.data_ptr(), pointer);
            assert_eq!(matrix.as_slice()[0].0, "borrowed");
            let matrix: &mut $matrix<BorrowedComponent<'_>> = $matrix::map_ref_mut(&mut elements);
            assert_eq!(matrix.data_mut_ptr().cast_const(), pointer);
            matrix.as_mut_slice()[$count - 1] = BorrowedComponent("changed");
            assert_eq!(elements[$count - 1].0, "changed");
        }
    };
}

matrix_borrow_fixture!(
    mat3_borrow_preserves_identity_without_scalar_bounds,
    Mat3,
    9
);
matrix_borrow_fixture!(
    mat4_borrow_preserves_identity_without_scalar_bounds,
    Mat4,
    16
);
