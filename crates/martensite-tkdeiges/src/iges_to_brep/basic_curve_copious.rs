//! Copious data: the polyline of its points as a B-spline of degree 1, in space and in the plane.
use super::IgesToBRepBasicCurve;
use crate::iges_conv_geom;
use crate::iges_geom::IgesGeomCopiousData;
use martensite_tkernel::ncollection::Array1;
use martensite_tkernel::precision;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg2d::geom2d::Geom2dBSplineCurve;
use martensite_tkg3d::geom::GeomBSplineCurve;
use martensite_tkmath::gp::{self, Point2, Point3};

/// The knots at the accumulated lengths, and the multiplicities of a polyline.
fn polyline_knots(lengths: impl Iterator<Item = f64>) -> (Vec<f64>, Vec<i32>) {
    let mut knots = vec![0.0];
    let mut total = 0.0;
    for length in lengths {
        total += length;
        knots.push(total);
    }
    let mut multiplicities = vec![1; knots.len()];
    let last = multiplicities.len() - 1;
    multiplicities[0] = 2;
    multiplicities[last] = 2;
    (knots, multiplicities)
}

impl IgesToBRepBasicCurve {
    /// The points in their position; `None` for fewer than two, with a fail. A form with no vectors
    /// to skip is a warning.
    fn read_points(
        &self,
        start: &Handle<IgesGeomCopiousData>,
        live: &IgesGeomCopiousData,
    ) -> Result<Option<Vec<Point3>>, Failure> {
        let form = live.form_number();
        if form != 11 && form != 12 && form != 63 {
            self.warn_key(start, "IGES_1240");
        }
        let nb_points = live.nb_points()?;
        if nb_points < 2 {
            self.fail_key(start, "IGES_1195");
            return Ok(None);
        }
        let positioned = self.applies_position(live);
        (1..=nb_points)
            .map(|rank| {
                if positioned {
                    live.transformed_point(rank)
                } else {
                    live.point(rank)
                }
            })
            .collect::<Result<Vec<Point3>, Failure>>()
            .map(Some)
    }
    /// The polyline in space, with neighbours that coincide dropped; a fail when it degenerates;
    /// `tool.transfer_copious_data(&data)?`.
    ///
    /// # Errors
    /// A failure of the entity or of the construction of the curve.
    pub fn transfer_copious_data(
        &self,
        start: &Handle<IgesGeomCopiousData>,
    ) -> Result<Handle<GeomBSplineCurve>, Failure> {
        let Some(live) = start.get() else {
            self.null_entity(start);
            return Ok(Handle::default());
        };
        let Some(mut poles) = self.read_points(start, live)? else {
            return Ok(Handle::default());
        };
        // A filter stays: a file can give two equal points.
        poles.dedup_by(|next, kept| next.is_equal(kept, gp::resolution()));
        if poles.len() == 1 {
            self.fail_key(start, "IGES_1235");
            return Ok(Handle::default());
        }
        let lengths = poles.windows(2).map(|pair| pair[1].distance(&pair[0]));
        let (knots, multiplicities) = polyline_knots(lengths);
        let curve = GeomBSplineCurve::new(
            &Array1::from_shared(&poles, 1),
            &Array1::from_shared(&knots, 1),
            &Array1::from_shared(&multiplicities, 1),
            1,
            false,
        )?;
        let tolerance = (self.get_eps_geom() / 10.0).max(precision::confusion());
        iges_conv_geom::increase_curve_continuity(&curve, tolerance, self.get_continuity())?;
        Ok(Handle::new(curve))
    }
    /// The polyline in the plane; `tool.transfer_2d_copious_data(&data)?`.
    ///
    /// # Errors
    /// A failure of the entity or of the construction of the curve.
    pub fn transfer_2d_copious_data(
        &mut self,
        start: &Handle<IgesGeomCopiousData>,
    ) -> Result<Handle<Geom2dBSplineCurve>, Failure> {
        let Some(live) = start.get() else {
            self.null_entity(start);
            return Ok(Handle::default());
        };
        let Some(read) = self.read_points(start, live)? else {
            return Ok(Handle::default());
        };
        let mut poles: Vec<Point2> = read
            .iter()
            .map(|point| Point2::new(point.x(), point.y()))
            .collect();
        poles.dedup_by(|next, kept| next.is_equal(*kept, gp::resolution()));
        if poles.len() == 1 {
            self.fail_key(start, "IGES_1235");
            return Ok(Handle::default());
        }
        let lengths = poles.windows(2).map(|pair| pair[1].distance(pair[0]));
        let (knots, multiplicities) = polyline_knots(lengths);
        let curve = Geom2dBSplineCurve::new(
            &Array1::from_shared(&poles, 1),
            &Array1::from_shared(&knots, 1),
            &Array1::from_shared(&multiplicities, 1),
            1,
            false,
        )?;
        let tolerance = precision::confusion().max(self.get_eps_geom() * self.get_uv_resolution()?);
        iges_conv_geom::increase_curve2d_continuity(&curve, tolerance, self.get_continuity())?;
        Ok(Handle::new(curve))
    }
}
