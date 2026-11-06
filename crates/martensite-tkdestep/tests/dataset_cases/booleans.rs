//! `bop`, `bopcommon`, `bcommon`, `bfuse`, `bsection`, `bopcheck`, `mkvolume`, `sewing`, `fillet`, `unifysamedom`:
//! the modelling commands the exchange scripts call on what they read, on real API, with the session
//! defaults the commands start with.
use crate::support::session::Session;
use crate::xs_session::XsSession;
use martensite_tkbo::{
    bop_algo::{BOPAlgoBOP, BOPAlgoCheckerSI, BOPAlgoMakerVolume, BOPAlgoPaveFiller, Operation},
    brep_algo_api::BRepAlgoAPISection,
};
use martensite_tkbrep::{
    topexp::Explorer,
    topods::{self, Shape},
};
use martensite_tkernel::{
    message::{Gravity, Msg, ProgressRange, Report},
    ncollection::List,
    precision,
    tcollection::AsciiString,
};
use martensite_tkfillet::{brep_fillet_api::BRepFilletAPIMakeFillet, chfi3d::FilletShape};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::geom_abs::Shape as Continuity;
use martensite_tkshhealing::shape_upgrade::ShapeUpgradeUnifySameDomain;
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeShape, BRepBuilderAPISewing};
use std::sync::Arc;

/// `BOPTest::ReportAlerts`: warnings then errors, one line per type of alert.
fn alerts_text(report: &Arc<Report>) -> String {
    let mut text = String::new();
    for (gravity, prefix) in [(Gravity::Warning, "Warning: "), (Gravity::Fail, "Error: ")] {
        let mut passed: Vec<&'static str> = Vec::new();
        for alert in report.get_alerts(gravity) {
            let kind = alert.dynamic_type().name();
            if passed.contains(&kind) {
                continue;
            }
            passed.push(kind);
            let key: AsciiString = alert.get_message_key();
            let mut message = Msg::from_key(&key);
            text += &format!("{prefix}{}\n", message.get().expect("a message").to_utf8());
        }
    }
    text
}

impl XsSession {
    /// `bop s1 s2`: the pave filler `bopcommon` works from.
    /// Example: `let filler = session.bop("c_1", "c_2");`.
    pub fn bop(&self, first: &str, second: &str) -> BOPAlgoPaveFiller {
        println!("bop {first} {second} ");
        let mut arguments: List<Shape> = List::default();
        arguments.append(self.bound_shape(first));
        arguments.append(self.bound_shape(second));
        let mut filler = BOPAlgoPaveFiller::new();
        filler.set_arguments(&arguments);
        filler.options_mut().set_run_parallel(false);
        filler.options_mut().set_fuzzy_value(precision::confusion());
        filler.perform();
        print!("{}", alerts_text(filler.options().report()));
        filler
    }

    /// `bopcommon result`, on the filler of `bop`.
    /// Example: `session.bopcommon(&mut filler, "res1")`.
    pub fn bopcommon(&mut self, filler: &mut BOPAlgoPaveFiller, result: &str) {
        println!("bopcommon {result} ");
        if filler.options().has_errors() {
            println!(" PaveFiller has not been done");
            return;
        }
        let arguments: Vec<Shape> = filler.arguments().iter().cloned().collect();
        let mut bop = BOPAlgoBOP::new();
        bop.add_argument(&arguments[0]);
        bop.add_tool(&arguments[1]);
        bop.set_operation(Operation::Common);
        bop.options_mut().set_run_parallel(false);
        bop.set_check_inverted(true);
        bop.perform_with_filler(filler);
        print!("{}", alerts_text(bop.options().report()));
        if !bop.options().has_errors() && !bop.shape().is_null() {
            self.raster.bind(result, &bop.shape().clone());
        }
    }

    /// `bcommon`, `bfuse`: an operation with its own filler; returns what the command writes.
    /// Example: `session.bsmt("bfuse", Operation::fuse, "a", "a1", "a2")`.
    pub fn bsmt(
        &mut self,
        command: &str,
        operation: Operation,
        result: &str,
        first: &str,
        second: &str,
    ) -> String {
        println!("{command} {result} {first} {second} ");
        let mut bop = BOPAlgoBOP::new();
        bop.add_argument(&self.bound_shape(first));
        bop.add_tool(&self.bound_shape(second));
        bop.set_operation(operation);
        bop.options_mut().set_fuzzy_value(precision::confusion());
        bop.options_mut().set_run_parallel(false);
        bop.set_check_inverted(true);
        bop.perform();
        let text = alerts_text(bop.options().report());
        print!("{text}");
        if !bop.options().has_errors() && !bop.shape().is_null() {
            self.raster.bind(result, &bop.shape().clone());
        }
        text
    }

    /// `bsection result s1 s2`.
    /// Example: `session.bsection("section", "face", "fuselage")`.
    pub fn bsection(&mut self, result: &str, first: &str, second: &str) -> String {
        println!("bsection {result} {first} {second} ");
        let mut section = BRepAlgoAPISection::from_shapes(
            &self.bound_shape(first),
            &self.bound_shape(second),
            None,
            false,
        )
        .expect("a section");
        section.approximation(true);
        section.compute_pcurve_on1(true);
        section.compute_pcurve_on2(true);
        section.set_fuzzy_value(precision::confusion());
        section.set_run_parallel(false);
        section
            .build()
            .unwrap_or_else(|failure| panic!("bsection {result}: {failure:?}"));
        let mut text = Vec::new();
        if section.has_warnings() {
            section.dump_warnings(&mut text).expect("a memory stream");
        }
        if !section.is_done() {
            section.dump_errors(&mut text).expect("a memory stream");
        } else if !section.shape().is_null() {
            self.raster.bind(result, &section.shape().clone());
        }
        let text = String::from_utf8_lossy(&text).into_owned();
        print!("{text}");
        text
    }

    /// `bopcheck name`: its last line says whether the shape interferes with itself; returns what it writes.
    /// Example: `session.bopcheck("common")`.
    pub fn bopcheck(&self, name: &str) -> String {
        println!("bopcheck {name} ");
        let mut arguments: List<Shape> = List::default();
        arguments.append(self.bound_shape(name));
        let mut checker = BOPAlgoCheckerSI::new();
        checker.set_arguments(&arguments);
        checker.set_level_of_check(9);
        checker.options_mut().set_run_parallel(false);
        checker
            .options_mut()
            .set_fuzzy_value(precision::confusion());
        checker.perform();
        let mut text = alerts_text(checker.options().report());
        let structure = checker.data_structure().expect("a data structure");
        let mut count = 0;
        for pair in structure.interferences().iter() {
            let (first, second) = pair.indices();
            if structure.is_new_shape(first) || structure.is_new_shape(second) {
                continue;
            }
            text += &format!("interference: x{first} x{second} \n");
            count += 2;
        }
        if checker.options().has_errors() {
            text += "There were errors during the operation, so the list may be incomplete.\n";
        }
        if count == 0 {
            text += " This shape seems to be OK.\n";
        }
        print!("{text}");
        text
    }

    /// `mkvolume result shapes...`, with its default intersection on.
    /// Example: `session.mkvolume("fuselage", &["shell"])`.
    pub fn mkvolume(&mut self, result: &str, names: &[&str]) {
        println!("mkvolume {result} {} ", names.join(" "));
        let mut arguments: List<Shape> = List::default();
        for name in names {
            arguments.append(self.bound_shape(name));
        }
        let mut maker = BOPAlgoMakerVolume::new();
        maker.set_arguments(&arguments);
        maker.set_intersect(true);
        maker.options_mut().set_run_parallel(false);
        maker.options_mut().set_fuzzy_value(precision::confusion());
        maker.set_avoid_internal_shapes(false);
        maker.perform();
        print!("{}", alerts_text(maker.options().report()));
        if !maker.options().has_errors() {
            self.raster.bind(result, &maker.shape().clone());
        }
    }

    /// `sewing result tolerance shapes...`.
    /// Example: `session.sewing("shell", 1.0e-6, &["a_1", "a_2"])`.
    pub fn sewing(&mut self, result: &str, tolerance: f64, tolerance_word: &str, names: &[&str]) {
        println!("sewing {result} {tolerance_word} {} ", names.join(" "));
        let mut sewer = BRepBuilderAPISewing::default();
        sewer.set_tolerance(tolerance);
        sewer.set_min_tolerance(tolerance);
        sewer.set_max_tolerance(tolerance * 1000.0);
        for name in names {
            sewer.add(&self.bound_shape(name)).expect("a shape to sew");
        }
        sewer
            .perform(&ProgressRange::default())
            .unwrap_or_else(|failure| panic!("sewing {result}: {failure:?}"));
        let mut dump = Vec::new();
        sewer.dump(&mut dump).expect("a memory stream");
        print!("{}", String::from_utf8_lossy(&dump));
        self.raster.bind(result, sewer.sewed_shape());
    }

    /// `fillet result shape radius edge`, with `tolblend`'s defaults; `false` when the command returned 1.
    /// Example: `session.fillet("res1", "m", 0.01, "m_1")`.
    pub fn fillet(&mut self, result: &str, shape: &str, radius: f64, edge: &str) -> bool {
        const TOLERANCES: [f64; 6] = [1.0e-2, 1.0e-4, 1.0e-5, 1.0e-3, 1.0e-4, 1.0e-2];
        let [ta, t3d, t2d, fl, tesp, tapp_angle] = TOLERANCES;
        println!("fillet {result} {shape} {radius} {edge} ");
        let attempt = || -> Result<Option<Shape>, martensite_tkernel::standard::Failure> {
            let mut maker =
                BRepFilletAPIMakeFillet::new(&self.bound_shape(shape), FilletShape::Rational)?;
            maker.set_params(ta, tesp, t2d, t3d, t2d, fl);
            maker.set_continuity(Continuity::C1, tapp_angle);
            let mut edges =
                Explorer::new(&self.bound_shape(edge), ShapeEnum::Edge, ShapeEnum::Shape)?;
            let mut added = 0;
            while edges.more() {
                maker.add_with_radius(radius, &topods::edge(edges.current())?)?;
                added += 1;
                edges.next()?;
            }
            if added == 0 {
                return Ok(None);
            }
            maker.build(&ProgressRange::default())?;
            Ok(maker
                .is_done()
                .then(|| maker.shape().expect("a done fillet").clone()))
        };
        match attempt() {
            Ok(Some(made)) => {
                self.raster.bind(result, &made);
                true
            }
            Ok(None) => false,
            Err(failure) => {
                println!("{}{failure:?}", crate::support::rules::EXCEPTION_PREFIX);
                false
            }
        }
    }

    /// `unifysamedom result shape`: faces and edges, safe input, no internal edges.
    /// Example: `session.unifysamedom("result", "s")`.
    pub fn unifysamedom(&mut self, result: &str, shape: &str) {
        println!("unifysamedom {result} {shape} ");
        let unifier = ShapeUpgradeUnifySameDomain::default();
        unifier
            .initialize(&self.bound_shape(shape), true, true, false)
            .unwrap_or_else(|failure| panic!("unifysamedom: {failure:?}"));
        unifier.set_safe_input_mode(true);
        unifier.allow_internal_edges(false);
        unifier.set_linear_tolerance(precision::confusion());
        unifier.set_angular_tolerance(precision::angular());
        unifier
            .build()
            .unwrap_or_else(|failure| panic!("unifysamedom: {failure:?}"));
        self.raster.bind(result, &unifier.shape());
    }
}
