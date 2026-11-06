//! The readers of the two entities that keep their coordinates inline: they read at most three
//! values straight into a local array, so a longer list is a warning and not an array.
use super::*;
use crate::step_data::StepDataStepReaderData;
use crate::step_entity::*;
use crate::step_geom::{StepGeomCartesianPoint, StepGeomDirection};
use crate::step_rw::*;

/// Reads parameter `nump` as at most three reals: the count, capped at 3, and the values.
fn read_inline_coordinates(
    data: &Handle<StepDataStepReaderData>,
    num: i32,
    nump: i32,
    mess: &str,
    ignored_warning: &str,
    ach: &Handle<InterfaceCheck>,
) -> Result<(i32, [f64; 3]), Failure> {
    let mut values = [0.0; 3];
    let mut nsub = 0;
    if !data.read_sub_list(num, nump, mess, ach, &mut nsub, false, 0, 0)? {
        return Ok((0, values));
    }
    let count = data.nb_params(nsub)?;
    if count > 3 {
        ach.add_warning_text(ignored_warning, "");
    }
    let nbcoord = count.min(3);
    for (rank, slot) in (1..=nbcoord).zip(values.iter_mut()) {
        let mut item = 0.0;
        if data.read_real(nsub, rank, mess, ach, &mut item)? {
            *slot = item;
        }
    }
    Ok((nbcoord, values))
}

impl RwStepGeomRwCartesianPoint {
    /// `read_step`: a name that cannot be read is mended to an empty string.
    ///
    /// # Errors
    /// an out of range failure for a parameter outside the record.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomCartesianPoint>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "cartesian_point")? {
            return Ok(());
        }
        let mut a_name = Handle::<HAsciiString>::default();
        if !data.read_string(num, 1, "name", ach, &mut a_name)? {
            ach.mend(Some("Set to empty string"), 0)?;
            a_name = Handle::new(HAsciiString::new(""));
        }
        let warning = "More than 3 coordinates, ignored";
        let (nbcoord, [x, y, z]) =
            read_inline_coordinates(data, num, 2, "coordinates", warning, ach)?;
        if nbcoord == 3 {
            ent.init_3d(&a_name, x, y, z);
        } else {
            ent.init_2d(&a_name, x, y);
        }
        Ok(())
    }
}

impl RwStepGeomRwDirection {
    /// `read_step`.
    ///
    /// # Errors
    /// an out of range failure for a parameter outside the record.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomDirection>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "direction")? {
            return Ok(());
        }
        let mut a_name = Handle::<HAsciiString>::default();
        data.read_string(num, 1, "name", ach, &mut a_name)?;
        let warning = "More than 3 direction ratios, ignored";
        let (nbcoord, [x, y, z]) =
            read_inline_coordinates(data, num, 2, "direction_ratios", warning, ach)?;
        if nbcoord == 3 {
            ent.init_3d(&a_name, x, y, z);
        } else {
            ent.init_2d(&a_name, x, y);
        }
        Ok(())
    }
}
