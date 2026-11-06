//! The reading of the directory part of an entity.
use super::param_reader::{iges_atoi, iges_msg};
use super::{
    IgesDataColorEntity, IgesDataDirPart, IgesDataIgesEntity, IgesDataIgesReaderData,
    IgesDataIgesReaderTool, IgesDataLabelDisplayEntity, IgesDataLevelListEntity,
    IgesDataLineFontEntity, IgesDataTransfEntity, IgesDataViewKindEntity, iges_live,
};
use martensite_tkernel::{
    message::Msg,
    standard::{Failure, Handle},
    tcollection::HAsciiString,
};
use martensite_tkxsbase::interface::{InterfaceCheck, InterfaceFileReaderDataParams};

impl IgesDataIgesReaderTool {
    /// Reads the directory part of an entity.
    ///
    /// # Errors
    /// A failure of the check messages or of the entity.
    pub fn read_dir(
        &self,
        ent: &Handle<IgesDataIgesEntity>,
        ir: &Handle<IgesDataIgesReaderData>,
        dp: &IgesDataDirPart,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        let live = iges_live(ent, "IGES reader tool: read dir")?;
        let data = iges_live(ir, "IGES reader tool: read dir")?;
        let fields = dp.values();
        let v = fields.values;
        live.init_type_and_form(v[0], v[16]);
        live.set_res_values(&fields.res1, &fields.res2);
        let bound = |num: i32| data.bound_entity(num).cast::<IgesDataIgesEntity>();
        let structure = if v[2] < 0 {
            bound((1 - v[2]) / 2)
        } else {
            Handle::default()
        };
        let mut last_field = Handle::default();
        self.read_line_font(live, v[3], &bound, &mut last_field, ach)?;
        self.read_level(live, v[4], &bound, &mut last_field, ach)?;
        if v[5] != 0 {
            let field = bound((1 + v[5]) / 2);
            last_field = field.clone();
            let view = field.cast::<dyn IgesDataViewKindEntity>();
            if view.is_null() {
                self.warn_field("XSTEP_31", ach)?;
                live.init_dir_field_entity(6, &field);
            } else {
                live.init_view(&view);
            }
        }
        if v[6] != 0 {
            let field = bound((1 + v[6]) / 2);
            last_field = field.clone();
            let transf = field.cast::<dyn IgesDataTransfEntity>();
            if transf.is_null() {
                self.warn_field("XSTEP_32", ach)?;
                live.init_dir_field_entity(7, &field);
            } else {
                live.init_transf(&transf);
            }
        }
        let (label, field_label) = self.read_label_display(v[7], &last_field, &bound, ach)?;
        live.init_status(v[8], v[9], v[10], v[11]);
        self.read_color(live, v[14], &bound, ach)?;
        live.init_misc(&structure, &label, v[13]);
        live.init_dir_field_entity(8, &field_label);
        let (short_label, subscript) = label_and_subscript(&fields.label, &fields.subscript);
        live.set_label(&short_label, subscript);
        let (maxweight, gradweight) = {
            let state = self.state.read();
            (state.themaxweight, state.thegradweight)
        };
        live.set_line_weight(data.default_line_weight(), maxweight, gradweight);
        Ok(())
    }
    fn warn_field(&self, key: &str, ach: &Handle<InterfaceCheck>) -> Result<(), Failure> {
        let msg = self.located_msg(key)?;
        if let Some(check) = ach.get() {
            check.send_warning(&msg);
        }
        Ok(())
    }
    /// A message whose arguments are the number and the type of the current record.
    pub(super) fn located_msg(&self, key: &str) -> Result<Msg, Failure> {
        let (cnum, ctyp) = {
            let state = self.state.read();
            (state.thecnum, state.thectyp)
        };
        let mut msg = iges_msg(key);
        msg.arg_integer(cnum)?;
        msg.arg_integer(ctyp.type_number())?;
        Ok(msg)
    }
    fn read_line_font(
        &self,
        live: &IgesDataIgesEntity,
        value: i32,
        bound: &dyn Fn(i32) -> Handle<IgesDataIgesEntity>,
        last_field: &mut Handle<IgesDataIgesEntity>,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        if value >= 0 {
            live.init_line_font(&Handle::default(), value);
            return Ok(());
        }
        let field = bound((1 - value) / 2);
        *last_field = field.clone();
        let font = field.cast::<dyn IgesDataLineFontEntity>();
        if font.is_null() {
            self.warn_field("XSTEP_29", ach)?;
            live.init_dir_field_entity(4, &field);
        } else {
            live.init_line_font(&font, 0);
        }
        Ok(())
    }
    fn read_level(
        &self,
        live: &IgesDataIgesEntity,
        value: i32,
        bound: &dyn Fn(i32) -> Handle<IgesDataIgesEntity>,
        last_field: &mut Handle<IgesDataIgesEntity>,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        if value >= 0 {
            live.init_level(&Handle::default(), value);
            return Ok(());
        }
        let field = bound((1 - value) / 2);
        *last_field = field.clone();
        let levels = field.cast::<dyn IgesDataLevelListEntity>();
        if levels.is_null() {
            self.warn_field("XSTEP_30", ach)?;
            live.init_dir_field_entity(5, &field);
        } else {
            live.init_level(&levels, -1);
        }
        Ok(())
    }
    /// The label display and the entity of its field. The kind is tested on the field read just
    /// before, not on this one, so a label display entity gives a warning; the entity of the field
    /// is what is kept at the end.
    fn read_label_display(
        &self,
        value: i32,
        last_field: &Handle<IgesDataIgesEntity>,
        bound: &dyn Fn(i32) -> Handle<IgesDataIgesEntity>,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<
        (
            Handle<dyn IgesDataLabelDisplayEntity>,
            Handle<IgesDataIgesEntity>,
        ),
        Failure,
    > {
        if value == 0 {
            return Ok((Handle::default(), Handle::default()));
        }
        let field = bound((1 + value) / 2);
        let label = last_field.cast::<dyn IgesDataLabelDisplayEntity>();
        if label.is_null() {
            self.warn_field("XSTEP_33", ach)?;
        }
        Ok((label, field))
    }
    fn read_color(
        &self,
        live: &IgesDataIgesEntity,
        value: i32,
        bound: &dyn Fn(i32) -> Handle<IgesDataIgesEntity>,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        if value >= 0 {
            live.init_color(&Handle::default(), value);
            return Ok(());
        }
        let color = bound((1 - value) / 2).cast::<dyn IgesDataColorEntity>();
        if color.is_null() {
            self.warn_field("XSTEP_34", ach)?;
            live.init_dir_field_entity(13, &Handle::default());
        } else {
            live.init_color(&color, 0);
        }
        Ok(())
    }
}

/// The short label and the subscript of a directory entry; -1 when there is no subscript.
fn label_and_subscript(label: &[u8; 9], subscript: &[u8; 9]) -> (Handle<HAsciiString>, i32) {
    let end = label.iter().position(|byte| *byte == 0).unwrap_or(9);
    let short_label = if label[..8].iter().any(|byte| *byte > b' ') {
        Handle::new(HAsciiString::new(&label[..end]))
    } else {
        Handle::default()
    };
    let digits = subscript
        .iter()
        .take(8)
        .take_while(|byte| **byte != 0)
        .any(|byte| *byte > b' ');
    let number = if digits { iges_atoi(subscript) } else { -1 };
    (short_label, number)
}
