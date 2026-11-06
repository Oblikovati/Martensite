//! Lines, conics and the curve dispatch.
use super::bspline_curves::GeomToStepMakeBoundedCurve;
use super::placements::{
    GeomToStepMakeAxis2Placement2d, GeomToStepMakeAxis2Placement3d, GeomToStepMakeCartesianPoint,
    GeomToStepMakeVector, live_geometry,
};
use super::{empty_step_name, geom_to_step_class};
use crate::step_data::{StepDataFactors, StepDataSelectType};
use crate::step_geom::{
    StepGeomAxis2Placement, StepGeomCircle, StepGeomConic, StepGeomCurve, StepGeomEllipse,
    StepGeomHyperbola, StepGeomLine, StepGeomParabola,
};
use martensite_tkernel::precision;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg2d::geom2d::{
    Geom2dBSplineCurve, Geom2dBoundedCurve, Geom2dCircle, Geom2dConic, Geom2dCurve, Geom2dEllipse,
    Geom2dHyperbola, Geom2dLine, Geom2dParabola, Geom2dTrimmedCurve,
};
use martensite_tkg3d::geom::{
    GeomBSplineCurve, GeomBezierCurve, GeomBoundedCurve, GeomCircle, GeomConic, GeomCurve,
    GeomEllipse, GeomHyperbola, GeomLine, GeomParabola, GeomTrimmedCurve,
};
use martensite_tkgeombase::geom2d_convert;
use martensite_tkmath::convert::ParameterisationType;
use martensite_tkmath::gp::{
    Axis2, Axis22d, Circle2, Circle3, Ellipse2, Ellipse3, Line2, Line3, Point2, Point3, Vector2,
    Vector3,
};

/// The select of a conic from a 3D placement.
fn select_axis3d(
    axis: &Axis2,
    factors: &StepDataFactors,
) -> Result<StepGeomAxis2Placement, Failure> {
    let placement = GeomToStepMakeAxis2Placement3d::from_ax2(axis, factors)?.value()?;
    let mut select = StepGeomAxis2Placement::default();
    select.set_value(&placement.cast())?;
    Ok(select)
}

/// The select of a conic from a 2D placement.
fn select_axis2d(
    axis: &Axis22d,
    factors: &StepDataFactors,
) -> Result<StepGeomAxis2Placement, Failure> {
    let placement = GeomToStepMakeAxis2Placement2d::from_ax22d(axis, factors)?.value()?;
    let mut select = StepGeomAxis2Placement::default();
    select.set_value(&placement.cast())?;
    Ok(select)
}

geom_to_step_class!(
    /// `GeomToStepMakeLine`: a line from a location and a unit vector.
    GeomToStepMakeLine,
    "GeomToStepMakeLine",
    StepGeomLine
);

impl GeomToStepMakeLine {
    fn with_parts(
        point: &GeomToStepMakeCartesianPoint,
        vector: &GeomToStepMakeVector,
    ) -> Result<Self, Failure> {
        let line = StepGeomLine::new();
        line.init(&empty_step_name(), &point.value()?, &vector.value()?);
        Ok(Self::done_with(Handle::new(line)))
    }
    /// `GeomToStepMakeLine(gp_Lin, factors)`.
    pub fn from_lin(line: &Line3, factors: &StepDataFactors) -> Result<Self, Failure> {
        let location: Point3 = line.location();
        let point = GeomToStepMakeCartesianPoint::from_pnt(&location, factors.length_factor());
        let vector = GeomToStepMakeVector::from_vec(&Vector3::from(line.direction()), factors)?;
        Self::with_parts(&point, &vector)
    }
    /// `GeomToStepMakeLine(gp_Lin2d, factors)`.
    pub fn from_lin2d(line: &Line2, factors: &StepDataFactors) -> Result<Self, Failure> {
        let location: Point2 = line.location();
        let point = GeomToStepMakeCartesianPoint::from_pnt2d(&location, factors.length_factor());
        let vector = GeomToStepMakeVector::from_vec2d(&Vector2::from(line.direction()), factors)?;
        Self::with_parts(&point, &vector)
    }
    /// `GeomToStepMakeLine(GeomLine, factors)`.
    pub fn from_geom(line: &Handle<GeomLine>, factors: &StepDataFactors) -> Result<Self, Failure> {
        Self::from_lin(&live_geometry(line, "GeomToStepMakeLine")?.lin(), factors)
    }
    /// `GeomToStepMakeLine(Geom2dLine, factors)`.
    pub fn from_geom2d(
        line: &Handle<Geom2dLine>,
        factors: &StepDataFactors,
    ) -> Result<Self, Failure> {
        Self::from_lin2d(&live_geometry(line, "GeomToStepMakeLine")?.lin2d(), factors)
    }
}

geom_to_step_class!(
    /// `GeomToStepMakeCircle`: a circle.
    GeomToStepMakeCircle,
    "GeomToStepMakeCircle",
    StepGeomCircle
);

impl GeomToStepMakeCircle {
    fn with_parts(select: &StepGeomAxis2Placement, radius: f64) -> Self {
        let circle = StepGeomCircle::new();
        circle.init(&empty_step_name(), select, radius);
        Self::done_with(Handle::new(circle))
    }
    /// `GeomToStepMakeCircle(gp_Circ, factors)`.
    pub fn from_circ(circle: &Circle3, factors: &StepDataFactors) -> Result<Self, Failure> {
        let select = select_axis3d(&circle.position(), factors)?;
        Ok(Self::with_parts(
            &select,
            circle.radius() / factors.length_factor(),
        ))
    }
    /// `GeomToStepMakeCircle(GeomCircle, factors)`.
    pub fn from_geom(
        circle: &Handle<GeomCircle>,
        factors: &StepDataFactors,
    ) -> Result<Self, Failure> {
        Self::from_circ(
            &live_geometry(circle, "GeomToStepMakeCircle")?.circ()?,
            factors,
        )
    }
    /// `GeomToStepMakeCircle(Geom2dCircle, factors)`: the 2D radius is not scaled.
    pub fn from_geom2d(
        circle: &Handle<Geom2dCircle>,
        factors: &StepDataFactors,
    ) -> Result<Self, Failure> {
        let gp: Circle2 = live_geometry(circle, "GeomToStepMakeCircle")?.circ2d()?;
        Ok(Self::with_parts(
            &select_axis2d(&gp.position(), factors)?,
            gp.radius(),
        ))
    }
}

geom_to_step_class!(
    /// `GeomToStepMakeEllipse`: an ellipse.
    GeomToStepMakeEllipse,
    "GeomToStepMakeEllipse",
    StepGeomEllipse
);

impl GeomToStepMakeEllipse {
    fn with_parts(select: &StepGeomAxis2Placement, major: f64, minor: f64) -> Self {
        let ellipse = StepGeomEllipse::new();
        ellipse.init(&empty_step_name(), select, major, minor);
        Self::done_with(Handle::new(ellipse))
    }
    /// `GeomToStepMakeEllipse(gp_Elips, factors)`.
    pub fn from_elips(ellipse: &Ellipse3, factors: &StepDataFactors) -> Result<Self, Failure> {
        let select = select_axis3d(&ellipse.position(), factors)?;
        let factor = factors.length_factor();
        Ok(Self::with_parts(
            &select,
            ellipse.major_radius() / factor,
            ellipse.minor_radius() / factor,
        ))
    }
    /// `GeomToStepMakeEllipse(GeomEllipse, factors)`.
    pub fn from_geom(
        ellipse: &Handle<GeomEllipse>,
        factors: &StepDataFactors,
    ) -> Result<Self, Failure> {
        Self::from_elips(
            &live_geometry(ellipse, "GeomToStepMakeEllipse")?.elips()?,
            factors,
        )
    }
    /// `GeomToStepMakeEllipse(Geom2dEllipse, factors)`: the 2D radii are not scaled.
    pub fn from_geom2d(
        ellipse: &Handle<Geom2dEllipse>,
        factors: &StepDataFactors,
    ) -> Result<Self, Failure> {
        let gp: Ellipse2 = live_geometry(ellipse, "GeomToStepMakeEllipse")?.elips2d()?;
        let select = select_axis2d(&gp.axis(), factors)?;
        Ok(Self::with_parts(
            &select,
            gp.major_radius(),
            gp.minor_radius(),
        ))
    }
}

geom_to_step_class!(
    /// `GeomToStepMakeHyperbola`: a hyperbola.
    GeomToStepMakeHyperbola,
    "GeomToStepMakeHyperbola",
    StepGeomHyperbola
);

impl GeomToStepMakeHyperbola {
    fn with_parts(select: &StepGeomAxis2Placement, major: f64, minor: f64) -> Self {
        let hyperbola = StepGeomHyperbola::new();
        hyperbola.init(&empty_step_name(), select, major, minor);
        Self::done_with(Handle::new(hyperbola))
    }
    /// `GeomToStepMakeHyperbola(Geom2dHyperbola, factors)`: the 2D radii are not scaled.
    pub fn from_geom2d(
        hyperbola: &Handle<Geom2dHyperbola>,
        factors: &StepDataFactors,
    ) -> Result<Self, Failure> {
        let gp = live_geometry(hyperbola, "GeomToStepMakeHyperbola")?.hypr2d()?;
        let select = select_axis2d(&gp.axis(), factors)?;
        Ok(Self::with_parts(
            &select,
            gp.major_radius(),
            gp.minor_radius(),
        ))
    }
    /// `GeomToStepMakeHyperbola(GeomHyperbola, factors)`.
    pub fn from_geom(
        hyperbola: &Handle<GeomHyperbola>,
        factors: &StepDataFactors,
    ) -> Result<Self, Failure> {
        let gp = live_geometry(hyperbola, "GeomToStepMakeHyperbola")?.hypr()?;
        let select = select_axis3d(&gp.position(), factors)?;
        let factor = factors.length_factor();
        Ok(Self::with_parts(
            &select,
            gp.major_radius() / factor,
            gp.minor_radius() / factor,
        ))
    }
}

geom_to_step_class!(
    /// `GeomToStepMakeParabola`: a parabola.
    GeomToStepMakeParabola,
    "GeomToStepMakeParabola",
    StepGeomParabola
);

impl GeomToStepMakeParabola {
    fn with_parts(select: &StepGeomAxis2Placement, focal: f64) -> Self {
        let parabola = StepGeomParabola::new();
        parabola.init(&empty_step_name(), select, focal);
        Self::done_with(Handle::new(parabola))
    }
    /// `GeomToStepMakeParabola(Geom2dParabola, factors)`: the 2D focal is not scaled.
    pub fn from_geom2d(
        parabola: &Handle<Geom2dParabola>,
        factors: &StepDataFactors,
    ) -> Result<Self, Failure> {
        let gp = live_geometry(parabola, "GeomToStepMakeParabola")?.parab2d()?;
        Ok(Self::with_parts(
            &select_axis2d(&gp.axis(), factors)?,
            gp.focal(),
        ))
    }
    /// `GeomToStepMakeParabola(GeomParabola, factors)`.
    pub fn from_geom(
        parabola: &Handle<GeomParabola>,
        factors: &StepDataFactors,
    ) -> Result<Self, Failure> {
        let gp = live_geometry(parabola, "GeomToStepMakeParabola")?.parab()?;
        let select = select_axis3d(&gp.position(), factors)?;
        Ok(Self::with_parts(
            &select,
            gp.focal() / factors.length_factor(),
        ))
    }
}

geom_to_step_class!(
    /// `GeomToStepMakeConic`: the dispatch over the conics.
    GeomToStepMakeConic,
    "GeomToStepMakeConic",
    StepGeomConic
);

impl GeomToStepMakeConic {
    /// `GeomToStepMakeConic(GeomConic, factors)`; another conic is not done.
    pub fn from_geom(
        conic: &Handle<dyn GeomConic>,
        factors: &StepDataFactors,
    ) -> Result<Self, Failure> {
        let circle = conic.cast::<GeomCircle>();
        if circle.is_some() {
            return Ok(Self::done_with(
                GeomToStepMakeCircle::from_geom(&circle, factors)?
                    .value()?
                    .cast(),
            ));
        }
        let ellipse = conic.cast::<GeomEllipse>();
        if ellipse.is_some() {
            return Ok(Self::done_with(
                GeomToStepMakeEllipse::from_geom(&ellipse, factors)?
                    .value()?
                    .cast(),
            ));
        }
        let hyperbola = conic.cast::<GeomHyperbola>();
        if hyperbola.is_some() {
            return Ok(Self::done_with(
                GeomToStepMakeHyperbola::from_geom(&hyperbola, factors)?
                    .value()?
                    .cast(),
            ));
        }
        let parabola = conic.cast::<GeomParabola>();
        if parabola.is_some() {
            return Ok(Self::done_with(
                GeomToStepMakeParabola::from_geom(&parabola, factors)?
                    .value()?
                    .cast(),
            ));
        }
        Ok(Self::not_done())
    }
    /// `GeomToStepMakeConic(Geom2dConic, factors)`; another 2D conic is not done.
    pub fn from_geom2d(
        conic: &Handle<dyn Geom2dConic>,
        factors: &StepDataFactors,
    ) -> Result<Self, Failure> {
        let circle = conic.cast::<Geom2dCircle>();
        if circle.is_some() {
            return Ok(Self::done_with(
                GeomToStepMakeCircle::from_geom2d(&circle, factors)?
                    .value()?
                    .cast(),
            ));
        }
        let ellipse = conic.cast::<Geom2dEllipse>();
        if ellipse.is_some() {
            return Ok(Self::done_with(
                GeomToStepMakeEllipse::from_geom2d(&ellipse, factors)?
                    .value()?
                    .cast(),
            ));
        }
        let hyperbola = conic.cast::<Geom2dHyperbola>();
        if hyperbola.is_some() {
            return Ok(Self::done_with(
                GeomToStepMakeHyperbola::from_geom2d(&hyperbola, factors)?
                    .value()?
                    .cast(),
            ));
        }
        let parabola = conic.cast::<Geom2dParabola>();
        if parabola.is_some() {
            return Ok(Self::done_with(
                GeomToStepMakeParabola::from_geom2d(&parabola, factors)?
                    .value()?
                    .cast(),
            ));
        }
        Ok(Self::not_done())
    }
}

geom_to_step_class!(
    /// `GeomToStepMakeCurve`: the dispatch over the curves.
    GeomToStepMakeCurve,
    "GeomToStepMakeCurve",
    StepGeomCurve
);

impl GeomToStepMakeCurve {
    /// The basis of a trimmed curve, cut to its range when it is a spline: the recorded
    /// conversion passes any other basis on untrimmed.
    fn trimmed_basis(trimmed: &GeomTrimmedCurve) -> Result<Handle<dyn GeomCurve>, Failure> {
        let basis = trimmed.basis_curve();
        let first = trimmed.first_parameter();
        let last = trimmed.last_parameter();
        if basis.cast::<GeomBSplineCurve>().is_some() {
            let copy = basis.copy()?.cast::<GeomBSplineCurve>();
            live_geometry(&copy, "GeomToStepMakeCurve")?.segment(
                first,
                last,
                precision::p_confusion(),
            )?;
            return Ok(copy.cast());
        }
        if basis.cast::<GeomBezierCurve>().is_some() {
            let copy = basis.copy()?.cast::<GeomBezierCurve>();
            live_geometry(&copy, "GeomToStepMakeCurve")?.segment(first, last)?;
            return Ok(copy.cast());
        }
        Ok(basis)
    }
    /// `GeomToStepMakeCurve(GeomCurve, factors)`; another curve is not done.
    pub fn from_geom(
        curve: &Handle<dyn GeomCurve>,
        factors: &StepDataFactors,
    ) -> Result<Self, Failure> {
        let line = curve.cast::<GeomLine>();
        if line.is_some() {
            return Ok(Self::done_with(
                GeomToStepMakeLine::from_geom(&line, factors)?
                    .value()?
                    .cast(),
            ));
        }
        let conic = curve.cast::<dyn GeomConic>();
        if conic.is_some() {
            return Ok(Self::done_with(
                GeomToStepMakeConic::from_geom(&conic, factors)?
                    .value()?
                    .cast(),
            ));
        }
        let trimmed = curve.cast::<GeomTrimmedCurve>();
        if let Some(live) = trimmed.get() {
            let basis = Self::trimmed_basis(live)?;
            return Ok(Self::done_with(Self::from_geom(&basis, factors)?.value()?));
        }
        let bounded = curve.cast::<dyn GeomBoundedCurve>();
        if bounded.is_some() {
            let made = GeomToStepMakeBoundedCurve::from_geom(&bounded, factors)?;
            return Ok(Self::done_with(made.value()?.cast()));
        }
        Ok(Self::not_done())
    }
    /// A circle or an ellipse with an indirect axis, which STEP cannot map, as a B-spline.
    fn indirect_conic_as_bspline(
        spline: Handle<Geom2dBSplineCurve>,
        factors: &StepDataFactors,
    ) -> Result<Self, Failure> {
        let bounded: Handle<dyn Geom2dBoundedCurve> = spline.cast();
        let made = GeomToStepMakeBoundedCurve::from_geom2d(&bounded, factors)?;
        Ok(Self::done_with(made.value()?.cast()))
    }
    /// True for a 2D circle or ellipse whose axis is indirect.
    fn is_indirect_conic(curve: &Handle<dyn Geom2dCurve>) -> Result<bool, Failure> {
        if let Some(circle) = curve.cast::<Geom2dCircle>().get() {
            return Ok(!circle.circ2d()?.is_direct());
        }
        if let Some(ellipse) = curve.cast::<Geom2dEllipse>().get() {
            return Ok(!ellipse.elips2d()?.is_direct());
        }
        Ok(false)
    }
    /// A 2D conic: indirect circles and ellipses are converted to avoid changing the topology.
    fn from_conic2d(
        curve: &Handle<dyn Geom2dCurve>,
        factors: &StepDataFactors,
    ) -> Result<Self, Failure> {
        if Self::is_indirect_conic(curve)? {
            let spline =
                geom2d_convert::curve_to_bspline_curve(curve, ParameterisationType::TgtThetaOver2)?;
            return Self::indirect_conic_as_bspline(spline, factors);
        }
        let made = GeomToStepMakeConic::from_geom2d(&curve.cast::<dyn Geom2dConic>(), factors)?;
        Ok(Self::done_with(made.value()?.cast()))
    }
    /// `GeomToStepMakeCurve(Geom2dCurve, factors)`; another curve is not done.
    ///
    /// `Geom2dTrimmedCurve` derives from the bounded curves, so the bounded branch takes it
    /// and fails in `Value`: the later trimmed branch is unreachable also.
    pub fn from_geom2d(
        curve: &Handle<dyn Geom2dCurve>,
        factors: &StepDataFactors,
    ) -> Result<Self, Failure> {
        let line = curve.cast::<Geom2dLine>();
        if line.is_some() {
            return Ok(Self::done_with(
                GeomToStepMakeLine::from_geom2d(&line, factors)?
                    .value()?
                    .cast(),
            ));
        }
        if curve.cast::<dyn Geom2dConic>().is_some() {
            return Self::from_conic2d(curve, factors);
        }
        let bounded = curve.cast::<dyn Geom2dBoundedCurve>();
        if bounded.is_some() {
            let made = GeomToStepMakeBoundedCurve::from_geom2d(&bounded, factors)?;
            return Ok(Self::done_with(made.value()?.cast()));
        }
        let trimmed = curve.cast::<Geom2dTrimmedCurve>();
        if let Some(live) = trimmed.get() {
            let made = Self::from_geom2d(&live.basis_curve(), factors)?;
            return Ok(Self::done_with(made.value()?));
        }
        Ok(Self::not_done())
    }
}
