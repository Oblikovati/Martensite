//! Restored and meshed shape regressions, each in an isolated worker under its group's policy.
use super::*;

use cases::{BooleanPath::*, Occ3721, UnifyInput::*};
use martensite_tkbo::bop_algo::Operation::{Common, Cut, Cut21, Fuse};
use model_view_cases as cases;
use std::error::Error;

/// A regression of the bugs group (1000 s CPU limit, bugs and common rules).
fn bugs(identity: &str, body: fn() -> Result<(), Box<dyn Error>>) {
    case_bugs_process_case::run_bugs_case(identity, body);
}

/// A bugs-group regression whose script declares `TODO` lines.
fn bugs_todo(
    identity: &str,
    todos: &[&'static str],
    body: impl FnOnce() -> Result<(), Box<dyn Error>>,
) {
    case_bugs_process_case::run_bugs_case_expecting(identity, todos, body);
}

/// A regression of the V3d group (1000 s CPU limit, V3d and common rules).
fn v3d(identity: &str, body: impl FnOnce() -> Result<(), Box<dyn Error>>) {
    case_v3d_process_case::run_case(identity, body);
}

#[test]
fn bug26372() {
    bugs(
        "model_view_regressions::bug26372",
        cases::incmesh_shaded_counts,
    );
}

#[test]
fn bug24127() {
    bugs(
        "model_view_regressions::bug24127",
        cases::bspline_face_deflection,
    );
}

#[test]
fn bug24959_1() {
    bugs(
        "model_view_regressions::bug24959_1",
        cases::bspline_face_bounded,
    );
}

#[test]
fn bug24959_2() {
    bugs(
        "model_view_regressions::bug24959_2",
        cases::long_bspline_face_bounded,
    );
}

#[test]
fn bug32471() {
    bugs(
        "model_view_regressions::bug32471",
        cases::planar_face_zero_deflection,
    );
}

#[test]
fn buc60707() {
    bugs(
        "model_view_regressions::buc60707",
        cases::fillet_shape_shaded,
    );
}

#[test]
fn bug31461() {
    bugs(
        "model_view_regressions::bug31461",
        cases::forced_coarser_mesh,
    );
}

#[test]
fn bug32767() {
    bugs("model_view_regressions::bug32767", cases::seam_edges_split);
}

#[test]
fn bug31258() {
    bugs(
        "model_view_regressions::bug31258",
        cases::restored_shaded_counts,
    );
}

#[test]
fn bug26716() {
    bugs(
        "model_view_regressions::bug26716",
        cases::shifted_wire_shaded,
    );
}

#[test]
fn bug26359() {
    bugs(
        "model_view_regressions::bug26359",
        cases::parabola_face_shaded,
    );
}

#[test]
fn bug258_1() {
    bugs("model_view_regressions::bug258_1", cases::shading_face_143);
}

#[test]
fn bug258_2() {
    bugs("model_view_regressions::bug258_2", cases::shading_face_144);
}

#[test]
fn bug6412() {
    bugs("model_view_regressions::bug6412", cases::own_shaded_present);
}

#[test]
fn bug32053() {
    bugs("model_view_regressions::bug32053", cases::scaled_face_mesh);
}

#[test]
fn bug22989() {
    bugs(
        "model_view_regressions::bug22989",
        cases::same_parameter_edge_length,
    );
}

#[test]
fn bug810_1() {
    bugs("model_view_regressions::bug810_1", cases::healed_shape_area);
}

#[test]
fn bug6412_1() {
    bugs(
        "model_view_regressions::bug6412_1",
        cases::area_with_empty_view,
    );
}

#[test]
fn bug19_2() {
    v3d(
        "model_view_regressions::bug19_2",
        cases::unfitted_own_shaded,
    );
}

#[test]
fn bug288_2() {
    v3d(
        "model_view_regressions::bug288_2",
        cases::own_shaded_not_faulty,
    );
}

#[test]
fn bug364() {
    v3d(
        "model_view_regressions::bug364",
        cases::invisible_face_shaded,
    );
}

#[test]
fn bug22304() {
    v3d(
        "model_view_regressions::bug22304",
        cases::v3d_tcleaned_shaded,
    );
}

#[test]
fn bug20373() {
    v3d(
        "model_view_regressions::bug20373",
        cases::bspline_sphere_shaded,
    );
}

#[test]
fn bug4894() {
    v3d(
        "model_view_regressions::bug4894",
        cases::cleaned_after_display,
    );
}

#[test]
fn bug23400() {
    v3d("model_view_regressions::bug23400", cases::thin_ring_top);
}

#[test]
fn bug288_8() {
    v3d(
        "model_view_regressions::bug288_8",
        cases::face_counts_shaded,
    );
}

#[test]
fn bug23886_1() {
    v3d("model_view_regressions::bug23886_1", || {
        let images = ["bug23886_1_1.png", "bug23886_1_2.png", "bug23886_1_3.png"];
        cases::notched_face_qualities("bug23886_face_01.brep", images)
    });
}

#[test]
fn bug23886_2() {
    v3d("model_view_regressions::bug23886_2", || {
        let images = ["bug23886_2_1.png", "bug23886_2_2.png", "bug23886_2_3.png"];
        cases::notched_face_qualities("bug23886_face_02.brep", images)
    });
}

#[test]
fn bug23886_3() {
    v3d("model_view_regressions::bug23886_3", || {
        let images = ["bug23886_3_1.png", "bug23886_3_2.png", "bug23886_3_3.png"];
        cases::notched_face_qualities("bug23886_face_03.brep", images)
    });
}

#[test]
fn bug23410() {
    bugs("model_view_regressions::bug23410", cases::green_box_color);
}

#[test]
fn bug3721_1() {
    bugs_todo(
        "model_view_regressions::bug3721_1",
        &["Error : The area of result shape is"],
        || {
            cases::occ3721_view(Occ3721 {
                swapped: false,
                path: Direct(Fuse),
                area: 71185.,
                image: "bug3721_1.png",
            })
        },
    );
}

#[test]
fn bug3721_2() {
    bugs_todo(
        "model_view_regressions::bug3721_2",
        &["Error : The area of result shape is"],
        || {
            cases::occ3721_view(Occ3721 {
                swapped: true,
                path: Direct(Fuse),
                area: 71185.,
                image: "bug3721_2.png",
            })
        },
    );
}

#[test]
fn bug3721_3() {
    bugs_todo(
        "model_view_regressions::bug3721_3",
        &["Error : The command is not valid. The area is 0"],
        || {
            cases::occ3721_view(Occ3721 {
                swapped: true,
                path: Direct(Common),
                area: 0.,
                image: "bug3721_3.png",
            })
        },
    );
}

#[test]
fn bug3721_4() {
    bugs("model_view_regressions::bug3721_4", || {
        cases::occ3721_view(Occ3721 {
            swapped: false,
            path: Direct(Cut),
            area: 71185.,
            image: "bug3721_4.png",
        })
    });
}

#[test]
fn bug3721_5() {
    bugs("model_view_regressions::bug3721_5", || {
        cases::occ3721_view(Occ3721 {
            swapped: true,
            path: Direct(Cut),
            area: 71185.,
            image: "bug3721_5.png",
        })
    });
}

#[test]
fn bug3721_6() {
    bugs("model_view_regressions::bug3721_6", || {
        cases::occ3721_view(Occ3721 {
            swapped: false,
            path: Prepared(Cut21),
            area: 71185.,
            image: "bug3721_6.png",
        })
    });
}

#[test]
fn bug3721_7() {
    bugs("model_view_regressions::bug3721_7", || {
        cases::occ3721_view(Occ3721 {
            swapped: true,
            path: Prepared(Cut21),
            area: 71185.,
            image: "bug3721_7.png",
        })
    });
}

#[test]
fn bug6278() {
    bugs(
        "model_view_regressions::bug6278",
        cases::fused_vertex_position,
    );
}

#[test]
fn bug23738() {
    bugs(
        "model_view_regressions::bug23738",
        cases::prepared_face_fuse,
    );
}

#[test]
fn bug22910_2() {
    bugs(
        "model_view_regressions::bug22910_2",
        cases::surface_face_meshed,
    );
}

#[test]
fn bug13() {
    bugs(
        "model_view_regressions::bug13",
        cases::surface_faces_section,
    );
}

#[test]
fn bug22822() {
    bugs(
        "model_view_regressions::bug22822",
        cases::self_intersecting_open_wire,
    );
}

#[test]
fn bug31631() {
    bugs("model_view_regressions::bug31631", cases::rail_pipe_faces);
}

#[test]
fn bug32601() {
    case_bugs_process_case::run_bugs_case_within(
        "model_view_regressions::bug32601",
        30,
        cases::bad_triangulation_shaded,
    );
}

#[test]
fn bug26489_1() {
    bugs("model_view_regressions::bug26489_1", || {
        cases::unified_view(
            PlaneOfWire("bug26489_p01usd.brep"),
            [4, 4, 1, 1, 0, 0, 0, 0, 10],
            "bug26489_1.png",
        )
    });
}

#[test]
fn bug26489_2() {
    bugs("model_view_regressions::bug26489_2", || {
        cases::unified_view(
            Restored("bug26489_p01usd.brep"),
            [4, 4, 1, 0, 0, 0, 0, 0, 9],
            "bug26489_2.png",
        )
    });
}

#[test]
fn bug26489_3() {
    bugs("model_view_regressions::bug26489_3", || {
        cases::unified_view(
            Restored("bug26489_3cir.brep"),
            [1, 1, 1, 1, 0, 0, 0, 0, 4],
            "bug26489_3.png",
        )
    });
}

#[test]
fn bug26489_5() {
    bugs("model_view_regressions::bug26489_5", || {
        cases::unified_view(
            FirstWire("bug26489_r003.brep"),
            [4, 4, 1, 0, 0, 0, 0, 0, 9],
            "bug26489_5.png",
        )
    });
}

#[test]
fn bug26489_6() {
    bugs("model_view_regressions::bug26489_6", || {
        cases::unified_view(
            Restored("bug26489_r002.brep"),
            [4, 4, 1, 1, 1, 0, 0, 1, 12],
            "bug26489_6.png",
        )
    });
}

#[test]
fn bug26489_4() {
    bugs(
        "model_view_regressions::bug26489_4",
        cases::unified_face_history,
    );
}

#[test]
fn bug22554_1() {
    bugs(
        "model_view_regressions::bug22554_1",
        cases::erased_points_circle,
    );
}

#[test]
fn bug22554_2() {
    bugs(
        "model_view_regressions::bug22554_2",
        cases::erased_points_plane,
    );
}

#[test]
fn bug22554_3() {
    bugs(
        "model_view_regressions::bug22554_3",
        cases::erased_points_triangle,
    );
}

#[test]
fn bug22554_4() {
    bugs(
        "model_view_regressions::bug22554_4",
        cases::erased_points_segment,
    );
}
