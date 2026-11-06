//! The value commands the hand-written scripts read back: `bounding`, `get_name`, `XFindColor`, on real API.
use crate::reports::g17;
use crate::xs_session::XsSession;
use martensite_tkernel::{
    quantity::ColorRgba,
    standard::Handle,
    tcollection::{AsciiString, ExtendedString},
};
use martensite_tklcaf::{
    tdata_std::{TDataStdGenericExtString, TDataStdName},
    tdf::{TdfLabel, TdfTool},
};
use martensite_tkmath::bnd::BoundingBox3;
use martensite_tktopalgo::brep_bnd_lib::BRepBndLib;
use martensite_tkxcaf::xcaf_doc::XCAFDocDocumentTool;

/// `TdfTool::label(doc, entry)`: the label of an entry; a null one when it does not exist.
pub fn label_of(session: &XsSession, doc: &str, entry: &str) -> TdfLabel {
    let data = session.documents[doc].get().expect("a document").get_data();
    let mut label = TdfLabel::default();
    TdfTool::label_from_entry(&data, &AsciiString::from(entry), &mut label, false)
        .unwrap_or_else(|failure| panic!("{entry}: {failure:?}"));
    label
}

impl XsSession {
    /// `bounding name [-dump]`: the box's six coordinates, written as the command writes them.
    /// Example: `session.bounding("a", false)`.
    pub fn bounding(&self, name: &str, words: &str, dump: bool) -> [f64; 6] {
        println!("bounding {name}{words} ");
        let shape = self.bound_shape(name);
        let mut bounds = BoundingBox3::default();
        BRepBndLib::add(&shape, &mut bounds, true)
            .unwrap_or_else(|failure| panic!("bounding {name}: {failure:?}"));
        let [xmin, ymin, zmin, xmax, ymax, zmax] = bounds
            .get()
            .unwrap_or_else(|failure| panic!("bounding {name}: {failure:?}"));
        if dump {
            println!("Axes-aligned bounding box");
            println!("X-range: {} {}", g17(xmin), g17(xmax));
            println!("Y-range: {} {}", g17(ymin), g17(ymax));
            println!("Z-range: {} {}", g17(zmin), g17(zmax));
            println!();
        } else {
            println!(
                "{} {} {} {} {} {}",
                g17(xmin),
                g17(ymin),
                g17(zmin),
                g17(xmax),
                g17(ymax),
                g17(zmax)
            );
            println!();
        }
        [xmin, ymin, zmin, xmax, ymax, zmax]
    }

    /// `GetName doc entry`: the name on a label, in UTF-8; empty with `get_name`'s complaint when none.
    /// Example: `session.get_name("D", "0:1:1:1")`.
    pub fn get_name(&self, doc: &str, entry: &str) -> String {
        println!("GetName {doc} {entry} ");
        let label = label_of(self, doc, entry);
        let mut name: Handle<TDataStdName> = Handle::default();
        if !label
            .find_attribute_as(&TDataStdName::get_id(), &mut name)
            .expect("a name")
        {
            println!("Error: label has no name");
            return String::new();
        }
        let text: ExtendedString = name.get().expect("a name").get();
        let text = String::from_utf8_lossy(AsciiString::from(&text).as_bytes()).into_owned();
        println!("{text}");
        text
    }

    /// `XFindColor doc r g b`: the entry of the colour's label, empty when the document has no such colour.
    /// Example: `session.find_color("D", 0.0, 0.0, 1.0)`.
    pub fn find_color(&self, doc: &str, red: f32, green: f32, blue: f32) -> String {
        println!("XFindColor {doc} {red} {green} {blue} ");
        let main = self.documents[doc]
            .get()
            .expect("a document")
            .main()
            .expect("a main label");
        let tool = XCAFDocDocumentTool::color_tool(&main).expect("a color tool");
        let color = ColorRgba::new(red, green, blue, 1.0).expect("a colour");
        let label = tool
            .get()
            .expect("a color tool")
            .find_color_label(&color)
            .expect("a colour search");
        let mut entry = AsciiString::default();
        TdfTool::entry(&label, &mut entry);
        let entry = String::from_utf8_lossy(entry.as_bytes()).into_owned();
        println!("{entry}");
        entry
    }
}
