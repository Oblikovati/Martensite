//! Native GetShape/GetReference/whatis observations without a script or presentation runtime.
use super::{
    naming_box_driver::function_result, naming_draw_document::NamingDocumentCase,
    naming_draw_presentation::NamingPresentation,
};
#[path = "boolean_typed_explode.rs"]
mod case_explode;
use martensite_tkbrep::topods::Shape;
use martensite_tkcaf::tnaming::TNamingTool;
use martensite_tkernel::{
    standard::{Failure, Handle},
    tcollection::AsciiString,
};
use martensite_tkg3d::top_abs::{shape_orientation_to_string, shape_type_to_string};
use martensite_tklcaf::{
    tdata_std::TDataStdUAttribute,
    tdf::{TdfAttributeState, TdfReference, TdfTool},
    tdoc_std::TDocStdDocument,
    tfunction::TFunctionFunction,
};
use std::collections::BTreeMap;
/// Named drawable state is deliberately thread-confined through the injected Rc presentation.
#[derive(Default)]
pub(super) struct NamingShapeBindings {
    shapes: BTreeMap<String, Shape>,
    presentation: NamingPresentation,
}
impl NamingShapeBindings {
    /// Share the caller-owned presentation, never a snapshot; `NamingShapeBindings::for_case(&case)`.
    pub(super) fn for_case(case: &NamingDocumentCase) -> Self {
        let presentation = case.presentation.clone();
        Self {
            shapes: BTreeMap::new(),
            presentation,
        }
    }
    /// Expose the same live scene for literal drawing commands; `bindings.presentation().only("S")`.
    pub(super) fn presentation(&self) -> NamingPresentation {
        // Sharing the same scene preserves all previously published drawable identities.
        self.presentation.clone()
    }
    /// Native GetShape requires a present, nonnull named result; `bindings.bind(&function,"S")?`.
    pub fn bind(
        &mut self,
        function: &Handle<TFunctionFunction>,
        name: &'static str,
    ) -> Result<(), Failure> {
        let shape = required_result(function)?;
        self.publish_shape(name, shape);
        Ok(())
    }
    /// Preserve literal erase before replacing the named result; `bindings.bind_after_erase(&function, "S")?`.
    pub(super) fn bind_after_erase(
        &mut self,
        function: &Handle<TFunctionFunction>,
        name: &'static str,
    ) -> Result<(), Failure> {
        self.presentation.erase_all();
        self.bind(function, name)
    }
    fn publish_shape(&mut self, name: &str, shape: Shape) {
        self.presentation.publish(name, &shape);
        self.shapes.insert(name.into(), shape);
    }
    /// Preserve actual source typed-explode aliases and drawable publications; `bindings.explode("S", ShapeEnum::Face)`.
    pub(super) fn explode(&mut self, name: &str, kind: martensite_tkg3d::top_abs::ShapeEnum) {
        let source = self
            .shapes
            .get(name)
            .unwrap_or_else(|| panic!("explode {name}: expected bound shape"));
        let entities = case_explode::all_distinct(source, kind);
        for (index, shape) in entities.into_iter().enumerate() {
            self.publish_shape(&format!("{name}_{}", index + 1), shape);
        }
    }
    /// Preserve the original face then face-edge explode order; `bindings.explode_face_edge("Box2", 1)`.
    pub(super) fn explode_face_edge(&mut self, name: &str, face: usize) {
        self.explode(name, martensite_tkg3d::top_abs::ShapeEnum::Face);
        self.explode(
            &format!("{name}_{face}"),
            martensite_tkg3d::top_abs::ShapeEnum::Edge,
        );
    }
    /// Native whatis silently ignores missing names; `bindings.whatis("Box2")?`.
    pub fn whatis(&self, name: &str) -> Result<String, Failure> {
        let Some(shape) = self.shapes.get(name) else {
            return Ok(String::new());
        };
        let mut text = format!(
            "shape {} {}",
            shape_type_to_string(shape.shape_type()?),
            shape_orientation_to_string(shape.orientation())
        );
        for (set, word) in shape_flags(shape)? {
            if set {
                text.push_str(word);
            }
        }
        Ok(text)
    }
}
fn required_result(function: &Handle<TFunctionFunction>) -> Result<Shape, Failure> {
    let named = function_result(function)?;
    if named.is_null() {
        return Err(Failure::new(
            "GetShape named result is null; expected named shape attribute",
        ));
    }
    let shape = TNamingTool::get_shape(&named)?;
    if shape.is_null() {
        return Err(Failure::new(
            "GetShape result is null; expected nonnull shape",
        ));
    }
    Ok(shape)
}
fn shape_flags(shape: &Shape) -> Result<[(bool, &'static str); 6], Failure> {
    Ok([
        (shape.free()?, " Free"),
        (shape.modified()?, " Modified"),
        (shape.orientable()?, " Orientable"),
        (shape.closed()?, " Closed"),
        (shape.infinite()?, " Infinite"),
        (shape.convex()?, " Convex"),
    ])
}
/// Publish actual whatis text before preserving its assertion; `observe_shape(&bindings, "P1", "shape VERTEX")`.
pub(super) fn observe_shape(bindings: &NamingShapeBindings, name: &str, prefix: &str) {
    let description = bindings.whatis(name).unwrap();
    println!("{description}");
    assert!(description.starts_with(prefix));
}
/// Publish actual GetReference text and preserve its expected entry; `observe_reference(&object, "0:1:1:1:2")`.
pub(super) fn observe_reference(object: &Handle<TDataStdUAttribute>, expected: &str) {
    let actual = reference_entry(object).unwrap();
    println!("{actual}");
    assert_eq!(actual, expected);
}
/// Original GetReference reads and returns the actual target-label entry; `reference_entry(&object)?`.
pub(super) fn reference_entry(object: &Handle<TDataStdUAttribute>) -> Result<String, Failure> {
    let mut reference: Handle<TdfReference> = Handle::default();
    if !object.find_attribute_as(&TdfReference::get_id(), &mut reference)? {
        return Err(Failure::new(
            "GetReference attribute is absent; expected TDF_Reference",
        ));
    }
    let mut entry = AsciiString::default();
    TdfTool::entry(&reference.get().unwrap().get(), &mut entry);
    Ok(String::from_utf8_lossy(entry.as_bytes()).into_owned())
}
/// E3 recreates the begin document, retaining default UndoLimit0; `new_command(&document)`.
pub(super) fn new_command(document: &Handle<TDocStdDocument>) {
    let limit = document.get_undo_limit();
    assert!(
        matches!(limit, 0 | 100),
        "expected named/driver undo policy, got {limit}"
    );
    if limit == 0 {
        assert!(!document.has_open_command());
    }
    document.new_command().unwrap();
    assert_eq!(document.has_open_command(), limit == 100);
}

#[test]
fn observations_preserve_reference_entry_and_missing_attribute_diagnostic() {
    let (document, object, function) = super::modeled_box();
    let mut bindings = NamingShapeBindings::default();
    assert_eq!(reference_entry(&object).unwrap(), "0:1:1:1:2");
    bindings.bind(&function, "Box").unwrap();
    assert!(
        bindings
            .whatis("Box")
            .unwrap()
            .starts_with("shape SOLID FORWARD")
    );
    assert_eq!(bindings.whatis("absent").unwrap(), "");
    let empty = super::naming_function_graph::add_object(&document).unwrap();
    assert!(reference_entry(&empty).is_err());
    let uncomputed = super::naming_function_graph::add_function(
        &empty,
        &martensite_tklcaf::guid_from_text("12e94543-6dbc-11d4-b9c8-0060b0ee281b"),
    )
    .unwrap();
    assert!(bindings.bind(&uncomputed, "Missing").is_err());
}
#[test]
fn whatis_reports_all_six_flags_in_native_order() {
    let (_document, _object, function) = super::modeled_box();
    let mut bindings = NamingShapeBindings::default();
    bindings.bind(&function, "Box").unwrap();
    let shape = bindings.shapes.get("Box").unwrap();
    shape.set_free(true).unwrap();
    shape.set_modified(true).unwrap();
    shape.set_orientable(true).unwrap();
    shape.set_closed(true).unwrap();
    shape.set_infinite(true).unwrap();
    shape.set_convex(true).unwrap();
    assert_eq!(
        bindings.whatis("Box").unwrap(),
        "shape SOLID FORWARD Free Modified Orientable Closed Infinite Convex"
    );
    shape.set_free(false).unwrap();
    assert!(!bindings.whatis("Box").unwrap().contains(" Free"));
}
