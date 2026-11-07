//! `IgesToBRep`: which entities are curves, surfaces and BRep entities, and the default containers.
use super::IgesToBRepAlgoContainer;
use crate::iges_basic::IgesBasicSingleParent;
use crate::iges_data::{IgesDataIgesEntity, iges_live};
use crate::iges_geom::{
    IgesGeomBSplineCurve, IgesGeomBSplineSurface, IgesGeomBoundary, IgesGeomBoundedSurface,
    IgesGeomCircularArc, IgesGeomCompositeCurve, IgesGeomConicArc, IgesGeomCopiousData,
    IgesGeomCurveOnSurface, IgesGeomLine, IgesGeomOffsetCurve, IgesGeomOffsetSurface,
    IgesGeomPlane, IgesGeomPoint, IgesGeomRuledSurface, IgesGeomSplineCurve, IgesGeomSplineSurface,
    IgesGeomSurfaceOfRevolution, IgesGeomTabulatedCylinder, IgesGeomTrimmedSurface,
};
use crate::iges_solid::{
    IgesSolidConicalSurface, IgesSolidCylindricalSurface, IgesSolidEdgeList, IgesSolidFace,
    IgesSolidLoop, IgesSolidManifoldSolid, IgesSolidPlaneSurface, IgesSolidShell,
    IgesSolidSphericalSurface, IgesSolidToroidalSurface, IgesSolidVertexList,
};
use martensite_tkbrep::brep::{BRepBuilder, BRepTool};
use martensite_tkbrep::topods::{Edge, Face};
use martensite_tkernel::precision;
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped, StandardTransient};
use martensite_tkg2d::geom2d::Geom2dCurve;
use martensite_tkg3d::top_abs::Orientation;
use martensite_tkgeombase::geom_lib;
use martensite_tkxsbase::xs_algo::XSAlgo;
use std::sync::{Mutex, MutexGuard, OnceLock, PoisonError};

const WHERE: &str = "IgesToBRep";

/// The default container, as a transfer finds it; the first init sets it.
static ALGO_CONTAINER: OnceLock<Mutex<Handle<IgesToBRepAlgoContainer>>> = OnceLock::new();

/// The reading actor and the settings of a transfer are one per process, so two transfers at the
/// same time would give wrong shapes with no failure.
static ONE_TRANSFER: Mutex<()> = Mutex::new(());

/// Holds the transfers of the other threads back while the guard lives. Not reentrant: only the
/// entry points of the readers take it. Example: `let _only_one = one_transfer_at_a_time();`.
pub(crate) fn one_transfer_at_a_time() -> MutexGuard<'static, ()> {
    ONE_TRANSFER.lock().unwrap_or_else(PoisonError::into_inner)
}

fn is_any_of<const N: usize>(start: &Handle<IgesDataIgesEntity>, kinds: [&str; N]) -> bool {
    kinds.iter().any(|kind| start.is_kind_name(kind))
}

/// The tools of the package.
///
/// ```
/// use martensite_tkdeiges::iges_data::IgesDataIgesEntity;
/// use martensite_tkdeiges::iges_to_brep::IgesToBRep;
/// use martensite_tkernel::standard::Handle;
/// assert!(!IgesToBRep::is_curve_and_surface(&Handle::<IgesDataIgesEntity>::default()));
/// ```
pub struct IgesToBRep;

impl IgesToBRep {
    /// Sets the default container, once; `IgesToBRep::init()?`.
    ///
    /// # Errors
    /// A failure of the init of the exchange algorithms.
    pub fn init() -> Result<(), Failure> {
        if ALGO_CONTAINER.get().is_some() {
            return Ok(());
        }
        XSAlgo::init()?;
        let _ = ALGO_CONTAINER.set(Mutex::new(Handle::new(IgesToBRepAlgoContainer::new())));
        Ok(())
    }
    /// Sets the default container; `IgesToBRep::set_algo_container(&container)?`.
    ///
    /// # Errors
    /// A failure of the init of the exchange algorithms.
    pub fn set_algo_container(container: &Handle<IgesToBRepAlgoContainer>) -> Result<(), Failure> {
        Self::init()?;
        if let Some(slot) = ALGO_CONTAINER.get() {
            *slot.lock().unwrap_or_else(PoisonError::into_inner) = container.clone();
        }
        Ok(())
    }
    /// The default container; `IgesToBRep::algo_container()?`.
    ///
    /// # Errors
    /// A null object failure when nothing has set it.
    pub fn algo_container() -> Result<Handle<IgesToBRepAlgoContainer>, Failure> {
        Self::init()?;
        let slot = ALGO_CONTAINER.get().ok_or_else(|| {
            martensite_tkernel::standard::exceptions::null_object(
                "IgesToBRep::algo_container: no container is set; expected init first",
            )
        })?;
        Ok(slot.lock().unwrap_or_else(PoisonError::into_inner).clone())
    }
    /// Whether the transfer of curves and surfaces takes an entity; `IgesToBRep::is_curve_and_surface(&entity)`.
    pub fn is_curve_and_surface(start: &Handle<IgesDataIgesEntity>) -> bool {
        Self::is_topo_curve(start) || Self::is_topo_surface(start) || Self::is_b_rep_entity(start)
    }
    /// Whether an entity is a basic curve (types 104, 110, 100, 106, 112, 126);
    /// `IgesToBRep::is_basic_curve(&entity)`.
    pub fn is_basic_curve(start: &Handle<IgesDataIgesEntity>) -> bool {
        is_any_of(
            start,
            [
                IgesGeomBSplineCurve::static_type().name(),
                IgesGeomLine::static_type().name(),
                IgesGeomCircularArc::static_type().name(),
                IgesGeomConicArc::static_type().name(),
                IgesGeomCopiousData::static_type().name(),
                IgesGeomSplineCurve::static_type().name(),
            ],
        )
    }
    /// Whether an entity is a basic surface (types 114, 128, 190 to 198);
    /// `IgesToBRep::is_basic_surface(&entity)`.
    pub fn is_basic_surface(start: &Handle<IgesDataIgesEntity>) -> bool {
        is_any_of(
            start,
            [
                IgesGeomBSplineSurface::static_type().name(),
                IgesGeomSplineSurface::static_type().name(),
                IgesSolidPlaneSurface::static_type().name(),
                IgesSolidCylindricalSurface::static_type().name(),
                IgesSolidConicalSurface::static_type().name(),
                IgesSolidSphericalSurface::static_type().name(),
                IgesSolidToroidalSurface::static_type().name(),
            ],
        )
    }
    /// Whether an entity is a curve (a basic one, or types 102, 116, 130, 141, 142);
    /// `IgesToBRep::is_topo_curve(&entity)`.
    pub fn is_topo_curve(start: &Handle<IgesDataIgesEntity>) -> bool {
        Self::is_basic_curve(start)
            || is_any_of(
                start,
                [
                    IgesGeomCompositeCurve::static_type().name(),
                    IgesGeomCurveOnSurface::static_type().name(),
                    IgesGeomBoundary::static_type().name(),
                    IgesGeomPoint::static_type().name(),
                    IgesGeomOffsetCurve::static_type().name(),
                ],
            )
    }
    /// A perforated face has only planes: as parent and as children.
    fn is_perforated_plane(start: &Handle<IgesDataIgesEntity>) -> bool {
        let parent = start.cast::<IgesBasicSingleParent>();
        let Some(live) = parent.get() else {
            return false;
        };
        if live.single_parent().cast::<IgesGeomPlane>().is_null() {
            return false;
        }
        let count = live.nb_children();
        (1..=count).all(|rank| {
            live.child(rank)
                .is_ok_and(|child| child.cast::<IgesGeomPlane>().is_some())
        })
    }
    /// Whether an entity is a surface (a basic one, or types 108, 118, 120, 122, 140, 143, 144, or a
    /// perforated plane); `IgesToBRep::is_topo_surface(&entity)`.
    pub fn is_topo_surface(start: &Handle<IgesDataIgesEntity>) -> bool {
        Self::is_basic_surface(start)
            || is_any_of(
                start,
                [
                    IgesGeomTrimmedSurface::static_type().name(),
                    IgesGeomSurfaceOfRevolution::static_type().name(),
                    IgesGeomTabulatedCylinder::static_type().name(),
                    IgesGeomRuledSurface::static_type().name(),
                    IgesGeomPlane::static_type().name(),
                    IgesGeomBoundedSurface::static_type().name(),
                    IgesGeomOffsetSurface::static_type().name(),
                ],
            )
            || Self::is_perforated_plane(start)
    }
    /// Whether an entity is a BRep entity (types 186, 502, 504, 508, 510, 514);
    /// `IgesToBRep::is_b_rep_entity(&entity)`.
    pub fn is_b_rep_entity(start: &Handle<IgesDataIgesEntity>) -> bool {
        is_any_of(
            start,
            [
                IgesSolidFace::static_type().name(),
                IgesSolidShell::static_type().name(),
                IgesSolidManifoldSolid::static_type().name(),
                IgesSolidVertexList::static_type().name(),
                IgesSolidEdgeList::static_type().name(),
                IgesSolidLoop::static_type().name(),
            ],
        )
    }
    /// Adds the curves of an entity to a list, a composite by its parts and without points; gives the
    /// length of the list; `IgesToBRep::iges_curve_to_sequence_of_iges_curve(&curve, &mut list)`.
    pub fn iges_curve_to_sequence_of_iges_curve(
        curve: &Handle<IgesDataIgesEntity>,
        sequence: &mut Vec<Handle<StandardTransient>>,
    ) -> i32 {
        let composite = curve.cast::<crate::iges_geom::IgesGeomCompositeCurve>();
        if let Some(live) = composite.get() {
            for rank in 1..=live.nb_curves() {
                if let Ok(part) = live.curve(rank) {
                    let _ = Self::iges_curve_to_sequence_of_iges_curve(&part, sequence);
                }
            }
        } else if curve.is_some()
            && Self::is_topo_curve(curve)
            && curve.cast::<IgesGeomPoint>().is_null()
        {
            sequence.push(curve.cast());
        }
        sequence.len() as i32
    }
    /// Copies the pcurve of an edge on a face to another edge, a seam when it has one already; gives
    /// false when the ranges cannot be made the same;
    /// `IgesToBRep::transfer_pcurve(&from, &to, &face)?`.
    ///
    /// # Errors
    /// A failure of the curves of the edges.
    pub fn transfer_pcurve(from: &Edge, to: &Edge, face: &Face) -> Result<bool, Failure> {
        let old = BRepTool::curve_on_face(to, face)?;
        let new = BRepTool::curve_on_face(from, face)?;
        let builder = BRepBuilder::default();
        let live = iges_live(&new.pcurve, WHERE)?;
        let copy: Handle<dyn Geom2dCurve> = live.copy()?.cast();
        let mut result = true;
        if old.pcurve.is_some() {
            let mut kept = old.pcurve.clone();
            if old.first != new.first || old.last != new.last {
                let mut same: Handle<dyn Geom2dCurve> = Handle::default();
                let made = geom_lib::same_range(
                    precision::p_confusion(),
                    &old.pcurve,
                    (old.first, old.last),
                    (new.first, new.last),
                    &mut same,
                );
                if made.is_ok() && same.is_some() {
                    kept = same;
                } else {
                    result = false;
                }
            }
            let kept: Handle<dyn Geom2dCurve> = iges_live(&kept, WHERE)?.copy()?.cast();
            if to.orientation() == Orientation::Forward {
                builder.update_edge_pcurves_on_face(to, (&copy, &kept), face, 0.0)?;
            } else {
                builder.update_edge_pcurves_on_face(to, (&kept, &copy), face, 0.0)?;
            }
        } else {
            builder.update_edge_pcurve_on_face(to, &copy, face, 0.0)?;
        }
        builder.range_on_face(to, face, new.first, new.last)?;
        let (curve, first, last) = BRepTool::curve(to)?;
        let same = curve.is_null() || (first == new.first && last == new.last);
        builder.same_range(to, same)?;
        Ok(result)
    }
}

/// The name of the class of an entity, for the messages of the translators.
pub(super) fn entity_type_name<T: ?Sized + RuntimeTyped>(entity: &Handle<T>) -> &'static str {
    entity.dynamic_type().map_or("", |kind| kind.name())
}
