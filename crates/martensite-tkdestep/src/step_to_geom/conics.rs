//! Conics, in 3D and 2D.
use super::StepToGeom;
use super::placements::{select_placement_2d, select_placement_3d};
use crate::step_data::StepDataFactors;
use crate::step_entity::step_live;
use crate::step_geom::{
    StepGeomCircle, StepGeomConic, StepGeomEllipse, StepGeomHyperbola, StepGeomParabola,
};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg2d::geom2d::{
    Geom2dCircle, Geom2dConic, Geom2dEllipse, Geom2dHyperbola, Geom2dParabola,
};
use martensite_tkg3d::geom::{GeomCircle, GeomConic, GeomEllipse, GeomHyperbola, GeomParabola};
use martensite_tkmath::gp::{Axis22d, Direction2};

impl StepToGeom {
    /// `MakeCircle`: null without a 3D placement.
    pub fn make_circle(
        sc: &Handle<StepGeomCircle>,
        factors: &StepDataFactors,
    ) -> Result<Handle<GeomCircle>, Failure> {
        let live = step_live(sc, "StepToGeom::MakeCircle")?;
        let placement = select_placement_3d(&live.position(), factors)?;
        let Some(placement) = placement.get() else {
            return Ok(Handle::default());
        };
        Ok(Handle::new(GeomCircle::new(
            placement.ax2()?,
            live.radius() * factors.length_factor(),
        )?))
    }
    /// `MakeCircle2d`: null without a 2D placement.
    pub fn make_circle_2d(
        sc: &Handle<StepGeomCircle>,
        factors: &StepDataFactors,
    ) -> Result<Handle<Geom2dCircle>, Failure> {
        let live = step_live(sc, "StepToGeom::MakeCircle2d")?;
        let placement = select_placement_2d(&live.position(), factors)?;
        let Some(placement) = placement.get() else {
            return Ok(Handle::default());
        };
        Ok(Handle::new(Geom2dCircle::from_axis(
            placement.ax2d(),
            live.radius(),
            true,
        )?))
    }
    /// `MakeEllipse`: axes given in the wrong order are swapped (:o9 abv 19 Feb 99).
    pub fn make_ellipse(
        sc: &Handle<StepGeomEllipse>,
        factors: &StepDataFactors,
    ) -> Result<Handle<GeomEllipse>, Failure> {
        let live = step_live(sc, "StepToGeom::MakeEllipse")?;
        let placement = select_placement_3d(&live.position(), factors)?;
        let Some(placement) = placement.get() else {
            return Ok(Handle::default());
        };
        let mut frame = placement.ax2()?;
        let major = live.semi_axis1() * factors.length_factor();
        let minor = live.semi_axis2() * factors.length_factor();
        if major - minor >= 0.0 {
            return Ok(Handle::new(GeomEllipse::new(frame, major, minor)?));
        }
        frame.set_x_direction(frame.x_direction().crossed(frame.direction())?)?;
        Ok(Handle::new(GeomEllipse::new(frame, minor, major)?))
    }
    /// `MakeEllipse2d`: :o9 abv 19 Feb 99: bm4_id_punch_b.stp #678: protection on the axes order.
    pub fn make_ellipse_2d(
        sc: &Handle<StepGeomEllipse>,
        factors: &StepDataFactors,
    ) -> Result<Handle<Geom2dEllipse>, Failure> {
        let live = step_live(sc, "StepToGeom::MakeEllipse2d")?;
        let placement = select_placement_2d(&live.position(), factors)?;
        let Some(placement) = placement.get() else {
            return Ok(Handle::default());
        };
        let mut frame = Axis22d::from_axis(placement.ax2d(), true)?;
        let (major, minor) = (live.semi_axis1(), live.semi_axis2());
        if major - minor >= 0.0 {
            return Ok(Handle::new(Geom2dEllipse::new(frame, major, minor)?));
        }
        let x_direction = frame.x_direction();
        frame.set_x_direction(Direction2::new(x_direction.x(), -x_direction.y())?)?;
        Ok(Handle::new(Geom2dEllipse::new(frame, minor, major)?))
    }
    /// `MakeHyperbola`: null without a 3D placement.
    pub fn make_hyperbola(
        sc: &Handle<StepGeomHyperbola>,
        factors: &StepDataFactors,
    ) -> Result<Handle<GeomHyperbola>, Failure> {
        let live = step_live(sc, "StepToGeom::MakeHyperbola")?;
        let placement = select_placement_3d(&live.position(), factors)?;
        let Some(placement) = placement.get() else {
            return Ok(Handle::default());
        };
        let lf = factors.length_factor();
        Ok(Handle::new(GeomHyperbola::new(
            placement.ax2()?,
            live.semi_axis() * lf,
            live.semi_imag_axis() * lf,
        )?))
    }
    /// `MakeHyperbola2d`: null without a 2D placement.
    pub fn make_hyperbola_2d(
        sc: &Handle<StepGeomHyperbola>,
        factors: &StepDataFactors,
    ) -> Result<Handle<Geom2dHyperbola>, Failure> {
        let live = step_live(sc, "StepToGeom::MakeHyperbola2d")?;
        let placement = select_placement_2d(&live.position(), factors)?;
        let Some(placement) = placement.get() else {
            return Ok(Handle::default());
        };
        let frame = Axis22d::from_axis(placement.ax2d(), true)?;
        Ok(Handle::new(Geom2dHyperbola::new(
            frame,
            live.semi_axis(),
            live.semi_imag_axis(),
        )?))
    }
    /// `make_parabola`: null without a 3D placement.
    pub fn make_parabola(
        sc: &Handle<StepGeomParabola>,
        factors: &StepDataFactors,
    ) -> Result<Handle<GeomParabola>, Failure> {
        let live = step_live(sc, "StepToGeom::MakeParabola")?;
        let placement = select_placement_3d(&live.position(), factors)?;
        let Some(placement) = placement.get() else {
            return Ok(Handle::default());
        };
        Ok(Handle::new(GeomParabola::new(
            placement.ax2()?,
            live.focal_dist() * factors.length_factor(),
        )?))
    }
    /// `MakeParabola2d`: null without a 2D placement.
    pub fn make_parabola_2d(
        sc: &Handle<StepGeomParabola>,
        factors: &StepDataFactors,
    ) -> Result<Handle<Geom2dParabola>, Failure> {
        let live = step_live(sc, "StepToGeom::MakeParabola2d")?;
        let placement = select_placement_2d(&live.position(), factors)?;
        let Some(placement) = placement.get() else {
            return Ok(Handle::default());
        };
        Ok(Handle::new(Geom2dParabola::new(
            Axis22d::from_axis(placement.ax2d(), true)?,
            live.focal_dist(),
        )?))
    }
    /// `make_conic`: null for another conic.
    pub fn make_conic(
        sc: &Handle<StepGeomConic>,
        factors: &StepDataFactors,
    ) -> Result<Handle<dyn GeomConic>, Failure> {
        let circle = sc.cast::<StepGeomCircle>();
        if circle.is_some() {
            return Ok(Self::make_circle(&circle, factors)?.cast());
        }
        let ellipse = sc.cast::<StepGeomEllipse>();
        if ellipse.is_some() {
            return Ok(Self::make_ellipse(&ellipse, factors)?.cast());
        }
        let hyperbola = sc.cast::<StepGeomHyperbola>();
        if hyperbola.is_some() {
            return Ok(Self::make_hyperbola(&hyperbola, factors)?.cast());
        }
        let parabola = sc.cast::<StepGeomParabola>();
        if parabola.is_some() {
            return Ok(Self::make_parabola(&parabola, factors)?.cast());
        }
        Ok(Handle::default())
    }
    /// `MakeConic2d`: null for another conic.
    pub fn make_conic_2d(
        sc: &Handle<StepGeomConic>,
        factors: &StepDataFactors,
    ) -> Result<Handle<dyn Geom2dConic>, Failure> {
        let circle = sc.cast::<StepGeomCircle>();
        if circle.is_some() {
            return Ok(Self::make_circle_2d(&circle, factors)?.cast());
        }
        let ellipse = sc.cast::<StepGeomEllipse>();
        if ellipse.is_some() {
            return Ok(Self::make_ellipse_2d(&ellipse, factors)?.cast());
        }
        let hyperbola = sc.cast::<StepGeomHyperbola>();
        if hyperbola.is_some() {
            return Ok(Self::make_hyperbola_2d(&hyperbola, factors)?.cast());
        }
        let parabola = sc.cast::<StepGeomParabola>();
        if parabola.is_some() {
            return Ok(Self::make_parabola_2d(&parabola, factors)?.cast());
        }
        Ok(Handle::default())
    }
}
