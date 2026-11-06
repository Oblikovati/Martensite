//! STEPSelections: selections over STEP models, and the assembly and shape-count explorers.
mod assembly_explorer;
mod assembly_links;
mod counter;
mod select_derived;
mod select_for_transfer;
mod select_instances;
mod selects;

pub use assembly_explorer::STEPSelectionsAssemblyExplorer;
pub use assembly_links::{STEPSelectionsAssemblyComponent, STEPSelectionsAssemblyLink};
pub use counter::STEPSelectionsCounter;
pub use select_derived::STEPSelectionsSelectDerived;
pub use select_for_transfer::STEPSelectionsSelectForTransfer;
pub use select_instances::STEPSelectionsSelectInstances;
pub use selects::{
    STEPSelectionsSelectAssembly, STEPSelectionsSelectFaces, STEPSelectionsSelectGSCurves,
};

use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkxsbase::interface::InterfaceEntityIterator;

/// `InterfaceEntityIterator` as a list.
pub(crate) fn entities_of(
    iterator: InterfaceEntityIterator,
) -> Result<Vec<Handle<StandardTransient>>, Failure> {
    let mut entities = Vec::new();
    iterator.start();
    while iterator.more() {
        entities.push(iterator.value()?);
        iterator.next();
    }
    Ok(entities)
}

/// Declares what an `IFSelectSelectExplore` subclass of level -1 repeats: the struct, the runtime
/// type, its `Default`, and the parts of the selection every class shares. The class's
/// `explore` and `explore_label` come in `explore_body`.
macro_rules! explore_selection {
    ($(#[$meta:meta])* $class:ident, $recorded:literal) => {
        $(#[$meta])*
        pub struct $class {
            transient: martensite_tkernel::standard::StandardTransient,
            base: martensite_tkxsbase::if_select::IFSelectSelectExploreBase,
        }
        martensite_tkernel::impl_transient!($class, $recorded, dyn martensite_tkxsbase::if_select::IFSelectSelectExplore, transient, interfaces: [dyn martensite_tkxsbase::if_select::IFSelectSelection, dyn martensite_tkxsbase::if_select::IFSelectSelectDeduct, dyn martensite_tkxsbase::if_select::IFSelectSelectExplore]);
        impl Default for $class {
            fn default() -> Self {
                Self {
                    transient: Default::default(),
                    base: martensite_tkxsbase::if_select::IFSelectSelectExploreBase::new(-1),
                }
            }
        }
        impl martensite_tkxsbase::if_select::IFSelectSelectDeduct for $class {
            fn deduct_base(&self) -> &martensite_tkxsbase::if_select::IFSelectSelectDeductBase {
                &self.base.deduct
            }
        }
    };
}
pub(crate) use explore_selection;

/// `IFSelectSelectExplore`'s `root_result`, `fill_iterator` and `label`.
macro_rules! explore_selection_members {
    ($class:ident) => {
        impl martensite_tkxsbase::if_select::IFSelectSelection for $class {
            fn root_result(
                &self,
                g: &martensite_tkxsbase::interface::InterfaceGraph,
            ) -> Result<
                martensite_tkxsbase::interface::InterfaceEntityIterator,
                martensite_tkernel::standard::Failure,
            > {
                martensite_tkxsbase::if_select::explore_root_result(self, g)
            }
            fn fill_iterator(
                &self,
                iter: &mut martensite_tkxsbase::if_select::IFSelectSelectionIterator,
            ) {
                martensite_tkxsbase::if_select::deduct_fill_iterator(self, iter)
            }
            fn label(&self) -> martensite_tkernel::tcollection::AsciiString {
                martensite_tkxsbase::if_select::explore_label(self)
            }
        }
    };
}
pub(crate) use explore_selection_members;
