//! The XCAFApp package: `XCAFAppApplication`, the application of XDE documents.
use crate::xcaf_doc::XCAFDocDocumentTool;
use crate::xcaf_prs::XCAFPrsDriver;
use martensite_tkcdf::cdf::CdfApplication;
use martensite_tkcdf::cdm::CdmDocument;
use martensite_tkernel::standard::dump::DumpStream;
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tklcaf::tdoc_std::{TDocStdApplication, TDocStdApplicationBase, TDocStdDocument};
use martensite_tkservice::json_dump::{base_class, class_begin, field_integer};
use martensite_tkvcaf::tprs_std::TPrsStdDriverTable;
use std::io;
use std::sync::OnceLock;

/// `XCAFAppApplication`: a `TDocStdApplication` whose new documents get the XDE tools.
///
/// ```
/// use martensite_tkernel::standard::Handle;
/// use martensite_tkernel::tcollection::ExtendedString;
/// use martensite_tklcaf::tdoc_std::{TDocStdApplication, TDocStdDocument};
/// use martensite_tkxcaf::xcaf_app::XCAFAppApplication;
/// use martensite_tkxcaf::xcaf_doc::XCAFDocDocumentTool;
/// let application = XCAFAppApplication::new();
/// let mut document: Handle<TDocStdDocument> = Handle::default();
/// application.new_tdoc_std_document(&ExtendedString::from_utf8("BinXCAF"), &mut document)?;
/// assert!(XCAFDocDocumentTool::is_xcaf_document(&document)?);
/// # Ok::<(), martensite_tkernel::standard::Failure>(())
/// ```
#[derive(Default)]
pub struct XCAFAppApplication {
    transient: StandardTransient,
    base: TDocStdApplicationBase,
}
martensite_tkernel::impl_transient!(XCAFAppApplication, "XCAFAppApplication", dyn TDocStdApplication, transient,
    interfaces: [dyn martensite_tkcdf::cdm::CdmApplication, dyn CdfApplication, dyn TDocStdApplication]);

impl XCAFAppApplication {
    /// `XCAFAppApplication()`, in its bound handle: registers the `XCAFPrsDriver`.
    pub fn new() -> Handle<XCAFAppApplication> {
        let application = Handle::new(Self::default());
        if let Some(own) = application.get() {
            martensite_tkcdf::cdm::CdmApplication::application_base(own)
                .bind_self(&application.cast());
        }
        if let Some(table) = TPrsStdDriverTable::get().get() {
            table.add_driver(
                &XCAFPrsDriver::get_id(),
                &Handle::new(XCAFPrsDriver::new()).cast(),
            );
        }
        application
    }
    /// `GetApplication`: the process-wide application, created once.
    pub fn get_application() -> Handle<XCAFAppApplication> {
        static APPLICATION: OnceLock<Handle<XCAFAppApplication>> = OnceLock::new();
        APPLICATION.get_or_init(Self::new).clone()
    }
}

impl martensite_tkcdf::cdm::CdmApplication for XCAFAppApplication {
    martensite_tklcaf::tdoc_std_application_cdm_members!();
}

impl CdfApplication for XCAFAppApplication {
    martensite_tklcaf::tdoc_std_application_cdf_members!();
    /// `InitDocument(aDoc)`: the document tool of the main label (`StandardNullObject` for
    /// a document that is not a `TDocStdDocument`).
    fn init_document(&self, document: &Handle<dyn CdmDocument>) -> Result<(), Failure> {
        let document = document.cast::<TDocStdDocument>();
        let main =
            martensite_tklcaf::live(&document, "XCAFAppApplication::InitDocument")?.main()?;
        XCAFDocDocumentTool::set(&main, true)?;
        Ok(())
    }
}

impl TDocStdApplication for XCAFAppApplication {
    fn tdoc_std_application_base(&self) -> &TDocStdApplicationBase {
        &self.base
    }
    /// `ResourcesName`: `XCAF`.
    fn resources_name(&self) -> String {
        "XCAF".to_owned()
    }
    /// `DumpJson`: the class, then the `TDocStdApplication` dump.
    fn dump_json(&self, stream: &mut DumpStream, depth: i32) -> io::Result<()> {
        class_begin(stream, "XCAFAppApplication")?;
        base_class(stream, depth, &|s, _| {
            class_begin(s, "TDocStd_Application")?;
            field_integer(s, "myIsDriverLoaded", i32::from(self.is_driver_loaded()))
        })
    }
}
