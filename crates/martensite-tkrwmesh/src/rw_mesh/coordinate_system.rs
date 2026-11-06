//! Axis conventions and their aliases.
use super::mesh_integer_setting;

mesh_integer_setting!(
    RwMeshCoordinateSystem,
    "Axis convention for exchanged mesh coordinates; `RwMeshCoordinateSystem::Z_UP`."
);

impl RwMeshCoordinateSystem {
    /// No declared axis convention.
    pub const UNDEFINED: Self = Self(-1);
    /// Positive Y forward and positive Z up.
    pub const POS_Y_FORWARD_POS_Z_UP: Self = Self(0);
    /// Negative Z forward and positive Y up.
    pub const NEG_Z_FORWARD_POS_Y_UP: Self = Self(1);
    /// Blender's positive-Y-forward, positive-Z-up convention.
    pub const BLENDER: Self = Self::POS_Y_FORWARD_POS_Z_UP;
    /// glTF's negative-Z-forward, positive-Y-up convention.
    pub const GLTF: Self = Self::NEG_Z_FORWARD_POS_Y_UP;
    /// The Z-up alias of the Blender convention.
    pub const Z_UP: Self = Self::BLENDER;
    /// The Y-up alias of the glTF convention.
    pub const Y_UP: Self = Self::GLTF;
}

#[cfg(test)]
mod tests {
    use super::RwMeshCoordinateSystem as System;

    #[test]
    fn coordinate_conventions_and_all_aliases_keep_their_declared_values() {
        assert_eq!(System::UNDEFINED.as_raw(), -1);
        assert_eq!(System::POS_Y_FORWARD_POS_Z_UP.as_raw(), 0);
        assert_eq!(System::NEG_Z_FORWARD_POS_Y_UP.as_raw(), 1);
        assert_eq!(System::BLENDER, System::POS_Y_FORWARD_POS_Z_UP);
        assert_eq!(System::GLTF, System::NEG_Z_FORWARD_POS_Y_UP);
        assert_eq!(System::Z_UP, System::BLENDER);
        assert_eq!(System::Y_UP, System::GLTF);
    }
}
