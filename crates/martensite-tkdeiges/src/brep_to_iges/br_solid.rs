//! `BRepToIgesBRSolid`: solids, composite solids and compounds as groups.
use super::{
    BRepToIgesBREntity, BRepToIgesBRShell, BRepToIgesBRWire, BRepToIgesOriginMap, entity_array,
};
use crate::iges_basic::IgesBasicGroup;
use crate::iges_data::IgesDataIgesEntity;
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::{self, CompSolid, Compound, Shape, Solid};
use martensite_tkernel::message::{ProgressRange, ProgressScope};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::top_abs::ShapeEnum;
use std::ops::{Deref, DerefMut};

/// Translates solids and compounds.
///
/// ```
/// use martensite_tkbrep::topods::Solid;
/// use martensite_tkdeiges::brep_to_iges::BRepToIgesBRSolid;
/// use martensite_tkernel::message::{ProgressRange, ProgressScope};
/// let tool = BRepToIgesBRSolid::new();
/// assert!(tool.transfer_solid(&Solid::default(), &ProgressRange::default()).unwrap().is_null());
/// ```
#[derive(Clone, Default)]
pub struct BRepToIgesBRSolid {
    base: BRepToIgesBREntity,
}

impl Deref for BRepToIgesBRSolid {
    type Target = BRepToIgesBREntity;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for BRepToIgesBRSolid {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

/// The result of a list: its only entity, or a group of all of them.
fn single_or_group(
    entities: &[Handle<IgesDataIgesEntity>],
    last: Handle<IgesDataIgesEntity>,
) -> Result<Handle<IgesDataIgesEntity>, Failure> {
    if entities.len() == 1 {
        return Ok(last);
    }
    let group = IgesBasicGroup::new();
    group.init(&entity_array(entities)?)?;
    Ok(Handle::new(group).cast())
}

/// The entities of the subshapes of one kind, out of `avoid`, each by `transfer`.
struct SubshapeWalk<'a> {
    owner: &'a Shape,
    entities: Vec<Handle<IgesDataIgesEntity>>,
    last: Handle<IgesDataIgesEntity>,
}

impl BRepToIgesBRSolid {
    /// A tool at the initial state; `BRepToIgesBRSolid::new()`.
    pub fn new() -> Self {
        Self::default()
    }
    /// A tool with the process and the model of `base`; `BRepToIgesBRSolid::with_entity(&base)`.
    pub fn with_entity(base: &BRepToIgesBREntity) -> Self {
        Self { base: base.clone() }
    }
    fn walk(
        &self,
        state: &mut SubshapeWalk<'_>,
        (kind, avoid): (ShapeEnum, ShapeEnum),
        scope: &ProgressScope<'_>,
        warning: &str,
        mut transfer: impl FnMut(&Shape, &ProgressRange) -> Result<Handle<IgesDataIgesEntity>, Failure>,
    ) -> Result<(), Failure> {
        let mut explorer = Explorer::new(state.owner, kind, avoid)?;
        while explorer.more() && scope.more() {
            let step = scope.next(1.0);
            let shape = explorer.current().clone();
            explorer.next()?;
            if shape.is_null() {
                self.add_warning(state.owner, warning)?;
                continue;
            }
            state.last = transfer(&shape, &step)?;
            if state.last.is_some() {
                state.entities.push(state.last.clone());
            }
        }
        Ok(())
    }
    /// The entity of a solid, a composite solid or a compound;
    /// `tool.transfer_solid_shape(&shape, &range)?`.
    ///
    /// # Errors
    /// A failure of the geometry of the shape.
    pub fn transfer_solid_shape(
        &self,
        start: &Shape,
        range: &ProgressRange,
    ) -> Result<Handle<IgesDataIgesEntity>, Failure> {
        if start.is_null() {
            return Ok(Handle::default());
        }
        match start.shape_type()? {
            ShapeEnum::Solid => self.transfer_solid(&topods::solid(start)?, range),
            ShapeEnum::CompSolid => self.transfer_comp_solid(&topods::comp_solid(start)?, range),
            ShapeEnum::Compound => self.transfer_compound(&topods::compound(start)?, range),
            _ => Ok(Handle::default()),
        }
    }
    /// The entity of a solid: its only shell, or a group; `tool.transfer_solid(&solid, &range)?`.
    ///
    /// # Errors
    /// A failure of the geometry of a shell or of the binding of the result.
    pub fn transfer_solid(
        &self,
        start: &Solid,
        range: &ProgressRange,
    ) -> Result<Handle<IgesDataIgesEntity>, Failure> {
        if start.is_null() {
            return Ok(Handle::default());
        }
        let count = super::count_subshapes(start, ShapeEnum::Shell, ShapeEnum::Shape)?;
        let scope = ProgressScope::unnamed(range, f64::from(count), false);
        let tool = BRepToIgesBRShell::with_entity(self);
        let mut walk = SubshapeWalk {
            owner: start,
            entities: Vec::new(),
            last: Handle::default(),
        };
        self.walk(
            &mut walk,
            (ShapeEnum::Shell, ShapeEnum::Shape),
            &scope,
            " an Shell is a null entity",
            |shape, step| tool.transfer_shell(&topods::shell(shape)?, step),
        )?;
        let result = single_or_group(&walk.entities, walk.last)?;
        self.set_shape_result(start, &result.cast())?;
        Ok(result)
    }
    /// The entity of a composite solid: its only solid, or a group;
    /// `tool.transfer_comp_solid(&solid, &range)?`.
    ///
    /// # Errors
    /// A failure of the geometry of a solid or of the binding of the result.
    pub fn transfer_comp_solid(
        &self,
        start: &CompSolid,
        range: &ProgressRange,
    ) -> Result<Handle<IgesDataIgesEntity>, Failure> {
        if start.is_null() {
            return Ok(Handle::default());
        }
        let count = super::count_subshapes(start, ShapeEnum::Solid, ShapeEnum::Shape)?;
        let scope = ProgressScope::unnamed(range, f64::from(count), false);
        let mut walk = SubshapeWalk {
            owner: start,
            entities: Vec::new(),
            last: Handle::default(),
        };
        self.walk(
            &mut walk,
            (ShapeEnum::Solid, ShapeEnum::Shape),
            &scope,
            " an Solid is a null entity",
            |shape, step| self.transfer_solid(&topods::solid(shape)?, step),
        )?;
        let result = single_or_group(&walk.entities, walk.last)?;
        self.set_shape_result(start, &result.cast())?;
        Ok(result)
    }
    /// The entity of a compound: its only part, or a group of all its free parts;
    /// `tool.transfer_compound(&compound, &range)?`.
    ///
    /// # Errors
    /// A failure of the geometry of a part or of the binding of the result.
    pub fn transfer_compound(
        &self,
        start: &Compound,
        range: &ProgressRange,
    ) -> Result<Handle<IgesDataIgesEntity>, Failure> {
        if start.is_null() {
            return Ok(Handle::default());
        }
        let parts = [
            (ShapeEnum::Solid, ShapeEnum::Shape),
            (ShapeEnum::Shell, ShapeEnum::Solid),
            (ShapeEnum::Face, ShapeEnum::Shell),
            (ShapeEnum::Wire, ShapeEnum::Face),
            (ShapeEnum::Edge, ShapeEnum::Wire),
            (ShapeEnum::Vertex, ShapeEnum::Edge),
        ];
        let mut count = 0;
        for (kind, avoid) in parts {
            count += super::count_subshapes(start, kind, avoid)?;
        }
        let scope = ProgressScope::unnamed(range, f64::from(count), false);
        let shells = BRepToIgesBRShell::with_entity(self);
        let wires = BRepToIgesBRWire::with_entity(self);
        let mut walk = SubshapeWalk {
            owner: start,
            entities: Vec::new(),
            last: Handle::default(),
        };
        self.walk(
            &mut walk,
            parts[0],
            &scope,
            " a Solid is a null entity",
            |shape, step| self.transfer_solid(&topods::solid(shape)?, step),
        )?;
        self.walk(
            &mut walk,
            parts[1],
            &scope,
            " a Shell is a null entity",
            |shape, step| shells.transfer_shell(&topods::shell(shape)?, step),
        )?;
        self.walk(
            &mut walk,
            parts[2],
            &scope,
            " a Face is a null entity",
            |shape, step| shells.transfer_face(&topods::face(shape)?, step),
        )?;
        self.walk(
            &mut walk,
            parts[3],
            &scope,
            " a Wire is a null entity",
            |shape, _| wires.transfer_wire(&topods::wire(shape)?),
        )?;
        self.walk(
            &mut walk,
            parts[4],
            &scope,
            " an Edge is a null entity",
            |shape, _| {
                wires.transfer_edge(
                    &topods::edge(shape)?,
                    &BRepToIgesOriginMap::default(),
                    false,
                )
            },
        )?;
        self.walk(
            &mut walk,
            parts[5],
            &scope,
            " a Vertex is a null entity",
            |shape, _| wires.transfer_vertex(&topods::vertex(shape)?),
        )?;
        let result = single_or_group(&walk.entities, walk.last)?;
        self.set_shape_result(start, &result.cast())?;
        Ok(result)
    }
}
