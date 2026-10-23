//! `fixwgaps` and `fixsmall` (ShapeFix_Wireframe) and
//! `directfaces` (ShapeCustom::DirectFaces).
use martensite_tkbrep::topods::Shape;
use martensite_tkshhealing::shape_custom::ShapeCustom;
use martensite_tkshhealing::shape_fix::{FixRoot, ShapeFixShapeTolerance, ShapeFixWireframe};

/// What a command sets (`None`: it sets nothing) and prints.
pub struct MaybeSet {
    pub shape: Option<Shape>,
    pub text: String,
}

/// `fixwgaps result shape [toler=0]`: FixWireGaps with the precision; sets `result` and
/// prints ` Wireframe gaps fixed on shape` only when the repair did something.
pub fn fixwgaps(shape: &Shape, precision: Option<f64>) -> MaybeSet {
    let tool = ShapeFixWireframe::from_shape(shape);
    tool.set_precision(precision.unwrap_or(0.));
    let fixed = tool
        .fix_wire_gaps()
        .unwrap_or_else(|failure| panic!("fixwgaps: {failure:?}"));
    done_or_nothing(fixed, &tool, " Wireframe gaps fixed on shape\n")
}

/// `fixsmall result shape [toler=1.]`: FixSmallEdges with the precision; sets `result`
/// and prints ` Small edges fixed on shape` only when the repair did something.
pub fn fixsmall(shape: &Shape, precision: Option<f64>) -> MaybeSet {
    let tool = ShapeFixWireframe::from_shape(shape);
    tool.set_precision(precision.unwrap_or(1.));
    let fixed = tool
        .fix_small_edges()
        .unwrap_or_else(|failure| panic!("fixsmall: {failure:?}"));
    done_or_nothing(fixed, &tool, " Small edges fixed on shape\n")
}

fn done_or_nothing(fixed: bool, tool: &ShapeFixWireframe, text: &str) -> MaybeSet {
    if !fixed {
        return MaybeSet {
            shape: None,
            text: String::new(),
        };
    }
    MaybeSet {
        shape: Some(tool.shape()),
        text: text.into(),
    }
}

/// `directfaces result shape`: `NO RESULT` (an error) for a null result, `No modif`
/// (nothing set) when the result equals the input, else `DirectFaces -> Result : result`
/// and the result set.
pub fn directfaces(shape: &Shape, result_name: &str) -> MaybeSet {
    let result = ShapeCustom::direct_faces(shape)
        .unwrap_or_else(|failure| panic!("directfaces: {failure:?}"));
    assert!(!result.is_null(), "directfaces: NO RESULT");
    if result == *shape {
        return MaybeSet {
            shape: None,
            text: "No modif\n".into(),
        };
    }
    MaybeSet {
        shape: Some(result),
        text: format!("DirectFaces -> Result : {result_name}\n"),
    }
}

/// `settolerance shape val`: ShapeFix_ShapeTolerance::LimitTolerance
/// to `[val, val]` on every sub-shape; prints `Setting Tolerance to val` (`%.17g`).
pub fn settolerance(shape: &Shape, value: f64) -> String {
    ShapeFixShapeTolerance::new()
        .limit_tolerance(
            shape,
            value,
            value,
            martensite_tkg3d::top_abs::ShapeEnum::Shape,
        )
        .unwrap_or_else(|failure| panic!("settolerance: {failure:?}"));
    format!(
        "Setting Tolerance to {}\n",
        super::super::printed::g17(value)
    )
}
