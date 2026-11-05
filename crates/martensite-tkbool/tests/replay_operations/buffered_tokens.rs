//! Operation-local token sink shared by ordinary tests and the stdout adapter.
use martensite_tkernel::standard::Failure;
use martensite_tkmath::gp::{Direction3, Point2, Point3, Xy, Xyz};
use std::{
    fmt::{Arguments, Write},
    sync::Mutex,
};

pub struct BufferedTokens {
    captured: Option<Mutex<String>>,
}
impl BufferedTokens {
    /// Development protocol output; `BufferedTokens::stdout().int(1)`.
    pub fn stdout() -> Self {
        Self { captured: None }
    }
    /// Independent request buffer; `BufferedTokens::captured().finish()`.
    #[cfg_attr(not(test), allow(dead_code))] // Used by in-process replay, not the stdout adapter.
    pub fn captured() -> Self {
        Self {
            captured: Some(Mutex::new(String::new())),
        }
    }
    pub(super) fn emit(&self, fields: Arguments<'_>) {
        let Some(captured) = &self.captured else {
            print!("{fields}");
            return;
        };
        captured
            .lock()
            .expect("one request owns the token buffer")
            .write_fmt(fields)
            .unwrap();
    }
    /// Take this request's complete output; `tokens.finish()`.
    #[cfg_attr(not(test), allow(dead_code))] // Used by in-process replay, not the stdout adapter.
    pub fn finish(&self) -> String {
        let captured = self
            .captured
            .as_ref()
            .expect("finish requires a captured request buffer");
        std::mem::take(&mut *captured.lock().expect("one request owns the token buffer"))
    }
    pub fn int(&self, value: i32) {
        self.emit(format_args!("{value} "));
    }
    pub fn real(&self, value: f64) {
        self.emit(format_args!("{} ", value.to_bits()));
    }
    pub fn flag(&self, value: bool) {
        self.int(i32::from(value));
    }
    pub fn end(&self) {
        self.emit(format_args!("\n"));
    }
    pub fn text(&self, bytes: &[u8]) {
        let hex: String = bytes.iter().map(|byte| format!("{byte:02x}")).collect();
        self.emit(format_args!("t{hex} "));
    }
    pub fn pnt(&self, point: Point3) {
        self.real(point.x());
        self.real(point.y());
        self.real(point.z());
    }
    pub fn pnt2d(&self, point: Point2) {
        self.real(point.x());
        self.real(point.y());
    }
    pub fn attempt(&self, action: impl FnOnce() -> Result<(), Failure>) {
        if let Err(error) = action() {
            self.emit(format_args!("{} ", error.exception_type()));
        }
    }
    /// Encode three spatial coordinates; `tokens.xyz(Xyz::new(1., 2., 3.))`.
    pub fn xyz(&self, value: Xyz) {
        self.real(value.x());
        self.real(value.y());
        self.real(value.z());
    }
    /// Encode a spatial unit direction; `tokens.dir(Direction3::Z)`.
    pub fn dir(&self, value: Direction3) {
        self.xyz(value.xyz());
    }
    /// Encode planar coordinates; `tokens.xy(Xy::new(1., 2.))`.
    pub fn xy(&self, value: Xy) {
        self.real(value.x());
        self.real(value.y());
    }
    /// Encode a successful value or its exception type; `tokens.show(Ok(1), |v| tokens.int(v))`.
    pub fn show<T>(&self, value: Result<T, Failure>, print: impl FnOnce(T)) {
        match value {
            Ok(value) => print(value),
            Err(error) => self.emit(format_args!("{} ", error.exception_type())),
        }
    }
    /// Keep console text before any raised exception; `tokens.captured_console(|s| { s.push_str("ok"); Ok(()) })`.
    pub fn captured_console(&self, body: impl FnOnce(&mut String) -> Result<(), Failure>) {
        let mut console = String::new();
        let result = body(&mut console);
        self.text(console.as_bytes());
        if let Err(error) = result {
            self.emit(format_args!("{} ", error.exception_type()));
        }
    }
}
