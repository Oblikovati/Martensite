//! Owned extension geometry and public representation fixtures with independent expectations.
use super::bound_failure;
use crate::curve_update_rebinding::rebinding_curve::RebindingParameterCurve;
use crate::curve3d_copy_callbacks::space_curve::RebindingSpaceCurve;
use martensite_tkbrep::brep::{
    BRepCurve3D, BRepCurveOnClosedSurface, BRepCurveOnSurface, BRepCurveRepresentation, BRepGCurve,
};
use martensite_tkernel::standard::{Failure, Handle, SharedState};
use martensite_tkg2d::geom2d::{Geom2dCurve, Geom2dLine};
use martensite_tkg3d::geom::GeomCurve;
use martensite_tkmath::{
    geom_abs::Shape,
    gp::{Axis2d, Point2, Point3, Transform3, Vector3},
    toploc::Location,
};
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

type BoundQuery = Arc<dyn Fn(&'static str) -> Result<f64, Failure> + Send + Sync>;
pub type QueryTrace = Arc<SharedState<Vec<&'static str>>>;

#[derive(Clone, Copy)]
pub enum RepresentationKind {
    Space,
    Surface,
    Seam,
}
enum QueryCurve {
    Space(Handle<RebindingSpaceCurve>),
    Planar(Handle<RebindingParameterCurve>),
}

pub struct SourceFixture {
    pub source: Handle<dyn BRepGCurve>,
    curve: QueryCurve,
    pub evaluations: Arc<AtomicUsize>,
}

impl SourceFixture {
    pub fn new(kind: RepresentationKind) -> Self {
        match kind {
            RepresentationKind::Space => Self::spatial(),
            RepresentationKind::Surface => Self::planar(false),
            RepresentationKind::Seam => Self::planar(true),
        }
    }

    fn spatial() -> Self {
        let evaluations = Arc::new(AtomicUsize::new(0));
        let curve = Handle::new(RebindingSpaceCurve::new(
            evaluations.clone(),
            Arc::new(|u| Ok(Point3::new(u, 0., 0.))),
        ));
        let source = Handle::new(BRepCurve3D::new(&curve.cast(), &Location::default())).cast();
        Self {
            source,
            curve: QueryCurve::Space(curve),
            evaluations,
        }
    }

    fn planar(seam: bool) -> Self {
        let evaluations = Arc::new(AtomicUsize::new(0));
        let curve = Handle::new(RebindingParameterCurve::new(
            evaluations.clone(),
            Arc::new(|u| Ok(Point2::new(u, 0.))),
        ));
        let source = planar_source(&curve.cast(), seam);
        Self {
            source,
            curve: QueryCurve::Planar(curve),
            evaluations,
        }
    }

    pub fn prepare(&self) {
        self.source.set_range(2., 4.).unwrap();
        self.evaluations.store(0, Ordering::SeqCst);
    }

    pub fn install(&self, query: BoundQuery) {
        match &self.curve {
            QueryCurve::Space(curve) => curve.set_fallible_query(query),
            QueryCurve::Planar(curve) => curve.set_fallible_query(query),
        }
    }

    pub fn bound_to(&self, copy: &dyn BRepCurveRepresentation) -> bool {
        match &self.curve {
            QueryCurve::Space(curve) => copy.curve3d().unwrap() == curve.cast::<dyn GeomCurve>(),
            QueryCurve::Planar(curve) => copy.pcurve().unwrap() == curve.cast::<dyn Geom2dCurve>(),
        }
    }

    fn rebinder(&self, owner: &Handle<dyn BRepGCurve>) -> Arc<dyn Fn() + Send + Sync> {
        let owner = owner.downgrade();
        match &self.curve {
            QueryCurve::Space(curve) => {
                let replacement = curve.cast::<dyn GeomCurve>();
                Arc::new(move || owner.upgrade().set_curve3d(&replacement).unwrap())
            }
            QueryCurve::Planar(curve) => {
                let replacement = curve.cast::<dyn Geom2dCurve>();
                Arc::new(move || owner.upgrade().set_pcurve(&replacement).unwrap())
            }
        }
    }
}

pub fn failure_query(fixture: &SourceFixture, mode: &'static str, trace: QueryTrace) {
    let owner = fixture.source.downgrade();
    fixture.install(Arc::new(move |name| {
        trace.write().push(name);
        let mut transform = Transform3::default();
        transform.set_translation(Vector3::new(trace.read().len() as f64, 0., 0.));
        owner.upgrade().set_location(&Location::new(transform));
        if mode == "both" || mode == name {
            return Err(bound_failure());
        }
        Ok(if name == "first" { -7. } else { 19. })
    }));
}

pub fn replacement_queries(
    original: &SourceFixture,
    replacement: &SourceFixture,
    trace: QueryTrace,
    fail_replacement: bool,
) {
    original.install(old_replacement_query(
        replacement.rebinder(&original.source),
        trace.clone(),
    ));
    replacement.install(new_replacement_query(trace, fail_replacement));
}

fn old_replacement_query(replace: Arc<dyn Fn() + Send + Sync>, trace: QueryTrace) -> BoundQuery {
    Arc::new(move |name| {
        trace.write().push(query_name("old", name));
        replace();
        Ok(if name == "first" { -7. } else { 19. })
    })
}

fn new_replacement_query(trace: QueryTrace, fail: bool) -> BoundQuery {
    Arc::new(move |name| {
        trace.write().push(query_name("new", name));
        if fail {
            return Err(bound_failure());
        }
        Ok(if name == "first" { -31. } else { 47. })
    })
}

fn query_name(binding: &str, name: &str) -> &'static str {
    match (binding, name) {
        ("old", "first") => "old.first",
        ("old", _) => "old.last",
        (_, "first") => "new.first",
        _ => "new.last",
    }
}

fn planar_source(primary: &Handle<dyn Geom2dCurve>, seam: bool) -> Handle<dyn BRepGCurve> {
    if seam {
        return seam_source(primary);
    }
    Handle::new(BRepCurveOnSurface::new(primary, &Handle::default(), &Location::default()).unwrap())
        .cast()
}

fn seam_source(primary: &Handle<dyn Geom2dCurve>) -> Handle<dyn BRepGCurve> {
    let second: Handle<dyn Geom2dCurve> = Handle::new(Geom2dLine::new(Axis2d::default())).cast();
    let surface = Handle::default();
    let location = Location::default();
    Handle::new(
        BRepCurveOnClosedSurface::new(primary, &second, &surface, &location, Shape::C0).unwrap(),
    )
    .cast()
}
