//! The `gdml_public` case bodies, part 5; each statement is one line of its script.
use super::*;

pub fn A9(s: &mut dyn Session) {
    compound(s, &[], "result");
    let geom_pln_sh4795400 = plane_axes(
        [0., 0., 0.],
        [0., 0., 1.],
        [1.1102230246251565e-016, -1., 0.],
    );
    ptorus_on(
        s,
        "sh4795400",
        geom_pln_sh4795400.axes(),
        31.358955689999998,
        22.9486554,
        180.,
    );
    settolerance(s, "sh4795400", 3.135895569e-007);
    let geom_pln_sh4794440 = plane_axes([0., 0., 38.323348539999998], [0., 0., 1.], [1., 0., 0.]);
    pcone_on(
        s,
        "sh4794440",
        geom_pln_sh4794440.axes(),
        [38.323348539999991, 85.45384150999999, 47.130492970000006],
        360.,
    );
    settolerance(s, "sh4794440", 8.5453841510000002e-007);
    copy(s, "sh4794440", "sh4794440_copy");
    trotate(s, "sh4794440_copy", [0., 0., 0.], [1., 0., 0.], 180.);
    ttranslate(s, "sh4794440_copy", [0., 0., 61.888595019999997]);
    ttranslate(s, "sh4794440_copy", [0., 31.358955689999998, 0.]);
    let geom_pln_sh47975B0 = plane_axes(
        [0., 0., 0.],
        [0., 0., 1.],
        [1.1102230246251565e-016, -1., 0.],
    );
    ptorus_on(
        s,
        "sh47975B0",
        geom_pln_sh47975B0.axes(),
        31.358955689999998,
        22.9486554,
        180.,
    );
    settolerance(s, "sh47975B0", 3.135895569e-007);
    let geom_pln_sh47985E0 = plane_axes([0., 0., 38.323348539999998], [0., 0., 1.], [1., 0., 0.]);
    pcone_on(
        s,
        "sh47985E0",
        geom_pln_sh47985E0.axes(),
        [38.323348539999991, 85.45384150999999, 47.130492970000006],
        360.,
    );
    settolerance(s, "sh47985E0", 8.5453841510000002e-007);
    copy(s, "sh47985E0", "sh47985E0_copy");
    ttranslate(s, "sh47985E0_copy", [0., 0., -61.888595019999997]);
    ttranslate(s, "sh47985E0_copy", [0., 31.358955689999998, 0.]);
    copy(s, "sh47975B0", "sh47975B0_copy");
    ttranslate(s, "sh47975B0_copy", [0., 31.358955689999998, 0.]);
    bcut(s, "sh439F990", "sh47975B0_copy", "sh47985E0_copy");
    copy(s, "sh439F990", "sh439F990_copy");
    let geom_pln_sh479BDE0 = plane_axes([0., 0., 0.], [0., 0., 1.], [1., 0., 0.]);
    pcone_on(
        s,
        "sh479BDE0",
        geom_pln_sh479BDE0.axes(),
        [0., 23.879209679999999, 25.222392169999999],
        360.,
    );
    settolerance(s, "sh479BDE0", 2.5222392169999994e-007);
    copy(s, "sh479BDE0", "sh479BDE0_copy");
    trotate(s, "sh479BDE0_copy", [0., 0., 0.], [1., 0., 0.], 180.);
    ttranslate(s, "sh479BDE0_copy", [0., 0., 2.0442502199999999]);
    ttranslate(s, "sh479BDE0_copy", [0., 31.358955689999998, 0.]);
    copy(s, "sh47975B0", "sh47975B0_copy");
    ttranslate(s, "sh47975B0_copy", [0., 31.358955689999998, 0.]);
    bcommon(s, "sh43A5340", "sh47975B0_copy", "sh479BDE0_copy");
    copy(s, "sh43A5340", "sh43A5340_copy");
    let geom_pln_sh479B4B0 = plane_axes([0., 0., 0.], [0., 0., 1.], [1., 0., 0.]);
    pcone_on(
        s,
        "sh479B4B0",
        geom_pln_sh479B4B0.axes(),
        [0., 23.879209679999999, 25.222392169999999],
        360.,
    );
    settolerance(s, "sh479B4B0", 2.5222392169999994e-007);
    copy(s, "sh479B4B0", "sh479B4B0_copy");
    ttranslate(s, "sh479B4B0_copy", [0., 0., -2.0442502199999999]);
    ttranslate(s, "sh479B4B0_copy", [0., 31.358955689999998, 0.]);
    copy(s, "sh47975B0", "sh47975B0_copy");
    ttranslate(s, "sh47975B0_copy", [0., 31.358955689999998, 0.]);
    bcommon(s, "sh44D0FB0", "sh47975B0_copy", "sh479B4B0_copy");
    copy(s, "sh44D0FB0", "sh44D0FB0_copy");
    let geom_pln_sh479B520 = plane_axes(
        [-0., -0., -9.7597608200000003],
        [0., 0., 1.],
        [1.1102230246251565e-016, -1., 0.],
    );
    pcylinder_on(
        s,
        "sh479B520",
        geom_pln_sh479B520.axes(),
        52.1288342,
        19.519521640000001,
        180.,
    );
    settolerance(s, "sh479B520", 5.2128834200000005e-007);
    let geom_pln_sh44D0220 = plane_axes(
        [0., 0., 41.118716509999999],
        [0., 0., 1.],
        [1.1102230246251565e-016, -1., 0.],
    );
    pcone_on(
        s,
        "sh44D0220",
        geom_pln_sh44D0220.axes(),
        [41.118716509999992, 52.128834199999993, 11.010117690000001],
        180.,
    );
    settolerance(s, "sh44D0220", 5.2128834200000005e-007);
    copy(s, "sh44D0220", "sh44D0220_copy");
    ttranslate(s, "sh44D0220_copy", [0., 0., -61.888595019999997]);
    ttranslate(s, "sh44D0220_copy", [0., 31.358955689999998, 0.]);
    let geom_pln_sh44D4E40 = plane_axes(
        [0., 0., 41.118716509999999],
        [0., 0., 1.],
        [1.1102230246251565e-016, -1., 0.],
    );
    pcone_on(
        s,
        "sh44D4E40",
        geom_pln_sh44D4E40.axes(),
        [41.118716509999992, 52.128834199999993, 11.010117690000001],
        180.,
    );
    settolerance(s, "sh44D4E40", 5.2128834200000005e-007);
    copy(s, "sh44D4E40", "sh44D4E40_copy");
    trotate(s, "sh44D4E40_copy", [0., 0., 0.], [1., 0., 0.], 180.);
    ttranslate(s, "sh44D4E40_copy", [0., 0., 61.888595019999997]);
    ttranslate(s, "sh44D4E40_copy", [0., 31.358955689999998, 0.]);
    let geom_pln_sh44D76F0 = plane_axes(
        [0., 0., 0.],
        [0., 0., 1.],
        [1.1102230246251565e-016, -1., 0.],
    );
    ptorus_on(
        s,
        "sh44D76F0",
        geom_pln_sh44D76F0.axes(),
        54.307611090000002,
        10.,
        180.,
    );
    settolerance(s, "sh44D76F0", 5.4307611090000002e-007);
    let geom_pln_sh44D8800 = plane_axes([-0., -0., -10.1], [0., 0., 1.], [1., 0., 0.]);
    pcylinder_on(
        s,
        "sh44D8800",
        geom_pln_sh44D8800.axes(),
        52.1288342,
        20.199999999999999,
        360.,
    );
    settolerance(s, "sh44D8800", 5.2128834200000005e-007);
    copy(s, "sh44D8800", "sh44D8800_copy");
    ttranslate(s, "sh44D8800_copy", [0., 31.358955689999998, 0.]);
    copy(s, "sh44D76F0", "sh44D76F0_copy");
    ttranslate(s, "sh44D76F0_copy", [0., 31.358955689999998, 0.]);
    bcommon(s, "sh43F0950", "sh44D76F0_copy", "sh44D8800_copy");
    copy(s, "sh43F0950", "sh43F0950_copy");
    let geom_pln_sh43A60D0 = plane_axes(
        [0., 0., 0.],
        [0., 0., 1.],
        [1.1102230246251565e-016, -1., 0.],
    );
    ptorus_on(
        s,
        "sh43A60D0",
        geom_pln_sh43A60D0.axes(),
        31.358955689999998,
        10.,
        180.,
    );
    settolerance(s, "sh43A60D0", 3.135895569e-007);
    let geom_pln_sh439F3E0 =
        plane_axes([-0., -0., -6.1893884400000001], [0., 0., 1.], [1., 0., 0.]);
    pcylinder_on(
        s,
        "sh439F3E0",
        geom_pln_sh439F3E0.axes(),
        41.529903679999997,
        12.37877688,
        360.,
    );
    settolerance(s, "sh439F3E0", 4.1529903679999998e-007);
    copy(s, "sh439F3E0", "sh439F3E0_copy");
    ttranslate(s, "sh439F3E0_copy", [0., 0., -14.58049007]);
    ttranslate(s, "sh439F3E0_copy", [0., 31.358955689999998, 0.]);
    copy(s, "sh43A60D0", "sh43A60D0_copy");
    ttranslate(s, "sh43A60D0_copy", [0., 0., -22.9486554]);
    ttranslate(s, "sh43A60D0_copy", [0., 31.358955689999998, 0.]);
    bcommon(s, "sh4454280", "sh43A60D0_copy", "sh439F3E0_copy");
    copy(s, "sh4454280", "sh4454280_copy");
    let geom_pln_sh44547C0 = plane_axes(
        [0., 0., 10.547398743224148],
        [0., 0., 1.],
        [1.1102230246251565e-016, -1., 0.],
    );
    pcone_on(
        s,
        "sh44547C0",
        geom_pln_sh44547C0.axes(),
        [9.9857120800000008, 21.700060650000001, 12.373269366775853],
        180.,
    );
    settolerance(s, "sh44547C0", 2.292066811e-007);
    copy(s, "sh44547C0", "sh44547C0_copy");
    trotate(s, "sh44547C0_copy", [0., 0., 0.], [1., 0., 0.], 180.);
    ttranslate(s, "sh44547C0_copy", [0., 0., 2.0442502199999999]);
    ttranslate(s, "sh44547C0_copy", [0., 31.358955689999998, 0.]);
    let c1_curve = circle_along([0., -5., 0.], [1., 0., 0.], 10.);
    let c2_curve = circle_along([0., 5., 0.], [1., 0., 0.], 10.);
    mkedge(s, "e1", &c1_curve);
    mkedge(s, "e2", &c2_curve);
    wire(s, "w1", &["e1"]);
    wire(s, "w2", &["e2"]);
    mkplane(s, "f1", "w1");
    mkplane(s, "f2", "w2");
    bcut(s, "f12", "f1", "f2");
    revol(s, "sh4452990", "f12", [0., 0., 0.], [0., 0., 1.], 180.);
    copy(s, "sh4452990", "sh4452990_copy");
    let geom_pln_sh4456430 = plane_axes(
        [0., 0., 0.],
        [0., 0., 1.],
        [1.1102230246251565e-016, -1., 0.],
    );
    psphere_turned(
        s,
        "sh4456430",
        geom_pln_sh4456430.axes(),
        8.6602540399999999,
        180.,
    );
    copy(s, "sh4456430", "sh4456430_copy");
    ttranslate(s, "sh4456430_copy", [0., 31.358955689999998, 0.]);
    copy(s, "sh4452990", "sh4452990_copy");
    ttranslate(s, "sh4452990_copy", [0., 31.358955689999998, 0.]);
    bfuse(s, "sh43B0790", "sh4452990_copy", "sh4456430_copy");
    copy(s, "sh43B0790", "sh43B0790_copy");
    let geom_pln_sh439E810 = plane_axes(
        [0., 0., 10.547398743224148],
        [0., 0., 1.],
        [1.1102230246251565e-016, -1., 0.],
    );
    pcone_on(
        s,
        "sh439E810",
        geom_pln_sh439E810.axes(),
        [9.9857120800000008, 21.700060650000001, 12.373269366775853],
        180.,
    );
    settolerance(s, "sh439E810", 2.292066811e-007);
    copy(s, "sh439E810", "sh439E810_copy");
    ttranslate(s, "sh439E810_copy", [0., 0., -2.0442502199999999]);
    ttranslate(s, "sh439E810_copy", [0., 31.358955689999998, 0.]);
    let geom_pln_sh43B03A0 = plane_axes(
        [0., 0., 0.],
        [0., 0., 1.],
        [1.1102230246251565e-016, -1., 0.],
    );
    ptorus_on(
        s,
        "sh43B03A0",
        geom_pln_sh43B03A0.axes(),
        31.358955689999998,
        10.,
        180.,
    );
    settolerance(s, "sh43B03A0", 3.135895569e-007);
    let geom_pln_sh43B6290 =
        plane_axes([-0., -0., -6.1893884400000001], [0., 0., 1.], [1., 0., 0.]);
    pcylinder_on(
        s,
        "sh43B6290",
        geom_pln_sh43B6290.axes(),
        41.529903679999997,
        12.37877688,
        360.,
    );
    settolerance(s, "sh43B6290", 4.1529903679999998e-007);
    copy(s, "sh43B6290", "sh43B6290_copy");
    ttranslate(s, "sh43B6290_copy", [0., 0., 26.95926695]);
    ttranslate(s, "sh43B6290_copy", [0., 31.358955689999998, 0.]);
    copy(s, "sh43B03A0", "sh43B03A0_copy");
    ttranslate(s, "sh43B03A0_copy", [0., 0., 22.9486554]);
    ttranslate(s, "sh43B03A0_copy", [0., 31.358955689999998, 0.]);
    bcut(s, "sh44A1AE0", "sh43B03A0_copy", "sh43B6290_copy");
    copy(s, "sh44A1AE0", "sh44A1AE0_copy");
    copy(s, "sh479B520", "sh479B520_copy");
    ttranslate(s, "sh479B520_copy", [0., 31.358955689999998, 0.]);
    bfuse(s, "sh4331250", "sh479B520_copy", "sh44D0220_copy");
    copy(s, "sh4331250", "sh4331250_copy");
    bfuse(s, "sh4338090", "sh4331250_copy", "sh44D4E40_copy");
    copy(s, "sh4338090", "sh4338090_copy");
    bcut(s, "sh44A60E0", "sh4338090_copy", "sh43F0950_copy");
    copy(s, "sh44A60E0", "sh44A60E0_copy");
    bcut(s, "sh433B900", "sh44A60E0_copy", "sh4454280_copy");
    copy(s, "sh433B900", "sh433B900_copy");
    bcut(s, "sh434BEA0", "sh433B900_copy", "sh44547C0_copy");
    copy(s, "sh434BEA0", "sh434BEA0_copy");
    bcut(s, "sh9647990", "sh434BEA0_copy", "sh43B0790_copy");
    copy(s, "sh9647990", "sh9647990_copy");
    bcut(s, "sh94CED50", "sh9647990_copy", "sh439E810_copy");
    copy(s, "sh94CED50", "sh94CED50_copy");
    bcut(s, "sh4336A40", "sh94CED50_copy", "sh44A1AE0_copy");
    copy(s, "sh4336A40", "sh4336A40_copy");
    let geom_pln_sh96196F0 = plane_axes(
        [0., 0., 0.],
        [0., 0., 1.],
        [1.1102230246251565e-016, -1., 0.],
    );
    ptorus_on(
        s,
        "sh96196F0",
        geom_pln_sh96196F0.axes(),
        31.358955689999998,
        10.,
        180.,
    );
    settolerance(s, "sh96196F0", 3.135895569e-007);
    let geom_pln_sh961A1E0 =
        plane_axes([-0., -0., -5.8499999999999996], [0., 0., 1.], [1., 0., 0.]);
    pcylinder_on(
        s,
        "sh961A1E0",
        geom_pln_sh961A1E0.axes(),
        41.658273819999998,
        11.699999999999999,
        360.,
    );
    settolerance(s, "sh961A1E0", 4.1658273819999996e-007);
    copy(s, "sh961A1E0", "sh961A1E0_copy");
    ttranslate(s, "sh961A1E0_copy", [0., 0., 4.3499999999999996]);
    ttranslate(s, "sh961A1E0_copy", [0., 31.358955689999998, 0.]);
    let geom_pln_sh960FDD0 =
        plane_axes([-0., -0., -5.8499999999999996], [0., 0., 1.], [1., 0., 0.]);
    pcylinder_on(
        s,
        "sh960FDD0",
        geom_pln_sh960FDD0.axes(),
        41.658273819999998,
        11.699999999999999,
        360.,
    );
    settolerance(s, "sh960FDD0", 4.1658273819999996e-007);
    copy(s, "sh960FDD0", "sh960FDD0_copy");
    ttranslate(s, "sh960FDD0_copy", [0., 0., 7.3499999999999996]);
    ttranslate(s, "sh960FDD0_copy", [0., 31.358955689999998, 0.]);
    copy(s, "sh96196F0", "sh96196F0_copy");
    ttranslate(s, "sh96196F0_copy", [0., 31.358955689999998, 0.]);
    bcommon(s, "sh96235C0", "sh96196F0_copy", "sh960FDD0_copy");
    copy(s, "sh96235C0", "sh96235C0_copy");
    let geom_pln_sh9616CF0 = plane_axes(
        [-0., -0., -1.5],
        [0., 0., 1.],
        [1.1102230246251565e-016, -1., 0.],
    );
    pcylinder_on(
        s,
        "sh9616CF0",
        geom_pln_sh9616CF0.axes(),
        38.858955690000002,
        3.,
        180.,
    );
    settolerance(s, "sh9616CF0", 3.8858955689999997e-007);
    let geom_pln_sh96181F0 = plane_axes(
        [-0., -0., -1.5],
        [0., 0., 1.],
        [1.1102230246251565e-016, -1., 0.],
    );
    pcylinder_on(
        s,
        "sh96181F0",
        geom_pln_sh96181F0.axes(),
        23.858955689999998,
        3.,
        180.,
    );
    settolerance(s, "sh96181F0", 2.3858955689999998e-007);
    copy(s, "sh96181F0", "sh96181F0_copy");
    ttranslate(s, "sh96181F0_copy", [0., 31.358955689999998, 0.]);
    copy(s, "sh9616CF0", "sh9616CF0_copy");
    ttranslate(s, "sh9616CF0_copy", [0., 31.358955689999998, 0.]);
    bcut(s, "sh4335AF0", "sh9616CF0_copy", "sh96181F0_copy");
    copy(s, "sh4335AF0", "sh4335AF0_copy");
    copy(s, "sh96196F0", "sh96196F0_copy");
    ttranslate(s, "sh96196F0_copy", [0., 31.358955689999998, 0.]);
    bcut(s, "sh96283A0", "sh96196F0_copy", "sh961A1E0_copy");
    copy(s, "sh96283A0", "sh96283A0_copy");
    bfuse(s, "sh962A780", "sh96283A0_copy", "sh96235C0_copy");
    copy(s, "sh962A780", "sh962A780_copy");
    bfuse(s, "sh4348470", "sh962A780_copy", "sh4335AF0_copy");
    copy(s, "sh4348470", "sh4348470_copy");
    copy(s, "sh4795400", "sh4795400_copy");
    ttranslate(s, "sh4795400_copy", [0., 31.358955689999998, 0.]);
    bcut(s, "sh434A850", "sh4795400_copy", "sh4794440_copy");
    copy(s, "sh434A850", "sh434A850_copy");
    bfuse(s, "sh434B5E0", "sh434A850_copy", "sh439F990_copy");
    copy(s, "sh434B5E0", "sh434B5E0_copy");
    bfuse(s, "sh434E600", "sh434B5E0_copy", "sh43A5340_copy");
    copy(s, "sh434E600", "sh434E600_copy");
    bfuse(s, "sh4335E70", "sh434E600_copy", "sh44D0FB0_copy");
    copy(s, "sh4335E70", "sh4335E70_copy");
    bfuse(s, "sh1129ED20", "sh4335E70_copy", "sh4336A40_copy");
    copy(s, "sh1129ED20", "sh1129ED20_copy");
    bcut(s, "sh43F4000", "sh1129ED20_copy", "sh4348470_copy");
    copy(s, "sh43F4000", "sh43F4000_copy");
    add(s, "sh43F4000_copy", "result");
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "sh47975B0_copy", Prop::Area, "17514.2");
    kind_is(s, "sh47975B0_copy", ShapeEnum::Solid);
}
