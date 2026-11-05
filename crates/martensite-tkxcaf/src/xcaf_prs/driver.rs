//! `XCAFPrsDriver`: the presentation driver of the XDE shape labels.
use super::XCAFPrsAISObject;
use crate::xcaf_doc::XCAFDocShapeTool;
use martensite_tkernel::standard::{Failure, Guid, Handle, StandardTransient};
use martensite_tklcaf::guid_from_text;
use martensite_tklcaf::tdf::TdfLabel;
use martensite_tkv3d::ais::AisInteractiveObject;
use martensite_tkvcaf::tprs_std::TPrsStdDriver;

/// `XCAFPrsDriver`.
///
/// ```
/// use martensite_tklcaf::tdf::TdfData;
/// use martensite_tkvcaf::tprs_std::TPrsStdDriver;
/// use martensite_tkxcaf::xcaf_prs::XCAFPrsDriver;
/// let data = TdfData::new();
/// let mut ais = Default::default();
/// assert!(!XCAFPrsDriver::new().update(&data.root(), &mut ais)?);
/// # Ok::<(), martensite_tkernel::standard::Failure>(())
/// ```
#[derive(Default)]
pub struct XCAFPrsDriver {
    transient: StandardTransient,
}
martensite_tkernel::impl_transient!(XCAFPrsDriver, "XCAFPrsDriver", dyn TPrsStdDriver, transient,
    interfaces: [dyn TPrsStdDriver]);

impl XCAFPrsDriver {
    /// `XCAFPrsDriver()`.
    pub fn new() -> Self {
        Self::default()
    }
    /// `GetID`: the GUID the driver is registered with.
    pub fn get_id() -> Guid {
        const ID: Guid = guid_from_text("5b896afc-3adf-11d4-b9b7-0060b0ee281b");
        ID
    }
}

impl TPrsStdDriver for XCAFPrsDriver {
    /// `Update(L, ais)`: a new `XCAFPrsAISObject` for a shape label.
    fn update(
        &self,
        label: &TdfLabel,
        ais: &mut Handle<dyn AisInteractiveObject>,
    ) -> Result<bool, Failure> {
        if !XCAFDocShapeTool::is_shape(label)? {
            return Ok(false);
        }
        *ais = XCAFPrsAISObject::new(label)?.cast();
        Ok(true)
    }
}
