//! BRepFillPipeShell probe operations, the Rust twin of
//! the recorded capture (the records are documented there).
use super::brep_fill_generator::{failed, list};
use super::brep_fill_location_law::at;
use super::buffered_tokens::BufferedTokens;
use super::common::Reader;
use super::shape::shape;
use super::shapes::{TpShapes, tp_shapes};
use martensite_tkbool::brep_fill::{BRepFillPipeShell, TransitionStyle, TypeOfContact};
use martensite_tkbrep::topexp;
use martensite_tkbrep::topods::{self, Shape};
use martensite_tkbrep::toptools::IndexedMapOfShape;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkgeomalgo::law::{LawFunction, LawLinear};
use martensite_tkmath::gp::Axis2;

/// The transition of a request (0 modified, 1 right, 2 round).
fn transition_of(value: i32) -> TransitionStyle {
    match value {
        1 => TransitionStyle::Right,
        2 => TransitionStyle::Round,
        _ => TransitionStyle::Modified,
    }
}

/// The contact of a request (0 none, 1 contact, 2 contact on border).
fn contact_of(value: i32) -> TypeOfContact {
    match value {
        1 => TypeOfContact::Contact,
        2 => TypeOfContact::ContactOnBorder,
        _ => TypeOfContact::NoContact,
    }
}

/// `bl_pipe_mode`.
fn pipe_mode(
    output: &BufferedTokens,
    r: &mut Reader<'_>,
    t: &TpShapes,
    pipe: &mut BRepFillPipeShell,
) -> Result<(), Failure> {
    match r.integer() {
        1 => pipe.set_frenet(true),
        2 => pipe.set_frenet(false),
        3 => pipe.set_discrete(),
        4 => {
            let (o, d, x) = (r.pnt(), r.dir(), r.dir());
            pipe.set_axe(&Axis2::with_x_direction(o, d, x)?)
        }
        5 => pipe.set_bi_normal(r.dir()),
        6 => {
            output.flag(pipe.set_spine_support(&at(t, r.integer())?)?);
            Ok(())
        }
        7 => {
            let g = topods::wire(&at(t, r.integer())?)?;
            let c = r.flag();
            pipe.set_auxiliary_spine(&g, c, contact_of(r.integer()))
        }
        _ => Ok(()),
    }
}

/// The pipe of the spine with its location mode.
fn pipe_of(
    output: &BufferedTokens,
    r: &mut Reader<'_>,
    t: &TpShapes,
) -> Result<BRepFillPipeShell, Failure> {
    let spine = topods::wire(&at(t, r.integer())?)?;
    let mut pipe = BRepFillPipeShell::new(&spine)?;
    pipe_mode(output, r, t, &mut pipe)?;
    Ok(pipe)
}

/// The settings and the profiles of the request; the first profile.
fn profiles(
    output: &BufferedTokens,
    r: &mut Reader<'_>,
    t: &TpShapes,
    pipe: &mut BRepFillPipeShell,
) -> Shape {
    let transition = transition_of(r.integer());
    let (angmin, angmax) = (r.real(), r.real());
    pipe.set_transition(transition, angmin, angmax);
    pipe.set_max_degree(r.integer());
    pipe.set_max_segments(r.integer());
    pipe.set_force_approx_c1(r.flag());
    let mut first = Shape::default();
    for k in 0..r.integer() {
        let p = at(t, r.integer()).unwrap_or_default();
        let v = r.integer();
        let (c, w) = (r.flag(), r.flag());
        if k == 0 {
            first = p.clone();
        }
        if v < 0 {
            output.attempt(|| pipe.add(&p, c, w));
        } else {
            output.attempt(|| pipe.add_at(&p, &topods::vertex(&at(t, v)?)?, c, w));
        }
    }
    if r.flag() {
        let lin = LawLinear::new();
        let (a, b, c, d) = (r.real(), r.real(), r.real(), r.real());
        output.attempt(|| {
            lin.set(a, b, c, d);
            let law: Handle<dyn LawFunction> = Handle::new(lin).cast();
            pipe.set_law(&first, &law, false, false)
        });
    }
    first
}

/// The outputs after Build.
fn print_pipe(
    output: &BufferedTokens,
    pipe: &mut BRepFillPipeShell,
    t: &TpShapes,
    built: bool,
    solid: bool,
) {
    output.int(pipe.get_status() as i32);
    if built {
        output.real(pipe.error_on_surface());
    }
    output.attempt(|| shape(output, pipe.shape()));
    output.attempt(|| shape(output, pipe.first_shape()));
    output.attempt(|| shape(output, pipe.last_shape()));
    if solid && built {
        output.attempt(|| {
            output.flag(pipe.make_solid()?);
            shape(output, pipe.shape())
        });
    }
    let mut map = IndexedMapOfShape::default();
    if !pipe.shape().is_null() {
        topexp::map_shapes(pipe.shape(), &mut map, true, true);
    }
    for s in &t.shapes {
        output.attempt(|| list(output, &map, &pipe.generated(s)));
    }
    if built {
        output.attempt(|| {
            let sections = pipe.simulate(3)?;
            output.int(sections.len() as i32);
            for s in sections.iter() {
                shape(output, s)?;
            }
            Ok(())
        });
    }
}

/// `bf_pipe_shell program spine mode... transition angmin angmax maxdeg maxseg c1 n
/// (profile vertex contact correction)... law [pdeb valdeb pfin valfin] solid`.
pub fn pipe_shell_case(output: &BufferedTokens, values: &[f64]) {
    let mut r = Reader::new(values);
    let t = match tp_shapes(&mut r) {
        Ok(t) => t,
        Err(error) => return failed(output, error),
    };
    let mut pipe = match pipe_of(output, &mut r, &t) {
        Ok(pipe) => pipe,
        Err(error) => return failed(output, error),
    };
    profiles(output, &mut r, &t, &mut pipe);
    let solid = r.flag();
    output.flag(pipe.is_ready());
    let built = match pipe.build() {
        Ok(built) => built,
        Err(error) => return failed(output, error),
    };
    output.flag(built);
    print_pipe(output, &mut pipe, &t, built, solid);
    output.end();
}
