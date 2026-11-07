//! The XDE commands `xcaf/end` runs on a document: colours, layers, labels and validation properties,
//! each writing what the command writes and returning what the script reads back.
use crate::documents::{entry_of, label_of, shape_of};
use crate::iges_session::IgesSession;
use crate::reports::g17;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::{
    quantity::{Color, ColorRgba},
    standard::Handle,
    tcollection::{AsciiString, ExtendedString},
};
use martensite_tkgeombase::gprop::GProps;
use martensite_tklcaf::tdf::TdfLabel;
use martensite_tktopalgo::brep_gprop::BRepGProp;
use martensite_tkxcaf::xcaf_doc::{
    XCAFDocArea, XCAFDocCentroid, XCAFDocColorTool, XCAFDocColorType, XCAFDocDocumentTool,
    XCAFDocLayerTool, XCAFDocShapeTool, XCAFDocVolume,
};

/// A command's echo and, when it returned any, its result.
fn answer(command: String, result: &str) -> String {
    println!("{command} ");
    if !result.is_empty() {
        println!("{result}");
    }
    result.to_owned()
}

fn utf8(text: &ExtendedString) -> String {
    String::from_utf8_lossy(AsciiString::from(text).as_bytes()).into_owned()
}

fn color_type(word: &str) -> XCAFDocColorType {
    match word {
        "s" => XCAFDocColorType::ColorSurf,
        "c" => XCAFDocColorType::ColorCurv,
        other => panic!("colour type {other}; expected s or c"),
    }
}

fn color_text(color: &ColorRgba) -> String {
    Color::string_name(color.rgb().name()).to_string()
}

fn rgba(rgb: [f32; 3]) -> ColorRgba {
    ColorRgba::new(rgb[0], rgb[1], rgb[2], 1.0).expect("a colour")
}

fn rgb_words(rgb: [f32; 3]) -> String {
    format!("{} {} {}", rgb[0], rgb[1], rgb[2])
}

impl IgesSession {
    fn main_label(&self, doc: &str) -> TdfLabel {
        self.documents[doc]
            .get()
            .expect("a document")
            .main()
            .expect("a main label")
    }

    fn colors(&self, doc: &str) -> Handle<XCAFDocColorTool> {
        XCAFDocDocumentTool::color_tool(&self.main_label(doc)).expect("a color tool")
    }

    fn layers(&self, doc: &str) -> Handle<XCAFDocLayerTool> {
        XCAFDocDocumentTool::layer_tool(&self.main_label(doc)).expect("a layer tool")
    }

    fn shapes(&self, doc: &str) -> Handle<XCAFDocShapeTool> {
        XCAFDocDocumentTool::shape_tool(&self.main_label(doc)).expect("a shape tool")
    }

    /// `XAddShape doc shape`. Example: `session.add_shape("D", "b")`.
    pub fn add_shape(&mut self, doc: &str, name: &str) -> String {
        let shape = self.bound_shape(name);
        self.ensure_length_unit(doc);
        let label = self
            .shapes(doc)
            .add_shape(&shape, true, true)
            .expect("a shape label");
        answer(format!("XAddShape {doc} {name}"), &entry_of(&label))
    }

    fn ensure_length_unit(&self, doc: &str) {
        let document = &self.documents[doc];
        let mut unit = 1.0;
        if !XCAFDocDocumentTool::get_length_unit(document, &mut unit).expect("a length unit") {
            martensite_tkxsbase::xs_algo::XSAlgoShapeProcessor::prepare_for_transfer();
            let unit = martensite_tkxsbase::xs_algo::xs_algo_units()
                .get_cascade_length_unit(martensite_tkernel::units_methods::LengthUnit::Meter);
            XCAFDocDocumentTool::set_length_unit(document, unit).expect("a length unit");
        }
    }

    /// `XAddColor doc r g b`. Example: `session.add_color("D", [1.0, 0.0, 0.0])`.
    pub fn add_color(&self, doc: &str, rgb: [f32; 3]) -> String {
        let label = self
            .colors(doc)
            .add_color(&rgba(rgb))
            .expect("a colour label");
        answer(
            format!("XAddColor {doc} {}", rgb_words(rgb)),
            &entry_of(&label),
        )
    }

    /// `XFindColor doc r g b`: empty when the document has no such colour.
    /// Example: `session.find_color("D", [1.0, 0.0, 0.0])`.
    pub fn find_color(&self, doc: &str, rgb: [f32; 3]) -> String {
        let label = self
            .colors(doc)
            .find_color_label(&rgba(rgb))
            .expect("a colour search");
        answer(
            format!("XFindColor {doc} {}", rgb_words(rgb)),
            &entry_of(&label),
        )
    }

    /// `XGetAllColors doc`: the names as a list. Example: `session.all_colors("D")`.
    pub fn all_colors(&self, doc: &str) -> Vec<String> {
        let mut names = Vec::new();
        for label in self.colors(doc).get_colors().expect("colours") {
            if let Some(color) = XCAFDocColorTool::get_color_rgba(&label).expect("a colour") {
                names.push(color_text(&color));
            }
        }
        let line: String = names.iter().map(|name| format!("{name} ")).collect();
        answer(format!("XGetAllColors {doc}"), &line);
        names
    }

    /// `XSetColor doc shape-or-label r g b kind`. Example: `session.set_color("D", "0:1:1:1", [1.0, 0.0, 0.0], "s")`.
    pub fn set_color(&self, doc: &str, target: &str, rgb: [f32; 3], kind: &str) {
        println!("XSetColor {doc} {target} {} {kind} ", rgb_words(rgb));
        let tool = self.colors(doc);
        let label = label_of(self, doc, target);
        if label.is_null() {
            tool.set_shape_color(&self.bound_shape(target), &rgba(rgb), color_type(kind))
                .expect("a colour");
        } else {
            tool.set_color(&label, &rgba(rgb), color_type(kind))
                .expect("a colour");
        }
    }

    /// `XGetShapeColor doc label kind`: the colour's name, empty when none.
    /// Example: `session.shape_color("D", "0:1:1:1", "s")`.
    pub fn shape_color(&self, doc: &str, entry: &str, kind: &str) -> String {
        let label = label_of(self, doc, entry);
        let color = XCAFDocColorTool::get_label_color(&label, color_type(kind)).expect("a colour");
        answer(
            format!("XGetShapeColor {doc} {entry} {kind}"),
            &color.as_ref().map(color_text).unwrap_or_default(),
        )
    }

    /// `XRemoveColor doc label`. Example: `session.remove_color("D", "0:1:2:1")`.
    pub fn remove_color(&self, doc: &str, entry: &str) {
        println!("XRemoveColor {doc} {entry} ");
        let label = label_of(self, doc, entry);
        assert!(!label.is_null(), "XRemoveColor: {entry} is not found");
        self.colors(doc)
            .remove_color(&label)
            .expect("a colour removed");
    }

    /// `XUnsetColor doc label kind`. Example: `session.unset_color("D", "0:1:1:1", "c")`.
    pub fn unset_color(&self, doc: &str, entry: &str, kind: &str) {
        println!("XUnsetColor {doc} {entry} {kind} ");
        let label = label_of(self, doc, entry);
        self.colors(doc)
            .unset_color(&label, color_type(kind))
            .expect("a colour unset");
    }

    /// `XFindShape doc shape`: the label of a top-level shape.
    /// Example: `session.find_shape("D", "b1")`.
    pub fn find_shape(&self, doc: &str, name: &str) -> String {
        let label = self
            .shapes(doc)
            .find_shape_label(&self.bound_shape(name), false)
            .expect("a shape search");
        answer(format!("XFindShape {doc} {name}"), &entry_of(&label))
    }

    /// `XGetTopLevelShapes doc`: the entries as a list. Example: `session.top_level_shapes("D")`.
    pub fn top_level_shapes(&self, doc: &str) -> Vec<String> {
        let entries: Vec<String> = self
            .shapes(doc)
            .get_shapes()
            .expect("shapes")
            .iter()
            .map(entry_of)
            .collect();
        let line: String = entries.iter().map(|entry| format!("{entry} ")).collect();
        answer(format!("XGetTopLevelShapes {doc}"), &line);
        entries
    }

    /// `XLabelInfo doc label`. Example: `session.label_info("D", "0:1:1:1")`.
    pub fn label_info(&self, doc: &str, entry: &str) -> String {
        let label = label_of(self, doc, entry);
        let mut text = String::new();
        if XCAFDocShapeTool::is_shape(&label).expect("a label test") {
            text += "There are a TopLevelShape";
        }
        if XCAFDocShapeTool::is_component(&label).expect("a label test") {
            text += "There are a Component";
        }
        if XCAFDocShapeTool::is_assembly(&label).expect("a label test") {
            text += "There are an Assembly";
        }
        if XCAFDocShapeTool::is_free(&label).expect("a label test") {
            text += "This Shape don't used";
        }
        answer(format!("XLabelInfo {doc} {entry}"), &text)
    }

    /// `XAddLayer doc name`. Example: `session.add_layer("D", "L1")`.
    pub fn add_layer(&self, doc: &str, name: &str) -> String {
        let label = self
            .layers(doc)
            .add_layer(&ExtendedString::from_utf8(name))
            .expect("a layer label");
        answer(format!("XAddLayer {doc} {name}"), &entry_of(&label))
    }

    /// `XFindLayer doc name`: empty when there is none. Example: `session.find_layer("D", "L1")`.
    pub fn find_layer(&self, doc: &str, name: &str) -> String {
        let label = self
            .layers(doc)
            .find_layer_with(&ExtendedString::from_utf8(name), false, false)
            .expect("a layer search");
        answer(format!("XFindLayer {doc} {name}"), &entry_of(&label))
    }

    /// `XGetAllLayers doc`: the quoted names as a list. Example: `session.all_layers("D")`.
    pub fn all_layers(&self, doc: &str) -> Vec<String> {
        let tool = self.layers(doc);
        let mut names = Vec::new();
        for label in tool.get_layer_labels().expect("layers") {
            let name = tool.get_layer(&label).expect("a layer").unwrap_or_default();
            names.push(format!("\"{}\"", utf8(&name)));
        }
        let line: String = names.iter().map(|name| format!("{name} ")).collect();
        answer(format!("XGetAllLayers {doc}"), &line);
        names
    }

    /// `XGetLayerLabels doc`: the entries as a list. Example: `session.layer_labels("D")`.
    pub fn layer_labels(&self, doc: &str) -> Vec<String> {
        let entries: Vec<String> = self
            .layers(doc)
            .get_layer_labels()
            .expect("layers")
            .iter()
            .map(entry_of)
            .collect();
        let line: String = entries.iter().map(|entry| format!("{entry} ")).collect();
        answer(format!("XGetLayerLabels {doc}"), &line);
        entries
    }

    /// `XGetOneLayer doc label`: the quoted name. Example: `session.one_layer("D", "0:1:3:1")`.
    pub fn one_layer(&self, doc: &str, entry: &str) -> String {
        let tool = self.layers(doc);
        let label = label_of(self, doc, entry);
        let name = tool.get_layer(&label).expect("a layer").unwrap_or_default();
        answer(
            format!("XGetOneLayer {doc} {entry}"),
            &format!("\"{}\"", utf8(&name)),
        )
    }

    /// `XRemoveLayer doc name`. Example: `session.remove_layer("D", "L2")`.
    pub fn remove_layer(&self, doc: &str, name: &str) {
        println!("XRemoveLayer {doc} {name} ");
        let tool = self.layers(doc);
        let label = tool
            .find_layer_with(&ExtendedString::from_utf8(name), false, false)
            .expect("a layer search");
        if !label.is_null() {
            tool.remove_layer(&label).expect("a layer removed");
        }
    }

    /// `XSetLayer doc shape-or-label name`. Example: `session.set_layer("D", "0:1:1:1", "L1")`.
    pub fn set_layer(&self, doc: &str, target: &str, name: &str) {
        println!("XSetLayer {doc} {target} {name} ");
        let tool = self.layers(doc);
        let layer = ExtendedString::from_utf8(name);
        let label = label_of(self, doc, target);
        if label.is_null() {
            tool.set_shape_layer(&self.bound_shape(target), &layer, false)
                .expect("a layer");
        } else {
            tool.set_layer(&label, &layer, false).expect("a layer");
        }
    }

    /// `XGetLayers doc shape-or-label`: the line the command writes.
    /// Example: `session.layers_of("D", "0:1:1:1")`.
    pub fn layers_of(&self, doc: &str, target: &str) -> String {
        let label = label_of(self, doc, target);
        let names = if label.is_null() {
            self.layers(doc)
                .get_shape_layers(&self.bound_shape(target))
                .expect("layers")
        } else {
            XCAFDocLayerTool::get_layers(&label).expect("layers")
        };
        let line: String = names
            .iter()
            .map(|name| format!("\"{}\" ", utf8(name)))
            .collect();
        answer(format!("XGetLayers {doc} {target}"), &line)
    }

    /// `XUnSetLayer doc shape-or-label name`. Example: `session.unset_layer("D", "c", "L3")`.
    pub fn unset_layer(&self, doc: &str, target: &str, name: &str) {
        println!("XUnSetLayer {doc} {target} {name} ");
        let tool = self.layers(doc);
        let layer = ExtendedString::from_utf8(name);
        let label = label_of(self, doc, target);
        if label.is_null() {
            tool.unset_shape_one_layer(&self.bound_shape(target), &layer)
                .expect("a layer unset");
        } else {
            tool.unset_one_layer(&label, &layer).expect("a layer unset");
        }
    }

    /// `XSetProps doc label eps`: writes the three measures and stores them on the label.
    /// Example: `session.set_props("T", "0:1:1:1", 1.0)`.
    pub fn set_props(&self, doc: &str, entry: &str, eps: f64, eps_word: &str) {
        println!("XSetProps {doc} {entry} {eps_word} ");
        let label = label_of(self, doc, entry);
        if label.is_null() {
            return;
        }
        let shape: Shape = shape_of(&label);
        let mut volume = GProps::default();
        BRepGProp::volume_properties_eps(&shape, &mut volume, eps, true, false)
            .expect("volume properties");
        XCAFDocVolume::set(&label, volume.mass()).expect("a volume");
        let centre = volume.centre_of_mass();
        XCAFDocCentroid::set(&label, &centre).expect("a centroid");
        let mut area = GProps::default();
        BRepGProp::surface_properties_eps(&shape, &mut area, eps, false)
            .expect("surface properties");
        XCAFDocArea::set(&label, area.mass()).expect("an area");
        println!(
            "{entry}: Volume = {}, Area = {}, Centroid is ({}, {}, {})",
            g17(volume.mass()),
            g17(area.mass()),
            g17(centre.x()),
            g17(centre.y()),
            g17(centre.z())
        );
    }

    /// `XGetVolume doc label`: the word the command writes. Example: `session.volume_word("T", "0:1:1:1")`.
    pub fn volume_word(&self, doc: &str, entry: &str) -> String {
        let label = label_of(self, doc, entry);
        let mut value = 0.0;
        let found = XCAFDocVolume::get_of_label(&label, &mut value).expect("a volume");
        answer(
            format!("XGetVolume {doc} {entry}"),
            &if found { g17(value) } else { String::new() },
        )
    }
}
