//! The reads of references to entities by the parameter reader.
use super::param_reader::{bound_entity, iges_atoi};
use super::{
    IgesDataIgesEntity, IgesDataIgesReaderData, IgesDataParamCursor, IgesDataParamReader,
    IgesDataStatus,
};
use martensite_tkernel::standard::{Failure, Handle, RuntimeType, RuntimeTyped};
use martensite_tkxsbase::interface::InterfaceParamType;

impl IgesDataParamReader {
    /// Reads a reference to an entity, into `val`; the status tells what went wrong and nothing is
    /// reported. A null reference is fine when `canbenul`.
    ///
    /// # Errors
    /// An out of range failure when the parameter list is short.
    pub fn read_entity_status(
        &mut self,
        ir: &Handle<IgesDataIgesReaderData>,
        pc: &IgesDataParamCursor,
        a_status: &mut IgesDataStatus,
        val: &mut Handle<IgesDataIgesEntity>,
        canbenul: bool,
    ) -> Result<bool, Failure> {
        *a_status = IgesDataStatus::EntityError;
        if !self.prepare_read(pc, None, false, 1) {
            return Ok(false);
        }
        let mut nval = 0;
        if !self.reading_entity_number(self.theindex, &mut nval)? {
            return Ok(false);
        }
        if nval == 0 {
            val.nullify();
            *a_status = if canbenul {
                IgesDataStatus::EntityOk
            } else {
                self.thelast = true;
                IgesDataStatus::ReferenceError
            };
            return Ok(canbenul);
        }
        *val = bound_entity(ir, nval);
        let Some(entity) = val.get() else {
            return Ok(canbenul);
        };
        if is_null_iges(entity, ir, nval) {
            val.nullify();
            *a_status = if canbenul {
                IgesDataStatus::EntityOk
            } else {
                self.thelast = true;
                IgesDataStatus::EntityError
            };
            return Ok(canbenul);
        }
        *a_status = IgesDataStatus::EntityOk;
        Ok(true)
    }
    /// Reads a reference to an entity, into `val`; what is wrong is reported under `mess`. A null
    /// reference is fine when `canbenul`.
    ///
    /// # Errors
    /// An out of range failure when the parameter list is short.
    pub fn read_entity(
        &mut self,
        ir: &Handle<IgesDataIgesReaderData>,
        pc: &IgesDataParamCursor,
        mess: &str,
        val: &mut Handle<IgesDataIgesEntity>,
        canbenul: bool,
    ) -> Result<bool, Failure> {
        if !self.prepare_read(pc, Some(mess), false, 1) {
            return Ok(false);
        }
        let mut nval = 0;
        if !self.reading_entity_number_text(self.theindex, mess, &mut nval)? {
            return Ok(false);
        }
        if nval == 0 {
            val.nullify();
            if !canbenul {
                self.labeled(true, mess, " : Null Reference", "");
                self.thelast = true;
            }
            return Ok(canbenul);
        }
        *val = bound_entity(ir, nval);
        let Some(entity) = val.get() else {
            return Ok(canbenul);
        };
        if is_null_iges(entity, ir, nval) {
            val.nullify();
            if !canbenul {
                self.labeled(true, mess, " : IGES Null Entity", "");
                self.thelast = true;
            }
            return Ok(canbenul);
        }
        Ok(true)
    }
    /// [`Self::read_entity_status`], for an entity of a required type; `val` is a handle of any
    /// class or kind and is null when the entity is of another type.
    ///
    /// # Errors
    /// An out of range failure when the parameter list is short.
    pub fn read_entity_typed_status<T: ?Sized + RuntimeTyped>(
        &mut self,
        ir: &Handle<IgesDataIgesReaderData>,
        pc: &IgesDataParamCursor,
        a_status: &mut IgesDataStatus,
        atype: &RuntimeType,
        val: &mut Handle<T>,
        canbenul: bool,
    ) -> Result<bool, Failure> {
        let mut read = val.cast::<IgesDataIgesEntity>();
        let res = self.read_entity_status(ir, pc, a_status, &mut read, canbenul)?;
        if res && read.is_some() && !read.is_kind(atype) {
            *a_status = IgesDataStatus::TypeError;
            self.thelast = true;
            val.nullify();
            return Ok(false);
        }
        *val = read.cast();
        Ok(res && (canbenul || val.is_some()))
    }
    /// [`Self::read_entity`], for an entity of a required type; `val` is a handle of any class or
    /// kind and is null when the entity is of another type.
    ///
    /// # Errors
    /// An out of range failure when the parameter list is short.
    pub fn read_entity_typed<T: ?Sized + RuntimeTyped>(
        &mut self,
        ir: &Handle<IgesDataIgesReaderData>,
        pc: &IgesDataParamCursor,
        mess: &str,
        atype: &RuntimeType,
        val: &mut Handle<T>,
        canbenul: bool,
    ) -> Result<bool, Failure> {
        let mut read = val.cast::<IgesDataIgesEntity>();
        let res = self.read_entity(ir, pc, mess, &mut read, canbenul)?;
        if res && read.is_some() && !read.is_kind(atype) {
            self.labeled(true, mess, " : Incorrect Type", "");
            self.thelast = true;
            val.nullify();
            return Ok(false);
        }
        *val = read.cast();
        Ok(res && (canbenul || val.is_some()))
    }
    /// Reads the entity number at a parameter number, 0 for a null reference; nothing is reported.
    ///
    /// # Errors
    /// An out of range failure for a bad number.
    pub fn reading_entity_number(&mut self, num: i32, val: &mut i32) -> Result<bool, Failure> {
        let parameter = self.param_at(num)?;
        *val = parameter.entity_number();
        if *val != 0 {
            return Ok(true);
        }
        let nulref = match parameter.param_type() {
            InterfaceParamType::ParamInteger => iges_atoi(parameter.c_value()) == 0,
            InterfaceParamType::ParamVoid => true,
            _ => false,
        };
        if !nulref {
            self.thelast = true;
        }
        Ok(nulref)
    }
    /// Reads the entity number at a parameter number, 0 for a null reference; what is wrong is
    /// reported under `mess`.
    ///
    /// # Errors
    /// An out of range failure for a bad number.
    pub fn reading_entity_number_text(
        &mut self,
        num: i32,
        mess: &str,
        val: &mut i32,
    ) -> Result<bool, Failure> {
        let read = self.reading_entity_number(num, val)?;
        if !read {
            self.labeled(true, mess, " : cannot refer to an Entity", "");
            self.thelast = true;
        }
        Ok(read)
    }
}

/// Whether a read entity is the null entity of the file: it is not filled yet and its directory
/// entry gives the type 0.
pub(super) fn is_null_iges(
    entity: &IgesDataIgesEntity,
    ir: &Handle<IgesDataIgesReaderData>,
    nval: i32,
) -> bool {
    entity.type_number() == 0
        && ir
            .get()
            .is_some_and(|data| data.dir_type(nval).type_number() == 0)
}
