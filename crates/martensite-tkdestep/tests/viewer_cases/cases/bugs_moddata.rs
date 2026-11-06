//! `tests/bugs/moddata_3`.
use crate::grading::BUGS;
use crate::host::ViewerHost;
use crate::support::nbshapes::NbCount;
use crate::support::rules::Verdict;
use crate::table::Place;
use crate::viewer_case;

fn bug25971(host: &mut ViewerHost) {
    let file = "bug25971_a_328_1.brep";
    println!("restore {file} a ");
    crate::support::restore::restore(&mut host.session.raster, file, "a");
    host.fixshape("f", "a", 1e-6);
    println!("explode f e ");
    host.session.explode("f", "e");
    host.dump_shape("f_4");
    host.session.pilot_command("newmodel");
    let saved = host.session.path_of(Place::Image("bug25971.stp"));
    let _ = std::fs::remove_file(&saved);
    println!("stepwrite a a {saved} ");
    host.session.stepwrite("a", "a", &saved);
    println!("stepread {saved} ff * ");
    host.session.stepread(&saved, "ff");
    let _ = std::fs::remove_file(&saved);
    println!("checknbshapes ff_1 -t -m Saved shape ");
    host.checknbshapes_typed(
        "ff_1",
        "Saved shape",
        &[
            NbCount::Vertices(4),
            NbCount::Edges(5),
            NbCount::Wires(1),
            NbCount::Faces(1),
            NbCount::Shells(1),
            NbCount::Solids(0),
            NbCount::CompSolids(0),
            NbCount::Compounds(0),
            NbCount::Shapes(12),
        ],
    );
    host.session.pilot_command("tpstat c");
    host.vinit_named("View1");
    host.vdisplay("a");
    host.vfit();
    host.vdump("bug25971_1");
    host.vinit_named("View2");
    host.vdisplay("ff_1");
    host.vfit();
    host.vdump("bug25971_2");
}
viewer_case!(
    bugs_moddata_3_bug25971,
    "tests/bugs/moddata_3/bug25971",
    BUGS,
    Verdict::Ok,
    bug25971
);

fn bug30497(host: &mut ViewerHost) {
    let file = host.session.path_of(Place::Data("as1-oc-214-mat.stp"));
    println!("testreadstep {file} s ");
    host.session.testreadstep(&file, "s");
    host.vclear();
    host.vinit_named("View1");
    host.vaxo();
    host.vdisplay_shaded("s", "s -dispmode 1");
    host.vfit();
    host.vselmode_on(2);
    host.vmoveto(150, 201);
    host.vdump("bug30497");
}
viewer_case!(
    bugs_moddata_3_bug30497,
    "tests/bugs/moddata_3/bug30497",
    BUGS,
    Verdict::Ok,
    bug30497
);
