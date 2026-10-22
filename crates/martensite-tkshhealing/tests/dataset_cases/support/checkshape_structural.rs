//! BRepTest_CheckCommands.cxx `checkshape shape prefix` (the structural output style):
//! ` -- The Shape <name> looks OK`, or the count of each problem and the faulty shapes by
//! type (the `<prefix>_v`, `_e`, ... compounds are scenario variables the cases never read).
use super::checkshape::CheckOptions;
use martensite_tkbrep::{
    topexp::Explorer,
    topods::{Shape, ShapeIterator},
    toptools::ShapeMapHasher,
};
use martensite_tkernel::ncollection::DataMap;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tktopalgo::brep_check::{BRepCheckAnalyzer, BRepCheckStatus, ListOfStatus};

/// `NumberOfStatus`: the count of the `BRepCheck_Status` values without `NoError`.
const NUMBER_OF_STATUS: usize = 36;

/// The logged result of checking a named shape.
pub fn checkshape_structural(
    shape: &Shape,
    name: &str,
    prefix: &str,
    options: CheckOptions,
) -> String {
    let text = structural_text(shape, name, prefix, options);
    super::log::result(&text);
    text
}

fn structural_text(shape: &Shape, name: &str, prefix: &str, options: CheckOptions) -> String {
    let analyzer = BRepCheckAnalyzer::new(shape, !options.top, options.parallel, options.exact)
        .unwrap_or_else(|failure| panic!("checkshape exception : {failure:?}"));
    if analyzer.is_valid().unwrap() {
        return format!(" -- The Shape {name} looks OK");
    }
    let mut collector = ProblemCollector {
        analyzer: &analyzer,
        map: DataMap::default(),
        shapes: Vec::new(),
        nb_problems: [0; NUMBER_OF_STATUS + 1],
    };
    collector.problem_shapes(shape);
    let mut text = format!(" -- The Shape {name} has problems :\n");
    text.push_str("  Check                                    Count\n");
    text.push_str(" ------------------------------------------------\n");
    for (status, label) in PROBLEM_LINES {
        let count = collector.nb_problems[*status as usize];
        if count > 0 {
            text.push_str(&format!("{label}{count}\n"));
        }
    }
    text.push_str(" ------------------------------------------------\n");
    let shapes = collector.shapes;
    text.push_str(&format!("*** Shapes with problems : {}\n", shapes.len()));
    let groups = [
        (ShapeEnum::Vertex, "v", "VERTEX"),
        (ShapeEnum::Edge, "e", "EDGE"),
        (ShapeEnum::Wire, "w", "WIRE"),
        (ShapeEnum::Face, "f", "FACE"),
        (ShapeEnum::Shell, "s", "SHELL"),
        (ShapeEnum::Solid, "o", "SOLID"),
    ];
    for (kind, suffix, label) in groups {
        let nb = shapes
            .iter()
            .filter(|s| s.shape_type().unwrap() == kind)
            .count();
        if nb == 0 {
            continue;
        }
        let pad = if nb > 9 { "" } else { " " };
        text.push_str(&format!(
            "{label}\t: {pad}{nb} Items -> compound named {prefix}_{suffix}\n"
        ));
    }
    text
}

fn first_status(list: &ListOfStatus) -> BRepCheckStatus {
    *list
        .iter()
        .next()
        .expect("NCollection_List::Iterator::Value")
}

/// The statics of the structural dump: `THE_SHAPE_CONTEXT_MAP`, the problem shapes and
/// the count of each status.
struct ProblemCollector<'a> {
    analyzer: &'a BRepCheckAnalyzer,
    map: DataMap<Shape, Vec<Shape>, ShapeMapHasher>,
    shapes: Vec<Shape>,
    nb_problems: [i32; NUMBER_OF_STATUS + 1],
}

impl ProblemCollector<'_> {
    /// `FillProblems(stat, NbProblems)`.
    fn fill_problems(&mut self, status: BRepCheckStatus) {
        let id = status as usize;
        if (1..=NUMBER_OF_STATUS).contains(&id) {
            self.nb_problems[id] += 1;
        }
    }

    fn add_problem(&mut self, shape: &Shape, status: BRepCheckStatus) {
        if !self.shapes.iter().any(|s| s.is_same(shape)) {
            self.shapes.push(shape.clone());
            self.fill_problems(status);
        }
    }

    /// `GetProblemSub(Ana, Shape, sl, NbProblems, Subtype)`.
    fn problem_sub(&mut self, shape: &Shape, sub_type: ShapeEnum) {
        let mut explorer = Explorer::new(shape, sub_type, ShapeEnum::Shape).unwrap();
        while explorer.more() {
            let sub = explorer.current().clone();
            let handle = self.analyzer.result(&sub).unwrap();
            let res = handle.get().expect("BRepCheck_Result");
            res.init_context_iterator().unwrap();
            while res.more_shape_in_context() {
                let seen = self.map.change_find(&sub).unwrap();
                if res.contextual_shape().unwrap().is_same(shape)
                    && !seen.iter().any(|s| s.is_same(shape))
                {
                    seen.push(shape.clone());
                    let status = first_status(&res.status_on_shape().unwrap());
                    if status != BRepCheckStatus::NoError {
                        self.add_problem(&sub, status);
                        self.add_problem(shape, status);
                    }
                    break;
                }
                res.next_shape_in_context();
            }
            explorer.next().unwrap();
        }
    }

    /// `GetProblemShapes(Ana, Shape, sl, NbProblems)`.
    fn problem_shapes(&mut self, shape: &Shape) {
        let mut iterator = ShapeIterator::new(shape, true, true);
        while let Ok(child) = iterator.value() {
            let child = child.clone();
            self.problem_shapes(&child);
            iterator.next();
        }
        let handle = self.analyzer.result(shape).unwrap();
        if let Some(res) = handle.get()
            && !self.map.is_bound(shape)
        {
            let status = first_status(&res.status().unwrap());
            if status != BRepCheckStatus::NoError {
                self.shapes.push(shape.clone());
                self.fill_problems(status);
            }
        }
        if !self.map.is_bound(shape) {
            self.map.bind(shape.clone(), Vec::new());
        }
        match shape.shape_type().unwrap() {
            ShapeEnum::Edge => self.problem_sub(shape, ShapeEnum::Vertex),
            ShapeEnum::Face => {
                self.problem_sub(shape, ShapeEnum::Wire);
                self.problem_sub(shape, ShapeEnum::Edge);
                self.problem_sub(shape, ShapeEnum::Vertex);
            }
            ShapeEnum::Solid => self.problem_sub(shape, ShapeEnum::Shell),
            _ => {}
        }
    }
}

/// The structural dump's lines, in the native order.
const PROBLEM_LINES: &[(BRepCheckStatus, &str)] = &[
    (
        BRepCheckStatus::InvalidPointOnCurve,
        "  Invalid Point on Curve ................... ",
    ),
    (
        BRepCheckStatus::InvalidPointOnCurveOnSurface,
        "  Invalid Point on CurveOnSurface .......... ",
    ),
    (
        BRepCheckStatus::InvalidPointOnSurface,
        "  Invalid Point on Surface ................. ",
    ),
    (
        BRepCheckStatus::No3DCurve,
        "  No 3D Curve .............................. ",
    ),
    (
        BRepCheckStatus::Multiple3DCurve,
        "  Multiple 3D Curve ........................ ",
    ),
    (
        BRepCheckStatus::Invalid3DCurve,
        "  Invalid 3D Curve ......................... ",
    ),
    (
        BRepCheckStatus::NoCurveOnSurface,
        "  No Curve on Surface ...................... ",
    ),
    (
        BRepCheckStatus::InvalidCurveOnSurface,
        "  Invalid Curve on Surface ................. ",
    ),
    (
        BRepCheckStatus::InvalidCurveOnClosedSurface,
        "  Invalid Curve on closed Surface .......... ",
    ),
    (
        BRepCheckStatus::InvalidSameRangeFlag,
        "  Invalid SameRange Flag ................... ",
    ),
    (
        BRepCheckStatus::InvalidSameParameterFlag,
        "  Invalid SameParameter Flag ............... ",
    ),
    (
        BRepCheckStatus::InvalidDegeneratedFlag,
        "  Invalid Degenerated Flag ................. ",
    ),
    (
        BRepCheckStatus::FreeEdge,
        "  Free Edge ................................ ",
    ),
    (
        BRepCheckStatus::InvalidMultiConnexity,
        "  Invalid MultiConnexity ................... ",
    ),
    (
        BRepCheckStatus::InvalidRange,
        "  Invalid Range ............................ ",
    ),
    (
        BRepCheckStatus::EmptyWire,
        "  Empty Wire ............................... ",
    ),
    (
        BRepCheckStatus::RedundantEdge,
        "  Redundant Edge ........................... ",
    ),
    (
        BRepCheckStatus::SelfIntersectingWire,
        "  Self Intersecting Wire ................... ",
    ),
    (
        BRepCheckStatus::NoSurface,
        "  No Surface ............................... ",
    ),
    (
        BRepCheckStatus::InvalidWire,
        "  Invalid Wire ............................. ",
    ),
    (
        BRepCheckStatus::RedundantWire,
        "  Redundant Wire ........................... ",
    ),
    (
        BRepCheckStatus::IntersectingWires,
        "  Intersecting Wires ....................... ",
    ),
    (
        BRepCheckStatus::InvalidImbricationOfWires,
        "  Invalid Imbrication of Wires ............. ",
    ),
    (
        BRepCheckStatus::EmptyShell,
        "  Empty Shell .............................. ",
    ),
    (
        BRepCheckStatus::RedundantFace,
        "  Redundant Face ........................... ",
    ),
    (
        BRepCheckStatus::UnorientableShape,
        "  Unorientable Shape ....................... ",
    ),
    (
        BRepCheckStatus::NotClosed,
        "  Not Closed ............................... ",
    ),
    (
        BRepCheckStatus::NotConnected,
        "  Not Connected ............................ ",
    ),
    (
        BRepCheckStatus::SubshapeNotInShape,
        "  Subshape not in Shape .................... ",
    ),
    (
        BRepCheckStatus::BadOrientation,
        "  Bad Orientation .......................... ",
    ),
    (
        BRepCheckStatus::BadOrientationOfSubshape,
        "  Bad Orientation of Subshape .............. ",
    ),
    (
        BRepCheckStatus::InvalidToleranceValue,
        "  Invalid tolerance value................... ",
    ),
    (
        BRepCheckStatus::InvalidPolygonOnTriangulation,
        "  Invalid polygon on triangulation.......... ",
    ),
    (
        BRepCheckStatus::InvalidImbricationOfShells,
        "  Invalid Imbrication of Shells............. ",
    ),
    (
        BRepCheckStatus::EnclosedRegion,
        "  Enclosed Region........................... ",
    ),
    (
        BRepCheckStatus::CheckFail,
        "  checkshape failure........................ ",
    ),
];
