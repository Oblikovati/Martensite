//! The `gdml_public` case bodies, part 10; each statement is one line of its script.
use super::*;

pub fn C1(s: &mut dyn Session) {
    compound(s, &[], "result");
    let geom_pln_sh2770860 = plane_axes([0., 0., 1752.6372651538566], [0., 0., 1.], [1., 0., 0.]);
    pcone_on(
        s,
        "sh2770860",
        geom_pln_sh2770860.axes(),
        [469.61773982, 666.83785218000003, 736.03547960614355],
        360.,
    );
    settolerance(s, "sh2770860", 2.4886727447599999e-005);
    let geom_pln_sh276FDE0 = plane_axes([0., 0., 80.068607794346875], [0., 0., 1.], [1., 0., 0.]);
    pcone_on(
        s,
        "sh276FDE0",
        geom_pln_sh276FDE0.axes(),
        [298.82011236999989, 666.8378521799998, 98.610056185653136],
        360.,
    );
    settolerance(s, "sh276FDE0", 6.6683785217999979e-006);
    copy(s, "sh276FDE0", "sh276FDE0_copy");
    trotate(s, "sh276FDE0_copy", [0., 0., 0.], [1., 0., 0.], -180.);
    ttranslate(
        s,
        "sh276FDE0_copy",
        [0., -2.7224587029195391e-009, 2667.3514087387584],
    );
    trotate(
        s,
        "sh276FDE0_copy",
        [0., 0., 0.],
        [1., 0., 0.],
        164.99999999999997,
    );
    ttranslate(s, "sh276FDE0_copy", [254., 381., 2195.4636134900002]);
    let geom_pln_sh2772A10 = plane_axes([0., 0., 26.951401570293473], [0., 0., 1.], [1., 0., 0.]);
    pcone_on(
        s,
        "sh2772A10",
        geom_pln_sh2772A10.axes(),
        [100.58400000000005, 470.63373982000024, 99.154528939706523],
        360.,
    );
    settolerance(s, "sh2772A10", 4.7063373982000028e-006);
    copy(s, "sh2772A10", "sh2772A10_copy");
    trotate(s, "sh2772A10_copy", [0., 0., 0.], [1., 0., 0.], -180.);
    ttranslate(
        s,
        "sh2772A10_copy",
        [0., -2.9245939003885724e-010, 1878.4709592669262],
    );
    trotate(
        s,
        "sh2772A10_copy",
        [0., 0., 0.],
        [1., 0., 0.],
        164.99999999999997,
    );
    ttranslate(s, "sh2772A10_copy", [254., 381., 2195.4636134900002]);
    let geom_pln_sh2773C70 = plane_axes([0., 0., 375.38459842556551], [0., 0., 1.], [1., 0., 0.]);
    pcone_on(
        s,
        "sh2773C70",
        geom_pln_sh2773C70.axes(),
        [100.584, 299.83611237000002, 743.61900687443438],
        360.,
    );
    settolerance(s, "sh2773C70", 1.1190036052999998e-005);
    copy(s, "sh2773C70", "sh2773C70_copy");
    ttranslate(
        s,
        "sh2773C70_copy",
        [0., -1.6200374375330284e-009, 1472.0709592705846],
    );
    trotate(
        s,
        "sh2773C70_copy",
        [0., 0., 0.],
        [1., 0., 0.],
        164.99999999999997,
    );
    ttranslate(s, "sh2773C70_copy", [254., 381., 2195.4636134900002]);
    copy(s, "sh2770860", "sh2770860_copy");
    trotate(
        s,
        "sh2770860_copy",
        [0., 0., 0.],
        [1., 0., 0.],
        164.99999999999997,
    );
    ttranslate(s, "sh2770860_copy", [254., 381., 2195.4636134900002]);
    bfuse(s, "sh4CDA9F0", "sh2770860_copy", "sh276FDE0_copy");
    copy(s, "sh4CDA9F0", "sh4CDA9F0_copy");
    bcut(s, "sh4CD6540", "sh4CDA9F0_copy", "sh2772A10_copy");
    copy(s, "sh4CD6540", "sh4CD6540_copy");
    bcut(s, "sh4977860", "sh4CD6540_copy", "sh2773C70_copy");
    copy(s, "sh4977860", "sh4977860_copy");
    add(s, "sh4977860_copy", "result");
}

pub fn C2(s: &mut dyn Session) {
    compound(s, &[], "result");
    let geom_pln_sh4882DC0 = plane_axes([-0., -0., -381.], [0., 0., 1.], [1., 0., 0.]);
    pcylinder_on(s, "sh4882DC0", geom_pln_sh4882DC0.axes(), 635., 762., 360.);
    settolerance(s, "sh4882DC0", 7.6199999999999999e-006);
    let geom_pln_sh4882340 = plane_axes([-0., -0., -381.], [0., 0., 1.], [1., 0., 0.]);
    pcylinder_on(s, "sh4882340", geom_pln_sh4882340.axes(), 127., 762., 360.);
    settolerance(s, "sh4882340", 7.6199999999999999e-006);
    copy(s, "sh4882340", "sh4882340_copy");
    trotate(s, "sh4882340_copy", [0., 0., 0.], [-1., 0., 0.], 90.);
    ttranslate(s, "sh4882340_copy", [63.5, 0., 254.]);
    copy(s, "sh4882DC0", "sh4882DC0_copy");
    trotate(s, "sh4882DC0_copy", [0., 0., 0.], [-1., 0., 0.], 90.);
    ttranslate(s, "sh4882DC0_copy", [63.5, 0., 254.]);
    bcut(s, "sh4AE56C0", "sh4882DC0_copy", "sh4882340_copy");
    copy(s, "sh4AE56C0", "sh4AE56C0_copy");
    add(s, "sh4AE56C0_copy", "result");
}

pub fn C3(s: &mut dyn Session) {
    compound(s, &[], "result");
    let geom_pln_sh482C8E0 = plane_axes(
        [-0., -0., -381.],
        [0., 0., 1.],
        [-1., 1.2246467991473532e-016, 0.],
    );
    pcylinder_on(s, "sh482C8E0", geom_pln_sh482C8E0.axes(), 635., 762., 270.);
    settolerance(s, "sh482C8E0", 7.6199999999999999e-006);
    let geom_pln_sh4AA7EC0 = plane_axes(
        [-0., -0., -381.],
        [0., 0., 1.],
        [-1., 1.2246467991473532e-016, 0.],
    );
    pcylinder_on(s, "sh4AA7EC0", geom_pln_sh4AA7EC0.axes(), 127., 762., 270.);
    settolerance(s, "sh4AA7EC0", 7.6199999999999999e-006);
    copy(s, "sh4AA7EC0", "sh4AA7EC0_copy");
    trotate(s, "sh4AA7EC0_copy", [0., 0., 0.], [-1., 0., 0.], 90.);
    ttranslate(s, "sh4AA7EC0_copy", [63.5, 0., 254.]);
    copy(s, "sh482C8E0", "sh482C8E0_copy");
    trotate(s, "sh482C8E0_copy", [0., 0., 0.], [-1., 0., 0.], 90.);
    ttranslate(s, "sh482C8E0_copy", [63.5, 0., 254.]);
    bcut(s, "sh470F120", "sh482C8E0_copy", "sh4AA7EC0_copy");
    copy(s, "sh470F120", "sh470F120_copy");
    add(s, "sh470F120_copy", "result");
}

pub fn C4(s: &mut dyn Session) {
    compound(s, &[], "result");
    make_box(s, "sh4B6F0F0", [-20., -20., -2.5], [40., 40., 5.]);
    make_box(
        s,
        "sh4DD0CC0",
        [-5., -5., -7.0984999999999996],
        [10., 10., 14.196999999999999],
    );
    copy(s, "sh4DD0CC0", "sh4DD0CC0_copy");
    trotate(s, "sh4DD0CC0_copy", [0., 0., 0.], [0., 0., 1.], 90.);
    ttranslate(s, "sh4DD0CC0_copy", [0., 0., 9.5984999999999996]);
    ttranslate(s, "sh4DD0CC0_copy", [20., 20., 2.5]);
    copy(s, "sh4B6F0F0", "sh4B6F0F0_copy");
    ttranslate(s, "sh4B6F0F0_copy", [20., 20., 2.5]);
    bfuse(s, "sh4DC8740", "sh4B6F0F0_copy", "sh4DD0CC0_copy");
    copy(s, "sh4DC8740", "sh4DC8740_copy");
    add(s, "sh4DC8740_copy", "result");
}

pub fn C5(s: &mut dyn Session) {
    compound(s, &[], "result");
    let geom_pln_sh4F112F0 = plane_axes(
        [-0., -0., -12.5],
        [0., 0., 1.],
        [-1., 1.2246467991473532e-016, 0.],
    );
    pcylinder_on(s, "sh4F112F0", geom_pln_sh4F112F0.axes(), 45., 25., 180.);
    settolerance(s, "sh4F112F0", 4.5000000000000003e-007);
    let geom_pln_sh4EFB8A0 = plane_axes(
        [-0., -0., -12.5],
        [0., 0., 1.],
        [-1., 1.2246467991473532e-016, 0.],
    );
    pcylinder_on(s, "sh4EFB8A0", geom_pln_sh4EFB8A0.axes(), 5., 25., 180.);
    settolerance(s, "sh4EFB8A0", 2.4999999999999999e-007);
    copy(s, "sh4EFB8A0", "sh4EFB8A0_copy");
    trotate(s, "sh4EFB8A0_copy", [0., 0., 0.], [-1., 0., 0.], 90.);
    ttranslate(s, "sh4EFB8A0_copy", [25., 0., 0.]);
    copy(s, "sh4F112F0", "sh4F112F0_copy");
    trotate(s, "sh4F112F0_copy", [0., 0., 0.], [-1., 0., 0.], 90.);
    ttranslate(s, "sh4F112F0_copy", [25., 0., 0.]);
    bcut(s, "sh4D8B540", "sh4F112F0_copy", "sh4EFB8A0_copy");
    copy(s, "sh4D8B540", "sh4D8B540_copy");
    add(s, "sh4D8B540_copy", "result");
}

pub fn C6(s: &mut dyn Session) {
    compound(s, &[], "result");
    let geom_pln_sh47F18A0 = plane_axes([0., 0., 228.60000000000002], [0., 0., 1.], [1., 0., 0.]);
    pcone_on(
        s,
        "sh47F18A0",
        geom_pln_sh47F18A0.axes(),
        [76.200000000000003, 101.59999999999999, 76.199999999999989],
        360.,
    );
    settolerance(s, "sh47F18A0", 3.0479999999999999e-006);
    let geom_pln_sh47F0E20 =
        plane_axes([-0., -0., -76.200000000000003], [0., 0., 1.], [1., 0., 0.]);
    pcylinder_on(
        s,
        "sh47F0E20",
        geom_pln_sh47F0E20.axes(),
        38.100000000000001,
        152.40000000000001,
        360.,
    );
    settolerance(s, "sh47F0E20", 1.5239999999999999e-006);
    let geom_pln_sh47F3200 = plane_axes([-0., -0., -31.75], [0., 0., 1.], [1., 0., 0.]);
    pcylinder_on(s, "sh47F3200", geom_pln_sh47F3200.axes(), 63.5, 63.5, 360.);
    settolerance(s, "sh47F3200", 6.3499999999999996e-007);
    let geom_pln_sh47F41C0 = plane_axes([-0., -0., -31.75], [0., 0., 1.], [1., 0., 0.]);
    pcylinder_on(
        s,
        "sh47F41C0",
        geom_pln_sh47F41C0.axes(),
        38.100000000000001,
        63.5,
        360.,
    );
    settolerance(s, "sh47F41C0", 6.3499999999999996e-007);
    copy(s, "sh47F41C0", "sh47F41C0_copy");
    ttranslate(
        s,
        "sh47F41C0_copy",
        [0., -2.7494673204841999e-014, 247.64999999999998],
    );
    trotate(s, "sh47F41C0_copy", [0., 0., 0.], [1., 0., 0.], 90.);
    ttranslate(s, "sh47F41C0_copy", [0., 342.89999999999998, 0.]);
    copy(s, "sh47F3200", "sh47F3200_copy");
    ttranslate(
        s,
        "sh47F3200_copy",
        [0., -2.7494673204841999e-014, 247.64999999999998],
    );
    trotate(s, "sh47F3200_copy", [0., 0., 0.], [1., 0., 0.], 90.);
    ttranslate(s, "sh47F3200_copy", [0., 342.89999999999998, 0.]);
    bcut(s, "sh4AFD1C0", "sh47F3200_copy", "sh47F41C0_copy");
    copy(s, "sh4AFD1C0", "sh4AFD1C0_copy");
    copy(s, "sh47F0E20", "sh47F0E20_copy");
    ttranslate(
        s,
        "sh47F0E20_copy",
        [0., -3.2429614549300819e-014, 292.09999999999997],
    );
    trotate(s, "sh47F0E20_copy", [0., 0., 0.], [1., 0., 0.], 90.);
    ttranslate(s, "sh47F0E20_copy", [0., 342.89999999999998, 0.]);
    bfuse(s, "sh4AFE030", "sh47F0E20_copy", "sh4AFD1C0_copy");
    copy(s, "sh4AFE030", "sh4AFE030_copy");
    copy(s, "sh47F18A0", "sh47F18A0_copy");
    trotate(s, "sh47F18A0_copy", [0., 0., 0.], [1., 0., 0.], 90.);
    ttranslate(s, "sh47F18A0_copy", [0., 342.89999999999998, 0.]);
    bcut(s, "sh47E7D40", "sh47F18A0_copy", "sh4AFE030_copy");
    copy(s, "sh47E7D40", "sh47E7D40_copy");
    add(s, "sh47E7D40_copy", "result");
}
