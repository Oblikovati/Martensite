//! What the cases share: reading text or files with the header protocol only, and the text
//! the reading gives: its status, its count, its messages and what the writer sends back.
use martensite_tkdestep::{
    header_section::HeaderSection,
    rw_header_section::RWHeaderSection,
    step_data::{StepDataStepModel, StepDataStepWriter},
    step_file::step_file_read,
};
use martensite_tkernel::standard::Handle;
use martensite_tkxsbase::interface::{InterfaceModelEntities, InterfaceModelReports};

/// A read: its status and its model.
pub struct Read {
    pub status: i32,
    pub model: Handle<StepDataStepModel>,
}

/// Reads `text` as a file.
pub fn read_text(text: &[u8]) -> Read {
    RWHeaderSection::init();
    let model = StepDataStepModel::new();
    let mut stream = text;
    let status = step_file_read(
        "text",
        Some(&mut stream),
        &model,
        &HeaderSection::protocol().cast(),
    )
    .expect("reading a text raises nothing");
    Read { status, model }
}

/// Reads a file.
pub fn read_file(path: &str) -> Read {
    RWHeaderSection::init();
    let model = StepDataStepModel::new();
    let status = step_file_read(path, None, &model, &HeaderSection::protocol().cast())
        .expect("reading a file raises nothing");
    Read { status, model }
}

impl Read {
    fn live(&self) -> &StepDataStepModel {
        self.model.get().expect("a model")
    }
    /// The fails of the global check.
    pub fn fails(&self) -> Vec<String> {
        let check = self.live().global_check(true);
        (1..=check.nb_fails())
            .map(|rank| {
                String::from_utf8_lossy(check.c_fail(rank, true).expect("a fail").as_bytes())
                    .into_owned()
            })
            .collect()
    }
    /// The warnings of the global check.
    pub fn warnings(&self) -> Vec<String> {
        let check = self.live().global_check(true);
        (1..=check.nb_warnings())
            .map(|rank| {
                String::from_utf8_lossy(check.c_warning(rank, true).expect("a warning").as_bytes())
                    .into_owned()
            })
            .collect()
    }
    /// The text the writer sends back.
    pub fn written(&self) -> Vec<u8> {
        let mut writer = StepDataStepWriter::new(&self.model);
        writer
            .send_model(&HeaderSection::protocol().cast(), false)
            .expect("the model is sent");
        let mut text = Vec::new();
        writer.print(&mut text).expect("the text is written");
        text
    }
    /// The stream compared with the reference.
    pub fn stream(&self) -> Vec<u8> {
        let fails = self.fails();
        let warnings = self.warnings();
        let mut text = format!(
            "status {}\nentities {}\nfails {}\n",
            self.status,
            self.live().nb_entities(),
            fails.len()
        );
        for fail in &fails {
            text.push_str(fail);
            text.push('\n');
        }
        text.push_str(&format!("warnings {}\n", warnings.len()));
        for warning in &warnings {
            text.push_str(warning);
            text.push('\n');
        }
        let mut bytes = text.into_bytes();
        if self.status == 0 {
            bytes.extend_from_slice(&self.written());
        }
        bytes
    }
}
