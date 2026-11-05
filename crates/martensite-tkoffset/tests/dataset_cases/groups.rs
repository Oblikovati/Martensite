//! The groups these cases inherit from: their parse rules, what their `begin` writes, and the end
//! scripts of each.
use crate::script::OffsetScript;
use crate::support::{
    checkprops::{Expected, checkprops},
    checkshape::checkshape,
    checkview::{CheckView, checkview},
    maxtolerance::{last_row_largest, maxtolerance},
    properties::{Prop, printed_mass, props},
    rules::{Rule, RuleMatch, RuleStatus},
    script_error::script_error,
};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::storage::format_general;
use martensite_tkg3d::top_abs::ShapeEnum;

fn is_faulty(word: &str) -> bool {
    word == "Faulty"
}

const BAD_SHAPE: Rule = Rule {
    status: RuleStatus::Failed,
    matching: RuleMatch::Word(is_faulty),
};
const MASS_MESSAGE: Rule = Rule {
    status: RuleStatus::Ok,
    matching: RuleMatch::Contains("Relative error of mass computation"),
};

/// `offset`, `thrusection`, `pipe`, `nproject` and `boolean`.
const SHAPE_RULES: &[Rule] = &[BAD_SHAPE, MASS_MESSAGE];
/// `blend`: a `blend` that returns 1 is only an incomplete test.
const BLEND_RULES: &[Rule] = &[
    BAD_SHAPE,
    Rule {
        status: RuleStatus::Ignore,
        matching: RuleMatch::Contains("ScriptStep Exception: tolerance ang : "),
    },
    MASS_MESSAGE,
];
/// `evolved`.
const EVOLVED_RULES: &[Rule] = &[BAD_SHAPE];
/// `draft`.
const DRAFT_RULES: &[Rule] = &[
    BAD_SHAPE,
    Rule {
        status: RuleStatus::Ignore,
        matching: RuleMatch::Contains("Error Status : Impossible"),
    },
    MASS_MESSAGE,
];
/// `feat`.
const FEAT_RULES: &[Rule] = &[
    BAD_SHAPE,
    Rule {
        status: RuleStatus::Ignore,
        matching: RuleMatch::Contains("No error"),
    },
    MASS_MESSAGE,
];
/// `bugs`.
const BUGS_RULES: &[Rule] = &[
    BAD_SHAPE,
    Rule {
        status: RuleStatus::Failed,
        matching: RuleMatch::Contains("failure reading attribute"),
    },
    Rule {
        status: RuleStatus::Skipped,
        matching: RuleMatch::Contains("Error: unsupported locale specification"),
    },
    MASS_MESSAGE,
];

/// The test folder of a case, which fixes its parse rules.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Group {
    Shape,
    Blend,
    Evolved,
    Draft,
    Feat,
    Bugs,
}

impl Group {
    /// The parse rules read before the common ones; `Group::Draft.rules()`.
    pub fn rules(self) -> &'static [Rule] {
        match self {
            Self::Shape => SHAPE_RULES,
            Self::Blend => BLEND_RULES,
            Self::Evolved => EVOLVED_RULES,
            Self::Draft => DRAFT_RULES,
            Self::Feat => FEAT_RULES,
            Self::Bugs => BUGS_RULES,
        }
    }
}

/// The end script a case's chain runs.
#[derive(Clone, Copy)]
pub enum End {
    /// Only the completion line, as `bugs/end` with its checks off and `evolved/end`.
    Completed,
    /// A script that writes the completion line itself, or stops before it.
    Nothing,
    /// `boolean/end`: `checkshape result` when bound.
    Boolean,
    /// `if {[isdraw result]} {checkshape; checkview} else {puts missing}`, then completion. The
    /// flag says the check's text is printed again, as `puts [checkshape result]` does.
    ResultView(&'static str, bool),
    /// `if {[isdraw result]} {checkview}`, then completion: `pipe/specific/end`.
    ViewIfBound,
    /// `feat/end`.
    Feat,
    /// `draft/end`.
    Draft,
    /// `offset/end`, for a shape offset; whether `OFFSETSHAPE` named no face, when it ran.
    OffsetShape(Option<bool>),
    /// `offset/end` of `set command withintersect`.
    OffsetWithIntersect,
    /// `offset/end` of `set command mkoffset`.
    OffsetWire(WireEnd),
}

/// The variables `offset/end` reads for `mkoffset`.
#[derive(Clone, Copy)]
pub struct WireEnd {
    pub off_param: f64,
    /// `set length value`; none for `set length empty`.
    pub length: Option<f64>,
    pub vertices: Option<usize>,
    pub edges: Option<usize>,
    pub wires: Option<usize>,
}

/// Runs the case's end script; `run_end(script, &End::completed)`.
pub fn run_end(script: &mut OffsetScript<'_>, end: &End) {
    match end {
        End::Nothing => return,
        End::Completed => {}
        End::Boolean => {
            if let Some(result) = script.session().shape("result").cloned() {
                checkshape(&result);
            }
        }
        End::ResultView(missing, printed) => result_view(script, missing, *printed),
        End::ViewIfBound => {
            if script.session().shape("result").is_some() {
                checkview(script.session(), &CheckView::of("result"));
            }
        }
        End::Feat => feat_end(script),
        End::Draft => draft_end(script),
        End::OffsetShape(no_faces) => offset_shape_end(script, *no_faces),
        End::OffsetWithIntersect => with_intersect_end(script),
        End::OffsetWire(wire) => crate::wire_end::wire_end(script, wire),
    }
    println!("TEST COMPLETED");
}

/// The shape of `[checkshape result]` when the script filled the variable: a null shape stops it.
fn checked_result(script: &mut OffsetScript<'_>) -> Shape {
    let session = script.session();
    match session.shape("result") {
        Some(result) => result.clone(),
        None => {
            println!("result is not a topological shape!!!\n");
            script_error("result is not a topological shape!!!");
        }
    }
}

fn result_view(script: &mut OffsetScript<'_>, missing: &str, printed: bool) {
    if !script.is_bound("result") {
        println!("{missing}");
        return;
    }
    let report = checkshape(&checked_result(script));
    if printed {
        println!("{}", report.text);
    }
    checkview(script.session(), &CheckView::of("result"));
}

fn draft_end(script: &mut OffsetScript<'_>) {
    if !script.is_bound("result") {
        println!("Error : The skin cannot be built.");
        return;
    }
    let result = checked_result(script);
    let report = checkshape(&result);
    println!("{}", report.text);
    if report.text.trim() == "This shape seems to be valid" {
        print!("{}", maxtolerance(script.session(), "result"));
        let tolerance = last_row_largest(&result);
        if tolerance > 1. {
            let shown = format_general(tolerance, 6);
            println!("Error: The tolerance of the resulting shape is too big ({shown}).");
        }
    }
    checkview(script.session(), &CheckView::of("result"));
}

/// `feat/end`.
fn feat_end(script: &mut OffsetScript<'_>) {
    if !script.is_bound("result") {
        println!("Error : The feat grid cannot be built.");
        return;
    }
    let result = checked_result(script);
    println!("{}", checkshape(&result).text);
    checkview(script.session(), &CheckView::of("result"));
}

/// `offset/end` for a shape offset.
fn offset_shape_end(script: &mut OffsetScript<'_>, no_faces: Option<bool>) {
    if !script.is_bound("result") {
        println!("Error : The offset cannot be built.");
        return;
    }
    let result = checked_result(script);
    println!("{}", checkshape(&result).text);
    let mass = printed_mass(props(Prop::Volume, &result, None));
    for face in crate::wire_end::exploded(script, "result", ShapeEnum::Face) {
        if printed_mass(props(Prop::Area, &face.1, None)) < 0. {
            println!(
                "Error : The area of face {} of the resulting shape is negative.",
                face.0
            );
        }
    }
    if no_faces == Some(true) {
        section_is_empty(script);
    }
    view_when_massive(script, mass);
}

/// `puts [bsection re result s]`, which must have no length.
fn section_is_empty(script: &mut OffsetScript<'_>) {
    crate::support::one_step::bsection(script.session(), "re", "result", "s");
    let Some(section): Option<Shape> = script.session().shape("re").cloned() else {
        return;
    };
    if printed_mass(props(Prop::Length, &section, None)) != 0. {
        println!("Error: bsection of the result and s is not equal to zero.");
    }
}

fn with_intersect_end(script: &mut OffsetScript<'_>) {
    if !script.is_bound("result") {
        println!("Error : The offset cannot be built.");
        return;
    }
    let result = checked_result(script);
    let mass = printed_mass(props(Prop::Area, &result, None));
    checkshape(&result);
    view_when_massive(script, mass);
}

/// `if {$m > 0} {checkview}`, of `result_unif` when the script made one.
pub fn view_when_massive(script: &mut OffsetScript<'_>, mass: f64) {
    if mass <= 0. {
        return;
    }
    let shown = if script.session().shape("result_unif").is_some() {
        "result_unif"
    } else {
        "result"
    };
    checkview(script.session(), &CheckView::of(shown));
}

/// `checkprops result -l length`; `expect_length(script, Some(8.78))`.
pub fn expect_length(script: &mut OffsetScript<'_>, length: Option<f64>) {
    let expected = length.map_or(Expected::Empty, Expected::Mass);
    checkprops(script.session(), "result", Prop::Length, expected);
}
