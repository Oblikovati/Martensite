//! The boolean commands that share one session: `baddobjects`, `baddtools`, `bfillds`, `bbop`,
//! `bbuild` and `bsplit`, with the options `bfuzzyvalue`, `bnondestructive`, `brunparallel` and
//! `setfillhistory` set before them and `boptions -default` after.
use super::session::Session;
use crate::common_operation::{case_bop_alerts::gravity_alert_text, report_result_alerts};
use martensite_tkbo::bop_algo::{
    BOPAlgoBOP, BOPAlgoBuilder, BOPAlgoPaveFiller, BOPAlgoSection, BOPAlgoSplitter, Glue,
    Operation, Options,
};
use martensite_tkbo::brep_algo_api::BRepAlgoAPICut;
use martensite_tkbrep::{brep_tools::History, topods::Shape};
use martensite_tkernel::{message::Gravity, ncollection::List, precision, standard::Handle};

/// The session's options; their default is `boptions -default`.
#[derive(Clone, Copy, Debug)]
pub struct BopOptions {
    /// `bfuzzyvalue`.
    pub fuzzy: f64,
    /// `bnondestructive`.
    pub non_destructive: bool,
    /// `brunparallel`.
    pub run_parallel: bool,
    /// `setfillhistory`.
    pub fill_history: bool,
}

impl Default for BopOptions {
    fn default() -> Self {
        Self {
            fuzzy: precision::confusion(),
            non_destructive: false,
            run_parallel: false,
            fill_history: true,
        }
    }
}

/// The arguments, options and prepared intersections a script's commands share.
/// Example: `let mut bop = BopSession::default();`.
#[derive(Default)]
pub struct BopSession {
    /// The options in force.
    pub options: BopOptions,
    objects: Vec<Shape>,
    tools: Vec<Shape>,
    filler: Option<BOPAlgoPaveFiller>,
    history: Option<Handle<History>>,
}

fn listed(shapes: impl IntoIterator<Item = Shape>) -> List<Shape> {
    shapes.into_iter().collect()
}

fn bound_shape(session: &dyn Session, name: &str) -> Shape {
    session
        .shape(name)
        .unwrap_or_else(|| panic!("{name}: expected a bound shape"))
        .clone()
}

impl BopSession {
    /// A session whose options were set before its first command.
    /// Example: `BopSession::with_options(BopOptions::default())`.
    pub fn with_options(options: BopOptions) -> Self {
        Self {
            options,
            ..Self::default()
        }
    }

    /// `bclearobjects`, `bcleartools`, `baddobjects objects`, `baddtools tools`.
    /// Example: `bop.arguments(s, &["a"], &["b"]);`.
    pub fn arguments(&mut self, session: &dyn Session, objects: &[&str], tools: &[&str]) {
        self.objects = objects.iter().map(|n| bound_shape(session, n)).collect();
        self.tools = tools.iter().map(|n| bound_shape(session, n)).collect();
    }

    /// `baddobjects name`: one more object, as the name holds it now; `bop.add_object(s, "ee");`.
    pub fn add_object(&mut self, session: &dyn Session, name: &str) {
        self.objects.push(bound_shape(session, name));
    }

    /// `bfillds`: prepares the intersections the next commands reuse; `bop.bfillds();`.
    pub fn bfillds(&mut self) {
        let mut filler = BOPAlgoPaveFiller::new();
        filler.set_arguments(&listed(self.objects.iter().chain(&self.tools).cloned()));
        filler.set_non_destructive(self.options.non_destructive);
        filler.set_glue(Glue::Off);
        let options = filler.options_mut();
        options.set_run_parallel(self.options.run_parallel);
        options.set_fuzzy_value(self.options.fuzzy);
        options.set_use_obb(false);
        filler.perform();
        report_result_alerts(filler.options());
        self.filler = Some(filler);
    }

    /// The prepared intersections, for a command that lives elsewhere.
    /// Example: `builder.perform_with_filler(bop.prepared_filler());`.
    pub fn prepared_filler(&mut self) -> &mut BOPAlgoPaveFiller {
        self.prepared()
    }

    /// Gives a cells builder the arguments and options `bcbuild` sets.
    /// Example: `bop.configure_cells(&mut builder);`.
    pub fn configure_cells(&self, builder: &mut martensite_tkbo::bop_algo::BOPAlgoCellsBuilder) {
        builder.set_arguments(&listed(self.objects.iter().chain(&self.tools).cloned()));
        builder.set_non_destructive(self.options.non_destructive);
        builder.set_glue(Glue::Off);
        builder.set_check_inverted(true);
        builder.set_to_fill_history(self.options.fill_history);
        let options = builder.options_mut();
        options.set_run_parallel(self.options.run_parallel);
        options.set_fuzzy_value(self.options.fuzzy);
        options.set_use_obb(false);
    }

    fn prepared(&mut self) -> &mut BOPAlgoPaveFiller {
        self.filler.as_mut().expect("bfillds before this command")
    }

    /// `bbop name op`: `Section` runs on every argument, the others on objects and tools.
    /// Example: `bop.bbop(s, "r_1", Operation::Fuse);`.
    pub fn bbop(&mut self, session: &mut dyn Session, name: &str, kind: Operation) {
        if kind == Operation::Section {
            self.section(session, name);
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
        publish(session, name, operation.options(), operation.shape());
    }

    /// `savehistory name` after `bbop`; `let bop_hist = bop.savehistory();`.
    pub fn savehistory(&self) -> Handle<History> {
        self.history.clone().expect("bbop before savehistory")
    }

    fn section(&mut self, session: &mut dyn Session, name: &str) {
        let mut operation = BOPAlgoSection::new();
        operation.set_arguments(&listed(self.objects.iter().chain(&self.tools).cloned()));
        operation
            .options_mut()
            .set_run_parallel(self.options.run_parallel);
        operation.set_check_inverted(true);
        operation.set_to_fill_history(self.options.fill_history);
        operation.perform_with_filler(self.prepared());
        publish(session, name, operation.options(), operation.shape());
    }

    /// `bapibop name 2` under `bsimplify -e 1 -f 1`: the API runs its own intersections, then
    /// unifies the result. Example: `bop.bapibop_cut_simplified(s, "plate");`.
    pub fn bapibop_cut_simplified(&mut self, session: &mut dyn Session, name: &str) {
        let mut cut = BRepAlgoAPICut::new();
        cut.set_arguments(&listed(self.objects.iter().cloned()));
        cut.set_tools(&listed(self.tools.iter().cloned()));
        cut.set_run_parallel(self.options.run_parallel);
        cut.set_fuzzy_value(self.options.fuzzy);
        cut.set_non_destructive(self.options.non_destructive);
        cut.set_glue(Glue::Off);
        cut.set_check_inverted(true);
        cut.set_use_obb(false);
        cut.set_to_fill_history(self.options.fill_history);
        cut.build().unwrap();
        cut.simplify_result(true, true, precision::angular())
            .unwrap();
        for (gravity, prefix) in [(Gravity::Warning, "Warning: "), (Gravity::Fail, "Error: ")] {
            print!("{}", gravity_alert_text(cut.get_report(), gravity, prefix));
        }
        if !cut.has_errors() {
            session.bind(name, cut.shape());
        }
    }

    /// `bbuild name`: the general fuse of every argument; `bop.bbuild(s, "rgf");`.
    pub fn bbuild(&mut self, session: &mut dyn Session, name: &str) {
        let mut builder = BOPAlgoBuilder::new();
        builder.set_arguments(&listed(self.objects.iter().chain(&self.tools).cloned()));
        builder
            .options_mut()
            .set_run_parallel(self.options.run_parallel);
        builder.set_check_inverted(true);
        builder.set_to_fill_history(self.options.fill_history);
        builder.perform_with_filler(self.prepared());
        publish(session, name, builder.options(), builder.shape());
    }

    /// `bsplit name`: the objects split by the tools; `bop.bsplit(s, "result");`.
    pub fn bsplit(&mut self, session: &mut dyn Session, name: &str) {
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
        publish(session, name, splitter.options(), splitter.shape());
    }
}

/// The end of each result command: its alerts, then the binding unless it failed or came out null.
fn publish(session: &mut dyn Session, name: &str, options: &Options, shape: &Shape) {
    report_result_alerts(options);
    if options.has_errors() {
        return;
    }
    if shape.is_null() {
        println!("Result is a null shape");
        return;
    }
    session.bind(name, shape);
}
