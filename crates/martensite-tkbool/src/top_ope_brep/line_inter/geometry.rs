use super::*;
use martensite_tkbrep::brep_adaptor::BRepAdaptorCurve2d;
use martensite_tkg3d::geom::{
    GeomCircle, GeomCurve, GeomEllipse, GeomHyperbola, GeomLine, GeomParabola, GeomTrimmedCurve,
};

impl TopOpeBRepLineInter {
    /// Recognizes periodic conics; `line.is_periodic()`.
    pub fn is_periodic(&self) -> bool {
        matches!(
            self.kind,
            TopOpeBRepTypeLineCurve::Circle | TopOpeBRepTypeLineCurve::Ellipse
        )
    }
    /// Returns the stored bounds' difference; `line.period()?`.
    pub fn period(&mut self) -> Result<f64, Failure> {
        let (first, last) = self.bounds()?;
        Ok(last - first)
    }
    /// Uses geometric first/last vertices, otherwise periodic natural bounds; `line.bounds()?`.
    pub fn bounds(&mut self) -> Result<(f64, f64), Failure> {
        let Some(line) = self.geometric.get() else {
            self.ok = false;
            return Ok((0., 0.));
        };
        let mut first = 0.;
        let mut last = if self.is_periodic() {
            std::f64::consts::TAU
        } else {
            0.
        };
        if line.has_first_point() {
            first = line.first_point()?.parameter_on_line();
        }
        if line.has_last_point() {
            last = line.last_point()?.parameter_on_line();
        }
        Ok((first, last))
    }
    /// Creates the corresponding exact conic, or a null handle for nonconics; `line.curve()?`.
    pub fn curve(&mut self) -> Result<Handle<dyn GeomCurve>, Failure> {
        use TopOpeBRepTypeLineCurve as T;
        if !matches!(
            self.kind,
            T::Line | T::Circle | T::Ellipse | T::Parabola | T::Hyperbola
        ) {
            self.ok = false;
            return Ok(Handle::default());
        }
        self.conic_curve()
    }
    fn conic_curve(&self) -> Result<Handle<dyn GeomCurve>, Failure> {
        use TopOpeBRepTypeLineCurve as T;
        let source = self
            .geometric
            .get()
            .ok_or_else(|| failure("Curve", "null GLine"))?;
        Ok(match self.kind {
            T::Line => Handle::new(GeomLine::from_lin(source.line()?)).cast(),
            T::Circle => Handle::new(GeomCircle::from_circ(source.circle()?)).cast(),
            T::Ellipse => Handle::new(GeomEllipse::from_elips(source.ellipse()?)).cast(),
            T::Parabola => Handle::new(GeomParabola::from_parab(source.parabola()?)).cast(),
            T::Hyperbola => Handle::new(GeomHyperbola::from_hypr(source.hyperbola()?)).cast(),
            _ => unreachable!(),
        })
    }
    /// Restricts the conic to a parameter range; `line.trimmed_curve(0., 1.)?`.
    pub fn trimmed_curve(
        &mut self,
        first: f64,
        last: f64,
    ) -> Result<Handle<dyn GeomCurve>, Failure> {
        Ok(Handle::new(GeomTrimmedCurve::new(
            &self.curve()?,
            first,
            last,
            true,
            true,
        )?)
        .cast())
    }
    /// Returns the original restriction edge, or a null shape; `line.arc()?`.
    pub fn arc(&self) -> Result<Shape, Failure> {
        if self.kind != TopOpeBRepTypeLineCurve::Restriction {
            return Ok(self.null_shape.clone());
        }
        let line = self
            .restriction
            .get()
            .ok_or_else(|| failure("Arc", "null RLine"))?;
        let source = if line.is_arc_on_s1() {
            line.arc_on_s1()
        } else {
            line.arc_on_s2()
        };
        let curve: Handle<BRepAdaptorCurve2d> = source.cast();
        Ok(curve
            .get()
            .ok_or_else(|| failure("Arc", "non-topological restriction"))?
            .edge()
            .into_shape())
    }
    /// Identifies the restriction's owning input face; `line.arc_is_edge(1)?`.
    pub fn arc_is_edge(&self, index: i32) -> Result<bool, Failure> {
        if self.kind != TopOpeBRepTypeLineCurve::Restriction {
            return Ok(false);
        }
        let first = self
            .restriction
            .get()
            .ok_or_else(|| failure("ArcIsEdge", "null RLine"))?
            .is_arc_on_s1();
        Ok(if index == 2 { !first } else { first })
    }
    /// Reads the geometric first-point flag; `line.has_first_point()?`.
    pub fn has_first_point(&self) -> Result<bool, Failure> {
        Ok(self
            .geometric
            .get()
            .ok_or_else(|| failure("HasFirstPoint", "line != GLine"))?
            .has_first_point())
    }
    /// Reads the geometric last-point flag; `line.has_last_point()?`.
    pub fn has_last_point(&self) -> Result<bool, Failure> {
        Ok(self
            .geometric
            .get()
            .ok_or_else(|| failure("HasLastPoint", "line != GLine"))?
            .has_last_point())
    }
}
