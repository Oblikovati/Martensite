//! The transfer of any entity with a geometry: curves, surfaces, BRep entities, groups, subfigures.
use super::{
    IgesToBRep, IgesToBRepBRepEntity, IgesToBRepCurveAndSurface, IgesToBRepTopoCurve,
    IgesToBRepTopoSurface,
};
use crate::iges_basic::{IgesBasicGroup, IgesBasicSingularSubfigure, IgesBasicSubfigureDef};
use crate::iges_data::{IgesDataIgesEntity, IgesDataToolLocation, iges_live};
use martensite_tkbrep::brep::BRepBuilder;
use martensite_tkbrep::topods::{Compound, Shape};
use martensite_tkernel::message::{Msg, ProgressRange, ProgressScope};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkmath::gp::{Transform3, Vector3};
use martensite_tkmath::toploc::Location;
use martensite_tkxsbase::interface::InterfaceStatic;

const WHERE: &str = "IgesToBRepCurveAndSurface";

impl IgesToBRepCurveAndSurface {
    /// The shape of a curve, surface or BRep entity; a fail for another entity;
    /// `tool.transfer_curve_and_surface(&entity, &range)?`.
    ///
    /// # Errors
    /// A failure of the entity or of the construction of the shape.
    pub fn transfer_curve_and_surface(
        &self,
        start: &Handle<IgesDataIgesEntity>,
        range: &ProgressRange,
    ) -> Result<Shape, Failure> {
        if start.is_null() {
            self.fail_key(start, "IGES_1005");
            return Ok(Shape::default());
        }
        if IgesToBRep::is_topo_curve(start) {
            return IgesToBRepTopoCurve::with_base(self).transfer_topo_curve(start);
        }
        if IgesToBRep::is_topo_surface(start) {
            return IgesToBRepTopoSurface::with_base(self).transfer_topo_surface(start);
        }
        if IgesToBRep::is_b_rep_entity(start) {
            return IgesToBRepBRepEntity::with_base(self).transfer_b_rep_entity(start, range);
        }
        self.fail_key(start, "IGES_1015");
        Ok(Shape::default())
    }
    /// The shape of an item, or the result it has; a failed transfer is the fail of the catalog.
    fn item_shape(
        &mut self,
        item: &Handle<IgesDataIgesEntity>,
        range: &ProgressRange,
    ) -> Result<Shape, Failure> {
        if self.has_shape_result(item)? {
            return self.get_shape_result(item);
        }
        Ok(self.transfer_geometry(item, range).unwrap_or_else(|_| {
            self.fail_key(item, "IGES_1015");
            Shape::default()
        }))
    }
    /// The subfigure of a singular subfigure, with the move it holds.
    fn subfigure_shape(
        &mut self,
        live: &IgesBasicSingularSubfigure,
        start: &Handle<IgesBasicSingularSubfigure>,
        range: &ProgressRange,
    ) -> Result<(Shape, Transform3), Failure> {
        let definition: Handle<IgesDataIgesEntity> = live.subfigure().cast();
        let translation = live.translation() * self.get_unit_factor();
        let mut placement = Transform3::default();
        placement.set_translation(Vector3::from(translation));
        if live.has_scale_factor() {
            placement.set_scale_factor(live.scale_factor())?;
        }
        if self.has_shape_result(&definition)? {
            return Ok((self.get_shape_result(&definition)?, placement));
        }
        let shape = match self.transfer_geometry(&definition, range) {
            Ok(shape) => shape,
            Err(_) => {
                self.fail_key(start, "IGES_1015");
                Shape::default()
            }
        };
        if !shape.is_null() {
            self.set_shape_result(&definition, &shape)?;
        }
        Ok((shape, placement))
    }
    fn rank_message(key: &str, rank: i32) -> Result<Msg, Failure> {
        let mut message = Msg::from_key(&key.into());
        message.arg_integer(rank)?;
        Ok(message)
    }
    /// The compound of the entities of a subfigure definition; a null shape without an entity.
    fn subfigure_definition_shape(
        &mut self,
        start: &Handle<IgesDataIgesEntity>,
        live: &IgesBasicSubfigureDef,
        range: &ProgressRange,
        only_visible: bool,
    ) -> Result<Shape, Failure> {
        let count = live.nb_entities();
        if count < 1 {
            self.fail_key(start, "XSTEP_210");
            return Ok(Shape::default());
        }
        let builder = BRepBuilder::default();
        let mut group = Compound::default();
        builder.make_compound(&mut group);
        let scope = ProgressScope::with_literal(range, "Subfigure item", f64::from(count), false);
        for rank in 1..=count {
            if !scope.more() {
                break;
            }
            let step = scope.next(1.0);
            let item = live.associated_entity(rank)?;
            let Some(body) = item.get() else {
                self.send_warning(start, &Self::rank_message("IGES_1020", rank)?);
                continue;
            };
            if only_visible && body.blank_status() == 1 {
                continue;
            }
            let shape = self.item_shape(&item, &step)?;
            if shape.is_null() {
                self.send_warning(start, &Self::rank_message("IGES_1025", rank)?);
            } else {
                builder.add(&mut group, &shape)?;
                self.set_shape_result(&item, &shape)?;
            }
        }
        Ok(group.into_shape())
    }
    /// The compound of the entities of a group; a null shape without an entity. Items without a
    /// shape give one warning at the end.
    fn group_shape(
        &mut self,
        start: &Handle<IgesDataIgesEntity>,
        live: &IgesBasicGroup,
        range: &ProgressRange,
        only_visible: bool,
    ) -> Result<Shape, Failure> {
        let form = iges_live(start, WHERE)?.form_number();
        let count = live.nb_entities();
        if count < 1 {
            let mut message = Msg::from_key(&"XSTEP_202".into());
            message.arg_integer(form)?;
            self.send_fail(start, &message);
            return Ok(Shape::default());
        }
        let builder = BRepBuilder::default();
        let mut group = Compound::default();
        builder.make_compound(&mut group);
        let scope = ProgressScope::with_literal(range, "Group item", f64::from(count), false);
        let mut problem = false;
        for rank in 1..=count {
            if !scope.more() {
                break;
            }
            let step = scope.next(1.0);
            let item = live.entity(rank)?;
            let Some(body) = item.get() else {
                self.send_fail(start, &Self::rank_message("IGES_1020", rank)?);
                continue;
            };
            if only_visible && body.blank_status() == 1 {
                continue;
            }
            let shape = self.item_shape(&item, &step)?;
            if shape.is_null() {
                problem = true;
            } else {
                builder.add(&mut group, &shape)?;
                self.set_shape_result(&item, &shape)?;
            }
        }
        if problem {
            self.send_warning(start, &Self::rank_message("IGES_1030", form)?);
        }
        Ok(group.into_shape())
    }
    /// The geometry of any entity, moved to its location; a null shape on a failure, with its fail;
    /// `tool.transfer_geometry(&entity, &range)?`.
    ///
    /// # Errors
    /// A failure of the entity or of the construction of the shape.
    pub fn transfer_geometry(
        &mut self,
        start: &Handle<IgesDataIgesEntity>,
        range: &ProgressRange,
    ) -> Result<Shape, Failure> {
        let Some(entity) = start.get() else {
            self.fail_key(start, "IGES_1005");
            return Ok(Shape::default());
        };
        // Invisible subentities are not transferred.
        let only_visible = InterfaceStatic::i_val("read.iges.onlyvisible") != 0;
        let blanked = only_visible && entity.blank_status() == 1;
        if IgesToBRep::is_curve_and_surface(start) {
            if blanked {
                return Ok(Shape::default());
            }
            return Ok(self
                .transfer_curve_and_surface(start, range)
                .unwrap_or_else(|_| {
                    self.fail_key(start, "IGES_1015");
                    Shape::default()
                }));
        }
        let mut subfigure_placement = None;
        let singular = start.cast::<IgesBasicSingularSubfigure>();
        let definition = start.cast::<IgesBasicSubfigureDef>();
        let group = start.cast::<IgesBasicGroup>();
        let mut result = if let Some(live) = singular.get() {
            if blanked {
                return Ok(Shape::default());
            }
            let (shape, placement) = self.subfigure_shape(live, &singular, range)?;
            subfigure_placement = Some(placement);
            shape
        } else if let Some(live) = definition.get() {
            let shape = self.subfigure_definition_shape(start, live, range, only_visible)?;
            if shape.is_null() {
                return Ok(shape);
            }
            shape
        } else if let Some(live) = group.get() {
            if blanked {
                return Ok(Shape::default());
            }
            let shape = self.group_shape(start, live, range, only_visible)?;
            if shape.is_null() {
                return Ok(shape);
            }
            shape
        } else {
            let mut message = Msg::from_key(&"IGES_1001".into());
            message.arg_integer(entity.form_number())?;
            self.send_fail(start, &message);
            return Ok(Shape::default());
        };
        if entity.has_transf() {
            let mut similarity = Transform3::default();
            self.set_epsilon(1.0e-4);
            let location = entity.compound_location()?;
            let unit = self.get_unit_factor();
            if IgesDataToolLocation::convert_location(
                self.get_epsilon(),
                &location,
                &mut similarity,
                unit,
            )? {
                if let Some(placement) = subfigure_placement {
                    similarity.set_translation_part(Vector3::from(
                        similarity.translation_part() + placement.translation_part(),
                    ));
                    similarity
                        .set_scale_factor(similarity.scale_factor() * placement.scale_factor())?;
                }
                result.move_by(&Location::new(similarity), false)?;
            } else {
                self.warn_key(start, "IGES_1035");
            }
        } else if let Some(placement) = subfigure_placement {
            result.move_by(&Location::new(placement), false)?;
        }
        Ok(result)
    }
}
