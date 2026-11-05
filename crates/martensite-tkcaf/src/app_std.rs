//! `AppStdApplication`: the standard OCAF application, whose resources are `Standard`.
use martensite_tkcdf::cdf::CdfApplication;
use martensite_tkcdf::cdm::CdmApplication;
use martensite_tkcdf::json_dump::{base_class, class_begin};
use martensite_tkernel::impl_transient;
use martensite_tkernel::standard::{Handle, StandardTransient, dump::DumpStream};
use martensite_tklcaf::tdoc_std::{TDocStdApplication, TDocStdApplicationBase};
use std::io;

/// `AppStdApplication`.
///
/// ```
/// use martensite_tkcaf::app_std::AppStdApplication;
/// use martensite_tklcaf::tdoc_std::TDocStdApplication;
/// let application = AppStdApplication::new();
/// assert_eq!(application.resources_name(), "Standard");
/// ```
#[derive(Default)]
pub struct AppStdApplication {
    transient: StandardTransient,
    base: TDocStdApplicationBase,
}
impl_transient!(AppStdApplication, "AppStdApplication", dyn TDocStdApplication, transient,
    interfaces: [dyn CdmApplication, dyn CdfApplication, dyn TDocStdApplication]);

impl AppStdApplication {
    /// `new AppStdApplication()`, already in its bound handle.
    pub fn new() -> Handle<AppStdApplication> {
        let application = Handle::new(Self::default());
        if let Some(own) = application.get() {
            CdmApplication::application_base(own).bind_self(&application.cast());
        }
        application
    }
    /// `DumpJson` (it hides the non-virtual `TDocStdApplication::DumpJson`): the class name,
    /// then the `TDocStdApplication` dump as its base.
    pub fn dump_json(&self, stream: &mut DumpStream, depth: i32) -> io::Result<()> {
        class_begin(stream, "AppStdApplication")?;
        base_class(stream, depth, &|s, d| {
            TDocStdApplication::dump_json(self, s, d)
        })
    }
}

impl CdmApplication for AppStdApplication {
    martensite_tklcaf::tdoc_std_application_cdm_members!();
}
impl CdfApplication for AppStdApplication {
    martensite_tklcaf::tdoc_std_application_cdf_members!();
}
impl TDocStdApplication for AppStdApplication {
    fn tdoc_std_application_base(&self) -> &TDocStdApplicationBase {
        &self.base
    }
    /// `ResourcesName`: `Standard`.
    fn resources_name(&self) -> String {
        String::from("Standard")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use martensite_tkernel::tcollection::ExtendedString;
    use martensite_tklcaf::tdoc_std::TDocStdDocument;

    #[test]
    fn documents_are_created_and_dumped_with_the_base() {
        let application = AppStdApplication::new();
        let mut document: Handle<TDocStdDocument> = Handle::default();
        application
            .new_tdoc_std_document(&ExtendedString::new("BinOcaf"), &mut document)
            .unwrap();
        assert_eq!(application.nb_documents().unwrap(), 1);
        let mut stream = DumpStream::default();
        application.dump_json(&mut stream, -1).unwrap();
        let text = String::from_utf8_lossy(stream.bytes()).into_owned();
        let expected = concat!(
            "\"className\": \"AppStdApplication\", ",
            "\"className\": \"TDocStd_Application\", \"IsDriverLoaded\": 1"
        );
        assert_eq!(text, expected);
    }
}
