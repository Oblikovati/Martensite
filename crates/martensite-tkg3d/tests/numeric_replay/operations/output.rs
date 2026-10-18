//! One operation owns its token buffer; there is no process or thread-local capture.
use std::{
    fmt::{Arguments, Write},
    sync::Mutex,
};

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
            .expect("poisoned token buffer; expected one request owner")
            .write_fmt(arguments)
            .expect("formatting operation tokens");
    }
    pub fn finish(&self) -> String {
        std::mem::take(
            &mut *self
                .text
                .lock()
                .expect("poisoned token buffer; expected one request owner"),
        )
    }
    pub fn set_parameters(&self, parameters: Vec<f64>) {
        *self
            .parameters
            .lock()
            .expect("poisoned parameters; expected one request owner") = parameters;
    }
    pub fn parameters(&self) -> Vec<f64> {
        self.parameters
            .lock()
            .expect("poisoned parameters; expected one request owner")
            .clone()
    }
}
