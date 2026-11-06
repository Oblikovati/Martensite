//! The shape commands of the scripts beyond the dataset cases': each on the library's own API, with
//! what it writes to the stream.
use crate::host::ViewerHost;
use crate::support::checkprops::{CheckTolerances, Expected, checkprops_within};
use crate::support::nbshapes::{Counting, NbCount, checknbshapes_named};
use crate::support::properties::{Prop, checkgravitycenter_of_area};
use crate::support::session::Session as _;
use crate::support::tricheck::tricheck;
use martensite_tkbrep::brep::BRepBuilder;
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::brep_tools;
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods;
use martensite_tkbrep::topods::{Compound, Face, Shape};
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::standard::Handle;
use martensite_tkernel::storage::IoStream;
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum};
use martensite_tkmath::bnd::BoundingBox3;
use martensite_tkmath::gp::{Point3, Transform3, Vector3};
use martensite_tkmath::poly::{MergeNodesTool, Triangulation, mesh_purpose};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tkshhealing::shape_fix::ShapeFixShape;
use martensite_tktopalgo::brep_bnd_lib::BRepBndLib;
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeShape as _, BRepBuilderAPITransform,
};
use std::collections::BTreeSet;
use std::io::{Read as _, Seek as _, SeekFrom};

impl ViewerHost {
    /// `checkprops name -s value`, within its default deviation.
    pub fn checkprops_surface(&self, name: &str, value: f64) {
        println!("checkprops {name} -s {value} ");
        checkprops_within(
            &self.session.raster,
            name,
            Prop::Area,
            Expected::Mass(value),
            CheckTolerances::default(),
        );
    }

    /// `checkshape name`.
    pub fn checkshape(&self, name: &str) {
        println!("checkshape {name} ");
        self.session.check_shape(name);
    }

    /// `checknbshapes name -kind count ...`.
    pub fn checknbshapes(&self, name: &str, words: &[&str]) {
        println!("checknbshapes {name} {} ", words.join(" "));
        self.session.check_counts(name, words);
    }

    /// `checknbshapes name -kind count ... -t -m message`.
    pub fn checknbshapes_typed(&self, name: &str, message: &str, counts: &[NbCount]) {
        checknbshapes_named(&self.session.raster, name, Counting::Typed, counts, message);
    }

    /// `checkgravitycenter name -s x y z tol`, in the case's words.
    pub fn checkgravitycenter_surface(&self, name: &str, expected: [&str; 3], tolerance: &str) {
        checkgravitycenter_of_area(name, &self.shape(name), expected, tolerance);
    }

    /// `bounding name`: xmin ymin zmin xmax ymax zmax, with the triangulation when there is one.
    pub fn bounding(&self, name: &str) -> [f64; 6] {
        println!("bounding {name} ");
        let mut bounds = BoundingBox3::default();
        BRepBndLib::add(&self.shape(name), &mut bounds, true).unwrap();
        let box_values = bounds.get().unwrap();
        let text: Vec<String> = box_values
            .iter()
            .map(|value| crate::view_commands::tcl_real(*value))
            .collect();
        println!("{}", text.join(" "));
        box_values
    }

    /// `tclean name`.
    pub fn tclean(&self, name: &str) {
        println!("tclean {name} ");
        brep_tools::clean(&self.shape(name), false).unwrap();
    }

    /// `tricheck name`: what the command returns.
    pub fn tricheck(&self, name: &str) -> String {
        tricheck(name, &self.shape(name))
    }

    /// `incmesh name value`.
    pub fn incmesh(&self, name: &str, deflection: &str) {
        println!("incmesh {name} {deflection} ");
        crate::meshing::incmesh(&self.session.raster, &[name, deflection]);
    }

    /// `checktrinfo name -tri value -nod value`, words after the name.
    pub fn checktrinfo(&self, name: &str, words: &[&str]) {
        println!("checktrinfo {name} {}", words.join(" "));
        let mut all = vec![name];
        all.extend_from_slice(words);
        crate::meshing::checktrinfo(&self.session.raster, &all);
    }

    /// `fixshape result shape precision`.
    pub fn fixshape(&mut self, result: &str, name: &str, precision: f64) {
        println!("fixshape {result} {name} {precision} ");
        let fix = ShapeFixShape::default();
        fix.init(self.shape(name));
        fix.set_precision(precision);
        fix.perform_with_progress(&ProgressRange::default())
            .unwrap();
        self.bind(result, &fix.shape().clone());
    }

    /// `dump name`: the shape's text.
    pub fn dump_shape(&self, name: &str) {
        println!("dump {name} ");
        println!("\n\n*********** Dump of {name} *************\n");
        let mut stream = IoStream::from_bytes(Vec::new());
        stream.set_precision(15);
        brep_tools::dump(&self.shape(name), &mut stream).unwrap();
        let mut device = stream.into_device().expect("a memory device");
        device.seek(SeekFrom::Start(0)).unwrap();
        let mut text = String::new();
        device.read_to_string(&mut text).unwrap();
        println!("{text}");
    }

    /// `box name dx dy dz` at the origin of its corner: `box name x y z dx dy dz`.
    pub fn box_at(&mut self, name: &str, corner: [f64; 3], size: [f64; 3]) {
        println!(
            "box {name} {} {} {} {} {} {} ",
            corner[0], corner[1], corner[2], size[0], size[1], size[2]
        );
        let mut made = BRepPrimAPIMakeBox::at_point(
            Point3::new(corner[0], corner[1], corner[2]),
            size[0],
            size[1],
            size[2],
        )
        .unwrap();
        self.bind(name, &made.solid().unwrap().into_shape());
    }

    /// `compound a b ... result`.
    pub fn compound(&mut self, parts: &[&str], result: &str) {
        println!("compound {} {result} ", parts.join(" "));
        let builder = BRepBuilder::default();
        let mut compound = Compound::default();
        builder.make_compound(&mut compound);
        let mut whole = compound.into_shape();
        for part in parts {
            builder.add(&mut whole, &self.shape(part)).unwrap();
        }
        self.bind(result, &whole);
    }

    /// `copy name result`: a new variable over the same shape, not a copy of the geometry.
    pub fn copy_shape(&mut self, name: &str, result: &str) {
        println!("copy {name} {result} ");
        self.session.copy_variable(name, result);
    }

    /// `ttranslate name dx dy dz`: the shape moves by a location.
    pub fn ttranslate(&mut self, name: &str, by: [f64; 3]) {
        println!("ttranslate {name} {} {} {} ", by[0], by[1], by[2]);
        let mut move_by = Transform3::default();
        move_by.set_translation(Vector3::new(by[0], by[1], by[2]));
        let mut moved =
            BRepBuilderAPITransform::with_shape(&self.shape(name), move_by, false, false).unwrap();
        self.bind(name, &moved.shape().unwrap().clone());
    }

    /// `trmergenodes name -angle degrees -oneFace result`: one face over a merged triangulation.
    pub fn trmergenodes_one_face(&mut self, name: &str, degrees: f64, result: &str) {
        println!("trmergenodes {name} -angle {degrees} -oneFace {result} ");
        let shape = self.shape(name);
        let (mut old_nodes, mut old_triangles) = (0, 0);
        let mut tool = MergeNodesTool::new(degrees * std::f64::consts::PI / 180.0, 0.0, 0);
        let mut faces = Explorer::new(&shape, ShapeEnum::Face, ShapeEnum::Shape).unwrap();
        while faces.more() {
            let face: Face = topods::face(faces.current()).unwrap();
            let (mesh, location) = BRepTool::triangulation(&face, mesh_purpose::NONE).unwrap();
            if let Some(live) = usable_mesh(&mesh) {
                old_nodes += live.nb_nodes();
                old_triangles += live.nb_triangles();
                let is_reversed = face.orientation() == Orientation::Reversed;
                tool.add_triangulation(&mesh, location.transformation(), is_reversed)
                    .unwrap();
            }
            faces.next().unwrap();
        }
        let merged = tool.result().unwrap();
        let live = merged.get().expect("a merged triangulation");
        let (new_nodes, new_triangles) = (live.nb_nodes(), live.nb_triangles());
        self.bind(result, &face_of_mesh(&merged));
        println!(
            "Old, Triangles: {old_triangles}, Nodes: {old_nodes}\nNew, Triangles: {new_triangles}, Nodes: {new_nodes}"
        );
    }

    /// `dall`: the case's shape variables go, so a later `isdraw` of one is false.
    /// Example: `host.dall(&["b1", "b2"])`.
    pub fn dall(&mut self, names: &[&str]) {
        println!("dall ");
        for name in names {
            self.session.raster.unbind(name);
        }
    }
}

/// `BRepBuilder::make_face(face, mesh)`: a face with no surface over the mesh.
pub fn face_of_mesh(mesh: &Handle<Triangulation>) -> Shape {
    let builder = BRepBuilder::default();
    let mut face = Face::default();
    builder.make_face(&mut face);
    builder
        .update_face_triangulation(&face, mesh, true)
        .unwrap();
    face.into()
}

fn usable_mesh(mesh: &Handle<Triangulation>) -> Option<&Triangulation> {
    mesh.get()
        .filter(|live| live.nb_nodes() >= 3 && live.nb_triangles() >= 1)
}

/// Keeps a set in explode order for the assembly comparisons; the set of names a script expects.
pub fn same_shape(first: &Shape, second: &Shape) -> bool {
    first.is_same(second)
}

/// `lsearch $info unknown` over a command's result.
pub fn mentions_unknown(text: &str) -> bool {
    let words: BTreeSet<&str> = text.split_whitespace().collect();
    words.contains("unknown")
}
