//! B-spline surfaces given as degree, knots with multiplicities and weighted poles, and the
//! faces over their natural bounds.
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::ncollection::{Array1, Array2};
use martensite_tkernel::precision;
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::geom::{GeomBSplineSurface, GeomSurface};
use martensite_tkmath::gp::Point3;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeFace;
use std::error::Error;

/// A non-periodic B-spline surface; poles are listed row by row of constant V, U fastest.
pub(crate) struct SplineSurface {
    pub degrees: (i32, i32),
    pub u_knots: &'static [(f64, i32)],
    pub v_knots: &'static [(f64, i32)],
    /// `[x, y, z, weight]` per pole.
    pub poles: &'static [[f64; 4]],
}

/// The degree 2 x 1 surface of the B-spline mesh regressions (5 x 2 poles).
pub(crate) const NARROW_SPLINE: SplineSurface = SplineSurface {
    degrees: (2, 1),
    u_knots: &[(0.0, 3), (0.34, 1), (0.67, 1), (1.0, 3)],
    v_knots: &[(0.0, 2), (1.0, 2)],
    poles: &[
        [10., 0., 0., 1.],
        [6., 0., 0., 1.],
        [4., 0., 0., 1.],
        [2., 0., 0., 1.],
        [0., 0., 0., 1.],
        [10., 5., 0., 1.],
        [6., 5., 0., 1.],
        [4., 5., 0., 1.],
        [1.5, 9., 0., 1.],
        [0., 5., 0., 1.],
    ],
};

/// The longer degree 2 x 1 surface with clustered knots (13 x 2 poles).
pub(crate) const LONG_SPLINE: SplineSurface = SplineSurface {
    degrees: (2, 1),
    u_knots: &[
        (0.0, 3),
        (0.1, 1),
        (0.2, 1),
        (0.3, 1),
        (0.4, 1),
        (0.5, 1),
        (0.525, 1),
        (0.55, 1),
        (0.575, 1),
        (0.8, 1),
        (0.9, 1),
        (1.0, 3),
    ],
    v_knots: &[(0.0, 2), (1.0, 2)],
    poles: &[
        [16., 0., 0., 1.],
        [14., 0., 0., 1.],
        [12., 0., 0., 1.],
        [10., 0., 0., 1.],
        [8., 0., 0., 1.],
        [6., 0., 0., 1.],
        [4., 0., 0., 1.],
        [2., 0., 0., 1.],
        [0., 0., 0., 1.],
        [-2., 0., 0., 1.],
        [-4., 0., 0., 1.],
        [-6., 0., 0., 1.],
        [-8., 0., 0., 1.],
        [16., 5., 0., 1.],
        [14., 5., 0., 1.],
        [12., 5., 0., 1.],
        [10., 5., 0., 1.],
        [8., 5., 0., 1.],
        [6., 5., 0., 1.],
        [4., 5., 0., 1.],
        [1.5, 9., 0., 1.],
        [0., 5., 0., 1.],
        [-2., 5., 0., 1.],
        [-4., 5., 0., 1.],
        [-6., 5., 0., 1.],
        [-8., 5., 0., 1.],
    ],
};

/// The face over the whole surface at the confusion tolerance; `bspline_face(&NARROW_SPLINE)?`.
pub(crate) fn bspline_face(spline: &SplineSurface) -> Result<Shape, Box<dyn Error>> {
    let surface: Handle<dyn GeomSurface> = Handle::new(spline_surface(spline)?).cast();
    let maker = BRepBuilderAPIMakeFace::from_surface(&surface, precision::confusion())?;
    Ok(maker.face()?.into_shape())
}

fn spline_surface(spline: &SplineSurface) -> Result<GeomBSplineSurface, Box<dyn Error>> {
    let (poles, weights) = pole_grid(spline)?;
    let ((u_values, u_mults), (v_values, v_mults)) =
        (knot_lists(spline.u_knots), knot_lists(spline.v_knots));
    let (uk, um) = (
        Array1::from_shared(&u_values, 1),
        Array1::from_shared(&u_mults, 1),
    );
    let (vk, vm) = (
        Array1::from_shared(&v_values, 1),
        Array1::from_shared(&v_mults, 1),
    );
    let u = (&uk, &um);
    Ok(GeomBSplineSurface::with_weights(
        &poles,
        &weights,
        u,
        (&vk, &vm),
        spline.degrees,
        (false, false),
    )?)
}

/// Poles and weights indexed (u, v) from 1, read with U fastest.
type PoleGrid = (Array2<'static, Point3>, Array2<'static, f64>);

fn pole_grid(spline: &SplineSurface) -> Result<PoleGrid, Box<dyn Error>> {
    let rows = i32::try_from(spline.poles.len()).expect("a small pole count");
    let columns = pole_count(spline.v_knots, spline.degrees.1);
    let u_count = rows / columns;
    let mut poles = Array2::with_bounds(1, u_count, 1, columns)?;
    let mut weights = Array2::with_bounds(1, u_count, 1, columns)?;
    for (index, [x, y, z, w]) in (0..).zip(spline.poles.iter().copied()) {
        let (u, v) = (index % u_count + 1, index / u_count + 1);
        poles.set_value(u, v, Point3::new(x, y, z))?;
        weights.set_value(u, v, w)?;
    }
    Ok((poles, weights))
}

fn pole_count(knots: &[(f64, i32)], degree: i32) -> i32 {
    knots.iter().map(|&(_, m)| m).sum::<i32>() - degree - 1
}

/// Knot values and multiplicities as two lists.
fn knot_lists(knots: &[(f64, i32)]) -> (Vec<f64>, Vec<i32>) {
    knots.iter().copied().unzip()
}
