//! `removefeatures`, the history commands that read it, and the checks its scripts make on them.
use super::{
    removed_features::check_is_feature_removed, script_error::script_error, session::Session,
};
use crate::common_operation::case_bop_alerts::gravity_alert_text;
use martensite_tkbo::brep_algo_api::BRepAlgoAPIDefeaturing;
use martensite_tkbrep::{
    brep::BRepBuilder,
    brep_tools::History,
    topods::{Compound, Shape},
};
use martensite_tkernel::{message::Gravity, standard::Handle};
use martensite_tkg3d::top_abs::ShapeEnum;

/// The written answer of `modified` when the shape stayed as it was.
pub const NOT_MODIFIED: &str = "The shape has not been modified.";
/// The written answer of `generated` when it found nothing.
pub const NOT_GENERATED: &str = "No shapes were generated from the shape.";

/// The history the session keeps from the last removal.
/// Example: `let mut removal = FeatureRemoval::default();`.
#[derive(Default)]
pub struct FeatureRemoval {
    history: Option<Handle<History>>,
}

fn bound(s: &dyn Session, command: &str, name: &str) -> Shape {
    s.shape(name)
        .unwrap_or_else(|| script_error(&format!("{command}: {name} is a null shape.")))
        .clone()
}

impl FeatureRemoval {
    /// `removefeatures result shape feature...`; `removal.remove(s, "result", "s", &["s_2"])`.
    pub fn remove(&mut self, s: &mut dyn Session, result: &str, shape: &str, features: &[&str]) {
        let mut operation = BRepAlgoAPIDefeaturing::new();
        operation.set_shape(&bound(s, "removefeatures", shape));
        for feature in features {
            operation.add_face_to_remove(&bound(s, "removefeatures", feature));
        }
        operation.set_to_fill_history(true);
        operation.build();
        for (gravity, prefix) in [(Gravity::Warning, "Warning: "), (Gravity::Fail, "Error: ")] {
            print!(
                "{}",
                gravity_alert_text(operation.get_report(), gravity, prefix)
            );
        }
        self.history = Some(operation.history().unwrap_or_default());
        if !operation.has_errors() {
            s.bind(result, operation.shape());
        }
    }

    /// `savehistory name`; `let rf_hist = removal.savehistory();`.
    pub fn savehistory(&self) -> Handle<History> {
        self.history
            .clone()
            .unwrap_or_else(|| script_error("savehistory: no history in the session"))
    }

    /// `CheckIsFeatureRemoved feature {kinds}`; `removal.check_removed(s, "s_2", &[ShapeEnum::Face])`.
    pub fn check_removed(&self, s: &dyn Session, feature: &str, kinds: &[ShapeEnum]) {
        let history = self.savehistory();
        let history = history.get().expect("a history");
        check_is_feature_removed(history, &bound(s, "CheckIsFeatureRemoved", feature), kinds);
    }
}

/// One shape as it is, several in a compound, in the history's order.
fn bound_images(s: &mut dyn Session, name: &str, images: &[Shape]) {
    if let [only] = images {
        s.bind(name, only);
        return;
    }
    let builder = BRepBuilder::default();
    let mut compound = Compound::default();
    builder.make_compound(&mut compound);
    for image in images {
        builder.add(&mut compound, image).unwrap();
    }
    s.bind(name, &compound.into_shape());
}

/// `modified name history shape`: whether anything was bound; otherwise writes the command's answer.
/// Example: `modified(s, "m2", &rf_hist, "s_2")`.
pub fn modified(s: &mut dyn Session, name: &str, history: &Handle<History>, shape: &str) -> bool {
    let shape = bound(s, "modified", shape);
    let images = history.get().expect("a history").modified(&shape).unwrap();
    if images.is_empty() {
        println!("{NOT_MODIFIED}");
        return false;
    }
    bound_images(s, name, &images);
    true
}

/// `generated name history shape`: whether anything was bound; otherwise writes the answer.
/// Example: `generated(s, "g17", &rf_hist, "s_17")`.
pub fn generated(s: &mut dyn Session, name: &str, history: &Handle<History>, shape: &str) -> bool {
    let shape = bound(s, "generated", shape);
    let images = history.get().expect("a history").generated(&shape).unwrap();
    if images.is_empty() {
        println!("{NOT_GENERATED}");
        return false;
    }
    bound_images(s, name, &images);
    true
}

/// `if {![regexp "same shapes" [compare a b]]} {puts message}`.
/// Example: `require_same_shapes(s, "m3", "m5", "Error: incorrect feature removal")`.
pub fn require_same_shapes(s: &dyn Session, first: &str, second: &str, message: &str) {
    let same = bound(s, "compare", first).is_same(&bound(s, "compare", second));
    if !same {
        println!("{message}");
    }
}

/// `if {![regexp "No shapes were generated" [generated g h shape]]} {puts message}`.
/// Example: `require_not_generated(s, "g1", &rf_hist, "s_1", "Error: ...")`.
pub fn require_not_generated(
    s: &mut dyn Session,
    name: &str,
    history: &Handle<History>,
    shape: &str,
    message: &str,
) {
    if generated(s, name, history, shape) {
        println!("{message}");
    }
}
