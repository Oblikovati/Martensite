//! `checkshape shape` with its contextual dump (the form without a result prefix).
//!
//! A valid shape prints `This shape seems to be valid`; an invalid one prints each faulty
//! sub-shape with its statuses and `Faulty shapes in variables faulty_1 to faulty_N`,
//! which the parse rules count as a failure. The text goes to the case's stream.
use super::{script_error::script_error, session::Session};
use martensite_tkbrep::{
    topexp::Explorer,
    topods::{Shape, ShapeIterator},
    toptools::ShapeMapHasher,
};
use martensite_tkernel::ncollection::DataMap;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tktopalgo::brep_check::{
    BRepCheck, BRepCheckAnalyzer, BRepCheckResult, BRepCheckStatus, ListOfStatus,
};

/// The `checkshape` options the cases pass.
#[derive(Clone, Copy, Debug, Default)]
pub struct CheckOptions {
    /// `-top`: topology only (no geometric controls).
    pub top: bool,
    /// `-parallel`.
    pub parallel: bool,
    /// `-exact`.
    pub exact: bool,
}

/// What `checkshape` prints: the script result text and the number of faulty shapes it names.
#[derive(Clone, Debug)]
pub struct CheckReport {
    pub text: String,
    pub nb_faulty: usize,
    valid: bool,
}

impl CheckReport {
    /// The report of a valid shape (`This shape seems to be valid`).
    pub fn is_valid(&self) -> bool {
        self.valid
    }

    /// Whether the text names faulty shapes (`\bFaulty\b` of the parse rules).
    pub fn has_faulty(&self) -> bool {
        self.text.contains("Faulty")
    }
}

/// `checkshape name [-top] [-parallel] [-exact]` of a shape, its text on the stream.
/// Example: `checkshape_report(&shape, CheckOptions::default()).is_valid()`.
pub fn checkshape_report(shape: &Shape, options: CheckOptions) -> CheckReport {
    let report = checkshape_text(shape, options);
    println!("{}", report.text);
    report
}

fn checkshape_text(shape: &Shape, options: CheckOptions) -> CheckReport {
    assert!(!shape.is_null(), "checkshape: not a topological shape");
    let analyzer = BRepCheckAnalyzer::new(shape, !options.top, options.parallel, options.exact)
        .unwrap_or_else(|failure| panic!("checkshape exception : {failure:?}"));
    if analyzer.is_valid().unwrap() {
        return CheckReport {
            text: "This shape seems to be valid".into(),
            nb_faulty: 0,
            valid: true,
        };
    }
    faulty_report(&analyzer, shape)
}

fn faulty_report(analyzer: &BRepCheckAnalyzer, shape: &Shape) -> CheckReport {
    let mut printer = ContextualPrinter {
        analyzer,
        map: DataMap::default(),
        faulty: Vec::new(),
        out: Vec::new(),
    };
    printer.print(shape);
    let nb_faulty = printer.faulty.len();
    let mut text = String::from_utf8_lossy(&printer.out).into_owned();
    text.push('\n');
    if nb_faulty != 0 {
        text += &format!("Faulty shapes in variables faulty_1 to faulty_{nb_faulty} \n");
    }
    text.push('\n');
    CheckReport {
        text,
        nb_faulty,
        valid: false,
    }
}

/// `checkshape name`, its text on the stream; `checkshape(&shape).is_valid()`.
pub fn checkshape(shape: &Shape) -> CheckReport {
    checkshape_report(shape, CheckOptions::default())
}

/// `[checkshape name]` read by a script that does not print it; `checkshape_text_quiet(&shape)`.
pub fn checkshape_text_quiet(shape: &Shape) -> CheckReport {
    checkshape_text(shape, CheckOptions::default())
}

/// `checkshape name`, by the name the script bound; `checkshape_named(s, "result")`.
pub fn checkshape_named(session: &dyn Session, name: &str) {
    checkshape(bound_for_check(session, name));
}

/// `checkshape -top name`: topology only; `checkshape_top(s, "sec")`.
pub fn checkshape_top(session: &dyn Session, name: &str) {
    let top = CheckOptions {
        top: true,
        ..CheckOptions::default()
    };
    checkshape_report(bound_for_check(session, name), top);
}

/// The command refuses a name that holds no shape, which stops the script.
fn bound_for_check<'a>(session: &'a dyn Session, name: &str) -> &'a Shape {
    session
        .shape(name)
        .unwrap_or_else(|| script_error(&format!("{name} is not a topological shape!!!")))
}

type ShapeContextMap = DataMap<Shape, Vec<Shape>, ShapeMapHasher>;

/// The state of one contextual dump: the contexts seen per shape, and the faulty shapes named.
struct ContextualPrinter<'a> {
    analyzer: &'a BRepCheckAnalyzer,
    map: ShapeContextMap,
    faulty: Vec<Shape>,
    out: Vec<u8>,
}

fn first_status(list: &ListOfStatus) -> BRepCheckStatus {
    *list
        .iter()
        .next()
        .expect("a check result with at least one status")
}

impl ContextualPrinter<'_> {
    /// The `faulty_N` name of a shape, a new one when it has none.
    fn name_of(&mut self, shape: &Shape) -> String {
        let index = match self.faulty.iter().position(|s| s.is_same(shape)) {
            Some(index) => index,
            None => {
                self.faulty.push(shape.clone());
                self.faulty.len() - 1
            }
        };
        format!("faulty_{}", index + 1)
    }

    fn print_statuses(&mut self, statuses: &ListOfStatus, skip_no_error: bool) {
        for status in statuses.iter() {
            if !(skip_no_error && *status == BRepCheckStatus::NoError) {
                BRepCheck::print(*status, &mut self.out).unwrap();
            }
        }
    }

    /// Reports each sub-shape of `sub_type` that is faulty in the context of `shape`.
    fn print_sub(&mut self, shape: &Shape, sub_type: ShapeEnum) {
        let mut explorer = Explorer::new(shape, sub_type, ShapeEnum::Shape).unwrap();
        while explorer.more() {
            let sub = explorer.current().clone();
            self.print_in_context(&sub, shape);
            explorer.next().unwrap();
        }
    }

    fn print_in_context(&mut self, sub: &Shape, shape: &Shape) {
        let handle = self.analyzer.result(sub).unwrap();
        let res: &dyn BRepCheckResult = handle.get().expect("a check result of the sub-shape");
        res.init_context_iterator().unwrap();
        while res.more_shape_in_context() {
            let context = res.contextual_shape().unwrap();
            let seen = self.map.change_find(sub).unwrap();
            if context.is_same(shape) && !seen.iter().any(|s| s.is_same(shape)) {
                seen.push(shape.clone());
                self.print_context_statuses(res, sub, shape);
                break;
            }
            res.next_shape_in_context();
        }
    }

    fn print_context_statuses(&mut self, res: &dyn BRepCheckResult, sub: &Shape, shape: &Shape) {
        let statuses = res.status_on_shape().unwrap();
        if first_status(&statuses) == BRepCheckStatus::NoError {
            return;
        }
        let name = self.name_of(sub);
        self.out.extend(format!("Shape {name} ").bytes());
        let name = self.name_of(shape);
        self.out.extend(format!(" on shape {name} :\n").bytes());
        self.print_statuses(&statuses, false);
    }

    /// Reports the children first, then `shape` itself, then its sub-shapes in context.
    fn print(&mut self, shape: &Shape) {
        let mut iterator = ShapeIterator::new(shape, true, true);
        while let Ok(child) = iterator.value() {
            let child = child.clone();
            self.print(&child);
            iterator.next();
        }
        self.print_own_statuses(shape);
        if !self.map.is_bound(shape) {
            self.map.bind(shape.clone(), Vec::new());
        }
        self.print_subs_of(shape);
    }

    fn print_own_statuses(&mut self, shape: &Shape) {
        let handle = self.analyzer.result(shape).unwrap();
        let Some(res) = handle.get() else {
            return;
        };
        if self.map.is_bound(shape) {
            return;
        }
        let statuses = res.status().unwrap();
        if first_status(&statuses) != BRepCheckStatus::NoError {
            let name = self.name_of(shape);
            self.out.extend(format!("On Shape {name} :\n").bytes());
            self.print_statuses(&statuses, true);
        }
    }

    fn print_subs_of(&mut self, shape: &Shape) {
        let kinds: &[ShapeEnum] = match shape.shape_type().unwrap() {
            ShapeEnum::Edge => &[ShapeEnum::Vertex],
            ShapeEnum::Wire => &[ShapeEnum::Edge, ShapeEnum::Vertex],
            ShapeEnum::Face => &[ShapeEnum::Wire, ShapeEnum::Edge, ShapeEnum::Vertex],
            ShapeEnum::Solid => &[ShapeEnum::Shell],
            _ => &[],
        };
        for kind in kinds {
            self.print_sub(shape, *kind);
        }
    }
}
