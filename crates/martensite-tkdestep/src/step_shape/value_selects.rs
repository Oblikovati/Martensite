//! The two value classes of `StepShape` that record which member they hold in a plain number, as
//!. They are not select types: the reader fills them through their setters.
#[allow(
    unused_imports,
    reason = "the classes of the package reference each other"
)]
use super::*;
use crate::step_entity::*;

/// `StepShapeBooleanOperand`: a solid model, a half space, a CSG primitive or a boolean result.
/// `set_type_of_content` is the only member that records the kind.
#[derive(Clone, Default)]
pub struct StepShapeBooleanOperand {
    the_solid_model: Handle<StepShapeSolidModel>,
    the_half_space_solid: Handle<StepShapeHalfSpaceSolid>,
    the_csg_primitive: StepShapeCsgPrimitive,
    the_boolean_result: Handle<StepShapeBooleanResult>,
    the_type_of_content: i32,
}

impl StepShapeBooleanOperand {
    /// `StepShapeBooleanOperand()`.
    pub fn new() -> Self {
        Self::default()
    }
    /// `set_type_of_content`.
    pub fn set_type_of_content(&mut self, a_type: i32) {
        self.the_type_of_content = a_type;
    }
    /// `type_of_content`.
    pub fn type_of_content(&self) -> i32 {
        self.the_type_of_content
    }
    /// `solid_model`.
    pub fn solid_model(&self) -> Handle<StepShapeSolidModel> {
        self.the_solid_model.clone()
    }
    /// `set_solid_model`.
    pub fn set_solid_model(&mut self, a_solid_model: &Handle<StepShapeSolidModel>) {
        self.the_solid_model = a_solid_model.clone();
    }
    /// `half_space_solid`.
    pub fn half_space_solid(&self) -> Handle<StepShapeHalfSpaceSolid> {
        self.the_half_space_solid.clone()
    }
    /// `set_half_space_solid`.
    pub fn set_half_space_solid(&mut self, a_half_space_solid: &Handle<StepShapeHalfSpaceSolid>) {
        self.the_half_space_solid = a_half_space_solid.clone();
    }
    /// `csg_primitive`.
    pub fn csg_primitive(&self) -> StepShapeCsgPrimitive {
        self.the_csg_primitive.clone()
    }
    /// `set_csg_primitive`.
    pub fn set_csg_primitive(&mut self, a_csg_primitive: &StepShapeCsgPrimitive) {
        self.the_csg_primitive = a_csg_primitive.clone();
    }
    /// `boolean_result`.
    pub fn boolean_result(&self) -> Handle<StepShapeBooleanResult> {
        self.the_boolean_result.clone()
    }
    /// `set_boolean_result`.
    pub fn set_boolean_result(&mut self, a_boolean_result: &Handle<StepShapeBooleanResult>) {
        self.the_boolean_result = a_boolean_result.clone();
    }
}

/// `StepShapeCsgSelect`: a boolean result (content 1) or a CSG primitive (content 2).
#[derive(Clone, Default)]
pub struct StepShapeCsgSelect {
    the_boolean_result: Handle<StepShapeBooleanResult>,
    the_csg_primitive: StepShapeCsgPrimitive,
    the_type_of_content: i32,
}

impl StepShapeCsgSelect {
    /// `StepShapeCsgSelect()`.
    pub fn new() -> Self {
        Self::default()
    }
    /// `set_type_of_content`.
    pub fn set_type_of_content(&mut self, a_type: i32) {
        self.the_type_of_content = a_type;
    }
    /// `type_of_content`.
    pub fn type_of_content(&self) -> i32 {
        self.the_type_of_content
    }
    /// `boolean_result`.
    pub fn boolean_result(&self) -> Handle<StepShapeBooleanResult> {
        self.the_boolean_result.clone()
    }
    /// `set_boolean_result`: also records the content as 1.
    pub fn set_boolean_result(&mut self, a_boolean_result: &Handle<StepShapeBooleanResult>) {
        self.the_boolean_result = a_boolean_result.clone();
        self.the_type_of_content = 1;
    }
    /// `csg_primitive`.
    pub fn csg_primitive(&self) -> StepShapeCsgPrimitive {
        self.the_csg_primitive.clone()
    }
    /// `set_csg_primitive`: also records the content as 2.
    pub fn set_csg_primitive(&mut self, a_csg_primitive: &StepShapeCsgPrimitive) {
        self.the_csg_primitive = a_csg_primitive.clone();
        self.the_type_of_content = 2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_csg_select_records_its_content_and_an_operand_does_not() {
        let mut select = StepShapeCsgSelect::new();
        assert_eq!(select.type_of_content(), 0);
        select.set_csg_primitive(&StepShapeCsgPrimitive::default());
        assert_eq!(select.type_of_content(), 2);
        select.set_boolean_result(&Handle::new(StepShapeBooleanResult::new()));
        assert_eq!(select.type_of_content(), 1);
        let mut operand = StepShapeBooleanOperand::new();
        operand.set_solid_model(&Handle::new(StepShapeSolidModel::new()));
        assert_eq!(operand.type_of_content(), 0);
        operand.set_type_of_content(1);
        assert!(operand.solid_model().is_some());
    }
}
