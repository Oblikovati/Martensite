//! The `gdml_public` case bodies, part 3; each statement is one line of its script.
use super::*;

pub fn A3(s: &mut dyn Session) {
    compound(s, &[], "result");
    make_box(
        s,
        "sh4E8BEE0",
        [-317.5, -304.80000000000001, -381.],
        [635., 609.60000000000002, 762.],
    );
    make_box(
        s,
        "sh4E8E870",
        [-571.5, -457.19999999999999, -635.],
        [1143., 914.39999999999998, 1270.],
    );
    let geom_pln_sh4E921C0 = plane_axes([-0., -0., -635.], [0., 0., 1.], [1., 0., 0.]);
    pcylinder_on(
        s,
        "sh4E921C0",
        geom_pln_sh4E921C0.axes(),
        419.10000000000002,
        1270.,
        360.,
    );
    settolerance(s, "sh4E921C0", 1.27e-005);
    copy(s, "sh4E921C0", "sh4E921C0_copy");
    trotate(
        s,
        "sh4E921C0_copy",
        [0., 0., 0.],
        [
            0.70710678118654757,
            -4.329780281177467e-017,
            0.70710678118654757,
        ],
        180.,
    );
    ttranslate(s, "sh4E921C0_copy", [-12.699999999999974, 0., 0.]);
    trotate(s, "sh4E921C0_copy", [0., 0., 0.], [0., 1., 0.], 90.);
    ttranslate(s, "sh4E921C0_copy", [635., 127., -139.69999999999999]);
    copy(s, "sh4E8E870", "sh4E8E870_copy");
    trotate(s, "sh4E8E870_copy", [0., 0., 0.], [0., -1., 0.], 90.);
    ttranslate(s, "sh4E8E870_copy", [-12.699999999999974, 0., 0.]);
    trotate(s, "sh4E8E870_copy", [0., 0., 0.], [0., 1., 0.], 90.);
    ttranslate(s, "sh4E8E870_copy", [635., 127., -139.69999999999999]);
    bcut(s, "sh4E85040", "sh4E8E870_copy", "sh4E921C0_copy");
    copy(s, "sh4E85040", "sh4E85040_copy");
    copy(s, "sh4E8BEE0", "sh4E8BEE0_copy");
    trotate(s, "sh4E8BEE0_copy", [0., 0., 0.], [0., 1., 0.], 90.);
    ttranslate(s, "sh4E8BEE0_copy", [635., 127., -139.69999999999999]);
    bcut(s, "sh4A708F0", "sh4E8BEE0_copy", "sh4E85040_copy");
    copy(s, "sh4A708F0", "sh4A708F0_copy");
    add(s, "sh4A708F0_copy", "result");
}

pub fn A4(s: &mut dyn Session) {
    compound(s, &[], "result");
    make_box(
        s,
        "sh49D5D00",
        [-317.5, -304.80000000000001, -381.],
        [635., 609.60000000000002, 762.],
    );
    make_box(
        s,
        "sh49D8690",
        [-571.5, -457.19999999999999, -381.],
        [1143., 914.39999999999998, 762.],
    );
    let geom_pln_sh49DBFE0 = plane_axes([-0., -0., -381.], [0., 0., 1.], [1., 0., 0.]);
    pcylinder_on(
        s,
        "sh49DBFE0",
        geom_pln_sh49DBFE0.axes(),
        419.10000000000002,
        762.,
        360.,
    );
    settolerance(s, "sh49DBFE0", 7.6199999999999999e-006);
    copy(s, "sh49DBFE0", "sh49DBFE0_copy");
    trotate(
        s,
        "sh49DBFE0_copy",
        [0., 0., 0.],
        [
            0.70710678118654757,
            -4.329780281177467e-017,
            0.70710678118654757,
        ],
        180.,
    );
    ttranslate(s, "sh49DBFE0_copy", [-266.69999999999999, 0., 0.]);
    trotate(s, "sh49DBFE0_copy", [0., 0., 0.], [0., 1., 0.], 90.);
    ttranslate(s, "sh49DBFE0_copy", [635., 127., -139.69999999999999]);
    copy(s, "sh49D8690", "sh49D8690_copy");
    trotate(s, "sh49D8690_copy", [0., 0., 0.], [0., -1., 0.], 90.);
    ttranslate(s, "sh49D8690_copy", [-266.69999999999999, 0., 0.]);
    trotate(s, "sh49D8690_copy", [0., 0., 0.], [0., 1., 0.], 90.);
    ttranslate(s, "sh49D8690_copy", [635., 127., -139.69999999999999]);
    bcut(s, "sh49A5250", "sh49D8690_copy", "sh49DBFE0_copy");
    copy(s, "sh49A5250", "sh49A5250_copy");
    copy(s, "sh49D5D00", "sh49D5D00_copy");
    trotate(s, "sh49D5D00_copy", [0., 0., 0.], [0., 1., 0.], 90.);
    ttranslate(s, "sh49D5D00_copy", [635., 127., -139.69999999999999]);
    bcut(s, "sh464ACD0", "sh49D5D00_copy", "sh49A5250_copy");
    copy(s, "sh464ACD0", "sh464ACD0_copy");
    add(s, "sh464ACD0_copy", "result");
}

pub fn A5(s: &mut dyn Session) {
    compound(s, &[], "result");
    let geom_pln_sh2860860 = plane_axes([0., 0., 323.93388673042631], [0., 0., 1.], [1., 0., 0.]);
    pcone_on(
        s,
        "sh2860860",
        geom_pln_sh2860860.axes(),
        [148.69096439999998, 515.32406046000006, 798.7363878495737],
        360.,
    );
    settolerance(s, "sh2860860", 1.12267027458e-005);
    let geom_pln_sh285FDE0 =
        plane_axes([0., 0., 0.00021224461522251307], [0., 0., 1.], [1., 0., 0.]);
    pcone_on(
        s,
        "sh285FDE0",
        geom_pln_sh285FDE0.axes(),
        [2.9651215899955146, 148.69096439977511, 0.010431115384777485],
        360.,
    );
    settolerance(s, "sh285FDE0", 1.4869096439977511e-006);
    copy(s, "sh285FDE0", "sh285FDE0_copy");
    trotate(
        s,
        "sh285FDE0_copy",
        [0., 0., 0.],
        [
            -0.37211395835926747,
            0.76355699062743898,
            0.52774228943506762,
        ],
        6.0267193763611507e-015,
    );
    ttranslate(
        s,
        "sh285FDE0_copy",
        [
            2.2054877035770915e-008,
            1.0123585525434464e-008,
            323.92324335930391,
        ],
    );
    trotate(
        s,
        "sh285FDE0_copy",
        [0., 0., 0.],
        [
            0.90882954575006403,
            0.41716766026590796,
            2.7755575615628907e-017,
        ],
        90.000000000000014,
    );
    ttranslate(
        s,
        "sh285FDE0_copy",
        [-194.94499999999999, 678.75574884000002, 127.],
    );
    let geom_pln_sh2862A10 =
        plane_axes([0., 0., 0.00020607070595257852], [0., 0., 1.], [1., 0., 0.]);
    pcone_on(
        s,
        "sh2862A10",
        geom_pln_sh2862A10.axes(),
        [2.8788700799980997, 515.35313995965976, 0.036683119294047425],
        360.,
    );
    settolerance(s, "sh2862A10", 5.1535313995965974e-006);
    copy(s, "sh2862A10", "sh2862A10_copy");
    trotate(
        s,
        "sh2862A10_copy",
        [0., 0., 0.],
        [
            -0.37211395835926747,
            0.76355699062743898,
            0.52774228943506762,
        ],
        6.0267193763611507e-015,
    );
    ttranslate(
        s,
        "sh2862A10_copy",
        [
            7.4577883424353786e-008,
            3.4232556345159537e-008,
            1122.6333874656173,
        ],
    );
    trotate(
        s,
        "sh2862A10_copy",
        [0., 0., 0.],
        [
            0.90882954575006403,
            0.41716766026590796,
            2.7755575615628907e-017,
        ],
        90.000000000000014,
    );
    ttranslate(
        s,
        "sh2862A10_copy",
        [-194.94499999999999, 678.75574884000002, 127.],
    );
    let geom_pln_sh2863C70 =
        plane_axes([-0., -0., -399.35507001000002], [0., 0., 1.], [1., 0., 0.]);
    pcylinder_on(
        s,
        "sh2863C70",
        geom_pln_sh2863C70.axes(),
        2.9079495799999999,
        798.71014002000004,
        360.,
    );
    settolerance(s, "sh2863C70", 7.987101400199999e-006);
    copy(s, "sh2863C70", "sh2863C70_copy");
    trotate(
        s,
        "sh2863C70_copy",
        [0., 0., 0.],
        [
            0.90882954575006414,
            0.41716766026590807,
            -4.4660810088114466e-017,
        ],
        180.,
    );
    ttranslate(
        s,
        "sh2863C70_copy",
        [
            4.9805848334472103e-008,
            2.2861769366500084e-008,
            723.27852561214502,
        ],
    );
    trotate(
        s,
        "sh2863C70_copy",
        [0., 0., 0.],
        [
            0.90882954575006403,
            0.41716766026590796,
            2.7755575615628907e-017,
        ],
        90.000000000000014,
    );
    ttranslate(
        s,
        "sh2863C70_copy",
        [-194.94499999999999, 678.75574884000002, 127.],
    );
    copy(s, "sh2860860", "sh2860860_copy");
    trotate(
        s,
        "sh2860860_copy",
        [0., 0., 0.],
        [
            0.90882954575006403,
            0.41716766026590796,
            2.7755575615628907e-017,
        ],
        90.000000000000014,
    );
    ttranslate(
        s,
        "sh2860860_copy",
        [-194.94499999999999, 678.75574884000002, 127.],
    );
    bfuse(s, "sh4B4C5E0", "sh2860860_copy", "sh285FDE0_copy");
    copy(s, "sh4B4C5E0", "sh4B4C5E0_copy");
    bcut(s, "sh4B4CAB0", "sh4B4C5E0_copy", "sh2862A10_copy");
    copy(s, "sh4B4CAB0", "sh4B4CAB0_copy");
    bcut(s, "sh2866C90", "sh4B4CAB0_copy", "sh2863C70_copy");
    copy(s, "sh2866C90", "sh2866C90_copy");
    add(s, "sh2866C90_copy", "result");
}

pub fn A6(s: &mut dyn Session) {
    compound(s, &[], "result");
    let geom_pln_sh4A1ADB0 = plane_axes([-0., -0., -98.15938036], [0., 0., 1.], [1., 0., 0.]);
    pcylinder_on(
        s,
        "sh4A1ADB0",
        geom_pln_sh4A1ADB0.axes(),
        555.80733567000004,
        196.31876072,
        360.,
    );
    settolerance(s, "sh4A1ADB0", 5.5580733567000004e-006);
    let geom_pln_sh4A1A330 = plane_axes([0., 0., 0.], [0., 0., 1.], [1., 0., 0.]);
    pcone_on(
        s,
        "sh4A1A330",
        geom_pln_sh4A1A330.axes(),
        [0., 555.80733567000016, 386.81270655999998],
        360.,
    );
    settolerance(s, "sh4A1A330", 5.5580733567000013e-006);
    copy(s, "sh4A1A330", "sh4A1A330_copy");
    trotate(s, "sh4A1A330_copy", [0., 0., 0.], [1., 0., 0.], 180.);
    ttranslate(
        s,
        "sh4A1A330_copy",
        [0., 3.4891115774371428e-008, 484.97208690922002],
    );
    trotate(
        s,
        "sh4A1A330_copy",
        [0., 0., 0.],
        [1., 0., 0.],
        130.88181030000001,
    );
    ttranslate(
        s,
        "sh4A1A330_copy",
        [254., -34.795627709999998, 204.22696378000001],
    );
    copy(s, "sh4A1ADB0", "sh4A1ADB0_copy");
    trotate(
        s,
        "sh4A1ADB0_copy",
        [0., 0., 0.],
        [1., 0., 0.],
        130.88181030000001,
    );
    ttranslate(
        s,
        "sh4A1ADB0_copy",
        [254., -34.795627709999998, 204.22696378000001],
    );
    bfuse(s, "sh4C4C8D0", "sh4A1ADB0_copy", "sh4A1A330_copy");
    copy(s, "sh4C4C8D0", "sh4C4C8D0_copy");
    add(s, "sh4C4C8D0_copy", "result");
}

pub fn A7(s: &mut dyn Session) {
    compound(s, &[], "result");
    make_box(s, "sh46B0230", [-254., -381., -381.], [508., 762., 762.]);
    make_box(s, "sh4912900", [-254., -190.5, -381.], [508., 381., 762.]);
    copy(s, "sh4912900", "sh4912900_copy");
    trotate(
        s,
        "sh4912900_copy",
        [0., 0., 0.],
        [
            -0.63331020410333672,
            0.75474971096672405,
            -0.17108786974603546,
        ],
        91.727941072350504,
    );
    ttranslate(
        s,
        "sh4912900_copy",
        [12.702649262360318, -217.06072879313561, 182.19951927],
    );
    trotate(
        s,
        "sh4912900_copy",
        [0., 0., 0.],
        [0., 0., 0.99999999999999989],
        150.00000000000003,
    );
    ttranslate(
        s,
        "sh4912900_copy",
        [-97.529547440000002, -202.95567883999999, 508.],
    );
    copy(s, "sh46B0230", "sh46B0230_copy");
    trotate(
        s,
        "sh46B0230_copy",
        [0., 0., 0.],
        [0., 0., 0.99999999999999989],
        150.00000000000003,
    );
    ttranslate(
        s,
        "sh46B0230_copy",
        [-97.529547440000002, -202.95567883999999, 508.],
    );
    bcut(s, "sh498BA20", "sh46B0230_copy", "sh4912900_copy");
    copy(s, "sh498BA20", "sh498BA20_copy");
    add(s, "sh498BA20_copy", "result");
}
