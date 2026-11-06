//! The selections of faces and parametric curves, and those that go through groups and subfigures.
use super::IgesSelectSelectBasicGeom;
use super::explores::{
    add, add_ranks, explore_group, explore_single_parent, explore_solid, iges_explore,
    is_basic_curve,
};
use crate::iges_basic::{IgesBasicSingularSubfigure, IgesBasicSubfigureDef};
use crate::iges_data::IgesDataIgesEntity;
use crate::iges_draw::{
    IgesDrawCircArraySubfigure, IgesDrawNetworkSubfigure, IgesDrawNetworkSubfigureDef,
    IgesDrawRectArraySubfigure,
};
use crate::iges_geom::{
    IgesGeomBoundary, IgesGeomBoundedSurface, IgesGeomCurveOnSurface, IgesGeomPlane,
    IgesGeomTrimmedSurface,
};
use crate::iges_solid::{IgesSolidFace, IgesSolidLoop};
use martensite_tkernel::{
    impl_transient,
    standard::{Failure, Handle, StandardTransient},
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

iges_explore! {
    /// The faces an entity holds.
    ///
    /// ```
    /// use martensite_tkdeiges::iges_select::IgesSelectSelectFaces;
    /// use martensite_tkxsbase::if_select::IFSelectSelectExplore;
    /// assert_eq!(IgesSelectSelectFaces::new().explore_label().as_bytes(), b"Faces");
    /// ```
    IgesSelectSelectFaces, "IgesSelectSelectFaces", {}
}

impl Default for IgesSelectSelectFaces {
    fn default() -> Self {
        Self::new()
    }
}

impl IgesSelectSelectFaces {
    /// The selection.
    pub fn new() -> Self {
        Self {
            transient: StandardTransient::default(),
            base: IFSelectSelectExploreBase::new(-1),
        }
    }
}

impl IFSelectSelectExplore for IgesSelectSelectFaces {
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
        if matches!(igt, 510 | 144 | 143 | 114 | 118 | 120 | 122 | 128 | 140) {
            return Ok(true);
        }
        if igt == 108 {
            let plane = ent.cast::<IgesGeomPlane>();
            return Ok(plane.get().is_some_and(IgesGeomPlane::has_bounding_curve));
        }
        if igt == 402 && live.form_number() == 9 {
            return explore_single_parent(ent, explored);
        }
        if igt == 402 {
            return explore_group(ent, explored);
        }
        Ok(explore_solid(igt, ent, explored)?.unwrap_or(false))
    }
    fn explore_label(&self) -> AsciiString {
        AsciiString::new("Faces")
    }
}

iges_explore! {
    /// The parametric curves an entity holds; `basic` goes down to the basic curves.
    ///
    /// ```
    /// use martensite_tkdeiges::iges_select::IgesSelectSelectPCurves;
    /// use martensite_tkxsbase::if_select::IFSelectSelectExplore;
    /// assert_eq!(IgesSelectSelectPCurves::new(true).explore_label().as_bytes(), b"Basic PCurves");
    /// ```
    IgesSelectSelectPCurves, "IgesSelectSelectPCurves", { thebasic: bool }
}

impl IgesSelectSelectPCurves {
    /// The selection.
    pub fn new(basic: bool) -> Self {
        Self {
            transient: StandardTransient::default(),
            base: IFSelectSelectExploreBase::new(-1),
            thebasic: basic,
        }
    }
    fn explore_surfaces(
        &self,
        igt: i32,
        ent: &Handle<StandardTransient>,
        explored: &mut InterfaceEntityIterator,
    ) -> Result<Option<bool>, Failure> {
        match igt {
            144 => {
                let trimmed = ent.cast::<IgesGeomTrimmedSurface>();
                if let Some(live) = trimmed.get() {
                    add(explored, &live.outer_contour());
                    add_ranks(explored, live.nb_inner_contours(), |rank| {
                        live.inner_contour(rank)
                    })?;
                }
                Ok(Some(true))
            }
            142 => {
                let on_surface = ent.cast::<IgesGeomCurveOnSurface>();
                if let Some(live) = on_surface.get() {
                    add(explored, &live.curve_uv());
                    if self.thebasic {
                        IgesSelectSelectBasicGeom::sub_curves(&live.curve_uv(), explored)?;
                    }
                }
                Ok(Some(true))
            }
            141 => {
                let boundary = ent.cast::<IgesGeomBoundary>();
                let Some(live) = boundary.get() else {
                    return Ok(Some(false));
                };
                let nb = live.nb_model_space_curves()?;
                for rank in 1..=nb {
                    add_ranks(explored, live.nb_parameter_curves(rank)?, |index| {
                        live.parameter_curve(rank, index)
                    })?;
                }
                Ok(Some(nb > 0))
            }
            143 => {
                let bounded = ent.cast::<IgesGeomBoundedSurface>();
                let Some(live) = bounded.get() else {
                    return Ok(Some(false));
                };
                let nb = live.nb_boundaries();
                add_ranks(explored, nb, |rank| live.boundary(rank))?;
                Ok(Some(nb != 0))
            }
            510 => {
                let face = ent.cast::<IgesSolidFace>();
                if let Some(live) = face.get() {
                    add_ranks(explored, live.nb_loops()?, |rank| live.loop_(rank))?;
                }
                Ok(Some(true))
            }
            508 => {
                let edges = ent.cast::<IgesSolidLoop>();
                if let Some(live) = edges.get() {
                    for rank in 1..=live.nb_edges() {
                        add_ranks(explored, live.nb_parameter_curves(rank)?, |index| {
                            live.parametric_curve(rank, index)
                        })?;
                    }
                }
                Ok(Some(true))
            }
            _ => Ok(None),
        }
    }
}

impl IFSelectSelectExplore for IgesSelectSelectPCurves {
    fn explore_base(&self) -> &IFSelectSelectExploreBase {
        &self.base
    }
    fn explore(
        &self,
        level: i32,
        ent: &Handle<StandardTransient>,
        _g: &InterfaceGraph,
        explored: &mut InterfaceEntityIterator,
    ) -> Result<bool, Failure> {
        let entity = ent.cast::<IgesDataIgesEntity>();
        let Some(live) = entity.get() else {
            return Ok(false);
        };
        let igt = live.type_number();
        if let Some(result) = self.explore_surfaces(igt, ent, explored)? {
            return Ok(result);
        }
        if igt == 402 {
            return explore_group(ent, explored);
        }
        if let Some(result) = explore_solid(igt, ent, explored)? {
            return Ok(result);
        }
        if level < 3 {
            return Ok(false);
        }
        Ok(is_basic_curve(igt, live.form_number()))
    }
    fn explore_label(&self) -> AsciiString {
        AsciiString::new(if self.thebasic {
            "Basic PCurves"
        } else {
            "Global PCurves"
        })
    }
}

iges_explore! {
    /// The content of groups, down to a level (0 for any).
    ///
    /// ```
    /// use martensite_tkdeiges::iges_select::IgesSelectSelectBypassGroup;
    /// use martensite_tkxsbase::if_select::IFSelectSelectExplore;
    /// assert_eq!(IgesSelectSelectBypassGroup::new(0).explore_label().as_bytes(), b"Content of Groups");
    /// ```
    IgesSelectSelectBypassGroup, "IgesSelectSelectBypassGroup", {}
}

impl IgesSelectSelectBypassGroup {
    /// The selection down to a level.
    pub fn new(level: i32) -> Self {
        Self {
            transient: StandardTransient::default(),
            base: IFSelectSelectExploreBase::new(level),
        }
    }
}

impl IFSelectSelectExplore for IgesSelectSelectBypassGroup {
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
        explore_group(ent, explored)?;
        Ok(true)
    }
    fn explore_label(&self) -> AsciiString {
        AsciiString::new("Content of Groups")
    }
}

iges_explore! {
    /// The content of subfigures, down to a level (0 for any).
    ///
    /// ```
    /// use martensite_tkdeiges::iges_select::IgesSelectSelectBypassSubfigure;
    /// use martensite_tkxsbase::if_select::IFSelectSelectExplore;
    /// assert_eq!(IgesSelectSelectBypassSubfigure::new(0).explore_label().as_bytes(), b"Content of Subfigures");
    /// ```
    IgesSelectSelectBypassSubfigure, "IgesSelectSelectBypassSubfigure", {}
}

impl IgesSelectSelectBypassSubfigure {
    /// The selection down to a level.
    pub fn new(level: i32) -> Self {
        Self {
            transient: StandardTransient::default(),
            base: IFSelectSelectExploreBase::new(level),
        }
    }
}

impl IFSelectSelectExplore for IgesSelectSelectBypassSubfigure {
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
        match live.type_number() {
            308 => {
                let definition = ent.cast::<IgesBasicSubfigureDef>();
                if let Some(subf) = definition.get() {
                    add_ranks(explored, subf.nb_entities(), |rank| {
                        subf.associated_entity(rank)
                    })?;
                }
            }
            408 => {
                let subfigure = ent.cast::<IgesBasicSingularSubfigure>();
                if let Some(subf) = subfigure.get() {
                    add(explored, &subf.subfigure());
                }
            }
            320 => {
                let definition = ent.cast::<IgesDrawNetworkSubfigureDef>();
                if let Some(subf) = definition.get() {
                    add_ranks(explored, subf.nb_entities()?, |rank| subf.entity(rank))?;
                }
            }
            420 => {
                let network = ent.cast::<IgesDrawNetworkSubfigure>();
                if let Some(subf) = network.get() {
                    add(explored, &subf.subfigure_definition());
                }
            }
            412 => {
                let array = ent.cast::<IgesDrawRectArraySubfigure>();
                if let Some(subf) = array.get() {
                    add(explored, &subf.base_entity());
                }
            }
            414 => {
                let array = ent.cast::<IgesDrawCircArraySubfigure>();
                if let Some(subf) = array.get() {
                    add(explored, &subf.base_entity());
                }
            }
            _ => {}
        }
        Ok(true)
    }
    fn explore_label(&self) -> AsciiString {
        AsciiString::new("Content of Subfigures")
    }
}
