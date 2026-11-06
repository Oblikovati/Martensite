//! `tests/xcaf/end` once, on `TypeOfFile igs` and `FACES VOLUME COLORS LAYERS`, the one path its IGES
//! grids take: it adds colours and layers to the first document, writes it, reads the file into a
//! second document, compares what both hold and shows the second.
use crate::iges_session::{IgesSession, image_folder, keep_written_file, locate_data_file};
use crate::support::{
    checkview::{CheckView, checkview},
    restore::restore,
    script_error::script_error,
    session::Session,
};
use crate::table::{Added, XcafRow};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::message::ProgressRange;
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tkshhealing::shape_fix::ShapeFixFreeBounds;
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeShape, BRepBuilderAPISewing};
use std::fmt::Write as _;

const MODE: &str = "FACES VOLUME COLORS LAYERS";
const RED: [f32; 3] = [1.0, 0.0, 0.0];
const GREEN: [f32; 3] = [0.0, 1.0, 0.0];
const BLUE: [f32; 3] = [0.0, 0.0, 1.0];
const YELLOW: [f32; 3] = [1.0, 1.0, 0.0];
const CYAN: [f32; 3] = [0.0, 1.0, 1.0];
const MAGENTA: [f32; 3] = [1.0, 0.0, 1.0];
const WHITE: [f32; 3] = [1.0, 1.0, 1.0];
const BLACK: [f32; 3] = [0.0, 0.0, 0.0];

/// What the script keeps of one document.
#[derive(Default)]
struct DocumentFacts {
    faces: usize,
    volume: String,
    closed_shells: i64,
    colors: Vec<String>,
    shape_labels: Vec<String>,
    layers: Vec<String>,
    layer_labels: Vec<String>,
}

/// The script's `ErrorCode`, which only ever rises, except where it sets 1 outright.
struct End<'a> {
    session: &'a mut IgesSession,
    row: &'a XcafRow,
    error_code: i32,
    is_comp: bool,
    first_closed_shells: i64,
}

/// Runs a row's script: `xcaf/begin`, the case's own lines, then `xcaf/end`.
/// Example: `run(&mut session, &row)`.
pub fn run(session: &mut IgesSession, row: &XcafRow) {
    println!("cpulimit 1000 ");
    println!("CPU and elapsed time limit set to 1000 seconds");
    if let Some(file) = row.restored {
        let path = locate_data_file(file)
            .unwrap_or_else(|| script_error(&format!("File {file} could not be found")));
        println!("restore {} Shape1 ", path.display());
        restore(&mut session.raster, file, "Shape1");
        println!("XNewDoc D_First ");
        session.new_document("D_First");
        session.add_shape("D_First", "Shape1");
    }
    End {
        session,
        row,
        error_code: 0,
        is_comp: false,
        first_closed_shells: 0,
    }
    .run();
}

fn script_list(words: &[String]) -> String {
    words.join(" ")
}

/// `expr abs(word)`: the digits of an integer stay; a real is written as `expr` writes it.
fn script_abs(word: &str) -> String {
    if !word.is_empty()
        && word
            .bytes()
            .all(|byte| byte.is_ascii_digit() || byte == b'-')
    {
        return word.trim_start_matches('-').to_owned();
    }
    script_real(word.parse::<f64>().expect("a real word").abs())
}

/// A real as a script writes it: its shortest digits, with a point or an exponent.
fn script_real(value: f64) -> String {
    let (digits, exponent) = {
        let text = format!("{value:e}");
        let (mantissa, exponent) = text.split_once('e').expect("an exponent");
        (
            mantissa.replace('.', ""),
            exponent.parse::<i32>().expect("an exponent"),
        )
    };
    if !(-4..17).contains(&exponent) {
        let (first, rest) = digits.split_at(1);
        let point = if rest.is_empty() {
            String::new()
        } else {
            format!(".{rest}")
        };
        let sign = if exponent < 0 { '-' } else { '+' };
        return format!("{first}{point}e{sign}{:02}", exponent.abs());
    }
    if exponent < 0 {
        return format!("0.{}{digits}", "0".repeat((-exponent - 1) as usize));
    }
    let whole = exponent as usize + 1;
    if digits.len() <= whole {
        return format!("{digits}{}.0", "0".repeat(whole - digits.len()));
    }
    format!("{}.{}", &digits[..whole], &digits[whole..])
}

/// `dtyp name`'s answer. Example: `type_line(&shape)`.
fn type_line(shape: &Shape) -> String {
    let kind = match shape.shape_type().expect("a shape's type") {
        ShapeEnum::Compound => "COMPOUND",
        ShapeEnum::CompSolid => "COMPSOLID",
        ShapeEnum::Solid => "SOLID",
        ShapeEnum::Shell => "SHELL",
        ShapeEnum::Face => "FACE",
        ShapeEnum::Wire => "WIRE",
        ShapeEnum::Edge => "EDGE",
        ShapeEnum::Vertex => "VERTEX",
        ShapeEnum::Shape => "SHAPE",
    };
    let orientation = match shape.orientation() {
        Orientation::Forward => "FORWARD",
        Orientation::Reversed => "REVERSED",
        Orientation::Internal => "INTERNAL",
        Orientation::External => "EXTERNAL",
    };
    let mut text = format!("shape {kind} {orientation}");
    for (name, set) in [
        ("Free", shape.free().expect("a flag")),
        ("Modified", shape.modified().expect("a flag")),
        ("Orientable", shape.orientable().expect("a flag")),
        ("Closed", shape.closed().expect("a flag")),
        ("Infinite", shape.infinite().expect("a flag")),
        ("Convex", shape.convex().expect("a flag")),
    ] {
        if set {
            let _ = write!(text, " {name}");
        }
    }
    text
}

fn is_orientable_closed(shape: &Shape) -> bool {
    shape.orientable().expect("a flag") && shape.closed().expect("a flag")
}

impl End<'_> {
    fn run(mut self) {
        println!("Test includes comparing for {MODE}");
        match self.row.added {
            Added::AssemblyColorsLayers => self.add_assembly_colors_layers(),
            Added::ColorsLayers => self.add_colors_layers(),
        }
        let first = self.first_facts();
        self.write_and_read_back();
        let second = self.second_facts();
        self.compare(&first, &second);
        self.show_second();
        match self.error_code {
            2 => println!("Error : errors occur during test"),
            1 => println!("WARNING OCCURS DURING TEST"),
            _ => {}
        }
        println!("TEST COMPLETED");
    }

    fn error(&mut self, message: &str) {
        println!("{message}");
        self.error_code = 2;
    }

    fn warn(&mut self, message: &str) {
        println!("{message}");
        if self.error_code == 0 {
            self.error_code = 1;
        }
    }

    fn require(&mut self, holds: bool, message: &str) {
        if !holds {
            self.error(message);
        }
    }

    fn require_shape_color(&mut self, shape: &str, kind: &str, expected: &str, message: &str) {
        let entry = self.session.find_shape("D_First", shape);
        let found = self.session.shape_color("D_First", &entry, kind);
        self.require(found == expected, message);
    }

    /// `box name dx dy dz` at the origin, or `box name x y z dx dy dz` with a corner.
    fn add_box(&mut self, name: &str, corner: [f64; 3], size: [f64; 3]) {
        let origin = martensite_tkmath::gp::Point3::new(corner[0], corner[1], corner[2]);
        let shape = BRepPrimAPIMakeBox::at_point(origin, size[0], size[1], size[2])
            .expect("a box")
            .shape()
            .expect("a box")
            .clone();
        self.session.raster.bind(name, &shape);
    }

    fn add_assembly_colors_layers(&mut self) {
        if !self.session.documents.contains_key("D_First") {
            println!("Creating new doc");
            println!("XNewDoc D_First ");
            self.session.new_document("D_First");
        }
        self.add_box("b1", [0.0; 3], [10.0; 3]);
        self.add_box("b2", [10.0; 3], [20.0; 3]);
        let builder = martensite_tkbrep::brep::BRepBuilder::default();
        let mut compound = martensite_tkbrep::topods::Compound::default();
        builder.make_compound(&mut compound);
        for name in ["b1", "b2"] {
            builder
                .add(&mut compound, &self.session.bound_shape(name))
                .expect("a compound");
        }
        println!("compound b1 b2 c ");
        self.session.raster.bind("c", &compound.into_shape());
        self.session.add_shape("D_First", "c");
        self.add_box("b3", [30.0; 3], [30.0; 3]);
        self.session.add_shape("D_First", "b3");
        self.assembly_colors();
        self.assembly_layers();
    }

    fn assembly_colors(&mut self) {
        let doc = "D_First";
        self.session.add_color(doc, RED);
        self.session.add_color(doc, GREEN);
        let found = (
            self.session.find_color(doc, RED),
            self.session.find_color(doc, RED),
        );
        self.require(
            !found.0.is_empty() && !found.1.is_empty(),
            "Error : Color was not added in the document or was not found after adding",
        );
        let count = self.session.all_colors(doc).len();
        self.require(
            count == 2,
            "Error : XGetAllColors function returns an invalid value",
        );
        for (shape, surface) in [("b1", RED), ("b2", GREEN), ("b3", BLUE), ("c", YELLOW)] {
            self.session.set_color(doc, shape, surface, "s");
        }
        for (shape, curves) in [("b1", CYAN), ("b2", MAGENTA), ("b3", WHITE), ("c", BLACK)] {
            self.session.set_color(doc, shape, curves, "c");
        }
        let count = self.session.all_colors(doc).len();
        self.require(
            count == 8,
            "Error : XGetAllColors function returns an invalid value after assigning colors to shapes in the document",
        );
        for (shape, kind, name, message) in [
            (
                "b1",
                "s",
                "RED",
                "Error : Shape b1 has an invalid surface color ",
            ),
            (
                "b2",
                "s",
                "GREEN",
                "Error : Shape b2 has an invalid surface color ",
            ),
            (
                "b3",
                "s",
                "BLUE",
                "Error : Shape b3 has an invalid surface color ",
            ),
            (
                "c",
                "s",
                "YELLOW",
                "Error : Shape c has an invalid surface color ",
            ),
            (
                "b1",
                "c",
                "CYAN",
                "Error : Shape b1 has an invalid surface color ",
            ),
            (
                "b2",
                "c",
                "MAGENTA",
                "Error : Shape b2 has an invalid surface color ",
            ),
            (
                "b3",
                "c",
                "WHITE",
                "Error : Shape b3 has an invalid surface color ",
            ),
            (
                "c",
                "c",
                "BLACK",
                "Error : Shape c has an invalid surface color ",
            ),
        ] {
            self.require_shape_color(shape, kind, name, message);
        }
        let entry = self.session.find_color(doc, RED);
        self.session.remove_color(doc, &entry);
        let entry = self.session.find_color(doc, MAGENTA);
        self.session.remove_color(doc, &entry);
        let count = self.session.all_colors(doc).len();
        self.require(
            count == 6,
            "Error : XGetAllColors function returns an invalid value after removing two colors from the document",
        );
        self.require_shape_color(
            "b1",
            "s",
            "",
            "Error : Color of b1 surface was removed from the document however XGetShapeColor function returns value ",
        );
        self.require_shape_color(
            "b2",
            "c",
            "",
            "Error : Color of b2 curves was removed from the document however XGetShapeColor function returns value ",
        );
    }

    fn three_layers(&mut self, added: &str, removing: &str, wrong_labels: &str) {
        let doc = "D_First";
        for name in ["L1", "L2", "L3"] {
            self.session.add_layer(doc, name);
        }
        let count = self.session.all_layers(doc).len();
        self.require(count == 3, added);
        let count = self.session.layer_labels(doc).len();
        self.require(count == 3, wrong_labels);
        let entries = ["L1", "L2", "L3"].map(|name| self.session.find_layer(doc, name));
        self.require(entries.iter().all(|entry| !entry.is_empty()), removing);
        for name in ["L1", "L2", "L3"] {
            let entry = self.session.find_layer(doc, name);
            let found = self.session.one_layer(doc, &entry);
            self.require(
                found == format!("\"{name}\""),
                &format!("Error : XGetOneLayer function works incorrectly for {name} layer"),
            );
        }
        self.session.add_layer(doc, "LtoRemove");
        let count = self.session.all_layers(doc).len();
        self.require(
            count == 4,
            "Error : Error occurs when adding a layer in the document ",
        );
        self.session.remove_layer(doc, "LtoRemove");
        let count = self.session.all_layers(doc).len();
        self.require(
            count == 3,
            "Error : Error occurs when removing a layer from the document ",
        );
    }

    fn assembly_layers(&mut self) {
        let doc = "D_First";
        self.three_layers(
            "Error : Error occurs during adding layers in document ",
            "Error : Layers were not added in the document correctly ",
            "Error : XGetLayerLabels function works incorrectly  ",
        );
        for (shape, layer) in [("b1", "L1"), ("b2", "L2"), ("b3", "L3"), ("c", "L3")] {
            self.session.set_layer(doc, shape, layer);
        }
        for (shape, layer, message) in [
            ("b1", "L1", "Error : Shape b1 is not on layer L1"),
            ("b2", "L2", "Error : Shape b2 is not on layer L2"),
            ("b3", "L3", "Error : Shape b3 is not on layer L3"),
            ("c", "L3", "Error : Compound c is not on layer L3"),
        ] {
            let found = self.session.layers_of(doc, shape);
            self.require(found == format!("\"{layer}\" "), message);
        }
        let entry = self.session.find_color(doc, YELLOW);
        self.session.remove_color(doc, &entry);
        let entry = self.session.find_color(doc, BLACK);
        self.session.remove_color(doc, &entry);
        self.session.unset_layer(doc, "c", "L3");
    }

    fn add_colors_layers(&mut self) {
        let doc = "D_First";
        self.session.add_color(doc, RED);
        self.session.add_color(doc, GREEN);
        let found = (
            self.session.find_color(doc, RED),
            self.session.find_color(doc, RED),
        );
        self.require(
            !found.0.is_empty() && !found.1.is_empty(),
            "Error : Color was not added in the document or was not found after adding",
        );
        let count = self.session.all_colors(doc).len();
        self.require(
            count == 2,
            "Error : XGetAllColors function returns an invalid value",
        );
        let list = self.session.top_level_shapes(doc);
        for entry in &list {
            self.session.set_color(doc, entry, RED, "s");
            self.session.set_color(doc, entry, CYAN, "c");
        }
        for entry in &list {
            let found = self.session.shape_color(doc, entry, "s");
            self.require(
                found == "RED",
                &format!("Error : Shape  located on label {entry} has an invalid surface color "),
            );
            let found = self.session.shape_color(doc, entry, "c");
            self.require(
                found == "CYAN",
                &format!("Error : Shape  located on label {entry} has an invalid curves color "),
            );
        }
        let entry = self.session.find_color(doc, CYAN);
        self.session.remove_color(doc, &entry);
        let count = self.session.all_colors(doc).len();
        self.require(
            count == 2,
            "Error : XGetAllColors function returns an invalid value after removing colors from the document",
        );
        for entry in &list {
            let found = self.session.shape_color(doc, entry, "c");
            self.require(
                found.is_empty(),
                "Error : Curves color was removed from the document however XGetShapeColor function returns value ",
            );
        }
        self.three_layers(
            "Error : Error occurs when adding layers in the document  ",
            "Error : Layers were not added in the document correctly ",
            "Error : XGetLayerLabels function works incorrectly  ",
        );
        self.layers_on_shapes(&list);
        self.drop_unused_on_assemblies();
    }

    /// `[expr $i / 2 == int ($i/2)]` holds for every `i`: the script's own integer division, so all
    /// shapes go on `L1`. Its other branch never runs.
    fn layers_on_shapes(&mut self, list: &[String]) {
        let doc = "D_First";
        for entry in list {
            self.session.set_layer(doc, entry, "L1");
        }
        for entry in list {
            let found = self.session.layers_of(doc, entry);
            self.require(
                found == "\"L1\" ",
                &format!("Error : Shape located on label {entry}  is not on layer \"L1\" "),
            );
        }
        self.session.remove_layer(doc, "L2");
        self.session.remove_layer(doc, "L3");
        let entry = self.session.find_color(doc, GREEN);
        self.session.remove_color(doc, &entry);
    }

    fn drop_unused_on_assemblies(&mut self) {
        let doc = "D_First";
        let list = self.session.top_level_shapes(doc);
        for entry in &list {
            let info = self.session.label_info(doc, entry);
            if info.contains("Assembly") || info.contains("This Shape is not used") {
                self.session.unset_layer(doc, entry, "L1");
                self.session.unset_color(doc, entry, "c");
                self.session.unset_color(doc, entry, "s");
            }
        }
        let list = self.session.top_level_shapes(doc);
        let (mut color_used, mut layer_used) = (false, false);
        for entry in &list {
            color_used |= !self.session.shape_color(doc, entry, "c").is_empty();
            color_used |= !self.session.shape_color(doc, entry, "s").is_empty();
            layer_used |= !self.session.layers_of(doc, entry).is_empty();
        }
        if !color_used {
            let entry = self.session.find_color(doc, RED);
            self.session.remove_color(doc, &entry);
        }
        if !layer_used {
            self.session.remove_layer(doc, "L1");
        }
    }

    fn get_one_shape(&mut self, name: &str, doc: &str) {
        println!("XGetOneShape {name} {doc} ");
        if self.session.get_one_shape(name, doc).is_err() {
            script_error(&format!("XGetOneShape {name} {doc}"));
        }
    }

    fn faces_of(&mut self, name: &str) -> usize {
        let shape = self.session.bound_shape(name);
        if shape.shape_type().expect("a shape's type") == ShapeEnum::Face {
            return 1;
        }
        self.session.explode(name, "F")
    }

    /// `[regexp "FACE" [dtyp name]]` looks at the whole answer: a compound holds no such word.
    fn first_facts(&mut self) -> DocumentFacts {
        let doc = "D_First";
        self.get_one_shape("res_First", doc);
        let faces = self.faces_of("res_First");
        self.get_one_shape("Shape_First", doc);
        let shape = self.session.bound_shape("Shape_First");
        println!("{}", type_line(&shape));
        self.is_comp = self.is_closed_body(&shape);
        let (volume, closed_shells) = if self.is_comp {
            self.first_volume()
        } else {
            println!("Volume of shape is equal 0");
            ("0".to_owned(), 0)
        };
        self.document_lists(doc, faces, volume, closed_shells)
    }

    fn is_closed_body(&mut self, shape: &Shape) -> bool {
        let kind = shape.shape_type().expect("a shape's type");
        if kind == ShapeEnum::Solid {
            return true;
        }
        if kind == ShapeEnum::Shell && is_orientable_closed(shape) {
            return true;
        }
        if kind != ShapeEnum::Compound {
            return false;
        }
        let mut closed = self.session.explode("Shape_First", "So") != 0;
        let shells = self.session.explode("Shape_First", "Sh");
        for number in 1..=shells {
            let shell = self.session.bound_shape(&format!("Shape_First_{number}"));
            closed |= is_orientable_closed(&shell);
        }
        closed
    }

    fn sew(&mut self, result: &str, tolerance: f64, tolerance_word: &str, name: &str) -> bool {
        println!("sewing {result} {tolerance_word} {name} ");
        let shape = self.session.bound_shape(name);
        let attempt = || -> Result<Shape, martensite_tkernel::standard::Failure> {
            let mut sewer = BRepBuilderAPISewing::default();
            sewer.set_tolerance(tolerance);
            sewer.set_min_tolerance(tolerance);
            sewer.set_max_tolerance(tolerance * 1000.0);
            sewer.add(&shape)?;
            sewer.perform(&ProgressRange::default())?;
            let mut dump = Vec::new();
            sewer.dump(&mut dump).expect("a memory stream");
            print!("{}", String::from_utf8_lossy(&dump));
            Ok(sewer.sewed_shape().clone())
        };
        match attempt() {
            Ok(sewn) => {
                self.session.raster.bind(result, &sewn);
                true
            }
            Err(_) => false,
        }
    }

    /// `catch { fbclose name sew close }` and the script's test of no vertex in either compound.
    fn no_free_bound(
        &mut self,
        name: &str,
        sew: f64,
        close: f64,
        sew_word: &str,
        close_word: &str,
    ) -> bool {
        println!("fbclose {name} {sew_word} {close_word} ");
        let shape = self.session.bound_shape(name);
        let bounds = ShapeFixFreeBounds::forecast(&shape, sew, close, false, false)
            .unwrap_or_else(|failure| panic!("fbclose {name}: {failure:?}"));
        println!("{name}_c - closed wires");
        println!("{name}_o - open wires");
        self.session.raster.bind(
            &format!("{name}_c"),
            &bounds.closed_wires().clone().into_shape(),
        );
        self.session.raster.bind(
            &format!("{name}_o"),
            &bounds.open_wires().clone().into_shape(),
        );
        self.session.explode(&format!("{name}_o"), "V") == 0
            && self.session.explode(&format!("{name}_c"), "V") == 0
    }

    fn first_volume(&mut self) -> (String, i64) {
        self.get_one_shape("Shape_First", "D_First");
        self.sew("r", 0.00001, "0.00001", "Shape_First");
        let mut closed = 0;
        let sewn = self.session.bound_shape("r");
        if sewn.shape_type().expect("a shape's type") == ShapeEnum::Shell {
            if self.no_free_bound("r", 0.001, 0.00001, "0.001", "0.00001") {
                println!("nbClosedShells_First = 1");
                closed = 1;
            }
        } else {
            // `$i_o` is no variable of the script: its `catch` swallows the error and counts nothing.
            let shells = self.session.explode("r", "Sh");
            for number in 1..=shells {
                self.no_free_bound(&format!("r_{number}"), 0.001, 0.00001, "0.001", "0.00001");
            }
        }
        self.first_closed_shells = closed;
        println!("XNewDoc T_First ");
        self.session.new_document("T_First");
        self.session.add_shape("T_First", "r");
        self.session.set_props("T_First", "0:1:1:1", 1.0, "1");
        (
            script_abs(&self.session.volume_word("T_First", "0:1:1:1")),
            closed,
        )
    }

    fn document_lists(
        &mut self,
        doc: &str,
        faces: usize,
        volume: String,
        closed_shells: i64,
    ) -> DocumentFacts {
        let mut colors = self.session.all_colors(doc);
        colors.sort();
        self.session.top_level_shapes(doc);
        let layers = self.session.all_layers(doc);
        let layer_labels = self.session.layer_labels(doc);
        let shape_labels = self.session.top_level_shapes(doc);
        DocumentFacts {
            faces,
            volume,
            closed_shells,
            colors,
            shape_labels,
            layers,
            layer_labels,
        }
    }

    fn write_and_read_back(&mut self) {
        let name = self.row.source.rsplit('/').next().expect("a case name");
        let file = image_folder(self.row.source).join(format!("{name}_D_First.igs"));
        let file = file.to_string_lossy().into_owned();
        println!("WriteIges D_First {file} ");
        if self.session.write_iges("D_First", &file).is_err() {
            script_error("WriteIges D_First");
        }
        keep_written_file(&file);
        println!("ReadIges D_Second {file} ");
        if self.session.read_iges("D_Second", &file).is_err() {
            script_error("ReadIges D_Second");
        }
        let _ = std::fs::remove_file(&file);
    }

    fn second_facts(&mut self) -> DocumentFacts {
        let doc = "D_Second";
        self.get_one_shape("res_Second", doc);
        let faces = self.faces_of("res_Second");
        let (volume, closed_shells) = if self.is_comp {
            self.second_volume()
        } else {
            println!("Volume of shape is equal 0");
            ("0".to_owned(), 0)
        };
        self.document_lists(doc, faces, volume, closed_shells)
    }

    /// Sews with a tolerance that grows tenfold until the second document holds as many closed
    /// shells as the first: five tries.
    fn second_volume(&mut self) -> (String, i64) {
        self.get_one_shape("Shape_Second", "D_Second");
        let wanted = self.first_closed_shells;
        let mut closed = 0;
        let mut performed = false;
        let mut tolerance: f64 = 0.00001;
        let mut word = "0.00001".to_owned();
        for _ in 0..5 {
            println!("PERFORM SEWING. TOLERANCE = {tolerance:.6}");
            self.sew("res", tolerance, &word, "Shape_Second");
            closed = 0;
            let sewn = self.session.bound_shape("res");
            if sewn.shape_type().expect("a shape's type") == ShapeEnum::Shell {
                if self.no_free_bound("res", 0.001, tolerance, "0.001", &word) {
                    closed = 1;
                }
            } else {
                // As in the first document: `$i_o` is no variable, so nothing is counted.
                let shells = self.session.explode("res", "Sh");
                for number in 1..=shells {
                    self.no_free_bound(
                        &format!("res_{number}"),
                        0.001,
                        0.000001,
                        "0.001",
                        "0.000001",
                    );
                }
            }
            if closed >= wanted {
                performed = true;
                break;
            }
            tolerance *= 10.0;
            word = script_real(tolerance);
        }
        println!("XNewDoc T_Second ");
        self.session.new_document("T_Second");
        self.session.add_shape("T_Second", "res");
        self.session.set_props("T_Second", "0:1:1:1", 1.0, "1");
        let volume = script_abs(&self.session.volume_word("T_Second", "0:1:1:1"));
        if !performed {
            println!("Error : SEWING IS NOT PERFORMED PROPERLY");
        }
        (volume, closed)
    }

    fn compare(&mut self, first: &DocumentFacts, second: &DocumentFacts) {
        println!("====================RESULTS OF COMPARISON====================");
        println!("================FACES===================");
        println!("Faces First: {}", first.faces);
        println!("Faces Second: {}", second.faces);
        self.require(
            first.faces == second.faces,
            "Error : The number of faces is wrong",
        );
        self.compare_volume(first, second);
        println!("==============COLORS=====================");
        println!("DocAllColors First: {}", script_list(&first.colors));
        println!("DocAllColors Second: {}", script_list(&second.colors));
        if second.colors.is_empty() && !first.colors.is_empty() {
            self.warn(
                "Warning : There are no colors in D_Second although they are present in D_First",
            );
        }
        self.compare_shape_labels(first, second, false);
        println!("================LAYERS===================");
        println!("DocLayers First: {} ", script_list(&first.layers));
        println!("DocLayers Second: {} ", script_list(&second.layers));
        if first.layers != second.layers {
            self.warn("Warning : Compared documents have a different set of LAYERS names");
        }
        println!(
            "DocLayerLabels First: {} ",
            script_list(&first.layer_labels)
        );
        println!(
            "DocLayerLabels Second: {} ",
            script_list(&second.layer_labels)
        );
        if first.layer_labels != second.layer_labels {
            self.warn("Warning : Compared documents have a different set of LAYERS labels");
        }
        self.compare_shape_labels(first, second, true);
    }

    fn compare_volume(&mut self, first: &DocumentFacts, second: &DocumentFacts) {
        println!("===============VOLUME====================");
        println!("DocVolume First: {}", first.volume);
        println!("DocVolume Second: {}", second.volume);
        if first.volume != second.volume {
            let (a, b): (f64, f64) = (
                first.volume.parse().expect("a volume"),
                second.volume.parse().expect("a volume"),
            );
            let largest = a.max(b);
            let difference = if largest > 0.0 {
                (a - b).abs() / largest * 100.0
            } else {
                0.0
            };
            if difference > 5.0 {
                self.error(&format!(
                    "Error : The difference between volumes is {difference:.6}%"
                ));
            } else {
                println!("The difference between volumes is {difference:.6}% - OK");
            }
        }
        println!("nbClosedShells First: {}", first.closed_shells);
        println!("nbClosedShells Second: {}", second.closed_shells);
        self.require(
            first.closed_shells == second.closed_shells,
            "Error : nbClosedShells is wrong",
        );
    }

    /// The layers' copy of this check sets the code to 1 outright, the colours' only from 0.
    fn compare_shape_labels(
        &mut self,
        first: &DocumentFacts,
        second: &DocumentFacts,
        of_layers: bool,
    ) {
        println!(
            "DocShapeLabels First: {} ",
            script_list(&first.shape_labels)
        );
        println!(
            "DocShapeLabels Second: {} ",
            script_list(&second.shape_labels)
        );
        if first.shape_labels == second.shape_labels {
            return;
        }
        if of_layers {
            println!("Warning : DocShapeLabels is wrong");
            self.error_code = 1;
        } else {
            self.warn("Warning : Compared documents have a different number of toplevel shapes ");
        }
    }

    fn show_second(&mut self) {
        self.get_one_shape("result", "D_Second");
        checkview(&mut self.session.raster, &CheckView::of("result"));
    }
}
