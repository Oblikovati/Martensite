//! A transparent compound of a box grid and a finely meshed sphere, then the faces its mesh
//! self-intersection test finds overlapping, shown red over it (OCC26180).
use super::prelude::*;
use crate::primitive_shapes::{box_at, compound_of, sphere, translated};
use crate::shape_mesh::{MeshRequest, mesh_shape};
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tktopalgo::brep_extrema::BRepExtremaSelfIntersection;

/// The 30 x 30 grid of 5-unit boxes 6 units apart, row-major as the script's loops add them.
fn box_grid() -> Result<Vec<Shape>, Box<dyn Error>> {
    let mut boxes = Vec::with_capacity(900);
    for i in 0..30 {
        for j in 0..30 {
            let corner = [f64::from(i) * 6., f64::from(j) * 6., 0.];
            boxes.push(box_at(corner, [5.; 3])?);
        }
    }
    Ok(boxes)
}

/// `selfintersect C -tol 0.0`: the compound of the overlapping sub-faces (`C_overlapped`).
fn overlapped_faces(shape: &Shape) -> Result<Shape, Box<dyn Error>> {
    let mut tool = BRepExtremaSelfIntersection::with_shape(shape, 0.0)?;
    tool.perform()?;
    if !tool.is_done() {
        println!("Error: Failed to perform proximity test");
    }
    let mut faces = Vec::new();
    let mut cursor = tool.overlap_elements().cursor();
    while cursor.more() {
        faces.push(tool.get_sub_shape(*cursor.key()?)?.into_shape());
        cursor.next();
    }
    Ok(compound_of(&faces)?)
}

/// `bug26180`: the transparent compound, then its overlapped faces in red, refitted.
pub(crate) fn overlapped_faces_view() -> Outcome {
    banner(&["========", "OCC26180", "========", ""]);
    let s = translated(&sphere(30.)?, [90., 90., 0.])?;
    mesh_shape(&s, &MeshRequest::new(0.002))?;
    let mut parts = box_grid()?;
    parts.push(s);
    let c = compound_of(&parts)?;
    let mut run = SessionRun::new()?;
    run.session().set_display_mode(1)?;
    run.session().display("C", &c, &DisplayRequest::default())?;
    run.session().set_transparency("C", 0.8)?;
    run.dump("bug26180_1.png")?;
    let overlapped = overlapped_faces(&c)?;
    run.session()
        .display("C_overlapped", &overlapped, &DisplayRequest::default())?;
    run.session()
        .set_color("C_overlapped", Color::named(NamedColor::RED))?;
    run.session().set_transparency("C_overlapped", 0.5)?;
    run.session().fit()?;
    run.dump("bug26180_2.png")?;
    run.finish()
}
