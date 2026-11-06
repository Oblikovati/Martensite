use crate::table::*;

step_case!(
    bugs_heal_bug24983,
    StepCase {
        source: "tests/bugs/heal/bug24983",
        group: "bugs",
        cpu_seconds: Some(1000),
        recorded: RecordedVerdict::Ok,
        steps: &[
            Step::Puts("========"),
            Step::Puts("0024983: For the incorrect seam edge in STEP file no fix is provided"),
            Step::Puts("========"),
            Step::StepRead(Place::Data("bug24983_111FACE.stp"), "a"),
            Step::TpCompound("result"),
            Step::Command("checkshape", &["result"]),
            Step::CheckView("result"),
        ]
    }
);
step_case!(
    bugs_heal_bug25013_1,
    StepCase {
        source: "tests/bugs/heal/bug25013_1",
        group: "bugs",
        cpu_seconds: Some(1000),
        recorded: RecordedVerdict::Ok,
        steps: &[
            Step::Puts("========="),
            Step::Puts(" 25013"),
            Step::Puts("========="),
            Step::Puts(""),
            Step::StepRead(Place::Data("bug25013_25013faces.stp"), "a"),
            Step::TpCompound("result"),
            Step::Command("checkshape", &["result"]),
            Step::CheckView("result"),
        ]
    }
);
step_case!(
    bugs_heal_bug25013_2,
    StepCase {
        source: "tests/bugs/heal/bug25013_2",
        group: "bugs",
        cpu_seconds: Some(1000),
        recorded: RecordedVerdict::Ok,
        steps: &[
            Step::Puts("========="),
            Step::Puts(" 25013"),
            Step::Puts("========="),
            Step::Puts(""),
            Step::StepRead(Place::Data("bug25013_25013face.stp"), "a"),
            Step::TpCompound("result"),
            Step::Command("checkshape", &["result"]),
            Step::CheckView("result"),
        ]
    }
);
