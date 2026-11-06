use crate::table::*;

step_case!(
    bugs_mesh_bug27693,
    StepCase {
        source: "tests/bugs/mesh/bug27693",
        group: "bugs",
        cpu_seconds: Some(1000),
        recorded: RecordedVerdict::Ok,
        steps: &[
            Step::Puts("======="),
            Step::Puts(
                "0027693: BRepMesh: fine edge tessellation leads to excess number of triangles along the whole U or V stripes"
            ),
            Step::Puts("======="),
            Step::Puts(""),
            Step::StepRead(Place::Data("bug27693_cornetta-step.stp"), "a"),
            Step::RenameVar("a_1", "a"),
            Step::Explode("a", "f"),
            Step::Copy("a_4", "f"),
            Step::Draw2d("don", &["f"]),
            Step::Command("incmesh", &["f", "0.01", "-a", "90"]),
            Step::Draw2d("isos", &["f", "0"]),
            Step::Draw2d("triangles", &["f"]),
            Step::Command(
                "checktrinfo",
                &[
                    "f",
                    "-tri",
                    "2220",
                    "-nod",
                    "1232",
                    "-defl",
                    "0.0099011902071009586",
                    "-tol_abs_defl",
                    "1e-6"
                ]
            ),
            Step::Draw2d("smallview", &["+X+Y"]),
            Step::Draw2d("fit", &[]),
            Step::Screenshot("photo"),
        ]
    }
);
