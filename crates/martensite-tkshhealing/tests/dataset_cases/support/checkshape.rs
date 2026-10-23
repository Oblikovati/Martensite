//! `checkshape [-top] shape [-parallel] [-exact]` with its
//! contextual dump (the form without a result prefix), on BRepCheck_Analyzer.
//!
//! A valid shape prints `This shape seems to be valid`; an invalid one prints each faulty
//! sub-shape with its statuses and `Faulty shapes in variables faulty_1 to faulty_N`,
//! which the test grids' parse rules count as a failure (`\bFaulty\b`); the result is
//! logged ([`super::log`]) and the case's log gets the verdict.
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

/// The shape-check result text and the number of faulty shapes it names.
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

/// `checkshape name [-top] [-parallel] [-exact]` of a shape, its result logged.
pub fn checkshape_report(shape: &Shape, options: CheckOptions) -> CheckReport {
    let report = checkshape_text(shape, options);
    super::log::result(&report.text);
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
    let mut printer = ContextualPrinter {
        analyzer: &analyzer,
        map: DataMap::default(),
        faulty: Vec::new(),
        out: Vec::new(),
    };
    printer.print(shape);
    let nb_faulty = printer.faulty.len();
    let mut text = String::from_utf8_lossy(&printer.out).into_owned();
    text.push('\n');
    if nb_faulty != 0 {
        text.push_str(&format!(
            "Faulty shapes in variables faulty_1 to faulty_{nb_faulty} \n"
        ));
    }
    text.push('\n');
    CheckReport {
        text,
        nb_faulty,
        valid: false,
    }
}

/// `checkshape name`, its result logged.
pub fn checkshape(shape: &Shape) -> CheckReport {
    checkshape_report(shape, CheckOptions::default())
}

type ShapeContextMap = DataMap<Shape, Vec<Shape>, ShapeMapHasher>;

/// The statics of the contextual dump: `THE_SHAPE_CONTEXT_MAP` and `lfaulty`.
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
        .expect("NCollection_List::Iterator::Value")
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

    /// `PrintSub(OS, Ana, S, Subtype)`.
    fn print_sub(&mut self, shape: &Shape, sub_type: ShapeEnum) {
        let mut explorer = Explorer::new(shape, sub_type, ShapeEnum::Shape).unwrap();
        while explorer.more() {
            let sub = explorer.current().clone();
            let handle = self.analyzer.result(&sub).unwrap();
            let res: &dyn BRepCheckResult = handle.get().expect("BRepCheck_Result");
            res.init_context_iterator().unwrap();
            while res.more_shape_in_context() {
                let context = res.contextual_shape().unwrap();
                let seen = self.map.change_find(&sub).unwrap();
                if context.is_same(shape) && !seen.iter().any(|s| s.is_same(shape)) {
                    seen.push(shape.clone());
                    let statuses = res.status_on_shape().unwrap();
                    if first_status(&statuses) != BRepCheckStatus::NoError {
                        let name = self.name_of(&sub);
                        self.out.extend(format!("Shape {name} ").bytes());
                        let name = self.name_of(shape);
                        self.out.extend(format!(" on shape {name} :\n").bytes());
                        self.print_statuses(&statuses, false);
                    }
                    break;
                }
                res.next_shape_in_context();
            }
            explorer.next().unwrap();
        }
    }

    /// `Print(OS, Ana, S)`.
    fn print(&mut self, shape: &Shape) {
        let mut iterator = ShapeIterator::new(shape, true, true);
        while let Ok(child) = iterator.value() {
            let child = child.clone();
            self.print(&child);
            iterator.next();
        }
        let handle = self.analyzer.result(shape).unwrap();
        if let Some(res) = handle.get()
            && !self.map.is_bound(shape)
        {
            let statuses = res.status().unwrap();
            if first_status(&statuses) != BRepCheckStatus::NoError {
                let name = self.name_of(shape);
                self.out.extend(format!("On Shape {name} :\n").bytes());
                self.print_statuses(&statuses, true);
            }
        }
        if !self.map.is_bound(shape) {
            self.map.bind(shape.clone(), Vec::new());
        }
        match shape.shape_type().unwrap() {
            ShapeEnum::Edge => self.print_sub(shape, ShapeEnum::Vertex),
            ShapeEnum::Wire => {
                self.print_sub(shape, ShapeEnum::Edge);
                self.print_sub(shape, ShapeEnum::Vertex);
            }
            ShapeEnum::Face => {
                self.print_sub(shape, ShapeEnum::Wire);
                self.print_sub(shape, ShapeEnum::Edge);
                self.print_sub(shape, ShapeEnum::Vertex);
            }
            ShapeEnum::Solid => self.print_sub(shape, ShapeEnum::Shell),
            _ => {}
        }
    }
}
