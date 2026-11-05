//! The modelling commands the scripts make their shapes with.
use super::commands::CafSession;
use crate::support::script_error::script_error;
use martensite_tkbrep::brep::BRepBuilder;
use martensite_tkbrep::brep_tools;
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::storage::IoStream;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::Point3;
use martensite_tkprim::brep_prim_api::{
    BRepPrimAPIMakeBox, BRepPrimAPIMakeCone, BRepPrimAPIMakeSphere,
};
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeShape, BRepBuilderAPIMakeVertex};
use std::path::PathBuf;

/// The header word of a shape file written with its drawable type.
const SHAPE_HEADER: &str = "DBRep_DrawableShape";

impl CafSession {
    /// `box name dx dy dz`.
    /// Example: `session.box_of_size("aBox1", 100, 200, 300);`.
    pub fn box_of_size(&mut self, name: &str, dx: i32, dy: i32, dz: i32) {
        println!("box {name} {dx} {dy} {dz} ");
        let sizes = [dx, dy, dz].map(f64::from);
        let mut solid = BRepPrimAPIMakeBox::from_sizes(sizes[0], sizes[1], sizes[2]).unwrap();
        self.bind_shape(name, solid.solid().unwrap().into_shape());
    }

    /// `box name x y z dx dy dz`.
    /// Example: `session.box_at("aBox2", [50, 50, 0], [100, 200, 300]);`.
    pub fn box_at(&mut self, name: &str, at: [i32; 3], size: [i32; 3]) {
        let [x, y, z] = at;
        let [dx, dy, dz] = size;
        println!("box {name} {x} {y} {z} {dx} {dy} {dz} ");
        let corner = Point3::new(f64::from(x), f64::from(y), f64::from(z));
        let (dx, dy, dz) = (f64::from(dx), f64::from(dy), f64::from(dz));
        let mut solid = BRepPrimAPIMakeBox::at_point(corner, dx, dy, dz).unwrap();
        self.bind_shape(name, solid.solid().unwrap().into_shape());
    }

    /// `psphere name r`.
    /// Example: `session.psphere("aSphere", 100);`.
    pub fn psphere(&mut self, name: &str, radius: i32) {
        println!("psphere {name} {radius} ");
        let mut maker = BRepPrimAPIMakeSphere::new(f64::from(radius)).unwrap();
        self.bind_shape(name, maker.shape().unwrap().clone());
    }

    /// `pcone name r1 r2 h`.
    /// Example: `session.pcone("aCone", 50, 20, 500);`.
    pub fn pcone(&mut self, name: &str, r1: i32, r2: i32, height: i32) {
        println!("pcone {name} {r1} {r2} {height} ");
        let (r1, r2, height) = (f64::from(r1), f64::from(r2), f64::from(height));
        let mut maker = BRepPrimAPIMakeCone::new(r1, r2, height).unwrap();
        self.bind_shape(name, maker.shape().unwrap().clone());
    }

    /// `explode name f`: binds `name_1`, `name_2`, ... to its faces, in the explorer's order.
    /// Example: `session.explode_faces("b3");`.
    pub fn explode_faces(&mut self, name: &str) {
        println!("explode {name} f ");
        let shape = self.shape(name).clone();
        let mut parts = Vec::new();
        let mut explorer = Explorer::new(&shape, ShapeEnum::Face, ShapeEnum::Shape).unwrap();
        while explorer.more() {
            parts.push(explorer.current().clone());
            explorer.next().unwrap();
        }
        let names: Vec<String> = (1..=parts.len()).map(|at| format!("{name}_{at}")).collect();
        for (part, name) in parts.into_iter().zip(&names) {
            self.bind_shape(name, part);
        }
    }

    /// `vertex name x y z`, which also drops what `name` held.
    /// Example: `session.vertex_at("a", [0., 0., 0.]);`.
    pub fn vertex_at(&mut self, name: &str, at: [f64; 3]) {
        let [x, y, z] = at;
        println!("vertex {name} {x} {y} {z} ");
        let mut maker = BRepBuilderAPIMakeVertex::new(Point3::new(x, y, z)).unwrap();
        self.bind_shape(name, maker.shape().unwrap().clone());
    }

    /// `restore [locate_data_file file] name`, from the test dataset's folder of its kind.
    /// Example: `session.restore("occ-source/Bottom.brep", "a");`.
    pub fn restore(&mut self, folder: &str, file: &str, name: &str) {
        let path = dataset_file(folder, file);
        println!("brestore {} {name} ", path.display());
        let bytes = std::fs::read(&path).unwrap_or_else(|cause| {
            script_error(&format!(
                "File {} could not be found: {cause}",
                path.display()
            ))
        });
        let mut stream = IoStream::from_bytes(without_header(&bytes).to_vec());
        let mut shape = Shape::default();
        brep_tools::read(
            &mut shape,
            &mut stream,
            &BRepBuilder::default(),
            &ProgressRange::default(),
        )
        .unwrap();
        self.bind_shape(name, shape);
    }
}

fn dataset_file(folder: &str, file: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../external/martensite-test-data")
        .join(folder)
        .join(file)
}

/// A shape file, past the drawable-type line some of them start with.
fn without_header(bytes: &[u8]) -> &[u8] {
    let Some(rest) = bytes.strip_prefix(SHAPE_HEADER.as_bytes()) else {
        return bytes;
    };
    let line_end = rest.iter().position(|byte| *byte == b'\n').unwrap_or(0);
    &rest[line_end + 1..]
}

#[test]
fn a_shape_file_header_line_is_skipped() {
    assert_eq!(without_header(b"DBRep_DrawableShape\nbody"), b"body");
    assert_eq!(without_header(b"CASCADE"), b"CASCADE");
}
