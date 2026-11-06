//! Boolean results of restored solids and faces, measured, then shown in the 3D check view
//! (OCC3721, OCC6278, OCC23738, OCC22910).
use super::geometry_faces::surface_face;
use super::prelude::*;
use crate::boolean_typed_explode::all_distinct;
use crate::common_operation;
use crate::shape_mesh::check_real;
use martensite_tkbo::bop_algo::Operation;
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::topods;
use martensite_tkg3d::top_abs::ShapeEnum;

/// How a regression builds its Boolean result.
#[derive(Clone, Copy, Debug)]
pub(crate) enum BooleanPath {
    /// `bfuse`, `bcommon` or `bcut` of the first operand with the second.
    Direct(Operation),
    /// `bop` of the operands, then the result command of the operation.
    Prepared(Operation),
}

/// One OCC3721 variant: the operand order, the Boolean path, the expected area and image.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Occ3721 {
    pub(crate) swapped: bool,
    pub(crate) path: BooleanPath,
    pub(crate) area: f64,
    pub(crate) image: &'static str,
}

/// Run `path` on `first` and `second`; `boolean(&a, &b, BooleanPath::Direct(Operation::Fuse))`.
pub(crate) fn boolean(first: &Shape, second: &Shape, path: BooleanPath) -> Shape {
    match path {
        BooleanPath::Direct(kind) => common_operation::direct(first, second, kind),
        BooleanPath::Prepared(kind) => common_operation::prepared(first, second, kind),
    }
}

fn restored_valid(name: &str) -> Result<Shape, Box<dyn Error>> {
    let shape = read_dataset_shape(name)?;
    check_shape_valid(&shape)?;
    Ok(shape)
}

/// `bug3721_<n>`: two restored solids combined, the result valid with the expected area.
pub(crate) fn occ3721_view(case: Occ3721) -> Result<(), Box<dyn Error>> {
    banner(&["================", "OCC3721", "================", ""]);
    let b1 = restored_valid("OCC3721-1.brep")?;
    let b2 = restored_valid("OCC3721-2.brep")?;
    let (first, second) = if case.swapped { (&b2, &b1) } else { (&b1, &b2) };
    let result = boolean(first, second, case.path);
    check_shape_valid(&result)?;
    check_mass(&result, &MassCheck::near(MassKind::Area, case.area))?;
    checkview_3d(&result, case.image)
}

/// `bug6278`: the fused solids keep four vertices; the second lies at (-32.3043, 20, 0).
pub(crate) fn fused_vertex_position() -> Result<(), Box<dyn Error>> {
    banner(&["================", "OCC6278", "================", ""]);
    let a = read_dataset_shape("OCC6278-A.brep")?;
    let b = read_dataset_shape("OCC6278-B.brep")?;
    let result = boolean(&a, &b, BooleanPath::Direct(Operation::Fuse));
    let vertices = all_distinct(&result, ShapeEnum::Vertex);
    check_sub_shape_count(&result, ShapeEnum::Vertex, "VERTEX", 4)?;
    let point = BRepTool::pnt(&topods::vertex(&vertices[1])?)?;
    check_real("x", point.x(), -32.3042674857046, 0., 0.001);
    check_real("y", point.y(), 20.0000000000001, 0., 0.001);
    if point.z() != 0. {
        check_real("z", point.z(), 0., 0., 0.001);
    }
    check_mass(&result, &MassCheck::near(MassKind::Length, 278.725))?;
    report_alone_vertices(&result)?;
    checkview_3d(&result, "bug6278.png")
}

/// `checknbshapes shape -<kind> <expected>` for one kind.
fn check_sub_shape_count(
    shape: &Shape,
    kind: ShapeEnum,
    label: &str,
    expected: usize,
) -> Result<(), Box<dyn Error>> {
    let found = all_distinct(shape, kind).len();
    if found != expected {
        println!("Error : number of {label} entities in shape is {found}, expected {expected}");
    }
    Ok(())
}

/// `bug23738`: the prepared fuse of two restored faces keeps its area and sub-shape counts.
pub(crate) fn prepared_face_fuse() -> Result<(), Box<dyn Error>> {
    banner(&["================", "OCC23738", "================", ""]);
    let b1 = read_dataset_shape("bug23738_b1.brep")?;
    let b2 = read_dataset_shape("bug23738_b2.brep")?;
    let result = boolean(&b1, &b2, BooleanPath::Prepared(Operation::Fuse));
    check_mass(&result, &MassCheck::near(MassKind::Area, 225009.))?;
    check_sub_shape_counts(&result, &[6, 10, 8, 6, 0, 0, 0, 1, 31])?;
    checkview_3d(&result, "bug23738.png")
}

/// `bug22910_2`: a face on the restored surface meshes at 0.0038477 without exception and
/// keeps its area.
pub(crate) fn surface_face_meshed() -> Result<(), Box<dyn Error>> {
    banner(&["================", "OCC22910", "================", ""]);
    let result = surface_face("s.rle")?;
    mesh_shape(&result, &MeshRequest::new(0.0038476968063336017))?;
    println!("OK OCC22910");
    check_mass(&result, &MassCheck::near(MassKind::Area, 0.110535))?;
    checkview_3d(&result, "bug22910_2.png")
}

/// `bug13`: the section of faces on two restored surfaces is a valid shape.
pub(crate) fn surface_faces_section() -> Result<(), Box<dyn Error>> {
    banner(&["================", "OCC13", "================", ""]);
    let f1 = surface_face("OCC13-1.draw")?;
    let f2 = surface_face("OCC13-2.draw")?;
    let result = common_operation::section(&f1, &f2);
    check_shape_valid(&result)?;
    checkview_3d(&result, "bug13.png")
}
