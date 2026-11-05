//! Current-line validation uses the source count, endpoint, and conic rules.
use super::*;
use crate::top_ope_brep_ds::TopOpeBRepDSTransition;

impl TopOpeBRepFacesFiller<'_> {
    /// Validates and loads the explicitly borrowed current line; `filler.load_line(&mut line)?`.
    pub fn load_line(&mut self, line: &mut TopOpeBRepLineInter) -> Result<(), Failure> {
        self.current_kind = Some(line.type_line_curve());
        let mut checked = self.check_line_current(line)?;
        self.line_inl = line.inl();
        if !checked && self.line_inl && line.type_line_curve() == TopOpeBRepTypeLineCurve::Line {
            checked = true;
        }
        line.set_ok(checked);
        self.line_ok = checked;
        if !checked {
            return Ok(());
        }
        line.compute_face_face_transition()?;
        Ok(())
    }

    fn check_line_current(&self, line: &mut TopOpeBRepLineInter) -> Result<bool, Failure> {
        self.check_line_with_retained(line, None)
    }
    /// Validates independent input against an unbound or externally borrowed current line; `filler.check_line(&mut line)?`.
    /// Shared-array bindings require `check_line_in_array` or `check_external_line_with_array` to avoid re-locking.
    pub fn check_line(&mut self, line: &mut TopOpeBRepLineInter) -> Result<bool, Failure> {
        if matches!(
            self.bound_line,
            Some(line_binding::BoundLine::Shared { .. })
        ) {
            return Err(line_array::shared_line_requires_guard());
        }
        let Some(mut retained) = self.bound_line.take() else {
            return self.check_line_current(line);
        };
        let result = retained.check(self, line);
        self.bound_line = Some(retained);
        result
    }
    pub(super) fn check_line_with_retained(
        &self,
        line: &mut TopOpeBRepLineInter,
        retained: Option<&mut TopOpeBRepLineInter>,
    ) -> Result<bool, Failure> {
        let tolerance = self
            .intersector
            .as_deref()
            .ok_or_else(classification::missing_intersector)?
            .get_tolerances()
            .0;
        use TopOpeBRepTypeLineCurve as Kind;
        let kind = line.type_line_curve();
        match kind {
            Kind::Walking => Ok(line.nb_vpoint() >= 2),
            Kind::Line => distinct_kept_vertices(line),
            Kind::Circle | Kind::Ellipse | Kind::Hyperbola => {
                conic_line_bounds(retained.unwrap_or(line), kind, tolerance)
            }
            _ => Ok(true),
        }
    }

    /// Reads a specified line's transition; `Self::face_face_transition_of(&line,1)?`.
    pub fn face_face_transition_of(
        line: &TopOpeBRepLineInter,
        index: i32,
    ) -> Result<&TopOpeBRepDSTransition, Failure> {
        line.face_face_transition(index)
    }

    /// Fills a point not on restrictions only at line bounds or an already known geometry; `filler.process_vpnoton_r(&point,&mut line)?`.
    pub fn process_vpnoton_r(
        &mut self,
        point: &TopOpeBRepVPointInter<'_>,
        line: &mut TopOpeBRepLineInter,
    ) -> Result<(), Failure> {
        let (first, last, _) = line.vp_bounds();
        let mut position = 0;
        let found = self
            .get_geometry(&self.curve_interferences, &mut position, point)?
            .is_some();
        if !found && point.index() != first && point.index() != last {
            return Ok(());
        }
        self.fill_line_point(point, point.index(), first, last)
    }
}

fn distinct_kept_vertices(line: &TopOpeBRepLineInter) -> Result<bool, Failure> {
    let indices: Vec<_> = (1..=line.nb_vpoint())
        .filter(|index| line.vpoint(*index).is_ok_and(|point| point.keep()))
        .collect();
    if indices.len() != 2 {
        return Ok(true);
    }
    let first = preferred_vertex(line.vpoint(indices[0])?)?;
    let second = preferred_vertex(line.vpoint(indices[1])?)?;
    Ok(first.is_null() || !first.is_same(&second))
}

fn preferred_vertex(point: &TopOpeBRepVPointInter<'_>) -> Result<Shape, Failure> {
    let mut vertex = Shape::default();
    if point.is_vertex_on_s1()? {
        vertex = point.vertex_on_s1()?;
    }
    if point.is_vertex_on_s2()? {
        vertex = point.vertex_on_s2()?;
    }
    Ok(vertex)
}

fn conic_line_bounds(
    line: &mut TopOpeBRepLineInter,
    kind: TopOpeBRepTypeLineCurve,
    tolerance: f64,
) -> Result<bool, Failure> {
    let (first, last, count) = line.vp_bounds();
    if count < 2 {
        return Ok(kind == TopOpeBRepTypeLineCurve::Circle);
    }
    if kind == TopOpeBRepTypeLineCurve::Hyperbola {
        return Ok(true);
    }
    let difference =
        (line.vpoint(first)?.parameter_on_line()? - line.vpoint(last)?.parameter_on_line()?).abs();
    Ok(!is_below(difference, tolerance))
}

/// `difference < tolerance`; a NaN difference is not below, so such a line is kept as before.
fn is_below(difference: f64, tolerance: f64) -> bool {
    difference.partial_cmp(&tolerance) == Some(std::cmp::Ordering::Less)
}

#[test]
fn a_nan_difference_is_not_below_the_tolerance() {
    assert!(is_below(0.5, 1.0));
    assert!(!is_below(1.0, 1.0));
    assert!(!is_below(f64::NAN, 1.0));
}
