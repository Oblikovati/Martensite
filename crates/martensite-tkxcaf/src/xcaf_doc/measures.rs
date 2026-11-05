//! The validation properties of a shape: `XCAFDocArea` and `XCAFDocVolume` (reals derived
//! from `TDataStdReal`) and `XCAFDocCentroid` (a point).
use super::support::{downcast, find, find_or_add, live};
use crate::json::field_point;
use martensite_tkernel::standard::dump::DumpStream;
use martensite_tkernel::standard::{Failure, Guid, Handle, SharedState, StandardTransient};
use martensite_tkernel::tcollection::AsciiString;
use martensite_tklcaf::guid_from_text;
use martensite_tklcaf::json::dump_attribute_json_base;
use martensite_tklcaf::tdata_std::TDataStdReal;
use martensite_tklcaf::tdf::{
    TdfAttribute, TdfAttributeBase, TdfAttributeState, TdfLabel, TdfRelocationTable,
};
use martensite_tkmath::gp::Point3;
use martensite_tkservice::json_dump::{class_begin, field_real};
use std::io;

/// Declares a real property derived from `TDataStdReal` (`XCAFDocArea`, `XCAFDocVolume`):
/// the embedded real is its RTTI parent, so its handle casts to `TDataStdReal`.
macro_rules! xcaf_real_property {
    ($(#[$doc:meta])* $name:ident, $class_name:literal, $id:literal, $word:literal) => {
        $(#[$doc])*
        pub struct $name {
            real: TDataStdReal,
        }
        martensite_tkernel::impl_transient!($name, $class_name, TDataStdReal, real, interfaces: [dyn TdfAttribute]);

        impl Default for $name {
            fn default() -> Self {
                super::registration::register_derived_attributes();
                Self { real: TDataStdReal::new() }
            }
        }

        impl $name {
            #[doc = concat!("`", $class_name, "()`.")]
            pub fn new() -> Self {
                Self::default()
            }
            /// `GetID`.
            pub fn get_id() -> Guid {
                const ID: Guid = guid_from_text($id);
                ID
            }
            /// `Set(L, V)`: finds or adds the attribute, then sets the value.
            pub fn set(label: &TdfLabel, value: f64) -> Result<Handle<$name>, Failure> {
                let attribute = find_or_add(label, &Self::get_id(), Self::new)?;
                live(&attribute, concat!($class_name, "::Set"))?.set_value(value)?;
                Ok(attribute)
            }
            /// `Set(V)`: `TDataStdReal::Set` (backed up when it changes).
            pub fn set_value(&self, value: f64) -> Result<(), Failure> {
                self.real.set_value(value)
            }
            /// `Get()`.
            pub fn get(&self) -> f64 {
                self.real.get()
            }
            /// `Get(label, value)`: the value of the label's attribute, false without it.
            pub fn get_of_label(label: &TdfLabel, value: &mut f64) -> Result<bool, Failure> {
                let found = find::<$name>(label, &Self::get_id())?;
                let Some(found) = found.get() else {
                    return Ok(false);
                };
                *value = found.get();
                Ok(true)
            }
            /// The embedded `TDataStdReal` (the native base class).
            pub fn real(&self) -> &TDataStdReal {
                &self.real
            }
        }

        impl TdfAttribute for $name {
            fn attribute_base(&self) -> &TdfAttributeBase {
                self.real.attribute_base()
            }
            fn id(&self) -> Guid {
                Self::get_id()
            }
            fn restore(&self, with: &Handle<dyn TdfAttribute>) -> Result<(), Failure> {
                self.real.restore(with)
            }
            /// `NewEmpty` (from `IMPLEMENT_DERIVED_ATTRIBUTE`).
            fn new_empty(&self) -> Handle<dyn TdfAttribute> {
                Handle::new(Self::new()).cast()
            }
            fn paste(
                &self,
                into: &Handle<dyn TdfAttribute>,
                table: &Handle<TdfRelocationTable>,
            ) -> Result<(), Failure> {
                self.real.paste(into, table)
            }
            #[doc = concat!("`Dump`: `\"", $word, " <value>\"`.")]
            fn dump(&self, stream: &mut dyn io::Write) -> io::Result<()> {
                write!(stream, concat!($word, " "))?;
                stream.write_all(AsciiString::from_real(self.get()).as_bytes())
            }
            fn dump_json(&self, stream: &mut DumpStream, depth: i32) -> io::Result<()> {
                class_begin(stream, $class_name)?;
                dump_attribute_json_base(self, stream, depth)?;
                field_real(stream, "myValue", self.get())
            }
        }
    };
}

xcaf_real_property!(
    /// `XCAFDocArea`: the area of a shape.
    ///
    /// ```
    /// use martensite_tklcaf::tdf::TdfData;
    /// use martensite_tkxcaf::xcaf_doc::XCAFDocArea;
    /// let data = TdfData::new();
    /// XCAFDocArea::set(&data.root(), 2.5)?;
    /// let mut area = 0.0;
    /// assert!(XCAFDocArea::get_of_label(&data.root(), &mut area)? && area == 2.5);
    /// # Ok::<(), martensite_tkernel::standard::Failure>(())
    /// ```
    XCAFDocArea, "XCAFDocArea", "efd212f2-6dfd-11d4-b9c8-0060b0ee281b", "Area"
);
xcaf_real_property!(
    /// `XCAFDocVolume`: the volume of a shape.
    XCAFDocVolume, "XCAFDocVolume", "efd212f1-6dfd-11d4-b9c8-0060b0ee281b", "Volume"
);

/// `XCAFDocCentroid::GetID()`.
const CENTROID_ID: Guid = guid_from_text("efd212f3-6dfd-11d4-b9c8-0060b0ee281b");

/// `XCAFDocCentroid`: the center of mass of a shape.
#[derive(Default)]
pub struct XCAFDocCentroid {
    transient: StandardTransient,
    base: TdfAttributeBase,
    centroid: SharedState<Point3>,
}
martensite_tkernel::impl_transient!(XCAFDocCentroid, "XCAFDocCentroid", dyn TdfAttribute, transient, interfaces: [dyn TdfAttribute]);

impl XCAFDocCentroid {
    /// `XCAFDocCentroid()`: the origin.
    pub fn new() -> Self {
        Self::default()
    }
    /// `GetID`.
    pub fn get_id() -> Guid {
        CENTROID_ID
    }
    /// `Set(L, pnt)`: finds or adds the attribute, then sets the point.
    pub fn set(label: &TdfLabel, point: &Point3) -> Result<Handle<XCAFDocCentroid>, Failure> {
        let attribute = find_or_add(label, &CENTROID_ID, Self::new)?;
        live(&attribute, "XCAFDocCentroid::Set")?.set_point(point)?;
        Ok(attribute)
    }
    /// `Set(pnt)`: backed up.
    pub fn set_point(&self, point: &Point3) -> Result<(), Failure> {
        self.backup()?;
        *self.centroid.write() = *point;
        Ok(())
    }
    /// `Get()`.
    pub fn get(&self) -> Point3 {
        *self.centroid.read()
    }
    /// `Get(label, pnt)`: the point of the label's centroid, false without it.
    pub fn get_of_label(label: &TdfLabel, point: &mut Point3) -> Result<bool, Failure> {
        let found = find::<XCAFDocCentroid>(label, &CENTROID_ID)?;
        let Some(found) = found.get() else {
            return Ok(false);
        };
        *point = found.get();
        Ok(true)
    }
}

impl TdfAttribute for XCAFDocCentroid {
    fn attribute_base(&self) -> &TdfAttributeBase {
        &self.base
    }
    fn id(&self) -> Guid {
        CENTROID_ID
    }
    fn restore(&self, with: &Handle<dyn TdfAttribute>) -> Result<(), Failure> {
        let other = downcast::<XCAFDocCentroid>(with, "XCAFDocCentroid::Restore")?;
        *self.centroid.write() = live(&other, "XCAFDocCentroid::Restore")?.get();
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
        let target = downcast::<XCAFDocCentroid>(into, "XCAFDocCentroid::Paste")?;
        live(&target, "XCAFDocCentroid::Paste")?.set_point(&self.get())
    }
    /// `Dump`: `"Centroid ( x,y,z)"` with the default stream precision.
    fn dump(&self, stream: &mut dyn io::Write) -> io::Result<()> {
        let point = self.get();
        let text = |value: f64| AsciiString::from_real(value).as_bytes().to_vec();
        stream.write_all(b"Centroid ( ")?;
        stream.write_all(&text(point.x()))?;
        stream.write_all(b",")?;
        stream.write_all(&text(point.y()))?;
        stream.write_all(b",")?;
        stream.write_all(&text(point.z()))?;
        stream.write_all(b")")
    }
    fn dump_json(&self, stream: &mut DumpStream, depth: i32) -> io::Result<()> {
        class_begin(stream, "XCAFDocCentroid")?;
        dump_attribute_json_base(self, stream, depth)?;
        field_point(stream, depth, "&myCentroid", &self.get())
    }
}
