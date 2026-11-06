//! `XCAFDocMaterial` (a named material with its density) and `XCAFDocDimTol` (the legacy
//! dimension and tolerance record).
use super::support::{downcast, find_or_add, live};
use crate::json::{field_h_string, field_raw_pointer};
use crate::xcaf_dim_tol_objects::array_values;
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::dump::DumpStream;
use martensite_tkernel::standard::{Failure, Guid, Handle, SharedState, StandardTransient};
use martensite_tkernel::tcollection::HAsciiString;
use martensite_tklcaf::guid_from_text;
use martensite_tklcaf::json::dump_attribute_json_base;
use martensite_tklcaf::tdf::{
    TdfAttribute, TdfAttributeBase, TdfAttributeState, TdfLabel, TdfRelocationTable,
};
use martensite_tkservice::json_dump::{class_begin, field_integer, field_real};
use std::io;

/// `XCAFDocMaterial::GetID()`.
const MATERIAL_ID: Guid = guid_from_text("efd212f8-6dfd-11d4-b9c8-0060b0ee281b");

/// The fields of `XCAFDocMaterial`.
#[derive(Clone, Default)]
pub struct XCAFDocMaterialFields {
    /// `myName`.
    pub name: Handle<HAsciiString>,
    /// `myDescription`.
    pub description: Handle<HAsciiString>,
    /// `myDensity` (uninitialized natively before `Set`; zero here, as a clearing allocator).
    pub density: f64,
    /// `myDensName`.
    pub dens_name: Handle<HAsciiString>,
    /// `myDensValType`.
    pub dens_val_type: Handle<HAsciiString>,
}

/// `XCAFDocMaterial`.
///
/// ```
/// use martensite_tkernel::{standard::Handle, tcollection::HAsciiString};
/// use martensite_tklcaf::tdf::TdfData;
/// use martensite_tkxcaf::xcaf_doc::{XCAFDocMaterial, XCAFDocMaterialFields};
/// let data = TdfData::new();
/// let name = Handle::new(HAsciiString::new("Steel"));
/// let fields = XCAFDocMaterialFields { name, density: 7.8, ..Default::default() };
/// let material = XCAFDocMaterial::set(&data.root(), &fields)?;
/// assert_eq!(material.get_density(), 7.8);
/// # Ok::<(), martensite_tkernel::standard::Failure>(())
/// ```
#[derive(Default)]
pub struct XCAFDocMaterial {
    transient: StandardTransient,
    base: TdfAttributeBase,
    fields: SharedState<XCAFDocMaterialFields>,
}
martensite_tkernel::impl_transient!(XCAFDocMaterial, "XCAFDocMaterial", dyn TdfAttribute, transient, interfaces: [dyn TdfAttribute]);

impl XCAFDocMaterial {
    /// `XCAFDocMaterial()`.
    pub fn new() -> Self {
        Self::default()
    }
    /// `GetID`.
    pub fn get_id() -> Guid {
        MATERIAL_ID
    }
    /// `Set(label, aName, aDescription, aDensity, aDensName, aDensValType)`.
    pub fn set(
        label: &TdfLabel,
        fields: &XCAFDocMaterialFields,
    ) -> Result<Handle<XCAFDocMaterial>, Failure> {
        let attribute = find_or_add(label, &MATERIAL_ID, Self::new)?;
        live(&attribute, "XCAFDocMaterial::Set")?.set_fields(fields);
        Ok(attribute)
    }
    /// `Set(aName, aDescription, aDensity, aDensName, aDensValType)`: not backed up, as
    /// natively.
    pub fn set_fields(&self, fields: &XCAFDocMaterialFields) {
        *self.fields.write() = fields.clone();
    }
    /// The five values.
    pub fn fields(&self) -> XCAFDocMaterialFields {
        self.fields.read().clone()
    }
    /// `GetName`.
    pub fn get_name(&self) -> Handle<HAsciiString> {
        self.fields.read().name.clone()
    }
    /// `GetDescription`.
    pub fn get_description(&self) -> Handle<HAsciiString> {
        self.fields.read().description.clone()
    }
    /// `GetDensity`.
    pub fn get_density(&self) -> f64 {
        self.fields.read().density
    }
    /// `GetDensName`.
    pub fn get_dens_name(&self) -> Handle<HAsciiString> {
        self.fields.read().dens_name.clone()
    }
    /// `GetDensValType`.
    pub fn get_dens_val_type(&self) -> Handle<HAsciiString> {
        self.fields.read().dens_val_type.clone()
    }
}

impl TdfAttribute for XCAFDocMaterial {
    fn attribute_base(&self) -> &TdfAttributeBase {
        &self.base
    }
    fn id(&self) -> Guid {
        MATERIAL_ID
    }
    fn restore(&self, with: &Handle<dyn TdfAttribute>) -> Result<(), Failure> {
        let other = downcast::<XCAFDocMaterial>(with, "XCAFDocMaterial::Restore")?;
        self.set_fields(&live(&other, "XCAFDocMaterial::Restore")?.fields());
        Ok(())
    }
    fn new_empty(&self) -> Handle<dyn TdfAttribute> {
        Handle::new(Self::new()).cast()
    }
    fn paste(
        &self,
        into: &Handle<dyn TdfAttribute>,
        _table: &Handle<TdfRelocationTable>,
    ) -> Result<(), Failure> {
        let target = downcast::<XCAFDocMaterial>(into, "XCAFDocMaterial::Paste")?;
        live(&target, "XCAFDocMaterial::Paste")?.set_fields(&self.fields());
        Ok(())
    }
    fn dump_json(&self, stream: &mut DumpStream, depth: i32) -> io::Result<()> {
        let fields = self.fields();
        class_begin(stream, "XCAFDocMaterial")?;
        dump_attribute_json_base(self, stream, depth)?;
        field_h_string(stream, "aMaterialName", &fields.name)?;
        field_h_string(stream, "aDescriptionName", &fields.description)?;
        field_real(stream, "myDensity", fields.density)?;
        field_h_string(stream, "aDensName", &fields.dens_name)?;
        field_h_string(stream, "aDensValType", &fields.dens_val_type)
    }
}

/// `XCAFDocDimTol::GetID()`.
const DIM_TOL_ID: Guid = guid_from_text("58ed092d-44de-11d8-8776-001083004c77");

/// The fields of `XCAFDocDimTol`.
#[derive(Clone, Default)]
struct DimTolFields {
    kind: i32,
    values: Handle<HArray1<f64>>,
    name: Handle<HAsciiString>,
    description: Handle<HAsciiString>,
}

/// `XCAFDocDimTol`: a kind (below 20 a dimension, else a tolerance), values, name and
/// description.
#[derive(Default)]
pub struct XCAFDocDimTol {
    transient: StandardTransient,
    base: TdfAttributeBase,
    fields: SharedState<DimTolFields>,
}
martensite_tkernel::impl_transient!(XCAFDocDimTol, "XCAFDocDimTol", dyn TdfAttribute, transient, interfaces: [dyn TdfAttribute]);

impl XCAFDocDimTol {
    /// `XCAFDocDimTol()`.
    pub fn new() -> Self {
        Self::default()
    }
    /// `GetID`.
    pub fn get_id() -> Guid {
        DIM_TOL_ID
    }
    /// `Set(label, kind, aVal, aName, aDescription)`.
    pub fn set(
        label: &TdfLabel,
        kind: i32,
        values: &Handle<HArray1<f64>>,
        name: &Handle<HAsciiString>,
        description: &Handle<HAsciiString>,
    ) -> Result<Handle<XCAFDocDimTol>, Failure> {
        let attribute = find_or_add(label, &DIM_TOL_ID, Self::new)?;
        live(&attribute, "XCAFDocDimTol::Set")?.set_values(kind, values, name, description)?;
        Ok(attribute)
    }
    /// `Set(kind, aVal, aName, aDescription)`: backed up.
    pub fn set_values(
        &self,
        kind: i32,
        values: &Handle<HArray1<f64>>,
        name: &Handle<HAsciiString>,
        description: &Handle<HAsciiString>,
    ) -> Result<(), Failure> {
        self.backup()?;
        *self.fields.write() = DimTolFields {
            kind,
            values: values.clone(),
            name: name.clone(),
            description: description.clone(),
        };
        Ok(())
    }
    /// `GetKind`.
    pub fn get_kind(&self) -> i32 {
        self.fields.read().kind
    }
    /// `GetVal`.
    pub fn get_val(&self) -> Handle<HArray1<f64>> {
        self.fields.read().values.clone()
    }
    /// `GetName`.
    pub fn get_name(&self) -> Handle<HAsciiString> {
        self.fields.read().name.clone()
    }
    /// `GetDescription`.
    pub fn get_description(&self) -> Handle<HAsciiString> {
        self.fields.read().description.clone()
    }
}

impl TdfAttribute for XCAFDocDimTol {
    fn attribute_base(&self) -> &TdfAttributeBase {
        &self.base
    }
    fn id(&self) -> Guid {
        DIM_TOL_ID
    }
    fn restore(&self, with: &Handle<dyn TdfAttribute>) -> Result<(), Failure> {
        let other = downcast::<XCAFDocDimTol>(with, "XCAFDocDimTol::Restore")?;
        let other = live(&other, "XCAFDocDimTol::Restore")?
            .fields
            .read()
            .clone();
        *self.fields.write() = other;
        Ok(())
    }
    fn new_empty(&self) -> Handle<dyn TdfAttribute> {
        Handle::new(Self::new()).cast()
    }
    fn paste(
        &self,
        into: &Handle<dyn TdfAttribute>,
        _table: &Handle<TdfRelocationTable>,
    ) -> Result<(), Failure> {
        let target = downcast::<XCAFDocDimTol>(into, "XCAFDocDimTol::Paste")?;
        let f = self.fields.read().clone();
        live(&target, "XCAFDocDimTol::Paste")?.set_values(
            f.kind,
            &f.values,
            &f.name,
            &f.description,
        )
    }
    /// `DumpJson`. Natively the values of a null array are dereferenced; the implementation dumps none.
    /// The name and the description are written as pointers (`myName.get()`), as natively.
    fn dump_json(&self, stream: &mut DumpStream, depth: i32) -> io::Result<()> {
        let f = self.fields.read().clone();
        class_begin(stream, "XCAFDocDimTol")?;
        dump_attribute_json_base(self, stream, depth)?;
        field_integer(stream, "myKind", f.kind)?;
        for value in array_values(&f.values).unwrap_or_default() {
            field_real(stream, "aValue", value)?;
        }
        field_raw_pointer(stream, "myName.get()", f.name.object_address())?;
        field_raw_pointer(
            stream,
            "myDescription.get()",
            f.description.object_address(),
        )
    }
}
