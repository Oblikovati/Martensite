//! `BRepAdaptor_Surface`: a face seen as a surface.
use super::forward::inherited_surface_members;
use crate::brep::BRepTool;
use crate::brep_tools;
use crate::topods::Face;
use martensite_tkernel::impl_transient;
use martensite_tkernel::ncollection::Array1;
use martensite_tkernel::standard::{Failure, Handle, SharedState};
use martensite_tkg3d::adaptor3d::{Adaptor3dCurve, Adaptor3dSurface};
use martensite_tkg3d::geom::{
    GeomBSplineSurface, GeomBezierSurface, GeomSurfaceD1, GeomSurfaceD2, GeomSurfaceD3,
};
use martensite_tkg3d::geom_adaptor::GeomAdaptorTransformedSurface;
use martensite_tkmath::geom_abs::{Shape, SurfaceType};
use martensite_tkmath::gp::{
    Axis1, Cone3, Cylinder3, Direction3, Plane3, Point3, Sphere3, Torus3, Vector3,
};

/// A face seen as a surface (`BRepAdaptor_Surface`): the surface of the face,
/// restricted to the UV bounds of the face or not, transformed by the face location.
///
/// The native class derives from `GeomAdaptor_TransformedSurface`; the Rust type embeds
/// it as its parent field and forwards every inherited member.
///
/// ```
/// use martensite_tkbrep::brep_adaptor::BRepAdaptorSurface;
/// let adaptor = BRepAdaptorSurface::default();
/// assert!(adaptor.face().is_null());
/// ```
#[derive(Default)]
pub struct BRepAdaptorSurface {
    parent: GeomAdaptorTransformedSurface,
    face: SharedState<Face>,
}
impl Clone for BRepAdaptorSurface {
    /// Copies face/adaptor values while retaining nested owners; `let copied = source.clone();`.
    fn clone(&self) -> Self {
        Self {
            parent: self.parent.clone(),
            face: SharedState::new(self.face()),
        }
    }
}

impl_transient!(
    BRepAdaptorSurface,
    "BRepAdaptor_Surface",
    GeomAdaptorTransformedSurface,
    parent,
    interfaces: [dyn Adaptor3dSurface]
);

impl BRepAdaptorSurface {
    /// `BRepAdaptor_Surface(F, R)`: `restriction` limits the surface to the UV bounds of
    /// the face; `BRepAdaptorSurface::with_face(&face, true)?`.
    pub fn with_face(face: &Face, restriction: bool) -> Result<Self, Failure> {
        let adaptor = Self::default();
        adaptor.initialize(face, restriction)?;
        Ok(adaptor)
    }

    /// The embedded parent adaptor (the `GeomAdaptor_TransformedSurface` base).
    pub fn transformed_surface(&self) -> &GeomAdaptorTransformedSurface {
        &self.parent
    }

    /// `Initialize(F, Restriction)`: a null face is ignored; otherwise the face is
    /// stored and, when it has a surface, the surface is loaded (on the face's UV bounds
    /// with `restriction`) with the face location; `adaptor.initialize(&face, true)?`.
    pub fn initialize(&self, face: &Face, restriction: bool) -> Result<(), Failure> {
        if face.is_null() {
            return Ok(());
        }
        *self.face.write() = face.clone();
        let (surface, location) = BRepTool::surface_with_location(face)?;
        if surface.is_null() {
            return Ok(());
        }
        let trsf = location.transformation();
        if !restriction {
            return self.parent.load(&surface, trsf);
        }
        let (umin, umax, vmin, vmax) = brep_tools::uv_bounds(face)?;
        self.parent
            .load_range(&surface, ((umin, umax, vmin, vmax), (0.0, 0.0)), trsf)
    }

    /// `Face()`; `adaptor.face()`.
    pub fn face(&self) -> Face {
        self.face.read().clone()
    }

    /// `Tolerance()`: the tolerance of the face; `adaptor.tolerance()?`.
    pub fn tolerance(&self) -> Result<f64, Failure> {
        BRepTool::face_tolerance(&self.face())
    }
}

impl Adaptor3dSurface for BRepAdaptorSurface {
    /// `ShallowCopy()`: a shallow copy of the original adaptor, the transformation, the
    /// shared transformed adaptor and the face.
    fn shallow_copy(&self) -> Result<Handle<dyn Adaptor3dSurface>, Failure> {
        let copy = Self {
            parent: self.parent.shallow_adaptor()?,
            face: SharedState::new(self.face()),
        };
        Ok(Handle::new(copy).cast())
    }

    inherited_surface_members!();
}
