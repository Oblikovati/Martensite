//! The translator commands of the hand-written scripts: `igesbrep`, `brepiges`, `stepread`, `stepwrite`,
//! on the session they share. What a command itself writes reaches the stream when it returns, after
//! the messages of the translation.
use crate::iges_session::{IgesSession, locate_data_file};
use crate::support::{script_error::script_error, session::Session};
use martensite_tkbrep::{topexp::Explorer, topods::Shape};
use martensite_tkdeiges::iges_control::{IgesControlReader, IgesControlWriter};
use martensite_tkdestep::{
    de_step::StepControlStepModelType,
    step_control::{StepControlReader, StepControlWriter},
};
use martensite_tkernel::{message::ProgressRange, standard::Failure, tcollection::AsciiString};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkxsbase::{
    if_select::IFSelectReturnStatus,
    interface::{InterfaceModelEntities, InterfaceStatic},
    xs_algo::XSAlgoShapeProcessor,
    xs_control::XSControlReaderOps,
};

/// A word of a script that may be `[locate_data_file name]`, as the path it stands for.
/// Example: `data_path("BUC50025.igs")`.
pub fn data_path(name: &str) -> String {
    locate_data_file(name)
        .unwrap_or_else(|| script_error(&format!("File {name} could not be found")))
        .to_string_lossy()
        .into_owned()
}

impl IgesSession {
    /// `explode name kind`: the sub-shapes of a kind as `name_N`, each once, in the order the explorer
    /// meets it; their count. Example: `session.explode("a", "F")`.
    pub fn explode(&mut self, name: &str, kind: &str) -> usize {
        let kind_of = match kind {
            "F" => ShapeEnum::Face,
            "V" => ShapeEnum::Vertex,
            "Sh" => ShapeEnum::Shell,
            "So" => ShapeEnum::Solid,
            other => panic!("explode: {other}; expected F V Sh So"),
        };
        let shape = self.bound_shape(name);
        let mut parts: Vec<Shape> = Vec::new();
        let mut explorer = Explorer::new(&shape, kind_of, ShapeEnum::Shape).expect("an explorer");
        while explorer.more() {
            let part = explorer.current().clone();
            if !parts.iter().any(|known| known.is_same(&part)) {
                parts.push(part);
            }
            explorer.next().expect("an explorer");
        }
        for (index, part) in parts.iter().enumerate() {
            self.raster.bind(&format!("{name}_{}", index + 1), part);
        }
        parts.len()
    }

    /// `igesbrep file name *`: all the visible roots, in one shape under `name`.
    /// Example: `session.igesbrep("a.igs", "b")`.
    pub fn igesbrep(&mut self, file: &str, name: &str) {
        println!("igesbrep {file} {name} * ");
        let mut result = String::new();
        let outcome = self.igesbrep_inner(file, name, &mut result);
        print!("{result}");
        if let Err(failure) = outcome {
            script_error(&format!("{failure:?}"));
        }
    }

    fn igesbrep_inner(
        &mut self,
        file: &str,
        name: &str,
        result: &mut String,
    ) -> Result<(), Failure> {
        self.select_norm("IGES")?;
        let mut reader = IgesControlReader::with_session(&self.work_session, false)?;
        *result +=
            &format!(" File IGES to read : {file}\n -- Names of variables prefixed by : {name}\n");
        if reader.read_file(file)? != IFSelectReturnStatus::RetDone {
            *result += &format!("Could not read file {file} , abandon\n");
            script_error(result);
        }
        let (parameters, flags) = XSAlgoShapeProcessor::read_processing_data(
            &AsciiString::from("read.iges.resource.name"),
            &AsciiString::from("read.iges.sequence"),
        )?;
        reader.xs_reader().set_shape_fix_parameters(parameters);
        reader.xs_reader().set_shape_process_flags(flags);
        *result += "All Geometry Transfer\n";
        *result += &format!(
            "spline_continuity (read) : {} (0 : no modif, 1 : C1, 2 : C2)\n",
            InterfaceStatic::i_val("read.iges.bspline.continuity")
        );
        *result += "  To modify : command  param read.iges.bspline.continuity\n";
        reader.set_read_visible(true);
        reader.transfer_roots(&ProgressRange::default())?;
        let shape = reader.xs_reader().one_shape();
        *result += &format!("Saving shape in variable : {name}\n");
        self.raster.bind(name, &shape);
        Ok(())
    }

    /// `brepiges shape file`: the shape to a file; the session takes the model.
    /// Example: `session.brepiges("b", "a.igs")`.
    pub fn brepiges(&mut self, shape_name: &str, file: &str) {
        println!("brepiges {shape_name} {file} ");
        let mut result = String::new();
        let outcome = self.brepiges_inner(shape_name, file, &mut result);
        print!("{result}");
        if let Err(failure) = outcome {
            script_error(&format!("{failure:?}"));
        }
    }

    fn brepiges_inner(
        &mut self,
        shape_name: &str,
        file: &str,
        result: &mut String,
    ) -> Result<(), Failure> {
        self.select_norm("IGES")?;
        let unit = InterfaceStatic::c_val("write.iges.unit");
        let mut writer = IgesControlWriter::with_unit(
            unit.as_bytes(),
            InterfaceStatic::i_val("write.iges.brep.mode"),
        )?;
        *result += &format!(
            "unit (write) : {}\n",
            String::from_utf8_lossy(unit.as_bytes())
        );
        *result += &format!(
            "mode  write  : {}\n",
            String::from_utf8_lossy(InterfaceStatic::c_val("write.iges.brep.mode").as_bytes())
        );
        *result += "  To modify : command  param\n";
        let (parameters, flags) = XSAlgoShapeProcessor::read_processing_data(
            &AsciiString::from("write.iges.resource.name"),
            &AsciiString::from("write.iges.sequence"),
        )?;
        writer.set_shape_fix_parameters(parameters);
        writer.set_shape_process_flags(flags);
        let shape = self.bound_shape(shape_name);
        let added = writer.add_shape(&shape, &ProgressRange::default())?;
        writer.compute_model()?;
        let model = writer.model();
        let entities = model.get().expect("a model").nb_entities();
        *result += &format!(
            "{} Shapes written, giving {entities} Entities\n",
            i32::from(added)
        );
        if !writer.write_file(file, false)? {
            *result += &format!(" Error: could not write file {file}\n");
            script_error(result);
        }
        *result += &format!(" File {file} written\n");
        self.work_session
            .get()
            .expect("a work session")
            .set_model(&model.cast(), true)?;
        Ok(())
    }

    /// `stepwrite a shape file`. Example: `session.stepwrite("b", "a.stp")`.
    pub fn stepwrite(&mut self, shape_name: &str, file: &str) {
        println!("stepwrite a {shape_name} {file} ");
        let mut result = String::new();
        let outcome = self.stepwrite_inner(shape_name, file, &mut result);
        print!("{result}");
        if let Err(failure) = outcome {
            script_error(&format!("{failure:?}"));
        }
    }

    fn stepwrite_inner(
        &mut self,
        shape_name: &str,
        file: &str,
        result: &mut String,
    ) -> Result<(), Failure> {
        self.select_norm("STEP")?;
        let shape = self.bound_shape(shape_name);
        let writer = StepControlWriter::with_session(&self.work_session, false)?;
        let (parameters, flags) = XSAlgoShapeProcessor::read_processing_data(
            &AsciiString::from("write.step.resource.name"),
            &AsciiString::from("write.step.sequence"),
        )?;
        writer.set_shape_fix_parameters(parameters)?;
        writer.set_shape_process_flags(flags)?;
        let status = writer.transfer(
            &shape,
            StepControlStepModelType::AS_IS,
            true,
            &ProgressRange::default(),
        )?;
        if status != IFSelectReturnStatus::RetDone {
            *result += &format!("Error: translation failed, status = {}\n", status as i32);
            return Ok(());
        }
        *result += "Translation: OK\n";
        *result += &match writer.write(file)? {
            IFSelectReturnStatus::RetVoid => "Error: No file written\n".to_owned(),
            IFSelectReturnStatus::RetDone => format!("File {file} written\n"),
            _ => format!("Error: File {file} written with fail messages\n"),
        };
        Ok(())
    }

    /// `stepread file name *`: each transferrable root as `name_N`.
    /// Example: `session.stepread("a.stp", "a")`.
    pub fn stepread(&mut self, file: &str, name: &str) {
        println!("stepread {file} {name} * ");
        let mut result = String::new();
        let outcome = self.stepread_inner(file, name, &mut result);
        print!("{result}");
        if let Err(failure) = outcome {
            script_error(&format!("{failure:?}"));
        }
    }

    fn stepread_inner(
        &mut self,
        file: &str,
        name: &str,
        result: &mut String,
    ) -> Result<(), Failure> {
        println!("Reduced model for translation without additional info will be used ");
        *result +=
            &format!(" File STEP to read : {file}\n -- Names of variables prefixed by : {name}\n");
        self.select_norm("STEP")?;
        let mut reader = StepControlReader::with_session(&self.work_session, false)?;
        if reader.read_file(file)? != IFSelectReturnStatus::RetDone {
            *result += &format!("Could not read file {file} , abandon\n");
            script_error(result);
        }
        let (parameters, flags) = XSAlgoShapeProcessor::read_processing_data(
            &AsciiString::from("read.step.resource.name"),
            &AsciiString::from("read.step.sequence"),
        )?;
        reader.xs_reader().set_shape_fix_parameters(parameters);
        reader.xs_reader().set_shape_process_flags(flags);
        // The selection reads the roots this count records in the transfer process.
        reader.nb_roots_for_transfer()?;
        *result += "Transferrable Roots : ";
        let list = self
            .work_session
            .get()
            .expect("a work session")
            .give_list_texts("xst-transferrable-roots", "")?;
        let Some(list) = list else {
            *result += "No list defined. Give a selection name or * for all transferrable roots\n";
            return Ok(());
        };
        *result += &format!("Nb entities selected : {}\n", list.len());
        let model = reader.xs_reader().model()?;
        for entity in &list {
            let number = model.get().expect("a model").number(entity);
            if number == 0 {
                continue;
            }
            if !reader.transfer_one(number, &ProgressRange::default())? {
                *result += &format!("Transfer entity n0 {number} : no result\n");
                continue;
            }
            let shapes = reader.xs_reader().nb_shapes();
            let bound = format!("{name}_{shapes}");
            *result += &format!("Transfer entity n0 {number} OK  -> Shape: {bound}\n");
            *result += &format!("Now, {shapes} Shapes produced\n");
            self.raster.bind(&bound, &reader.xs_reader().shape(shapes)?);
        }
        Ok(())
    }
}
