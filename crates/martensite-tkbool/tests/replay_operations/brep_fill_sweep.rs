//! BRepFillSweep probe operations, the Rust twin of
//! the recorded capture (the records are documented there).
use super::brep_fill_generator::{failed, indexed};
use super::brep_fill_location_law::{at, law_of};
use super::buffered_tokens::BufferedTokens;
use super::common::Reader;
use super::shape::shape;
use super::shapes::{TpShapes, tp_shapes};
use martensite_tkbool::brep_fill::{
    BRepFillLocationLaw, BRepFillSectionLaw, BRepFillShapeLaw, BRepFillSweep, SweepBuildOptions,
    SweepHistory, SweepTablesMap, TransitionStyle,
};
use martensite_tkbrep::topexp;
use martensite_tkbrep::topods::{self, Shape, Wire};
use martensite_tkbrep::toptools::{IndexedMapOfShape, MapOfShape};
use martensite_tkernel::ncollection::HArray2;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::top_abs::ShapeEnum;

/// `bl_sweep_table`.
fn sweep_table(
    output: &BufferedTokens,
    result: &IndexedMapOfShape,
    table: &Handle<HArray2<Shape>>,
) -> Result<(), Failure> {
    let Some(shared) = table.get() else {
        output.int(-1);
        return Ok(());
    };
    let t = shared.array2();
    for bound in [t.lower_row(), t.upper_row(), t.lower_col(), t.upper_col()] {
        output.int(bound);
    }
    for i in t.lower_row()..=t.upper_row() {
        for j in t.lower_col()..=t.upper_col() {
            indexed(
                output,
                result,
                t.value(i, j)
                    .map_err(|e| Failure::out_of_range(&e.to_string()))?,
            )?;
        }
    }
    Ok(())
}

/// The section law of the vertex or wire.
fn section_law(section: &Shape) -> Result<Handle<dyn BRepFillSectionLaw>, Failure> {
    if section.shape_type()? == ShapeEnum::Vertex {
        let law = BRepFillShapeLaw::with_vertex(&topods::vertex(section)?, true)?;
        return Ok(Handle::new(law).cast());
    }
    Ok(Handle::new(BRepFillShapeLaw::with_wire(&topods::wire(section)?, true)?).cast())
}

/// The transition of a request (0 modified, 1 right, 2 round).
fn transition_of(value: i32) -> TransitionStyle {
    match value {
        1 => TransitionStyle::Right,
        2 => TransitionStyle::Round,
        _ => TransitionStyle::Modified,
    }
}

/// The settings of a request after the laws.
struct SweepRequest {
    transition: TransitionStyle,
    kpart: bool,
    bounds: i32,
    tol3d: f64,
    angmin: f64,
    angmax: f64,
    c1: bool,
}

/// The settings read from the request.
fn request_of(r: &mut Reader<'_>) -> SweepRequest {
    let (transition, kpart, bounds) = (transition_of(r.integer()), r.flag(), r.integer());
    let (tol3d, angmin, angmax, c1) = (r.real(), r.real(), r.real(), r.flag());
    SweepRequest {
        transition,
        kpart,
        bounds,
        tol3d,
        angmin,
        angmax,
        c1,
    }
}

/// The sweep set up as the request says.
fn sweep_of(
    output: &BufferedTokens,
    sec: &Handle<dyn BRepFillSectionLaw>,
    loc: &Handle<dyn BRepFillLocationLaw>,
    section: &Shape,
    q: &SweepRequest,
) -> BRepFillSweep {
    let mut sweep = BRepFillSweep::new(sec, loc, q.kpart);
    sweep.set_tolerance(q.tol3d, 1.0, 1.0e-5, 1.0e-2);
    sweep.set_angular_control(q.angmin, q.angmax);
    sweep.set_force_approx_c1(q.c1);
    if q.bounds > 0 && section.shape_type().ok() == Some(ShapeEnum::Wire) {
        let w = Wire::unchecked(section.clone());
        let none = Wire::default();
        let first = if q.bounds == 3 { &none } else { &w };
        let last = if q.bounds >= 2 { &w } else { &none };
        output.attempt(|| sweep.set_bounds(first, last));
    }
    sweep
}

/// The outputs of a built sweep.
fn print_sweep(output: &BufferedTokens, sweep: &BRepFillSweep, sec: &dyn BRepFillSectionLaw) {
    output.flag(sweep.is_done());
    output.real(sweep.error_on_surface());
    let result = sweep.shape();
    output.attempt(|| shape(output, &result));
    let mut map = IndexedMapOfShape::default();
    if !result.is_null() {
        topexp::map_shapes(&result, &mut map, true, true);
    }
    output.attempt(|| sweep_table(output, &map, &sweep.sub_shape()));
    output.attempt(|| sweep_table(output, &map, &sweep.inter_faces()));
    output.attempt(|| sweep_table(output, &map, &sweep.sections()));
    if !sec.is_vertex() {
        for i in 1..=sec.nb_law().unwrap_or(0) {
            output.attempt(|| {
                output.int(sweep.tape(i)?.nb_children());
                Ok(())
            });
        }
    }
}

/// `bf_sweep program path section law... transition kpart bounds tol3d angmin angmax c1`.
pub fn sweep_case(output: &BufferedTokens, values: &[f64]) {
    let mut r = Reader::new(values);
    let t: TpShapes = match tp_shapes(&mut r) {
        Ok(t) => t,
        Err(error) => return failed(output, error),
    };
    let (path, section) = match (at(&t, r.integer()), at(&t, r.integer())) {
        (Ok(p), Ok(s)) => (p, s),
        (Err(error), _) | (_, Err(error)) => return failed(output, error),
    };
    let laws = law_of(output, &mut r, &t, &path).and_then(|loc| Ok((loc, section_law(&section)?)));
    let (loc, sec) = match laws {
        Ok(laws) => laws,
        Err(error) => return failed(output, error),
    };
    let q = request_of(&mut r);
    let Some(loc) = loc else {
        return output.end();
    };
    let mut sweep = sweep_of(output, &sec, &loc, &section, &q);
    let (mut reversed, mut tapes, mut rails) = (
        MapOfShape::default(),
        SweepTablesMap::default(),
        SweepTablesMap::default(),
    );
    let mut hist = SweepHistory {
        reversed_edges: &mut reversed,
        tapes: &mut tapes,
        rails: &mut rails,
    };
    let options = SweepBuildOptions {
        transition: q.transition,
        ..SweepBuildOptions::default()
    };
    if let Err(error) = sweep.build(&mut hist, &options) {
        return failed(output, error);
    }
    if let Some(sec) = sec.get() {
        print_sweep(output, &sweep, sec);
    }
    output.int(reversed.extent() as i32);
    output.int(tapes.extent() as i32);
    output.int(rails.extent() as i32);
    output.end();
}
