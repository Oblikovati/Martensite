//! `XStat doc 1`'s structure, `XCheckProps`, `XGetLayerRefs` and the transaction commands of the scripts.
use crate::reports::g17;
use crate::script_values::label_of;
use crate::xs_session::XsSession;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::{
    quantity::Color,
    standard::Handle,
    tcollection::{AsciiString, ExtendedString},
};
use martensite_tkgeombase::gprop::GProps;
use martensite_tklcaf::tdf::TdfAttributeState;
use martensite_tklcaf::{
    tdata_std::{TDataStdGenericExtString, TDataStdName},
    tdf::{TdfChildIterator, TdfLabel, TdfTool},
};
use martensite_tktopalgo::brep_gprop::BRepGProp;
use martensite_tkxcaf::xcaf_doc::{
    XCAFDoc, XCAFDocArea, XCAFDocCentroid, XCAFDocColorTool, XCAFDocColorType, XCAFDocDocumentTool,
    XCAFDocGraphNode, XCAFDocLayerTool, XCAFDocShapeTool, XCAFDocVolume,
};

fn utf8(text: &ExtendedString) -> String {
    String::from_utf8_lossy(AsciiString::from(text).as_bytes()).into_owned()
}

fn entry_of(label: &TdfLabel) -> String {
    let mut entry = AsciiString::default();
    TdfTool::entry(label, &mut entry);
    String::from_utf8_lossy(entry.as_bytes()).into_owned()
}

fn name_of(label: &TdfLabel) -> Option<String> {
    let mut name: Handle<TDataStdName> = Handle::default();
    label
        .find_attribute_as(&TDataStdName::get_id(), &mut name)
        .expect("a name")
        .then(|| utf8(&name.get().expect("a name").get()))
}

/// One label of `XStat`'s structure: its entry, name and attributes, then its children a level down.
fn structure_of(session: &XsSession, doc: &str, label: &TdfLabel, level: usize, text: &mut String) {
    *text += &"  ".repeat(level + 1);
    *text += &entry_of(label);
    *text += &format!(
        " {}  has attributes: ",
        name_of(label).unwrap_or_else(|| "NoName".to_owned())
    );
    let mut centroid: Handle<XCAFDocCentroid> = Handle::default();
    if label
        .find_attribute_as(&XCAFDocCentroid::get_id(), &mut centroid)
        .expect("a centroid")
    {
        *text += "Centroid ";
    }
    let mut value = 0.0;
    if XCAFDocVolume::get_of_label(label, &mut value).expect("a volume") {
        *text += &format!("Volume({}) ", g17(value));
    }
    if XCAFDocArea::get_of_label(label, &mut value).expect("an area") {
        *text += &format!("Area({}) ", g17(value));
    }
    let color = [
        XCAFDocColorType::ColorGen,
        XCAFDocColorType::ColorSurf,
        XCAFDocColorType::ColorCurv,
    ]
    .into_iter()
    .find_map(|kind| XCAFDocColorTool::get_label_color(label, kind).expect("a colour"));
    if let Some(color) = color {
        *text += &format!(
            "Color({} {}) ",
            Color::string_name(color.rgb().name()),
            color.alpha()
        );
    }
    let layers = XCAFDocLayerTool::get_layers(label).expect("layers");
    if !layers.is_empty() {
        let names: Vec<String> = layers
            .iter()
            .map(|name| format!("\"{}\"", utf8(name)))
            .collect();
        *text += &format!("Layer({}) ", names.join(" "));
    }
    text.push('\n');
    if label.has_child() {
        let mut children = TdfChildIterator::new(label, false).expect("children");
        while children.more() {
            structure_of(session, doc, &children.value().clone(), level + 1, text);
            children.next();
        }
    }
}

impl XsSession {
    /// `XStat doc 1`: `Structure of shapes in the document:`, each label, then what `XStat doc` writes;
    /// returns all of it as the `regexp`s read it.
    /// Example: `session.statistics_with_structure("D")`.
    pub fn statistics_with_structure(&self, doc: &str) -> String {
        println!("XStat {doc} 1 ");
        let main = self.documents[doc]
            .get()
            .expect("a document")
            .main()
            .expect("a main label");
        let tool = XCAFDocDocumentTool::shape_tool(&main).expect("a shape tool");
        let mut text = String::from("Structure of shapes in the document:\n");
        for label in tool
            .get()
            .expect("a shape tool")
            .get_free_shapes()
            .expect("free shapes")
        {
            structure_of(self, doc, &label, 0, &mut text);
        }
        let statistics = crate::reports::statistics_with_text(&self.documents[doc]).1;
        text += &statistics;
        print!("{text}");
        text
    }

    /// `XCheckProps doc`: the defects of each shape label's validation properties, as a table.
    /// Example: `session.check_props("D0")`.
    pub fn check_props(&self, doc: &str) -> String {
        println!("XCheckProps {doc} ");
        let main = self.documents[doc]
            .get()
            .expect("a document")
            .main()
            .expect("a main label");
        let tool = XCAFDocDocumentTool::shape_tool(&main).expect("a shape tool");
        let mut labels = tool
            .get()
            .expect("a shape tool")
            .get_shapes()
            .expect("shapes");
        let mut text = String::from(
            "Label            Area defect   Volume defect    dX      dY      dZ    Name\n",
        );
        let mut at = 0;
        while at < labels.len() {
            let label = labels[at].clone();
            if XCAFDocShapeTool::is_assembly(&label).expect("an assembly test") {
                let mut after = at + 1;
                let mut components = Vec::new();
                XCAFDocShapeTool::get_components(&label, &mut components, false)
                    .expect("components");
                for component in components {
                    if has_props(&component) {
                        labels.insert(after, component);
                        after += 1;
                    }
                }
            }
            text += &props_line(&label);
            at += 1;
        }
        print!("{text}");
        text
    }

    /// `XGetLayerRefs doc entry`: what the command writes.
    /// Example: `session.layer_refs("d", "0:1:3:1")`.
    pub fn layer_refs(&self, doc: &str, entry: &str) -> String {
        println!("XGetLayerRefs {doc} {entry} ");
        let label = label_of(self, doc, entry);
        let mut node: Handle<XCAFDocGraphNode> = Handle::default();
        let mut text = String::new();
        if !label
            .find_attribute_as(&XCAFDoc::layer_ref_guid(), &mut node)
            .expect("a graph node")
        {
            text += &format!("Error: Label \"{entry}\" does not have a layer ref.\n");
            print!("{text}");
            return text;
        }
        let node = node.get().expect("a graph node");
        if node.nb_children() > 0 {
            text += &format!("Label \"{entry}\" childs:\n");
            for child in node.children() {
                text += &format!("{}\n", entry_of(&child.get().expect("a child").label()));
            }
        }
        print!("{text}");
        text
    }
}

fn has_props(label: &TdfLabel) -> bool {
    let mut volume: Handle<XCAFDocVolume> = Handle::default();
    let mut area: Handle<XCAFDocArea> = Handle::default();
    let mut centroid: Handle<XCAFDocCentroid> = Handle::default();
    label
        .find_attribute_as(&XCAFDocVolume::get_id(), &mut volume)
        .expect("a volume")
        || label
            .find_attribute_as(&XCAFDocArea::get_id(), &mut area)
            .expect("an area")
        || label
            .find_attribute_as(&XCAFDocCentroid::get_id(), &mut centroid)
            .expect("a centroid")
}

/// `(int)` of a percentage, with the `999` for a measure near zero.
fn percent(stored: f64, measured: f64) -> i32 {
    if measured.abs() > 1e-10 {
        (100. * (stored - measured) / measured) as i32
    } else {
        999
    }
}

fn props_line(label: &TdfLabel) -> String {
    let mut text = format!("{:<12.12}", entry_of(label));
    let mut shape = Shape::default();
    XCAFDocShapeTool::get_shape(label, &mut shape).expect("a label's shape");
    let mut area: Handle<XCAFDocArea> = Handle::default();
    if label
        .find_attribute_as(&XCAFDocArea::get_id(), &mut area)
        .expect("an area")
    {
        let mut props = GProps::default();
        BRepGProp::surface_properties_eps(&shape, &mut props, 0.001, false)
            .expect("surface properties");
        let stored = area.get().expect("an area").get();
        text += &format!(
            "{:9.1} ({:3}%)",
            stored - props.mass(),
            percent(stored, props.mass())
        );
    } else {
        text += &" ".repeat(16);
    }
    let mut volume: Handle<XCAFDocVolume> = Handle::default();
    let mut centroid: Handle<XCAFDocCentroid> = Handle::default();
    let has_volume = label
        .find_attribute_as(&XCAFDocVolume::get_id(), &mut volume)
        .expect("a volume");
    let has_centroid = label
        .find_attribute_as(&XCAFDocCentroid::get_id(), &mut centroid)
        .expect("a centroid");
    if has_volume || has_centroid {
        let mut props = GProps::default();
        BRepGProp::volume_properties_eps(&shape, &mut props, 0.001, true, false)
            .expect("volume properties");
        let measured = props.mass();
        let centre = props.centre_of_mass();
        if has_volume {
            let stored = volume.get().expect("a volume").get();
            text += &format!(
                "{:9.1} ({:3}%)",
                stored - measured,
                percent(stored, measured)
            );
        } else {
            text += &" ".repeat(16);
        }
        if has_centroid {
            let point = centroid.get().expect("a centroid").get();
            text += &format!(
                " {:7.2} {:7.2} {:7.2}",
                point.x() - centre.x(),
                point.y() - centre.y(),
                point.z() - centre.z()
            );
        } else {
            text += &" ".repeat(24);
        }
    } else {
        text += &" ".repeat(40);
    }
    if let Some(name) = name_of(label) {
        text += &format!("  \"{name}\"");
    }
    text.push('\n');
    text
}
