//! The `gdml_public` case bodies, part 6; each statement is one line of its script.
use super::*;

pub fn B1(s: &mut dyn Session) {
    compound(s, &[], "result");
    make_box(s, "sh46487D0", [-190.5, -254., -381.], [381., 508., 762.]);
    make_box(
        s,
        "sh489CBE0",
        [-127., -152.40000000000001, -381.],
        [254., 304.80000000000001, 762.],
    );
    copy(s, "sh489CBE0", "sh489CBE0_copy");
    trotate(
        s,
        "sh489CBE0_copy",
        [0., 0., 0.],
        [
            -0.57735026918962584,
            -0.57735026918962584,
            -0.57735026918962584,
        ],
        119.99999999999999,
    );
    ttranslate(s, "sh489CBE0_copy", [0., 0., 127.00000000000006]);
    trotate(s, "sh489CBE0_copy", [0., 0., 0.], [0., 1., 0.], 90.);
    ttranslate(s, "sh489CBE0_copy", [508., 508., -317.5]);
    copy(s, "sh46487D0", "sh46487D0_copy");
    trotate(s, "sh46487D0_copy", [0., 0., 0.], [0., 1., 0.], 90.);
    ttranslate(s, "sh46487D0_copy", [508., 508., -317.5]);
    bfuse(s, "sh45011B0", "sh46487D0_copy", "sh489CBE0_copy");
    copy(s, "sh45011B0", "sh45011B0_copy");
    add(s, "sh45011B0_copy", "result");
}

pub fn B2(s: &mut dyn Session) {
    compound(s, &[], "result");
    make_box(
        s,
        "sh467E330",
        [-321.87638082500001, -263.17581868000002, -254.],
        [643.75276165000002, 526.35163736000004, 508.],
    );
    let geom_pln_sh48D1E70 = plane_axes([-0., -0., -508.], [0., 0., 1.], [1., 0., 0.]);
    pcylinder_on(
        s,
        "sh48D1E70",
        geom_pln_sh48D1E70.axes(),
        76.200000000000003,
        1016.,
        360.,
    );
    settolerance(s, "sh48D1E70", 1.0159999999999999e-005);
    copy(s, "sh48D1E70", "sh48D1E70_copy");
    trotate(
        s,
        "sh48D1E70_copy",
        [0., 0., 0.],
        [
            -0.70710677745571249,
            3.6866829955513545e-009,
            0.70710678491738266,
        ],
        -179.99999957753724,
    );
    ttranslate(
        s,
        "sh48D1E70_copy",
        [186.12362050792001, -136.17581735759262, 126.99999945592533],
    );
    trotate(
        s,
        "sh48D1E70_copy",
        [0., 0., 0.],
        [
            -0.679047659633503,
            -0.27890596661104489,
            0.67904766970737829,
        ],
        148.8317924119616,
    );
    ttranslate(
        s,
        "sh48D1E70_copy",
        [0., -309.31813764999998, -164.27444069000001],
    );
    copy(s, "sh467E330", "sh467E330_copy");
    trotate(
        s,
        "sh467E330_copy",
        [0., 0., 0.],
        [
            -0.679047659633503,
            -0.27890596661104489,
            0.67904766970737829,
        ],
        148.8317924119616,
    );
    ttranslate(
        s,
        "sh467E330_copy",
        [0., -309.31813764999998, -164.27444069000001],
    );
    bcut(s, "sh48C48E0", "sh467E330_copy", "sh48D1E70_copy");
    copy(s, "sh48C48E0", "sh48C48E0_copy");
    add(s, "sh48C48E0_copy", "result");
    checkprops(s, "result", Prop::Area, Expected::Mass(2.13824e+006));
    checkprops(s, "result", Prop::Volume, Expected::Mass(1.60388e+008));
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(10),
            NbCount::Edges(15),
            NbCount::Wires(9),
            NbCount::Faces(7),
            NbCount::Shells(1),
            NbCount::Solids(1),
        ],
    );
}

pub fn B3(s: &mut dyn Session) {
    compound(s, &[], "result");
    make_box(
        s,
        "sh4A2C530",
        [-321.87638082500001, -263.17581868000002, -254.],
        [643.75276165000002, 526.35163736000004, 508.],
    );
    let geom_pln_sh4C82480 = plane_axes([-0., -0., -254.], [0., 0., 1.], [1., 0., 0.]);
    pcylinder_on(
        s,
        "sh4C82480",
        geom_pln_sh4C82480.axes(),
        76.200000000000003,
        508.,
        360.,
    );
    settolerance(s, "sh4C82480", 5.0799999999999996e-006);
    copy(s, "sh4C82480", "sh4C82480_copy");
    trotate(
        s,
        "sh4C82480_copy",
        [0., 0., 0.],
        [
            -0.70710677745571249,
            3.6866829955513545e-009,
            0.70710678491738266,
        ],
        -179.99999957753724,
    );
    ttranslate(
        s,
        "sh4C82480_copy",
        [-67.876379483809757, -136.17581735118694, 127.0000021362337],
    );
    trotate(
        s,
        "sh4C82480_copy",
        [0., 0., 0.],
        [
            -0.679047659633503,
            -0.27890596661104489,
            0.67904766970737829,
        ],
        148.8317924119616,
    );
    ttranslate(
        s,
        "sh4C82480_copy",
        [0., -309.31813764999998, -164.27444069000001],
    );
    copy(s, "sh4A2C530", "sh4A2C530_copy");
    trotate(
        s,
        "sh4A2C530_copy",
        [0., 0., 0.],
        [
            -0.679047659633503,
            -0.27890596661104489,
            0.67904766970737829,
        ],
        148.8317924119616,
    );
    ttranslate(
        s,
        "sh4A2C530_copy",
        [0., -309.31813764999998, -164.27444069000001],
    );
    bcut(s, "sh4C74CC0", "sh4A2C530_copy", "sh4C82480_copy");
    copy(s, "sh4C74CC0", "sh4C74CC0_copy");
    add(s, "sh4C74CC0_copy", "result");
}
