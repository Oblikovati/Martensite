//! `checkshape name prefix`: the structural dump, as text, with the count of each kind of faulty shape.
//! The compounds it binds are not read by any case, so only their counts are kept.
use martensite_tkbrep::{
    topexp::Explorer,
    topods::{Shape, ShapeIterator},
    toptools::ShapeMapHasher,
};
use martensite_tkernel::{
    ncollection::DataMap,
    standard::{Failure, Handle},
};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tktopalgo::brep_check::{BRepCheckAnalyzer, BRepCheckResult, BRepCheckStatus};

const NUMBER_OF_STATUS: usize = 36;
type ShapeContextMap = DataMap<Shape, Vec<Shape>, ShapeMapHasher>;

fn contains(list: &[Shape], shape: &Shape) -> bool {
    list.iter().any(|s| s.is_same(shape))
}

/// The lines of the structural dump, in the order.
pub(super) const PROBLEM_LINES: &[(BRepCheckStatus, &str)] = &[
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

/// `THE_SHAPE_CONTEXT_MAP(sub)`: a no such object failure when `sub` is not bound.
fn context_of<'a>(
    map: &'a mut ShapeContextMap,
    sub: &Shape,
) -> Result<&'a mut Vec<Shape>, Failure> {
    map.change_find(sub)
}

/// The first status of a list (`itl.Value()` after `initialize`).
fn first_status(
    list: &martensite_tktopalgo::brep_check::ListOfStatus,
) -> Result<BRepCheckStatus, Failure> {
    list.iter()
        .next()
        .copied()
        .ok_or_else(|| Failure::with_message("Standard_NoSuchObject", "List::Iterator::Value"))
}

/// The checker of a sub-shape, a null object failure when there is none.
fn live_result(result: &Handle<dyn BRepCheckResult>) -> Result<&dyn BRepCheckResult, Failure> {
    result
        .get()
        .ok_or_else(|| Failure::with_message("Standard_NullObject", "BRepCheckResult"))
}

/// The statics of the structural dump: `THE_SHAPE_CONTEXT_MAP`, the problem shapes
/// and the count of each status.
struct ProblemCollector<'a> {
    analyzer: &'a BRepCheckAnalyzer,
    map: ShapeContextMap,
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

    /// `GetProblemSub(Ana, Shape, sl, NbProblems, Subtype)`.
    fn problem_sub(&mut self, shape: &Shape, sub_type: ShapeEnum) -> Result<(), Failure> {
        let mut explorer = Explorer::new(shape, sub_type, ShapeEnum::Shape)?;
        while explorer.more() {
            let sub = explorer.current().clone();
            let handle = self.analyzer.result(&sub)?;
            let res = live_result(&handle)?;
            res.init_context_iterator()?;
            while res.more_shape_in_context() {
                if res.contextual_shape()?.is_same(shape)
                    && !contains(context_of(&mut self.map, &sub)?, shape)
                {
                    context_of(&mut self.map, &sub)?.push(shape.clone());
                    let status = first_status(&res.status_on_shape()?)?;
                    if status != BRepCheckStatus::NoError {
                        if !self.shapes.iter().any(|s| s.is_same(&sub)) {
                            self.shapes.push(sub.clone());
                            self.fill_problems(status);
                        }
                        if !self.shapes.iter().any(|s| s.is_same(shape)) {
                            self.shapes.push(shape.clone());
                            self.fill_problems(status);
                        }
                    }
                    break;
                }
                res.next_shape_in_context();
            }
            explorer.next()?;
        }
        Ok(())
    }

    /// `GetProblemShapes(Ana, Shape, sl, NbProblems)`.
    fn problem_shapes(&mut self, shape: &Shape) -> Result<(), Failure> {
        let mut iter = ShapeIterator::new(shape, true, true);
        while iter.more() {
            let child = iter.value()?.clone();
            self.problem_shapes(&child)?;
            iter.next();
        }
        let style = shape.shape_type()?;
        let handle = self.analyzer.result(shape)?;
        if let Some(res) = handle.get()
            && !self.map.is_bound(shape)
        {
            let status = first_status(&res.status()?)?;
            if status != BRepCheckStatus::NoError {
                self.shapes.push(shape.clone());
                self.fill_problems(status);
            }
        }
        if !self.map.is_bound(shape) {
            self.map.bind(shape.clone(), Vec::new());
        }
        match style {
            ShapeEnum::Edge => self.problem_sub(shape, ShapeEnum::Vertex)?,
            ShapeEnum::Face => {
                self.problem_sub(shape, ShapeEnum::Wire)?;
                self.problem_sub(shape, ShapeEnum::Edge)?;
                self.problem_sub(shape, ShapeEnum::Vertex)?;
            }
            ShapeEnum::Solid => self.problem_sub(shape, ShapeEnum::Shell)?,
            _ => {}
        }
        Ok(())
    }
}

/// `checkshape name pref` of a shape: ` -- The Shape name looks OK` or the dump.
/// Example: `structural_check(&shape, "res_1", "ff")`.
pub fn structural_check(shape: &Shape, name: &str, pref: &str) -> Result<String, Failure> {
    let analyzer = BRepCheckAnalyzer::new(shape, true, false, false)?;
    if analyzer.is_valid()? {
        return Ok(format!(" -- The Shape {name} looks OK"));
    }
    structural_dump(&analyzer, name, pref, shape)
}

fn structural_dump(
    analyzer: &BRepCheckAnalyzer,
    name: &str,
    pref: &str,
    shape: &Shape,
) -> Result<String, Failure> {
    let mut text = format!(
        " -- The Shape {name} has problems :\n  Check                                    Count\n ------------------------------------------------\n"
    );
    let mut collector = ProblemCollector {
        analyzer,
        map: ShapeContextMap::default(),
        shapes: Vec::new(),
        nb_problems: [0; NUMBER_OF_STATUS + 1],
    };
    collector.problem_shapes(shape)?;
    for (status, label) in PROBLEM_LINES {
        let count = collector.nb_problems[*status as usize];
        if count > 0 {
            text += &format!("{label}{count}\n");
        }
    }
    text += " ------------------------------------------------\n";
    text += &format!("*** Shapes with problems : {}\n", collector.shapes.len());
    let groups = [
        (ShapeEnum::Vertex, "v", "VERTEX"),
        (ShapeEnum::Edge, "e", "EDGE"),
        (ShapeEnum::Wire, "w", "WIRE"),
        (ShapeEnum::Face, "f", "FACE"),
        (ShapeEnum::Shell, "s", "SHELL"),
        (ShapeEnum::Solid, "o", "SOLID"),
    ];
    for (kind, suffix, label) in groups {
        let mut nb = 0;
        for faulty in &collector.shapes {
            nb += usize::from(faulty.shape_type()? == kind);
        }
        if nb > 0 {
            let pad = if nb > 9 { "" } else { " " };
            text += &format!("{label}\t: {pad}{nb} Items -> compound named {pref}_{suffix}\n");
        }
    }
    Ok(text)
}
