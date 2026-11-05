//! Live control state for external builder composition.
use super::{BuilderExtensionState, BuilderExtensionStateMut};
use crate::top_ope_brep_tool::TopOpeBRepToolShapeClassifier;

impl BuilderExtensionState<'_> {
    /// Borrows the retained classification defined; `let slot = state.classification_defined();`.
    pub fn classification_defined(&self) -> &bool {
        &self.builder.my_classify_def
    }

    /// Borrows the retained classification value; `let slot = state.classification_value();`.
    pub fn classification_value(&self) -> &bool {
        &self.builder.my_classify_val
    }

    /// Borrows the retained diagnostic name; `let slot = state.diagnostic_name();`.
    pub fn diagnostic_name(&self) -> &String {
        &self.builder.my_empty_as
    }

    /// Borrows the retained special case; `let slot = state.special_case();`.
    pub fn special_case(&self) -> &i32 {
        &self.builder.my_is_k_part
    }

    /// Borrows the retained process on; `let slot = state.process_on();`.
    pub fn process_on(&self) -> &bool {
        &self.builder.my_process_on
    }

    /// Borrows the retained shape set index; `let slot = state.shape_set_index();`.
    pub fn shape_set_index(&self) -> &i32 {
        &self.builder.my_shaset_index
    }

    /// Borrows the retained section complete; `let slot = state.section_complete();`.
    pub fn section_complete(&self) -> &bool {
        &self.builder.my_section_done
    }

    /// Borrows the retained classifier; `let slot = state.classifier();`.
    pub fn classifier(&self) -> &TopOpeBRepToolShapeClassifier {
        &self.builder.my_shape_classifier
    }

    /// Borrows the retained section edges complete; `let slot = state.section_edges_complete();`.
    pub fn section_edges_complete(&self) -> &bool {
        &self.builder.my_split_section_edges_done
    }
}
impl BuilderExtensionStateMut<'_> {
    /// Edits the retained classification defined; `let slot = state.classification_defined_mut();`.
    pub fn classification_defined_mut(&mut self) -> &mut bool {
        &mut self.builder.my_classify_def
    }

    /// Edits the retained classification value; `let slot = state.classification_value_mut();`.
    pub fn classification_value_mut(&mut self) -> &mut bool {
        &mut self.builder.my_classify_val
    }

    /// Edits the retained diagnostic name; `let slot = state.diagnostic_name_mut();`.
    pub fn diagnostic_name_mut(&mut self) -> &mut String {
        &mut self.builder.my_empty_as
    }

    /// Edits the retained special case; `let slot = state.special_case_mut();`.
    pub fn special_case_mut(&mut self) -> &mut i32 {
        &mut self.builder.my_is_k_part
    }

    /// Edits the retained process on; `let slot = state.process_on_mut();`.
    pub fn process_on_mut(&mut self) -> &mut bool {
        &mut self.builder.my_process_on
    }

    /// Edits the retained shape set index; `let slot = state.shape_set_index_mut();`.
    pub fn shape_set_index_mut(&mut self) -> &mut i32 {
        &mut self.builder.my_shaset_index
    }

    /// Edits the retained section complete; `let slot = state.section_complete_mut();`.
    pub fn section_complete_mut(&mut self) -> &mut bool {
        &mut self.builder.my_section_done
    }

    /// Edits the retained classifier; `let slot = state.classifier_mut();`.
    pub fn classifier_mut(&mut self) -> &mut TopOpeBRepToolShapeClassifier {
        &mut self.builder.my_shape_classifier
    }

    /// Edits the retained section edges complete; `let slot = state.section_edges_complete_mut();`.
    pub fn section_edges_complete_mut(&mut self) -> &mut bool {
        &mut self.builder.my_split_section_edges_done
    }
}
