//! Sensitive arrays and segments retain their original owner and sensitivity factors.
use super::*;
impl SelectMgrSelectableObject for PrimitiveArrayPresentation {
    fn selectable_base(&self) -> &SelectMgrSelectableObjectBase {
        &self.base.selectable
    }
    fn compute_selection(
        &self,
        selection: &dyn HandleReference<SelectMgrSelection>,
        mode: i32,
    ) -> Result<(), Failure> {
        let parray = self.primitive_array();
        let Some(array) = parray.get().filter(|_| mode == 0) else {
            return Ok(());
        };
        let this: Handle<dyn SelectMgrSelectableObject> = self.presentable_base().this().cast();
        let owner = SelectMgrEntityOwner::with_selectable(&this, 0).into_cast();
        let selection = selection.observe();
        select_array(&parray, array, &owner, &selection)
    }
}
fn select_array(
    parray: &Handle<Graphic3dArrayOfPrimitives>,
    array: &Graphic3dArrayOfPrimitives,
    owner: &Handle<dyn crate::select_mgr::SelectMgrEntityOwnerSource>,
    selection: &SelectMgrSelection,
) -> Result<(), Failure> {
    if !parray.cast::<Graphic3dArrayOfTriangles>().is_null() {
        return select_triangles(array, owner, selection);
    }
    if !parray.cast::<Graphic3dArrayOfSegments>().is_null() {
        return select_segments(array, owner, selection);
    }
    select_points(array, owner, selection)
}

fn select_triangles(
    array: &Graphic3dArrayOfPrimitives,
    owner: &Handle<dyn crate::select_mgr::SelectMgrEntityOwnerSource>,
    selection: &SelectMgrSelection,
) -> Result<(), Failure> {
    let sensitive = Select3DSensitivePrimitiveArray::new(owner)?;
    sensitive.init_triangulation_all(
        &array.attributes(),
        &array.indices(),
        &Location::default(),
        true,
        1,
    )?;
    selection.add(&Handle::new(sensitive).cast())
}
fn select_points(
    array: &Graphic3dArrayOfPrimitives,
    owner: &Handle<dyn crate::select_mgr::SelectMgrEntityOwnerSource>,
    selection: &SelectMgrSelection,
) -> Result<(), Failure> {
    let sensitive = Select3DSensitivePrimitiveArray::new(owner)?;
    sensitive.set_sensitivity_factor(8)?;
    sensitive.init_points_all(
        &array.attributes(),
        &array.indices(),
        &Location::default(),
        true,
        1,
    )?;
    selection.add(&Handle::new(sensitive).cast())
}
fn select_segments(
    array: &Graphic3dArrayOfPrimitives,
    owner: &Handle<dyn crate::select_mgr::SelectMgrEntityOwnerSource>,
    selection: &SelectMgrSelection,
) -> Result<(), Failure> {
    let by_edges = array.edge_number() > 0;
    let count = segment_count(array, by_edges);
    let mut rank = 1;
    while rank <= count {
        let (a, b) = segment_ranks(array, by_edges, rank)?;
        let segment = Select3DSensitiveSegment::new(owner, array.vertice(a)?, array.vertice(b)?);
        segment.set_sensitivity_factor(4)?;
        selection.add(&Handle::new(segment).cast())?;
        rank += 2;
    }
    Ok(())
}

fn segment_ranks(
    array: &Graphic3dArrayOfPrimitives,
    indexed: bool,
    rank: i32,
) -> Result<(i32, i32), Failure> {
    if indexed {
        return Ok((array.edge(rank)?, array.edge(rank + 1)?));
    }
    Ok((rank, rank + 1))
}

fn segment_count(array: &Graphic3dArrayOfPrimitives, indexed: bool) -> i32 {
    if indexed {
        return array.edge_number();
    }
    array.vertex_number()
}
