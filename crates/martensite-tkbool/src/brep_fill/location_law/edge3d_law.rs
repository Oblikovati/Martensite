//! `BRepFillEdge3DLaw`: one copy of a GeomFill location law on the 3d curve of each
//! edge of the path; `BRepFillDraftLaw`: the same with draft laws, and their cleaning.
use super::continuity::{junction, to_g0};
use super::{BRepFillLocationLaw, BRepFillLocationLawBase, oriented_edge_curve};
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::brep_tools::WireExplorer;
use martensite_tkbrep::topods::Wire;
use martensite_tkernel::impl_transient;
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkg3d::adaptor3d::Adaptor3dCurve;
use martensite_tkg3d::geom_adaptor::GeomAdaptorCurve;
use martensite_tkgeomalgo::geom_fill::{GeomFillLocationDraft, GeomFillLocationLaw, LocationD0};
use martensite_tkmath::gp::Vector3;

/// The base of a law that places a copy of `law` on the 3d curve of each edge.
pub(super) fn edge3d_base(
    path: &Wire,
    law: &Handle<dyn GeomFillLocationLaw>,
) -> Result<BRepFillLocationLawBase, Failure> {
    let mut base = BRepFillLocationLawBase::new(path)?;
    let Some(model) = law.get() else {
        return Err(Failure::with_type(
            "Standard_NullObject",
            "BRepFillEdge3DLaw: null location law; expected a law",
            "BRepFillEdge3DLaw",
        ));
    };
    let mut ipath: usize = 0;
    let mut explorer = WireExplorer::with_wire(&base.path)?;
    while explorer.more() {
        let edge = explorer.current().clone();
        explorer.next()?;
        if BRepTool::degenerated(&edge)? {
            continue;
        }
        ipath += 1;
        base.edges[ipath - 1] = edge.as_ref().clone();
        let (curve, first, last) = oriented_edge_curve(&edge)?;
        let ac: Handle<dyn Adaptor3dCurve> =
            Handle::new(GeomAdaptorCurve::with_range(&curve, first, last)?).cast();
        let copy = model.copy()?;
        if let Some(laws) = base.laws.as_mut() {
            laws[ipath - 1] = copy.clone();
        }
        if let Some(copy) = copy.get() {
            copy.set_curve(&ac)?;
        }
    }
    Ok(base)
}

/// The location law of a path along the 3d curves of its edges (`BRepFillEdge3DLaw`);
/// `let law = BRepFillEdge3DLaw::new(&path, &frenet_law)?;`.
pub struct BRepFillEdge3DLaw {
    parent: StandardTransient,
    base: BRepFillLocationLawBase,
}
impl_transient!(
    BRepFillEdge3DLaw,
    "BRepFillEdge3DLaw",
    dyn BRepFillLocationLaw,
    parent,
    interfaces: [dyn BRepFillLocationLaw]
);

impl BRepFillEdge3DLaw {
    /// `BRepFillEdge3DLaw(Path, Law)`.
    pub fn new(path: &Wire, law: &Handle<dyn GeomFillLocationLaw>) -> Result<Self, Failure> {
        Ok(Self {
            parent: StandardTransient::default(),
            base: edge3d_base(path, law)?,
        })
    }
}

impl BRepFillLocationLaw for BRepFillEdge3DLaw {
    fn location_law_base(&self) -> &BRepFillLocationLawBase {
        &self.base
    }
}

/// The draft location law of a path (`BRepFillDraftLaw`, an Edge3DLaw of draft laws);
/// `let law = BRepFillDraftLaw::new(&path, &draft)?;`.
pub struct BRepFillDraftLaw {
    // BRepFillDraftLaw requires an actual Edge3DLaw parent for ancestry and projection.
    parent: BRepFillEdge3DLaw,
}
impl_transient!(
    BRepFillDraftLaw,
    "BRepFillDraftLaw",
    BRepFillEdge3DLaw,
    parent,
    interfaces: [dyn BRepFillLocationLaw]
);

impl BRepFillDraftLaw {
    /// `BRepFillDraftLaw(Path, Law)`.
    pub fn new(path: &Wire, law: &Handle<GeomFillLocationDraft>) -> Result<Self, Failure> {
        let law: Handle<dyn GeomFillLocationLaw> = law.clone().cast();
        Ok(Self {
            parent: BRepFillEdge3DLaw::new(path, &law)?,
        })
    }

    /// `CleanLaw(TolAngular)`: the G0 placement where the normals of consecutive laws
    /// are parallel; `law.clean_law(1e-2)?`.
    pub fn clean_law(&self, tol_angular: f64) -> Result<(), Failure> {
        let base = self.parent.location_law_base();
        let (_, mut last) = base.law_at(1)?.get_domain()?;
        let (mut m1, mut m2) = (LocationD0::default(), LocationD0::default());
        for ipath in 2..=base.count()? {
            junction(base, ipath - 1, ipath, &mut last, &mut m1, &mut m2)?;
            let n1 = Vector3::from(m1.m.column(1)?);
            let n2 = Vector3::from(m2.m.column(1)?);
            if n1.is_parallel(n2, tol_angular)? {
                // Correction G0 of the normals.
                base.law_at(ipath)?.set_trsf(&to_g0(&m1.m, &m2.m)?)?;
            }
        }
        Ok(())
    }
}

impl BRepFillLocationLaw for BRepFillDraftLaw {
    fn location_law_base(&self) -> &BRepFillLocationLawBase {
        self.parent.location_law_base()
    }
}
