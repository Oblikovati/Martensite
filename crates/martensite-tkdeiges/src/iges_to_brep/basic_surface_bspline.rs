//! Rational B-spline surfaces (type 128).
use super::IgesToBRepBasicSurface;
use super::knot_gathering::{
    GatheredKnots, KnotCompare, correct_multiplicities, gather_knots, without_ranks,
};
use crate::iges_data::{IgesDataToolLocation, iges_live};
use crate::iges_geom::IgesGeomBSplineSurface;
use martensite_tkernel::ncollection::{Array1, Array2, array2_failure};
use martensite_tkernel::precision;
use martensite_tkernel::standard::real::epsilon;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::geom::{GeomBSplineSurface, GeomSurface};
use martensite_tkmath::gp::{Point3, Transform3};
use martensite_tkshhealing::shape_algo::ShapeAlgo;
use martensite_tkshhealing::shape_construct::ShapeConstructCurve;

/// The knots of one direction, with its name for the messages.
struct DirectionKnots {
    gathered: GatheredKnots,
    degree: i32,
}

/// A grid of rows of values as an array from (1, 1).
fn grid_array<T: Copy + Default + Send + Sync + 'static>(
    rows: &[Vec<T>],
) -> Result<Array2<'static, T>, Failure> {
    let nb_columns = rows.first().map_or(0, Vec::len) as i32;
    let mut array =
        Array2::with_bounds(1, rows.len() as i32, 1, nb_columns).map_err(array2_failure)?;
    for (row, values) in (1..).zip(rows) {
        for (column, value) in (1..).zip(values) {
            array
                .set_value(row, column, *value)
                .map_err(array2_failure)?;
        }
    }
    Ok(array)
}

/// Rows and columns without the removed ranks.
fn reduced_grid<T: Copy>(rows: &[Vec<T>], in_u: &[i32], in_v: &[i32]) -> Vec<Vec<T>> {
    without_ranks(rows, in_u)
        .iter()
        .map(|row| without_ranks(row, in_v))
        .collect()
}

fn fixed_knots(knots: &[f64]) -> Result<Vec<f64>, Failure> {
    let mut fixed = super::basic_curve_splines::owned_array(knots)?;
    ShapeConstructCurve::fix_knots(&mut fixed)?;
    Ok(fixed.as_slice().to_vec())
}

impl IgesToBRepBasicSurface {
    fn surface_poles(&self, live: &IgesGeomBSplineSurface) -> Result<Vec<Vec<Point3>>, Failure> {
        let positioned = !self.get_mode_transfer() && live.has_transf();
        let read_row = |row: i32| {
            (0..=live.upper_index_v())
                .map(|column| {
                    if positioned {
                        live.transformed_pole(row, column)
                    } else {
                        live.pole(row, column)
                    }
                })
                .collect::<Result<Vec<Point3>, Failure>>()
        };
        (0..=live.upper_index_u()).map(read_row).collect()
    }
    fn direction_knots(
        &self,
        start: &Handle<IgesGeomBSplineSurface>,
        read: (Vec<f64>, Vec<i32>),
        degree: i32,
        name: &[u8],
    ) -> Result<DirectionKnots, Failure> {
        let gathered = correct_multiplicities(read, degree, || {
            self.warn_with_args(start, "IGES_1200", &[name, name, name])
        })?;
        Ok(DirectionKnots { gathered, degree })
    }
    fn warn_on_sum(
        &self,
        start: &Handle<IgesGeomBSplineSurface>,
        direction: &DirectionKnots,
        nb_poles: i32,
        name: &[u8],
    ) -> Result<(), Failure> {
        let kept = nb_poles - direction.gathered.removed_poles.len() as i32;
        if direction.gathered.sum_of_multiplicities != kept + direction.degree + 1 {
            self.warn_with_args(start, "IGES_1210", &[name, name])?;
        }
        Ok(())
    }
    /// The poles moved by the position again, as they always are out of topology mode.
    fn repositioned(
        &self,
        start: &Handle<IgesGeomBSplineSurface>,
        live: &IgesGeomBSplineSurface,
        poles: &mut [Vec<Point3>],
    ) -> Result<(), Failure> {
        if self.get_mode_transfer() || !live.has_transf() {
            return Ok(());
        }
        let mut similarity = Transform3::default();
        let location = live.compound_location()?;
        if !IgesDataToolLocation::convert_location(
            self.get_epsilon(),
            &location,
            &mut similarity,
            1.0,
        )? {
            self.warn_key(start, "IGES_1035");
            return Ok(());
        }
        for pole in poles.iter_mut().flatten() {
            *pole = similarity.apply(*pole);
        }
        Ok(())
    }
    /// The weights of the poles, `None` after a fail for a weight that is not positive.
    fn surface_weights(
        &self,
        start: &Handle<IgesGeomBSplineSurface>,
        live: &IgesGeomBSplineSurface,
    ) -> Result<Option<Vec<Vec<f64>>>, Failure> {
        let reference = live.weight(0, 0)?;
        let mut constant = true;
        let mut rows = Vec::new();
        for row in 0..=live.upper_index_u() {
            let mut values = Vec::new();
            for column in 0..=live.upper_index_v() {
                let weight = live.weight(row, column)?;
                constant = constant && (weight - reference).abs() <= epsilon(reference);
                // A weight that is not positive is refused.
                if weight < precision::p_confusion() {
                    self.fail_key(start, "IGES_1215");
                    return Ok(None);
                }
                values.push(weight);
            }
            rows.push(values);
        }
        if constant {
            self.warn_with_args(start, "IGES_1220", &[b"surface"])?;
        }
        Ok(Some(rows))
    }
    /// One warning per weight that is 1000 past the extremes so far, one fail for knots out of order.
    fn knots_and_weights_check(
        &self,
        start: &Handle<IgesGeomBSplineSurface>,
        weights: Option<&[Vec<f64>]>,
        u_knots: &[f64],
        v_knots: &[f64],
    ) -> bool {
        if let Some(rows) = weights {
            let mut extremes = (rows[0][0], rows[0][0]);
            for &weight in rows.iter().flatten() {
                extremes = (extremes.0.min(weight), extremes.1.max(weight));
                if extremes.1 - extremes.0 > 1000.0 {
                    self.warn_key(start, "IGES_1374");
                }
            }
        }
        let descending = |knots: &[f64]| knots.windows(2).any(|pair| pair[1] < pair[0]);
        if descending(u_knots) || descending(v_knots) {
            self.fail_key(start, "IGES_1373");
            return false;
        }
        true
    }
    /// The B-spline surface of the entity, its continuity raised, forced periodic in a closed
    /// direction; `tool.transfer_bspline_surface(&surface)?`.
    ///
    /// # Errors
    /// A failure of the entity or of the construction of the surface.
    pub fn transfer_bspline_surface(
        &self,
        start: &Handle<IgesGeomBSplineSurface>,
    ) -> Result<Handle<GeomBSplineSurface>, Failure> {
        let Some(live) = start.get() else {
            self.fail_key(start, "IGES_1005");
            return Ok(Handle::default());
        };
        let (degree_u, degree_v) = (live.degree_u(), live.degree_v());
        let max_degree = GeomBSplineSurface::max_degree();
        if degree_u <= 0 || degree_u > max_degree || degree_v <= 0 || degree_v > max_degree {
            self.fail_key(start, "IGES_1310");
            return Ok(Handle::default());
        }
        let (nb_u, nb_v) = (live.nb_poles_u(), live.nb_poles_v());
        if nb_u < 2 || nb_v < 2 {
            self.fail_key(start, "IGES_1195");
            return Ok(Handle::default());
        }
        let read_poles = self.surface_poles(live)?;
        let read_u = gather_knots(
            live.nb_knots_u()?,
            degree_u,
            |rank| live.knot_u(rank),
            KnotCompare::AtPrevious,
        )?;
        let u = self.direction_knots(start, read_u, degree_u, b"U")?;
        self.warn_on_sum(start, &u, nb_u, b"U")?;
        let read_v = gather_knots(
            live.nb_knots_v()?,
            degree_v,
            |rank| live.knot_v(rank),
            KnotCompare::AtPrevious,
        )?;
        let v = self.direction_knots(start, read_v, degree_v, b"V")?;
        self.warn_on_sum(start, &v, nb_v, b"V")?;
        let (removed_u, removed_v) = (&u.gathered.removed_poles, &v.gathered.removed_poles);
        let mut poles = reduced_grid(&read_poles, removed_u, removed_v);
        self.repositioned(start, live, &mut poles)?;
        let surface = self.built_surface(start, live, &poles, &u, &v)?;
        let Some(surface) = surface else {
            return Ok(Handle::default());
        };
        self.raise_continuity(&surface, degree_u, degree_v)?;
        self.forced_periodic(start, live, surface)
    }
    fn built_surface(
        &self,
        start: &Handle<IgesGeomBSplineSurface>,
        live: &IgesGeomBSplineSurface,
        poles: &[Vec<Point3>],
        u: &DirectionKnots,
        v: &DirectionKnots,
    ) -> Result<Option<GeomBSplineSurface>, Failure> {
        let (u_knots, v_knots) = (&u.gathered.knots, &v.gathered.knots);
        let degrees = (u.degree, v.degree);
        let pole_array = grid_array(poles)?;
        let u_mults = Array1::from_shared(&u.gathered.multiplicities, 1);
        let v_mults = Array1::from_shared(&v.gathered.multiplicities, 1);
        if live.is_polynomial(false)? {
            if !self.knots_and_weights_check(start, None, u_knots, v_knots) {
                return Ok(None);
            }
            let (fixed_u, fixed_v) = (fixed_knots(u_knots)?, fixed_knots(v_knots)?);
            return GeomBSplineSurface::new(
                &pole_array,
                (&Array1::from_shared(&fixed_u, 1), &u_mults),
                (&Array1::from_shared(&fixed_v, 1), &v_mults),
                degrees,
                (false, false),
            )
            .map(Some);
        }
        let Some(read) = self.surface_weights(start, live)? else {
            return Ok(None);
        };
        let weights = reduced_grid(&read, &u.gathered.removed_poles, &v.gathered.removed_poles);
        if !self.knots_and_weights_check(start, Some(&weights), u_knots, v_knots) {
            return Ok(None);
        }
        let (fixed_u, fixed_v) = (fixed_knots(u_knots)?, fixed_knots(v_knots)?);
        GeomBSplineSurface::with_weights(
            &pole_array,
            &grid_array(&weights)?,
            (&Array1::from_shared(&fixed_u, 1), &u_mults),
            (&Array1::from_shared(&fixed_v, 1), &v_mults),
            degrees,
            (false, false),
        )
        .map(Some)
    }
    /// Removes knots from the last interior one down, for a continuity of 2 or more.
    fn raise_continuity(
        &self,
        surface: &GeomBSplineSurface,
        degree_u: i32,
        degree_v: i32,
    ) -> Result<(), Failure> {
        if self.get_continuity() < 2 {
            return Ok(());
        }
        let tolerance = self.get_eps_geom();
        for rank in ((surface.first_u_knot_index()? + 1)..surface.last_u_knot_index()?).rev() {
            let _ = surface.remove_u_knot(rank, degree_u - 2, tolerance)?;
            let _ = surface.remove_u_knot(rank, degree_u - 1, tolerance)?;
        }
        for rank in ((surface.first_v_knot_index()? + 1)..surface.last_v_knot_index()?).rev() {
            let _ = surface.remove_v_knot(rank, degree_v - 2, tolerance)?;
            let _ = surface.remove_v_knot(rank, degree_v - 1, tolerance)?;
        }
        Ok(())
    }
    /// Closed surfaces are forced to be periodic, or wires
    /// that lie over the seam are not correct.
    fn forced_periodic(
        &self,
        start: &Handle<IgesGeomBSplineSurface>,
        live: &IgesGeomBSplineSurface,
        surface: GeomBSplineSurface,
    ) -> Result<Handle<GeomBSplineSurface>, Failure> {
        let periodic_u = live.is_closed_u() && (live.is_periodic_u() || surface.is_u_closed()?);
        let periodic_v = live.is_closed_v() && (live.is_periodic_v() || surface.is_v_closed()?);
        let surface = Handle::new(surface);
        if !periodic_u && !periodic_v {
            return Ok(surface);
        }
        let container = ShapeAlgo::algo_container();
        let periodic = iges_live(&container, "IgesToBRepBasicSurface")?
            .convert_to_periodic(&surface.cast())?;
        let periodic = periodic.cast::<GeomBSplineSurface>();
        if periodic.is_null() {
            return Ok(surface);
        }
        // Told through the messages.
        self.warn_key(start, "IGES_1221");
        Ok(periodic)
    }
}
