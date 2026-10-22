//! The QABugs commands the TKShHealing cases call (src/Draw/TKQADraw/QABugs at
//! 3d097a0328e71b826377d4814ab05ec3c3d23871), on the Rust API. Each returns what it sets
//! and the text it prints; a `Standard_Failure` the command catches becomes its
//! `... Exception` text, as natively.
use martensite_tkbrep::{brep_tools::ReShape, topods::Shape};
use martensite_tkernel::{message::ProgressRange, precision, standard::Handle};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkshhealing::shape_build::ShapeBuildReShape;
use martensite_tkshhealing::shape_extend::ShapeExtendStatus;
use martensite_tkshhealing::shape_process::{
    ShapeProcess, ShapeProcessOperLibrary, ShapeProcessShapeContext,
};
use martensite_tkshhealing::shape_upgrade::ShapeUpgradeShapeDivideAngle;

/// What a QA command sets (`None`: nothing) and prints.
pub struct QaOutput {
    pub shape: Option<Shape>,
    pub text: String,
}

/// QABugs_11.cxx `OCC22 result shape compound considerLocation`: the sub-shapes of
/// `compound` split by ShapeUpgrade_ShapeDivideAngle(PI/2) (precision Confusion, max
/// tolerance 0.1) into a ShapeBuild_ReShape context, which then rebuilds `shape`.
pub fn occ22(shape: &Shape, compound: &Shape, consider_location: bool) -> QaOutput {
    let attempt = || -> Result<QaOutput, martensite_tkernel::standard::Failure> {
        let upgrade = ShapeUpgradeShapeDivideAngle::new(std::f64::consts::PI / 2.);
        upgrade.set_precision(precision::confusion());
        upgrade.set_max_tolerance(0.1);
        upgrade.init(compound);
        let reshape = Handle::new(ShapeBuildReShape::new());
        upgrade.set_context(&reshape);
        if consider_location {
            reshape.set_mode_consider_location(true);
        }
        let mut text = String::new();
        if upgrade.perform(false)? {
            text.push_str("Upgrade_SplitRevolution_Done \n");
        } else if upgrade.status_flag(ShapeExtendStatus::Ok) {
            text.push_str("Upgrade_SplitRevolution_OK \n");
        } else if upgrade.status_flag(ShapeExtendStatus::Fail) {
            text.push_str("OCC22 FAULTY. Operation failed. Angle was not divided\n");
            return Ok(QaOutput { shape: None, text });
        }
        let result = reshape.apply(shape, ShapeEnum::Shape)?;
        Ok(QaOutput {
            shape: Some(result),
            text,
        })
    };
    attempt().unwrap_or_else(|_| QaOutput {
        shape: None,
        text: "OCC22 Exception \n".into(),
    })
}

/// QABugs_11.cxx `OCC24 result shape compound resource sequence`: the ShapeProcess
/// sequence run on a shape context of `compound` (detalisation EDGE), its map replayed
/// into a ShapeBuild_ReShape that rebuilds `shape`.
pub fn occ24(shape: &Shape, compound: &Shape, resource: &str, sequence: &str) -> QaOutput {
    let attempt = || -> Result<Shape, martensite_tkernel::standard::Failure> {
        ShapeProcessOperLibrary::init()?;
        let context = Handle::new(ShapeProcessShapeContext::with_shape(
            compound, resource, "",
        )?);
        context.set_detalisation(ShapeEnum::Edge);
        ShapeProcess::perform_sequence(
            &context.clone().cast(),
            sequence,
            &ProgressRange::default(),
        )?;
        let reshape = ShapeBuildReShape::new();
        for (key, value) in context.map().items() {
            reshape.replace(key, value)?;
        }
        reshape.apply(shape, ShapeEnum::Shape)
    };
    match attempt() {
        Ok(result) => QaOutput {
            shape: Some(result),
            text: String::new(),
        },
        Err(_) => QaOutput {
            shape: None,
            text: "OCC24 Exception \n".into(),
        },
    }
}
