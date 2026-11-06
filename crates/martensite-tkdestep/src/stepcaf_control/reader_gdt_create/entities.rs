//! The GD&T entities created by `STEPCAFControlReader`.
use super::*;

impl OldVersion<'_> {
    /// A tolerance and its datums on an advanced face.
    fn read_tolerance(
        &self,
        entity: &Handle<StandardTransient>,
        item: &Handle<StepReprRepresentationItem>,
    ) -> Result<(), Failure> {
        let tolerance = entity.cast::<StepDimTolGeometricTolerance>();
        let tolerance = step_live(&tolerance, WHERE)?;
        // read common data for tolerance
        let magnitude = measure_with_unit(&tolerance.magnitude());
        let Some(magnitude) = magnitude.get() else {
            return Ok(());
        };
        let Some(named) = named_unit_of(&magnitude.unit_component()) else {
            return Ok(());
        };
        let value =
            magnitude.value_component() * context_of_unit(&named, self.factors)?.length_factor();
        let values = shared_array1(vec![value]);
        let (name, description) = (tolerance.name(), tolerance.description());
        let face = item.cast::<StepShapeAdvancedFace>();
        if face.is_null() {
            return Ok(());
        }
        let Some(label) = self.label_of_item(&face.cast())? else {
            return Ok(());
        };
        // read specific data for tolerance
        let tool = self.dim_tol_tool;
        if let Some(complex) = entity
            .cast::<StepDimTolGeoTolAndGeoTolWthDatRefAndModGeoTolAndPosTol>()
            .get()
        {
            let modified = complex.get_modified_geometric_tolerance();
            let kind = 20 + modified.get().map_or(0, |live| live.modifier() as i32 + 1);
            let tolerance_label =
                tool.set_dim_tol_values(&label, kind, &values, &name, &description)?;
            // translate datums connected with this tolerance
            let reference = complex.get_geometric_tolerance_with_datum_reference();
            if reference.is_some() {
                read_datums(
                    self.shape_tool,
                    tool,
                    self.session,
                    &tolerance_label,
                    &reference,
                )?;
            }
        } else if entity
            .cast::<StepDimTolGeometricToleranceWithDatumReference>()
            .is_some()
        {
            let kind = kind_of_reference_tolerance(entity);
            let tolerance_label =
                tool.set_dim_tol_values(&label, kind, &values, &name, &description)?;
            read_datums(
                self.shape_tool,
                tool,
                self.session,
                &tolerance_label,
                &entity.cast(),
            )?;
        } else if let Some(modified) = entity.cast::<StepDimTolModifiedGeometricTolerance>().get() {
            tool.set_dim_tol_values(
                &label,
                35 + modified.modifier() as i32,
                &values,
                &name,
                &description,
            )?;
        } else if let Some(kind) = kind_of_plain_tolerance(entity) {
            tool.set_dim_tol_values(&label, kind, &values, &name, &description)?;
        }
        Ok(())
    }
    /// The property definition of the old version: dimensions and tolerances are made directly.
    fn read_property(
        &self,
        entity: &Handle<StandardTransient>,
        definition: &Handle<StepReprPropertyDefinition>,
    ) -> Result<(), Failure> {
        let mut item = Handle::<StepReprRepresentationItem>::default();
        for sharing in self.session.sharings(&definition.cast())? {
            if let Some(sdr) = sharing
                .cast::<StepShapeShapeDefinitionRepresentation>()
                .get()
            {
                let representation = sdr.used_representation();
                if let Some(representation) =
                    representation.get().filter(|live| live.nb_items() > 0)
                {
                    item = representation.items_value(1)?;
                }
            }
        }
        if item.is_null() {
            return Ok(());
        }
        if entity.cast::<StepShapeDimensionalSize>().is_some() {
            return self.read_dimension(entity, &item);
        }
        if entity.cast::<StepDimTolGeometricTolerance>().is_some() {
            return self.read_tolerance(entity, &item);
        }
        Ok(())
    }
}

/// The labels of items: a wire without a label gives those of its edges.
fn labels_of_items(
    items: &[Handle<StandardTransient>],
    session: &SessionGraph,
    tool: &XCAFDocShapeTool,
) -> Result<Vec<TdfLabel>, Failure> {
    let mut labels = Vec::new();
    for item in items {
        let shape = session.shape_at(find_shape_index_for_dgt(item, session)?)?;
        if shape.is_null() {
            continue;
        }
        let mut label = TdfLabel::default();
        tool.search(&shape, &mut label, [true, true, true])?;
        if label.is_null() && shape.shape_type()? == ShapeEnum::Wire {
            let mut edges = Explorer::new(&shape, ShapeEnum::Edge, ShapeEnum::Shape)?;
            while edges.more() {
                let mut edge_label = TdfLabel::default();
                tool.search(edges.current(), &mut edge_label, [true, true, true])?;
                if !edge_label.is_null() {
                    labels.push(edge_label);
                }
                edges.next()?;
            }
        }
        if !label.is_null() {
            labels.push(label);
        }
    }
    Ok(labels)
}

/// What the shapes of an entity gather while it is read.
#[derive(Default)]
struct CollectedItems {
    first: Vec<Handle<StandardTransient>>,
    second: Vec<Handle<StandardTransient>>,
    is_all_around: bool,
    is_all_over: bool,
}

impl ReaderState {
    /// The aspects of one attribute an entity shares, gathering the all-over items.
    fn aspects_of_attribute(
        session: &SessionGraph,
        attribute: &Handle<StandardTransient>,
        collected: &mut CollectedItems,
    ) -> Result<Vec<Handle<StepReprShapeAspect>>, Failure> {
        let mut aspects = Vec::new();
        if attribute.cast::<StepReprProductDefinitionShape>().is_some() {
            // if associating tolerances with part (All-Over)
            for sharing in session.sharings(attribute)? {
                if sharing
                    .cast::<StepShapeShapeDefinitionRepresentation>()
                    .is_null()
                {
                    continue;
                }
                collected.is_all_over = true;
                for shared in session.shareds(&sharing)? {
                    if shared
                        .cast::<StepShapeAdvancedBrepShapeRepresentation>()
                        .is_some()
                    {
                        collected.first.push(shared);
                    }
                }
            }
        } else if attribute.cast::<StepShapeDimensionalLocation>().is_some()
            || attribute.cast::<StepShapeDimensionalSize>().is_some()
        {
            // if tolerance attached to dimension
            for shared in session.shareds(attribute)? {
                let aspect = shared.cast::<StepReprShapeAspect>();
                if aspect.is_some() {
                    collect_shape_aspect(&aspect, session, &mut aspects)?;
                }
            }
        } else if attribute.cast::<StepReprShapeAspect>().is_some() {
            // if applied AllAround Modifier
            collected.is_all_around |= attribute.cast::<StepReprAllAroundShapeAspect>().is_some();
            // dimensions and default tolerances
            collect_shape_aspect(&attribute.cast(), session, &mut aspects)?;
        }
        Ok(aspects)
    }
    /// The representation items of the aspects; the old version of the structure is read here.
    fn items_of_aspects(
        old: &OldVersion<'_>,
        entity: &Handle<StandardTransient>,
        aspects: &[Handle<StepReprShapeAspect>],
    ) -> Result<Vec<Handle<StandardTransient>>, Failure> {
        let mut items = Vec::new();
        for aspect in aspects {
            // the first sharing of either kind ends the search
            let mut usage = Handle::<StepAP242GeometricItemSpecificUsage>::default();
            let mut definition = Handle::<StepReprPropertyDefinition>::default();
            for sharing in old.session.sharings(&aspect.cast())? {
                usage = sharing.cast();
                definition = sharing.cast();
                if usage.is_some() || definition.is_some() {
                    break;
                }
            }
            if definition.is_some() {
                old.read_property(entity, &definition)?;
            } else if let Some(usage) = usage.get() {
                for rank in 1..=usage.nb_identified_item() {
                    items.push(usage.identified_item_value(rank)?.cast());
                }
            }
        }
        Ok(items)
    }
    /// The label of the entity on the shapes: a dimension or a tolerance.
    fn add_gdt_label(
        &mut self,
        entity: &Handle<StandardTransient>,
        tool: &XCAFDocDimTolTool,
        first: &[TdfLabel],
        second: &[TdfLabel],
        collected: &CollectedItems,
    ) -> Result<TdfLabel, Failure> {
        let ascii_name = |text: &str| ExtendedString::from_ascii(&AsciiString::new(text));
        if entity.cast::<StepDimTolGeometricTolerance>().is_null() {
            let label = tool.add_dimension()?;
            self.gdt_map.bind(entity.clone(), label.clone());
            tool.lock(&label)?;
            XCAFDocDimension::set(&label)?;
            let kind = if entity.cast::<StepShapeDimensionalSize>().is_some() {
                "DGT:Dimensional_Size"
            } else {
                "DGT:Dimensional_Location"
            };
            TDataStdName::set(&label, &ascii_name(kind))?;
            tool.set_dimension_shapes(first, second, &label)?;
            return Ok(label);
        }
        let label = tool.add_geom_tolerance()?;
        self.gdt_map.bind(entity.clone(), label.clone());
        tool.lock(&label)?;
        let attribute = XCAFDocGeomTolerance::set(&label)?;
        TDataStdName::set(&label, &ascii_name("DGT:GeomTolerance"))?;
        tool.set_geom_tolerance_shapes(first, &label)?;
        let attribute = step_live(&attribute, WHERE)?;
        let object = attribute.get_object()?;
        let live_object = step_live(&object, WHERE)?;
        if collected.is_all_around {
            live_object.add_modifier(XCAFDimTolObjectsGeomToleranceModif::AllAround);
        } else if collected.is_all_over {
            live_object.add_modifier(XCAFDimTolObjectsGeomToleranceModif::AllOver);
        }
        attribute.set_object(&object)?;
        Ok(label)
    }
    /// `createGDTObjectInXCAF`: the label of a dimension or a tolerance; null without shapes.
    pub(in crate::stepcaf_control) fn create_gdt_object_in_xcaf(
        &mut self,
        entity: &Handle<StandardTransient>,
        doc: &Handle<TDocStdDocument>,
        ws: &Handle<XSControlWorkSession>,
        factors: &StepDataFactors,
    ) -> Result<TdfLabel, Failure> {
        // protection against invalid input
        let Some(semantic_name) = semantic_name_of(entity) else {
            return Ok(TdfLabel::default());
        };
        let main = step_live(doc, WHERE)?.main()?;
        let shape_tool = XCAFDocDocumentTool::shape_tool(&main)?;
        let dim_tol_tool = XCAFDocDocumentTool::dim_tol_tool(&main)?;
        let live_tool = step_live(&dim_tol_tool, WHERE)?;
        let session = SessionGraph::of_process(ws)?;
        let old = OldVersion {
            session: &session,
            shape_tool: &shape_tool,
            dim_tol_tool: live_tool,
            factors,
        };
        let mut collected = CollectedItems::default();
        // Collect all Shape_Aspect entities
        for attribute in session.shareds(entity)? {
            let aspects = Self::aspects_of_attribute(&session, &attribute, &mut collected)?;
            // Collect all representation items
            let items = Self::items_of_aspects(&old, entity, &aspects)?;
            if !items.is_empty() {
                if collected.first.is_empty() {
                    collected.first = items;
                } else {
                    collected.second = items;
                }
            }
        }
        if collected.first.is_empty() {
            return Ok(TdfLabel::default());
        }
        // Collect shapes
        let live_shape_tool = step_live(&shape_tool, WHERE)?;
        let first = labels_of_items(&collected.first, &session, live_shape_tool)?;
        // for dimensional location
        let second = labels_of_items(&collected.second, &session, live_shape_tool)?;
        if first.is_empty() {
            return Ok(TdfLabel::default());
        }
        // add to XCAF
        let label = self.add_gdt_label(entity, live_tool, &first, &second, &collected)?;
        if let Some(name) = semantic_name.get() {
            TDataStdName::set(&label, &ExtendedString::from_ascii(&name.string()))?;
        }
        self.read_datums_ap242(entity, &label, doc, ws, factors)?;
        Ok(label)
    }
}
