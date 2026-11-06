//! The commands of the exchange scripts other than the document ones: `stepread`, `stepwrite`,
//! `tpcompound`, the variables, on the session they share.
use crate::de_end::locate_data_file;
use crate::support::{
    checkshape::checkshape_named,
    nbshapes::{Counting, NbCount, checknbshapes, checknbshapes_like},
    script_error::script_error,
    session::Session,
};
use crate::table::Place;
use crate::xs_session::XsSession;
use martensite_tkbrep::{topexp::Explorer, topods::Shape};
use martensite_tkdestep::{
    de_step::{DeStepParameters, StepControlStepModelType},
    step_control::{StepControlController, StepControlReader, StepControlWriter},
};
use martensite_tkernel::{message::ProgressRange, standard::Failure, tcollection::AsciiString};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkxsbase::{
    if_select::IFSelectReturnStatus, interface::InterfaceModelEntities,
    xs_algo::XSAlgoShapeProcessor, xs_control::XSControlReaderOps,
};
use std::path::PathBuf;

/// The folder a case writes in, as the case's `$imagedir`.
pub fn image_file(source: &str, name: &str) -> PathBuf {
    let folder = PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
        .join("step-cases")
        .join(source.replace('/', "_"));
    std::fs::create_dir_all(&folder).unwrap();
    folder.join(name)
}

impl XsSession {
    /// A path word of the script: the data set's file, else a script error as `locate_data_file` raises.
    /// Example: `session.path_of(Place::Data("a.stp"))`.
    pub fn path_of(&self, place: Place) -> String {
        match place {
            Place::Image(name) => image_file(&self.source, name)
                .to_string_lossy()
                .into_owned(),
            Place::Data(name) => locate_data_file(name)
                .unwrap_or_else(|| script_error(&format!("File {name} could not be found")))
                .to_string_lossy()
                .into_owned(),
        }
    }

    /// `stepread file name *`: each transferrable root as `name_N`; returns what the command wrote.
    /// Example: `session.stepread("a.stp", "a")`.
    pub fn stepread(&mut self, file: &str, name: &str) -> String {
        let mut result = String::new();
        if let Err(failure) = self.stepread_inner(file, name, &mut result) {
            print!("{result}");
            script_error(&format!("{failure:?}"));
        }
        print!("{result}");
        result
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
        self.select_step_norm()?;
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
        reader
            .xs_reader()
            .set_shape_process_flags(if self.read_healing_off {
                Default::default()
            } else {
                flags
            });
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
            self.transfer_entity(&mut reader, number, name, result)?;
        }
        Ok(())
    }

    fn transfer_entity(
        &mut self,
        reader: &mut StepControlReader,
        number: i32,
        name: &str,
        result: &mut String,
    ) -> Result<(), Failure> {
        if !reader.transfer_one(number, &ProgressRange::default())? {
            *result += &format!("Transfer entity n0 {number} : no result\n");
            return Ok(());
        }
        let shapes = reader.xs_reader().nb_shapes();
        let bound = format!("{name}_{shapes}");
        *result += &format!("Transfer entity n0 {number} OK  -> Shape: {bound}\n");
        *result += &format!("Now, {shapes} Shapes produced\n");
        self.raster.bind(&bound, &reader.xs_reader().shape(shapes)?);
        Ok(())
    }

    /// `stepwrite mode shape file`: as the case's mode word names it.
    /// Example: `session.stepwrite("a", "result", "out.stp")`.
    pub fn stepwrite(&mut self, mode: &str, shape_name: &str, file: &str) {
        let mut result = String::new();
        if let Err(failure) = self.stepwrite_inner(mode, shape_name, file, &mut result) {
            print!("{result}");
            script_error(&format!("{failure:?}"));
        }
        print!("{result}");
    }

    /// `stepwrite mode shape`, without a file: the translation stays in the session's model.
    /// Example: `session.stepwrite_to_model("a", "b")`.
    pub fn stepwrite_to_model(&mut self, mode: &str, shape_name: &str) {
        let mut result = String::new();
        if let Err(failure) = self.stepwrite_inner(mode, shape_name, "", &mut result) {
            print!("{result}");
            script_error(&format!("{failure:?}"));
        }
        print!("{result}");
    }

    fn stepwrite_inner(
        &mut self,
        mode: &str,
        shape_name: &str,
        file: &str,
        result: &mut String,
    ) -> Result<(), Failure> {
        self.select_step_norm()?;
        let kind = write_mode(mode);
        let shape = self.bound_shape(shape_name);
        let writer = StepControlWriter::with_session(&self.work_session, false)?;
        let parameters_and_flags = XSAlgoShapeProcessor::read_processing_data(
            &AsciiString::from("write.step.resource.name"),
            &AsciiString::from("write.step.sequence"),
        )?;
        writer.set_shape_fix_parameters(parameters_and_flags.0)?;
        writer.set_shape_process_flags(parameters_and_flags.1)?;
        let status = writer.transfer(&shape, kind, true, &ProgressRange::default())?;
        if status != IFSelectReturnStatus::RetDone {
            *result += &format!("Error: translation failed, status = {}\n", status as i32);
            return Ok(());
        }
        *result += "Translation: OK\n";
        if file.is_empty() {
            *result += " Now, to write a file, command : writeall filename\n";
            return Ok(());
        }
        *result += &match writer.write(file)? {
            IFSelectReturnStatus::RetVoid => "Error: No file written\n".to_owned(),
            IFSelectReturnStatus::RetDone => format!("File {file} written\n"),
            IFSelectReturnStatus::RetStop => {
                "Error on writing file: no space on disk or destination is write protected\n"
                    .to_owned()
            }
            _ => format!("Error: File {file} written with fail messages\n"),
        };
        Ok(())
    }

    /// `testreadstep file name`: a reader of its own, on a model of its own.
    /// Example: `session.testreadstep("a.stp", "a")`.
    pub fn testreadstep(&mut self, file: &str, name: &str) {
        self.testread(file, name, false);
    }

    /// `testreadstep file name -stream`: the same, from a stream under the file's short name.
    /// Example: `session.testreadstream("a.stp", "a")`.
    pub fn testreadstream(&mut self, file: &str, name: &str) {
        self.testread(file, name, true);
    }

    fn testread(&mut self, file: &str, name: &str, from_stream: bool) {
        let read = || -> Result<Option<(Shape, i32)>, Failure> {
            StepControlController::init()?;
            let mut parameters = DeStepParameters::default();
            parameters.init_from_static();
            let mut reader = StepControlReader::new()?;
            let (fix, flags) = XSAlgoShapeProcessor::read_processing_data(
                &AsciiString::from("read.step.resource.name"),
                &AsciiString::from("read.step.sequence"),
            )?;
            reader.xs_reader().set_shape_fix_parameters(fix);
            reader.xs_reader().set_shape_process_flags(flags);
            let status = if from_stream {
                let bytes = std::fs::read(file)
                    .map_err(|cause| Failure::new(&format!("{file}: {cause}")))?;
                let short = file.rsplit('/').next().unwrap_or(file);
                reader.read_stream_with(short, &parameters, &mut bytes.as_slice())?
            } else {
                reader.read_file_with(file, &parameters)?
            };
            if status != IFSelectReturnStatus::RetDone {
                return Ok(None);
            }
            reader.transfer_roots(&ProgressRange::default())?;
            Ok(Some((
                reader.xs_reader().one_shape(),
                reader.xs_reader().nb_shapes(),
            )))
        };
        match read().unwrap_or_else(|failure| panic!("testreadstep {file}: {failure:?}")) {
            Some((shape, count)) => {
                self.raster.bind(name, &shape);
                println!("Count of shapes produced : {count}");
            }
            None => println!(
                "Error: Problem with reading shape by file: [{file}]\nCount of shapes produced : 0"
            ),
        }
    }

    /// `testwritestep file shape`: a writer of its own, on a model of its own.
    /// Example: `session.testwritestep("a.stp", "a")`.
    pub fn testwritestep(&mut self, file: &str, shape_name: &str) {
        self.testwrite(file, shape_name, false);
    }

    /// `testwritestep file shape -stream`: the same, through a stream.
    /// Example: `session.testwritestream("a.stp", "a")`.
    pub fn testwritestream(&mut self, file: &str, shape_name: &str) {
        self.testwrite(file, shape_name, true);
    }

    fn testwrite(&mut self, file: &str, shape_name: &str, to_stream: bool) {
        let shape = self.bound_shape(shape_name);
        let parameters_of_the_writer = || -> Result<StepControlWriter, Failure> {
            let writer = StepControlWriter::new()?;
            let (fix, flags) = XSAlgoShapeProcessor::read_processing_data(
                &AsciiString::from("write.step.resource.name"),
                &AsciiString::from("write.step.sequence"),
            )?;
            writer.set_shape_fix_parameters(fix)?;
            writer.set_shape_process_flags(flags)?;
            Ok(writer)
        };
        let write = || -> Result<String, Failure> {
            let mut parameters = DeStepParameters::default();
            parameters.init_from_static();
            let writer = parameters_of_the_writer()?;
            let status = writer.transfer_with(
                &shape,
                StepControlStepModelType::AS_IS,
                &parameters,
                true,
                &ProgressRange::default(),
            )?;
            if status != IFSelectReturnStatus::RetDone {
                return Ok("Error: Can't transfer input shape".to_owned());
            }
            let written = if to_stream {
                let mut stream = std::fs::File::create(file)
                    .map_err(|cause| Failure::new(&format!("{file}: {cause}")))?;
                writer.write_stream(&mut stream)?
            } else {
                writer.write(file)?
            };
            Ok(if written == IFSelectReturnStatus::RetDone {
                String::new()
            } else {
                format!("Error on writing file: [{file}]")
            })
        };
        let complaint =
            write().unwrap_or_else(|failure| panic!("testwritestep {file}: {failure:?}"));
        // The command's text comes back after the translation's messages, a complaint first and on the same line.
        println!("{complaint}File(s) are Written");
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

    /// `renamevar from to`. Example: `session.renamevar("a_1", "a")`.
    pub fn renamevar(&mut self, from: &str, to: &str) {
        let shape = self.bound_shape(from);
        self.raster.unbind(from);
        self.raster.bind(to, &shape);
    }

    /// `copy from to`. Example: `session.copy_variable("a", "b")`.
    pub fn copy_variable(&mut self, from: &str, to: &str) {
        let shape = self.bound_shape(from);
        self.raster.bind(to, &shape);
    }

    /// `explode name kind`: the sub-shapes of a kind (or the direct children) as `name_N`; their count.
    /// Example: `session.explode("a", "f")`.
    pub fn explode(&mut self, name: &str, kind: &str) -> usize {
        let shape = self.bound_shape(name);
        let parts = parts_of(&shape, kind);
        for (index, part) in parts.iter().enumerate() {
            self.raster.bind(&format!("{name}_{}", index + 1), part);
        }
        parts.len()
    }

    /// `explode name kind`, echoed with the names it binds, as the command writes them.
    /// Example: `session.explode_echoed("a", "f")`.
    pub fn explode_echoed(&mut self, name: &str, kind: &str) -> usize {
        println!("explode {name} {kind} ");
        let count = self.explode(name, kind);
        let names: Vec<String> = (1..=count)
            .map(|number| format!("{name}_{number}"))
            .collect();
        println!("{} ", names.join(" "));
        count
    }

    /// `checkshape name`. Example: `session.check_shape("result")`.
    pub fn check_shape(&self, name: &str) {
        checkshape_named(&self.raster, name);
    }

    /// `checkshape name f`'s line: a shape without faults.
    /// Example: `session.check_short("res")`.
    pub fn check_short(&self, name: &str) -> String {
        crate::support::structural_dump::structural_check(&self.bound_shape(name), name, "f")
            .unwrap_or_else(|failure| panic!("checkshape exception : {failure:?}"))
    }

    /// `checknbshapes name -kind count ...`. Example: `session.check_counts("r", &["-solid", "4"])`.
    pub fn check_counts(&self, name: &str, words: &[&str]) {
        let counts: Vec<NbCount> = words
            .chunks(2)
            .map(|pair| count_of(pair[0], pair[1]))
            .collect();
        checknbshapes(&self.raster, name, Counting::Located, &counts);
    }

    /// `checknbshapes a -ref [nbshapes b]`. Example: `session.check_counts_like("a", "b")`.
    pub fn check_counts_like(&self, name: &str, other: &str) {
        checknbshapes_like(&self.raster, name, other);
    }
}

fn parts_of(shape: &Shape, kind: &str) -> Vec<Shape> {
    let kind = match kind.to_ascii_lowercase().as_str() {
        "f" => ShapeEnum::Face,
        "e" => ShapeEnum::Edge,
        "v" => ShapeEnum::Vertex,
        "w" => ShapeEnum::Wire,
        "sh" => ShapeEnum::Shell,
        "so" => ShapeEnum::Solid,
        "" => return children_of(shape),
        other => panic!("explode: {other}; expected f e v w sh so"),
    };
    // `explode` lists a sub-shape once, in the order the explorer meets it.
    let mut parts: Vec<Shape> = Vec::new();
    let mut explorer = Explorer::new(shape, kind, ShapeEnum::Shape).expect("an explorer");
    while explorer.more() {
        let part = explorer.current().clone();
        if !parts.iter().any(|known| known.is_same(&part)) {
            parts.push(part);
        }
        explorer.next().expect("an explorer step");
    }
    parts
}

fn children_of(shape: &Shape) -> Vec<Shape> {
    let mut parts = Vec::new();
    let mut children = martensite_tkbrep::topods::ShapeIterator::new(shape, true, true);
    while children.more() {
        parts.push(children.value().expect("a child").clone());
        children.next();
    }
    parts
}

fn write_mode(word: &str) -> StepControlStepModelType {
    match word.as_bytes().first() {
        Some(b'a' | b'0') => StepControlStepModelType::AS_IS,
        Some(b'f' | b'1') => StepControlStepModelType::FACETED_BREP,
        Some(b's' | b'2') => StepControlStepModelType::SHELL_BASED_SURFACE_MODEL,
        Some(b'm' | b'3') => StepControlStepModelType::MANIFOLD_SOLID_BREP,
        Some(b'w' | b'4') => StepControlStepModelType::GEOMETRIC_CURVE_SET,
        _ => panic!("stepwrite: {word}; expected a mode in fsmw"),
    }
}

fn count_of(word: &str, value: &str) -> NbCount {
    let count: usize = value
        .parse()
        .unwrap_or_else(|_| panic!("checknbshapes: {value}; expected a count"));
    match word {
        "-vertex" => NbCount::Vertices(count),
        "-edge" => NbCount::Edges(count),
        "-wire" => NbCount::Wires(count),
        "-face" => NbCount::Faces(count),
        "-shell" => NbCount::Shells(count),
        "-solid" => NbCount::Solids(count),
        "-compsolid" => NbCount::CompSolids(count),
        "-compound" => NbCount::Compounds(count),
        "-shape" => NbCount::Shapes(count),
        other => panic!("checknbshapes: {other}; expected a kind"),
    }
}

/// `stepfileunits file`: the units of a file; what it writes, returned too.
/// Example: `stepfileunits("a.stp")`.
pub fn stepfileunits(file: &str) -> String {
    let mut reader = StepControlReader::new().expect("a reader");
    assert!(
        reader.read_file(file).expect("a read") == IFSelectReturnStatus::RetDone,
        "stepfileunits {file}: expected a model"
    );
    let units = reader
        .file_units()
        .unwrap_or_else(|failure| panic!("stepfileunits {file}: {failure:?}"));
    let rule = "=====================================================\n";
    let mut text = String::new();
    for (title, names) in [
        ("LENGTH Unit", &units[0]),
        ("Angle Unit", &units[1]),
        ("Solid Angle Unit", &units[2]),
    ] {
        text += rule;
        text += &format!("{title}\n");
        for name in names {
            text += &format!("{}\n", String::from_utf8_lossy(name.as_bytes()));
        }
    }
    print!("{text}");
    text
}
