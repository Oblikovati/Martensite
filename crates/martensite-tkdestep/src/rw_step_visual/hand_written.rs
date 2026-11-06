//! The RW members whose values are not entity arrays: coordinates in `Xyz`, object handles
//! on plain arrays, and typed parameters compared with their type names.
use super::*;
use crate::step_data::{StepDataStepReaderData, StepDataStepWriter};
use crate::step_entity::*;
use crate::step_geom::StepGeomAxis2Placement3d;
use crate::step_rw::*;
use crate::step_visual::*;
use martensite_tkernel::standard::exceptions;

/// The tessellated items of a set: an owned shared array, null when not read.
type TessellatedItems = ObjectHandle<Array1<'static, Handle<StepVisualTessellatedItem>>>;

/// The array of a object handle, or the failure of its null dereference.
fn live_items<'a>(
    items: &'a TessellatedItems,
    where_called: &str,
) -> Result<&'a Array1<'static, Handle<StepVisualTessellatedItem>>, Failure> {
    items
        .get()
        .ok_or_else(|| exceptions::null_object(&format!("{where_called}: null array")))
}

/// Reads the `items` list of a tessellated set; null when the parameter is no list.
fn read_tessellated_items(
    data: &StepDataStepReaderData,
    num: i32,
    nump: i32,
    ach: &Handle<InterfaceCheck>,
) -> Result<TessellatedItems, Failure> {
    let mut nsub = 0;
    if !data.read_sub_list(num, nump, "items", ach, &mut nsub, false, 0, 0)? {
        return Ok(ObjectHandle::default());
    }
    let count = data.nb_params(nsub)?;
    let mut items = Array1::<'static, Handle<StepVisualTessellatedItem>>::with_bounds(1, count)
        .map_err(|error| error.into_failure("tessellated items"))?;
    for rank in 1..=count {
        let mut item = Handle::<StepVisualTessellatedItem>::default();
        if data.read_entity(
            nsub,
            rank,
            "item",
            ach,
            StepVisualTessellatedItem::static_type(),
            &mut item,
        )? {
            items
                .set_value(rank, item)
                .map_err(|error| error.into_failure("tessellated items"))?;
        }
    }
    Ok(ObjectHandle::new(items))
}

/// Sends every item of a tessellated set.
fn send_tessellated_items(
    sw: &mut StepDataStepWriter,
    items: &TessellatedItems,
    where_called: &str,
) -> Result<(), Failure> {
    sw.open_sub()?;
    let items = live_items(items, where_called)?;
    for rank in items.lower()..=items.upper() {
        let item = items
            .value(rank)
            .map_err(|error| error.into_failure(where_called))?;
        sw.send_entity(item)?;
    }
    sw.close_sub()
}

/// Adds every item of a tessellated set to the sharing iterator.
fn share_tessellated_items(
    iter: &mut InterfaceEntityIterator,
    items: &TessellatedItems,
    where_called: &str,
) -> Result<(), Failure> {
    let items = live_items(items, where_called)?;
    for rank in items.lower()..=items.upper() {
        let item = items
            .value(rank)
            .map_err(|error| error.into_failure(where_called))?;
        iter.add_item(&item.cast());
    }
    Ok(())
}

impl RwStepVisualRwCoordinatesList {
    /// `read_step`. An empty list leaves the entity unset, and a coordinate that
    /// cannot be read stays 0.
    ///
    /// # Errors
    /// an out of range failure for a record outside the file.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepVisualCoordinatesList>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "coordinate list")? {
            return Ok(());
        }
        let mut a_name = Handle::<HAsciiString>::default();
        data.read_string(num, 1, "name", ach, &mut a_name)?;
        let mut nb_p = 0;
        data.read_integer(num, 2, "number_points", ach, &mut nb_p)?;
        let mut a_points = Handle::<HArray1<Xyz>>::default();
        let mut nsub2 = 0;
        if data.read_sub_list(num, 3, "items", ach, &mut nsub2, false, 0, 0)? {
            let nb2 = data.nb_params(nsub2)?;
            if nb2 == 0 {
                return Ok(());
            }
            a_points = step_new_array1::<Xyz>(1, nb2)?;
            for rank in 1..=nb2 {
                let point = read_point(data, nsub2, rank, ach)?;
                step_array1_set(
                    &a_points,
                    rank,
                    point,
                    "RwStepVisualRwCoordinatesList::ReadStep",
                )?;
            }
        }
        step_live(ent, "RwStepVisualRwCoordinatesList::ReadStep")?.init(&a_name, &a_points);
        Ok(())
    }
    /// `WriteStep`.
    ///
    /// # Errors
    /// a null object failure for a null entity or point array.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepVisualCoordinatesList>,
    ) -> Result<(), Failure> {
        const WHERE: &str = "RwStepVisualRwCoordinatesList::WriteStep";
        let entity = step_live(ent, WHERE)?;
        sw.send_entity(&entity.name())?;
        let points = entity.points();
        let live = step_live(&points, WHERE)?;
        let count = live.array1().length();
        sw.send_integer(count)?;
        sw.open_sub()?;
        for rank in 1..=count {
            sw.open_sub()?;
            let point = step_array1_item(&points, rank, WHERE)?;
            sw.send_real(point.x())?;
            sw.send_real(point.y())?;
            sw.send_real(point.z())?;
            sw.close_sub()?;
        }
        sw.close_sub()
    }
}

/// One point of a coordinate list: more than three coordinates are ignored with a warning.
fn read_point(
    data: &StepDataStepReaderData,
    nsub: i32,
    rank: i32,
    ach: &Handle<InterfaceCheck>,
) -> Result<Xyz, Failure> {
    let mut point = Xyz::new(0.0, 0.0, 0.0);
    let mut nsub3 = 0;
    if !data.read_sub_list(nsub, rank, "coordinates", ach, &mut nsub3, false, 0, 0)? {
        return Ok(point);
    }
    let nb3 = data.nb_params(nsub3)?;
    if nb3 > 3 {
        ach.add_warning_text("More than 3 coordinates, ignored", "");
    }
    for coordinate in 1..=nb3.min(3) {
        let mut value = 0.0;
        if data.read_real(nsub3, coordinate, "coordinates", ach, &mut value)? {
            point.set_coord(coordinate, value)?;
        }
    }
    Ok(point)
}

impl RwStepVisualRwTessellatedGeometricSet {
    /// `read_step`.
    ///
    /// # Errors
    /// an out of range failure for a record outside the file.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepVisualTessellatedGeometricSet>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "tessellated_geometric_set")? {
            return Ok(());
        }
        let mut a_name = Handle::<HAsciiString>::default();
        data.read_string(num, 1, "name", ach, &mut a_name)?;
        let items = read_tessellated_items(data, num, 2, ach)?;
        step_live(ent, "RwStepVisualRwTessellatedGeometricSet::ReadStep")?.init(&a_name, &items);
        Ok(())
    }
    /// `WriteStep`.
    ///
    /// # Errors
    /// a null object failure for a null entity or item array.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepVisualTessellatedGeometricSet>,
    ) -> Result<(), Failure> {
        const WHERE: &str = "RwStepVisualRwTessellatedGeometricSet::WriteStep";
        let entity = step_live(ent, WHERE)?;
        sw.send_entity(&entity.name())?;
        send_tessellated_items(sw, &entity.items(), WHERE)
    }
    /// `share`.
    ///
    /// # Errors
    /// a null object failure for a null entity or item array.
    pub fn share(
        &self,
        ent: &Handle<StepVisualTessellatedGeometricSet>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        const WHERE: &str = "RwStepVisualRwTessellatedGeometricSet::Share";
        share_tessellated_items(iter, &step_live(ent, WHERE)?.items(), WHERE)
    }
}

impl RwStepVisualRwRepositionedTessellatedGeometricSet {
    /// `read_step`: a complex record of the three supertypes.
    ///
    /// # Errors
    /// an out of range failure for a record outside the file.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepVisualRepositionedTessellatedGeometricSet>,
    ) -> Result<(), Failure> {
        let mut record = 0;
        data.named_for_complex("REPOSITIONED_TESSELLATED_ITEM", num, &mut record, ach);
        if !data.check_nb_params(record, 1, ach, "location")? {
            return Ok(());
        }
        let mut a_location = Handle::<StepGeomAxis2Placement3d>::default();
        data.read_entity(
            record,
            1,
            "location",
            ach,
            StepGeomAxis2Placement3d::static_type(),
            &mut a_location,
        )?;
        data.named_for_complex("REPRESENTATION_ITEM", num, &mut record, ach);
        if !data.check_nb_params(record, 1, ach, "name")? {
            return Ok(());
        }
        let mut a_name = Handle::<HAsciiString>::default();
        data.read_string(record, 1, "name", ach, &mut a_name)?;
        data.named_for_complex("TESSELLATED_GEOMETRIC_SET", num, &mut record, ach);
        let items = read_tessellated_items(data, record, 1, ach)?;
        step_live(
            ent,
            "RwStepVisualRwRepositionedTessellatedGeometricSet::ReadStep",
        )?
        .init(&a_name, &items, &a_location);
        Ok(())
    }
    /// `WriteStep`: the complex record with its supertypes in alphabetical order.
    ///
    /// # Errors
    /// a null object failure for a null entity or item array.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepVisualRepositionedTessellatedGeometricSet>,
    ) -> Result<(), Failure> {
        const WHERE: &str = "RwStepVisualRwRepositionedTessellatedGeometricSet::WriteStep";
        let entity = step_live(ent, WHERE)?;
        step_start_entity(sw, "GEOMETRIC_REPRESENTATION_ITEM")?;
        step_start_entity(sw, "REPOSITIONED_TESSELLATED_ITEM")?;
        sw.send_entity(&entity.location())?;
        step_start_entity(sw, "REPRESENTATION_ITEM")?;
        sw.send_entity(&entity.name())?;
        step_start_entity(sw, "TESSELLATED_GEOMETRIC_SET")?;
        send_tessellated_items(sw, &entity.items(), WHERE)?;
        step_start_entity(sw, "TESSELLATED_ITEM")
    }
    /// `share`.
    ///
    /// # Errors
    /// a null object failure for a null entity or item array.
    pub fn share(
        &self,
        ent: &Handle<StepVisualRepositionedTessellatedGeometricSet>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        const WHERE: &str = "RwStepVisualRwRepositionedTessellatedGeometricSet::Share";
        let entity = step_live(ent, WHERE)?;
        share_tessellated_items(iter, &entity.items(), WHERE)?;
        iter.add_item(&entity.location().cast());
        Ok(())
    }
}

impl RwStepVisualRwTessellatedCurveSet {
    /// `read_step`. An empty list leaves the entity unset, and a curve that is no
    /// list is dropped from the array.
    ///
    /// # Errors
    /// an out of range failure for a record outside the file.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepVisualTessellatedCurveSet>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "tessellated_curve_set")? {
            return Ok(());
        }
        let mut a_name = Handle::<HAsciiString>::default();
        data.read_string(num, 1, "name", ach, &mut a_name)?;
        let mut a_coord_list = Handle::<StepVisualCoordinatesList>::default();
        data.read_entity(
            num,
            2,
            "coord_list",
            ach,
            StepVisualCoordinatesList::static_type(),
            &mut a_coord_list,
        )?;
        let mut curves = DynamicArray::<Handle<HSequence<i32>>>::default();
        let mut nsub2 = 0;
        if data.read_sub_list(num, 3, "curves", ach, &mut nsub2, false, 0, 0)? {
            let nb2 = data.nb_params(nsub2)?;
            if nb2 == 0 {
                return Ok(());
            }
            for rank in 1..=nb2 {
                if let Some(curve) = read_curve(data, nsub2, rank, ach)? {
                    curves.append(curve);
                }
            }
        }
        step_live(ent, "RwStepVisualRwTessellatedCurveSet::ReadStep")?.init(
            &a_name,
            &a_coord_list,
            &ObjectHandle::new(curves),
        );
        Ok(())
    }
    /// `WriteStep`.
    ///
    /// # Errors
    /// a null object failure for a null entity or curve array.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepVisualTessellatedCurveSet>,
    ) -> Result<(), Failure> {
        const WHERE: &str = "RwStepVisualRwTessellatedCurveSet::WriteStep";
        let entity = step_live(ent, WHERE)?;
        sw.send_entity(&entity.name())?;
        sw.send_entity(&entity.coord_list())?;
        sw.open_sub()?;
        let curves = entity.curves();
        let curves = curves
            .get()
            .ok_or_else(|| exceptions::null_object(&format!("{WHERE}: null array")))?;
        for curve in curves.iter() {
            sw.open_sub()?;
            let live = step_live(curve, WHERE)?;
            for point in live.sequence(|points| points.iter().copied().collect::<Vec<i32>>()) {
                sw.send_integer(point)?;
            }
            sw.close_sub()?;
        }
        sw.close_sub()
    }
}

/// One curve of a curve set, `None` when its parameter is no list.
fn read_curve(
    data: &StepDataStepReaderData,
    nsub: i32,
    rank: i32,
    ach: &Handle<InterfaceCheck>,
) -> Result<Option<Handle<HSequence<i32>>>, Failure> {
    let mut nsub3 = 0;
    if !data.read_sub_list(
        nsub,
        rank,
        "number_coordinates",
        ach,
        &mut nsub3,
        false,
        0,
        0,
    )? {
        return Ok(None);
    }
    let curve = HSequence::<i32>::default();
    for coordinate in 1..=data.nb_params(nsub3)? {
        let mut value = 0;
        if data.read_integer(nsub3, coordinate, "coordinates", ach, &mut value)? {
            curve.append(value);
        }
    }
    Ok(Some(Handle::new(curve)))
}

impl RwStepVisualRwSurfaceStyleParameterLine {
    /// `read_step`. The select is reused, so a later item keeps the counts an
    /// earlier one set.
    ///
    /// # Errors
    /// an out of range failure for a record outside the file.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepVisualSurfaceStyleParameterLine>,
    ) -> Result<(), Failure> {
        const WHERE: &str = "RwStepVisualRwSurfaceStyleParameterLine::ReadStep";
        if !data.check_nb_params(num, 2, ach, "surface_style_parameter_line")? {
            return Ok(());
        }
        let mut a_style = Handle::<StepVisualCurveStyle>::default();
        data.read_entity(
            num,
            1,
            "style_of_parameter_lines",
            ach,
            StepVisualCurveStyle::static_type(),
            &mut a_style,
        )?;
        let mut a_counts = Handle::<HArray1<StepVisualDirectionCountSelect>>::default();
        let mut nsub2 = 0;
        if data.read_sub_list(num, 2, "direction_counts", ach, &mut nsub2, false, 0, 0)? {
            let nb2 = data.nb_params(nsub2)?;
            a_counts = step_new_array1::<StepVisualDirectionCountSelect>(1, nb2)?;
            let mut select = StepVisualDirectionCountSelect::new();
            let mut item = 0;
            for rank in 1..=nb2 {
                let (mut numr, mut numpr, mut true_type) = (0, 0, AsciiString::default());
                if !data.read_typed_param(
                    nsub2,
                    rank,
                    true,
                    "direction_count",
                    ach,
                    &mut numr,
                    &mut numpr,
                    &mut true_type,
                )? {
                    ach.add_fail_text("Parameter #2 (direction_counts) item is not TYPED", "");
                    continue;
                }
                if !data.read_integer(numr, numpr, "direction_counts", ach, &mut item)? {
                    ach.add_fail_text("Parameter #2 (direction_counts) item is not an INTEGER", "");
                    continue;
                }
                match true_type.as_bytes() {
                    b"U_DIRECTION_COUNT" => select.set_u_direction_count(item),
                    b"V_DIRECTION_COUNT" => select.set_v_direction_count(item),
                    _ => {
                        ach.add_fail_text(
                            "Parameter #2 (direction_counts) item has illegal TYPE",
                            "",
                        );
                        continue;
                    }
                }
                step_array1_set(&a_counts, rank, select, WHERE)?;
            }
        }
        step_live(ent, WHERE)?.init(&a_style, &a_counts);
        Ok(())
    }
}

impl RwStepVisualRwTextStyleWithBoxCharacteristics {
    /// `read_step`. An unknown type name still stores the value, under the content
    /// number of the previous item.
    ///
    /// # Errors
    /// an out of range failure for a record outside the file.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepVisualTextStyleWithBoxCharacteristics>,
    ) -> Result<(), Failure> {
        const WHERE: &str = "RwStepVisualRwTextStyleWithBoxCharacteristics::ReadStep";
        if !data.check_nb_params(
            num,
            3,
            ach,
            "text_style_with_box_characteristics has not 3 parameter(s)",
        )? {
            return Ok(());
        }
        let mut a_name = Handle::<HAsciiString>::default();
        data.read_string(num, 1, "name", ach, &mut a_name)?;
        let mut an_appearance = Handle::<StepVisualTextStyleForDefinedFont>::default();
        data.read_entity(
            num,
            2,
            "character_appearance",
            ach,
            StepVisualTextStyleForDefinedFont::static_type(),
            &mut an_appearance,
        )?;
        let mut a_characteristics = Handle::<HArray1<StepVisualBoxCharacteristicSelect>>::default();
        let nsub3 = data.sub_list_number(num, 3, false)?;
        if nsub3 == 0 {
            ach.add_fail_text("Parameter #3 (characteristics) is not a LIST", "");
        } else {
            let nb3 = data.nb_params(nsub3)?;
            a_characteristics = step_new_array1::<StepVisualBoxCharacteristicSelect>(1, nb3)?;
            let mut select = StepVisualBoxCharacteristicSelect::new();
            let mut item = 0.0;
            for rank in 1..=nb3 {
                let (mut numr, mut numpr, mut true_type) = (0, 0, AsciiString::default());
                if !data.read_typed_param(
                    nsub3,
                    rank,
                    true,
                    "characteristics",
                    ach,
                    &mut numr,
                    &mut numpr,
                    &mut true_type,
                )? {
                    ach.add_fail_text("Parameter #3 (characteristics) item is not TYPED", "");
                    continue;
                }
                if !data.read_real(numr, numpr, "characteristics", ach, &mut item)? {
                    ach.add_fail_text("Parameter #3 (characteristics) item is not a REAL", "");
                    continue;
                }
                select.set_real_value(item);
                match true_type.as_bytes() {
                    b"BOX_HEIGHT" => select.set_type_of_content(1),
                    b"BOX_WIDTH" => select.set_type_of_content(2),
                    b"BOX_SLANT_ANGLE" => select.set_type_of_content(3),
                    b"BOX_ROTATE_ANGLE" => select.set_type_of_content(4),
                    _ => ach
                        .add_fail_text("Parameter #3 (characteristics) item has illegal TYPE", ""),
                }
                step_array1_set(&a_characteristics, rank, select, WHERE)?;
            }
        }
        step_live(ent, WHERE)?.init(&a_name, &an_appearance, &a_characteristics);
        Ok(())
    }
}
