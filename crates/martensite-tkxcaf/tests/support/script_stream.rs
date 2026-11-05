//! The lines a document case reports, so a run can be compared line by line with a reference log.
//! Commands are echoed as written; every answer comes from the document or the view under test.
use std::path::Path;

/// One case's report; it is saved only when `MARTENSITE_DOCUMENT_ARTIFACT_DIR` asks for it.
#[derive(Default)]
pub struct ScriptStream {
    lines: Vec<String>,
}

impl ScriptStream {
    /// Add one reported line; `stream.line("0:1:1:1")`.
    pub fn line(&mut self, text: impl Into<String>) {
        self.lines.push(text.into());
    }

    /// Add lines at once; `stream.lines(["cpulimit 1000", "pload OCAF"])`.
    pub fn lines<const N: usize>(&mut self, texts: [&str; N]) {
        self.lines.extend(texts.map(String::from));
    }

    /// Save the report as `<case>.stream.txt`, if a run asked for it; `stream.save("bug28044")`.
    pub fn save(&self, case: &str) {
        let Some(directory) = std::env::var_os("MARTENSITE_DOCUMENT_ARTIFACT_DIR") else {
            return;
        };
        std::fs::create_dir_all(&directory).unwrap();
        let file = Path::new(&directory).join(format!("{case}.stream.txt"));
        std::fs::write(file, self.lines.join("\n") + "\n").unwrap();
    }
}
