//! MakeEllipse2d, MakeHyperbola2d and MakeParabola2d: the Geom2d conics.
use crate::gce::{self, ErrorType};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg2d::geom2d::{Geom2dEllipse, Geom2dHyperbola, Geom2dParabola};
use martensite_tkmath::gp::{Axis2d, Axis22d, Ellipse2, Hyperbola2, Parabola2, Point2};

/// MakeEllipse2d: builds a Ellipse from a gp ellipse, a frame, a major
/// axis, or the ends of its axes and its center.
///
/// `let ellipse = MakeEllipse2d::from_frame(Axis22d::default(), 2.0, 1.0)?.value()?;`
#[derive(Clone, Default)]
pub struct MakeEllipse2d {
    the_result: Handle<Geom2dEllipse>,
    the_error: ErrorType,
}
gc_result!(MakeEllipse2d, Geom2dEllipse, "GC_MakeEllipse2d");
gc_from_gce!(MakeEllipse2d, gce::MakeElips2d, Geom2dEllipse::from_elips2d);

impl MakeEllipse2d {
    /// MakeEllipse2d(E): the Geom2d ellipse of `ellipse`.
    /// `MakeEllipse2d::from_elips(e)`.
    pub fn from_elips(ellipse: Ellipse2) -> Self {
        Self::built(Geom2dEllipse::from_elips2d(ellipse))
    }

    /// MakeEllipse2d(Axis, MajorRadius, MinorRadius): MakeElips2d with a
    /// frame. `MakeEllipse2d::from_frame(frame, 2.0, 1.0)?`.
    pub fn from_frame(
        frame: Axis22d,
        major_radius: f64,
        minor_radius: f64,
    ) -> Result<Self, Failure> {
        Self::from_gce(gce::MakeElips2d::from_frame(
            frame,
            major_radius,
            minor_radius,
        )?)
    }

    /// MakeEllipse2d(MajorAxis, MajorRadius, MinorRadius, Sense): MakeElips2d
    /// with a major axis. `MakeEllipse2d::new(axis, 2.0, 1.0, true)?`.
    pub fn new(
        major_axis: Axis2d,
        major_radius: f64,
        minor_radius: f64,
        sense: bool,
    ) -> Result<Self, Failure> {
        Self::from_gce(gce::MakeElips2d::new(
            major_axis,
            major_radius,
            minor_radius,
            sense,
        )?)
    }

    /// MakeEllipse2d(S1, S2, Center): MakeElips2d(S1, S2, Center).
    /// `MakeEllipse2d::from_points(s1, s2, center)?`.
    pub fn from_points(s1: Point2, s2: Point2, center: Point2) -> Result<Self, Failure> {
        Self::from_gce(gce::MakeElips2d::from_points(s1, s2, center)?)
    }
}

/// MakeHyperbola2d: builds a Hyperbola from a gp hyperbola, a major
/// axis, a frame, or the ends of its axes and its center.
///
/// `let hyperbola = MakeHyperbola2d::from_frame(Axis22d::default(), 2.0, 1.0)?.value()?;`
#[derive(Clone, Default)]
pub struct MakeHyperbola2d {
    the_result: Handle<Geom2dHyperbola>,
    the_error: ErrorType,
}
gc_result!(MakeHyperbola2d, Geom2dHyperbola, "GC_MakeHyperbola2d");
gc_from_gce!(
    MakeHyperbola2d,
    gce::MakeHypr2d,
    Geom2dHyperbola::from_hypr2d
);

impl MakeHyperbola2d {
    /// MakeHyperbola2d(H): the Geom2d hyperbola of `hyperbola`.
    /// `MakeHyperbola2d::from_hypr(h)`.
    pub fn from_hypr(hyperbola: Hyperbola2) -> Self {
        Self::built(Geom2dHyperbola::from_hypr2d(hyperbola))
    }

    /// MakeHyperbola2d(MajorAxis, MajorRadius, MinorRadius, Sense):
    /// MakeHypr2d with a major axis. `MakeHyperbola2d::new(axis, 2.0, 1.0, true)?`.
    pub fn new(
        major_axis: Axis2d,
        major_radius: f64,
        minor_radius: f64,
        sense: bool,
    ) -> Result<Self, Failure> {
        Self::from_gce(gce::MakeHypr2d::new(
            major_axis,
            major_radius,
            minor_radius,
            sense,
        )?)
    }

    /// MakeHyperbola2d(Axis, MajorRadius, MinorRadius): MakeHypr2d with a
    /// frame. `MakeHyperbola2d::from_frame(frame, 2.0, 1.0)?`.
    pub fn from_frame(
        frame: Axis22d,
        major_radius: f64,
        minor_radius: f64,
    ) -> Result<Self, Failure> {
        Self::from_gce(gce::MakeHypr2d::from_frame(
            frame,
            major_radius,
            minor_radius,
        )?)
    }

    /// MakeHyperbola2d(S1, S2, Center): MakeHypr2d(S1, S2, Center).
    /// `MakeHyperbola2d::from_points(s1, s2, center)?`.
    pub fn from_points(s1: Point2, s2: Point2, center: Point2) -> Result<Self, Failure> {
        Self::from_gce(gce::MakeHypr2d::from_points(s1, s2, center)?)
    }
}

/// MakeParabola2d: builds a Parabola from a gp parabola, a mirror axis,
/// a frame, a directrix and focus, or its apex and a point of its axis.
///
/// `let parabola = MakeParabola2d::from_frame(Axis22d::default(), 1.0)?.value()?;`
#[derive(Clone, Default)]
pub struct MakeParabola2d {
    the_result: Handle<Geom2dParabola>,
    the_error: ErrorType,
}
gc_result!(MakeParabola2d, Geom2dParabola, "GC_MakeParabola2d");
gc_from_gce!(
    MakeParabola2d,
    gce::MakeParab2d,
    Geom2dParabola::from_parab2d
);

impl MakeParabola2d {
    /// MakeParabola2d(Prb): the Geom2d parabola of `parabola`.
    /// `MakeParabola2d::from_parab(p)`.
    pub fn from_parab(parabola: Parabola2) -> Self {
        Self::built(Geom2dParabola::from_parab2d(parabola))
    }

    /// MakeParabola2d(MirrorAxis, Focal, Sense): NullFocusLength when
    /// `focal < 0`. `MakeParabola2d::new(axis, 1.0, true)?`.
    pub fn new(mirror_axis: Axis2d, focal: f64, sense: bool) -> Result<Self, Failure> {
        if focal < 0.0 {
            return Ok(Self::failed(ErrorType::NullFocusLength));
        }
        Ok(Self::built(Geom2dParabola::from_axis(
            mirror_axis,
            focal,
            sense,
        )?))
    }

    /// MakeParabola2d(Axis, Focal): NullFocusLength when `focal < 0`.
    /// `MakeParabola2d::from_frame(frame, 1.0)?`.
    pub fn from_frame(frame: Axis22d, focal: f64) -> Result<Self, Failure> {
        if focal < 0.0 {
            return Ok(Self::failed(ErrorType::NullFocusLength));
        }
        Ok(Self::built(Geom2dParabola::new(frame, focal)?))
    }

    /// MakeParabola2d(D, F, Sense): the parabola `Parab2d(D, F, Sense)`,
    /// always done. `MakeParabola2d::from_directrix(axis, focus, true)?`.
    pub fn from_directrix(directrix: Axis2d, focus: Point2, sense: bool) -> Result<Self, Failure> {
        let parabola = Parabola2::from_directrix(directrix, focus, sense)?;
        Ok(Self::built(Geom2dParabola::from_parab2d(parabola)))
    }

    /// MakeParabola2d(S1, O): MakeParab2d(S1, O) with the default sense.
    /// `MakeParabola2d::from_points(s1, apex)?`.
    pub fn from_points(s1: Point2, apex: Point2) -> Result<Self, Failure> {
        Self::from_gce(gce::MakeParab2d::from_points(s1, apex, true)?)
    }
}
