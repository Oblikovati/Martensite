//! The commands of the format: `igesbrep`, `testreadiges`, `brepiges`, `testwriteiges` and `tpcompound`,
//! on the session the scripts share. What such a command itself writes reaches the stream when it
//! returns, after the messages of the translation.
use crate::support::{script_error::script_error, session::Session};
use crate::xs_session::XsSession;
use martensite_tkdeiges::iges_control::{IgesControlReader, IgesControlWriter};
use martensite_tkernel::{message::ProgressRange, standard::Failure, tcollection::AsciiString};
use martensite_tkxsbase::{
    if_select::IFSelectReturnStatus,
    interface::{InterfaceInterfaceModel, InterfaceModelEntities, InterfaceStatic},
    xs_algo::XSAlgoShapeProcessor,
    xs_control::XSControlReaderOps,
};

fn text_of(value: &AsciiString) -> String {
    String::from_utf8_lossy(value.as_bytes()).into_owned()
}

fn static_text(key: &str) -> String {
    text_of(&InterfaceStatic::c_val(key))
}

fn set_read_processing(reader: &mut IgesControlReader) -> Result<(), Failure> {
    let (parameters, flags) = XSAlgoShapeProcessor::read_processing_data(
        &AsciiString::from("read.iges.resource.name"),
        &AsciiString::from("read.iges.sequence"),
    )?;
    reader.xs_reader().set_shape_fix_parameters(parameters);
    reader.xs_reader().set_shape_process_flags(flags);
    Ok(())
}

fn set_write_processing(writer: &mut IgesControlWriter) -> Result<(), Failure> {
    let (parameters, flags) = XSAlgoShapeProcessor::read_processing_data(
        &AsciiString::from("write.iges.resource.name"),
        &AsciiString::from("write.iges.sequence"),
    )?;
    writer.set_shape_fix_parameters(parameters);
    writer.set_shape_process_flags(flags);
    Ok(())
}

impl XsSession {
    /// `igesbrep file name selection`, `igesread` being the same command; returns what it wrote.
    /// Example: `session.igesbrep("igesbrep", "a.igs", "a", "*")`.
    pub fn igesbrep(&mut self, command: &str, file: &str, name: &str, selection: &str) -> String {
        println!("{command} {file} {name} {selection} ");
        let mut result = String::new();
        let outcome = self.igesbrep_inner(file, name, selection, &mut result);
        print!("{result}");
        match outcome {
            Ok(true) => result,
            Ok(false) => script_error(&format!("{command} {file}")),
            Err(failure) => script_error(&format!("{failure:?}")),
        }
    }

    fn igesbrep_inner(
        &mut self,
        file: &str,
        name: &str,
        selection: &str,
        result: &mut String,
    ) -> Result<bool, Failure> {
        self.select_iges_norm()?;
        let mut reader = IgesControlReader::with_session(&self.work_session, false)?;
        let session = self.work_session.get().expect("a work session");
        session.set_mode_stat(true);
        *result += &format!(" File IGES to read : {file}\n");
        *result += &format!(" -- Names of variables prefixed by : {name}\n");
        if reader.read_file(file)? != IFSelectReturnStatus::RetDone {
            *result += &format!("Could not read file {file} , abandon\n");
            return Ok(false);
        }
        set_read_processing(&mut reader)?;
        if selection == "*" {
            self.transfer_all(&mut reader, name, result)?;
            return Ok(true);
        }
        self.transfer_selection(&mut reader, name, selection, result)?;
        Ok(true)
    }

    fn transfer_all(
        &mut self,
        reader: &mut IgesControlReader,
        name: &str,
        result: &mut String,
    ) -> Result<(), Failure> {
        let continuity = InterfaceStatic::i_val("read.iges.bspline.continuity");
        *result += "All Geometry Transfer\n";
        *result +=
            &format!("spline_continuity (read) : {continuity} (0 : no modif, 1 : C1, 2 : C2)\n");
        *result += "  To modify : command  param read.iges.bspline.continuity\n";
        self.work_session
            .get()
            .expect("a work session")
            .clear_context()?;
        reader.set_read_visible(true);
        reader.transfer_roots(&ProgressRange::default())?;
        let shape = reader.xs_reader().one_shape();
        *result += &format!("Saving shape in variable : {name}\n");
        self.raster.bind(name, &shape);
        Ok(())
    }

    fn transfer_selection(
        &mut self,
        reader: &mut IgesControlReader,
        name: &str,
        selection: &str,
        result: &mut String,
    ) -> Result<(), Failure> {
        let session = self.work_session.get().expect("a work session");
        let list = if selection == "*r" {
            *result += "All Roots : ";
            session.give_list_texts("xst-model-roots", "")?
        } else {
            *result += &format!("List given by {selection} : ");
            session.give_list_texts(selection, "")?
        };
        let Some(list) = list else {
            *result +=
                "No list defined. Give a selection name or * for all visible transferable roots\n";
            return Ok(());
        };
        *result += &format!("Nb entities selected : {}\n", list.len());
        if list.is_empty() {
            return Ok(());
        }
        let model = reader.xs_reader().model()?;
        let mut produced = 0;
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
            produced += 1;
        }
        *result += &format!("Nb Shapes successfully produced : {produced}\n");
        Ok(())
    }

    /// `testreadiges file name`: a reader of its own, on a model of its own.
    /// Example: `session.testreadiges("a.igs", "a")`.
    pub fn testreadiges(&mut self, file: &str, name: &str) {
        println!("testreadiges {file} {name} ");
        let read = || -> Result<Option<(martensite_tkbrep::topods::Shape, i32)>, Failure> {
            let mut reader = IgesControlReader::new()?;
            if reader.read_file(file)? != IFSelectReturnStatus::RetDone {
                return Ok(None);
            }
            set_read_processing(&mut reader)?;
            reader.transfer_roots(&ProgressRange::default())?;
            Ok(Some((
                reader.xs_reader().one_shape(),
                reader.xs_reader().nb_shapes(),
            )))
        };
        let outcome = read().unwrap_or_else(|failure| script_error(&format!("{failure:?}")));
        let Some((shape, count)) = outcome else {
            println!("Status from reading IGES file {file} : file not found");
            script_error(&format!("testreadiges {file}"));
        };
        self.raster.bind(name, &shape);
        println!("Status from reading IGES file {file} : file read");
        println!("Count of shapes produced : {count}");
    }

    /// `brepiges name file`, in the file mode and unit of the settings; the model stays in the session.
    /// Example: `session.brepiges("a", "a.igs")`.
    pub fn brepiges(&mut self, name: &str, file: &str) {
        println!("brepiges {name} {file} ");
        let mut result = String::new();
        let outcome = self.brepiges_inner(name, file, &mut result);
        print!("{result}");
        match outcome {
            Ok(true) => {}
            Ok(false) => script_error(&format!("brepiges {file}")),
            Err(failure) => script_error(&format!("{failure:?}")),
        }
    }

    fn brepiges_inner(
        &mut self,
        name: &str,
        file: &str,
        result: &mut String,
    ) -> Result<bool, Failure> {
        self.select_iges_norm()?;
        let unit = static_text("write.iges.unit");
        let mode = InterfaceStatic::i_val("write.iges.brep.mode");
        let mut writer = IgesControlWriter::with_unit(unit.as_bytes(), mode)?;
        *result += &format!("unit (write) : {unit}\n");
        *result += &format!("mode  write  : {}\n", static_text("write.iges.brep.mode"));
        *result += "  To modify : command  param\n";
        set_write_processing(&mut writer)?;
        let shape = self.bound_shape(name);
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
            return Ok(false);
        }
        *result += &format!(" File {file} written\n");
        let generic: martensite_tkernel::standard::Handle<dyn InterfaceInterfaceModel> =
            model.cast();
        self.work_session
            .get()
            .expect("a work session")
            .set_model(&generic, false)?;
        Ok(true)
    }

    /// `testwriteiges file name`: a writer of its own, on a model of its own.
    /// Example: `session.testwriteiges("a.igs", "a")`.
    pub fn testwriteiges(&mut self, file: &str, name: &str) {
        println!("testwriteiges {file} {name} ");
        let shape = self.bound_shape(name);
        let write = || -> Result<&'static str, Failure> {
            let mut writer = IgesControlWriter::new()?;
            set_write_processing(&mut writer)?;
            if !writer.add_shape(&shape, &ProgressRange::default())? {
                return Ok("Shape not add");
            }
            if !writer.write_file(file, false)? {
                return Ok("Error on writing file");
            }
            Ok("File Is Written")
        };
        let said = write().unwrap_or_else(|failure| script_error(&format!("{failure:?}")));
        println!("{said}");
        if said != "File Is Written" {
            script_error(&format!("testwriteiges {file}"));
        }
    }

    /// `tpcompound name`: all the results of the transfer in one compound.
    /// Example: `session.tpcompound("result")`.
    pub fn tpcompound(&mut self, name: &str) {
        self.pilot_command(&format!("tpcompound {name}"));
        let vars = self.work_session.get().expect("a work session").vars();
        let mut bound = name.to_owned();
        let shape = vars.get().expect("variables").get_shape(&mut bound);
        self.raster.bind(name, &shape);
    }
}
