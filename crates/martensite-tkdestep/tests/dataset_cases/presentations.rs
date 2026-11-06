//! `gdt/presentation`: `bug 27235`'s centre of mass and mass of all the presentations, read and read again.
use crate::de_end::locate_data_file;
use crate::script_commands::image_file;
use crate::support::script_error::script_error;
use crate::xs_session::XsSession;
use martensite_tkbrep::{
    brep::BRepBuilder,
    topods::{Compound, Shape},
};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkgeombase::gprop::GProps;
use martensite_tklcaf::tdf::TdfLabel;
use martensite_tktopalgo::brep_gprop::BRepGProp;
use martensite_tkxcaf::xcaf_doc::{
    XCAFDocDatum, XCAFDocDimension, XCAFDocDocumentTool, XCAFDocGeomTolerance, XCAFDocShapeTool,
};

/// The reference of a `gdt/presentation` row: centre of mass and mass of each read.
pub struct Presentation {
    pub file: &'static str,
    pub first: [f64; 4],
    pub second: [f64; 4],
}

fn with_shapes(
    labels: &[TdfLabel],
    mut shape_of: impl FnMut(&TdfLabel) -> Result<Option<Shape>, Failure>,
    mut add: impl FnMut(&Shape),
) -> Result<(), Failure> {
    for label in labels {
        if let Some(shape) = shape_of(label)?
            && !shape.is_null()
        {
            add(&shape);
        }
    }
    Ok(())
}

/// `bug 27235 doc`: the four numbers it writes, `x y z mass`.
fn presentations_of(session: &XsSession, doc: &str) -> Result<[f64; 4], Failure> {
    println!("27235 {doc} ");
    let main = session.documents[doc].get().expect("a document").main()?;
    let tool = XCAFDocDocumentTool::dim_tol_tool(&main)?;
    let tool = tool.get().expect("a tool");
    let shapes = XCAFDocDocumentTool::shape_tool(&main)?;
    let mut labels = shapes.get().expect("a tool").get_shapes()?;
    let mut at = 0;
    while at < labels.len() {
        let label = labels[at].clone();
        XCAFDocShapeTool::get_sub_shapes(&label, &mut labels)?;
        at += 1;
    }
    let builder = BRepBuilder::default();
    let mut compound = Compound::default();
    builder.make_compound(&mut compound);
    let mut pending: Vec<Shape> = Vec::new();
    with_shapes(
        &tool.get_dimension_labels()?,
        |label| {
            let mut attribute: Handle<XCAFDocDimension> = Handle::default();
            if !label.find_attribute_as(&XCAFDocDimension::get_id(), &mut attribute)? {
                return Ok(None);
            }
            Ok(attribute
                .get()
                .expect("a dimension")
                .get_object()?
                .get()
                .map(|object| object.get_presentation()))
        },
        |shape| pending.push(shape.clone()),
    )?;
    with_shapes(
        &tool.get_geom_tolerance_labels()?,
        |label| {
            let mut attribute: Handle<XCAFDocGeomTolerance> = Handle::default();
            if !label.find_attribute_as(&XCAFDocGeomTolerance::get_id(), &mut attribute)? {
                return Ok(None);
            }
            Ok(attribute
                .get()
                .expect("a tolerance")
                .get_object()?
                .get()
                .map(|object| object.get_presentation()))
        },
        |shape| pending.push(shape.clone()),
    )?;
    for label in &labels {
        let mut datums = Vec::new();
        if tool.get_ref_datum_label(label, &mut datums)? {
            with_shapes(
                &datums,
                |datum| {
                    let mut attribute: Handle<XCAFDocDatum> = Handle::default();
                    if !datum.find_attribute_as(&XCAFDocDatum::get_id(), &mut attribute)? {
                        return Ok(None);
                    }
                    Ok(attribute
                        .get()
                        .expect("a datum")
                        .get_object()?
                        .get()
                        .map(|object| object.get_presentation()))
                },
                |shape| pending.push(shape.clone()),
            )?;
        }
    }
    for shape in &pending {
        builder.add(&mut compound, shape)?;
    }
    let mut props = GProps::default();
    BRepGProp::linear_properties(&compound.into_shape(), &mut props, false, false)?;
    let centre = props.centre_of_mass();
    let values = [centre.x(), centre.y(), centre.z(), props.mass()];
    println!(
        "Centre of mass: {} {} {}",
        crate::reports::g17(values[0]),
        crate::reports::g17(values[1]),
        crate::reports::g17(values[2])
    );
    println!("Mass: {}", crate::reports::g17(values[3]));
    Ok(values)
}

fn read(session: &mut XsSession, doc: &str, path: &str) {
    println!("ReadStep {doc} {path} ");
    if session.read_step(doc, path).is_err() {
        println!(
            "Error: {} - file was not read - exception ",
            if doc == "D_First" { "First" } else { "Second" }
        );
        script_error(doc);
    }
}

/// The script: `gdt/begin`, read, write, read back, each `bug 27235` against its reference within `1e-4`.
/// Example: `run(&mut session, "C3", &row)`.
pub fn run(session: &mut XsSession, case: &str, row: &Presentation) {
    session.pilot_command("param write.step.schema 5");
    session.pilot_command("newmodel");
    let Some(path) = locate_data_file(row.file) else {
        script_error(&format!("File {} could not be found", row.file));
    };
    read(session, "D_First", &path.to_string_lossy());
    println!();
    let first =
        presentations_of(session, "D_First").unwrap_or_else(|failure| panic!("27235: {failure:?}"));
    println!();
    println!(" ");
    println!("-----------------------------WRITING FILE ------------------------------");
    let written = image_file(
        session.source.clone().as_str(),
        &format!("{case}_D_First.stp"),
    )
    .to_string_lossy()
    .into_owned();
    println!("WriteStep D_First {written} ");
    if session.write_step("D_First", &written).is_err() {
        println!("Error: First - file was not written - exception");
        script_error("WriteStep");
    }
    println!();
    read(session, "D_Second", &written);
    let _ = std::fs::remove_file(&written);
    println!("Close D_First ");
    session.close_document("D_First");
    println!();
    let second = presentations_of(session, "D_Second")
        .unwrap_or_else(|failure| panic!("27235: {failure:?}"));
    println!("Close D_Second ");
    session.close_document("D_Second");
    println!();
    println!("========================== Comparison with reference data ========");
    println!();
    let mut differences = 0;
    let mut report = String::new();
    for index in 0..4 {
        if (row.first[index] - first[index]).abs() > 1e-4
            || (row.second[index] - second[index]).abs() > 1e-4
        {
            differences += 1;
            report += &format!(
                " Reference data - {} ({})\n Current data - {} ({})\n--------------------------------------------------------------------\n",
                row.second[index], row.first[index], second[index], first[index]
            );
        }
    }
    if differences > 0 {
        println!("Error : {differences} differences with reference data found :\n{report}");
    } else {
        println!("Comparison of current result with reference data - OK\n");
    }
    println!("--------------------------------------------------------------------");
    println!();
    println!("TEST COMPLETED");
    session.pilot_command("param write.step.schema 4");
    session.pilot_command("newmodel");
}
