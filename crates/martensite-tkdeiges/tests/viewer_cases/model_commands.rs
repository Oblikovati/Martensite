//! The commands of the shaded scripts that are neither the exchange nor the viewer's own: `nproject`,
//! `prism`, `viso`, `xbounds`, `triarea`, `dump` of a curve, the booleans of `bug26047`.
use crate::host::ViewerHost;
use martensite_tkbrep::{
    brep::BRepTool,
    brep_tools,
    topexp::Explorer,
    topods::{self, Shape},
};
use martensite_tkernel::{
    message::ProgressRange, ncollection::List, standard::Handle, storage::format_general,
};
use martensite_tkg3d::{geom::GeomBSplineCurve, top_abs::ShapeEnum};
use martensite_tkgeombase::gprop::GProps;
use martensite_tkmath::{
    geom_abs::Shape as Continuity,
    gp::{Direction3, Vector3},
    poly::mesh_purpose,
};
use martensite_tkoffset::brep_offset_api::BRepOffsetAPINormalProjection;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakePrism;
use martensite_tktopalgo::{brep_builder_api::BRepBuilderAPIMakeShape as _, brep_gprop::BRepGProp};

fn faces_of(shape: &Shape) -> Vec<topods::Face> {
    let mut found = Vec::new();
    let mut faces = Explorer::new(shape, ShapeEnum::Face, ShapeEnum::Shape).unwrap();
    while faces.more() {
        found.push(topods::face(faces.current()).unwrap());
        faces.next().unwrap();
    }
    found
}

impl ViewerHost {
    /// `nproject result edge face`: the edge projected along the normals of the face.
    pub fn nproject(&mut self, result: &str, edge: &str, face: &str) {
        println!("nproject {result} {edge} {face} ");
        const TOLERANCE: f64 = 1.0e-4;
        const MAX_DEGREE: i32 = 14;
        const MAX_SEGMENTS: i32 = 16;
        let mut projection = BRepOffsetAPINormalProjection::new();
        projection.init(&self.shape(face));
        projection.add(&self.shape(edge)).unwrap();
        projection.set_params(
            TOLERANCE,
            TOLERANCE.powf(2.0 / 3.0),
            Continuity::C1,
            MAX_DEGREE,
            MAX_SEGMENTS,
        );
        projection.build(&ProgressRange::default()).unwrap();
        let mut wires = List::default();
        if projection.build_wire(&mut wires).unwrap() {
            println!(" BuildWire OK ");
        }
        let projected = projection.shape().unwrap().clone();
        self.bind(result, &projected);
    }

    /// `prism result base dx dy dz word`: an `Inf` word makes it infinite, any other a plain vector.
    pub fn prism(&mut self, result: &str, base: &str, along: [f64; 3], word: &str) {
        println!("prism {result} {base} 0 1 0 {word} ");
        let shape = self.shape(base);
        let vector = Vector3::new(along[0], along[1], along[2]);
        let mut maker = if word.starts_with(['i', 'I']) {
            let unit = Direction3::from_vector(vector).unwrap();
            BRepPrimAPIMakePrism::infinite(&shape, unit, true, false, true).unwrap()
        } else {
            BRepPrimAPIMakePrism::new(&shape, vector, false, true).unwrap()
        };
        let made = maker.shape().unwrap().clone();
        self.bind(result, &made);
    }

    /// `xbounds name`: the parameter range of a face.
    pub fn xbounds(&self, name: &str) {
        println!("xbounds {name} ");
        let face = faces_of(&self.shape(name)).remove(0);
        let (u_min, u_max, v_min, v_max) = brep_tools::uv_bounds(&face).unwrap();
        for value in [u_min, u_max, v_min, v_max] {
            println!("{}", format_general(value, 6));
        }
    }

    /// `triarea name eps`: the area of the triangles, then the area of the faces at that precision.
    pub fn triarea(&self, name: &str, epsilon: f64, epsilon_word: &str) -> (f64, f64) {
        println!("triarea {name} {epsilon_word} ");
        let shape = self.shape(name);
        let mut of_triangles = 0.0;
        for face in faces_of(&shape) {
            let (mesh, location) = BRepTool::triangulation(&face, mesh_purpose::NONE).unwrap();
            let Some(mesh) = mesh.get() else {
                continue;
            };
            let moved = location.transformation();
            for at in 1..=mesh.nb_triangles() {
                let triangle = mesh.triangle(at).unwrap();
                let [a, b, c] = [1, 2, 3].map(|corner| {
                    let node = triangle.value(corner).unwrap();
                    mesh.node(node).unwrap().transformed(moved)
                });
                let first = Vector3::between(a, b);
                let second = Vector3::between(a, c);
                of_triangles += first.crossed(second).magnitude() * 0.5;
            }
        }
        let mut properties = GProps::new();
        BRepGProp::surface_properties_eps(&shape, &mut properties, epsilon, false).unwrap();
        let of_faces = properties.mass();
        println!(
            "{} {}",
            format_general(of_triangles, 17),
            format_general(of_faces, 17)
        );
        (of_triangles, of_faces)
    }

    /// `dump name` of a B-spline curve, as the drawable writes it.
    pub fn dump_bspline_curve(&self, name: &str, curve: &Handle<GeomBSplineCurve>) {
        let live = curve.get().unwrap();
        println!("dump {name} ");
        println!("\n\n*********** Dump of {name} *************\n");
        println!("BSplineCurve");
        println!(
            "  Degree {}, {} Poles, {}  Knots",
            live.degree(),
            live.nb_poles(),
            live.nb_knots()
        );
        println!("Poles :\n");
        for at in 1..=live.nb_poles() {
            let pole = live.pole(at).unwrap();
            println!(
                "{at:>4} : {}, {}, {} ",
                format_general(pole.x(), 10),
                format_general(pole.y(), 10),
                format_general(pole.z(), 10)
            );
        }
        println!("Knots :\n");
        for at in 1..=live.nb_knots() {
            println!(
                "{at:>4} :  {} {}",
                format_general(live.knot(at).unwrap(), 10),
                live.multiplicity(at).unwrap()
            );
        }
    }
}
