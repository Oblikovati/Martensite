//! The `bugs_modalg_1` case bodies, part 1; each statement is one line of its script.
use super::*;

pub fn buc60462_2(s: &mut dyn Session) {
    println!(
        "TODO OCC27024 ALL: Error : operation bfuse is WRONG because number of SOLID entities in shape"
    );
    println!("==========");
    println!("BUC60462");
    println!("==========");
    restore(s, "buc60462c.brep", "a");
    checkshape_named(s, "a");
    restore(s, "buc60462d.brep", "b");
    checkshape_named(s, "b");
    println!("Fuse begin");
    bfuse(s, "result", "a", "b");
    println!("Fuse end");
    checkshape_named(s, "result");
    checknbshapes_named(
        s,
        "result",
        Counting::Located,
        &[NbCount::Solids(1)],
        "operation bfuse",
    );
    checkprops(s, "result", Prop::Area, Expected::Mass(93237.9));
    checkview(s, &CheckView::of("result"));
}

pub fn buc60531_1(s: &mut dyn Session) {
    println!("========================");
    println!("BUC60531");
    println!("========================");
    restore(s, "buc60531a.brep", "r1");
    checkshape_named(s, "r1");
    restore(s, "buc60531b.brep", "pl1");
    checkshape_named(s, "pl1");
    bcut(s, "result", "r1", "pl1");
    checkprops(s, "result", Prop::Area, Expected::Mass(119879.));
    checkshape_named(s, "result");
    checkview(s, &CheckView::of("result"));
}

pub fn buc60531_2(s: &mut dyn Session) {
    println!("========================");
    println!("BUC60531");
    println!("========================");
    println!("TODO OCC12345 ALL: Error : The area of result shape is");
    restore(s, "buc60531a.brep", "r1");
    checkshape_named(s, "r1");
    restore(s, "buc60531b.brep", "pl1");
    checkshape_named(s, "pl1");
    bcut(s, "r2", "r1", "pl1");
    checkshape_named(s, "r2");
    restore(s, "buc60531c.brep", "pl2");
    checkshape_named(s, "pl2");
    bcut(s, "result", "r2", "pl2");
    checkprops(s, "result", Prop::Area, Expected::Mass(115935.));
    checkshape_named(s, "result");
    checkview(s, &CheckView::of("result"));
}

pub fn buc60585_1(s: &mut dyn Session) {
    println!("========================");
    println!("BUC60585");
    println!("========================");
    restore(s, "buc60585a.brep", "s");
    checkshape_named(s, "s");
    let geom_ps = plane_axes(
        [2036.25, -97.5, -1460.499755859375],
        [0.0, -0.75103523489975432, -0.66026212668838646],
        [1.0, 0.0, 0.0],
    );
    mkface(s, "fs", &geom_ps.surface);
    bsection(s, "result", "s", "fs");
    checkprops(s, "result", Prop::Length, Expected::Mass(117.81));
    checkshape_named(s, "result");
    checksection(s, "result");
    checkview(s, &CheckView::of("result"));
}

pub fn buc60585_2(s: &mut dyn Session) {
    println!("========================");
    println!("BUC60585");
    println!("========================");
    restore(s, "buc60585b.brep", "t");
    checkshape_named(s, "t");
    let geom_pt = plane_axes(
        [2036.25, -97.5, -1460.499755859375],
        [0.0, -1.0, 0.0],
        [1.0, 0.0, 0.0],
    );
    mkface(s, "ft", &geom_pt.surface);
    bsection(s, "result", "t", "ft");
    checkprops(s, "result", Prop::Length, Expected::Mass(22.0446));
    checkshape_named(s, "result");
    checksection(s, "result");
    checkview(s, &CheckView::of("result"));
}

pub fn buc60623_1(s: &mut dyn Session) {
    println!("========================");
    println!("BUC60623");
    println!("========================");
    restore(s, "buc60623a.brep", "a");
    checkshape_named(s, "a");
    restore(s, "buc60623b.brep", "b");
    checkshape_named(s, "b");
    bsection(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(1475.98));
    checkshape_named(s, "result");
    checksection(s, "result");
    checkview(s, &CheckView::of("result"));
}

pub fn buc60623_2(s: &mut dyn Session) {
    println!("========================");
    println!("BUC60623");
    println!("========================");
    restore(s, "buc60623a.brep", "a");
    checkshape_named(s, "a");
    restore(s, "buc60623b.brep", "b");
    checkshape_named(s, "b");
    bsection(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(1475.98));
    checkshape_named(s, "result");
    checksection(s, "result");
    checkview(s, &CheckView::of("result"));
}

pub fn buc60669(s: &mut dyn Session) {
    println!("==================");
    println!("BUC60669");
    println!("==================");
    println!();
    restore(s, "buc60669a.brep", "shell1");
    checkshape_named(s, "shell1");
    restore(s, "buc60669b.brep", "shell2");
    checkshape_named(s, "shell2");
    bsection(s, "result", "shell1", "shell2");
    checkprops(s, "result", Prop::Length, Expected::Mass(104.754));
    checkshape_named(s, "result");
    checksection(s, "result");
    checkview(s, &CheckView::of("result"));
}

pub fn buc60703_2(s: &mut dyn Session) {
    println!("========================");
    println!("BUC60703");
    println!("=================================");
    println!("Action with models with CORRECT tolerance");
    println!("=================================");
    println!();
    restore(s, "buc60703c.brep", "a");
    restore(s, "buc60703e.brep", "b");
    bfuse(s, "result", "a", "b");
    checkshape_named(s, "result");
    checkprops(s, "result", Prop::Area, Expected::Mass(172.662));
    checkprops(s, "result", Prop::Volume, Expected::Mass(129.604));
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Wires(10),
            NbCount::Faces(10),
            NbCount::Shells(1),
            NbCount::Solids(1),
        ],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn buc60703_3(s: &mut dyn Session) {
    println!("========================");
    println!("BUC60703");
    println!("========================");
    restore(s, "buc60703e.brep", "a");
    restore(s, "buc60703d.brep", "b");
    bfuse(s, "result", "a", "b");
    checkshape_named(s, "result");
    checkprops(s, "result", Prop::Area, Expected::Mass(307.2));
    checkprops(s, "result", Prop::Volume, Expected::Mass(252.773));
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Wires(10),
            NbCount::Faces(9),
            NbCount::Shells(1),
            NbCount::Solids(1),
        ],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn buc60703_4(s: &mut dyn Session) {
    println!("========================");
    println!("BUC60703");
    println!("========================");
    println!();
    restore(s, "buc60703d.brep", "a");
    restore(s, "buc60703f.brep", "b");
    bfuse(s, "result", "a", "b");
    checkshape_named(s, "result");
    checkprops(s, "result", Prop::Area, Expected::Mass(322.013));
    checkprops(s, "result", Prop::Volume, Expected::Mass(375.942));
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Wires(6),
            NbCount::Faces(4),
            NbCount::Shells(1),
            NbCount::Solids(1),
        ],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn buc60704(s: &mut dyn Session) {
    println!("========================");
    println!("BUC60704");
    println!("BUC60709");
    println!("========================");
    restore(s, "buc60704a.brep", "p");
    checkshape_named(s, "p");
    restore(s, "buc60704b.brep", "s");
    checkshape_named(s, "s");
    bfuse(s, "result", "p", "s");
    checkprops(s, "result", Prop::Area, Expected::Mass(39.0889));
    checkshape_named(s, "result");
    checkview(s, &CheckView::of("result"));
}

pub fn buc60776_1(s: &mut dyn Session) {
    println!("========================");
    println!("BUC60776");
    println!("========================");
    restore(s, "buc60776a.brep", "a");
    checkshape_named(s, "a");
    restore(s, "buc60776b.brep", "b");
    checkshape_named(s, "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(60771.3));
    checkshape_named(s, "result");
    checkview(s, &CheckView::of("result"));
}

pub fn buc60776_2(s: &mut dyn Session) {
    println!("========================");
    println!("BUC60776");
    println!("========================");
    restore(s, "buc60776c.brep", "a");
    checkshape_named(s, "a");
    restore(s, "buc60776d.brep", "b");
    checkshape_named(s, "b");
    bsection(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(112.906));
    checkshape_named(s, "result");
    checksection(s, "result");
    checkview(s, &CheckView::of("result"));
}

pub fn buc60787_1(s: &mut dyn Session) {
    println!("========");
    println!("BUC60787");
    println!("BUC60783");
    println!("========");
    restore(s, "buc60787a.brep", "S1");
    checkshape_named(s, "S1");
    restore(s, "buc60787b.brep", "S2");
    checkshape_named(s, "S1");
    bcommon(s, "result", "S1", "S2");
    checknbshapes_named(
        s,
        "result",
        Counting::Located,
        &[NbCount::Shells(1), NbCount::Compounds(1)],
        "result should be a shell, but not a compound of faces",
    );
    checkprops(s, "result", Prop::Area, Expected::Mass(45000.));
    checkshape_named(s, "result");
    checkview(s, &CheckView::of("result"));
}

pub fn buc60787_2(s: &mut dyn Session) {
    println!("========");
    println!("BUC60787");
    println!("BUC60783");
    println!("========");
    restore(s, "buc60787c.brep", "S1");
    checkshape_named(s, "S1");
    restore(s, "buc60787d.brep", "S2");
    checkshape_named(s, "S2");
    bcut(s, "result", "S1", "S2");
    checknbshapes_named(
        s,
        "result",
        Counting::Located,
        &[NbCount::Shells(1), NbCount::Compounds(1)],
        "result should be a shell, but not a compound of faces",
    );
    checkprops(s, "result", Prop::Area, Expected::Mass(150200.));
    checkshape_named(s, "result");
    checkview(s, &CheckView::of("result"));
}

pub fn buc60788_2(s: &mut dyn Session) {
    println!("========");
    println!("BUC60788");
    println!("case 3");
    println!("========");
    restore(s, "buc60788e.brep", "S1");
    checkshape_named(s, "S1");
    restore(s, "buc60787d.brep", "S2");
    checkshape_named(s, "S2");
    bcut(s, "result", "S1", "S2");
    checknbshapes_named(
        s,
        "result",
        Counting::Located,
        &[NbCount::Shells(1), NbCount::Compounds(1)],
        "result should be a shell, but not a compound of faces",
    );
    checkprops(s, "result", Prop::Area, Expected::Mass(150200.));
    checkshape_named(s, "result");
    checkview(s, &CheckView::of("result"));
}

pub fn buc60788_3(s: &mut dyn Session) {
    println!("========");
    println!("BUC60788");
    println!("case 2");
    println!("========");
    restore(s, "buc60788c.brep", "S1");
    checkshape_named(s, "S1");
    restore(s, "buc60787b.brep", "S2");
    checkshape_named(s, "S1");
    bcommon(s, "result", "S1", "S2");
    checknbshapes_named(
        s,
        "result",
        Counting::Located,
        &[NbCount::Shells(1), NbCount::Compounds(1)],
        "result should be a shell, but not a compound of faces",
    );
    checkprops(s, "result", Prop::Area, Expected::Mass(45000.));
    checkshape_named(s, "result");
    checkview(s, &CheckView::of("result"));
}

pub fn buc60803(s: &mut dyn Session) {
    println!("========================");
    println!("BUC60803");
    println!("========================");
    restore(s, "buc60803a.brep", "a");
    checkshape_named(s, "a");
    restore(s, "buc60803b.brep", "b");
    checkshape_named(s, "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(1.86064e+07));
    checkshape_named(s, "result");
    checkview(s, &CheckView::of("result"));
}

pub fn buc60865(s: &mut dyn Session) {
    println!("============");
    println!("BUC60865");
    println!("BUC60864");
    println!("============");
    println!();
    restore(s, "BUC60865_sh6.brep", "sh");
    checkshape_named(s, "sh");
    let geom_pl = plane_along([1.1, 0.99999, 1.], [0., 1., 0.]);
    mkface(s, "f", &geom_pl.surface);
    bsection(s, "result", "sh", "f");
    checknbshapes_named(
        s,
        "result",
        Counting::Located,
        &[NbCount::Edges(5)],
        "SECTION operation",
    );
    checkprops(s, "result", Prop::Length, Expected::Mass(4.5));
    checkshape_named(s, "result");
    checksection(s, "result");
    checkview(s, &CheckView::of("result"));
}

pub fn buc60926(s: &mut dyn Session) {
    println!("============");
    println!("BUC60926");
    println!("============");
    println!();
    restore(s, "shading_132.brep", "a");
    checkshape_named(s, "a");
    restore(s, "BUC60926-2.brep", "b");
    checkshape_named(s, "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(41.7175));
    checkshape_named(s, "result");
    checkview(s, &CheckView::of("result"));
}

pub fn bug101(s: &mut dyn Session) {
    println!("================");
    println!("BUC60663");
    println!("BUC60662");
    println!("OCC101");
    println!("================");
    println!();
    restore(s, "buc60663a.brep", "sh1");
    checkshape_named(s, "sh1");
    restore(s, "buc60663b.brep", "sh2");
    checkshape_named(s, "sh2");
    bsection(s, "result", "sh1", "sh2");
    checkprops(s, "result", Prop::Length, Expected::Mass(26.2001));
    checkshape_named(s, "result");
    checksection(s, "result");
    checkview(s, &CheckView::of("result"));
}

pub fn bug102_1(s: &mut dyn Session) {
    println!("================");
    println!("BUC60803");
    println!("OCC102");
    println!("================");
    println!();
    restore(s, "buc60803a.brep", "sh1");
    checkshape_named(s, "sh1");
    restore(s, "buc60803b.brep", "sh2");
    checkshape_named(s, "sh2");
    bfuse(s, "result", "sh1", "sh2");
    checkprops(s, "result", Prop::Area, Expected::Mass(1.86064e+07));
    checkshape_named(s, "result");
    checkview(s, &CheckView::of("result"));
}
