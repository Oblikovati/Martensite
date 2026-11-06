//! Composite solids and compounds as groups.
use super::BRepToIgesBRepEntity;
use crate::brep_to_iges::{BRepToIgesOriginMap, count_subshapes, entity_array};
use crate::iges_basic::IgesBasicGroup;
use crate::iges_data::IgesDataIgesEntity;
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::{self, CompSolid, Compound};
use martensite_tkernel::message::{ProgressRange, ProgressScope};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::top_abs::ShapeEnum;

fn group_of(
    entities: &[Handle<IgesDataIgesEntity>],
) -> Result<Handle<IgesDataIgesEntity>, Failure> {
    let group = IgesBasicGroup::new();
    group.init(&entity_array(entities)?)?;
    Ok(Handle::new(group).cast())
}

impl BRepToIgesBRepEntity {
    /// The entity of a composite solid: its only solid, or a group;
    /// `tool.transfer_comp_solid(&solid, &range)?`.
    ///
    /// # Errors
    /// A failure of a solid or of the binding of the result.
    pub fn transfer_comp_solid(
        &mut self,
        start: &CompSolid,
        range: &ProgressRange<'_>,
    ) -> Result<Handle<IgesDataIgesEntity>, Failure> {
        if start.is_null() {
            return Ok(Handle::default());
        }
        let count = count_subshapes(start, ShapeEnum::Solid, ShapeEnum::Shape)?;
        let scope = ProgressScope::unnamed(range, f64::from(count), false);
        let mut solids: Vec<Handle<IgesDataIgesEntity>> = Vec::new();
        let mut explorer = Explorer::new(start, ShapeEnum::Solid, ShapeEnum::Shape)?;
        while explorer.more() && scope.more() {
            let step = scope.next(1.0);
            let solid = topods::solid(explorer.current())?;
            explorer.next()?;
            if solid.is_null() {
                self.add_warning(start, " an Solid is a null entity")?;
                continue;
            }
            solids.push(self.transfer_solid(&solid, &step)?.cast());
        }
        let result = match solids.len() {
            0 => Handle::default(),
            1 => solids[0].clone(),
            _ => group_of(&solids)?,
        };
        self.set_shape_result(start, &result.cast())?;
        Ok(result)
    }
    /// The group of all the free parts of a compound, null without one;
    /// `tool.transfer_compound(&compound, &range)?`.
    ///
    /// # Errors
    /// A failure of a part or of the binding of the result.
    pub fn transfer_compound(
        &mut self,
        start: &Compound,
        range: &ProgressRange<'_>,
    ) -> Result<Handle<IgesDataIgesEntity>, Failure> {
        if start.is_null() {
            return Ok(Handle::default());
        }
        let kinds = [
            (ShapeEnum::Solid, ShapeEnum::Shape),
            (ShapeEnum::Shell, ShapeEnum::Solid),
            (ShapeEnum::Face, ShapeEnum::Shell),
            (ShapeEnum::Wire, ShapeEnum::Face),
            (ShapeEnum::Edge, ShapeEnum::Wire),
            (ShapeEnum::Vertex, ShapeEnum::Edge),
        ];
        let mut count = 0;
        for (kind, avoid) in kinds {
            count += count_subshapes(start, kind, avoid)?;
        }
        let scope = ProgressScope::unnamed(range, f64::from(count), false);
        let mut parts: Vec<Handle<IgesDataIgesEntity>> = Vec::new();
        for (kind, avoid) in kinds {
            let mut explorer = Explorer::new(start, kind, avoid)?;
            while explorer.more() && scope.more() {
                let step = scope.next(1.0);
                let shape = explorer.current().clone();
                explorer.next()?;
                if shape.is_null() {
                    self.add_warning(start, " a null part of a compound")?;
                    continue;
                }
                let origins = BRepToIgesOriginMap::default();
                let part: Handle<IgesDataIgesEntity> = match kind {
                    ShapeEnum::Solid => self.transfer_solid(&topods::solid(&shape)?, &step)?.cast(),
                    ShapeEnum::Shell => self.transfer_shell(&topods::shell(&shape)?, &step)?.cast(),
                    ShapeEnum::Face => self.transfer_face(&topods::face(&shape)?)?.cast(),
                    ShapeEnum::Wire => self.wire_tool()?.transfer_wire(&topods::wire(&shape)?)?,
                    ShapeEnum::Edge => {
                        self.wire_tool()?
                            .transfer_edge(&topods::edge(&shape)?, &origins, false)?
                    }
                    _ => self
                        .wire_tool()?
                        .transfer_vertex(&topods::vertex(&shape)?)?,
                };
                if part.is_some() {
                    parts.push(part);
                }
            }
        }
        let result = if parts.is_empty() {
            Handle::default()
        } else {
            group_of(&parts)?
        };
        self.set_shape_result(start, &result.cast())?;
        Ok(result)
    }
}
