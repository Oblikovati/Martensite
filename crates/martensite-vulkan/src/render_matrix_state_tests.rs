//! GlMatrixState: Push appends (or overwrites) the current matrix at the stack
//! head, Pop restores it, SetIdentity resets only the current matrix, and the templated
//! SetCurrent converts element by element.
use super::RenderMatrixState;
use martensite_tkernel::ncollection::{Mat4, Vec3};

fn translation(x: f64) -> Mat4<f64> {
    let mut matrix = Mat4::identity();
    matrix.translate(Vec3::new(x, 2., 3.));
    matrix
}

#[test]
fn push_and_pop_restore_matrices_last_in_first_out() {
    let mut state = RenderMatrixState::<f64>::default();
    assert_eq!(state.current(), &Mat4::identity());
    state.push();
    state.set_current(&translation(1.));
    state.push();
    *state.change_current() = translation(5.);
    assert_eq!((state.depth(), state.current()), (2, &translation(5.)));
    state.pop().unwrap();
    assert_eq!(state.current(), &translation(1.));
    state.pop().unwrap();
    assert_eq!((state.depth(), state.current()), (0, &Mat4::identity()));
}

#[test]
fn pop_of_an_empty_stack_is_rejected_and_keeps_the_current_matrix() {
    let mut state = RenderMatrixState::<f64>::default();
    state.set_current(&translation(4.));
    let error = state.pop().unwrap_err().to_string();
    assert!(error.contains("empty matrix stack"), "{error}");
    assert_eq!(state.current(), &translation(4.));
    // A reused head slot holds the newly pushed matrix (native SetValue), not the older one.
    state.push();
    state.pop().unwrap();
    state.set_current(&translation(6.));
    state.push();
    state.set_identity();
    state.pop().unwrap();
    assert_eq!(state.current(), &translation(6.));
}

#[test]
fn set_identity_and_converting_set_current_change_only_the_current_matrix() {
    let mut state = RenderMatrixState::<f32>::default();
    state.push();
    state.set_current_converted(&translation(0.1));
    assert_eq!(state.current(), &Mat4::<f32>::from_other(&translation(0.1)));
    assert_eq!(state.current().get_value(0, 3), 0.1_f32);
    state.set_identity();
    assert_eq!((state.depth(), state.current()), (1, &Mat4::identity()));
}
