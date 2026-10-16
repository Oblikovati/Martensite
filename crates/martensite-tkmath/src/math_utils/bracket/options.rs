/// Optional finite bounds and cached endpoint values; `MinBracketOptions::default()`.
#[derive(Debug, Clone)]
pub struct MinBracketOptions {
    /// `max_iterations` field; access with `result.max_iterations`.
    pub max_iterations: u32,
    /// `use_limits` field; access with `result.use_limits`.
    pub use_limits: bool,
    /// `left_limit` field; access with `result.left_limit`.
    pub left_limit: f64,
    /// `right_limit` field; access with `result.right_limit`.
    pub right_limit: f64,
    /// `has_fa` field; access with `result.has_fa`.
    pub has_fa: bool,
    /// `has_fb` field; access with `result.has_fb`.
    pub has_fb: bool,
    /// `fa` field; access with `result.fa`.
    pub fa: f64,
    /// `fb` field; access with `result.fb`.
    pub fb: f64,
}
impl Default for MinBracketOptions {
    fn default() -> Self {
        Self {
            max_iterations: 50,
            use_limits: false,
            left_limit: 0.0,
            right_limit: 0.0,
            has_fa: false,
            has_fb: false,
            fa: 0.0,
            fb: 0.0,
        }
    }
}
