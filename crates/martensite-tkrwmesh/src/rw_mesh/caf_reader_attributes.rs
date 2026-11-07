//! The attributes a reader puts on document labels.
use super::caf_reader::RwMeshCafDocumentTools;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::message::Message;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkernel::tcollection::{AsciiString, ExtendedString};
use martensite_tkg3d::top_abs::{ShapeEnum, shape_type_to_string};
use martensite_tklcaf::tdata_std::{TDataStdName, TDataStdNamedData};
use martensite_tklcaf::tdf::{TdfAttribute, TdfLabel};
use martensite_tkxcaf::xcaf_doc::{XCAFDocColorType, XCAFDocShapeMapTool};
use martensite_tkxcaf::xcaf_prs::XCAFPrsStyle;

/// Sets a name from text; `set_name(&label, &name)?`.
pub(super) fn set_name(label: &TdfLabel, name: &AsciiString) -> Result<(), Failure> {
    TDataStdName::set(label, &ExtendedString::from_ascii(name))?;
    Ok(())
}

/// The capitalized shape type; `shape_type_name(ShapeEnum::Face)` is `Face`.
pub(super) fn shape_type_name(shape_type: ShapeEnum) -> AsciiString {
    let mut name = AsciiString::new(shape_type_to_string(shape_type));
    name.capitalize();
    name
}

/// Puts a name on a label: its own, a shape type under a parent, or the parent's name at the root.
pub(super) fn set_shape_name(
    label: &TdfLabel,
    shape_type: ShapeEnum,
    name: &AsciiString,
    parent: &TdfLabel,
    parent_name: &AsciiString,
) -> Result<(), Failure> {
    if !name.is_empty() {
        return set_name(label, name);
    }
    if !parent.is_null() {
        return set_name(label, &shape_type_name(shape_type));
    }
    if !parent_name.is_empty() {
        return set_name(label, parent_name);
    }
    Ok(())
}

pub(super) fn set_shape_style(
    tools: &RwMeshCafDocumentTools,
    label: &TdfLabel,
    style: &XCAFPrsStyle,
) -> Result<(), Failure> {
    let colors = tools.color_tool.get();
    if let Some(colors) = colors.filter(|_| style.is_set_color_surf()) {
        colors.set_color(
            label,
            &style.get_color_surf_rgba(),
            XCAFDocColorType::ColorSurf,
        )?;
    }
    if let Some(colors) = colors.filter(|_| style.is_set_color_curv()) {
        let curve = martensite_tkernel::quantity::ColorRgba::from_rgb(style.get_color_curv(), 1.0);
        colors.set_color(label, &curve, XCAFDocColorType::ColorCurv)?;
    }
    set_shape_material(tools, label, style)
}

fn set_shape_material(
    tools: &RwMeshCafDocumentTools,
    label: &TdfLabel,
    style: &XCAFPrsStyle,
) -> Result<(), Failure> {
    let Some(material) = style.material().get() else {
        return Ok(());
    };
    let Some(materials) = tools.vis_material_tool.get() else {
        return Ok(());
    };
    let mut material_label = material.attribute_base().label();
    if material_label.is_null() {
        let name = material
            .raw_name()
            .get()
            .map_or_else(AsciiString::default, |raw| raw.string().clone());
        material_label = materials.add_material_attribute(style.material(), &name)?;
    }
    materials.set_shape_material(label, &material_label)
}

pub(super) fn set_shape_named_data(
    label: &TdfLabel,
    named_data: &Handle<TDataStdNamedData>,
) -> Result<(), Failure> {
    let Some(data) = named_data.get() else {
        return Ok(());
    };
    let data_label = data.attribute_base().label();
    let mut other: Handle<TDataStdNamedData> = Handle::default();
    if label.find_attribute_as(&data.id(), &mut other)? {
        let same = other
            .get()
            .is_some_and(|o| o.attribute_base().label() == data_label);
        if !same {
            send_alarm("Error! Different NamedData is already set to shape");
        }
    } else if data_label.is_null() {
        label.add_attribute(&named_data.clone().cast(), true)?;
    } else {
        send_alarm("Error! Skipped NamedData instance shared across shapes");
    }
    Ok(())
}

pub(super) fn record_sub_shapes(label: &TdfLabel, shape: &Shape) -> Result<(), Failure> {
    if let Some(map) = XCAFDocShapeMapTool::set(label)?.get() {
        map.set_shape(shape);
    }
    Ok(())
}

fn send_alarm(text: &str) {
    Message::send_alarm_text(&AsciiString::new(text));
}
