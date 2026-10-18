//! MakeEllipse and MakeHyperbola: Ellipse and Hyperbola.
use crate::gce::{self, ErrorType};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::geom::{GeomEllipse, GeomHyperbola};
use martensite_tkmath::gp::{Axis2, Ellipse3, Hyperbola3, Point3};

/// MakeEllipse: builds a Ellipse from a gp ellipse, a frame and radii, or
/// the ends of its axes and its center.
///
/// `let ellipse = MakeEllipse::new(Axis2::default(), 2.0, 1.0)?.value()?;`
#[derive(Clone, Default)]
pub struct MakeEllipse {
    the_result: Handle<GeomEllipse>,
    the_error: ErrorType,
}
gc_result!(MakeEllipse, GeomEllipse, "GC_MakeEllipse");
gc_from_gce!(MakeEllipse, gce::MakeElips, GeomEllipse::from_elips);

impl MakeEllipse {
    /// MakeEllipse(E): the Geom ellipse of `ellipse`. `MakeEllipse::from_elips(e)`.
    pub fn from_elips(ellipse: Ellipse3) -> Self {
        Self::built(GeomEllipse::from_elips(ellipse))
    }

    /// MakeEllipse(A2, MajorRadius, MinorRadius): NegativeRadius when
    /// `minor < 0`, then InvertAxis (not InvertRadius, as MakeElips
    /// reports) when `major < minor`. `MakeEllipse::new(frame, 2.0, 1.0)?`.
    pub fn new(frame: Axis2, major_radius: f64, minor_radius: f64) -> Result<Self, Failure> {
        if minor_radius < 0.0 {
            return Ok(Self::failed(ErrorType::NegativeRadius));
        }
        if major_radius < minor_radius {
            return Ok(Self::failed(ErrorType::InvertAxis));
        }
        let ellipse = Ellipse3::new(frame, major_radius, minor_radius)?;
        Ok(Self::built(GeomEllipse::from_elips(ellipse)))
    }

    /// MakeEllipse(S1, S2, Center): MakeElips(S1, S2, Center).
    /// `MakeEllipse::from_points(s1, s2, center)?`.
    pub fn from_points(s1: Point3, s2: Point3, center: Point3) -> Result<Self, Failure> {
        Self::from_gce(gce::MakeElips::from_points(s1, s2, center)?)
    }
}

/// MakeHyperbola: builds a Hyperbola from a gp hyperbola, a frame and
/// radii, or the ends of its axes and its center.
///
/// `let hyperbola = MakeHyperbola::new(Axis2::default(), 2.0, 1.0)?.value()?;`
#[derive(Clone, Default)]
pub struct MakeHyperbola {
    the_result: Handle<GeomHyperbola>,
    the_error: ErrorType,
}
gc_result!(MakeHyperbola, GeomHyperbola, "GC_MakeHyperbola");
gc_from_gce!(MakeHyperbola, gce::MakeHypr, GeomHyperbola::from_hypr);

impl MakeHyperbola {
    /// MakeHyperbola(H): the Geom hyperbola of `hyperbola`.
    /// `MakeHyperbola::from_hypr(h)`.
    pub fn from_hypr(hyperbola: Hyperbola3) -> Self {
        Self::built(GeomHyperbola::from_hypr(hyperbola))
    }

    /// MakeHyperbola(A2, MajorRadius, MinorRadius): NegativeRadius when a
    /// radius is negative. `MakeHyperbola::new(frame, 2.0, 1.0)?`.
    pub fn new(frame: Axis2, major_radius: f64, minor_radius: f64) -> Result<Self, Failure> {
        if major_radius < 0.0 || minor_radius < 0.0 {
            return Ok(Self::failed(ErrorType::NegativeRadius));
        }
        let hyperbola = Hyperbola3::new(frame, major_radius, minor_radius)?;
        Ok(Self::built(GeomHyperbola::from_hypr(hyperbola)))
    }

    /// MakeHyperbola(S1, S2, Center): MakeHypr(S1, S2, Center).
    /// `MakeHyperbola::from_points(s1, s2, center)?`.
    pub fn from_points(s1: Point3, s2: Point3, center: Point3) -> Result<Self, Failure> {
        Self::from_gce(gce::MakeHypr::from_points(s1, s2, center)?)
    }
}
