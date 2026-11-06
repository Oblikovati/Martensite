//! `STEPCAFControlWriter::writeDGTsAP242`: the datums, dimensions, tolerances and presentations of
//! the AP242 schema.
use super::STEPCAFControlGDTProperty;
use super::writer::{WHERE, WriterState};
use super::writer_datum::relate_aspects;
use super::writer_dim_values::{write_derived_geometry, write_dim_values};
use super::writer_gdt_common::{GdtPresentation, bytes_of, empty, graph_of};
use super::writer_support::model_of;
use super::writer_tolerance::{DatumMap, write_datum_system};
use crate::step_ap242::StepAP242GeometricItemSpecificUsage;
use crate::step_construct::{StepConstructRenderingProperties, StepConstructStyles};
use crate::step_data::{StepDataFactors, StepDataLogical, StepDataSelectType};
use crate::step_dim_tol::StepDimTolDatum;
use crate::step_entity::{step_array1_set, step_live, step_new_array1};
use crate::step_geom::StepGeomCartesianPoint;
use crate::step_repr::{
    StepReprCompositeShapeAspect, StepReprConstructiveGeometryRepresentation,
    StepReprConstructiveGeometryRepresentationRelationship, StepReprProductDefinitionShape,
    StepReprRepresentationContext, StepReprRepresentationItem, StepReprShapeAspect,
};
use crate::step_shape::{
    StepShapeAngleRelator, StepShapeAngularLocation, StepShapeAngularSize,
    StepShapeDimensionalCharacteristic, StepShapeDimensionalLocation,
    StepShapeDimensionalLocationWithPath, StepShapeDimensionalSize,
    StepShapeDimensionalSizeWithPath,
};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tklcaf::tdf::TdfLabel;
use martensite_tkxcaf::xcaf_dim_tol_objects::{
    XCAFDimTolObjectsAngularQualifier, XCAFDimTolObjectsDimensionObject,
    XCAFDimTolObjectsDimensionType,
};
use martensite_tkxcaf::xcaf_doc::{
    XCAFDocDatum, XCAFDocDimTolTool, XCAFDocDimension, XCAFDocDocumentTool, XCAFDocShapeTool,
};
use martensite_tkxsbase::interface::InterfaceModelEntities;
use martensite_tkxsbase::xs_control::XSControlWorkSession;
use std::collections::HashSet;

fn shape_of(label: &TdfLabel) -> Result<Shape, Failure> {
    let mut shape = Shape::default();
    XCAFDocShapeTool::get_shape(label, &mut shape)?;
    Ok(shape)
}

/// The relator of an angular dimension: small, large, or equal otherwise.
fn angle_relator(object: &XCAFDimTolObjectsDimensionObject) -> StepShapeAngleRelator {
    if !object.has_qualifier() {
        return StepShapeAngleRelator::Equal;
    }
    match object.get_angular_qualifier() {
        XCAFDimTolObjectsAngularQualifier::Small => StepShapeAngleRelator::Small,
        XCAFDimTolObjectsAngularQualifier::Large => StepShapeAngleRelator::Large,
        _ => StepShapeAngleRelator::Equal,
    }
}

/// What the loop over the dimensions shares.
#[derive(Default)]
struct DimensionWalk {
    context: Handle<StepReprRepresentationContext>,
    dummy_context: Handle<StepReprRepresentationContext>,
    dummy_usage: Handle<StepAP242GeometricItemSpecificUsage>,
    points: Vec<Handle<StepGeomCartesianPoint>>,
}

impl DimensionWalk {
    fn keep_context(&mut self) {
        if self.context.is_null() && self.dummy_context.is_some() {
            self.context = self.dummy_context.clone();
        }
    }
}

impl WriterState {
    /// Fills the state shared by the presentations: the curve style and the common shape.
    fn prepare_presentations(&mut self, ws: &Handle<XSControlWorkSession>) -> Result<(), Failure> {
        // Common entities for presentation
        let styles = StepConstructStyles::with_ws(ws)?;
        let color = StepConstructStyles::encode_color(&Color::named(NamedColor::WHITE))?;
        let psa = styles.make_color_psa(
            &Handle::default(),
            &color,
            &color,
            &StepConstructRenderingProperties::new(),
            false,
        )?;
        step_array1_set(&self.gdt_prs_curve_style, 1, psa, WHERE)?;
        if self.gdt_common_pds.is_null() {
            let model = model_of(ws)?;
            for index in 1..=model.nb_entities() {
                let pds: Handle<StepReprProductDefinitionShape> = model.value(index)?.cast();
                if pds.is_some() {
                    self.gdt_common_pds = pds;
                    break;
                }
            }
        }
        Ok(())
    }
    /// Find all shapes with datums, and write them.
    fn write_all_datums(
        &mut self,
        ws: &Handle<XSControlWorkSession>,
        factors: &StepDataFactors,
    ) -> Result<DatumMap, Failure> {
        let mut datum_map = DatumMap::new();
        let mut labels = Vec::new();
        for root in self.root_labels.clone() {
            let tool = XCAFDocDocumentTool::dim_tol_tool(&root)?;
            labels.extend(step_live(&tool, WHERE)?.get_datum_labels()?);
        }
        let mut name_ids = HashSet::new();
        for datum_label in &labels {
            let (mut shapes, mut null_sequence) = (Vec::new(), Vec::new());
            XCAFDocDimTolTool::get_ref_shape_label(datum_label, &mut shapes, &mut null_sequence)?;
            let mut attribute: Handle<XCAFDocDatum> = Handle::default();
            datum_label.find_attribute_as(&XCAFDocDatum::get_id(), &mut attribute)?;
            let object = step_live(&attribute, WHERE)?.get_object()?;
            let live = step_live(&object, WHERE)?;
            let name = bytes_of(&live.get_name())?;
            let mut name_id = name.clone();
            name_id.extend(live.get_datum_target_number().to_string().into_bytes());
            if !name_ids.insert(name_id) {
                continue;
            }
            let written = datum_map.get(&name).cloned();
            let is_first = written.is_none();
            let written: Handle<StepDimTolDatum> =
                written.map_or_else(Handle::default, |w| w.cast());
            let datum =
                self.write_datum_ap242(ws, &shapes, datum_label, is_first, &written, factors)?;
            // Add created Datum into Map
            datum_map.insert(name, datum.cast());
        }
        Ok(datum_map)
    }
    /// The aspect of the first shapes of a dimension: of one shape, or a composite.
    fn first_shapes_aspect(
        &self,
        ws: &Handle<XSControlWorkSession>,
        dimension: &TdfLabel,
        shapes: &[TdfLabel],
        walk: &mut DimensionWalk,
    ) -> Result<Handle<StepReprShapeAspect>, Failure> {
        let model = model_of(ws)?;
        if let [single] = shapes {
            let aspect = self.write_shape_aspect(
                ws,
                dimension,
                &shape_of(single)?,
                &mut walk.dummy_context,
                &mut walk.dummy_usage,
            )?;
            walk.keep_context();
            return Ok(aspect);
        }
        let mut composite: Handle<StepReprCompositeShapeAspect> = Handle::default();
        for label in shapes {
            let aspect = self.write_shape_aspect(
                ws,
                dimension,
                &shape_of(label)?,
                &mut walk.dummy_context,
                &mut walk.dummy_usage,
            )?;
            let Some(live) = aspect.get() else {
                continue;
            };
            if composite.is_null() {
                composite = Handle::new(StepReprCompositeShapeAspect::new());
                step_live(&composite, WHERE)?.init(
                    &live.name(),
                    &live.description(),
                    &live.of_shape(),
                    live.product_definitional(),
                );
                model.add_with_refs(&composite.cast(), 0, false)?;
            }
            relate_aspects(&model, &composite.cast(), &aspect, empty())?;
            walk.keep_context();
        }
        Ok(composite.cast())
    }
    /// The aspect of the second shapes: a shape without aspect fails here, unlike the first.
    fn second_shapes_aspect(
        &self,
        ws: &Handle<XSControlWorkSession>,
        dimension: &TdfLabel,
        shapes: &[TdfLabel],
        walk: &mut DimensionWalk,
    ) -> Result<Handle<StepReprShapeAspect>, Failure> {
        let model = model_of(ws)?;
        if let [single] = shapes {
            let aspect = self.write_shape_aspect(
                ws,
                dimension,
                &shape_of(single)?,
                &mut walk.dummy_context,
                &mut walk.dummy_usage,
            )?;
            walk.keep_context();
            return Ok(aspect);
        }
        let mut composite: Handle<StepReprCompositeShapeAspect> = Handle::default();
        for label in shapes {
            let aspect = self.write_shape_aspect(
                ws,
                dimension,
                &shape_of(label)?,
                &mut walk.dummy_context,
                &mut walk.dummy_usage,
            )?;
            if composite.is_null() && aspect.is_some() {
                composite = Handle::new(StepReprCompositeShapeAspect::new());
            }
            let live = step_live(&aspect, WHERE)?;
            step_live(&composite, WHERE)?.init(
                &live.name(),
                &live.description(),
                &live.of_shape(),
                live.product_definitional(),
            );
            relate_aspects(&model, &composite.cast(), &aspect, empty())?;
            walk.keep_context();
        }
        Ok(composite.cast())
    }
    /// The entity of a dimension, of the subtype of its type. Null value for another type.
    fn dimension_entity(
        &self,
        ws: &Handle<XSControlWorkSession>,
        dimension_label: &TdfLabel,
        object: &XCAFDimTolObjectsDimensionObject,
        first: &Handle<StepReprShapeAspect>,
        second: &Handle<StepReprShapeAspect>,
        walk: &mut DimensionWalk,
    ) -> Result<StepShapeDimensionalCharacteristic, Failure> {
        let mut dimension = StepShapeDimensionalCharacteristic::default();
        let kind = object.get_type();
        if XCAFDimTolObjectsDimensionObject::is_dimensional_location(kind) {
            // Dimensional_Location
            let entity = Handle::new(StepShapeDimensionalLocation::new());
            entity.init(
                &STEPCAFControlGDTProperty::get_dim_type_name(kind),
                false,
                &Handle::default(),
                first,
                second,
            );
            dimension.set_value(&entity.cast())?;
        } else if kind == XCAFDimTolObjectsDimensionType::LocationAngular {
            // Angular_Location
            let entity = Handle::new(StepShapeAngularLocation::new());
            entity.init(
                &empty(),
                false,
                &Handle::default(),
                first,
                second,
                angle_relator(object),
            );
            dimension.set_value(&entity.cast())?;
        } else if kind == XCAFDimTolObjectsDimensionType::LocationWithPath {
            // Dimensional_Location_With_Path
            let entity = Handle::new(StepShapeDimensionalLocationWithPath::new());
            let path = self.write_shape_aspect(
                ws,
                dimension_label,
                &object.get_path().into(),
                &mut walk.dummy_context,
                &mut walk.dummy_usage,
            )?;
            entity.init(&empty(), false, &Handle::default(), first, second, &path);
            dimension.set_value(&entity.cast())?;
        } else if XCAFDimTolObjectsDimensionObject::is_dimensional_size(kind) {
            // Dimensional_Size
            let entity = Handle::new(StepShapeDimensionalSize::new());
            entity.init(first, &STEPCAFControlGDTProperty::get_dim_type_name(kind));
            dimension.set_value(&entity.cast())?;
        } else if kind == XCAFDimTolObjectsDimensionType::SizeAngular {
            // Angular_Size
            let entity = Handle::new(StepShapeAngularSize::new());
            entity.init(first, &empty(), angle_relator(object));
            dimension.set_value(&entity.cast())?;
        } else if kind == XCAFDimTolObjectsDimensionType::SizeWithPath {
            // Dimensional_Size_With_Path
            let entity = Handle::new(StepShapeDimensionalSizeWithPath::new());
            let path = self.write_shape_aspect(
                ws,
                dimension_label,
                &object.get_path().into(),
                &mut walk.dummy_context,
                &mut walk.dummy_usage,
            )?;
            entity.init(first, &empty(), &path);
            dimension.set_value(&entity.cast())?;
        }
        Ok(dimension)
    }
    /// One dimension.
    fn write_dimension_ap242(
        &mut self,
        ws: &Handle<XSControlWorkSession>,
        label: &TdfLabel,
        representation: &Handle<StepReprConstructiveGeometryRepresentation>,
        walk: &mut DimensionWalk,
        factors: &StepDataFactors,
    ) -> Result<(), Failure> {
        let model = model_of(ws)?;
        let mut attribute: Handle<XCAFDocDimension> = Handle::default();
        if !label.find_attribute_as(&XCAFDocDimension::get_id(), &mut attribute)? {
            return Ok(());
        }
        let object = step_live(&attribute, WHERE)?.get_object()?;
        let Some(live) = object.get() else {
            return Ok(());
        };
        let presentation = |has_semantic: bool, has_plane: bool| GdtPresentation {
            shape: live.get_presentation(),
            name: live.get_presentation_name(),
            has_semantic,
            has_plane,
            plane: live.get_plane(),
            text_position: live.get_point_text_attach(),
        };
        if live.get_type() == XCAFDimTolObjectsDimensionType::CommonLabel {
            let aspect = Handle::new(StepReprShapeAspect::new());
            aspect.init(
                &empty(),
                &empty(),
                &self.gdt_common_pds,
                StepDataLogical::LTrue,
            );
            model.add_with_refs(&aspect.cast(), 0, false)?;
            self.write_presentation(
                ws,
                &presentation(live.has_plane(), false),
                &aspect.cast(),
                factors,
            )?;
        }
        let (mut firsts, mut seconds) = (Vec::new(), Vec::new());
        if !XCAFDocDimTolTool::get_ref_shape_label(label, &mut firsts, &mut seconds)? {
            return Ok(());
        }
        // Write links with shapes
        let mut first = Handle::default();
        if !firsts.is_empty() {
            first = self.first_shapes_aspect(ws, label, &firsts, walk)?;
        }
        let mut second = Handle::default();
        if !seconds.is_empty() {
            second = self.second_shapes_aspect(ws, label, &seconds, walk)?;
        }
        if live.get_type() == XCAFDimTolObjectsDimensionType::DimensionPresentation {
            return self.write_presentation(
                ws,
                &presentation(live.has_plane(), false),
                &first.cast(),
                factors,
            );
        }
        // Write dimensions
        if live.has_point() || live.has_point2() {
            write_derived_geometry(
                ws,
                live,
                representation,
                &mut first,
                &mut second,
                &mut walk.points,
                factors,
            )?;
        }
        let dimension = self.dimension_entity(ws, label, live, &first, &second, walk)?;
        // Write values
        write_dim_values(ws, live, &walk.context, &dimension, factors)?;
        // Annotation plane and Presentation
        self.write_presentation(
            ws,
            &presentation(true, live.has_plane()),
            &dimension.value(),
            factors,
        )
    }
    /// `writeDGTsAP242`.
    pub(super) fn write_dgts_ap242(
        &mut self,
        ws: &Handle<XSControlWorkSession>,
        _labels: &[TdfLabel],
        factors: &StepDataFactors,
    ) -> Result<bool, Failure> {
        // Get working data
        let model = model_of(ws)?;
        if graph_of(ws)?.is_null() {
            return Ok(false);
        }
        self.prepare_presentations(ws)?;
        let datum_map = self.write_all_datums(ws, factors)?;
        // write Dimensions
        let mut labels = Vec::new();
        for root in self.root_labels.clone() {
            let tool = XCAFDocDocumentTool::dim_tol_tool(&root)?;
            labels.extend(step_live(&tool, WHERE)?.get_dimension_labels()?);
        }
        // Auxiliary entities for derived geometry
        let representation = Handle::new(StepReprConstructiveGeometryRepresentation::new());
        let relationship =
            Handle::new(StepReprConstructiveGeometryRepresentationRelationship::new());
        let mut walk = DimensionWalk::default();
        for label in &labels {
            self.write_dimension_ap242(ws, label, &representation, &mut walk, factors)?;
        }
        // Write Derived geometry
        if !walk.points.is_empty() {
            let items =
                step_new_array1::<Handle<StepReprRepresentationItem>>(1, walk.points.len() as i32)?;
            for (index, point) in walk.points.iter().enumerate() {
                step_array1_set(&items, index as i32 + 1, point.cast(), WHERE)?;
            }
            representation.init(&empty(), &items, &walk.dummy_context);
            let used = step_live(&walk.dummy_usage, WHERE)?.used_representation();
            relationship.init(&empty(), &empty(), &used, &representation.cast());
            model.add_with_refs(&relationship.cast(), 0, false)?;
        }
        // write Geometric Tolerances
        let mut tolerances = Vec::new();
        for root in self.root_labels.clone() {
            let tool = XCAFDocDocumentTool::dim_tol_tool(&root)?;
            tolerances.extend(step_live(&tool, WHERE)?.get_geom_tolerance_labels()?);
        }
        for label in &tolerances {
            let (mut firsts, mut null_sequence) = (Vec::new(), Vec::new());
            if !XCAFDocDimTolTool::get_ref_shape_label(label, &mut firsts, &mut null_sequence)? {
                continue;
            }
            let mut datums = Vec::new();
            XCAFDocDimTolTool::get_datum_with_object_of_toler_labels(label, &mut datums)?;
            let datum_system = if datums.is_empty() {
                Handle::default()
            } else {
                write_datum_system(ws, label, &datums, &datum_map, &walk.context, factors)?
            };
            self.write_geom_tolerance(ws, &firsts, label, &datum_system, &walk.context, factors)?;
        }
        // Write Draughting model for Annotation Planes
        if self.gdt_annotations.is_empty() {
            return Ok(true);
        }
        let items = step_new_array1::<Handle<StepReprRepresentationItem>>(
            1,
            self.gdt_annotations.len() as i32,
        )?;
        for (index, annotation) in self.gdt_annotations.iter().enumerate() {
            step_array1_set(&items, index as i32 + 1, annotation.clone(), WHERE)?;
        }
        step_live(&self.gdt_presentation_dm, WHERE)?.init(&empty(), &items, &walk.context);
        model.add_with_refs(&self.gdt_presentation_dm.cast(), 0, false)?;
        Ok(true)
    }
}
