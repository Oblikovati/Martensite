use crate::table::*;

step_case!(
    de_wrapper_step_a1,
    StepCase {
        source: "tests/de_wrapper/step/A1",
        group: "de_wrapper",
        cpu_seconds: None,
        recorded: RecordedVerdict::Ok,
        steps: &[
            Step::Puts("============"),
            Step::Puts(
                "0032821: DEWrapper - Implementation of a common toolkit for importing and exporting CAD files"
            ),
            Step::Puts("============"),
            Step::Puts(""),
            Step::Catch(&Step::Close("D_First")),
            Step::Catch(&Step::Close("D_Second")),
            Step::ReadStep("D_First", Place::Data("bm2_pe_t4-A.stp")),
            Step::XGetOneShape("S_First", "D_First"),
            Step::ReadFile("D_Second", Place::Data("bm2_pe_t4-A.stp")),
            Step::XGetOneShape("S_Second", "D_Second"),
            Step::CheckNbShapesLike("S_Second", "S_First"),
            Step::Close("D_First"),
            Step::Close("D_Second"),
        ]
    }
);
step_case!(
    de_wrapper_step_a2,
    StepCase {
        source: "tests/de_wrapper/step/A2",
        group: "de_wrapper",
        cpu_seconds: None,
        recorded: RecordedVerdict::Ok,
        steps: &[
            Step::Puts("============"),
            Step::Puts(
                "0032821: DEWrapper - Implementation of a common toolkit for importing and exporting CAD files"
            ),
            Step::Puts("============"),
            Step::Puts(""),
            Step::Catch(&Step::Close("D_First")),
            Step::Catch(&Step::Close("D_Second")),
            Step::ReadStep("D_First", Place::Data("bm2_pe_t4-A.stp")),
            Step::XGetOneShape("S_First", "D_First"),
            Step::WriteFile("D_First", Place::Image("A2.stp")),
            Step::ReadStep("D_Second", Place::Image("A2.stp")),
            Step::XGetOneShape("S_Second", "D_Second"),
            Step::CheckNbShapesLike("S_Second", "S_First"),
            Step::FileDelete(Place::Image("A2.stp")),
            Step::Close("D_First"),
            Step::Close("D_Second"),
        ]
    }
);
step_case!(
    de_wrapper_step_a3,
    StepCase {
        source: "tests/de_wrapper/step/A3",
        group: "de_wrapper",
        cpu_seconds: None,
        recorded: RecordedVerdict::Ok,
        steps: &[
            Step::Puts("============"),
            Step::Puts(
                "0032821: DEWrapper - Implementation of a common toolkit for importing and exporting CAD files"
            ),
            Step::Puts("============"),
            Step::Puts(""),
            Step::Catch(&Step::Close("D_First")),
            Step::Catch(&Step::Close("D_Second")),
            Step::ReadStep("D_First", Place::Data("bm2_pe_t4-A.stp")),
            Step::XGetOneShape("S_First", "D_First"),
            Step::WriteShapeFile("S_First", Place::Image("A3.stp")),
            Step::ReadStep("D_Second", Place::Image("A3.stp")),
            Step::XGetOneShape("S_Second", "D_Second"),
            Step::CheckNbShapesLike("S_Second", "S_First"),
            Step::FileDelete(Place::Image("A3.stp")),
            Step::Close("D_First"),
            Step::Close("D_Second"),
        ]
    }
);
step_case!(
    de_wrapper_step_a4,
    StepCase {
        source: "tests/de_wrapper/step/A4",
        group: "de_wrapper",
        cpu_seconds: None,
        recorded: RecordedVerdict::Ok,
        steps: &[
            Step::Puts("============"),
            Step::Puts(
                "0032821: DEWrapper - Implementation of a common toolkit for importing and exporting CAD files"
            ),
            Step::Puts("============"),
            Step::Puts(""),
            Step::ReadShapeFile("S_First", Place::Data("bm2_pe_t4-A.stp")),
            Step::WriteShapeFile("S_First", Place::Image("A4.stp")),
            Step::ReadShapeFile("S_Second", Place::Image("A4.stp")),
            Step::CheckNbShapesLike("S_Second", "S_First"),
            Step::FileDelete(Place::Image("A4.stp")),
        ]
    }
);
