use super::super::TopOpeBRepWPointInter;
use super::*;
use martensite_tkgeomalgo::int_patch::IntPatchPointLine;

impl TopOpeBRepLineInter {
    /// Reads the indexed point, including native slot zero; `line.vpoint(1)?`.
    pub fn vpoint(&self, index: i32) -> Result<&TopOpeBRepVPointInter<'static>, Failure> {
        let index = self.point_index(index, "VPoint")?;
        Ok(&self.points[index])
    }
    /// Mutates one classification record; `line.change_vpoint(1)?.change_keep(true)`.
    pub fn change_vpoint(
        &mut self,
        index: i32,
    ) -> Result<&mut TopOpeBRepVPointInter<'static>, Failure> {
        let index = self.point_index(index, "ChangeVPoint")?;
        Ok(&mut self.points[index])
    }
    /// Detects exact parameter coincidence among kept points; `line.set_inl()?`.
    pub fn set_inl(&mut self) -> Result<(), Failure> {
        let parameters: Vec<_> = self
            .kept_points()
            .map(|(_, point)| point.parameter_on_line())
            .collect::<Result<_, _>>()?;
        self.inl = parameters
            .first()
            .is_some_and(|first| parameters.iter().all(|parameter| parameter == first));
        Ok(())
    }
    /// Determines closure from extremal kept noninternal points; `line.set_is_v_closed()?`.
    pub fn set_is_v_closed(&mut self) -> Result<(), Failure> {
        if self.inl {
            self.closed = false;
            return Ok(());
        }
        if self.nb_vpoint() < 2 {
            self.ok = false;
            self.closed = false;
            return Ok(());
        }
        let bounds = self.noninternal_extrema()?;
        self.closed = match bounds {
            None => true,
            Some((first, last)) => {
                let (a, b) = (first.tolerance()?, last.tolerance()?);
                let tolerance = if a < b { b } else { a };
                first.value()?.is_equal(&last.value()?, tolerance)
            }
        };
        Ok(())
    }
    fn noninternal_extrema(
        &self,
    ) -> Result<
        Option<(
            &TopOpeBRepVPointInter<'static>,
            &TopOpeBRepVPointInter<'static>,
        )>,
        Failure,
    > {
        let mut minimum = f64::MAX;
        let mut maximum = -f64::MAX;
        let (mut first, mut last) = (None, None);
        for (_, point) in self.kept_points() {
            if point.is_internal() {
                continue;
            }
            let parameter = point.parameter_on_line()?;
            if parameter < minimum {
                minimum = parameter;
                first = Some(point);
            }
            if parameter > maximum {
                maximum = parameter;
                last = Some(point);
            }
        }
        Ok(first.zip(last))
    }
    /// Finds a kept point on either surface domain; `line.set_has_vp_on_r()?`.
    pub fn set_has_vp_on_r(&mut self) -> Result<(), Failure> {
        let mut found = false;
        for (_, point) in self.kept_points() {
            if point.is_on_dom_s1()? || point.is_on_dom_s2()? {
                found = true;
                break;
            }
        }
        self.has_restriction = found;
        Ok(())
    }
    /// Computes first/last kept indices and their count; `line.set_vp_bounds()`.
    pub fn set_vp_bounds(&mut self) {
        let indices: Vec<_> = self.kept_points().map(|(index, _)| index).collect();
        self.point_bounds = Some((
            indices.first().copied().unwrap_or(self.nb_vpoint() + 1),
            indices.last().copied().unwrap_or(0),
            indices.len() as i32,
        ));
    }
    /// Reads computed bounds, invalidating an uninitialized query; `line.vp_bounds()`.
    pub fn vp_bounds(&mut self) -> (i32, i32, i32) {
        self.point_bounds.unwrap_or_else(|| {
            self.ok = false;
            (0, 0, 0)
        })
    }
    /// Reports whether a kept point is internal to both domains; `line.has_v_internal()?`.
    pub fn has_v_internal(&self) -> Result<bool, Failure> {
        for (_, point) in self.kept_points() {
            if point.is_internal() {
                return Ok(true);
            }
        }
        Ok(false)
    }
    /// Counts sampled points only for walking lines; `line.nb_wpoint()?`.
    pub fn nb_wpoint(&self) -> Result<i32, Failure> {
        if self.kind != TopOpeBRepTypeLineCurve::Walking {
            return Ok(0);
        }
        self.walking
            .get()
            .ok_or_else(|| failure("NbWPoint", "null WLine"))?
            .nb_pnts()
    }
    /// Loads the current sampled point and borrows it safely; `line.wpoint(1)?`.
    pub fn wpoint(&mut self, index: i32) -> Result<TopOpeBRepWPointInter<'_>, Failure> {
        let source: Option<Handle<dyn IntPatchPointLine>> = match self.kind {
            TopOpeBRepTypeLineCurve::Restriction => Some(self.restriction.cast()),
            TopOpeBRepTypeLineCurve::Walking => Some(self.walking.cast()),
            _ => None,
        };
        if let Some(source) = source {
            self.current_walk = Some(
                source
                    .get()
                    .ok_or_else(|| failure("WPoint", index))?
                    .point(index)?,
            );
        }
        let mut view = TopOpeBRepWPointInter::new();
        if let Some(point) = &self.current_walk {
            view.set(point);
        }
        Ok(view)
    }
    fn point_index(&self, index: i32, member: &str) -> Result<usize, Failure> {
        let kind = if self.points.is_empty() {
            "Standard_NullObject"
        } else {
            "Standard_OutOfRange"
        };
        if index < 0 || index as usize >= self.points.len() {
            return Err(Failure::with_message(
                kind,
                &format!(
                    "TopOpeBRepLineInter::{member}: index={index}; expected 0..={}",
                    self.nb_vpoint()
                ),
            ));
        }
        Ok(index as usize)
    }
    fn kept_points(&self) -> impl Iterator<Item = (i32, &TopOpeBRepVPointInter<'static>)> {
        self.points
            .iter()
            .enumerate()
            .skip(1)
            .filter(|(_, point)| point.keep())
            .map(|(index, point)| (index as i32, point))
    }
}
