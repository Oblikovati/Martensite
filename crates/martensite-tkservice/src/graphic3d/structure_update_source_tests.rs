//! The layer an update of one structure reports to the views (#594).
use super::update_source;
use crate::graphic3d::*;
use crate::null_driver::NullGraphicDriver;
use martensite_tkernel::standard::Handle;

fn structure(manager: &Handle<Graphic3dStructureManager>) -> Handle<dyn Graphic3dStructureSource> {
    let structure = Graphic3dStructure::new(manager, &Handle::default()).unwrap();
    let source: Handle<dyn Graphic3dStructureSource> = Handle::new(structure).cast();
    graphic3d_structure_bind(&source);
    source
}

#[test]
fn an_update_reports_the_structure_layer_unless_others_connect_it() {
    let driver = NullGraphicDriver::new_handle();
    let manager = Handle::new(Graphic3dStructureManager::new(&driver).unwrap());
    let (parent, child) = (structure(&manager), structure(&manager));
    let (parent_ref, child_ref) = (parent.get().unwrap(), child.get().unwrap());
    assert_eq!(update_source(child_ref.structure_base(), 3), 3);
    parent_ref.connect(&child, Graphic3dTypeOfConnection::Descendant, false);
    assert_eq!(update_source(parent_ref.structure_base(), 3), 3);
    assert_eq!(
        update_source(child_ref.structure_base(), 3),
        GRAPHIC3D_ZLAYER_ID_UNKNOWN,
        "a connected structure draws in its ancestors' layers"
    );
}
