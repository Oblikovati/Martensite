//! Original constraint-array constructors and ordered multi-point commands.
use super::{
    common::{self as p, Reader, attempt, end},
    fixture_input::GeometryFixtureInput,
    linear_tokens::dump,
    multi_curve_tokens::point,
    print::Output,
};
use martensite_tkernel::{ncollection::Array1, standard::Failure};
use martensite_tkgeombase::app_def::MultiPointConstraint;
use martensite_tkmath::gp::{Point2, Point3, Vector2, Vector3};

/// An array of `values` from `lower`.
pub fn array<T: Clone + Default>(lower: i32, values: &[T]) -> Array1<'static, T> {
    let mut a: Array1<'static, T> = Array1::with_size(values.len());
    a.update_lower_bound(lower);
    a.as_mut_slice().clone_from_slice(values);
    a
}

/// The points and vector arrays of an `ad_mpc` request.
struct MpcArrays {
    p3: Vec<Point3>,
    p2: Vec<Point2>,
    v3: Vec<Vector3>,
    v2: Vec<Vector2>,
    c3: Vec<Vector3>,
    c2: Vec<Vector2>,
}

fn read_arrays(r: &mut impl GeometryFixtureInput, counts: [i32; 6]) -> MpcArrays {
    let [n3, n2, nv3, nv2, nc3, nc2] = counts;
    MpcArrays {
        p3: (0..n3).map(|_| r.pnt()).collect(),
        p2: (0..n2).map(|_| r.pnt2d()).collect(),
        v3: (0..nv3).map(|_| r.vec()).collect(),
        v2: (0..nv2).map(|_| r.vec2d()).collect(),
        c3: (0..nc3).map(|_| r.vec()).collect(),
        c2: (0..nc2).map(|_| r.vec2d()).collect(),
    }
}

/// `gb_ad_read_mpc`: the constructor of `kind` over arrays starting at `lower`.
pub fn read_mpc(r: &mut impl GeometryFixtureInput) -> Result<MultiPointConstraint, Failure> {
    let (kind, lower) = (r.integer(), r.integer());
    let counts = [0; 6].map(|_| r.integer());
    let a = read_arrays(r, counts);
    let (p3, p2) = (array(lower, &a.p3), array(lower, &a.p2));
    let (v3, v2, c3, c2) = (
        array(lower, &a.v3),
        array(lower, &a.v2),
        array(lower, &a.c3),
        array(lower, &a.c2),
    );
    Ok(match kind {
        0 => MultiPointConstraint::new(),
        1 => MultiPointConstraint::with_counts(counts[0], counts[1])?,
        2 => MultiPointConstraint::from_points(&p3),
        3 => MultiPointConstraint::from_points2d(&p2),
        4 => MultiPointConstraint::from_points_and_2d(&p3, &p2),
        5 => MultiPointConstraint::with_tangents_and_curvatures(&p3, &p2, &v3, &v2, &c3, &c2)?,
        6 => MultiPointConstraint::with_tangents(&p3, &p2, &v3, &v2)?,
        7 => MultiPointConstraint::with_tangents_and_curvatures_3d(&p3, &v3, &c3)?,
        8 => MultiPointConstraint::with_tangents_3d(&p3, &v3)?,
        9 => MultiPointConstraint::with_tangents_2d(&p2, &v2)?,
        _ => MultiPointConstraint::with_tangents_and_curvatures_2d(&p2, &v2, &c2)?,
    })
}

/// `gb_ad_mpc`: the points and the two flags.
pub fn mpc(output: &Output, m: &MultiPointConstraint) -> Result<(), Failure> {
    point(output, m)?;
    p::flag(output, m.is_tangency_point());
    p::flag(output, m.is_curvature_point());
    Ok(())
}

fn mpc_command(
    output: &Output,
    r: &mut impl GeometryFixtureInput,
    m: &mut MultiPointConstraint,
) -> Result<(), Failure> {
    let code = r.integer();
    match code {
        1 => {
            let i = r.integer();
            m.set_tang(i, r.vec())
        }
        2 => m.tang(r.integer()).map(|v| p::vec(output, v)),
        3 => {
            let i = r.integer();
            m.set_tang2d(i, r.vec2d())
        }
        4 => m.tang2d(r.integer()).map(|v| p::vec2d(output, v)),
        5 => {
            let i = r.integer();
            m.set_curv(i, r.vec())
        }
        6 => m.curv(r.integer()).map(|v| p::vec(output, v)),
        7 => {
            let i = r.integer();
            m.set_curv2d(i, r.vec2d())
        }
        8 => m.curv2d(r.integer()).map(|v| p::vec2d(output, v)),
        9 => {
            p::flag(output, m.is_tangency_point());
            p::flag(output, m.is_curvature_point());
            Ok(())
        }
        10 => {
            let mut text = String::new();
            let _ = m.dump(&mut text);
            dump(output, &text);
            Ok(())
        }
        11 => {
            let mut copy = m.clone();
            let (i, v) = (r.integer(), r.vec());
            attempt(output, || copy.set_tang(i, v));
            p::flag(output, m.is_tangency_point());
            attempt(output, || m.tang(i).map(|v| p::vec(output, v)));
            Ok(())
        }
        _ => mpc(output, m),
    }
}

pub fn mpc_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    attempt(output, || {
        let mut m = read_mpc(&mut r)?;
        mpc(output, &m)?;
        let count = r.integer();
        for _ in 0..count {
            attempt(output, || mpc_command(output, &mut r, &mut m));
        }
        Ok(())
    });
    end(output);
}
