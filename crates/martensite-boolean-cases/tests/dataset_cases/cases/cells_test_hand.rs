//! `cells_test` bodies written by hand: the script reads the names `explode` answers with.
use crate::support::{
    beyond_script::part_count,
    bop_session::BopSession,
    cells_session::CellsSession,
    checkprops::{Expected, checkprops},
    commands::{explode, make_box},
    hooks::require_whatis,
    nbshapes::{Counting, NbCount, checknbshapes_named},
    properties::Prop,
    session::Session,
    surface_commands::mkface_bounded,
    topology_commands::plane_along,
};

pub fn J3(s: &mut dyn Session) {
    let mut bop = BopSession::default();
    let mut cells = CellsSession::default();
    println!("========");
    println!("OCC28528");
    println!("========");
    println!();
    println!("Arguments: three interfering boxes cut by a plane");
    make_box(s, "b1", [0., 0., 0.], [10., 10., 10.]);
    make_box(s, "b2", [7., 0., 0.], [10., 10., 10.]);
    make_box(s, "b3", [5., 0., 5.], [10., 10., 10.]);
    let geom_p = plane_along([5., 5., 5.], [0., 1., 0.]);
    mkface_bounded(s, "f", &geom_p.surface, (-20., 20., -20., 20.));
    bop.arguments(s, &["b1", "b2", "b3"], &["f"]);
    bop.bfillds();
    cells.bcbuild(s, &mut bop, "r");
    println!("Results");
    println!("All split shapes");
    cells.bcaddall(s, "result", 0, false);
    checknbshapes_named(
        s,
        "result",
        Counting::Located,
        &[NbCount::Solids(14)],
        "Number of SOLIDs in the result containing all splits",
    );
    checknbshapes_named(
        s,
        "result",
        Counting::Located,
        &[NbCount::Faces(66)],
        "Number of FACEs in the result containing all splits",
    );
    checkprops(s, "result", Prop::Area, Expected::Mass(4120.));
    checkprops(s, "result", Prop::Volume, Expected::Mass(2200.));
    println!("Splits of solids");
    cells.bcremove(s, "result", &[("f", 1)]);
    checknbshapes_named(
        s,
        "result",
        Counting::Located,
        &[NbCount::Solids(14)],
        "Number of SOLIDs in the result containing only splits of solids",
    );
    checknbshapes_named(
        s,
        "result",
        Counting::Located,
        &[NbCount::Faces(65)],
        "Number of FACEs in the result containing only splits of solids",
    );
    checkprops(s, "result", Prop::Area, Expected::Mass(2520.));
    checkprops(s, "result", Prop::Volume, Expected::Mass(2200.));
    println!("Splits of the face");
    cells.bcremoveall();
    cells.bcadd(s, "result", &[("f", 1)], 0, false);
    checknbshapes_named(
        s,
        "result",
        Counting::Located,
        &[NbCount::Faces(8)],
        "Number of FACEs in the result containing only splits of the face",
    );
    checkprops(s, "result", Prop::Area, Expected::Mass(1600.));
    println!("Cut solids from face");
    cells.bcremoveall();
    cells.bcadd(
        s,
        "result",
        &[("f", 1), ("b1", 0), ("b2", 0), ("b3", 0)],
        0,
        false,
    );
    checknbshapes_named(
        s,
        "result",
        Counting::Located,
        &[NbCount::Faces(1)],
        "Number of FACEs in the result of CUT(face, solids) operation",
    );
    checkprops(s, "result", Prop::Area, Expected::Mass(1380.));
    println!("Splits of only one solid");
    cells.bcremoveall();
    cells.bcadd(s, "result", &[("b1", 1)], 0, false);
    checknbshapes_named(
        s,
        "result",
        Counting::Located,
        &[NbCount::Solids(8)],
        "Number of FACEs in the result containing only the splits of first solid",
    );
    checkprops(s, "result", Prop::Volume, Expected::Mass(1000.));
    println!("Making containers");
    println!("Make SHELL and COMPSOLID from all splits parts");
    cells.bcremoveall();
    cells.bcaddall(s, "result", 0, false);
    cells.bcmakecontainers(s, "result");
    explode(s, "result", None);
    require_whatis(s, "result_1", "SHELL", "Error: Shell is not created");
    checknbshapes_named(
        s,
        "result_1",
        Counting::Located,
        &[NbCount::Faces(8)],
        "Number of FACEs in the SHELL built from all splits of face",
    );
    require_whatis(
        s,
        "result_2",
        "COMPSOLID",
        "Error: COMPSOLID is not created",
    );
    checknbshapes_named(
        s,
        "result_2",
        Counting::Located,
        &[NbCount::Solids(14)],
        "Number of SOLIDs in the COMPSOLID built from splits of all solids",
    );
    println!("Remove splits of the first solid from the COMPSOLID");
    cells.bcremove(s, "result", &[("b1", 1)]);
    explode(s, "result", None);
    require_whatis(s, "result_1", "SHELL", "Error: Shell has been destroyed");
    require_whatis(
        s,
        "result_2",
        "COMPSOLID",
        "Error: COMPSOLID has been destroyed",
    );
    checknbshapes_named(
        s,
        "result_2",
        Counting::Located,
        &[NbCount::Solids(6)],
        "Number of SOLIDs in the COMPSOLID built from splits of two solids not included in the first solid",
    );
    println!("Remove splits of face included into first solid from the shell");
    cells.bcremove(s, "result", &[("f", 1), ("b1", 1)]);
    explode(s, "result", None);
    require_whatis(s, "result_1", "SHELL", "Error: Shell has been destroyed");
    checknbshapes_named(
        s,
        "result_1",
        Counting::Located,
        &[NbCount::Faces(4)],
        "Number of FACEs in the SHELL built from splits of face not included into the first solid",
    );
    require_whatis(
        s,
        "result_2",
        "COMPSOLID",
        "Error: COMPSOLID has been destroyed",
    );
    println!("Destroy the shell completely by removing all splits of the face from the result");
    cells.bcremove(s, "result", &[("f", 1)]);
    if explode(s, "result", None)
        .iter()
        .any(|name| name.contains("_2"))
    {
        println!("Error: Shell has not been destroyed");
    }
    require_whatis(
        s,
        "result_1",
        "COMPSOLID",
        "Error: COMPSOLID has been destroyed",
    );
    println!("Removing internal boundaries");
    println!(
        "Remove internal boundaries in the result of COMMON operation between first two solids"
    );
    cells.bcremoveall();
    cells.bcadd(s, "result", &[("b1", 1), ("b2", 1)], 1, true);
    checknbshapes_named(
        s,
        "result",
        Counting::Located,
        &[NbCount::Solids(1)],
        "Number of SOLIDs in the result of COMMON(b1, b2) operation",
    );
    println!(
        "Remove internal boundaries in the result of COMMON operation between first solid and face"
    );
    cells.bcremoveall();
    cells.bcadd(s, "result", &[("b1", 1), ("f", 1)], 1, true);
    checknbshapes_named(
        s,
        "result",
        Counting::Located,
        &[NbCount::Faces(1)],
        "Number of FACEs in the result of COMMON(b1, f) operation",
    );
    println!("Make Fuse operation between two solids and face");
    cells.bcremoveall();
    cells.bcadd(s, "result", &[("b1", 1)], 1, false);
    cells.bcadd(s, "result", &[("b3", 1)], 1, false);
    cells.bcadd(s, "result", &[("f", 1), ("b1", 0), ("b3", 0)], 2, false);
    cells.bcmakecontainers(s, "result");
    cells.bcremoveint(s, "result");
    let expl = explode(s, "result", None).join(" ");
    if !expl.contains("result_1 result_2") {
        println!("Error: The unification did not work");
    }
    if expl.contains("result_3") {
        println!("Error: The unification did not work");
    }
    require_whatis(
        s,
        "result_1",
        "SOLID",
        "Error: The solids have not been fused",
    );
    require_whatis(
        s,
        "result_2",
        "FACE",
        "Error: The faces have not been unified",
    );
    println!("Fuse splits of solids with different material for each solid");
    cells.bcremoveall();
    cells.bcadd(s, "result", &[("b1", 1)], 1, true);
    cells.bcadd(s, "result", &[("b2", 1)], 2, true);
    cells.bcadd(s, "result", &[("b3", 1)], 3, true);
    checknbshapes_named(
        s,
        "result",
        Counting::Located,
        &[NbCount::Solids(3)],
        "Number of shapes in the result of Unification of splits of each solid",
    );
    // Beyond the script: checks an earlier test of this case made.
    part_count(s, "result", None, 3);
}
