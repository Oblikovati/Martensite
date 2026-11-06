//! The shape variables of one running script, under the names the script gives them.
use crate::support::session::{Session, ViewStep};
use martensite_tkbrep::topods::Shape;
use std::collections::HashMap;

/// The variables; the view lives in [`crate::scene::ViewScene`], not here.
#[derive(Default)]
pub struct Variables {
    shapes: HashMap<String, Shape>,
}

impl Variables {
    /// The shape in a variable; `variables.get("result")`. An unbound name stops the case.
    pub fn get(&self, name: &str) -> Shape {
        self.shapes
            .get(name)
            .cloned()
            .unwrap_or_else(|| panic!("variable {name:?}; expected a bound shape"))
    }

    /// Fills a variable; `variables.set("result", shape)`.
    pub fn set(&mut self, name: &str, shape: Shape) {
        self.shapes.insert(name.to_owned(), shape);
    }
}

impl Session for Variables {
    fn bind(&mut self, name: &str, shape: &Shape) {
        self.shapes.insert(name.to_owned(), shape.clone());
    }

    fn shape(&self, name: &str) -> Option<&Shape> {
        self.shapes.get(name)
    }

    fn view(&mut self, step: ViewStep<'_>) {
        panic!("view step {step:?}; these cases draw through their scene");
    }
}
