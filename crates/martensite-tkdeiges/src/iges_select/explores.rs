//! The selections that go down the structure of entities to the geometry they hold.
use crate::iges_basic::{IgesBasicGroup, IgesBasicSingleParent};
use crate::iges_data::{IgesDataIgesEntity, IgesDataSingleParentEntity};
use crate::iges_geom::{
    IgesGeomBoundary, IgesGeomBoundedSurface, IgesGeomCompositeCurve, IgesGeomCurveOnSurface,
    IgesGeomPlane, IgesGeomTrimmedSurface,
};
use crate::iges_solid::{
    IgesSolidEdgeList, IgesSolidFace, IgesSolidLoop, IgesSolidManifoldSolid, IgesSolidShell,
};
use martensite_tkernel::{
    impl_transient,
    standard::{Failure, Handle, RuntimeTyped, StandardTransient},
    tcollection::AsciiString,
};
use martensite_tkxsbase::{
    if_select::{
        IFSelectSelectDeduct, IFSelectSelectDeductBase, IFSelectSelectExplore,
        IFSelectSelectExploreBase, IFSelectSelection, IFSelectSelectionIterator,
        deduct_fill_iterator, explore_label, explore_root_result,
    },
    interface::{InterfaceEntityIterator, InterfaceGraph},
};

/// Declares a selection that explores: its fields, its runtime type and its general members.
macro_rules! iges_explore {
    ($(#[$meta:meta])* $class:ident, $recorded:literal, { $($field:ident: $ty:ty),* $(,)? }) => {
        $(#[$meta])*
        pub struct $class {
            transient: StandardTransient,
            base: IFSelectSelectExploreBase,
            $($field: $ty,)*
        }
        impl_transient!(
            $class,
            $recorded,
            dyn IFSelectSelectExplore,
            transient,
            interfaces: [dyn IFSelectSelection, dyn IFSelectSelectDeduct, dyn IFSelectSelectExplore]
        );
        impl IFSelectSelection for $class {
            fn root_result(&self, g: &InterfaceGraph) -> Result<InterfaceEntityIterator, Failure> {
                explore_root_result(self, g)
            }
            fn fill_iterator(&self, iter: &mut IFSelectSelectionIterator) {
                deduct_fill_iterator(self, iter)
            }
            fn label(&self) -> AsciiString {
                explore_label(self)
            }
        }
        impl IFSelectSelectDeduct for $class {
            fn deduct_base(&self) -> &IFSelectSelectDeductBase {
                &self.base.deduct
            }
        }
    };
}
pub(super) use iges_explore;

/// Adds an entity of any kind to what is explored; a null one is ignored by the iterator.
pub(super) fn add<T: ?Sized + RuntimeTyped>(
    explored: &mut InterfaceEntityIterator,
    ent: &Handle<T>,
) {
    explored.add_item(&ent.cast());
}

/// Adds the items of ranks 1 to `nb` of an accessor.
pub(super) fn add_ranks<T: ?Sized + RuntimeTyped>(
    explored: &mut InterfaceEntityIterator,
    nb: i32,
    mut item: impl FnMut(i32) -> Result<Handle<T>, Failure>,
) -> Result<(), Failure> {
    for rank in 1..=nb {
        add(explored, &item(rank)?);
    }
    Ok(())
}

/// The parent and children of a single parent entity; false when `ent` is not one.
pub(super) fn explore_single_parent(
    ent: &Handle<StandardTransient>,
    explored: &mut InterfaceEntityIterator,
) -> Result<bool, Failure> {
    let parent = ent.cast::<IgesBasicSingleParent>();
    let Some(live) = parent.get() else {
        return Ok(false);
    };
    add(explored, &IgesDataSingleParentEntity::single_parent(live));
    add_ranks(
        explored,
        IgesDataSingleParentEntity::nb_children(live)?,
        |rank| IgesDataSingleParentEntity::child(live, rank),
    )?;
    Ok(true)
}

/// The content of a group; false when `ent` is not one.
pub(super) fn explore_group(
    ent: &Handle<StandardTransient>,
    explored: &mut InterfaceEntityIterator,
) -> Result<bool, Failure> {
    let group = ent.cast::<IgesBasicGroup>();
    let Some(live) = group.get() else {
        return Ok(false);
    };
    add_ranks(explored, live.nb_entities(), |rank| live.entity(rank))?;
    Ok(true)
}

/// The content of the solid structures: manifold solid and shell.
pub(super) fn explore_solid(
    igt: i32,
    ent: &Handle<StandardTransient>,
    explored: &mut InterfaceEntityIterator,
) -> Result<Option<bool>, Failure> {
    if igt == 186 {
        let solid = ent.cast::<IgesSolidManifoldSolid>();
        if let Some(live) = solid.get() {
            add(explored, &live.shell());
            add_ranks(explored, live.nb_void_shells(), |rank| {
                live.void_shell(rank)
            })?;
        }
        return Ok(Some(true));
    }
    if igt == 514 {
        let shell = ent.cast::<IgesSolidShell>();
        if let Some(live) = shell.get() {
            add_ranks(explored, live.nb_faces()?, |rank| live.face(rank))?;
        }
        return Ok(Some(true));
    }
    Ok(None)
}

/// Whether a type and form is a basic curve.
pub(super) fn is_basic_curve(igt: i32, form: i32) -> bool {
    if igt == 106 {
        return form < 20;
    }
    (100..=106).contains(&igt) || matches!(igt, 110 | 112 | 116 | 126 | 130)
}

/// Whether a type is a basic surface.
fn is_basic_surface(igt: i32) -> bool {
    matches!(igt, 114 | 118 | 120 | 122 | 128 | 140 | 190)
}

iges_explore! {
    /// The basic geometry an entity holds: curves for a positive mode, with composite curves
    /// exploded for 2; surfaces for a negative one; both for 0.
    ///
    /// ```
    /// use martensite_tkdeiges::iges_select::IgesSelectSelectBasicGeom;
    /// use martensite_tkxsbase::if_select::IFSelectSelectExplore;
    /// let select = IgesSelectSelectBasicGeom::new(2);
    /// assert!(select.curves_only());
    /// assert_eq!(select.explore_label().as_bytes(), b"Basic Curves 3d");
    /// ```
    IgesSelectSelectBasicGeom, "IgesSelectSelectBasicGeom", { thegeom: i32 }
}

impl IgesSelectSelectBasicGeom {
    /// The selection of a mode.
    pub fn new(mode: i32) -> Self {
        Self {
            transient: StandardTransient::default(),
            base: IFSelectSelectExploreBase::new(-1),
            thegeom: mode,
        }
    }
    /// Whether only curves are selected.
    pub fn curves_only(&self) -> bool {
        self.thegeom > 0
    }
    /// The curves of a composite curve, or whether an entity is a basic curve itself.
    ///
    /// # Errors
    /// A failure of the entity.
    pub fn sub_curves(
        ent: &Handle<IgesDataIgesEntity>,
        explored: &mut InterfaceEntityIterator,
    ) -> Result<bool, Failure> {
        let Some(live) = ent.get() else {
            return Ok(false);
        };
        let igt = live.type_number();
        if igt == 102 {
            let composite = ent.cast::<IgesGeomCompositeCurve>();
            if let Some(curves) = composite.get() {
                add_ranks(explored, curves.nb_curves(), |rank| curves.curve(rank))?;
            }
            return Ok(true);
        }
        Ok(is_basic_curve(igt, live.form_number()))
    }
    fn explore_surfaces(
        &self,
        igt: i32,
        ent: &Handle<StandardTransient>,
        explored: &mut InterfaceEntityIterator,
    ) -> Result<Option<bool>, Failure> {
        match igt {
            108 => {
                let plane = ent.cast::<IgesGeomPlane>();
                let Some(live) = plane.get() else {
                    return Ok(Some(false));
                };
                if self.thegeom >= 0 {
                    add(explored, &live.bounding_curve());
                }
                Ok(Some(live.has_bounding_curve()))
            }
            144 => {
                let trimmed = ent.cast::<IgesGeomTrimmedSurface>();
                if let Some(live) = trimmed.get() {
                    if self.thegeom >= 0 {
                        add(explored, &live.outer_contour());
                        add_ranks(explored, live.nb_inner_contours(), |rank| {
                            live.inner_contour(rank)
                        })?;
                    } else {
                        add(explored, &live.surface());
                    }
                }
                Ok(Some(true))
            }
            143 => {
                let bounded = ent.cast::<IgesGeomBoundedSurface>();
                let Some(live) = bounded.get() else {
                    return Ok(Some(true));
                };
                if self.thegeom >= 0 {
                    let nb = live.nb_boundaries();
                    add_ranks(explored, nb, |rank| live.boundary(rank))?;
                    return Ok(Some(nb != 0));
                }
                add(explored, &live.surface());
                Ok(Some(true))
            }
            510 => {
                let face = ent.cast::<IgesSolidFace>();
                if let Some(live) = face.get() {
                    if self.thegeom >= 0 {
                        add_ranks(explored, live.nb_loops()?, |rank| live.loop_(rank))?;
                    } else {
                        add(explored, &live.surface());
                    }
                }
                Ok(Some(true))
            }
            _ => Ok(None),
        }
    }
    fn explore_curves(
        &self,
        igt: i32,
        ent: &Handle<StandardTransient>,
        explored: &mut InterfaceEntityIterator,
    ) -> Result<Option<bool>, Failure> {
        if self.thegeom < 0 {
            return Ok(None);
        }
        match igt {
            142 => {
                let on_surface = ent.cast::<IgesGeomCurveOnSurface>();
                if let Some(live) = on_surface.get() {
                    add(explored, &live.curve3d());
                }
                Ok(Some(true))
            }
            141 => {
                let boundary = ent.cast::<IgesGeomBoundary>();
                let Some(live) = boundary.get() else {
                    return Ok(Some(false));
                };
                let nb = live.nb_model_space_curves()?;
                add_ranks(explored, nb, |rank| live.model_space_curve(rank))?;
                Ok(Some(nb > 0))
            }
            508 => {
                let edges = ent.cast::<IgesSolidLoop>();
                let Some(live) = edges.get() else {
                    return Ok(Some(true));
                };
                for rank in 1..=live.nb_edges() {
                    let list = live.edge(rank)?.cast::<IgesSolidEdgeList>();
                    if let Some(edge_list) = list.get() {
                        add(explored, &edge_list.curve(live.list_index(rank)?)?);
                    }
                }
                Ok(Some(true))
            }
            _ => Ok(None),
        }
    }
}

impl IFSelectSelectExplore for IgesSelectSelectBasicGeom {
    fn explore_base(&self) -> &IFSelectSelectExploreBase {
        &self.base
    }
    fn explore(
        &self,
        _level: i32,
        ent: &Handle<StandardTransient>,
        _g: &InterfaceGraph,
        explored: &mut InterfaceEntityIterator,
    ) -> Result<bool, Failure> {
        let entity = ent.cast::<IgesDataIgesEntity>();
        let Some(live) = entity.get() else {
            return Ok(false);
        };
        let igt = live.type_number();
        let form = live.form_number();
        if igt == 102 && self.thegeom == 2 {
            return Self::sub_curves(&entity, explored);
        }
        if igt == 106 {
            return Ok(form < 20);
        }
        if is_basic_curve(igt, form) {
            return Ok(self.thegeom >= 0);
        }
        if is_basic_surface(igt) {
            return Ok(self.thegeom <= 0);
        }
        if let Some(result) = self.explore_surfaces(igt, ent, explored)? {
            return Ok(result);
        }
        if let Some(result) = self.explore_curves(igt, ent, explored)? {
            return Ok(result);
        }
        if igt == 402 && form == 9 {
            return explore_single_parent(ent, explored);
        }
        if igt == 402 {
            return explore_group(ent, explored);
        }
        Ok(explore_solid(igt, ent, explored)?.unwrap_or(false))
    }
    fn explore_label(&self) -> AsciiString {
        AsciiString::new(match self.thegeom {
            2 => "Basic Curves 3d",
            mode if mode > 0 => "Curves 3d",
            mode if mode < 0 => "Surfaces",
            _ => "Basic Geometry",
        })
    }
}
