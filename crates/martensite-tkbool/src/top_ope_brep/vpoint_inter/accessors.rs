use super::{TopOpeBRepVPointInter, source_domain_error};
use martensite_tkernel::standard::Failure;
use martensite_tkgeomalgo::int_surf::IntSurfTransition;
use martensite_tkmath::gp::Point3;

impl TopOpeBRepVPointInter<'_> {
    /// Reads the borrowed point's transition on s1; `view.transition_on_s1()?`.
    pub fn transition_on_s1(&self) -> Result<IntSurfTransition, Failure> {
        self.source()?
            .transition_on_s1()
            .map_err(source_domain_error)
    }
    /// Reads the borrowed point's transition on s2; `view.transition_on_s2()?`.
    pub fn transition_on_s2(&self) -> Result<IntSurfTransition, Failure> {
        self.source()?
            .transition_on_s2()
            .map_err(source_domain_error)
    }
    /// Reads the borrowed point's transition line arc1; `view.transition_line_arc1()?`.
    pub fn transition_line_arc1(&self) -> Result<IntSurfTransition, Failure> {
        self.source()?
            .transition_line_arc1()
            .map_err(source_domain_error)
    }
    /// Reads the borrowed point's transition line arc2; `view.transition_line_arc2()?`.
    pub fn transition_line_arc2(&self) -> Result<IntSurfTransition, Failure> {
        self.source()?
            .transition_line_arc2()
            .map_err(source_domain_error)
    }
    /// Reads the borrowed point's is on dom s1; `view.is_on_dom_s1()?`.
    pub fn is_on_dom_s1(&self) -> Result<bool, Failure> {
        Ok(self.source()?.is_on_dom_s1())
    }
    /// Reads the borrowed point's is on dom s2; `view.is_on_dom_s2()?`.
    pub fn is_on_dom_s2(&self) -> Result<bool, Failure> {
        Ok(self.source()?.is_on_dom_s2())
    }
    /// Reads the borrowed point's parameters on s1; `view.parameters_on_s1()?`.
    pub fn parameters_on_s1(&self) -> Result<(f64, f64), Failure> {
        Ok(self.source()?.parameters_on_s1())
    }
    /// Reads the borrowed point's parameters on s2; `view.parameters_on_s2()?`.
    pub fn parameters_on_s2(&self) -> Result<(f64, f64), Failure> {
        Ok(self.source()?.parameters_on_s2())
    }
    /// Reads the borrowed point's value; `view.value()?`.
    pub fn value(&self) -> Result<Point3, Failure> {
        Ok(self.source()?.value())
    }
    /// Reads the borrowed point's tolerance; `view.tolerance()?`.
    pub fn tolerance(&self) -> Result<f64, Failure> {
        Ok(self.source()?.tolerance())
    }
    /// Reads the borrowed point's parameter on line; `view.parameter_on_line()?`.
    pub fn parameter_on_line(&self) -> Result<f64, Failure> {
        Ok(self.source()?.parameter_on_line())
    }
    /// Reads the borrowed point's parameter on arc1; `view.parameter_on_arc1()?`.
    pub fn parameter_on_arc1(&self) -> Result<f64, Failure> {
        self.source()?
            .parameter_on_arc1()
            .map_err(source_domain_error)
    }
    /// Reads the borrowed point's parameter on arc2; `view.parameter_on_arc2()?`.
    pub fn parameter_on_arc2(&self) -> Result<f64, Failure> {
        self.source()?
            .parameter_on_arc2()
            .map_err(source_domain_error)
    }
    /// Reads the borrowed point's is vertex on s1; `view.is_vertex_on_s1()?`.
    pub fn is_vertex_on_s1(&self) -> Result<bool, Failure> {
        Ok(self.source()?.is_vertex_on_s1())
    }
    /// Reads the borrowed point's is vertex on s2; `view.is_vertex_on_s2()?`.
    pub fn is_vertex_on_s2(&self) -> Result<bool, Failure> {
        Ok(self.source()?.is_vertex_on_s2())
    }
    /// Reads the borrowed point's is multiple; `view.is_multiple()?`.
    pub fn is_multiple(&self) -> Result<bool, Failure> {
        Ok(self.source()?.is_multiple())
    }
}
