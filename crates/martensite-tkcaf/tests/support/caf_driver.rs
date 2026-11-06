//! Fixed original driver identities share the canonical graph fixture without duplicating helper tests.
use crate::{case_driver_process_case, tnaming_selection_driver as selection_fixture};
use selection_fixture::naming_draw_document::NamingDocumentCase;

use selection_fixture::draw_cylinder_cut_checks::caf_driver_a4_recomputes_case_graph_with_retained_begin_document as a4;
use selection_fixture::draw_cylinder_cut_checks::caf_driver_a5_recomputes_case_graph_with_retained_begin_document as a5;
use selection_fixture::draw_e3_checks::caf_driver_a3_recomputes_case_graph_with_retained_begin_document as a3;
use selection_fixture::draw_fillet_cut_checks::caf_driver_a1_recomputes_case_graph_with_retained_begin_document as a1;
use selection_fixture::draw_fused_fillet_checks::caf_driver_a6_recomputes_case_graph_with_retained_begin_document as a6;
use selection_fixture::draw_polygon_cut_checks::caf_driver_b6_retains_case_offsets_and_caught_solve_outcome as b6;
use selection_fixture::draw_polygon_cut_checks::caf_driver_b7_recomputes_four_case_edge_selections_with_retained_begin_document as b7;
use selection_fixture::draw_polygon_fuse_checks::caf_driver_b5_recomputes_case_graph_with_retained_begin_document as b5;
use selection_fixture::draw_prism_fuse_checks::caf_driver_b4_recomputes_case_graph_with_retained_begin_document as b4;
use selection_fixture::draw_revolution_chain_checks::caf_driver_a7_recomputes_case_graph_with_retained_begin_document as a7;
use selection_fixture::draw_revolution_chain_checks::caf_driver_a8_recomputes_case_graph_with_retained_begin_document as a8;
use selection_fixture::draw_sphere_fuse_checks::caf_driver_b1_recomputes_case_graph_with_retained_begin_document as b1;

/// Immutable original selection is Send+Sync; `run("caf_driver_a1", CafDriverCase::A1)`.
#[derive(Clone, Copy)]
pub enum CafDriverCase {
    /// Recompute the attached cylinder cut and fillet graph.
    A1,
    /// Recompute the translated box cut and both selection modes.
    A3,
    /// Recompute the tall cylinder cut and both selection modes.
    A4,
    /// Recompute the short cylinder cut and single selections.
    A5,
    /// Recompute the fused-box cylinder cut and fillet.
    A6,
    /// Recompute the prism and full/sectioned revolution chain.
    A7,
    /// Recompute three sectioned revolutions and their fusions.
    A8,
    /// Recompute the cylinder cut and four attached sphere fusions.
    B1,
    /// Recompute dependent prisms and their fusion.
    B4,
    /// Recompute the point-profile prisms and sphere cut.
    B5,
    /// Retain original point offsets and caught solver outcome.
    B6,
    /// Recompute the four original oriented edge attachments.
    B7,
}

impl CafDriverCase {
    fn execute(self, lifecycle: NamingDocumentCase) {
        match self {
            Self::A1 => a1(lifecycle),
            Self::A3 => a3(lifecycle),
            Self::A4 => a4(lifecycle),
            Self::A5 => a5(lifecycle),
            Self::A6 => a6(lifecycle),
            Self::A7 => a7(lifecycle),
            Self::A8 => a8(lifecycle),
            Self::B1 => b1(lifecycle),
            Self::B4 => b4(lifecycle),
            Self::B5 => b5(lifecycle),
            Self::B6 => b6(lifecycle),
            Self::B7 => b7(lifecycle),
        }
    }
}

/// Run a complete source graph with real presentation/end and process policy; `run("caf_driver_a1", CafDriverCase::A1)`.
pub fn run(identity: &str, case: CafDriverCase) {
    let _: &(dyn Send + Sync) = &case;
    case_driver_process_case::run_presented_driver_case(identity, |lifecycle| {
        case.execute(lifecycle);
    });
}
