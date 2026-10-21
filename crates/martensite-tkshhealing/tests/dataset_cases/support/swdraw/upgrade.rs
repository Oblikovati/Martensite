//! SWscenario_ShapeUpgrade.cxx `DT_ShapeDivide`, `DT_ShapeConvert`, `DT_SplitAngle`,
//! `DT_ClosedSplit`, `DT_ToBspl` and `DT_SplitByNumber`: the tool on the input shape,
//! its `Status: ...` lines, then `ShapeFix::SameParameter(res, false)` on the result.
use super::{same_parameter_fix, status_lines};
use martensite_tkbrep::topods::Shape;
use martensite_tkshhealing::shape_custom::ShapeCustom;
use martensite_tkshhealing::shape_upgrade::{
    ShapeUpgradeShapeConvertToBezier, ShapeUpgradeShapeDivideAngle, ShapeUpgradeShapeDivideArea,
    ShapeUpgradeShapeDivideClosed, ShapeUpgradeShapeDivideContinuity,
};
use std::f64::consts::PI;

/// The result of a DT_* command: the shape it sets and the text it prints.
pub struct Printed {
    pub shape: Shape,
    pub text: String,
}

fn fixed(shape: Shape, text: String) -> Printed {
    same_parameter_fix(&shape);
    Printed { shape, text }
}

fn failed(command: &str, failure: impl std::fmt::Debug) -> ! {
    panic!("{command}: {failure:?}")
}

/// `DT_ShapeDivide result shape [tol]`: ShapeUpgrade_ShapeDivideContinuity.
pub fn dt_shape_divide(shape: &Shape, tolerance: Option<f64>) -> Printed {
    let tool = ShapeUpgradeShapeDivideContinuity::new(shape.clone());
    if let Some(tolerance) = tolerance {
        tool.set_tolerance(tolerance);
    }
    tool.perform(true)
        .unwrap_or_else(|f| failed("DT_ShapeDivide", f));
    let text = status_lines(|flag| tool.status_flag(flag));
    fixed(tool.result(), text)
}

/// `DT_ShapeConvert result shape convert2d convert3d`: ShapeUpgrade_ShapeConvertToBezier
/// with the surface conversion on.
pub fn dt_shape_convert(shape: &Shape, convert_2d: bool, convert_3d: bool) -> Printed {
    let tool = ShapeUpgradeShapeConvertToBezier::new(shape.clone());
    tool.set_surface_conversion(true);
    if convert_2d {
        tool.set_2d_conversion(true);
    }
    if convert_3d {
        tool.set_3d_conversion(true);
    }
    tool.perform(true)
        .unwrap_or_else(|f| failed("DT_ShapeConvert", f));
    let text = status_lines(|flag| tool.status_flag(flag));
    fixed(tool.result(), text)
}

/// `DT_ShapeConvertRev result shape convert2d convert3d` (no `noline`/`nocircle`/`noconic`
/// words): ShapeCustom::ConvertToRevolution, printing `No modif` or `ConvertToRevolution
/// -> Result : `, then the Bezier conversion of DT_ShapeConvert; a null revolution result
/// is the command's `NO RESULT` error.
pub fn dt_shape_convert_rev(shape: &Shape, convert_2d: bool, convert_3d: bool) -> Printed {
    let revolved = ShapeCustom::convert_to_revolution(shape)
        .unwrap_or_else(|f| failed("DT_ShapeConvertRev ConvertToRevolution", f));
    assert!(!revolved.is_null(), "DT_ShapeConvertRev: NO RESULT");
    let mut text = if revolved == *shape {
        "No modif\n".to_string()
    } else {
        "ConvertToRevolution -> Result : \n".to_string()
    };
    let converted = dt_shape_convert(&revolved, convert_2d, convert_3d);
    text.push_str(&converted.text);
    Printed {
        shape: converted.shape,
        text,
    }
}

/// `DT_SplitAngle result shape [maxangle=95]`: ShapeUpgrade_ShapeDivideAngle (the angle in
/// degrees, at least 1).
pub fn dt_split_angle(shape: &Shape, max_angle: Option<f64>) -> Printed {
    let max_angle = max_angle.map_or(95.0, |angle| if angle < 1.0 { 1.0 } else { angle });
    let tool = ShapeUpgradeShapeDivideAngle::with_shape(max_angle * PI / 180.0, shape.clone());
    tool.perform(true)
        .unwrap_or_else(|f| failed("DT_SplitAngle", f));
    let text = status_lines(|flag| tool.status_flag(flag));
    fixed(tool.result(), text)
}

/// `DT_ClosedSplit result shape`: ShapeUpgrade_ShapeDivideClosed; it prints nothing.
pub fn dt_closed_split(shape: &Shape) -> Shape {
    let tool = ShapeUpgradeShapeDivideClosed::new(shape.clone());
    tool.perform(true)
        .unwrap_or_else(|f| failed("DT_ClosedSplit", f));
    fixed(tool.result(), String::new()).shape
}

/// `DT_ToBspl result shape [options=ero]`: ShapeCustom::ConvertToRevolution, then
/// ConvertToBSpline of the extrusion (e), revolution (r), offset (o) and plane (p)
/// surfaces the options name; it prints nothing.
pub fn dt_to_bspl(shape: &Shape, options: &str) -> Shape {
    let revolved = ShapeCustom::convert_to_revolution(shape)
        .unwrap_or_else(|f| failed("DT_ToBspl ConvertToRevolution", f));
    let result = ShapeCustom::convert_to_bspline(
        &revolved,
        options.contains('e'),
        options.contains('r'),
        options.contains('o'),
        options.contains('p'),
    )
    .unwrap_or_else(|f| failed("DT_ToBspl ConvertToBSpline", f));
    fixed(result, String::new()).shape
}

/// `DT_SplitByNumber result face number [number2]`: ShapeUpgrade_ShapeDivideArea splitting
/// by number, into `parts` or into `parts` by `number2` in U and V; it prints nothing.
pub fn dt_split_by_number(face: &Shape, parts: i32, number2: Option<i32>) -> Shape {
    let tool = ShapeUpgradeShapeDivideArea::new(face.clone());
    tool.set_splitting_by_number(true);
    match number2 {
        None => tool.set_nb_parts(parts),
        Some(number2) => tool
            .set_numbers_uv_splits(parts, number2)
            .unwrap_or_else(|f| failed("DT_SplitByNumber SetNumbersUVSplits", f)),
    }
    tool.perform(true)
        .unwrap_or_else(|f| failed("DT_SplitByNumber", f));
    fixed(tool.result(), String::new()).shape
}
