//! The two shape-tree output cases use quoted atoms without embedded spaces.
use super::subshape_and_assembly::{
    assert_document_kept_in_session, single_box_compound, translated_unit_box,
};
use super::*;

// OCC23911 keeps the actual document alive while the returned text is checked.
struct OpenShapeDumpScene {
    context: XdeShapeRegressionContext,
    output: String,
}

fn shape_dump_scene(context: XdeShapeRegressionContext, compound: bool) -> OpenShapeDumpScene {
    let part = translated_unit_box();
    let shape = if compound {
        single_box_compound(&part)
    } else {
        part
    };
    context.add_shape(&shape, true);
    let mut output = Vec::new();
    context
        .shape_tool()
        .dump_shapes(&mut output, false)
        .unwrap();
    OpenShapeDumpScene {
        context,
        output: String::from_utf8(output).unwrap(),
    }
}

fn assert_nonempty_dump_list(output: &str) {
    // These fixed dumps use only bare atoms and whole quoted atoms. Prove that
    // grammar before counting, so this preserves the original list-length check.
    assert!(!output.contains(['{', '}', '\\']));
    let tokens: Vec<&str> = output.split_whitespace().collect();
    for token in &tokens {
        let quotes = token.bytes().filter(|byte| *byte == b'"').count();
        assert!(quotes == 0 || (quotes == 2 && token.starts_with('"') && token.ends_with('"')));
    }
    assert!(!tokens.is_empty());
}

const LOCATED_PART_DUMP_CASE: &str =
    "document_regressions::shape_dump::located_part_dump_contains_instance_and_solid_atoms";

#[test]
fn located_part_dump_contains_instance_and_solid_atoms() {
    case_process_case::run(LOCATED_PART_DUMP_CASE, 1000, || {
        let OpenShapeDumpScene {
            context: _context,
            output,
        } = shape_dump_scene(XdeShapeRegressionContext::new(), false);
        assert_nonempty_dump_list(&output);
        assert!(output.contains("INSTANCE"));
        assert!(output.contains("SOLID"));
    });
}

const COMPOUND_SHAPE_DUMP_CASE: &str =
    "document_regressions::shape_dump::compound_dump_contains_assembly_and_solid_atoms";

#[test]
fn compound_dump_contains_assembly_and_solid_atoms() {
    case_process_case::run(COMPOUND_SHAPE_DUMP_CASE, 1000, || {
        let OpenShapeDumpScene {
            context: _context,
            output,
        } = shape_dump_scene(XdeShapeRegressionContext::new(), true);
        assert_nonempty_dump_list(&output);
        assert!(output.contains("ASSEMBLY"));
        assert!(output.contains("SOLID"));
    });
}

#[test]
fn dump_predicates_keep_document_registered_without_explicit_close() {
    use martensite_tklcaf::tdoc_std::TDocStdStandardApplication;
    for compound in [false, true] {
        let application = TDocStdStandardApplication::new();
        let context = XdeShapeRegressionContext::with_application(&application);
        let document = context.document.clone();
        let scene = shape_dump_scene(context, compound);
        assert_nonempty_dump_list(&scene.output);
        assert_document_kept_in_session(&application, &scene.context.document);
        drop(scene);
        assert_document_kept_in_session(&application, &document);
    }
}
