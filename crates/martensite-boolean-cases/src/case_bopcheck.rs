//! The default full bopcheck response shared without imposing a caller acceptance policy.
use crate::common_operation::case_bop_alerts::gravity_alert_text;
use martensite_tkbo::{
    bop_algo::BOPAlgoCheckerSI,
    bop_ds::{DataStructure, tools},
};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::{message::Gravity, ncollection::List, precision};

/// Preserve the default command request; `configured_checker(&shape)`.
pub fn configured_checker(result: &Shape) -> BOPAlgoCheckerSI {
    let mut checker = BOPAlgoCheckerSI::new();
    checker.set_arguments(&[result.clone()].into_iter().collect::<List<_>>());
    checker.set_level_of_check(DataStructure::nb_interf_types() - 1);
    checker.options_mut().set_run_parallel(false);
    checker
        .options_mut()
        .set_fuzzy_value(precision::confusion());
    checker
}

fn case_pairs(structure: &DataStructure) -> Vec<(i32, i32, i32)> {
    let mut pairs = Vec::new();
    for pair in structure.interferences() {
        let (first, second) = pair.indices();
        if structure.is_new_shape(first) || structure.is_new_shape(second) {
            continue;
        }
        let kind = tools::type_pair_to_integer(
            structure.shape_info(first).unwrap().shape_type(),
            structure.shape_info(second).unwrap().shape_type(),
        );
        pairs.push((kind, first, second));
    }
    pairs.sort_unstable();
    pairs
}

fn pair_text(kind: i32, first: i32, second: i32) -> String {
    let labels = [
        "V/V", "V/E", "E/E", "V/F", "E/F", "F/F", "V/Z", "E/Z", "F/Z", "Z/Z",
    ];
    let label = labels
        .get(kind as usize)
        .unwrap_or_else(|| panic!("bopcheck interference kind {kind}: expected source type0..9"));
    format!("{label}: x{first} x{second} \n")
}

/// Keep warning, failure, pair and completion text intact; `complete_response(&checker)`.
pub fn complete_response(checker: &BOPAlgoCheckerSI) -> String {
    let mut response = String::new();
    for (gravity, prefix) in [(Gravity::Warning, "Warning: "), (Gravity::Fail, "Error: ")] {
        response.push_str(&gravity_alert_text(
            checker.options().report(),
            gravity,
            prefix,
        ));
    }
    let pairs = case_pairs(
        checker
            .data_structure()
            .expect("bopcheck: expected actual performed structure"),
    );
    for &(kind, first, second) in &pairs {
        response.push_str(&pair_text(kind, first, second));
    }
    append_source_completion(&mut response, checker.options().has_errors(), pairs.len());
    response
}

fn append_source_completion(response: &mut String, has_errors: bool, pair_count: usize) {
    if has_errors {
        response
            .push_str("There were errors during the operation, so the list may be incomplete.\n");
    }
    if pair_count == 0 {
        response.push_str(" This shape seems to be OK.\n");
    }
}
