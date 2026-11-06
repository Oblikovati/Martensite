//! `XDumpNbDGTs`, `XDumpDGTs` and `XGetProperties`, on real API: what the end scripts read back.
use crate::reports::g17;
use crate::xs_session::XsSession;
use martensite_tkernel::tcollection::HAsciiString;
use martensite_tkernel::{
    standard::{Failure, Handle},
    tcollection::AsciiString,
};
use martensite_tklcaf::tdf::{TdfAttributeState, TdfChildIterator, TdfLabel, TdfTool};
use martensite_tkmath::gp::Axis2;
use martensite_tkxcaf::{
    xcaf_dim_tol_objects::{
        XCAFDimTolObjectsDatumModifWithValue, XCAFDimTolObjectsDatumTargetType,
        XCAFDimTolObjectsDimensionFormVariance, XCAFDimTolObjectsDimensionGrade,
        XCAFDimTolObjectsDimensionObject, XCAFDimTolObjectsDimensionType,
        XCAFDimTolObjectsGeomToleranceMatReqModif, XCAFDimTolObjectsGeomToleranceObject,
        XCAFDimTolObjectsGeomToleranceZoneModif,
    },
    xcaf_doc::{
        XCAFDoc, XCAFDocDatum, XCAFDocDimTolTool, XCAFDocDimension, XCAFDocDocumentTool,
        XCAFDocGeomTolerance, XCAFDocGraphNode, XCAFDocShapeTool,
    },
};

fn entry_of(label: &TdfLabel) -> String {
    let mut entry = AsciiString::default();
    TdfTool::entry(label, &mut entry);
    String::from_utf8_lossy(entry.as_bytes()).into_owned()
}

fn text_of(name: &Handle<HAsciiString>) -> Option<String> {
    name.get()
        .map(|name| String::from_utf8_lossy(name.string().as_bytes()).into_owned())
}

/// The labels both dumps walk: every shape, each followed by its sub-shapes, appended as they are met.
fn all_shape_labels(shapes: &XCAFDocShapeTool) -> Result<Vec<TdfLabel>, Failure> {
    let mut labels = shapes.get_shapes()?;
    let mut at = 0;
    while at < labels.len() {
        let label = labels[at].clone();
        XCAFDocShapeTool::get_sub_shapes(&label, &mut labels)?;
        at += 1;
    }
    Ok(labels)
}

fn found<T: martensite_tkernel::standard::RuntimeTyped + ?Sized>(
    label: &TdfLabel,
    id: &martensite_tkernel::standard::Guid,
) -> Option<Handle<T>> {
    let mut attribute: Handle<T> = Handle::default();
    label
        .find_attribute_as(id, &mut attribute)
        .expect("an attribute")
        .then_some(attribute)
}

/// The three axes written with no separator between coordinates.
fn axis_text(axis: &Axis2) -> String {
    let (location, x, y) = (axis.location(), axis.x_direction(), axis.y_direction());
    format!(
        ", A ( L ({}{}{}), XD ({}{}{}), RD ({}{}{}))",
        g17(location.x()),
        g17(location.y()),
        g17(location.z()),
        g17(x.x()),
        g17(x.y()),
        g17(x.z()),
        g17(y.x()),
        g17(y.y()),
        g17(y.z()),
    )
}

fn connection_text(tag: &str, name: Option<String>, is_point: bool) -> String {
    let kind = if is_point { "P," } else { "CS," };
    format!(" {tag} \"{}\" {kind}", name.unwrap_or_default())
}

fn dimension_details(object: &XCAFDimTolObjectsDimensionObject) -> String {
    let mut text = String::from(" (");
    if let Some(name) = text_of(&object.get_semantic_name()) {
        text += &format!(" N \"{name}\"");
    }
    text += &format!(" T {}", object.get_type() as i32);
    if object.is_dim_with_range() {
        text += &format!(
            ", LB {}, UB {}",
            g17(object.get_lower_bound()),
            g17(object.get_upper_bound())
        );
    } else {
        text += &format!(", V {}", g17(object.get_value()));
        if object.is_dim_with_plus_minus_tolerance() {
            text += &format!(
                ", VL {}, VU {}",
                g17(object.get_lower_tol_value()),
                g17(object.get_upper_tol_value())
            );
        } else if object.is_dim_with_class_of_tolerance() {
            let (mut hole, mut form, mut grade) = (
                false,
                XCAFDimTolObjectsDimensionFormVariance::default(),
                XCAFDimTolObjectsDimensionGrade::default(),
            );
            object.get_class_of_tolerance(&mut hole, &mut form, &mut grade);
            text += &format!(
                ", H {} F {} G {}",
                i32::from(hole),
                form as i32,
                grade as i32
            );
        }
    }
    if object.has_qualifier() {
        text += &format!(", Q {}", object.get_qualifier() as i32);
    }
    if object.has_angular_qualifier() {
        text += &format!(", AQ {}", object.get_angular_qualifier() as i32);
    }
    if object.get_type() == XCAFDimTolObjectsDimensionType::LocationOriented {
        let mut direction = martensite_tkmath::gp::Direction3::default();
        object.get_direction(&mut direction);
        text += &format!(
            ", D ({}, {}, {})",
            g17(direction.x()),
            g17(direction.y()),
            g17(direction.z())
        );
    }
    let modifiers = object.get_modifiers();
    if !modifiers.is_empty() {
        text.push(',');
        for modifier in modifiers {
            text += &format!(" M {}", modifier as i32);
        }
    }
    text += &format!(", P {}", i32::from(!object.get_path().is_null()));
    if object.has_point() {
        text += &connection_text(
            "Conn1",
            text_of(&object.get_connection_name()),
            object.is_point_connection(),
        );
    }
    if object.has_point2() {
        text += &connection_text(
            "Conn2",
            text_of(&object.get_connection_name2()),
            object.is_point_connection2(),
        );
    }
    text + " )"
}

fn tolerance_details(object: &XCAFDimTolObjectsGeomToleranceObject) -> String {
    let mut text = String::from(" (");
    if let Some(name) = text_of(&object.get_semantic_name()) {
        text += &format!(" N \"{name}\"");
    }
    text += &format!(
        " T {} TV {}, V {}",
        object.get_type() as i32,
        object.get_type_of_value() as i32,
        g17(object.get_value())
    );
    if object.has_axis() {
        text += &axis_text(&object.get_axis());
    }
    let modifiers = object.get_modifiers();
    if !modifiers.is_empty() {
        text.push(',');
        for modifier in modifiers {
            text += &format!(" M {}", modifier as i32);
        }
    }
    if object.get_material_requirement_modifier() != XCAFDimTolObjectsGeomToleranceMatReqModif::None
    {
        text += &format!(", MR {}", object.get_material_requirement_modifier() as i32);
    }
    if object.get_max_value_modifier() > 0.0 {
        text += &format!("MaxV {}", g17(object.get_max_value_modifier()));
    }
    if object.get_zone_modifier() != XCAFDimTolObjectsGeomToleranceZoneModif::None {
        text += &format!(", ZM {}", object.get_zone_modifier() as i32);
        if object.get_value_of_zone_modifier() > 0.0 {
            text += &format!(" ZMV {}", g17(object.get_value_of_zone_modifier()));
        }
    }
    text + " )"
}

/// The datums of a tolerance: each names the tolerance's semantic name, as the `aDimTolObj` does.
fn tolerance_datums(
    label: &TdfLabel,
    tolerance: &XCAFDimTolObjectsGeomToleranceObject,
    (i, j): (usize, usize),
    text: &mut String,
) -> Result<(), Failure> {
    let Some(node) = found::<XCAFDocGraphNode>(label, &XCAFDoc::datum_tol_ref_guid()) else {
        return Ok(());
    };
    for (k, child) in node
        .get()
        .expect("a graph node")
        .children()
        .iter()
        .enumerate()
    {
        let child_label = child.get().expect("a child").label();
        let Some(datum) = found::<XCAFDocDatum>(&child_label, &XCAFDocDatum::get_id()) else {
            continue;
        };
        let datum = datum.get().expect("a datum").get_object()?;
        let datum = datum.get().expect("a datum object");
        *text += &format!(
            "\n \t \t {} Datum.{i}.{j}.{}",
            entry_of(&child_label),
            k + 1
        );
        *text += " (";
        if let Some(name) = text_of(&tolerance.get_semantic_name()) {
            *text += &format!(" N \"{name}\"");
        }
        let modifiers = datum.get_modifiers();
        if !modifiers.is_empty() {
            text.push(',');
            for modifier in modifiers {
                *text += &format!(" M {}", modifier as i32);
            }
        }
        let (modifier, value) = datum.get_modifier_with_value();
        if modifier != XCAFDimTolObjectsDatumModifWithValue::None {
            *text += &format!(", MV{} {}", modifier as i32, g17(value));
        }
        *text += " )";
    }
    Ok(())
}

/// One shape label's lines of `XDumpDGTs doc all 1`.
fn dump_label(
    tool: &XCAFDocDimTolTool,
    label: &TdfLabel,
    i: usize,
    text: &mut String,
) -> Result<(), Failure> {
    let mut header_written = false;
    let header = |written: &mut bool, text: &mut String| {
        if !*written {
            *text += &format!("\n {} Shape.{i}", entry_of(label));
            *written = true;
        }
    };
    let mut dimensions = Vec::new();
    tool.get_ref_dimension_labels(label, &mut dimensions)?;
    for (j, gdt) in dimensions.iter().enumerate() {
        let Some(attribute) = found::<XCAFDocDimension>(gdt, &XCAFDocDimension::get_id()) else {
            continue;
        };
        let object = attribute.get().expect("a dimension").get_object()?;
        header(&mut header_written, text);
        *text += &format!("\n \t {} Dimension.{i}.{}", entry_of(gdt), j + 1);
        *text += &dimension_details(object.get().expect("a dimension object"));
    }
    let mut tolerances = Vec::new();
    tool.get_ref_geom_tolerance_labels(label, &mut tolerances)?;
    for (j, gdt) in tolerances.iter().enumerate() {
        let Some(attribute) = found::<XCAFDocGeomTolerance>(gdt, &XCAFDocGeomTolerance::get_id())
        else {
            continue;
        };
        let object = attribute.get().expect("a tolerance").get_object()?;
        let object = object.get().expect("a tolerance object");
        header(&mut header_written, text);
        *text += &format!("\n \t {} GeomTolerance.{i}.{}", entry_of(gdt), j + 1);
        *text += &tolerance_details(object);
        tolerance_datums(gdt, object, (i, j + 1), text)?;
    }
    let mut datums = Vec::new();
    if tool.get_ref_datum_label(label, &mut datums)? {
        for (j, datum) in datums.iter().enumerate() {
            let Some(attribute) = found::<XCAFDocDatum>(datum, &XCAFDocDatum::get_id()) else {
                continue;
            };
            let object = attribute.get().expect("a datum").get_object()?;
            let object = object.get().expect("a datum object");
            if !object.is_datum_target() {
                continue;
            }
            header(&mut header_written, text);
            // Names the first datum here, whichever the target is.
            *text += &format!("\n \t {} Datum target.{i}.{}", entry_of(&datums[0]), j + 1);
            *text += " (";
            if let Some(name) = text_of(&object.get_semantic_name()) {
                *text += &format!(" N \"{name}\"");
            }
            let kind = object.get_datum_target_type();
            *text += &format!(" T {}", kind as i32);
            if kind != XCAFDimTolObjectsDatumTargetType::Area {
                *text += &axis_text(&object.get_datum_target_axis());
                if kind != XCAFDimTolObjectsDatumTargetType::Point {
                    *text += &format!(", L {}", g17(object.get_datum_target_length()));
                    if kind == XCAFDimTolObjectsDatumTargetType::Rectangle {
                        *text += &format!(", W {}", g17(object.get_datum_target_width()));
                    }
                }
            }
            *text += " )";
        }
    }
    Ok(())
}

/// `XDumpNbDGTs doc`'s counts of datums over the shape labels: features, attached datums, targets.
fn datum_counts(tool: &XCAFDocDimTolTool, labels: &[TdfLabel]) -> Result<(i32, i32, i32), Failure> {
    let (mut features, mut attached, mut targets) = (0, 0, 0);
    for label in labels {
        let mut datums = Vec::new();
        if !tool.get_ref_datum_label(label, &mut datums)? {
            continue;
        }
        let mut is_datum = false;
        for datum in &datums {
            let Some(attribute) = found::<XCAFDocDatum>(datum, &XCAFDocDatum::get_id()) else {
                continue;
            };
            if attribute
                .get()
                .expect("a datum")
                .get_object()?
                .get()
                .expect("a datum object")
                .is_datum_target()
            {
                targets += 1;
            } else {
                attached += 1;
                is_datum = true;
            }
        }
        if is_datum {
            features += 1;
        }
    }
    Ok((features, attached, targets))
}

impl XsSession {
    fn tools_of(
        &self,
        doc: &str,
    ) -> Result<(Handle<XCAFDocDimTolTool>, Handle<XCAFDocShapeTool>), Failure> {
        let main = self.documents[doc].get().expect("a document").main()?;
        Ok((
            XCAFDocDocumentTool::dim_tol_tool(&main)?,
            XCAFDocDocumentTool::shape_tool(&main)?,
        ))
    }

    /// `XDumpNbDGTs doc`: written to the stream, returned as the command's result.
    /// Example: `session.dump_nb_dgts("D_First")`.
    pub fn dump_nb_dgts(&self, doc: &str) -> String {
        println!("XDumpNbDGTs {doc} ");
        let (tool, shapes) = self.tools_of(doc).expect("the tools of a document");
        let (tool, shapes) = (tool.get().expect("a tool"), shapes.get().expect("a tool"));
        let labels = all_shape_labels(shapes).expect("shape labels");
        let (features, attached, targets) = datum_counts(tool, &labels).expect("datums");
        let text = format!(
            "\n NbOfDimensions          : {}\n NbOfTolerances          : {}\n NbOfDatumFeature        : {features}\n NbOfAttachedDatum       : {attached}\n NbOfDatumTarget         : {targets}",
            tool.get_dimension_labels().expect("dimensions").len(),
            tool.get_geom_tolerance_labels().expect("tolerances").len(),
        );
        println!("{text}");
        text
    }

    /// `XDumpDGTs doc all 1`. Example: `session.dump_dgts("D_First")`.
    pub fn dump_dgts(&self, doc: &str) -> String {
        println!("XDumpDGTs {doc} all 1 ");
        let (tool, shapes) = self.tools_of(doc).expect("the tools of a document");
        let (tool, shapes) = (tool.get().expect("a tool"), shapes.get().expect("a tool"));
        let labels = all_shape_labels(shapes).expect("shape labels");
        let mut text = String::new();
        for (at, label) in labels.iter().enumerate() {
            dump_label(tool, label, at + 1, &mut text).expect("a dump");
        }
        println!("{text}");
        text
    }

    /// `XGetProperties doc`: each shape label's named data, `Property for [entry]:`.
    /// Example: `session.get_properties("D")`.
    pub fn get_properties(&self, doc: &str) -> String {
        println!("XGetProperties {doc} ");
        let (_, shapes) = self.tools_of(doc).expect("the tools of a document");
        let shapes = shapes.get().expect("a tool");
        let mut text = String::new();
        let mut children = TdfChildIterator::new(&shapes.label(), true).expect("children");
        while children.more() {
            let label = children.value().clone();
            let data = shapes
                .get_named_properties(&label, false)
                .expect("named data");
            if let Some(data) = data.get() {
                text += &format!("Property for [{}]:\n", entry_of(&label));
                data.load_deferred_data(false);
                crate::xde_properties::append_named_data(data, &mut text);
            }
            children.next();
        }
        print!("{text}");
        text
    }
}
