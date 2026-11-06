//! `read_file`, `write_file`, `readfile`, `writefile`: the commands that go through the process-wide wrapper
//! and the STEP provider the toolkit registers in it.
use crate::documents::CommandFailed;
use crate::support::{rules::EXCEPTION_PREFIX, session::Session};
use crate::xs_session::XsSession;
use martensite_tkbrep::topods::Shape;
use martensite_tkde::de::{DePluginHolder, DeWorkSession, DeWrapper};
use martensite_tkdestep::de_step::DeStepConfigurationNode;
use martensite_tkernel::{
    message::ProgressRange,
    standard::{Failure, Handle},
    tcollection::AsciiString,
};
use std::sync::Once;

/// `pload XDE`: the plugin holder of the STEP node, for the life of the process, as the static one.
fn register_step_plugin() {
    static REGISTERED: Once = Once::new();
    REGISTERED.call_once(|| {
        let holder = DePluginHolder::<DeStepConfigurationNode>::new()
            .unwrap_or_else(|failure| panic!("the STEP plugin: {failure:?}"));
        std::mem::forget(holder);
    });
}

/// `DeWrapper::global_wrapper()->Copy()`, with the `-conf` text loaded when there is one.
fn configured_wrapper(conf: Option<&str>) -> Result<Option<Handle<DeWrapper>>, Failure> {
    register_step_plugin();
    let wrapper = DeWrapper::global_wrapper()
        .get()
        .expect("a global wrapper")
        .copy();
    if let Some(text) = conf
        && !wrapper
            .get()
            .expect("a copied wrapper")
            .load(&AsciiString::from(text), true)?
    {
        return Ok(None);
    }
    Ok(Some(wrapper))
}

/// The line the shell echoes for a command, with its `-conf` word.
fn echo(command: &str, name: &str, file: &str, conf: Option<&str>) {
    match conf {
        Some(text) => println!("{command} {name} {file} -conf {text} "),
        None => println!("{command} {name} {file} "),
    }
}

/// A command's outcome as the shell sees it: an exception is written, `false` returns 1.
fn outcome(transferred: Result<bool, Failure>) -> Result<(), CommandFailed> {
    match transferred {
        Ok(true) => Ok(()),
        Ok(false) => Err(CommandFailed),
        Err(failure) => {
            println!("{EXCEPTION_PREFIX}{failure:?}");
            Err(CommandFailed)
        }
    }
}

impl XsSession {
    fn de_work_session(&self) -> DeWorkSession {
        self.work_session.clone().cast()
    }

    /// `ReadFile doc file [-conf text]`: into the named document, new if there is none.
    /// Example: `session.read_file("D", "a.stp", None)`.
    pub fn read_file(
        &mut self,
        name: &str,
        file: &str,
        conf: Option<&str>,
    ) -> Result<(), CommandFailed> {
        echo("ReadFile", name, file, conf);
        if !self.documents.contains_key(name) {
            self.new_named_document(name);
        }
        let document = self.documents[name].clone();
        let mut session = self.de_work_session();
        outcome(configured_wrapper(conf).and_then(|wrapper| {
            let Some(wrapper) = wrapper else {
                return Ok(false);
            };
            wrapper
                .get()
                .expect("a wrapper")
                .read_document_with_session(
                    &AsciiString::from(file),
                    &document,
                    &mut session,
                    &ProgressRange::default(),
                )
        }))
    }

    /// `WriteFile doc file [-conf text]`. Example: `session.write_file("D", "a.stp", None)`.
    pub fn write_file(
        &mut self,
        name: &str,
        file: &str,
        conf: Option<&str>,
    ) -> Result<(), CommandFailed> {
        echo("WriteFile", name, file, conf);
        let Some(document) = self.documents.get(name).cloned() else {
            println!("Error: incorrect document");
            return Err(CommandFailed);
        };
        let mut session = self.de_work_session();
        outcome(configured_wrapper(conf).and_then(|wrapper| {
            let Some(wrapper) = wrapper else {
                return Ok(false);
            };
            wrapper
                .get()
                .expect("a wrapper")
                .write_document_with_session(
                    &AsciiString::from(file),
                    &document,
                    &mut session,
                    &ProgressRange::default(),
                )
        }))
    }

    /// `readfile shape file [-conf text]`. Example: `session.read_shape_file("S", "a.stp", None)`.
    pub fn read_shape_file(
        &mut self,
        name: &str,
        file: &str,
        conf: Option<&str>,
    ) -> Result<(), CommandFailed> {
        echo("readfile", name, file, conf);
        let mut shape = Shape::default();
        let mut session = self.de_work_session();
        outcome(configured_wrapper(conf).and_then(|wrapper| {
            let Some(wrapper) = wrapper else {
                return Ok(false);
            };
            wrapper.get().expect("a wrapper").read_shape_with_session(
                &AsciiString::from(file),
                &mut shape,
                &mut session,
                &ProgressRange::default(),
            )
        }))?;
        self.raster.bind(name, &shape);
        Ok(())
    }

    /// `writefile shape file [-conf text]`. Example: `session.write_shape_file("S", "a.stp", None)`.
    pub fn write_shape_file(
        &mut self,
        name: &str,
        file: &str,
        conf: Option<&str>,
    ) -> Result<(), CommandFailed> {
        echo("writefile", name, file, conf);
        let shape = self.raster.shape(name).cloned().unwrap_or_default();
        if shape.is_null() {
            println!("Error: incorrect shape {name}");
            return Err(CommandFailed);
        }
        let mut session = self.de_work_session();
        outcome(configured_wrapper(conf).and_then(|wrapper| {
            let Some(wrapper) = wrapper else {
                return Ok(false);
            };
            wrapper.get().expect("a wrapper").write_shape_with_session(
                &AsciiString::from(file),
                &shape,
                &mut session,
                &ProgressRange::default(),
            )
        }))
    }
}
