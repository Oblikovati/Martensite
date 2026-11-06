//! The reads of lists by the parameter reader.
use super::param_reader::{bound_entity, iges_atoi};
use super::param_reader_read::label_of;
use super::{
    IgesDataIgesEntity, IgesDataIgesReaderData, IgesDataParamCursor, IgesDataParamReader,
    IgesDataReadMessage, iges_array1_item, iges_array1_set, iges_msg, iges_new_array1,
};
use martensite_tkernel::{
    message::Msg,
    ncollection::HArray1,
    standard::{Failure, Handle, StandardTransient},
    tcollection::{AsciiString, HAsciiString},
};
use martensite_tkxsbase::interface::{InterfaceEntityList, InterfaceParamType};

impl IgesDataParamReader {
    /// A new array for the items of a list read, from `index`, null for an empty list.
    fn list_array<T: Default + Send + Sync + 'static>(
        &self,
        val: &mut Handle<HArray1<T>>,
        index: i32,
    ) -> Result<bool, Failure> {
        if self.thenbitem == 0 {
            return Ok(false);
        }
        *val = iges_new_array1(index, self.list_upper(index))?;
        Ok(true)
    }
    fn list_upper(&self, index: i32) -> i32 {
        index + self.thenbitem * self.thetermsz - 1
    }
    /// Reads a list of integers, into an array from `index`; void parameters give 0. An empty list
    /// leaves `val` as it is.
    ///
    /// # Errors
    /// An out of range failure when the parameter list is short.
    pub fn read_ints<'a>(
        &mut self,
        pc: &IgesDataParamCursor,
        mess: impl Into<IgesDataReadMessage<'a>>,
        val: &mut Handle<HArray1<i32>>,
        index: i32,
    ) -> Result<bool, Failure> {
        let mess = mess.into();
        if !self.prepare_read(pc, label_of(mess), true, 1) {
            return Ok(false);
        }
        if !self.list_array(val, index)? {
            return Ok(true);
        }
        let mut ind = index;
        let mut i = self.first_read();
        while i > 0 {
            let parameter = self.param_at(i)?;
            let item = match parameter.param_type() {
                InterfaceParamType::ParamInteger => iges_atoi(parameter.c_value()),
                InterfaceParamType::ParamVoid => 0,
                _ => {
                    let what = format!(" : not an Integer, rank {i}");
                    self.report_fail(mess, &what, " : not an Integer, rank %d");
                    return Ok(false);
                }
            };
            iges_array1_set(val, ind, item, "read_ints")?;
            ind += 1;
            i = self.next_read();
        }
        Ok(true)
    }
    /// Reads a list of reals, into an array from `index`; void parameters give 0. An empty list
    /// leaves `val` as it is. A catalog message is not used.
    ///
    /// # Errors
    /// An out of range failure when the parameter list is short.
    pub fn read_reals<'a>(
        &mut self,
        pc: &IgesDataParamCursor,
        mess: impl Into<IgesDataReadMessage<'a>>,
        val: &mut Handle<HArray1<f64>>,
        index: i32,
    ) -> Result<bool, Failure> {
        let label = label_of(mess.into());
        if !self.prepare_read(pc, label, true, 1) {
            return Ok(false);
        }
        if !self.list_array(val, index)? {
            return Ok(true);
        }
        let mut ind = index;
        let mut i = self.first_read();
        while i > 0 {
            let mut rval = 0.0;
            let read = match label {
                Some(text) => self.reading_real_text(i, text, &mut rval)?,
                None => self.reading_real(i, &mut rval)?,
            };
            if !read {
                return Ok(false);
            }
            iges_array1_set(val, ind, rval, "read_reals")?;
            ind += 1;
            i = self.next_read();
        }
        Ok(true)
    }
    /// Reads a list of counted texts, into an array from `index`; void parameters give empty texts.
    /// An empty list leaves `val` as it is.
    ///
    /// # Errors
    /// An out of range failure when the parameter list is short.
    pub fn read_texts<'a>(
        &mut self,
        pc: &IgesDataParamCursor,
        mess: impl Into<IgesDataReadMessage<'a>>,
        val: &mut Handle<HArray1<Handle<HAsciiString>>>,
        index: i32,
    ) -> Result<bool, Failure> {
        let mess = mess.into();
        if !self.prepare_read(pc, label_of(mess), true, 1) {
            return Ok(false);
        }
        if !self.list_array(val, index)? {
            return Ok(true);
        }
        let mut ind = index;
        let mut i = self.first_read();
        while i > 0 {
            let parameter = self.param_at(i)?;
            let item = match parameter.param_type() {
                InterfaceParamType::ParamText => {
                    match self.counted_text(parameter.c_value(), mess) {
                        Some(text) => text,
                        None => return Ok(false),
                    }
                }
                InterfaceParamType::ParamVoid => Handle::new(HAsciiString::new("")),
                _ => {
                    self.report_fail(mess, " : not given as a Text", "");
                    return Ok(false);
                }
            };
            iges_array1_set(val, ind, item, "read_texts")?;
            ind += 1;
            i = self.next_read();
        }
        Ok(true)
    }
    /// Reads a list of references, into an array from `index`; null, negative and wrong references
    /// are left out and the array shrinks, null when none is left. An empty list leaves `val`
    /// as it is.
    ///
    /// # Errors
    /// An out of range failure when the parameter list is short.
    pub fn read_ents<'a>(
        &mut self,
        ir: &Handle<IgesDataIgesReaderData>,
        pc: &IgesDataParamCursor,
        mess: impl Into<IgesDataReadMessage<'a>>,
        val: &mut Handle<HArray1<Handle<IgesDataIgesEntity>>>,
        index: i32,
    ) -> Result<bool, Failure> {
        let mess = mess.into();
        let label = label_of(mess);
        if !self.prepare_read(pc, label, true, 1) {
            return Ok(false);
        }
        if !self.list_array(val, index)? {
            return Ok(true);
        }
        let indmax = self.list_upper(index);
        let (mut ind, mut nbneg, mut nbnul) = (index, 0, 0);
        let mut i = self.first_read();
        while i > 0 {
            let nval = self.list_entity_number(i, label)?;
            nbneg += i32::from(nval < 0);
            if nval > 0 {
                let entity = bound_entity(ir, nval);
                if entity.is_null()
                    || ir
                        .get()
                        .is_some_and(|d| d.dir_type(nval).type_number() == 0)
                {
                    nbnul += 1;
                } else {
                    iges_array1_set(val, ind, entity, "read_ents")?;
                    ind += 1;
                }
            }
            i = self.next_read();
        }
        shrink_entities(val, index, ind, indmax)?;
        self.report_skipped(mess, nbneg, nbnul);
        Ok(true)
    }
    fn list_entity_number(&mut self, num: i32, label: Option<&str>) -> Result<i32, Failure> {
        let mut nval = 0;
        let read = match label {
            Some(text) => self.reading_entity_number_text(num, text, &mut nval)?,
            None => self.reading_entity_number(num, &mut nval)?,
        };
        Ok(if read { nval } else { 0 })
    }
    fn report_skipped(&mut self, mess: IgesDataReadMessage<'_>, nbneg: i32, nbnul: i32) {
        match mess {
            IgesDataReadMessage::Msg(msg) => {
                if nbnul > 0 {
                    self.send_warning(msg);
                }
            }
            IgesDataReadMessage::Text(_) => {
                if nbneg > 0 {
                    self.add_warning(
                        &format!("Skipped Negative Pointer(s), count {nbneg}"),
                        "Skipped Negative Pointer(s), count %d",
                    );
                }
                if nbnul > 0 {
                    self.add_warning(
                        &format!("Skipped Null Type Entity(ies), count {nbnul}"),
                        "Skipped Null Type Entity(ies), count %d",
                    );
                }
            }
        }
    }
    /// Reads a list of references into an entity list, keeping the order when `ord`, else
    /// keeping each once. Null, negative and wrong references are left out with a warning.
    ///
    /// # Errors
    /// An out of range failure when the parameter list is short.
    pub fn read_ent_list<'a>(
        &mut self,
        ir: &Handle<IgesDataIgesReaderData>,
        pc: &IgesDataParamCursor,
        mess: impl Into<IgesDataReadMessage<'a>>,
        val: &mut InterfaceEntityList,
        ord: bool,
    ) -> Result<bool, Failure> {
        let mess = mess.into();
        let label = label_of(mess);
        if !self.prepare_read(pc, label, true, 1) {
            return Ok(false);
        }
        val.clear();
        if self.thenbitem == 0 {
            return Ok(true);
        }
        let mut msg = catalog_of(mess);
        let mut i = self.first_read();
        while i > 0 {
            let mut nval = 0;
            let read = match label {
                Some(text) => self.reading_entity_number_text(i, text, &mut nval)?,
                None => self.reading_entity_number(i, &mut nval)?,
            };
            if !read {
                return Ok(false);
            }
            self.list_item(ir, nval, label, msg.as_mut(), val, ord)?;
            i = self.next_read();
        }
        Ok(true)
    }
    fn list_item(
        &mut self,
        ir: &Handle<IgesDataIgesReaderData>,
        nval: i32,
        label: Option<&str>,
        msg: Option<&mut Msg>,
        val: &mut InterfaceEntityList,
        ord: bool,
    ) -> Result<(), Failure> {
        if nval < 0 {
            self.warn_item(label, msg, "XSTEP_219", " Negative Pointer, skipped")?;
            return Ok(());
        }
        if nval == 0 {
            return Ok(());
        }
        let entity = bound_entity(ir, nval);
        if entity.is_null() {
            return self.warn_item(label, msg, "XSTEP_216", " Null Pointer, skipped");
        }
        if ir
            .get()
            .is_some_and(|d| d.dir_type(nval).type_number() == 0)
        {
            if label.is_some() {
                self.add_warning(" Pointer to IGES Null Entity, skipped", "");
            } else {
                // The text of the catalog entry is itself looked up as a key.
                let key = AsciiString::from(iges_msg("XSTEP_217").value());
                self.send_warning(&Msg::from_key(&key));
            }
            return Ok(());
        }
        let shared: Handle<StandardTransient> = entity.cast();
        if ord {
            val.append(&shared)
        } else {
            val.add(&shared)
        }
    }
    fn warn_item(
        &mut self,
        label: Option<&str>,
        msg: Option<&mut Msg>,
        key: &str,
        text: &str,
    ) -> Result<(), Failure> {
        if label.is_some() {
            self.add_warning(text, "");
            return Ok(());
        }
        if let Some(message) = msg {
            message.arg_extended(iges_msg(key).value())?;
            self.send_warning(message);
        }
        Ok(())
    }
}

/// A copy of the catalog message of a read: its arguments are filled as the read goes.
fn catalog_of(mess: IgesDataReadMessage<'_>) -> Option<Msg> {
    match mess {
        IgesDataReadMessage::Msg(msg) => Some(msg.clone()),
        IgesDataReadMessage::Text(_) => None,
    }
}

/// Drops the gaps of a list of references: null when nothing was read, else shorter.
fn shrink_entities(
    val: &mut Handle<HArray1<Handle<IgesDataIgesEntity>>>,
    index: i32,
    ind: i32,
    indmax: i32,
) -> Result<(), Failure> {
    if ind == indmax + 1 {
        return Ok(());
    }
    if ind == index {
        val.nullify();
        return Ok(());
    }
    let shorter = iges_new_array1(index, ind - 1)?;
    for i in index..ind {
        iges_array1_set(
            &shorter,
            i,
            iges_array1_item(val, i, "read_ents")?,
            "read_ents",
        )?;
    }
    *val = shorter;
    Ok(())
}
