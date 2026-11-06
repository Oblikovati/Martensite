//! The transfer of `STEPCAFControlReader`.
use super::*;

impl ReaderState {
    /// The shapes resulting from each product definition and shape representation of the model, to
    /// distinguish compounds representing assemblies from hybrid models and shape sets.
    fn collect_products(
        reader: &StepControlReader,
        model: &Handle<StepDataStepModel>,
        shapes: &MapOfShape,
    ) -> Result<TransferredProducts, Failure> {
        let mut made = TransferredProducts {
            new_shapes: MapOfShape::default(),
            shape_pd: DataMap::default(),
            pd_file: DataMap::default(),
            seq_pds: Handle::new(HSequence::default()),
        };
        let ws = reader.xs_reader().ws();
        let tr = step_live(&ws, WHERE)?.transfer_reader();
        let tp = step_live(&tr, WHERE)?.transient_process();
        let tp = step_live(&tp, WHERE)?;
        let model = step_live(model, WHERE)?;
        for i in 1..=model.nb_entities() {
            let entity = model.value(i)?;
            step_live(&entity, WHERE)?;
            if entity.cast::<StepReprProductDefinitionShape>().is_some() {
                // sequence for acceleration ReadMaterials
                step_live(&made.seq_pds, WHERE)?.append(entity.clone());
            }
            let pd = entity.cast::<StepBasicProductDefinition>();
            let is_sr = entity.cast::<StepShapeShapeRepresentation>().is_some();
            if pd.is_null() && !is_sr {
                continue;
            }
            let index = tp.map_index(&entity);
            if index <= 0 {
                continue;
            }
            let shape = TransferBRep::shape_result(&tp.map_item(index)?);
            if shape.is_null() || !shapes.contains(&shape) {
                continue;
            }
            made.new_shapes.add(shape.clone());
            if pd.is_some() {
                made.shape_pd.bind(shape, pd.clone());
                made.pd_file.bind(pd, Handle::default());
            }
        }
        Ok(made)
    }
    /// Reads the external references of the current transfer, with the map SDR -> extern file.
    fn load_extern_refs(
        &mut self,
        reader: &StepControlReader,
        doc: &Handle<TDocStdDocument>,
        products: &mut TransferredProducts,
        scope_range: &ProgressRange<'_>,
    ) -> Result<(), Failure> {
        // get file name and directory name of the main file
        let ws = reader.xs_reader().ws();
        let tr = step_live(&ws, WHERE)?.transfer_reader();
        let tp = step_live(&tr, WHERE)?.transient_process();
        let mut mainfile = CadPath::new(step_live(&ws, WHERE)?.loaded_file().as_bytes());
        let main_name = format!(
            "{}{}",
            String::from_utf8_lossy(mainfile.name().as_bytes()),
            String::from_utf8_lossy(mainfile.extension().as_bytes())
        );
        mainfile.set_name("");
        mainfile.set_extension("");
        let dpath = mainfile.system_name();
        // Load external references (only for relevant SDRs)
        let mut ext_refs = StepConstructExternRefs::with_ws(&ws)?;
        ext_refs.load_extern_refs()?;
        let count = ext_refs.nb_extern_refs();
        let scope = ProgressScope::unnamed(scope_range, f64::from(count), false);
        let mut i = 1;
        while i <= count && scope.more() {
            let range = scope.next(1.0);
            let num = i;
            i += 1;
            // get and check filename of the current extern ref
            let filename = ext_refs.file_name(num)?;
            if filename.is_empty() {
                continue; // not a valid extern ref
            }
            // compute true path to the extern file
            let mut fullname = CadPath::absolute_path(dpath.as_bytes(), filename.as_bytes())?;
            if fullname.is_empty() {
                fullname = AsciiString::new(&filename);
            }
            // check for not the same file
            let main_full_name = CadPath::absolute_path(dpath.as_bytes(), main_name.as_bytes())?;
            if main_full_name
                .as_bytes()
                .eq_ignore_ascii_case(fullname.as_bytes())
            {
                step_live(&tp, WHERE)?.add_warning_text(
                    &ext_refs.doc_file(num)?.cast(),
                    "External reference file is the same main file",
                    "",
                );
                continue; // not a valid extern ref
            }
            // get and check PD associated with the current extern ref
            let pd = ext_refs.prod_def(num)?;
            if pd.is_null() || !products.pd_file.is_bound(&pd) {
                continue; // or this PD is not concerned by current transfer
            }
            // read extern file (or use existing data) and record its data
            let file = self.read_extern_file(&filename, fullname.as_bytes(), doc, &range)?;
            products.pd_file.bind(pd, file);
        }
        Ok(())
    }
    /// `transfer(rd, num, doc, Lseq, asOne)`: the roots of a reader, then the data of each mode.
    pub(in crate::stepcaf_control) fn transfer(
        &mut self,
        reader: &mut StepControlReader,
        nroot: i32,
        doc: &Handle<TDocStdDocument>,
        labels: &mut Vec<TdfLabel>,
        as_one: bool,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        reader.xs_reader_mut().clear_shapes();
        let model = reader.step_model()?;
        let mut factors = StepDataFactors::default();
        Self::prepare_units(&model, doc, &mut factors)?;
        // Read all shapes
        let roots = reader.nb_roots_for_transfer()?;
        if roots <= 0 {
            return Ok(false);
        }
        let root_scope = ProgressScope::unnamed(progress, 2.0, false);
        if !Self::transfer_roots(reader, nroot, roots, &root_scope)? {
            return Ok(false);
        }
        let count = reader.xs_reader().nb_shapes();
        if count <= 0 {
            return Ok(false);
        }
        // Fill a map of (top-level) shapes resulting from that transfer
        // Only these shapes will be considered further
        let mut shapes = MapOfShape::default();
        for i in 1..=count {
            fill_shapes_map(&reader.xs_reader().shape(i)?, &mut shapes)?;
        }
        let mut products = Self::collect_products(reader, &model, &shapes)?;
        let scope_range = root_scope.next(1.0);
        self.load_extern_refs(reader, doc, &mut products, &scope_range)?;
        // and insert them to the document
        let main = step_live(doc, WHERE)?.main()?;
        let tool = XCAFDocDocumentTool::shape_tool(&main)?;
        if tool.is_null() {
            return Ok(false);
        }
        if as_one {
            labels.push(self.add_shape(
                &reader.xs_reader().one_shape(),
                &tool,
                &products.new_shapes,
                &products.shape_pd,
                &products.pd_file,
            )?);
        } else {
            for i in 1..=count {
                labels.push(self.add_shape(
                    &reader.xs_reader().shape(i)?,
                    &tool,
                    &products.new_shapes,
                    &products.shape_pd,
                    &products.pd_file,
                )?);
            }
        }
        let ws = reader.xs_reader().ws();
        self.read_each_mode(&ws, doc, &products, &factors)?;
        // Expand resulting CAF structure for sub-shapes (optionally with their names) if requested
        self.expand_sub_shapes(&tool, &products.shape_pd)?;
        // Update assembly compounds
        step_live(&tool, WHERE)?.update_assemblies()?;
        Ok(true)
    }
    /// The readers of the modes that are on, in the order.
    fn read_each_mode(
        &mut self,
        ws: &Handle<XSControlWorkSession>,
        doc: &Handle<TDocStdDocument>,
        products: &TransferredProducts,
        factors: &StepDataFactors,
    ) -> Result<(), Failure> {
        if self.color_mode {
            self.read_colors(ws, doc, factors)?;
        }
        if self.name_mode {
            self.read_names(ws, doc, &products.pd_file)?;
        }
        if self.props_mode {
            self.read_val_props(ws, doc, &products.pd_file, factors)?;
        }
        if self.layer_mode {
            self.read_layers(ws, doc)?;
        }
        if self.shuo_mode {
            self.read_shuos(ws, doc, &products.pd_file)?;
        }
        if self.gdt_mode {
            self.read_gdts(ws, doc, factors)?;
        }
        if self.mat_mode {
            self.read_materials(ws, doc, &products.seq_pds, factors)?;
        }
        if self.view_mode {
            self.read_views(ws, doc, factors)?;
        }
        if self.meta_mode {
            self.read_metadata(ws, doc, factors)?;
        }
        if self.product_meta_mode {
            self.read_product_metadata(ws, doc)?;
        }
        Ok(())
    }
    /// `add_shape`: the label of a shape of the transfer in the document, adding it with its
    /// components when it is an assembly.
    fn add_shape(
        &mut self,
        shape: &Shape,
        tool: &Handle<XCAFDocShapeTool>,
        new_shapes: &MapOfShape,
        shape_pd: &ShapePdMap,
        pd_file: &PdFileMap,
    ) -> Result<TdfLabel, Failure> {
        // if shape has already been mapped, just return corresponding label
        if let Ok(known) = self.map.find(shape) {
            return Ok(known.clone());
        }
        let live_tool = step_live(tool, WHERE)?;
        // if shape is located, create instance
        if !shape.location().is_identity() {
            let mut located = shape.clone();
            located.set_location(&Location::default(), false)?;
            self.add_shape(&located, tool, new_shapes, shape_pd, pd_file)?;
            let label = live_tool.add_shape(shape, false, true)?; // should create reference
            self.map.bind(shape.clone(), label.clone());
            return Ok(label);
        }
        // if shape is not compound, simple add it
        if shape.shape_type()? != ShapeEnum::Compound {
            let label = live_tool.add_shape(shape, false, true)?;
            self.map.bind(shape.clone(), label.clone());
            return Ok(label);
        }
        self.add_compound(shape, tool, new_shapes, shape_pd, pd_file)
    }
    /// The compound of `add_shape`: a whole, an external reference, or an assembly.
    fn add_compound(
        &mut self,
        shape: &Shape,
        tool: &Handle<XCAFDocShapeTool>,
        new_shapes: &MapOfShape,
        shape_pd: &ShapePdMap,
        pd_file: &PdFileMap,
    ) -> Result<TdfLabel, Failure> {
        let live_tool = step_live(tool, WHERE)?;
        // for compounds, compute number of subshapes and check whether this is assembly
        let (mut is_assembly, mut nb_components) = (false, 0);
        let mut components = ShapeIterator::new(shape, true, true);
        while components.more() && !is_assembly {
            let mut sub = components.value()?.clone();
            sub.set_location(&Location::default(), false)?;
            is_assembly = new_shapes.contains(&sub);
            components.next();
            nb_components += 1;
        }
        // check whether it has associated external ref
        let mut names: Vec<Handle<HAsciiString>> = Vec::new();
        if let Ok(pd) = shape_pd.find(shape)
            && let Ok(file) = pd_file.find(pd)
            && let Some(file) = file.get()
        {
            // (store information on extern refs in the document)
            names.push(file.get_name());
            // if yes, just return corresponding label, but if components >0, ignore extern ref!
            let label = file.get_label();
            if !label.is_null() && nb_components <= 0 {
                self.map.bind(shape.clone(), label.clone());
                live_tool.set_extern_refs_on(&label, &names)?;
                return Ok(label);
            }
        }
        // add compound either as a whole,
        if !is_assembly {
            let label = live_tool.add_shape(shape, false, true)?;
            if !names.is_empty() {
                live_tool.set_extern_refs_on(&label, &names)?;
            }
            self.map.bind(shape.clone(), label.clone());
            return Ok(label);
        }
        // or as assembly, component-by-component
        let label = live_tool.new_shape()?;
        let mut components = ShapeIterator::new(shape, true, true);
        while components.more() {
            let current = components.value()?.clone();
            let mut sub = current.clone();
            sub.set_location(&Location::default(), false)?;
            let sub_label = self.add_shape(&sub, tool, new_shapes, shape_pd, pd_file)?;
            if !sub_label.is_null() {
                let instance = live_tool.add_component(&label, &sub_label, current.location())?;
                if !self.map.is_bound(&current) {
                    self.map.bind(current, instance);
                }
            }
            components.next();
        }
        if !names.is_empty() {
            live_tool.set_extern_refs_on(&label, &names)?;
        }
        self.map.bind(shape.clone(), label.clone());
        Ok(label)
    }
    /// `read_extern_file`: reads an external file once, in a new session.
    fn read_extern_file(
        &mut self,
        file: &str,
        fullname: &[u8],
        doc: &Handle<TDocStdDocument>,
        progress: &ProgressRange<'_>,
    ) -> Result<Handle<STEPCAFControlExternFile>, Failure> {
        // if the file is already read, associate it with SDR
        let key = AsciiString::new(file);
        if let Ok(known) = self.files.find(&key) {
            return Ok(known.clone());
        }
        // create new WorkSession and Reader
        let new_ws = XSControlWorkSession::new();
        step_live(&new_ws, WHERE)?.select_norm("STEP")?;
        let mut reader = StepControlReader::with_session(&new_ws, false)?;
        // start to fill the resulting ExternFile structure
        let extern_file = Handle::new(STEPCAFControlExternFile::new());
        let live = step_live(&extern_file, WHERE)?;
        live.set_ws(&new_ws);
        live.set_name(&Handle::new(HAsciiString::new(file)));
        // read file
        live.set_load_status(reader.read_file(&String::from_utf8_lossy(fullname))?);
        // transfer in single-result mode
        if live.get_load_status() == IFSelectReturnStatus::RetDone {
            let mut labels = Vec::new();
            live.set_transfer_status(self.transfer(
                &mut reader,
                0,
                doc,
                &mut labels,
                false,
                progress,
            )?);
            if let Some(first) = labels.first() {
                live.set_label(first);
            }
        }
        // add read file to dictionary
        self.files.bind(key, extern_file.clone());
        Ok(extern_file)
    }
}

/// `find_instance`: the instance label of a NAUO.
pub(in crate::stepcaf_control) fn find_instance_label(
    nauo: &Handle<StepReprNextAssemblyUsageOccurrence>,
    tool: &Handle<XCAFDocShapeTool>,
    tp: &Handle<martensite_tkxsbase::transfer::TransferTransientProcess>,
    shape_labels: &STEPCAFControlShapeLabelMap,
) -> Result<TdfLabel, Failure> {
    // get shape resulting from CDSR (in fact, only location is interesting)
    let live_tp = step_live(tp, WHERE)?;
    let binder = live_tp.find(&nauo.cast());
    let Some(binder_live) = binder.get() else {
        return Ok(TdfLabel::default());
    };
    if !binder_live.has_result() {
        return Ok(TdfLabel::default());
    }
    let shape = TransferBRep::shape_result(&binder);
    if shape.is_null() {
        return Ok(TdfLabel::default());
    }
    let mut label = TdfLabel::default();
    match shape_labels.find(&shape) {
        Ok(found) => label = found.clone(),
        Err(_) => {
            step_live(tool, WHERE)?.search(&shape, &mut label, [true, true, false])?;
        }
    }
    Ok(label)
}

impl STEPCAFControlReader {
    /// `find_instance`: the label of the instance a NAUO made, null when not found.
    pub fn find_instance(
        nauo: &Handle<StepReprNextAssemblyUsageOccurrence>,
        tool: &Handle<XCAFDocShapeTool>,
        construct_tool: &crate::step_construct::StepConstructTool,
        shape_labels: &STEPCAFControlShapeLabelMap,
    ) -> Result<TdfLabel, Failure> {
        find_instance_label(nauo, tool, construct_tool.transient_process(), shape_labels)
    }
}
