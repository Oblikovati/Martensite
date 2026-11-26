//! 4x4 inversion: explicit cofactor expansion in a fixed term order.
use super::Mat4;
use crate::ncollection::MatrixScalar;
use crate::standard::Failure;

impl<T: MatrixScalar> Mat4<T> {
    /// Writes the adjugate-based inverse and determinant.
    ///
    /// Returns false for a zero determinant; `out` then holds the unscaled cofactors,
    /// on purpose. Example: `matrix.inverted_into(&mut out, &mut det)`.
    #[inline]
    pub fn inverted_into(&self, output: &mut Self, determinant: &mut T) -> bool {
        let m = self.data();
        // 12 shared 2x2 sub-determinants (Laplace expansion) replace 48 redundant minors.
        let s0 = m[10] * m[15] - m[11] * m[14];
        let s1 = m[6] * m[15] - m[7] * m[14];
        let s2 = m[6] * m[11] - m[7] * m[10];
        let s3 = m[2] * m[15] - m[3] * m[14];
        let s4 = m[2] * m[11] - m[3] * m[10];
        let s5 = m[2] * m[7] - m[3] * m[6];

        let c0 = m[9] * m[15] - m[11] * m[13];
        let c1 = m[5] * m[15] - m[7] * m[13];
        let c2 = m[5] * m[11] - m[7] * m[9];
        let c3 = m[1] * m[15] - m[3] * m[13];
        let c4 = m[1] * m[11] - m[3] * m[9];
        let c5 = m[1] * m[7] - m[3] * m[5];

        let d0 = m[9] * m[14] - m[10] * m[13];
        let d1 = m[5] * m[14] - m[6] * m[13];
        let d2 = m[5] * m[10] - m[6] * m[9];
        let d3 = m[1] * m[14] - m[2] * m[13];
        let d4 = m[1] * m[10] - m[2] * m[9];
        let d5 = m[1] * m[6] - m[2] * m[5];

        let inverse: [T; 16] = [
            m[5] * s0 - m[9] * s1 + m[13] * s2,
            m[9] * s3 - m[1] * s0 - m[13] * s4,
            m[1] * s1 - m[5] * s3 + m[13] * s5,
            m[5] * s4 - m[1] * s2 - m[9] * s5,
            m[8] * s1 - m[4] * s0 - m[12] * s2,
            m[0] * s0 - m[8] * s3 + m[12] * s4,
            m[4] * s3 - m[0] * s1 - m[12] * s5,
            m[0] * s2 - m[4] * s4 + m[8] * s5,
            m[4] * c0 - m[8] * c1 + m[12] * c2,
            m[8] * c3 - m[0] * c0 - m[12] * c4,
            m[0] * c1 - m[4] * c3 + m[12] * c5,
            m[4] * c4 - m[0] * c2 - m[8] * c5,
            m[8] * d1 - m[4] * d0 - m[12] * d2,
            m[0] * d0 - m[8] * d3 + m[12] * d4,
            m[4] * d3 - m[0] * d1 - m[12] * d5,
            m[0] * d2 - m[4] * d4 + m[8] * d5,
        ];
        *determinant =
            m[0] * inverse[0] + m[1] * inverse[4] + m[2] * inverse[8] + m[3] * inverse[12];
        if *determinant == T::from(0) {
            *output = Self::from_data(inverse);
            return false;
        }
        let divisor = T::from(1) / *determinant;
        *output = Self::from_data(inverse.map(|value| value * divisor));
        true
    }
    /// The inverse without the determinant; `matrix.inverted_to(&mut out)`.
    pub fn inverted_to(&self, output: &mut Self) -> bool {
        let mut determinant: T = T::from(0);
        self.inverted_into(output, &mut determinant)
    }
    /// Returns the inverse or `Standard_ConstructionError`; `matrix.inverted()?`.
    pub fn inverted(&self) -> Result<Self, Failure> {
        let mut output = Self::default();
        if self.inverted_to(&mut output) {
            return Ok(output);
        }
        Err(Failure::with_type(
            "Standard_ConstructionError",
            "NCollection_Mat4::Inverted() - matrix has zero determinant",
            "",
        ))
    }
}
