//! `StepDataFactors`: the unit factors of a transfer.

/// ```
/// use martensite_tkdestep::step_data::StepDataFactors;
/// let mut factors = StepDataFactors::new();
/// factors.initialize_factors(2.0, 4.0, 8.0);
/// assert_eq!(factors.factor_radian_degree(), 0.25);
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StepDataFactors {
    my_length_factor: f64,
    my_plane_angle_factor: f64,
    my_solid_angle_factor: f64,
    my_fact_rd: f64,
    my_fact_dr: f64,
    my_cascade_unit: f64,
}

impl Default for StepDataFactors {
    fn default() -> Self {
        Self {
            my_length_factor: 1.0,
            my_plane_angle_factor: 1.0,
            my_solid_angle_factor: 1.0,
            my_fact_rd: 1.0,
            my_fact_dr: 1.0,
            my_cascade_unit: 1.0,
        }
    }
}

impl StepDataFactors {
    /// `StepDataFactors()`: every factor 1.
    pub fn new() -> Self {
        Self::default()
    }
    /// `initialize_factors(length, plane angle, solid angle)`.
    pub fn initialize_factors(&mut self, length: f64, plane_angle: f64, solid_angle: f64) {
        self.my_length_factor = length;
        self.my_plane_angle_factor = plane_angle;
        self.my_solid_angle_factor = solid_angle;
        self.my_fact_rd = 1.0 / plane_angle;
        self.my_fact_dr = plane_angle;
    }
    /// `set_cascade_unit(unit)`.
    pub fn set_cascade_unit(&mut self, unit: f64) {
        self.my_cascade_unit = unit;
    }
    /// `cascade_unit`.
    pub fn cascade_unit(&self) -> f64 {
        self.my_cascade_unit
    }
    /// `length_factor`.
    pub fn length_factor(&self) -> f64 {
        self.my_length_factor
    }
    /// `plane_angle_factor`.
    pub fn plane_angle_factor(&self) -> f64 {
        self.my_plane_angle_factor
    }
    /// `solid_angle_factor`.
    pub fn solid_angle_factor(&self) -> f64 {
        self.my_solid_angle_factor
    }
    /// `factor_radian_degree`.
    pub fn factor_radian_degree(&self) -> f64 {
        self.my_fact_rd
    }
    /// `factor_degree_radian`.
    pub fn factor_degree_radian(&self) -> f64 {
        self.my_fact_dr
    }
}
