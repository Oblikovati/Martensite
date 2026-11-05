//! One-based intersection traversal and keep filtering against native state rules.
use martensite_tkbool::top_ope_brep::{TopOpeBRepLineInter, TopOpeBRepVPointInterIterator};
use martensite_tkbrep::brep_adaptor::BRepAdaptorSurface;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkgeomalgo::int_patch::{
    IntPatchGLine, IntPatchLine, IntPatchLineTransitions, IntPatchPoint,
};
use martensite_tkmath::gp::{Direction3, Line3, Point3};

fn three_point_line() -> Result<TopOpeBRepLineInter, Failure> {
    let axis = Line3::new(Point3::default(), Direction3::X);
    let geometric = IntPatchGLine::from_line(&axis, false, IntPatchLineTransitions::Undecided)?;
    for parameter in [1., 2., 3.] {
        let mut point = IntPatchPoint::new();
        point.set_value_tangent(Point3::new(parameter, 0., 0.), 1e-7, false);
        point.set_parameter(parameter);
        geometric.add_vertex(&point)?;
    }
    let source: Handle<dyn IntPatchLine> = Handle::new(geometric).cast();
    let mut line = TopOpeBRepLineInter::new();
    line.set_line(
        &source,
        &BRepAdaptorSurface::default(),
        &BRepAdaptorSurface::default(),
    )?;
    Ok(line)
}

#[test]
fn unbound_iterator_preserves_native_index_count_defaults() {
    let mut iterator = TopOpeBRepVPointInterIterator::new();
    assert!(iterator.more());
    assert_eq!(iterator.current_vp_index().unwrap(), 0);
    assert!(iterator.line_inter().is_none());
    assert_eq!(
        iterator.current_vp().err().unwrap().exception_type(),
        "Standard_NullObject"
    );
    assert!(iterator.restart().is_err());
    iterator.next().unwrap();
    assert!(!iterator.more());
    assert_eq!(
        iterator.current_vp_index().unwrap_err().message(),
        "TopOpeBRepVPointInterIterator::CurrentVPIndex"
    );
}

#[test]
fn unfiltered_traversal_visits_every_index_and_mutates_the_line() -> Result<(), Failure> {
    let mut line = three_point_line()?;
    let identity = &line as *const TopOpeBRepLineInter;
    let mut iterator = TopOpeBRepVPointInterIterator::with_line(&mut line)?;
    assert_eq!(iterator.line_inter().unwrap() as *const _, identity);
    for index in 1..=3 {
        assert_eq!(iterator.current_vp_index()?, index);
        assert_eq!(iterator.current_vp()?.parameter_on_line()?, index as f64);
        iterator.change_current_vp()?.set_index(10 * index);
        iterator.next()?;
    }
    assert!(!iterator.more());
    assert_eq!(
        iterator.current_vp().err().unwrap().message(),
        "TopOpeBRepVPointInterIterator::CurrentVP"
    );
    assert_eq!(
        iterator.change_current_vp().err().unwrap().message(),
        "TopOpeBRepVPointInterIterator::ChangeCurrentVP"
    );
    iterator.restart()?;
    assert_eq!(iterator.current_vp()?.index(), 10);
    assert_eq!(line.vpoint(3)?.index(), 30);
    Ok(())
}

#[test]
fn filtering_skips_rejected_points_on_init_next_and_restart() -> Result<(), Failure> {
    let mut line = three_point_line()?;
    line.change_vpoint(2)?.change_keep(true);
    let mut iterator = TopOpeBRepVPointInterIterator::new();
    iterator.init(&mut line, true)?;
    assert_eq!(iterator.current_vp_index()?, 2);
    iterator.change_current_vp()?.change_keep(false);
    iterator.next()?;
    assert!(!iterator.more());
    iterator.restart()?;
    assert!(!iterator.more());
    Ok(())
}

#[test]
fn empty_initialized_line_terminates_at_index_one() -> Result<(), Failure> {
    let mut line = TopOpeBRepLineInter::new();
    let mut iterator = TopOpeBRepVPointInterIterator::with_line(&mut line)?;
    assert!(!iterator.more());
    iterator.restart()?;
    assert!(!iterator.more());
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<TopOpeBRepVPointInterIterator<'static>>();
    Ok(())
}
