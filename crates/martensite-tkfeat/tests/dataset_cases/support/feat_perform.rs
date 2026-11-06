//! `featperform` and `featperformval`: one defined feature, performed to its limits; its result and
//! history are set when it is done, as the command does.
use super::{
    feat_script::{FeatScript, checked, command_failed},
    session::Session,
};
use martensite_tkbrep::{
    brep_tools::{History, HistoryAlgorithm},
    topods::Shape,
};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkfeat::brep_feat::{self as brep_feat, FormFeature, StatusError};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
use std::f64::consts::PI;

/// The feature kind a `featperform` names.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kas {
    Prism,
    Revol,
    DPrism,
    LinearForm,
    RevolutionForm,
}

impl Kas {
    fn word(self) -> &'static str {
        match self {
            Kas::Prism => "prism",
            Kas::Revol => "revol",
            Kas::DPrism => "dprism",
            Kas::LinearForm => "lf",
            Kas::RevolutionForm => "rf",
        }
    }
}

/// Stands in for `the shared history::set_history`: reads a builder through its own queries.
struct MakeShapeHistory<'a, T: BRepBuilderAPIMakeShape>(&'a mut T);

impl<T: BRepBuilderAPIMakeShape> HistoryAlgorithm for MakeShapeHistory<'_, T> {
    fn is_deleted(&mut self, shape: &Shape) -> Result<bool, Failure> {
        self.0.is_deleted(shape)
    }

    fn modified(&mut self, shape: &Shape) -> Result<Vec<Shape>, Failure> {
        Ok(self.0.modified(shape)?.iter().cloned().collect())
    }

    fn generated(&mut self, shape: &Shape) -> Result<Vec<Shape>, Failure> {
        Ok(self.0.generated(shape)?.iter().cloned().collect())
    }
}

fn not_defined(kas: Kas) -> ! {
    command_failed(&format!("{} not defined", kas.word()));
}

fn status_stop(error: StatusError) -> ! {
    let mut text = Vec::new();
    brep_feat::print(error, &mut text).expect("a vector takes the status text");
    command_failed(&format!("{}\n", String::from_utf8_lossy(&text)));
}

/// Sets a done feature under `name` and, for the swept features, the session's history.
fn finish<T: BRepBuilderAPIMakeShape>(
    s: &mut FeatScript<'_>,
    name: &str,
    feature: &mut T,
    error: StatusError,
    with_history: bool,
) {
    if !feature.is_done() {
        status_stop(error);
    }
    if with_history {
        let inputs = s.features.inputs.clone();
        let history = checked(History::from_algorithm(
            &inputs,
            &mut MakeShapeHistory(feature),
        ));
        s.history = Some(Handle::new(history));
    }
    let shape = checked(feature.shape()).clone();
    s.bind(name, &shape);
}

/// `featperform kind result`: through everything.
/// Example: `featperform_all(s, Kas::DPrism, "result")`.
pub fn featperform_all(s: &mut FeatScript<'_>, kas: Kas, name: &str) {
    match kas {
        Kas::Prism => run_prism(s, name, |f| f.perform_thru_all()),
        Kas::Revol => run_revol(s, name, |f| f.perform_thru_all()),
        Kas::DPrism => run_dprism(s, name, |f| f.perform_thru_all()),
        Kas::LinearForm => {
            let mut f = s
                .features
                .linear_form
                .take()
                .unwrap_or_else(|| not_defined(kas));
            checked(f.perform());
            let error = f.rib().current_status_error();
            finish(s, name, &mut f, error, false);
            s.features.linear_form = Some(f);
        }
        Kas::RevolutionForm => {
            let mut f = s
                .features
                .revolution_form
                .take()
                .unwrap_or_else(|| not_defined(kas));
            checked(f.perform());
            let error = f.rib().current_status_error();
            finish(s, name, &mut f, error, false);
            s.features.revolution_form = Some(f);
        }
    }
}

fn run_prism(
    s: &mut FeatScript<'_>,
    name: &str,
    perform: impl FnOnce(&mut martensite_tkfeat::brep_feat::BRepFeatMakePrism) -> Result<(), Failure>,
) {
    let mut f = s
        .features
        .prism
        .take()
        .unwrap_or_else(|| not_defined(Kas::Prism));
    checked(perform(&mut f));
    let error = f.form().current_status_error();
    finish(s, name, &mut f, error, true);
    s.features.prism = Some(f);
}

fn run_dprism(
    s: &mut FeatScript<'_>,
    name: &str,
    perform: impl FnOnce(&mut martensite_tkfeat::brep_feat::BRepFeatMakeDPrism) -> Result<(), Failure>,
) {
    let mut f = s
        .features
        .dprism
        .take()
        .unwrap_or_else(|| not_defined(Kas::DPrism));
    checked(perform(&mut f));
    let error = f.form().current_status_error();
    finish(s, name, &mut f, error, true);
    s.features.dprism = Some(f);
}

fn run_revol(
    s: &mut FeatScript<'_>,
    name: &str,
    perform: impl FnOnce(&mut martensite_tkfeat::brep_feat::BRepFeatMakeRevol) -> Result<(), Failure>,
) {
    let mut f = s
        .features
        .revol
        .take()
        .unwrap_or_else(|| not_defined(Kas::Revol));
    checked(perform(&mut f));
    let error = f.form().current_status_error();
    finish(s, name, &mut f, error, true);
    s.features.revol = Some(f);
}

/// `featperform kind result until`: a null `until` ends the prism and drafted prism at the end of
/// the shape, and returns 1 for a revolution. Example: `featperform_until(s, Kas::prism, "r", "f")`.
pub fn featperform_until(s: &mut FeatScript<'_>, kas: Kas, name: &str, until: &str) {
    let until = s.shape_or_null(until);
    let null = until.is_null();
    match kas {
        Kas::Prism if null => run_prism(s, name, |f| f.perform_until_end()),
        Kas::Prism => run_prism(s, name, |f| f.perform_until(&until)),
        Kas::DPrism if null => run_dprism(s, name, |f| f.perform_until_end()),
        Kas::DPrism => run_dprism(s, name, |f| f.perform_until(&until)),
        Kas::Revol if null => command_failed(""),
        Kas::Revol => run_revol(s, name, |f| f.perform_until(&until)),
        Kas::LinearForm | Kas::RevolutionForm => {
            command_failed(&format!("invalid command for {}", kas.word()));
        }
    }
}

/// `featperform kind result from until`: a null `from` starts the prism and drafted prism at the
/// end of the shape. Example: `featperform_from_until(s, Kas::prism, "r", "ff", "fu")`.
pub fn featperform_from_until(
    s: &mut FeatScript<'_>,
    kas: Kas,
    name: &str,
    from: &str,
    until: &str,
) {
    let (from, until) = (s.shape_or_null(from), s.shape_or_null(until));
    if until.is_null() {
        command_failed("");
    }
    let null = from.is_null();
    match kas {
        Kas::Prism if null => run_prism(s, name, |f| f.perform_from_end(&until)),
        Kas::Prism => run_prism(s, name, |f| f.perform_from_until(&from, &until)),
        Kas::DPrism if null => run_dprism(s, name, |f| f.perform_from_end(&until)),
        Kas::DPrism => run_dprism(s, name, |f| f.perform_from_until(&from, &until)),
        Kas::Revol if null => command_failed(""),
        Kas::Revol => run_revol(s, name, |f| f.perform_from_until(&from, &until)),
        Kas::LinearForm | Kas::RevolutionForm => command_failed(""),
    }
}

/// `featperformval kind result value`: a length, height, or an angle in degrees.
/// Example: `featperformval(s, Kas::DPrism, "result", -100.)`.
pub fn featperformval(s: &mut FeatScript<'_>, kas: Kas, name: &str, value: f64) {
    match kas {
        Kas::Prism => run_prism(s, name, |f| f.perform_length(value)),
        Kas::Revol => run_revol(s, name, |f| f.perform_angle(value * (PI / 180.0))),
        Kas::DPrism => run_dprism(s, name, |f| f.perform_height(value)),
        Kas::LinearForm | Kas::RevolutionForm => {
            command_failed(&format!("invalid command for {}", kas.word()));
        }
    }
}

/// `featperformval kind result value until`.
/// Example: `featperformval_until(s, Kas::DPrism, "result", 100., "cyl_3")`.
pub fn featperformval_until(s: &mut FeatScript<'_>, kas: Kas, name: &str, value: f64, until: &str) {
    let until = s.shape_or_null(until);
    match kas {
        Kas::Prism => run_prism(s, name, |f| f.perform_until_height(&until, value)),
        Kas::Revol => run_revol(s, name, |f| {
            f.perform_until_angle(&until, value * (PI / 180.0))
        }),
        Kas::DPrism => run_dprism(s, name, |f| f.perform_until_height(&until, value)),
        Kas::LinearForm | Kas::RevolutionForm => {
            command_failed("invalid command for ribs or slots")
        }
    }
}
