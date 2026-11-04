//! The ordinary clip matrix is the float GLSL chain `(P * W) * M`, not a rounded double product.
use super::ordinary_clip;
use martensite_tkernel::ncollection::Mat4;
use martensite_tkmath::gp::{Direction3, Point3};
use martensite_tkservice::graphic3d::Graphic3dCamera;

/// Column-major float product with in-order column sums, as Mesa lowers a GLSL `mat4 * mat4`.
fn glsl_product(left: [f32; 16], right: [f32; 16]) -> [f32; 16] {
    let mut out = [0f32; 16];
    for column in 0..4 {
        for row in 0..4 {
            let mut sum = left[row] * right[column * 4];
            for k in 1..4 {
                sum += left[k * 4 + row] * right[column * 4 + k];
            }
            out[column * 4 + row] = sum;
        }
    }
    out
}

fn oblique_camera() -> Graphic3dCamera {
    let camera = Graphic3dCamera::default();
    camera.set_eye(&Point3::new(431.7, -213.3, 377.1)).unwrap();
    camera.set_center(&Point3::new(17.3, 41.9, -3.7)).unwrap();
    camera.set_up(&Direction3::new(-0.41, 0.41, 0.81).unwrap());
    camera.set_scale(271.3);
    camera
}

#[test]
fn ordinary_clip_is_the_float_glsl_chain_of_float_camera_matrices() {
    let camera = oblique_camera();
    let mut model = Mat4::<f64>::identity();
    model.set_column(
        3,
        martensite_tkernel::ncollection::Vec4::new(1.25, -3.5, 0.75, 1.),
    );
    let float_model = Mat4::<f32>::from_other(&model).data();
    let view = glsl_product(
        camera.projection_matrix_f().data(),
        camera.orientation_matrix_f().data(),
    );
    assert_eq!(
        ordinary_clip(&camera, model),
        glsl_product(view, float_model)
    );
}

#[test]
fn rounding_the_double_product_is_not_the_native_matrix() {
    // Guards the reason for the float chain: the two differ in the last bits.
    let camera = oblique_camera();
    let double = (camera.projection_matrix() * camera.orientation_matrix())
        .data()
        .map(|value| value as f32);
    assert_ne!(ordinary_clip(&camera, Mat4::identity()), double);
}
