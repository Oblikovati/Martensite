//! What the analysis commands of the end scripts print: `XStat`, `statshape`, `tolerance`, on real API.
//! Each writes the text to the stream and returns the numbers the case's `regexp`s read.
use crate::support::{session::Session, structural_dump::structural_check};
use crate::xs_session::XsSession;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::{quantity::Color, standard::Handle, tcollection::ExtendedString};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tklcaf::{
    tdata_std::{TDataStdGenericExtString, TDataStdName},
    tdf::TdfLabel,
    tdoc_std::TDocStdDocument,
};
use martensite_tkshhealing::shape_analysis::{
    ShapeAnalysisShapeContents, ShapeAnalysisShapeTolerance,
};
use martensite_tkxcaf::xcaf_doc::{
    XCAFDocArea, XCAFDocCentroid, XCAFDocColorTool, XCAFDocColorType, XCAFDocDocumentTool,
    XCAFDocLayerTool, XCAFDocVisMaterialTool, XCAFDocVolume,
};

/// `XStat`'s numbers: labels per level, then the labels with each kind of attribute.
#[derive(Clone, Debug, Default)]
pub struct DocumentStatistics {
    pub printed: bool,
    pub levels: [i64; 21],
    pub total: i64,
    pub names: i64,
    pub with_color: i64,
    pub with_layer: i64,
    pub centroids: i64,
    pub volumes: i64,
    pub areas: i64,
    pub colors: i64,
    pub color_names: String,
    pub layers: i64,
    pub layer_names: String,
}

struct StatisticsWalk {
    tools: (Handle<XCAFDocColorTool>, Handle<XCAFDocLayerTool>),
    with_material: i64,
    statistics: DocumentStatistics,
}

impl StatisticsWalk {
    /// `StatAssembly`: one label and its children, a level down.
    fn visit(&mut self, label: &TdfLabel, level: usize) {
        self.count_properties(label);
        self.count_color(label);
        self.count_layer_and_material(label);
        self.statistics.levels[level] += 1;
        if !label.has_child() {
            return;
        }
        for tag in 1..=label.nb_children().expect("children of a label") {
            let child = label.find_child(tag, true).expect("a child label");
            self.visit(&child, level + 1);
        }
    }

    fn count_properties(&mut self, label: &TdfLabel) {
        let stats = &mut self.statistics;
        if label.is_attribute(&TDataStdName::get_id()).expect("a name") {
            stats.names += 1;
        }
        if label
            .is_attribute(&XCAFDocCentroid::get_id())
            .expect("a centroid")
        {
            stats.centroids += 1;
        }
        if XCAFDocVolume::get_of_label(label, &mut 0.0).expect("a volume") {
            stats.volumes += 1;
        }
        if XCAFDocArea::get_of_label(label, &mut 0.0).expect("an area") {
            stats.areas += 1;
        }
    }

    fn count_color(&mut self, label: &TdfLabel) {
        let tool = self.tools.0.get().expect("a color tool");
        let colored = [
            XCAFDocColorType::ColorGen,
            XCAFDocColorType::ColorSurf,
            XCAFDocColorType::ColorCurv,
        ]
        .into_iter()
        .any(|kind| {
            XCAFDocColorTool::get_label_color(label, kind)
                .expect("a color")
                .is_some()
        });
        if colored || tool.is_color_by_layer(label).expect("a layer color") {
            self.statistics.with_color += 1;
        }
    }

    fn count_layer_and_material(&mut self, label: &TdfLabel) {
        if !XCAFDocLayerTool::get_layers(label)
            .expect("layers")
            .is_empty()
        {
            self.statistics.with_layer += 1;
        }
        if XCAFDocVisMaterialTool::get_shape_material_label(label)
            .expect("a material")
            .is_some()
        {
            self.with_material += 1;
        }
    }
}

fn utf8(text: &ExtendedString) -> String {
    String::from_utf8_lossy(martensite_tkernel::tcollection::AsciiString::from(text).as_bytes())
        .into_owned()
}

/// The text of `XStat` from the second paragraph on, and the colours' and layers' lines.
fn print_statistics(
    main: &TdfLabel,
    walk: &StatisticsWalk,
    text: &mut String,
) -> DocumentStatistics {
    let mut stats = walk.statistics.clone();
    *text += &format!("\nStatistis of shapes in the document:\n");
    for (level, count) in stats.levels.iter().enumerate() {
        if *count == 0 {
            break;
        }
        *text += &format!("level N {level} : {count}\n");
        stats.total += count;
    }
    *text += &format!(
        "Total number of labels for shapes in the document = {}\n",
        stats.total
    );
    *text += &format!("Number of labels with name = {}\n", stats.names);
    *text += &format!("Number of labels with color link = {}\n", stats.with_color);
    *text += &format!("Number of labels with layer link = {}\n", stats.with_layer);
    *text += &format!(
        "Number of labels with vis material link = {}\n",
        walk.with_material
    );
    *text += &format!("\nStatistis of Props in the document:\n");
    *text += &format!("Number of Centroid Props = {}\n", stats.centroids);
    *text += &format!("Number of Volume Props = {}\n", stats.volumes);
    *text += &format!("Number of Area Props = {}\n", stats.areas);
    print_colors(main, &mut stats, text);
    print_layers(main, &mut stats, text);
    print_vis_materials(main, text);
    stats
}

fn print_colors(main: &TdfLabel, stats: &mut DocumentStatistics, text: &mut String) {
    let tool = XCAFDocDocumentTool::color_tool(main).expect("a color tool");
    let labels = tool.get_colors().expect("colors");
    stats.colors = labels.len() as i64;
    *text += &format!("\nNumber of colors = {}\n", labels.len());
    if labels.is_empty() {
        return;
    }
    for label in &labels {
        let color = XCAFDocColorTool::get_color(label)
            .expect("a color")
            .expect("a colour label");
        stats.color_names += Color::string_name(color.name());
        stats.color_names.push(' ');
    }
    *text += &format!("{}\n", stats.color_names);
}

fn print_layers(main: &TdfLabel, stats: &mut DocumentStatistics, text: &mut String) {
    let tool = XCAFDocDocumentTool::layer_tool(main).expect("a layer tool");
    let labels = tool.get_layer_labels().expect("layers");
    stats.layers = labels.len() as i64;
    *text += &format!("\nNumber of layers = {}\n", labels.len());
    if labels.is_empty() {
        return;
    }
    for label in &labels {
        let name = tool
            .get()
            .expect("a layer tool")
            .get_layer(label)
            .expect("a layer");
        stats.layer_names += &format!("\"{}\" ", name.as_ref().map(utf8).unwrap_or_default());
    }
    *text += &format!("{}\n", stats.layer_names);
}

fn print_vis_materials(main: &TdfLabel, text: &mut String) {
    let tool = XCAFDocDocumentTool::vis_material_tool(main).expect("a material tool");
    let labels = tool.get_materials().expect("materials");
    *text += &format!("\nNumber of vis materials = {}\n", labels.len());
    if !labels.is_empty() {
        let mut line = String::new();
        for label in &labels {
            let mut name: Handle<TDataStdName> = Handle::default();
            if label
                .find_attribute_as(&TDataStdName::get_id(), &mut name)
                .expect("a name")
            {
                line += &format!("\"{}\" ", utf8(&name.get().expect("a name").get()));
            }
        }
        *text += &format!("{line}\n");
    }
    text.push('\n');
}

/// `XStat name`: what it prints, and the numbers; nothing for a document without a shape label.
/// Example: `statistics_of(&document, "D_First")`.
pub fn statistics_of(document: &Handle<TDocStdDocument>) -> DocumentStatistics {
    let (statistics, text) = statistics_with_text(document);
    print!("{text}");
    statistics
}

/// `XStat name`'s numbers and its text, for a script that reads it back.
/// Example: `statistics_with_text(&document).1`.
pub fn statistics_with_text(document: &Handle<TDocStdDocument>) -> (DocumentStatistics, String) {
    let main = document
        .get()
        .expect("a document")
        .main()
        .expect("a main label");
    let shape_tool = XCAFDocDocumentTool::shape_tool(&main).expect("a shape tool");
    let labels = shape_tool.get_shapes().expect("shapes");
    if labels.is_empty() {
        return (DocumentStatistics::default(), String::new());
    }
    let color_tool = XCAFDocDocumentTool::color_tool(&main).expect("a color tool");
    let layer_tool = XCAFDocDocumentTool::layer_tool(&main).expect("a layer tool");
    let mut walk = StatisticsWalk {
        tools: (color_tool, layer_tool),
        with_material: 0,
        statistics: DocumentStatistics {
            printed: true,
            ..DocumentStatistics::default()
        },
    };
    for label in &labels {
        walk.visit(label, 0);
    }
    let mut text = String::new();
    let statistics = print_statistics(&main, &walk, &mut text);
    (statistics, text)
}

/// `tolerance name`: writes its four lines; `(MAX, AVG)` of the shape as a whole.
/// Example: `whole_tolerance(&shape)`.
pub fn whole_tolerance(shape: &Shape) -> (f64, f64) {
    let rows = [
        ("Tolerance", ShapeEnum::Shape),
        ("FACE   ", ShapeEnum::Face),
        ("EDGE   ", ShapeEnum::Edge),
        ("VERTEX ", ShapeEnum::Vertex),
    ];
    let mut whole = (0.0, 0.0);
    for (title, kind) in rows {
        let mut tolerance = ShapeAnalysisShapeTolerance::new();
        tolerance.init_tolerance();
        tolerance
            .add_tolerance(shape, kind)
            .expect("a shape's tolerances");
        let [max, avg, min] = [1, 0, -1].map(|mode| tolerance.global_tolerance(mode));
        let separator = if kind == ShapeEnum::Shape { " " } else { " : " };
        println!(
            "{title}{separator}MAX={} AVG={} MIN={}",
            g17(max),
            g17(avg),
            g17(min)
        );
        if kind == ShapeEnum::Shape {
            whole = (max, avg);
        }
    }
    whole
}

/// A real as `the test report` writes one. Example: `g17(0.1) == "0.10000000000000001"`.
pub fn g17(value: f64) -> String {
    martensite_tkernel::storage::format_general(value, 17)
}

/// What `de/end` reads from `statshape`: its lines are the ones the regexps match.
#[derive(Clone, Copy, Debug, Default)]
pub struct ContentCounts {
    pub solids: i64,
    pub shells: i64,
    pub faces: i64,
    pub free_wires: i64,
}

/// `statshape name`: every line it writes, in its order. Example: `contents_of(&shape)`.
pub fn contents_of(shape: &Shape) -> ContentCounts {
    let mut analyzer = ShapeAnalysisShapeContents::new();
    analyzer.perform(shape).expect("a shape's contents");
    println!("Count    Item\n-----    ----");
    let a = &analyzer;
    let rows: [(i32, &str); 26] = [
        (a.nb_edges(), "\t EDGE (Oriented)"),
        (a.nb_shared_edges(), "\t EDGE (Shared)"),
        (a.nb_free_edges(), "\t EDGE (Free)"),
        (a.nb_faces(), "\t FACE"),
        (a.nb_free_faces(), "\t FACE (Free)"),
        (a.nb_free_wires(), "\t WIRE (Free)"),
        (a.nb_shells(), "\t SHELL"),
        (a.nb_solids(), "\t SOLID"),
        (a.nb_vertices(), "\t VERTEX (Oriented)"),
        (a.nb_shared_vertices(), "\t VERTEX (Shared)"),
        (a.nb_wires(), "\t WIRE"),
        (
            a.nb_faces_with_several_wires(),
            "\tFace with more than one wire",
        ),
        (a.nb_missing_pcurves(), "\tNo pcurve"),
        (a.nb_solids_with_voids(), "\tSOLID with voids"),
        (a.nb_wires_with_seam(), "\tWire(s) with one seam edge"),
        (
            a.nb_wires_with_several_seams(),
            "\tWire(s) with several seam edges",
        ),
        (a.nb_big_splines(), "\tbigspl : BSpline > 8192 poles"),
        (a.nb_bezier_surfaces(), "\tbezsur : BezierSurface"),
        (a.nb_bspline_surfaces(), "\tbspsur : BSplineSurface"),
        (a.nb_c0_curves(), "\tc0curv : Curve Only C0"),
        (a.nb_c0_surfaces(), "\tc0surf : Surface Only C0"),
        (a.nb_indirect_surfaces(), "\tindsur : Indirect Surface"),
        (a.nb_offset_curves(), "\tofcur  : Offset Curve(s)"),
        (a.nb_offset_surfaces(), "\tofsur  : Offset Surface"),
        (a.nb_trimmed_curves_2d(), "\ttrc2d  : Trimmed Curve2d"),
        (a.nb_trimmed_curves_3d(), "\ttrc3d  : Trimmed Curve3d"),
    ];
    for (count, label) in rows {
        if count > 0 {
            println!("{count}{label}");
        }
    }
    if a.nb_trimmed_surfaces() > 0 {
        println!(
            "{}\ttrimsu : RectangularTrimmedSurface",
            a.nb_trimmed_surfaces()
        );
    }
    // `([-0-9.+eE]+)\t+ +FACE` reads the first such line: the free faces when no other face is there.
    let faces = if a.nb_faces() > 0 {
        a.nb_faces()
    } else {
        a.nb_free_faces()
    };
    ContentCounts {
        solids: i64::from(a.nb_solids()),
        shells: i64::from(a.nb_shells()),
        faces: i64::from(faces),
        free_wires: i64::from(a.nb_free_wires()),
    }
}

/// What `de/end` reads from `checkshape name ff`: the items of each kind.
#[derive(Clone, Copy, Debug, Default)]
pub struct CheckCounts {
    pub wires: i64,
    pub faces: i64,
    pub shells: i64,
    pub solids: i64,
}

/// `checkshape name ff`: `check_counts` from `<KIND>\t+: +N`. Example: `check_counts(&shape, "res_1")`.
pub fn check_counts(shape: &Shape, name: &str) -> CheckCounts {
    let text = structural_check(shape, name, "ff")
        .unwrap_or_else(|failure| panic!("checkshape exception : {failure:?}"));
    println!("{text}");
    let items = |kind: &str| {
        text.lines()
            .find_map(|line| line.strip_prefix(kind)?.strip_prefix("\t: "))
            .and_then(|rest| rest.split_whitespace().next()?.parse().ok())
            .unwrap_or(0)
    };
    CheckCounts {
        wires: items("WIRE"),
        faces: items("FACE"),
        shells: items("SHELL"),
        solids: items("SOLID"),
    }
}

impl XsSession {
    /// The shape a script bound; a missing one is a script error.
    /// Example: `session.bound_shape("res_1")`.
    pub fn bound_shape(&self, name: &str) -> Shape {
        self.raster
            .shape(name)
            .cloned()
            .unwrap_or_else(|| panic!("{name}: expected a bound shape"))
    }
}
