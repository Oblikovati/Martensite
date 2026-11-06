use crate::table::*;

step_case!(
    bugs_step_bug24283_1,
    StepCase {
        source: "tests/bugs/step/bug24283_1",
        group: "bugs",
        cpu_seconds: Some(1000),
        recorded: RecordedVerdict::Ok,
        steps: &[
            Step::Puts("============"),
            Step::Puts("24283"),
            Step::Puts("============"),
            Step::Puts(""),
            Step::ReadStep("D", Place::Data("bug24283_spheres.stp")),
        ]
    }
);
step_case!(
    bugs_step_bug25415,
    StepCase {
        source: "tests/bugs/step/bug25415",
        group: "bugs",
        cpu_seconds: Some(1000),
        recorded: RecordedVerdict::Ok,
        steps: &[
            Step::Puts("========"),
            Step::Puts("25415"),
            Step::Puts("========"),
            Step::Puts(""),
            Step::StepRead(
                Place::Data("bug25415_part_of_hdrv8w64p254_4x2_1016x502h838p.stp"),
                "a"
            ),
            Step::Command("checkshape", &["a_1"]),
        ]
    }
);
step_case!(
    bugs_step_bug27342,
    StepCase {
        source: "tests/bugs/step/bug27342",
        group: "bugs",
        cpu_seconds: Some(1000),
        recorded: RecordedVerdict::Ok,
        steps: &[
            Step::Puts("=========="),
            Step::Puts("0027342: Data Exchange, STEP - support streams for import / export"),
            Step::Puts("=========="),
            Step::Puts(""),
            Step::Puts("Testing read of STEP file using stream interface"),
            Step::TestReadStream(Place::Data("PRO7071.stp"), "result"),
            Step::Command("checkshape", &["result"]),
            Step::Command("checknbshapes", &["result", "-solid", "4"]),
        ]
    }
);
step_case!(
    bugs_step_bug29478,
    StepCase {
        source: "tests/bugs/step/bug29478",
        group: "bugs",
        cpu_seconds: Some(1000),
        recorded: RecordedVerdict::Ok,
        steps: &[
            Step::Puts("============"),
            Step::Puts("29478"),
            Step::Puts("============"),
            Step::Puts(""),
            Step::TestReadStep(Place::Data("bug29478.stp"), "orig"),
            Step::Command("checkshape", &["orig"]),
        ]
    }
);
step_case!(
    bugs_step_bug30053,
    StepCase {
        source: "tests/bugs/step/bug30053",
        group: "bugs",
        cpu_seconds: Some(1000),
        recorded: RecordedVerdict::Ok,
        steps: &[
            Step::Puts("==================================================="),
            Step::Puts(" 0030533: STEP read fails due to comment string "),
            Step::Puts("==================================================="),
            Step::Puts(""),
            Step::ReadStep("D", Place::Data("bug30053.stp")),
            Step::Close("D"),
        ]
    }
);
step_case!(
    bugs_step_bug30790,
    StepCase {
        source: "tests/bugs/step/bug30790",
        group: "bugs",
        cpu_seconds: Some(1000),
        recorded: RecordedVerdict::Known,
        steps: &[
            Step::Puts("========================"),
            Step::Puts("0030790: Crash when loading specific step file"),
            Step::Puts("========================"),
            Step::Puts("TODO 30790 ALL: Faulty shapes in variables faulty_1 to"),
            Step::StepRead(Place::Data("bug30790.step"), "a"),
            Step::TpCompound("result"),
            Step::Command("checkshape", &["result"]),
            Step::CheckView("result"),
        ]
    }
);
step_case!(
    bugs_step_bug31292,
    StepCase {
        source: "tests/bugs/step/bug31292",
        group: "bugs",
        cpu_seconds: Some(1000),
        recorded: RecordedVerdict::Ok,
        steps: &[
            Step::Puts("========================"),
            Step::Puts(
                "0031292: Data Exchange - SIGSEGV on reading STEP file with references to invalid entities"
            ),
            Step::Puts("========================"),
            Step::StepRead(Place::Data("bug31292.stp"), "a"),
            Step::RenameVar("a_1", "result"),
            Step::Command("checkshape", &["result"]),
            Step::CheckView("result"),
        ]
    }
);
step_case!(
    bugs_step_bug32914,
    StepCase {
        source: "tests/bugs/step/bug32914",
        group: "bugs",
        cpu_seconds: Some(1000),
        recorded: RecordedVerdict::Ok,
        steps: &[
            Step::Puts("==================================================="),
            Step::Puts(
                " 0032914: Data Exchange - Some parts of compound are lost while writing STEP in nonmanifold mode "
            ),
            Step::Puts("==================================================="),
            Step::Puts(""),
            Step::Restore(Place::Data("OCC32914.brep"), "s"),
            Step::Param("write.step.nonmanifold", "1"),
            Step::StepWrite("0", "s", Place::Image("32914.step")),
            Step::TestReadStep(Place::Image("32914.step"), "result"),
            Step::Command(
                "checknbshapes",
                &[
                    "result",
                    "-vertex",
                    "16",
                    "-edge",
                    "17",
                    "-wire",
                    "7",
                    "-face",
                    "7",
                    "-shell",
                    "2",
                    "-solid",
                    "0",
                    "-compsolid",
                    "0",
                    "-compound",
                    "4",
                    "-shape",
                    "53"
                ]
            ),
            Step::CheckView("result"),
        ]
    }
);
step_case!(
    bugs_step_bug33410,
    StepCase {
        source: "tests/bugs/step/bug33410",
        group: "bugs",
        cpu_seconds: Some(1000),
        recorded: RecordedVerdict::Ok,
        steps: &[
            Step::Puts("========================"),
            Step::Puts(
                "0033410: Data Exchange, Step Import - TRIANGULATED_FACE from STEP where there are no pnval entries"
            ),
            Step::Puts("========================"),
            Step::ReadStep("D", Place::Data("bug33410.step")),
            Step::XGetOneShape("sh", "D"),
            Step::Command(
                "checktrinfo",
                &["sh", "-face", "1", "-nod", "81", "-tri", "128"]
            ),
            Step::Close("D"),
        ]
    }
);
step_case!(
    bugs_step_bug33665,
    StepCase {
        source: "tests/bugs/step/bug33665",
        group: "bugs",
        cpu_seconds: Some(1000),
        recorded: RecordedVerdict::Ok,
        steps: &[
            Step::Puts("# ====================================================================="),
            Step::Puts("# 0033665: Data Exchange, Step Import - TransferRoots crashes for"),
            Step::Puts("# invalid STEP files."),
            Step::Puts("# ====================================================================="),
            Step::TestReadStep(Place::Data("bug33665.step"), "aShape"),
            Step::Command("checkshape", &["aShape"]),
        ]
    }
);
step_case!(
    bugs_step_bug5215,
    StepCase {
        source: "tests/bugs/step/bug5215",
        group: "bugs",
        cpu_seconds: Some(1000),
        recorded: RecordedVerdict::Ok,
        steps: &[
            Step::Puts("============"),
            Step::Puts("5215"),
            Step::Puts("============"),
            Step::Puts(""),
            Step::StepRead(Place::Data("trj4_k1_geo-md-203.stp"), "a"),
            Step::TpCompound("result"),
            Step::Command("checkshape", &["result"]),
            Step::CheckView("result"),
        ]
    }
);
