//! The StdSelect owner filters (`StdSelectEdgeFilter`, `StdSelectFaceFilter`,
//! `StdSelectShapeTypeFilter`) and their type enumerations, which accept a
//! `StdSelectBRepOwner` by the type of its shape or of the underlying geometry.
use super::{StdSelectBRepOwnerMembers, StdSelectBRepOwnerSource};
use crate::prs3d::Prs3dDrawer;
use crate::select_mgr::{SelectMgrEntityOwnerSource, SelectMgrFilter, SelectMgrSelection};
use martensite_tkbrep::brep_adaptor::{BRepAdaptorCurve, BRepAdaptorSurface};
use martensite_tkbrep::topods::{Edge, Face, Shape};
use martensite_tkernel::standard::{Failure, Handle, SharedState, StandardTransient};
use martensite_tkg3d::adaptor3d::{Adaptor3dCurve, Adaptor3dSurface};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::geom_abs::{CurveType, SurfaceType};

/// `StdSelectTypeOfEdge`: the edges an edge filter accepts.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum StdSelectTypeOfEdge {
    /// `StdSelectAnyEdge`.
    #[default]
    AnyEdge,
    /// `StdSelectLine`.
    Line,
    /// `StdSelectCircle`.
    Circle,
}

/// `StdSelectTypeOfFace`: the faces a face filter accepts.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum StdSelectTypeOfFace {
    /// `StdSelectAnyFace`.
    #[default]
    AnyFace,
    /// `StdSelectPlane`.
    Plane,
    /// `StdSelectCylinder`.
    Cylinder,
    /// `StdSelectSphere`.
    Sphere,
    /// `StdSelectTorus`.
    Torus,
    /// `StdSelectRevol`: a cylinder, cone, torus, sphere or surface of revolution.
    Revol,
    /// `StdSelectCone`.
    Cone,
}

/// The shape of a BRep owner, None for another owner (the native down cast).
fn brep_owner_shape(owner: &Handle<dyn SelectMgrEntityOwnerSource>) -> Option<Shape> {
    let brep = owner.cast::<dyn StdSelectBRepOwnerSource>();
    brep.get().map(|owner| owner.shape_ref().clone_shape())
}

/// `StdSelectEdgeFilter`: accepts the edges of BRep owners, of any type, lines or circles.
/// Example: `StdSelectEdgeFilter::new(StdSelectTypeOfEdge::Line)`.
pub struct StdSelectEdgeFilter {
    transient: StandardTransient,
    my_type: SharedState<StdSelectTypeOfEdge>,
}
martensite_tkernel::impl_transient!(StdSelectEdgeFilter, "StdSelectEdgeFilter", dyn SelectMgrFilter, transient,
    interfaces: [dyn SelectMgrFilter]);

impl StdSelectEdgeFilter {
    /// The filter of an edge type; `StdSelectEdgeFilter::new(StdSelectTypeOfEdge::Circle)`.
    pub fn new(edge: StdSelectTypeOfEdge) -> Self {
        Self {
            transient: StandardTransient::default(),
            my_type: SharedState::new(edge),
        }
    }
    /// `SetType`; `filter.set_type(StdSelectTypeOfEdge::AnyEdge)`.
    pub fn set_type(&self, new_type: StdSelectTypeOfEdge) {
        *self.my_type.write() = new_type;
    }
    /// `Type`; `filter.type_()`.
    pub fn type_(&self) -> StdSelectTypeOfEdge {
        *self.my_type.read()
    }
}

impl SelectMgrFilter for StdSelectEdgeFilter {
    /// `IsOk`: a BRep owner of an edge of the type; a null shape fails with
    /// `Standard_NullObject` (a null dereference natively), as does an edge without curve.
    fn is_ok(&self, owner: &Handle<dyn SelectMgrEntityOwnerSource>) -> Result<bool, Failure> {
        let Some(shape) = brep_owner_shape(owner) else {
            return Ok(false);
        };
        if shape.shape_type()? != ShapeEnum::Edge {
            return Ok(false);
        }
        let wanted = match self.type_() {
            StdSelectTypeOfEdge::AnyEdge => return Ok(true),
            StdSelectTypeOfEdge::Line => CurveType::Line,
            StdSelectTypeOfEdge::Circle => CurveType::Circle,
        };
        Edge::with_shape_reference(&shape, |edge| {
            Ok(BRepAdaptorCurve::with_edge(edge)?.get_type()? == wanted)
        })
    }
    /// `ActsOn`: the edge mode only.
    fn acts_on(&self, standard_mode: ShapeEnum) -> bool {
        standard_mode == ShapeEnum::Edge
    }
}

/// `StdSelectFaceFilter`: accepts the faces of BRep owners by surface type.
/// Example: `StdSelectFaceFilter::new(StdSelectTypeOfFace::Plane)`.
pub struct StdSelectFaceFilter {
    transient: StandardTransient,
    my_type: SharedState<StdSelectTypeOfFace>,
}
martensite_tkernel::impl_transient!(StdSelectFaceFilter, "StdSelectFaceFilter", dyn SelectMgrFilter, transient,
    interfaces: [dyn SelectMgrFilter]);

impl StdSelectFaceFilter {
    /// The filter of a face type; `StdSelectFaceFilter::new(StdSelectTypeOfFace::Revol)`.
    pub fn new(type_of_face: StdSelectTypeOfFace) -> Self {
        Self {
            transient: StandardTransient::default(),
            my_type: SharedState::new(type_of_face),
        }
    }
    /// `SetType`; `filter.set_type(StdSelectTypeOfFace::Cone)`.
    pub fn set_type(&self, type_of_face: StdSelectTypeOfFace) {
        *self.my_type.write() = type_of_face;
    }
    /// `Type`; `filter.type_()`.
    pub fn type_(&self) -> StdSelectTypeOfFace {
        *self.my_type.read()
    }
}

impl SelectMgrFilter for StdSelectFaceFilter {
    /// `IsOk`: a BRep owner of a face of the type; a null shape fails with
    /// `Standard_NullObject` (a null dereference natively), as does a face without surface.
    fn is_ok(&self, owner: &Handle<dyn SelectMgrEntityOwnerSource>) -> Result<bool, Failure> {
        let Some(shape) = brep_owner_shape(owner) else {
            return Ok(false);
        };
        if shape.shape_type()? != ShapeEnum::Face {
            return Ok(false);
        }
        let kind = self.type_();
        if kind == StdSelectTypeOfFace::AnyFace {
            return Ok(true);
        }
        let surface = Face::with_shape_reference(&shape, |face| {
            BRepAdaptorSurface::with_face(face, true)?.get_type()
        })?;
        Ok(match kind {
            StdSelectTypeOfFace::AnyFace => true,
            StdSelectTypeOfFace::Plane => surface == SurfaceType::Plane,
            StdSelectTypeOfFace::Cylinder => surface == SurfaceType::Cylinder,
            StdSelectTypeOfFace::Sphere => surface == SurfaceType::Sphere,
            StdSelectTypeOfFace::Torus => surface == SurfaceType::Torus,
            StdSelectTypeOfFace::Revol => matches!(
                surface,
                SurfaceType::Cylinder
                    | SurfaceType::Cone
                    | SurfaceType::Torus
                    | SurfaceType::Sphere
                    | SurfaceType::SurfaceOfRevolution
            ),
            StdSelectTypeOfFace::Cone => surface == SurfaceType::Cone,
        })
    }
    /// `ActsOn`: the face mode only.
    fn acts_on(&self, standard_mode: ShapeEnum) -> bool {
        standard_mode == ShapeEnum::Face
    }
}

/// `StdSelectShapeTypeFilter`: accepts the BRep owners of a shape type.
/// Example: `StdSelectShapeTypeFilter::new(ShapeEnum::Vertex)`.
pub struct StdSelectShapeTypeFilter {
    transient: StandardTransient,
    my_type: ShapeEnum,
}
martensite_tkernel::impl_transient!(StdSelectShapeTypeFilter, "StdSelectShapeTypeFilter", dyn SelectMgrFilter, transient,
    interfaces: [dyn SelectMgrFilter]);

impl StdSelectShapeTypeFilter {
    /// The filter of a shape type; `StdSelectShapeTypeFilter::new(ShapeEnum::Edge)`.
    pub fn new(shape_type: ShapeEnum) -> Self {
        Self {
            transient: StandardTransient::default(),
            my_type: shape_type,
        }
    }
    /// `Type`; `filter.type_()`.
    pub fn type_(&self) -> ShapeEnum {
        self.my_type
    }
}

impl SelectMgrFilter for StdSelectShapeTypeFilter {
    /// `IsOk`: a BRep owner with a shape of the type.
    fn is_ok(&self, owner: &Handle<dyn SelectMgrEntityOwnerSource>) -> Result<bool, Failure> {
        match brep_owner_shape(owner) {
            Some(shape) if !shape.is_null() => Ok(shape.shape_type()? == self.my_type),
            _ => Ok(false),
        }
    }
    /// `ActsOn`: the mode of the type only.
    fn acts_on(&self, standard_mode: ShapeEnum) -> bool {
        standard_mode == self.my_type
    }
}

/// `StdSelect::SetDrawerForBRepOwner`: does nothing (the native body is commented out).
/// Example: `std_select_set_drawer_for_brep_owner(&selection, &drawer)`.
pub fn std_select_set_drawer_for_brep_owner(
    _selection: &Handle<SelectMgrSelection>,
    _drawer: &Handle<Prs3dDrawer>,
) {
}
