//! The `StdSelect` package class: a namespace whose one member is a static drawer setter.
use super::std_select_set_drawer_for_brep_owner;
use crate::prs3d::Prs3dDrawer;
use crate::select_mgr::SelectMgrSelection;
use martensite_tkernel::standard::Handle;

/// `StdSelect`: the package class; `StdSelect::set_drawer_for_brep_owner(&selection, &drawer)`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct StdSelect;

impl StdSelect {
    /// `SetDrawerForBRepOwner`: leaves every owner unchanged, as the native body is commented out.
    /// Example: `StdSelect::set_drawer_for_brep_owner(&selection, &drawer)`.
    pub fn set_drawer_for_brep_owner(
        selection: &Handle<SelectMgrSelection>,
        drawer: &Handle<Prs3dDrawer>,
    ) {
        std_select_set_drawer_for_brep_owner(selection, drawer);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_drawer_for_brep_owner_leaves_the_selection_unchanged() {
        let selection = Handle::new(SelectMgrSelection::new(0));
        let drawer = Handle::new(Prs3dDrawer::new());
        StdSelect::set_drawer_for_brep_owner(&selection, &drawer);
        assert!(selection.is_empty());
        StdSelect::set_drawer_for_brep_owner(&Handle::default(), &Handle::default());
    }
}
