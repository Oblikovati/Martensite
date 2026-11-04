//! Explicit arithmetic follows clip_position.glsl; earlier versions retain separate operations.
use crate::RenderVertex;

pub(super) fn clip(matrix: [f32; 16], vertex: RenderVertex) -> [f32; 4] {
    let [x, y, z] = vertex.position;
    std::array::from_fn(|row| {
        let xy = matrix[row].mul_add(x, matrix[4 + row] * y);
        let xyz = matrix[8 + row].mul_add(z, xy);
        matrix[12 + row].mul_add(vertex.weight, xyz)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cancellation_retains_the_exact_product_residue_in_all_components() {
        // (1+2^-23)*(1-2^-23)-1 = -2^-46; separate product rounding erases it.
        let mut matrix = [0.; 16];
        for row in 0..4 {
            matrix[row] = 1. + (martensite_cmath::powi(f64::from(2f32), -23) as f32);
            matrix[4 + row] = -1.;
        }
        let vertex = RenderVertex::new(
            [
                1. - (martensite_cmath::powi(f64::from(2f32), -23) as f32),
                1.,
                0.,
            ],
            [1.; 4],
        );
        assert_eq!(
            clip(matrix, vertex),
            [-(martensite_cmath::powi(f64::from(2f32), -46) as f32); 4]
        );
        assert_eq!(matrix[0] * vertex.position[0] - 1., 0.);
    }
    #[test]
    fn perspective_weight_and_depth_use_the_same_fused_order() {
        let mut matrix = crate::identity_clip();
        matrix[3] = 0.5;
        matrix[11] = 0.25;
        let mut vertex = RenderVertex::new([0.5, -0.25, 0.75], [1.; 4]);
        vertex.weight = 2.;
        assert_eq!(clip(matrix, vertex), [0.5, -0.25, 0.75, 2.4375]);
    }
}
