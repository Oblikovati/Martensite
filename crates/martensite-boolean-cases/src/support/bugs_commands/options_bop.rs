//! The whole Boolean session of a script: `bop`, the one-step commands and the argument commands
//! share the options of `bfuzzyvalue`, `brunparallel`, `bnondestructive` and `setfillhistory`, and
//! the last operation's history, which `savehistory` keeps.
use crate::common_operation::report_result_alerts;
use crate::support::{bop_session::BopOptions, commands::bound_shape, session::Session};
use martensite_tkbo::{
    bop_algo::{
        BOPAlgoBOP, BOPAlgoBuilder, BOPAlgoPaveFiller, BOPAlgoSection, BOPAlgoSplitter, Glue,
        Operation, Options,
    },
    brep_algo_api::{
        BRepAlgoAPIBuilderAlgo, BRepAlgoAPICommon, BRepAlgoAPICut, BRepAlgoAPIFuse,
        BRepAlgoAPISection,
    },
};
use martensite_tkbrep::{
    brep::BRepBuilder,
    brep_tools::History,
    topods::{Compound, Shape, ShapeIterator},
};
use martensite_tkernel::{message::Gravity, ncollection::List, osd::Timer, standard::Handle};
use martensite_tkg3d::top_abs::ShapeEnum;

fn listed(shapes: impl IntoIterator<Item = Shape>) -> List<Shape> {
    shapes.into_iter().collect()
}

/// The `Tps` line of a timed command, seconds as `%7.2f`; returns the seconds it prints.
fn written_time(timer: &Timer) -> f64 {
    let seconds = timer.elapsed_time();
    println!("  Tps: {seconds:7.2}");
    format!("{seconds:.2}").parse().unwrap()
}

/// The options, arguments, prepared intersections and last history of one script.
/// Example: `let mut bop = ScriptBop::default();`.
#[derive(Default)]
pub struct ScriptBop {
    /// The options in force; `boptions -default` is [`ScriptBop::reset_options`].
    pub options: BopOptions,
    /// `bsimplify -e`.
    pub simplify_edges: bool,
    /// `bsimplify -f`.
    pub simplify_faces: bool,
    /// `bglue 1` is shift, `bglue 2` full; off by default.
    pub glue: Glue,
    objects: Vec<Shape>,
    tools: Vec<Shape>,
    filler: Option<BOPAlgoPaveFiller>,
    history: Option<Handle<History>>,
    builder: Option<BOPAlgoBuilder>,
}

impl ScriptBop {
    /// `boptions -default`; `bop.reset_options();`.
    pub fn reset_options(&mut self) {
        self.options = BopOptions::default();
    }

    /// `bclearobjects`; `bop.clear_objects();`.
    pub fn clear_objects(&mut self) {
        self.objects.clear();
    }

    /// `bcleartools`; `bop.clear_tools();`.
    pub fn clear_tools(&mut self) {
        self.tools.clear();
    }

    /// `baddobjects names`; `bop.add_objects(s, &["a", "b"]);`.
    pub fn add_objects(&mut self, s: &dyn Session, names: &[&str]) {
        self.objects
            .extend(names.iter().map(|name| bound_shape(s, name)));
    }

    /// `baddtools names`; `bop.add_tools(s, &["b"]);`.
    pub fn add_tools(&mut self, s: &dyn Session, names: &[&str]) {
        self.tools
            .extend(names.iter().map(|name| bound_shape(s, name)));
    }

    /// `baddcompound name`: each direct child of the compound becomes an object.
    /// Example: `bop.add_compound(s, "b1");`.
    pub fn add_compound(&mut self, s: &dyn Session, name: &str) {
        let shape = bound_shape(s, name);
        let mut iterator = ShapeIterator::new(&shape, true, true);
        while let Ok(child) = iterator.value() {
            self.objects.push(child.clone());
            iterator.next();
        }
    }

    /// `eval baddtools [explode name kind]`: binds the parts and adds them as tools.
    /// Example: `bop.add_exploded_tools(s, "b1_2", ShapeEnum::Edge);`.
    pub fn add_exploded_tools(&mut self, s: &mut dyn Session, name: &str, kind: ShapeEnum) {
        for part in crate::support::commands::explode(s, name, Some(kind)) {
            self.tools.push(bound_shape(s, &part));
        }
    }

    /// How many objects and tools the session holds; `bop.argument_total()`.
    pub fn argument_total(&self) -> usize {
        self.objects.len() + self.tools.len()
    }

    /// Objects, then tools; `bop.all_arguments()`.
    pub fn all_arguments(&self) -> Vec<Shape> {
        self.objects.iter().chain(&self.tools).cloned().collect()
    }

    fn configured_filler(&self, arguments: &[Shape]) -> BOPAlgoPaveFiller {
        let mut filler = BOPAlgoPaveFiller::new();
        filler.set_arguments(&listed(arguments.iter().cloned()));
        filler.set_non_destructive(self.options.non_destructive);
        filler.set_glue(self.glue);
        let options = filler.options_mut();
        options.set_run_parallel(self.options.run_parallel);
        options.set_fuzzy_value(self.options.fuzzy);
        options.set_use_obb(false);
        filler
    }

    /// `bop first second`: replaces the prepared intersections; `bop.bop(s, "a", "b");`.
    pub fn bop(&mut self, s: &dyn Session, first: &str, second: &str) {
        let arguments = [bound_shape(s, first), bound_shape(s, second)];
        let mut filler = self.configured_filler(&arguments);
        filler.perform();
        report_result_alerts(filler.options());
        self.objects = vec![arguments[0].clone()];
        self.tools = vec![arguments[1].clone()];
        self.filler = Some(filler);
    }

    /// `bfillds`: prepares the intersections of every argument; `bop.bfillds();`.
    pub fn bfillds(&mut self) {
        let mut filler = self.configured_filler(&self.all_arguments());
        filler.perform();
        report_result_alerts(filler.options());
        self.filler = Some(filler);
    }

    /// `regexp "Warning" [bfillds]`: prepares the intersections and returns what the command writes.
    /// Example: `bop.bfillds_answer().contains("Warning")`.
    pub fn bfillds_answer(&mut self) -> String {
        let mut filler = self.configured_filler(&self.all_arguments());
        filler.perform();
        let mut answer = String::new();
        for (gravity, prefix) in [(Gravity::Warning, "Warning: "), (Gravity::Fail, "Error: ")] {
            answer += &crate::common_operation::case_bop_alerts::gravity_alert_text(
                filler.options().report(),
                gravity,
                prefix,
            );
        }
        print!("{answer}");
        self.filler = Some(filler);
        answer
    }

    /// `boporigin name`: binds `name_or` to the shape's origins; writes the command's lines.
    /// Example: `bop.boporigin(s, "r_1");`.
    pub fn boporigin(&self, s: &mut dyn Session, name: &str) {
        let shape = bound_shape(s, name);
        let builder = self.builder.as_ref().expect("bbuild before boporigin");
        let Ok(found) = builder.origins().find(&shape) else {
            println!(" no origins found");
            return;
        };
        let origins: Vec<Shape> = found.iter().cloned().collect();
        let target = format!("{name}_or");
        if let [only] = origins.as_slice() {
            s.bind(&target, only);
            println!("1 origin found\n{target}");
            return;
        }
        let bindable = BRepBuilder::default();
        let mut whole = Compound::default();
        bindable.make_compound(&mut whole);
        let mut whole = whole.into_shape();
        for origin in &origins {
            bindable.add(&mut whole, origin).unwrap();
        }
        s.bind(&target, &whole);
        println!("{} origins found\n{target}", origins.len());
    }

    /// `bfillds -t`: writes the time the intersections took; `bop.bfillds_timed();`.
    pub fn bfillds_timed(&mut self) -> f64 {
        let mut timer = Timer::new(false);
        timer.start();
        self.bfillds();
        timer.stop();
        written_time(&timer)
    }

    /// The intersections `bfillds` prepared; `bop.prepared()`.
    pub fn prepared(&mut self) -> &mut BOPAlgoPaveFiller {
        self.filler
            .as_mut()
            .unwrap_or_else(|| panic!("this command needs the prepared intersections of bfillds"))
    }

    /// `bbop name op`: `Section` runs on every argument; `bop.bbop(s, "r", Operation::Fuse);`.
    pub fn bbop(&mut self, s: &mut dyn Session, name: &str, kind: Operation) {
        if kind == Operation::Section {
            self.section(s, name);
            return;
        }
        let mut operation = BOPAlgoBOP::new();
        for object in &self.objects {
            operation.add_argument(object);
        }
        for tool in &self.tools {
            operation.add_tool(tool);
        }
        operation.set_operation(kind);
        operation
            .options_mut()
            .set_run_parallel(self.options.run_parallel);
        operation.set_check_inverted(true);
        operation.set_to_fill_history(self.options.fill_history);
        operation.perform_with_filler(self.prepared());
        self.history = operation.history();
        publish(s, name, operation.options(), operation.shape());
    }

    /// `bopsection name`, or `bbop name 4`: the section of every argument.
    /// Example: `bop.section(s, "result");`.
    pub fn section(&mut self, s: &mut dyn Session, name: &str) {
        let mut operation = BOPAlgoSection::new();
        operation.set_arguments(&listed(self.all_arguments()));
        operation
            .options_mut()
            .set_run_parallel(self.options.run_parallel);
        operation.set_check_inverted(true);
        operation.set_to_fill_history(self.options.fill_history);
        operation.perform_with_filler(self.prepared());
        self.history = operation.history();
        publish(s, name, operation.options(), operation.shape());
    }

    /// `bbuild name`: the general fuse of every argument; `bop.bbuild(s, "rgf");`.
    pub fn bbuild(&mut self, s: &mut dyn Session, name: &str) {
        let mut builder = BOPAlgoBuilder::new();
        builder.set_arguments(&listed(self.all_arguments()));
        builder
            .options_mut()
            .set_run_parallel(self.options.run_parallel);
        builder.set_check_inverted(true);
        builder.set_to_fill_history(self.options.fill_history);
        builder.perform_with_filler(self.prepared());
        self.history = builder.history();
        publish(s, name, builder.options(), builder.shape());
        self.builder = Some(builder);
    }

    /// `bbuild name -t`: returns the time it writes; `bop.bbuild_timed(s, "result")`.
    pub fn bbuild_timed(&mut self, s: &mut dyn Session, name: &str) -> f64 {
        let mut timer = Timer::new(false);
        timer.start();
        self.bbuild(s, name);
        timer.stop();
        written_time(&timer)
    }

    /// `bbop name op -t`: returns the time it writes; `bop.bbop_timed(s, "r", Operation::Section)`.
    pub fn bbop_timed(&mut self, s: &mut dyn Session, name: &str, kind: Operation) -> f64 {
        let mut timer = Timer::new(false);
        timer.start();
        self.bbop(s, name, kind);
        timer.stop();
        written_time(&timer)
    }

    /// `bsplit name`: the objects split by the tools; `bop.bsplit(s, "result");`.
    pub fn bsplit(&mut self, s: &mut dyn Session, name: &str) {
        let mut splitter = BOPAlgoSplitter::new();
        splitter.set_arguments(&listed(self.objects.iter().cloned()));
        splitter.set_tools(&listed(self.tools.iter().cloned()));
        splitter
            .options_mut()
            .set_run_parallel(self.options.run_parallel);
        splitter.set_non_destructive(self.options.non_destructive);
        splitter.options_mut().set_fuzzy_value(self.options.fuzzy);
        splitter.set_check_inverted(true);
        splitter.set_to_fill_history(self.options.fill_history);
        splitter.perform_with_filler(self.prepared());
        self.history = splitter.history();
        publish(s, name, splitter.options(), splitter.shape());
    }

    /// `bcommon|bfuse|bcut|btuc name first second` under the session's options.
    /// Example: `bop.direct(s, "r", "c1", "c2", Operation::Common);`.
    pub fn direct(
        &mut self,
        s: &mut dyn Session,
        name: &str,
        (first, second): (&str, &str),
        kind: Operation,
    ) {
        let mut operation = BOPAlgoBOP::new();
        operation.add_argument(&bound_shape(s, first));
        operation.add_tool(&bound_shape(s, second));
        operation.set_operation(kind);
        operation.set_glue(self.glue);
        operation.options_mut().set_fuzzy_value(self.options.fuzzy);
        operation.set_non_destructive(self.options.non_destructive);
        operation
            .options_mut()
            .set_run_parallel(self.options.run_parallel);
        operation.options_mut().set_use_obb(false);
        operation.set_check_inverted(true);
        operation.set_to_fill_history(self.options.fill_history);
        operation.perform();
        self.history = operation.history();
        report_result_alerts(operation.options());
        if operation.options().has_errors() {
            return;
        }
        s.bind(name, operation.shape());
    }

    /// `bapibop name op` on the session's objects and tools; the tools come first for `Cut21`.
    /// Example: `bop.bapibop(s, "r", Operation::Cut);`.
    pub fn bapibop(&mut self, s: &mut dyn Session, name: &str, kind: Operation) {
        let (first, second) = if kind == Operation::Cut21 {
            (self.tools.clone(), self.objects.clone())
        } else {
            (self.objects.clone(), self.tools.clone())
        };
        let (mut common, mut fuse, mut cut);
        let operation = match kind {
            Operation::Common => {
                common = BRepAlgoAPICommon::new();
                &mut *common
            }
            Operation::Fuse => {
                fuse = BRepAlgoAPIFuse::new();
                &mut *fuse
            }
            Operation::Cut | Operation::Cut21 => {
                cut = BRepAlgoAPICut::new();
                &mut *cut
            }
            other => panic!("bapibop {name} {other:?}: expected a Boolean operation"),
        };
        operation.set_arguments(&listed(first));
        operation.set_tools(&listed(second));
        self.apply_api_options(operation);
        operation
            .build()
            .unwrap_or_else(|failure| panic!("bapibop {name}: {failure:?}"));
        self.finish_api(s, name, operation);
    }

    fn apply_api_options(&self, builder: &mut BRepAlgoAPIBuilderAlgo) {
        builder.set_run_parallel(self.options.run_parallel);
        builder.set_fuzzy_value(self.options.fuzzy);
        builder.set_non_destructive(self.options.non_destructive);
        builder.set_glue(self.glue);
        builder.set_check_inverted(true);
        builder.set_use_obb(false);
        builder.set_to_fill_history(self.options.fill_history);
    }

    fn finish_api(
        &mut self,
        s: &mut dyn Session,
        name: &str,
        builder: &mut BRepAlgoAPIBuilderAlgo,
    ) {
        builder
            .simplify_result(self.simplify_edges, self.simplify_faces, 1.0e-12)
            .unwrap_or_else(|failure| panic!("bsimplify {name}: {failure:?}"));
        self.history = builder.history().cloned();
        report_api_alerts(builder);
        if builder.has_errors() {
            return;
        }
        let shape = builder.shape().clone();
        if shape.is_null() {
            println!("Result is a null shape");
            return;
        }
        s.bind(name, &shape);
    }

    /// `bbop name op` read back by a script's regexp: returns the alerts it writes.
    /// Example: `bop.bbop_answer(s, "r", Operation::Fuse)`.
    pub fn bbop_answer(&mut self, s: &mut dyn Session, name: &str, kind: Operation) -> String {
        let mut operation = BOPAlgoBOP::new();
        for object in &self.objects {
            operation.add_argument(object);
        }
        for tool in &self.tools {
            operation.add_tool(tool);
        }
        operation.set_operation(kind);
        operation
            .options_mut()
            .set_run_parallel(self.options.run_parallel);
        operation.set_check_inverted(true);
        operation.set_to_fill_history(self.options.fill_history);
        operation.perform_with_filler(self.prepared());
        let mut answer = String::new();
        for (gravity, prefix) in [(Gravity::Warning, "Warning: "), (Gravity::Fail, "Error: ")] {
            answer += &crate::common_operation::case_bop_alerts::gravity_alert_text(
                operation.options().report(),
                gravity,
                prefix,
            );
        }
        print!("{answer}");
        if !operation.options().has_errors() {
            s.bind(name, operation.shape());
        }
        answer
    }

    /// `bapibop name 4` on the session's objects and tools; `bop.bapisection(s, "r4_3")`.
    pub fn bapisection(&mut self, s: &mut dyn Session, name: &str) {
        let mut section = BRepAlgoAPISection::new();
        section.set_arguments(&listed(self.objects.clone()));
        section.set_tools(&listed(self.tools.clone()));
        self.apply_api_options(&mut section);
        section
            .build()
            .unwrap_or_else(|failure| panic!("bapibop {name}: {failure:?}"));
        self.finish_api(s, name, &mut section);
    }

    /// `savehistory`: the history of the last operation, none when it kept none.
    /// Example: `let h = bop.saved_history();`.
    pub fn saved_history(&self) -> Option<Handle<History>> {
        self.history.clone()
    }
}

pub fn report_api_alerts(builder: &BRepAlgoAPIBuilderAlgo) {
    let mut text = Vec::new();
    builder.dump_warnings(&mut text).unwrap();
    builder.dump_errors(&mut text).unwrap();
    print!("{}", String::from_utf8_lossy(&text));
}

/// The end of each result command: its alerts, then the binding unless it failed or came out null.
fn publish(s: &mut dyn Session, name: &str, options: &Options, shape: &Shape) {
    report_result_alerts(options);
    if options.has_errors() {
        return;
    }
    if shape.is_null() {
        println!("Result is a null shape");
        return;
    }
    s.bind(name, shape);
}

pub use super::history_query::{generated, isdeleted, modified};
pub use super::section_flags::{SectionFlags, bsection_with};
