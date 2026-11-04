//! The private structure classes of the V3d grids and trihedron
//! (`V3dRectangularGrid::RectangularGridStructure`, `V3dCircularGrid::CircularGridStructure`,
//! `V3dTrihedron::TrihedronStructure`): a `Graphic3dStructure` whose `Compute` asks its owner
//! to rebuild the groups. The owner is a weak handle (a raw pointer natively).
use martensite_tkernel::standard::{Failure, Handle, StandardTransient, Transient, WeakHandle};
use martensite_tkservice::graphic3d::{
    Graphic3dStructureBase, Graphic3dStructureManager, Graphic3dStructureSource,
    graphic3d_structure_bind,
};

/// The owner of a computed structure: rebuilds the structure's groups.
pub trait V3dStructureComputer: Transient {
    /// The native `Compute` override body; `owner.compute_structure()`.
    fn compute_structure(&self);
}
martensite_tkernel::impl_transient_interface!(
    dyn V3dStructureComputer,
    "V3dStructureComputer",
    StandardTransient
);

/// A structure whose `Compute` calls back its owner.
///
/// Example: `let s = V3dComputedStructure::create(&manager, &grid.clone().cast())?;`.
pub struct V3dComputedStructure {
    transient: StandardTransient,
    base: Graphic3dStructureBase,
    owner: WeakHandle<dyn V3dStructureComputer>,
}
#[path = "computed_structure_type.rs"]
mod inherited_type;

impl V3dComputedStructure {
    /// A structure of a manager computed by `owner`; the manager without driver is refused.
    /// Example: `V3dComputedStructure::create(&manager, &owner)?`.
    pub fn create(
        manager: &Handle<Graphic3dStructureManager>,
        owner: &Handle<dyn V3dStructureComputer>,
    ) -> Result<Handle<dyn Graphic3dStructureSource>, Failure> {
        let structure = Handle::new(Self {
            transient: StandardTransient::default(),
            base: Graphic3dStructureBase::new(manager, &Handle::default())?,
            owner: owner.downgrade(),
        });
        let structure: Handle<dyn Graphic3dStructureSource> = structure.cast();
        graphic3d_structure_bind(&structure);
        Ok(structure)
    }
}

impl Graphic3dStructureSource for V3dComputedStructure {
    fn structure_base(&self) -> &Graphic3dStructureBase {
        &self.base
    }
    /// `Compute`: the owner's rebuild, nothing once the owner is gone.
    fn compute(&self) {
        if let Some(owner) = self.owner.upgrade().get() {
            owner.compute_structure();
        }
    }
}
