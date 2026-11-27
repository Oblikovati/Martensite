//! The custom uniform variables of a Graphic3dShaderProgram as flat uniform values
//! (GlShaderProgram::ApplyVariables): floats, integer vectors and column-major matrices.
use super::uniform_values::UserUniformValues;
use martensite_tkernel::ncollection::{Mat3, Mat4, Vec2, Vec3, Vec4};
use martensite_tkservice::graphic3d::{
    Graphic3dShaderProgram, Graphic3dValueAccess, Graphic3dValueInterface,
};

/// Every variable with a supported value, in push order (a later push of a name wins when
/// the block is filled, as the native program applies them in order);
/// `program_variables(&program)[0].0 == "uColor"`.
pub fn program_variables(program: &Graphic3dShaderProgram) -> Vec<(String, UserUniformValues)> {
    let mut variables: Vec<(String, UserUniformValues)> = Vec::new();
    for variable in program.variables() {
        let Some(value) = variable.value() else {
            continue;
        };
        let Some(values) = flat_values(value) else {
            continue;
        };
        let name = String::from_utf8_lossy(variable.name().as_bytes()).into_owned();
        variables.retain(|(known, _)| *known != name);
        variables.push((name, values));
    }
    variables
}

/// The flat components of one uniform value, or `None` for an unsupported type.
fn flat_values(value: &dyn Graphic3dValueInterface) -> Option<UserUniformValues> {
    float_values(value)
        .map(UserUniformValues::Float)
        .or_else(|| int_values(value).map(UserUniformValues::Int))
}

fn float_values(value: &dyn Graphic3dValueInterface) -> Option<Vec<f32>> {
    if let Some(v) = value.as_value::<f32>() {
        return Some(vec![*v]);
    }
    if let Some(v) = value.as_value::<Vec2<f32>>() {
        return Some(v.data().to_vec());
    }
    if let Some(v) = value.as_value::<Vec3<f32>>() {
        return Some(v.data().to_vec());
    }
    if let Some(v) = value.as_value::<Vec4<f32>>() {
        return Some(v.data().to_vec());
    }
    matrix_values(value)
}

fn matrix_values(value: &dyn Graphic3dValueInterface) -> Option<Vec<f32>> {
    if let Some(matrix) = value.as_value::<Mat3<f32>>() {
        return Some(matrix.as_slice().to_vec());
    }
    value
        .as_value::<Mat4<f32>>()
        .map(|matrix| matrix.as_slice().to_vec())
}

fn int_values(value: &dyn Graphic3dValueInterface) -> Option<Vec<i32>> {
    if let Some(v) = value.as_value::<i32>() {
        return Some(vec![*v]);
    }
    if let Some(v) = value.as_value::<Vec2<i32>>() {
        return Some(v.data().to_vec());
    }
    if let Some(v) = value.as_value::<Vec3<i32>>() {
        return Some(v.data().to_vec());
    }
    value.as_value::<Vec4<i32>>().map(|v| v.data().to_vec())
}
