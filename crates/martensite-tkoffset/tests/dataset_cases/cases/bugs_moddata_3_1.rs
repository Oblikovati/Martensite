//! Script case tables.
#[allow(unused_imports)]
use crate::conditions::{Cond, Value};
#[allow(unused_imports)]
use crate::extra_steps::*;
#[allow(unused_imports)]
use crate::groups::{End, WireEnd};
#[allow(unused_imports)]
use crate::steps::*;
#[allow(unused_imports)]
use crate::support::{properties::Prop, session::ViewStep};

offset_case!(
    bugs_moddata_3_buc60634,
    "tests/bugs/moddata_3/buc60634",
    Bugs,
    1000,
    End::Completed,
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("Output with mistake is correct!"),
        Step::Puts("TODO #60634 ALL: Standard_ConstructionError: BRepFillDraft"),
        Step::Puts("=================="),
        Step::Puts("BUC60634"),
        Step::Puts("=================="),
        Step::More(More::Surface(Surface::Sphere("s", &[], 10.0))),
        Step::Extra(Extra::MkShell("sh", "s")),
        Step::Topo(Topo::Explode("sh", "f")),
        Step::Run(crate::one_off::draft_of_sphere),
    ]
);

offset_case!(
    bugs_moddata_3_bug23025,
    "tests/bugs/moddata_3/bug23025",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("================"),
        Step::Puts("23025"),
        Step::Puts("================"),
        Step::Topo(Topo::Restore("brep/bug23025_u.brep", "u")),
        Step::Topo(Topo::Explode("u", "e")),
        Step::Profile(Profile::Wire("w", &["u_1", "u_2", "u_3"])),
        Step::Extra(Extra::Draft("result", "w", [0.0, 0.0, 1.0], 0.2, "100", -1)),
        Step::Check(Check::Shape("result")),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_moddata_3_bug25940,
    "tests/bugs/moddata_3/bug25940",
    Bugs,
    1000,
    End::Completed,
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("TODO 25940 ALL: Error: MakeThickSolid creates C0 continuous edges"),
        Step::Puts("============"),
        Step::Puts("25940"),
        Step::Puts("============"),
        Step::Topo(Topo::Restore(
            "brep/bug25940_thickness-extrude1-tcl-Face.brep",
            "Face"
        )),
        Step::Sweep(Sweep::Prism("Extrude", "Face", [-102.0, 0.0, 0.0])),
        Step::Topo(Topo::Explode("Extrude", "F")),
        Step::Offset(Offset::Parameter(1e-07, false, 'a', false)),
        Step::Offset(Offset::Load("Extrude", 3.34, &["Extrude_5"])),
        Step::Offset(Offset::Perform("Thickness001")),
        Step::Run(crate::one_off::continuity_report),
        Step::View(ViewStep::SmallView),
        Step::View(ViewStep::Donly(&["Thickness001"])),
        Step::View(ViewStep::Fit),
        Step::Screenshot("photo"),
        Step::View(ViewStep::Donly(&["Extrude"])),
        Step::Screenshot("photo"),
        Step::View(ViewStep::Donly(&["Face"])),
        Step::Screenshot("photo"),
        Step::CheckView("Thickness001"),
    ]
);
