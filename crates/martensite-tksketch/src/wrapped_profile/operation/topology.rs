//! Preserve declared face orientation and insertion order in an operation-local shell.
pub(in crate::surface_sketch::wrapped_profile) use super::handedness::DevelopedMapHandedness;
use martensite_tkbrep::topods::{Builder, Face, Shell, Solid};
use martensite_tkernel::standard::Failure;

pub(in crate::surface_sketch::wrapped_profile) struct SolidAssembly {
    shell: Shell,
}
impl SolidAssembly {
    pub(in crate::surface_sketch::wrapped_profile) fn new() -> Self {
        let mut shell = Shell::default();
        Builder.make_shell(&mut shell);
        Self { shell }
    }

    pub(in crate::surface_sketch::wrapped_profile) fn add(
        &mut self,
        face: &Face,
        reverse: bool,
    ) -> Result<(), Failure> {
        if reverse {
            return Builder.add(&mut self.shell, &face.reversed());
        }
        Builder.add(&mut self.shell, face)
    }

    pub(in crate::surface_sketch::wrapped_profile) fn finish(self) -> Result<Solid, Failure> {
        self.shell.set_closed(true)?;
        let mut solid = Solid::default();
        Builder.make_solid(&mut solid);
        Builder.add(&mut solid, &self.shell)?;
        Ok(solid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn assembly_propagates_null_faces_before_finishing() {
        let mut assembly = SolidAssembly::new();
        assert!(assembly.add(&Face::default(), false).is_err());
        assert!(assembly.add(&Face::default(), true).is_err());
        assert!(!assembly.finish().unwrap().is_null());
    }
}
