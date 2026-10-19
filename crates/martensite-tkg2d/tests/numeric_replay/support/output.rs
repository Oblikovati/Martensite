//! One request owns its output and transform parameters; replay has no global capture state.
use std::fmt::{Arguments, Write as _};
use std::sync::Mutex;

pub struct Output {
    text: Mutex<String>,
    parameters: Mutex<Vec<f64>>,
}

impl Output {
    pub fn captured() -> Self {
        Self {
            text: Mutex::new(String::new()),
            parameters: Mutex::new(Vec::new()),
        }
    }
    pub fn emit(&self, arguments: Arguments<'_>) {
        self.text
            .lock()
            .expect("poisoned output; expected exclusive request buffer")
            .write_fmt(arguments)
            .expect("formatting request tokens");
    }
    pub fn finish(&self) -> String {
        std::mem::take(
            &mut *self
                .text
                .lock()
                .expect("poisoned output; expected exclusive request buffer"),
        )
    }
    pub fn set_parameters(&self, parameters: Vec<f64>) {
        *self
            .parameters
            .lock()
            .expect("poisoned parameters; expected request-local values") = parameters;
    }
    pub fn parameters(&self) -> Vec<f64> {
        self.parameters
            .lock()
            .expect("poisoned parameters; expected request-local values")
            .clone()
    }
}
