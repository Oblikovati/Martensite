use crate::predicates::{OrientationSign, orient2d};

/// An original-coordinate projection; no rounded tangent construction is used.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CoordinateProjection {
    /// Keep X and Y.
    Xy,
    /// Keep X and Z.
    Xz,
    /// Keep Y and Z.
    Yz,
}
impl CoordinateProjection {
    pub(super) const ALL: [Self; 3] = [Self::Xy, Self::Xz, Self::Yz];
    fn project(self, point: [f64; 3]) -> [f64; 2] {
        match self {
            Self::Xy => [point[0], point[1]],
            Self::Xz => [point[0], point[2]],
            Self::Yz => [point[1], point[2]],
        }
    }
    pub(super) fn strict_orientation(self, corners: [[f64; 3]; 4]) -> Option<OrientationSign> {
        let projected = corners.map(|p| self.project(p));
        let mut common = OrientationSign::Zero;
        for i in 0..4 {
            let sign =
                orient2d(projected[i], projected[(i + 1) % 4], projected[(i + 2) % 4]).ok()?;
            if sign == OrientationSign::Zero || (i != 0 && sign != common) {
                return None;
            }
            common = sign;
        }
        Some(common)
    }
}
