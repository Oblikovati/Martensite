//! The two viewer cases the generator does not read: their scripts branch on command output.
use crate::steps::{Bool, Place, Prim, Step, V3d};
use crate::support::checkview::CheckView;

const RESULT_VIEW: Step = Step::CheckView(CheckView {
    display: "result",
    with: &[],
    otherwise: &[],
    property: None,
    image: "bug28119_1-2d",
});

// Its script's CPU budget is about a machine it has to itself: unoptimized it takes 163 s of its
// 200 s alone, and runs out beside the other cases. The budget is unchanged.
blend_case!(
    alone bugs_modalg_7_bug28119_1,
    "tests/bugs/modalg_7/bug28119_1",
    group: BugsCpu200,
    steps: &[
        Step::Puts("========"),
        Step::Puts("OCC28119: Blend fails on fused identical but shifted tori"),
        Step::Puts("========"),
        Step::Puts("TODO ?OCC28119 ALL: Error: Cannot find the result of BLEND-operation. The result of BOP operation will be returned."),
        Step::Puts("TODO ?OCC29910 ALL: exception"),
        Step::Puts("TODO ?OCC29910 ALL: Exception"),
        Step::Puts("TODO ?OCC29910 ALL: Error: Exception in bfuseblend operation"),
        Step::Puts("CPU and elapsed time limit set to 200 seconds"),
        Step::Prim(Prim::Torus("t1", &[100.0, 10.0])),
        Step::Prim(Prim::Torus("t2", &[100.0, 10.0])),
        Step::Place(Place::Translate(&["t2"], [100.0, 0.0, 0.0])),
        Step::Caught(
            &Step::Bool(Bool::FuseBlend("result", "t1", "t2", 5.0)),
            "Error: Exception in bfuseblend operation.",
            "",
        ),
        RESULT_VIEW,
        Step::V3d(V3d::ResultView("result", "bug28119_1-3d.png")),
    ]
);

// The QA command builds its shapes and shows each in the viewer `vinit` opened.
blend_case!(
    bugs_moddata_3_buc60811,
    "tests/bugs/moddata_3/buc60811",
    group: Bugs,
    steps: &[
        Step::Puts("REQUIRED All: Error is 5"),
        Step::Puts("========================"),
        Step::Puts("BUC60811"),
        Step::Puts("========================"),
        Step::V3d(V3d::Init),
        Step::Sample(crate::steps::Sample::Buc60811),
        Step::V3d(V3d::ShowBuc60811),
    ]
);
