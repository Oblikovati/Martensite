use super::*;
use martensite_tkbrep::brep_adaptor::BRepAdaptorSurface;
use martensite_tkg3d::adaptor3d::Adaptor3dSurface;
use martensite_tkgeomalgo::int_patch::{
    IntPatchALine, IntPatchALineToWLine, IntPatchIType, IntPatchPoint, IntPatchPointLine,
};

impl TopOpeBRepLineInter {
    /// Loads vertices and converts analytic lines to their first walking segment.
    /// `line.set_line(&intersection, &surface1, &surface2)?`.
    pub fn set_line(
        &mut self,
        source: &Handle<dyn IntPatchLine>,
        first: &BRepAdaptorSurface,
        second: &BRepAdaptorSurface,
    ) -> Result<(), Failure> {
        let kind = source
            .get()
            .ok_or_else(|| failure("SetLine", "null source"))?
            .arc_type();
        self.source = source.clone();
        self.kind = line_kind(kind);
        self.load_handle(kind, first, second)?;
        let count = self.source_vertex_count()?;
        let mut points = vec![TopOpeBRepVPointInter::new()];
        for index in 1..=count {
            let mut point = TopOpeBRepVPointInter::new();
            point.set_owned_point(self.source_vertex(index)?);
            point.set_index(index);
            points.push(point);
        }
        self.points = points;
        Ok(())
    }
    fn load_handle(
        &mut self,
        kind: IntPatchIType,
        first: &BRepAdaptorSurface,
        second: &BRepAdaptorSurface,
    ) -> Result<(), Failure> {
        match kind {
            IntPatchIType::Analytic => self.convert_analytic(first, second)?,
            IntPatchIType::Walking => self.walking = self.source.cast(),
            IntPatchIType::Restriction => self.restriction = self.source.cast(),
            _ => self.geometric = self.source.cast(),
        }
        Ok(())
    }
    fn convert_analytic(
        &mut self,
        first: &BRepAdaptorSurface,
        second: &BRepAdaptorSurface,
    ) -> Result<(), Failure> {
        let s1: Handle<dyn Adaptor3dSurface> = Handle::new(first.clone()).cast();
        let s2: Handle<dyn Adaptor3dSurface> = Handle::new(second.clone()).cast();
        let converter = IntPatchALineToWLine::new(&s1, &s2, 200)?;
        let mut converted = Vec::new();
        converter.make_wline(&self.source.cast::<IntPatchALine>(), &mut converted)?;
        if let Some(first) = converted.first() {
            self.walking = first.cast();
        }
        self.kind = TopOpeBRepTypeLineCurve::Walking;
        Ok(())
    }
    fn source_vertex_count(&self) -> Result<i32, Failure> {
        match self.kind {
            TopOpeBRepTypeLineCurve::Walking => Ok(self
                .walking
                .get()
                .ok_or_else(|| failure("NbVertex", "null WLine"))?
                .nb_vertex()),
            TopOpeBRepTypeLineCurve::Restriction => Ok(self
                .restriction
                .get()
                .ok_or_else(|| failure("NbVertex", "null RLine"))?
                .nb_vertex()),
            _ => Ok(self
                .geometric
                .get()
                .ok_or_else(|| failure("NbVertex", "null GLine"))?
                .nb_vertex()),
        }
    }
    fn source_vertex(&self, index: i32) -> Result<IntPatchPoint, Failure> {
        match self.kind {
            TopOpeBRepTypeLineCurve::Walking => self
                .walking
                .get()
                .ok_or_else(|| failure("Vertex", "null WLine"))?
                .vertex(index),
            TopOpeBRepTypeLineCurve::Restriction => self
                .restriction
                .get()
                .ok_or_else(|| failure("Vertex", "null RLine"))?
                .vertex(index),
            _ => self
                .geometric
                .get()
                .ok_or_else(|| failure("Vertex", "null GLine"))?
                .vertex(index),
        }
    }
}
fn line_kind(kind: IntPatchIType) -> TopOpeBRepTypeLineCurve {
    use TopOpeBRepTypeLineCurve as T;
    match kind {
        IntPatchIType::Analytic => T::Analytic,
        IntPatchIType::Restriction => T::Restriction,
        IntPatchIType::Walking => T::Walking,
        IntPatchIType::Lin => T::Line,
        IntPatchIType::Circle => T::Circle,
        IntPatchIType::Ellipse => T::Ellipse,
        IntPatchIType::Parabola => T::Parabola,
        IntPatchIType::Hyperbola => T::Hyperbola,
    }
}
