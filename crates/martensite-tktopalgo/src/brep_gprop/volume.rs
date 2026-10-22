//! The volume properties of `BRepGProp` by Gauss integration of the faces.
use super::domain::BRepGPropDomain;
use super::face::BRepGPropFace;
use super::mesh_props::{BRepGPropMeshProps, MeshObjType};
use super::package::{BRepGProp, FaceSources, located_origin};
use super::vinert::BRepGPropVinert;
use super::volume_gk::VolumeGKFlags;
use super::volume_solids::volume_props;
use crate::brep_check::explore::sub_shapes;
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::topods::{self, Face, Shape};
use martensite_tkbrep::toptools::MapOfShape;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum};
use martensite_tkgeombase::gprop::GProps;
use martensite_tkmath::gp::Point3;

/// The largest relative quadrature error accepted by the fixed-properties route.
///
/// A trimmed boundary can make a single Gauss pass under-resolve an otherwise smooth
/// surface.  This limit preserves the inexpensive fixed route where it is adequate and
/// switches to the existing adaptive integrator only when its independent two-order
/// estimate says it is not.
const FIXED_DOMAIN_ERROR_LIMIT: f64 = 1.0e-5;

/// The flags of the volume computations (`SkipShared`, `UseTriangulation`).
#[derive(Clone, Copy, Debug)]
pub(super) struct VolumeFlags {
    pub(super) skip_shared: bool,
    pub(super) use_triangulation: bool,
}

impl BRepGProp {
    /// `VolumeProperties(S, VProps, OnlyClosed, SkipShared, UseTriangulation)`: `props`
    /// becomes a framework at the located origin holding the volume of `shape` (of each
    /// closed shell with `only_closed`); error-bounded Gauss-Kronrod integration, meshes
    /// when asked or when a face has no surface; `BRepGProp::volume_properties(&shape, &mut props, false, false, false)?`.
    pub fn volume_properties(
        shape: &Shape,
        props: &mut GProps,
        only_closed: bool,
        skip_shared: bool,
        use_triangulation: bool,
    ) -> Result<(), Failure> {
        if use_triangulation {
            let flags = VolumeFlags {
                skip_shared,
                use_triangulation,
            };
            volume_by_shells(shape, props, 1.0, only_closed, flags)?;
            return Ok(());
        }
        let flags = VolumeGKFlags {
            cg_flag: true,
            i_flag: true,
            skip_shared,
            ..VolumeGKFlags::default()
        };
        Self::volume_properties_gk(shape, props, FIXED_DOMAIN_ERROR_LIMIT, only_closed, flags)?;
        Ok(())
    }

    /// `VolumeProperties(S, VProps, Eps, OnlyClosed, SkipShared)`: the adaptive
    /// integration of each face with the relative error `eps` (non-adaptive above
    /// 0.001); the largest error reached;
    /// `BRepGProp::volume_properties_eps(&shape, &mut props, 1e-6, false, false)?`.
    pub fn volume_properties_eps(
        shape: &Shape,
        props: &mut GProps,
        eps: f64,
        only_closed: bool,
        skip_shared: bool,
    ) -> Result<f64, Failure> {
        let flags = VolumeFlags {
            skip_shared,
            use_triangulation: false,
        };
        volume_by_shells(shape, props, eps, only_closed, flags)
    }
}

/// Both VolumeProperties: the framework at the located origin, then the shape, or each
/// closed shell (each shared shell once with `skip_shared`); the largest error.
fn volume_by_shells(
    shape: &Shape,
    props: &mut GProps,
    eps: f64,
    only_closed: bool,
    flags: VolumeFlags,
) -> Result<f64, Failure> {
    *props = GProps::with_location(located_origin(shape));
    if !only_closed {
        return volume_props(shape, props, eps, flags);
    }
    let mut shells = MapOfShape::default();
    let mut error_max = 0.0;
    for shell in sub_shapes(shape, ShapeEnum::Shell)? {
        if flags.skip_shared && !shells.add(shell.clone()) {
            continue;
        }
        if BRepTool::is_closed(&shell)? {
            let error = volume_props(&shell, props, eps, flags)?;
            if error_max < error {
                error_max = error;
            }
        }
    }
    Ok(error_max)
}

/// The FORWARD and REVERSED faces seen so far (`aFwdFMap`, `aRvsFMap`).
#[derive(Default)]
pub(super) struct OrientedFaces {
    forward: MapOfShape,
    reversed: MapOfShape,
}

impl OrientedFaces {
    /// Whether `face` was already met with its orientation (FORWARD or REVERSED), recording
    /// it; INTERNAL and EXTERNAL faces are never recorded.
    pub(super) fn seen(&mut self, face: &Face) -> bool {
        match face.orientation() {
            Orientation::Forward => !self.forward.add(face.clone().into_shape()),
            Orientation::Reversed => !self.reversed.add(face.clone().into_shape()),
            _ => false,
        }
    }
}

/// Whether a face counts for a volume: FORWARD or REVERSED.
pub(super) fn is_oriented(face: &Face) -> bool {
    matches!(
        face.orientation(),
        Orientation::Forward | Orientation::Reversed
    )
}

/// `volumePropertiesFaces`: the volume of the FORWARD and REVERSED faces of `shape` about
/// `centre`, one Vinert and one mesh framework reused for every face; the largest error.
pub(super) fn volume_faces(
    shape: &Shape,
    props: &mut GProps,
    centre: Point3,
    eps: f64,
    flags: VolumeFlags,
) -> Result<f64, Failure> {
    let mut volume = BRepGPropVinert::new();
    volume.set_location(centre);
    let mut mesh = BRepGPropMeshProps::new(MeshObjType::Vinert);
    mesh.set_location(centre);
    let (mut tool, mut domain) = (BRepGPropFace::new(false), BRepGPropDomain::new());
    let mut seen = OrientedFaces::default();
    let mut error_max = 0.0;
    for shape in sub_shapes(shape, ShapeEnum::Face)? {
        let face = topods::face(&shape)?;
        if flags.skip_shared && seen.seen(&face) {
            continue;
        }
        let sources = FaceSources::of(&face)?;
        if (sources.no_mesh && sources.no_surface) || !is_oriented(&face) {
            continue;
        }
        if sources.use_mesh(flags.use_triangulation) {
            mesh.perform_located(&sources.mesh, &sources.location, face.orientation())?;
            props.add(&mesh, 1.0)?;
            continue;
        }
        let error = volume_face(&face, (&mut tool, &mut domain), &mut volume, eps)?;
        if error_max < error {
            error_max = error;
        }
        props.add(&volume, 1.0)?;
    }
    Ok(error_max)
}

/// The Vinert integration of one face: adaptive below `eps` 1 (its error), else on the
/// natural bounds or with an error-bounded domain pass.
fn volume_face(
    face: &Face,
    (tool, domain): (&mut BRepGPropFace, &mut BRepGPropDomain),
    volume: &mut BRepGPropVinert,
    eps: f64,
) -> Result<f64, Failure> {
    tool.load_face(face)?;
    let is_natural = face.nb_children() == 0;
    if !is_natural {
        domain.init_face(face)?;
    }
    if eps < 1.0 {
        volume.perform_domain_eps(tool, domain, eps)?;
        return Ok(volume.get_epsilon());
    }
    if is_natural {
        volume.perform(tool)?;
        return Ok(0.0);
    }
    volume.perform_domain(tool, domain)?;
    Ok(0.0)
}
