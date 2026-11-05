//! What a script reads back at run time: the values it keeps in variables and the conditions of
//! its `if` commands. Everything a generator could resolve statically is resolved there.
use crate::script::OffsetScript;
use crate::support::{
    checkshape::{CheckOptions, checkshape_report, checkshape_text_quiet},
    maxtolerance::maxtolerance,
    section_check::checksection_count,
    tolerance_check::tolerance_report,
};
use martensite_boolean_cases::support::{
    bop_session::BopOptions,
    named_checks::{bopargcheck_faulties, bopcheck},
};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
use std::collections::HashMap;

/// `set name [command]`: what the variable takes.
#[derive(Clone, Copy)]
pub enum Value {
    /// `[checkshape name]`: its report, which the command also prints.
    CheckShape(&'static str),
    /// `[checkshape name prefix]`: the plain report, which says `OK` for a valid shape.
    CheckShapeNamed(&'static str),
    /// `[tolerance name]`.
    Tolerance(&'static str),
    /// `[maxtolerance name]`.
    MaxTolerance(&'static str),
    /// `[llength [explode name kind]]`.
    ExplodeCount(&'static str, &'static str),
    /// `[checkmaxtol name]`, which the command also writes.
    CheckedMaxTolerance(&'static str),
    /// `[bopcheck name]`.
    Bopcheck(&'static str),
    /// `[catch {step}]`, which a condition reads through `Cond::failed`.
    Text(&'static str),
    /// `[bopargcheck name]`, which the command also writes.
    ArgumentCheck(&'static str),
    /// The report's `Mass: value`, from `lprops` and the others.
    Mass(crate::support::properties::Prop, &'static str),
    /// `[checksection name]`: its report.
    Section(&'static str),
    /// `nb alone Vertices: n` of `[checksection name]`.
    AloneVertices(&'static str),
    /// `MAX=value` of `[tolerance name]`.
    ToleranceMax(&'static str),
    /// `[whatis name]`.
    Kind(&'static str),
    /// `[isdeleted history name]`.
    Deleted(&'static str, &'static str),
    /// The last `buildsweep`'s answer: `Buildsweep: Not Done` when it is not done.
    SweepOutcome,
}

/// The values of one running script.
#[derive(Default)]
pub struct ScriptValues {
    texts: HashMap<&'static str, String>,
}

impl ScriptValues {
    /// Keeps `text` under `name`; `values.keep("che", "This shape seems to be valid".into())`.
    pub fn keep(&mut self, name: &'static str, text: String) {
        self.texts.insert(name, text);
    }

    /// The text kept under `name`, empty when the script never set it.
    pub fn text(&self, name: &str) -> &str {
        self.texts.get(name).map_or("", String::as_str)
    }
}

/// The condition of an `if` that only the run can decide.
pub enum Cond {
    /// `[isdraw name]`.
    Bound(&'static str),
    /// `![isdraw name]`.
    Unbound(&'static str),
    /// `[catch {step}]` with what fails in it.
    Failed(&'static crate::steps::Step),
    /// `[regexp text [whatis name]]`: the name's shape kind holds the text.
    KindHolds(&'static str, &'static str),
    /// `![regexp text [whatis name]]`.
    KindLacks(&'static str, &'static str),
    /// `[llength [whatis name]] < 4`: nothing to describe.
    NullShape(&'static str),
    /// `$a + shift < $b`, two variables the run filled.
    Below(&'static str, &'static str, f64),
    /// `$a >= $b`.
    NotBelow(&'static str, &'static str),
    /// `$name > limit`, a variable the run filled against a literal.
    Above(&'static str, f64),
    /// `$name != value`.
    Differs(&'static str, f64),
    /// `[regexp "Faulties" [bopargcheck name]]`.
    ArgumentFaulties(&'static str),
    /// `[regexp {literal} $name]`.
    Holds(&'static str, &'static str),
    /// `![regexp {literal} $name]`.
    Lacks(&'static str, &'static str),
}

/// Computes a value; `value_of(script, &Value::check_shape("result"))`.
pub fn value_of(script: &mut OffsetScript<'_>, value: &Value) -> String {
    match *value {
        Value::CheckShape(name) => checkshape_text(script, name),
        Value::CheckShapeNamed(name) => plain_checkshape(script, name),
        Value::Tolerance(name) => tolerance_report(script.session(), name),
        Value::MaxTolerance(name) => maxtolerance(script.session(), name),
        Value::ExplodeCount(name, kind) => {
            let kind = crate::input_shapes::explode_kind(kind)
                .unwrap_or_else(|f| panic!("explode: {f:?}"));
            crate::wire_end::exploded(script, name, kind)
                .len()
                .to_string()
        }
        Value::CheckedMaxTolerance(name) => tolerance_report(script.session(), name),
        Value::Bopcheck(name) => bopcheck(&script.shape(name), &BopOptions::default()),
        Value::Text(text) => text.to_owned(),
        Value::ArgumentCheck(name) => argument_check(&script.shape(name)),
        Value::Mass(kind, name) => {
            let shape = script.shape(name);
            crate::support::properties::props(kind, &shape, None).to_string()
        }
        Value::Section(name) => {
            let alone = checksection_count(script.session(), name);
            format!(" nb alone Vertices : {alone}")
        }
        Value::AloneVertices(name) => checksection_count(script.session(), name).to_string(),
        Value::ToleranceMax(name) => tolerance_report(script.session(), name),
        Value::Kind(name) => script
            .shape(name)
            .shape_type()
            .map(|kind| format!("{kind:?}").to_uppercase())
            .unwrap_or_default(),
        Value::Deleted(history, name) => deleted_text(script, history, name),
        Value::SweepOutcome => {
            let done = script
                .commands
                .sweep
                .as_ref()
                .is_some_and(|sweep| sweep.is_done());
            if done {
                String::new()
            } else {
                "Buildsweep : Not Done".to_owned()
            }
        }
    }
}

/// The verdict `bopargcheck` writes, which the command there writes too.
pub fn argument_check(shape: &martensite_tkbrep::topods::Shape) -> String {
    let text = if bopargcheck_faulties(shape, &BopOptions::default()) {
        "Faulties, that can not be treated by BOP, are detected."
    } else {
        "Shape(s) seem(s) to be valid for BOP."
    };
    text.to_owned()
}

fn deleted_text(script: &mut OffsetScript<'_>, history: &str, name: &str) -> String {
    let shape = script.shape(name);
    let deleted = script
        .saved_histories
        .get(history)
        .and_then(|history| {
            history
                .get()
                .map(|history| history.is_removed(&shape).unwrap_or(false))
        })
        .unwrap_or(false);
    let text = if deleted { "Deleted." } else { "Not deleted." };
    text.to_owned()
}

fn checkshape_text(script: &mut OffsetScript<'_>, name: &str) -> String {
    let shape = script.shape(name);
    checkshape_report(&shape, CheckOptions::default()).text
}

fn plain_checkshape(script: &mut OffsetScript<'_>, name: &str) -> String {
    let report = checkshape_text_quiet(&script.shape(name));
    let text = if report.nb_faulty == 0 && report.is_valid() {
        format!(" -- The Shape {name} looks OK")
    } else {
        report.text
    };
    println!("{text}");
    text
}
