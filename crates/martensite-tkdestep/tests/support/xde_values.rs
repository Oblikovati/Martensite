//! Prints every value of an XDE document as one record a line, in the format of the reference
//! program, so a document read here can be compared value by value with what that program read.
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::{self, Shape};
use martensite_tkernel::standard::Handle;
use martensite_tkernel::tcollection::{AsciiString, ExtendedString, HAsciiString};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tklcaf::tdata_std::{
    TDataStdGenericExtString, TDataStdName, TDataStdNamedData, TDataStdNamedMap,
};
use martensite_tklcaf::tdf::{TdfChildIterator, TdfLabel, TdfTool};
use martensite_tkmath::gp::{Axis2, Xyz};
use martensite_tkxcaf::xcaf_dim_tol_objects::{
    XCAFDimTolObjectsDimensionFormVariance, XCAFDimTolObjectsDimensionGrade,
};
use martensite_tkxcaf::xcaf_doc::{
    XCAFDocArea, XCAFDocCentroid, XCAFDocClippingPlaneTool, XCAFDocColorTool, XCAFDocColorType,
    XCAFDocDatum, XCAFDocDimTolTool, XCAFDocDimension, XCAFDocDocumentTool, XCAFDocGeomTolerance,
    XCAFDocLayerTool, XCAFDocMaterialTool, XCAFDocShapeTool, XCAFDocView, XCAFDocVolume,
};
use std::collections::BTreeMap;

fn ascii(value: &AsciiString) -> String {
    String::from_utf8_lossy(value.as_bytes()).into_owned()
}

fn entry(label: &TdfLabel) -> String {
    let mut text = AsciiString::from_bytes(b"");
    TdfTool::entry(label, &mut text);
    ascii(&text)
}

fn entries(labels: &[TdfLabel]) -> String {
    let inner: String = labels.iter().map(|l| entry(l) + " ").collect();
    format!("[{inner}]")
}

fn text(value: &Handle<HAsciiString>) -> String {
    value
        .get()
        .map_or("<null>".to_owned(), |s| format!("'{}'", ascii(&s.string())))
}

fn extended(value: &ExtendedString) -> String {
    ascii(&AsciiString::from_extended(value, None))
}

fn xyz(value: Xyz) -> String {
    format!("({} {} {})", value.x(), value.y(), value.z())
}

fn ax2(axis: &Axis2) -> String {
    format!(
        "{}{}{}",
        xyz(axis.location().xyz()),
        xyz(axis.direction().xyz()),
        xyz(axis.x_direction().xyz())
    )
}

/// The type, edge count, vertex count and the sum of the vertices of a shape.
fn shape_summary(shape: &Shape) -> String {
    if shape.is_null() {
        return "<noshape>".to_owned();
    }
    let mut vertices = Explorer::new(shape, ShapeEnum::Vertex, ShapeEnum::Shape).expect("explorer");
    let (mut nb, mut sum) = (0, Xyz::new(0.0, 0.0, 0.0));
    while vertices.more() {
        let point = BRepTool::pnt(&topods::vertex(vertices.current()).expect("vertex"));
        sum += point.expect("point").xyz();
        nb += 1;
        vertices.next().expect("next");
    }
    let mut edges = Explorer::new(shape, ShapeEnum::Edge, ShapeEnum::Shape).expect("explorer");
    let mut nb_edges = 0;
    while edges.more() {
        nb_edges += 1;
        edges.next().expect("next");
    }
    format!(
        "<{} e{nb_edges} v{nb} {}>",
        shape.shape_type().expect("type") as i32,
        xyz(sum)
    )
}

/// Each label under `root`, at all levels.
fn all_labels(root: &TdfLabel) -> Vec<TdfLabel> {
    let mut labels = Vec::new();
    let mut children = TdfChildIterator::new(root, true).expect("iterator");
    while children.more() {
        labels.push(children.value().clone());
        children.next();
    }
    labels
}

fn dump_names(root: &TdfLabel, out: &mut Vec<String>) {
    for label in all_labels(root) {
        let mut name: Handle<TDataStdName> = Handle::default();
        if label
            .find_attribute_as(&TDataStdName::get_id(), &mut name)
            .expect("find")
        {
            let value = extended(&name.get().expect("name").get())
                .replace('\n', "\\n")
                .replace('\r', "\\r");
            out.push(format!("NAME {} '{value}'", entry(&label)));
        }
    }
}

fn dump_named_data(main: &TdfLabel, out: &mut Vec<String>) {
    for label in all_labels(main) {
        let mut data: Handle<TDataStdNamedData> = Handle::default();
        if !label
            .find_attribute_as(&TDataStdNamedData::get_id(), &mut data)
            .expect("find")
        {
            continue;
        }
        let data = data.get().expect("data");
        let mut sorted = BTreeMap::new();
        if data.has_strings() {
            for (key, value) in data
                .get_strings_container()
                .get()
                .expect("map")
                .map()
                .items()
            {
                sorted.insert(extended(key), format!("S'{}'", extended(value)));
            }
        }
        if data.has_integers() {
            for (key, value) in data
                .get_integers_container()
                .get()
                .expect("map")
                .map()
                .items()
            {
                sorted.insert(extended(key), format!("I{value}"));
            }
        }
        if data.has_reals() {
            for (key, value) in data.get_reals_container().get().expect("map").map().items() {
                sorted.insert(extended(key), format!("R={value}"));
            }
        }
        for (key, value) in sorted {
            out.push(format!("DATA {} '{key}' {value}", entry(&label)));
        }
    }
}

fn dump_shape(label: &TdfLabel, out: &mut Vec<String>) {
    let mut shape = Shape::default();
    XCAFDocShapeTool::get_shape(label, &mut shape).expect("shape");
    let mut line = format!("SHAPE {} {}", entry(label), shape_summary(&shape));
    for kind in [
        XCAFDocColorType::ColorGen,
        XCAFDocColorType::ColorSurf,
        XCAFDocColorType::ColorCurv,
    ] {
        let mut color = TdfLabel::default();
        if XCAFDocColorTool::get_color_label(label, kind, &mut color).expect("color") {
            line += &format!(" c{}", entry(&color));
        } else {
            line += " c-";
        }
    }
    let layers = XCAFDocLayerTool::get_layer_labels_of(label).expect("layers");
    line += &format!(" l{}", entries(&layers));
    let mut area: Handle<XCAFDocArea> = Handle::default();
    if label
        .find_attribute_as(&XCAFDocArea::get_id(), &mut area)
        .expect("find")
    {
        line += &format!(" a={}", area.get().expect("area").get());
    }
    let mut volume: Handle<XCAFDocVolume> = Handle::default();
    if label
        .find_attribute_as(&XCAFDocVolume::get_id(), &mut volume)
        .expect("find")
    {
        line += &format!(" v={}", volume.get().expect("volume").get());
    }
    let mut centroid: Handle<XCAFDocCentroid> = Handle::default();
    if label
        .find_attribute_as(&XCAFDocCentroid::get_id(), &mut centroid)
        .expect("find")
    {
        line += &format!(" m={}", xyz(centroid.get().expect("centroid").get().xyz()));
    }
    out.push(line);
}

fn dump_colors_layers_materials(main: &TdfLabel, out: &mut Vec<String>) {
    for label in XCAFDocDocumentTool::color_tool(main)
        .expect("tool")
        .get_colors()
        .expect("colors")
    {
        let color = XCAFDocColorTool::get_color_rgba(&label)
            .expect("get")
            .expect("color");
        let rgb = color.rgb();
        out.push(format!(
            "COLOR {} {} {}",
            entry(&label),
            xyz(Xyz::new(rgb.red(), rgb.green(), rgb.blue())),
            f64::from(color.alpha())
        ));
    }
    let layer_tool = XCAFDocDocumentTool::layer_tool(main).expect("tool");
    for label in layer_tool.get_layer_labels().expect("layers") {
        let name = layer_tool.get_layer(&label).expect("get").expect("layer");
        let visible = layer_tool.is_visible(&label).expect("visible");
        out.push(format!(
            "LAYER {} '{}' {}",
            entry(&label),
            extended(&name),
            i32::from(visible)
        ));
    }
    let material_tool = XCAFDocDocumentTool::material_tool(main).expect("tool");
    for label in material_tool.get_material_labels().expect("materials") {
        let fields = XCAFDocMaterialTool::get_material(&label)
            .expect("get")
            .expect("material");
        out.push(format!(
            "MAT {} {} {} {} {} {}",
            entry(&label),
            text(&fields.name),
            text(&fields.description),
            fields.density,
            text(&fields.dens_name),
            text(&fields.dens_val_type)
        ));
    }
}

fn point_or_dash(has: bool, point: impl FnOnce() -> Xyz) -> String {
    if has { xyz(point()) } else { "-".to_owned() }
}

fn plane_or_dash(has: bool, axis: impl FnOnce() -> Axis2) -> String {
    if has { ax2(&axis()) } else { "-".to_owned() }
}

fn refs(label: &TdfLabel) -> (Vec<TdfLabel>, Vec<TdfLabel>) {
    let (mut firsts, mut seconds) = (Vec::new(), Vec::new());
    XCAFDocDimTolTool::get_ref_shape_label(label, &mut firsts, &mut seconds).expect("refs");
    (firsts, seconds)
}

fn dump_dimension(label: &TdfLabel, out: &mut Vec<String>) {
    let mut attribute: Handle<XCAFDocDimension> = Handle::default();
    if !label
        .find_attribute_as(&XCAFDocDimension::get_id(), &mut attribute)
        .expect("find")
    {
        out.push(format!("DIM {} <noattr>", entry(label)));
        return;
    }
    let object = attribute.get().expect("dim").get_object().expect("object");
    let object = object.get().expect("object");
    let values = object.get_values();
    let vals: String = values.get().map_or(String::new(), |array| {
        array.array1().iter().map(|v| format!("{v} ")).collect()
    });
    let (mut hole, mut form, mut grade) = (
        false,
        XCAFDimTolObjectsDimensionFormVariance::None,
        XCAFDimTolObjectsDimensionGrade::IT01,
    );
    let has_class = object.get_class_of_tolerance(&mut hole, &mut form, &mut grade);
    let (left, right) = object.get_nb_of_decimal_places();
    let modifiers: String = object
        .get_modifiers()
        .iter()
        .map(|m| format!("{} ", *m as i32))
        .collect();
    let mut direction = martensite_tkmath::gp::Direction3::new(1.0, 0.0, 0.0).expect("x");
    let has_dir = object.get_direction(&mut direction);
    out.push(format!(
        "DIM {} name={} type={} q={} aq={} vals=[{vals}] u={} l={} range={} pm={} class={},{},{},{} dec={left},{right} mods=[{modifiers}] dir={}{} path={}",
        entry(label),
        text(&object.get_semantic_name()),
        object.get_type() as i32,
        object.get_qualifier() as i32,
        object.get_angular_qualifier() as i32,
        object.get_upper_bound(),
        object.get_lower_bound(),
        i32::from(object.is_dim_with_range()),
        i32::from(object.is_dim_with_plus_minus_tolerance()),
        i32::from(has_class),
        i32::from(hole),
        form as i32,
        grade as i32,
        i32::from(has_dir),
        if has_dir { xyz(direction.xyz()) } else { String::new() },
        shape_summary(&object.get_path().into()),
    ));
    let mut pres = format!(
        "plane={} pt={} pt2={} text={} conn={},{}",
        plane_or_dash(object.has_plane(), || object.get_plane()),
        point_or_dash(object.has_point(), || object.get_point().xyz()),
        point_or_dash(object.has_point2(), || object.get_point2().xyz()),
        point_or_dash(object.has_text_point(), || object
            .get_point_text_attach()
            .xyz()),
        i32::from(object.is_point_connection()),
        i32::from(object.is_point_connection2()),
    );
    if object.is_point_connection() {
        pres += &format!(
            " {}{}",
            ax2(&object.get_connection_axis()),
            text(&object.get_connection_name())
        );
    }
    if object.is_point_connection2() {
        pres += &format!(
            " {}{}",
            ax2(&object.get_connection_axis2()),
            text(&object.get_connection_name2())
        );
    }
    pres += &format!(
        " pres={}{} desc={}",
        shape_summary(&object.get_presentation()),
        text(&object.get_presentation_name()),
        object.nb_descriptions()
    );
    for number in 0..object.nb_descriptions() {
        pres += &format!(
            " {}{}",
            text(&object.get_description(number)),
            text(&object.get_description_name(number))
        );
    }
    out.push(format!("DIMP {} {pres}", entry(label)));
    let (firsts, seconds) = refs(label);
    out.push(format!(
        "DIMR {} {} {}",
        entry(label),
        entries(&firsts),
        entries(&seconds)
    ));
}

fn dump_tolerance(label: &TdfLabel, out: &mut Vec<String>) {
    let mut attribute: Handle<XCAFDocGeomTolerance> = Handle::default();
    if !label
        .find_attribute_as(&XCAFDocGeomTolerance::get_id(), &mut attribute)
        .expect("find")
    {
        out.push(format!("TOL {} <noattr>", entry(label)));
        return;
    }
    let object = attribute.get().expect("tol").get_object().expect("object");
    let object = object.get().expect("object");
    let modifiers: String = object
        .get_modifiers()
        .iter()
        .map(|m| format!("{} ", *m as i32))
        .collect();
    out.push(format!(
        "TOL {} name={} type={} tv={} val={} mat={} zone={} zv={} mods=[{modifiers}] max={} axis={}",
        entry(label),
        text(&object.get_semantic_name()),
        object.get_type() as i32,
        object.get_type_of_value() as i32,
        object.get_value(),
        object.get_material_requirement_modifier() as i32,
        object.get_zone_modifier() as i32,
        object.get_value_of_zone_modifier(),
        object.get_max_value_modifier(),
        plane_or_dash(object.has_axis(), || object.get_axis()),
    ));
    out.push(format!(
        "TOLP {} plane={} pt={} text={} pres={}{} aff={}",
        entry(label),
        plane_or_dash(object.has_plane(), || object.get_plane()),
        point_or_dash(object.has_point(), || object.get_point().xyz()),
        point_or_dash(object.has_point_text(), || object
            .get_point_text_attach()
            .xyz()),
        shape_summary(&object.get_presentation()),
        text(&object.get_presentation_name()),
        object.get_affected_plane_type() as i32,
    ));
    let (firsts, seconds) = refs(label);
    let mut datums = Vec::new();
    XCAFDocDimTolTool::get_datum_with_object_of_toler_labels(label, &mut datums).expect("datums");
    out.push(format!(
        "TOLR {} {} {} {}",
        entry(label),
        entries(&firsts),
        entries(&seconds),
        entries(&datums)
    ));
}

fn dump_datum(label: &TdfLabel, out: &mut Vec<String>) {
    let mut attribute: Handle<XCAFDocDatum> = Handle::default();
    if !label
        .find_attribute_as(&XCAFDocDatum::get_id(), &mut attribute)
        .expect("find")
    {
        out.push(format!("DATUM {} <noattr>", entry(label)));
        return;
    }
    let object = attribute
        .get()
        .expect("datum")
        .get_object()
        .expect("object");
    let object = object.get().expect("object");
    let modifiers: String = object
        .get_modifiers()
        .iter()
        .map(|m| format!("{} ", *m as i32))
        .collect();
    let (with, with_value) = object.get_modifier_with_value();
    let mut line = format!(
        "DATUM {} sem={} name={} mods=[{modifiers}] with={},{with_value} pos={} target={} ttype={} params={}",
        entry(label),
        text(&object.get_semantic_name()),
        text(&object.get_name()),
        with as i32,
        object.get_position(),
        i32::from(object.is_datum_target()),
        object.get_datum_target_type() as i32,
        i32::from(object.has_datum_target_params()),
    );
    if object.has_datum_target_params() {
        line += &format!(
            " {} {} {}",
            ax2(&object.get_datum_target_axis()),
            object.get_datum_target_length(),
            object.get_datum_target_width()
        );
    }
    line += &format!(
        " num={} tshape={}",
        object.get_datum_target_number(),
        shape_summary(&object.get_datum_target())
    );
    out.push(line);
    out.push(format!(
        "DATUMP {} plane={} pt={} text={} pres={}{}",
        entry(label),
        plane_or_dash(object.has_plane(), || object.get_plane()),
        point_or_dash(object.has_point(), || object.get_point().xyz()),
        point_or_dash(object.has_point_text(), || object
            .get_point_text_attach()
            .xyz()),
        shape_summary(&object.get_presentation()),
        text(&object.get_presentation_name()),
    ));
    let (firsts, seconds) = refs(label);
    out.push(format!(
        "DATUMR {} {} {}",
        entry(label),
        entries(&firsts),
        entries(&seconds)
    ));
}

fn dump_views(main: &TdfLabel, out: &mut Vec<String>) {
    let tool = XCAFDocDocumentTool::view_tool(main).expect("tool");
    for label in tool.get_view_labels().expect("views") {
        let mut attribute: Handle<XCAFDocView> = Handle::default();
        if !label
            .find_attribute_as(&XCAFDocView::get_id(), &mut attribute)
            .expect("find")
        {
            out.push(format!("VIEW {} <noattr>", entry(&label)));
            continue;
        }
        let object = attribute.get().expect("view").get_object().expect("object");
        let object = object.get().expect("object");
        let front = object.has_front_plane_clipping();
        let back = object.has_back_plane_clipping();
        out.push(format!(
            "VIEW {} name={} type={} pp={} dir={} up={} zoom={} h={} v={} clip={} front={},{} back={},{} sides={} gdtp={}",
            entry(&label),
            text(&object.name()),
            object.kind() as i32,
            xyz(object.projection_point().xyz()),
            xyz(object.view_direction().xyz()),
            xyz(object.up_direction().xyz()),
            object.zoom_factor(),
            object.window_horizontal_size(),
            object.window_vertical_size(),
            text(&object.clipping_expression()),
            i32::from(front),
            if front { object.front_plane_distance().to_string() } else { "-".to_owned() },
            i32::from(back),
            if back { object.back_plane_distance().to_string() } else { "-".to_owned() },
            i32::from(object.has_view_volume_sides_clipping()),
            object.nb_gdt_points(),
        ));
        let or_empty = |refs: Option<Vec<TdfLabel>>| refs.unwrap_or_default();
        out.push(format!(
            "VIEWR {} {} {} {}",
            entry(&label),
            entries(&or_empty(tool.get_ref_shape_label(&label).expect("refs"))),
            entries(&or_empty(tool.get_ref_gdt_label(&label).expect("refs"))),
            entries(&or_empty(
                tool.get_ref_clipping_plane_label(&label).expect("refs")
            )),
        ));
    }
    dump_clipping_planes(main, out);
}

fn dump_clipping_planes(main: &TdfLabel, out: &mut Vec<String>) {
    let tool: Handle<XCAFDocClippingPlaneTool> =
        XCAFDocDocumentTool::clipping_plane_tool(main).expect("tool");
    for label in tool.get_clipping_planes().expect("planes") {
        let plane = tool
            .get_clipping_plane(&label)
            .expect("get")
            .expect("plane");
        out.push(format!(
            "PLANE {} '{}' {}{} {}",
            entry(&label),
            extended(&plane.name),
            xyz(plane.plane.location().xyz()),
            xyz(plane.plane.axis().direction().xyz()),
            i32::from(plane.capping)
        ));
    }
}

/// All the records of a document, in the order of the reference program.
pub fn dump_document(main: &TdfLabel) -> Vec<String> {
    let mut out = Vec::new();
    dump_names(&main.root().expect("root"), &mut out);
    dump_named_data(main, &mut out);
    let shape_tool = XCAFDocDocumentTool::shape_tool(main).expect("tool");
    for label in all_labels(&shape_tool.base_label()) {
        if XCAFDocShapeTool::is_shape(&label).expect("is shape") {
            dump_shape(&label, &mut out);
        }
    }
    dump_colors_layers_materials(main, &mut out);
    let dim_tol = XCAFDocDocumentTool::dim_tol_tool(main).expect("tool");
    for label in dim_tol.get_dimension_labels().expect("dims") {
        dump_dimension(&label, &mut out);
    }
    for label in dim_tol.get_geom_tolerance_labels().expect("tols") {
        dump_tolerance(&label, &mut out);
    }
    for label in dim_tol.get_datum_labels().expect("datums") {
        dump_datum(&label, &mut out);
    }
    dump_views(main, &mut out);
    out
}

fn numbers_agree(left: &str, right: &str, relative: f64) -> bool {
    let (Ok(a), Ok(b)) = (left.parse::<f64>(), right.parse::<f64>()) else {
        return left == right;
    };
    (a - b).abs() <= relative * a.abs().max(b.abs()).max(1.0)
}

fn tokens(line: &str) -> Vec<String> {
    line.replace(['(', ')', '[', ']', ',', '='], " ")
        .replace('<', " < ")
        .replace('>', " > ")
        .split_whitespace()
        .map(str::to_owned)
        .collect()
}

/// The records that differ, as pairs of the line and this line.
pub fn differences(recorded: &[String], here: &[String], relative: f64) -> Vec<(String, String)> {
    let mut found = Vec::new();
    for index in 0..recorded.len().max(here.len()) {
        let left = recorded.get(index).cloned().unwrap_or_default();
        let right = here.get(index).cloned().unwrap_or_default();
        let (a, b) = (tokens(&left), tokens(&right));
        let same =
            a.len() == b.len() && a.iter().zip(&b).all(|(x, y)| numbers_agree(x, y, relative));
        if !same {
            found.push((left, right));
        }
    }
    found
}
