//! `lowalgos_classifier` and intss, `geometry_revsurf` and `perf_modalg` case
//! bodies whose inputs are in the public
//! dataset.
use crate::support::{
    checkshape::checkshape,
    commands::{dump_surface, explode_type, mksurface},
    dataset::located_bytes,
    distance::b2dclassify,
    log,
    props::{Prop, checkprops},
    qa::occ377,
    restore::restore_as,
    session::{Session, checkview_display, set, set_exploded},
    text::llength,
    topology::{checksection, mkface},
};
use martensite_tkbrep::{brep::BRepBuilder, brep_tools, topods::Shape};
use martensite_tkernel::{
    message::ProgressRange, standard::Handle, storage::IoStream, storage::format_general,
};
use martensite_tkg3d::{
    geom::{GeomSurface, GeomSurfaceOfRevolution},
    top_abs::{ShapeEnum, State},
};
use martensite_tkgeomalgo::geom_int::{GeomIntIntSS, GeomIntIntSSApprox};
use martensite_tkmath::{
    bnd::BoundingBox3,
    gp::{Axis1, Direction3, Line3, Point2, Point3},
};
use martensite_tktopalgo::{
    brep_bnd_lib::BRepBndLib, int_curves_face::IntCurvesFaceShapeIntersector,
};

/// `lowalgos_classifier`.
pub mod classifier {
    use super::*;

    /// Both points are IN the face (a failure message names an undefined
    /// variable, which is a script error).
    pub fn buc60609(s: &mut dyn Session) {
        let a = restore_as(s, "buc60609a.brep", "a").shape();
        let p1 = set(s, "p1", Point2::new(0.14, 5.1));
        let p2 = set(s, "p2", Point2::new(1.28, 5.1));
        for (point, which) in [(p1, "first"), (p2, "second")] {
            if b2dclassify(&a, point) != State::In {
                panic!("can't read \"{which}\": no such variable");
            }
            log::puts(&format!("BUC60609 {which} OK:  It is INSIDE"));
        }
    }

    /// The point is ON the first face.
    pub fn bug23849_2(s: &mut dyn Session) {
        let b1 = restore_as(s, "bug23849_segment_1.brep", "b1").shape();
        let faces = set_exploded(s, "b1", explode_type(&b1, ShapeEnum::Face));
        let p2d = set(
            s,
            "p2D",
            Point2::new(4.1882396063720497, 50.643343428894831),
        );
        if b2dclassify(&faces[0], p2d) == State::On {
            log::puts("OK OCC23849");
        } else {
            log::puts("Faulty OCC23849");
        }
    }

    /// BRepTopAdaptor_FClass2d gives the same state for each wire and its
    /// reversal (the command reads the file itself with BRepTools::Read).
    pub fn bug377(s: &mut dyn Session) {
        let mut shape = Shape::default();
        let mut stream = IoStream::from_bytes(located_bytes("OCC377.brep"));
        brep_tools::read(
            &mut shape,
            &mut stream,
            &BRepBuilder::default(),
            &ProgressRange::default(),
        )
        .unwrap();
        let result_info = occ377(&shape, Point2::new(1.9812, -0.9557948), 6.45);
        if result_info.contains("FAULTY") {
            log::puts("Error : OCC377");
        }
        checkview_display(s, None, "bug377");
    }

    /// The point is not classified IN.
    pub fn bug538(s: &mut dyn Session) {
        log::todo("Faulty OCC132");
        let face = restore_as(s, "OCC538.brep", "Face").shape();
        checkshape(&face);
        let p = set(s, "p", Point2::new(0.2453959532714904, -1.246869357124329));
        let state = b2dclassify(&face, p);
        let res1 = crate::support::distance::state_text(state);
        if llength(res1) < 4 || state != State::In {
            log::puts("Faulty OCC132");
        } else {
            log::puts("OCC132 : OK");
        }
    }
}

/// `lowalgos_intss`.
pub mod intss {
    use super::*;

    /// `intersect name s1 s2`: GeomInt_IntSS (Confusion, approximated); `false` is the
    /// command's `No intersections found!` (a script error).
    fn intersect_surfaces(
        s: &mut dyn Session,
        name: &str,
        s1: &Handle<dyn GeomSurface>,
        s2: &Handle<dyn GeomSurface>,
    ) {
        let mut inters = GeomIntIntSS::new();
        inters
            .perform(
                s1,
                s2,
                martensite_tkernel::precision::confusion(),
                GeomIntIntSSApprox::default(),
            )
            .unwrap();
        assert!(inters.is_done(), "No intersections found!");
        let nb_lines = inters.nb_lines();
        let mut text = String::new();
        if nb_lines >= 2 {
            for i in 1..=nb_lines {
                set(s, &format!("{name}_{i}"), inters.line(i).unwrap());
                text.push_str(&format!("{name}_{i} "));
            }
        } else if nb_lines == 1 {
            set(s, name, inters.line(1).unwrap());
            text.push_str(&format!("{name} "));
        }
        for i in 1..=inters.nb_points().unwrap() {
            text.push_str(&format!("{name}_p_{i} "));
        }
        log::result(&text);
    }

    /// Both intersections of the two surfaces succeed; `checksection` of the
    /// curve variable (not a shape) finds no alone vertex.
    pub fn buc60623_3(s: &mut dyn Session) {
        let a = restore_as(s, "buc60623a.brep", "a").shape();
        checkshape(&a);
        let b = restore_as(s, "buc60623b.brep", "b").shape();
        checkshape(&b);
        let s1 = set(s, "S1", mksurface(&a));
        let s2 = set(s, "S2", mksurface(&b));
        for (name, x, y) in [("I1", &s1, &s2), ("I2", &s2, &s1)] {
            intersect_surfaces(s, name, x, y);
            // DBRep::Get of a curve variable is a null shape: no vertex.
            if checksection(&Shape::default()) != 0 {
                log::puts(" Faulty : Intersection is not correct");
            }
        }
        checkview_display(s, None, "buc60623_3");
    }
}

/// `geometry_revsurf`.
pub mod revsurf {
    use super::*;

    /// A1: the surface of revolution of the B-spline about (1, 0, 0) Z, its D2 values
    /// (printed), its face and area.
    pub fn a1(s: &mut dyn Session) {
        let base = restore_as(s, "base_revol", "base_revol").curve();
        let axis = Axis1::new(
            Point3::new(1., 0., 0.),
            Direction3::new(0., 0., 1.).unwrap(),
        );
        let revolution: Handle<dyn GeomSurface> =
            Handle::new(GeomSurfaceOfRevolution::new(&base, axis).unwrap()).cast();
        let revolution = set(s, "revolution1", revolution);
        let surface = revolution.get().unwrap();
        let mut dvx = 0.0;
        for (u, prefix) in [(0.0, "dv"), (std::f64::consts::PI, "dov")] {
            let d2 = surface.d2(u, 1.0).unwrap();
            let values = [
                ("x", d2.point.x()),
                ("y", d2.point.y()),
                ("z", d2.point.z()),
                ("dux", d2.d1u.x()),
                ("duy", d2.d1u.y()),
                ("duz", d2.d1u.z()),
                (&format!("{prefix}x"), d2.d1v.x()),
                (&format!("{prefix}y"), d2.d1v.y()),
                (&format!("{prefix}z"), d2.d1v.z()),
            ];
            for (name, value) in values {
                log::result(&format!(
                    "\n\n*********** Dump of {name} *************\n{}",
                    format_general(value, 15)
                ));
            }
            if prefix == "dv" {
                dvx = d2.d1v.x();
            } else {
                log::result(&format_general(d2.d1v.x() + dvx, 15));
            }
        }
        log::result(&dump_surface("revolution1", &revolution));
        let result = set(s, "result", mkface(&revolution));
        checkprops(&result, Prop::Area, 13.8136);
        checkview_display(s, Some("result"), "A1");
    }
}

/// `perf_modalg`.
pub mod perf {
    use super::*;

    /// IntCurvesFace_ShapeIntersector::PerformNearest of 251 x 251 lines along
    /// Y over the bounding box must end within the cpulimit (the chronometer is only
    /// printed).
    pub fn bug25413(s: &mut dyn Session) {
        let w = restore_as(s, "bug25413.brep", "w").shape();
        let mut inter = IntCurvesFaceShapeIntersector::new();
        inter
            .load(&w, martensite_tkernel::precision::confusion())
            .unwrap();
        let mut bounds = BoundingBox3::default();
        BRepBndLib::add(&w, &mut bounds, true).unwrap();
        let [x_min, _, z_min, x_max, _, z_max] = bounds.get().unwrap();
        let n = 250.0;
        let (x_step, z_step) = ((x_max - x_min) / n, (z_max - z_min) / n);
        let direction = Direction3::new(0., 1., 0.).unwrap();
        let start = std::time::Instant::now();
        let mut x = x_min;
        while x <= x_max {
            let mut z = z_min;
            while z <= z_max {
                let line = Line3::new(Point3::new(x, 0.0, z), direction);
                inter.perform_nearest(&line, -100., 100.).unwrap();
                z += z_step;
            }
            x += x_step;
        }
        log::puts(&format!(
            "COUNTER OCC25413: {}",
            start.elapsed().as_secs_f64()
        ));
    }
}
