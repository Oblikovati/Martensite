use super::*;
pub(super) fn keep(
    context: &TopOpeBRepKPartContext,
    pair: &Pairing<'_>,
    line: &mut Line,
    ds: &DS,
    tolerance_uv: f64,
    parameter: &mut f64,
    transition: &mut Transition,
) -> Result<bool, Failure> {
    correct_nonoriented_transition(pair, transition);
    if !pair.list.is_empty() && pair.point.index() == 1 {
        return Ok(false);
    }
    let previous = pair.previous()?;
    correct_unknown_transition(pair, &previous, transition);
    if previous.is_unknown() {
        return Ok(!leaving(transition) && pair.point.index() != pair.last);
    }
    let keep = !leaving(&previous) && leaving(transition);
    if !pair.same_parameter || !pair.seam {
        return Ok(keep);
    }
    if !line.has_first_point()? && !line.has_last_point()? {
        parameter::periodize(line, pair.found, parameter)?;
        *transition = previous.complement();
        return Ok(keep);
    }
    if !keep {
        return Ok(false);
    }
    let same_geometry = same_geometry(pair, ds)?;
    let same_uv = same_closed_uv(context, pair, line, tolerance_uv)?;
    Ok(!(same_geometry && same_uv))
}

fn correct_nonoriented_transition(pair: &Pairing<'_>, transition: &mut Transition) {
    if !pair.oriented_edge() {
        if pair.point.index() == pair.first {
            *transition = entering();
        }
        if pair.point.index() == pair.last {
            *transition = exiting();
        }
    }
}
fn correct_unknown_transition(
    pair: &Pairing<'_>,
    previous: &Transition,
    transition: &mut Transition,
) {
    if !transition.is_unknown() {
        return;
    }
    if pair.point.index() == pair.first {
        *transition = entering();
    }
    if pair.point.index() == pair.last && !previous.is_unknown() && !leaving(previous) {
        *transition = previous.complement();
    }
}
fn same_geometry(pair: &Pairing<'_>, ds: &DS) -> Result<bool, Failure> {
    let interference = pair
        .list
        .iter()
        .last()
        .ok_or_else(null_interference)?
        .get()
        .ok_or_else(null_interference)?;
    let point = PointTool::from_vpoint(pair.point)?;
    match interference.geometry_type() {
        Kind::Point => Ok(point.is_equal(ds.point(interference.geometry())?)),
        Kind::Vertex => Ok(point.is_equal(&PointTool::from_shape(
            ds.shape(interference.geometry(), true),
        )?)),
        _ => Ok(false),
    }
}
fn same_closed_uv(
    context: &TopOpeBRepKPartContext,
    pair: &Pairing<'_>,
    line: &Line,
    tolerance: f64,
) -> Result<bool, Failure> {
    // The source fetches the cached point even on an open line; retain its checked-index failure.
    let previous = line.vpoint(context.last_vpoint_index)?;
    if !line.is_v_closed() {
        return Ok(true);
    }
    for side in [1, 2] {
        if pair
            .point
            .surface_parameters(side)?
            .distance(previous.surface_parameters(side)?)
            .partial_cmp(&tolerance)
            != Some(std::cmp::Ordering::Less)
        {
            return Ok(false);
        }
    }
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use martensite_tkgeomalgo::int_patch::IntPatchPoint;
    use martensite_tkmath::gp::Point3;
    #[test]
    fn geometric_identity_uses_larger_recorded_tolerance() {
        let mut source = IntPatchPoint::new();
        source.set_value_tangent(Point3::new(0.1, 0.0, 0.0), 0.2, false);
        let mut point = VPoint::new();
        point.set_point(&source);
        let mut ds = DS::new();
        let index = ds.add_point(&crate::top_ope_brep_ds::TopOpeBRepDSPoint::new(
            Point3::default(),
            0.01,
        ));
        let mut list = Interferences::default();
        list.append(InterferenceTool::make_curve_interference(
            &entering(),
            Kind::Curve,
            1,
            Kind::Point,
            index,
            0.0,
        ));
        let pair = Pairing {
            found: &Handle::default(),
            list: &list,
            point: &point,
            edge: &Shape::default(),
            same_parameter: true,
            seam: true,
            first: 1,
            last: 2,
        };
        assert!(same_geometry(&pair, &ds).unwrap());
    }
    #[test]
    fn cached_open_line_point_is_checked_before_uv_shortcut() {
        let mut line = super::super::tests::line(false);
        let point = VPoint::new();
        let list = Interferences::default();
        let pair = Pairing {
            found: &Handle::default(),
            list: &list,
            point: &point,
            edge: &Shape::default(),
            same_parameter: true,
            seam: true,
            first: 1,
            last: 2,
        };
        assert!(same_closed_uv(&TopOpeBRepKPartContext::default(), &pair, &line, 1e-8).unwrap());
        line.set_ok(false);
        assert!(
            same_closed_uv(
                &TopOpeBRepKPartContext {
                    last_vpoint_index: 4
                },
                &pair,
                &line,
                1e-8
            )
            .is_err()
        );
    }
    #[test]
    fn closed_line_seam_requires_both_uv_pairs_with_strict_tolerance() {
        let mut line = super::super::tests::line(false);
        for index in 1..=3 {
            line.change_vpoint(index).unwrap().change_keep(true);
        }
        line.set_is_v_closed().unwrap();
        assert!(line.is_v_closed());
        let mut source = IntPatchPoint::new();
        source.set_parameters(1e-8, 0.0, 0.0, 0.0);
        let mut point = VPoint::new();
        point.set_point(&source);
        let pair = Pairing {
            found: &Handle::default(),
            list: &Interferences::default(),
            point: &point,
            edge: &Shape::default(),
            same_parameter: true,
            seam: true,
            first: 1,
            last: 3,
        };
        let context = TopOpeBRepKPartContext {
            last_vpoint_index: 1,
        };
        assert!(!same_closed_uv(&context, &pair, &line, 1e-8).unwrap());
        assert!(same_closed_uv(&context, &pair, &line, 2e-8).unwrap());
        assert!(!same_closed_uv(&context, &pair, &line, f64::NAN).unwrap());
    }
    #[test]
    fn repeated_first_point_still_updates_nonoriented_transition() {
        let mut point = VPoint::new();
        point.set_index(1);
        let mut edge = Shape::default();
        edge.set_orientation(Orientation::Internal);
        let mut list = Interferences::default();
        list.append(InterferenceTool::make_curve_interference(
            &entering(),
            Kind::Curve,
            1,
            Kind::Point,
            1,
            0.0,
        ));
        let pair = Pairing {
            found: &Handle::default(),
            list: &list,
            point: &point,
            edge: &edge,
            same_parameter: false,
            seam: false,
            first: 1,
            last: 3,
        };
        let mut transition = exiting();
        assert!(
            !keep(
                &TopOpeBRepKPartContext::default(),
                &pair,
                &mut super::super::tests::line(false),
                &DS::new(),
                1e-8,
                &mut 0.0,
                &mut transition
            )
            .unwrap()
        );
        assert_eq!(transition, entering());
    }
    #[test]
    fn cached_point_failure_is_preserved_when_geometry_differs() {
        let mut line = super::super::tests::line(false);
        line.line_g().get().unwrap().set_first_point(1);
        let mut source = IntPatchPoint::new();
        source.set_value_tangent(Point3::default(), 1e-8, false);
        let mut point = VPoint::new();
        point.set_point(&source);
        point.set_index(2);
        let mut ds = DS::new();
        let index = ds.add_point(&crate::top_ope_brep_ds::TopOpeBRepDSPoint::new(
            Point3::new(10.0, 0.0, 0.0),
            1e-8,
        ));
        let found = InterferenceTool::make_curve_interference(
            &entering(),
            Kind::Curve,
            1,
            Kind::Point,
            index,
            0.0,
        );
        let mut list = Interferences::default();
        list.append(found.clone());
        let pair = Pairing {
            found: &found,
            list: &list,
            point: &point,
            edge: &Shape::default(),
            same_parameter: true,
            seam: true,
            first: 1,
            last: 3,
        };
        let context = TopOpeBRepKPartContext {
            last_vpoint_index: 50,
        };
        assert!(
            keep(
                &context,
                &pair,
                &mut line,
                &ds,
                1e-8,
                &mut 0.0,
                &mut exiting()
            )
            .is_err()
        );
    }
}
