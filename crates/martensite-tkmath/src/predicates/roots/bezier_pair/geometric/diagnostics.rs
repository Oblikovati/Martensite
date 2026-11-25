// SPDX-License-Identifier: GPL-2.0-only
//! Diagnostics retain the actual witness operands, rather than only a completion label.
use super::*;
impl std::fmt::Debug for SectorWitness {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        out.debug_struct("SectorWitness")
            .field("vectors", &self.vectors)
            .field("first", &self.first)
            .field("signs", &self.signs)
            .finish()
    }
}
impl std::fmt::Debug for HullWitness {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        out.debug_struct("HullWitness")
            .field("edge", &self.edge)
            .field("side", &self.side)
            .field("signs", &self.signs)
            .finish()
    }
}
impl std::fmt::Debug for GeometricProof {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Box { axis, bounds } => out
                .debug_struct("Box")
                .field("axis", axis)
                .field("bounds", bounds)
                .finish(),
            Self::Monotone(witness) => out.debug_tuple("Monotone").field(witness).finish(),
            Self::Endpoint { sector, contact } => out
                .debug_struct("Endpoint")
                .field("sector", sector)
                .field("contact", contact)
                .finish(),
            Self::Hull(witness) => out.debug_tuple("Hull").field(witness).finish(),
        }
    }
}
