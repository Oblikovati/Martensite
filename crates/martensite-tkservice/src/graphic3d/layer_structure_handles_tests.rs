use super::*;
use crate::graphic3d::{GRAPHIC3D_ZLAYER_ID_DEFAULT, Graphic3dStructureManager};
use crate::null_driver::{NullCStructure, NullGraphicDriver};
use martensite_tkernel::standard::Handle;

fn structure(manager: &Handle<Graphic3dStructureManager>) -> Handle<dyn Graphic3dCStructure> {
    NullCStructure::new(manager).unwrap().cast()
}

#[test]
fn structure_handles_lists_every_priority_in_draw_order() {
    let driver = NullGraphicDriver::new_handle();
    let manager = Handle::new(Graphic3dStructureManager::new(&driver).unwrap());
    let layer = Graphic3dLayer::new(GRAPHIC3D_ZLAYER_ID_DEFAULT, None);
    let (low, high, normal) = (
        structure(&manager),
        structure(&manager),
        structure(&manager),
    );
    layer.add(&high, Graphic3dDisplayPriority::Highlight, false);
    layer.add(&low, Graphic3dDisplayPriority::Bottom, false);
    layer.add(&normal, Graphic3dDisplayPriority::Normal, false);
    let listed: Vec<_> = layer
        .structure_handles()
        .iter()
        .map(Handle::object_address)
        .collect();
    let copied: Vec<_> = layer
        .array_of_structures()
        .iter()
        .flat_map(|map| {
            map.iter()
                .map(|weak| weak.upgrade().object_address())
                .collect::<Vec<_>>()
        })
        .collect();
    assert_eq!(listed, copied);
    let expected: Vec<_> = [&low, &normal, &high]
        .iter()
        .map(|s| s.object_address())
        .collect();
    assert_eq!(listed, expected);
}
