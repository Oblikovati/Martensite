//! The `gdml_public` case bodies, part 8; each statement is one line of its script.
use super::*;

pub fn B5(s: &mut dyn Session) {
    compound(s, &[], "result");
    let geom_pln_sh44CB200 = plane_axes([-0., -0., -254.], [0., 0., 1.], [1., 0., 0.]);
    pcylinder_on(
        s,
        "sh44CB200",
        geom_pln_sh44CB200.axes(),
        50.799999999999997,
        508.,
        360.,
    );
    settolerance(s, "sh44CB200", 5.0799999999999996e-006);
    let geom_pln_sh44CA860 = plane_axes([-0., -0., -254.], [0., 0., 1.], [1., 0., 0.]);
    pcylinder_on(
        s,
        "sh44CA860",
        geom_pln_sh44CA860.axes(),
        50.799999999999997,
        508.,
        360.,
    );
    settolerance(s, "sh44CA860", 5.0799999999999996e-006);
    copy(s, "sh44CA860", "sh44CA860_copy");
    ttranslate(s, "sh44CA860_copy", [0., 254., 0.]);
    trotate(s, "sh44CA860_copy", [0., 0., 0.], [0., 1., 0.], 90.);
    ttranslate(s, "sh44CA860_copy", [254., 0., -127.]);
    let geom_pln_sh44CD3B0 = plane_axes([-0., -0., -254.], [0., 0., 1.], [1., 0., 0.]);
    pcylinder_on(
        s,
        "sh44CD3B0",
        geom_pln_sh44CD3B0.axes(),
        50.799999999999997,
        508.,
        360.,
    );
    settolerance(s, "sh44CD3B0", 5.0799999999999996e-006);
    copy(s, "sh44CD3B0", "sh44CD3B0_copy");
    ttranslate(s, "sh44CD3B0_copy", [0., 508., 0.]);
    trotate(s, "sh44CD3B0_copy", [0., 0., 0.], [0., 1., 0.], 90.);
    ttranslate(s, "sh44CD3B0_copy", [254., 0., -127.]);
    let geom_pln_sh44CE610 = plane_axes([-0., -0., -254.], [0., 0., 1.], [1., 0., 0.]);
    pcylinder_on(
        s,
        "sh44CE610",
        geom_pln_sh44CE610.axes(),
        50.799999999999997,
        508.,
        360.,
    );
    settolerance(s, "sh44CE610", 5.0799999999999996e-006);
    copy(s, "sh44CE610", "sh44CE610_copy");
    ttranslate(s, "sh44CE610_copy", [0., -254., 0.]);
    trotate(s, "sh44CE610_copy", [0., 0., 0.], [0., 1., 0.], 90.);
    ttranslate(s, "sh44CE610_copy", [254., 0., -127.]);
    let geom_pln_sh4471A30 = plane_axes([-0., -0., -254.], [0., 0., 1.], [1., 0., 0.]);
    pcylinder_on(
        s,
        "sh4471A30",
        geom_pln_sh4471A30.axes(),
        50.799999999999997,
        508.,
        360.,
    );
    settolerance(s, "sh4471A30", 5.0799999999999996e-006);
    copy(s, "sh4471A30", "sh4471A30_copy");
    ttranslate(s, "sh4471A30_copy", [0., -508., 0.]);
    trotate(s, "sh4471A30_copy", [0., 0., 0.], [0., 1., 0.], 90.);
    ttranslate(s, "sh4471A30_copy", [254., 0., -127.]);
    make_box(
        s,
        "sh4472C20",
        [-25.399999999999999, -50.799999999999997, -254.],
        [50.799999999999997, 101.59999999999999, 508.],
    );
    let geom_pln_sh4475F50 = plane_axes(
        [-0., -0., -254.],
        [0., 0., 1.],
        [-1., 1.2246467991473532e-016, 0.],
    );
    pcylinder_on(
        s,
        "sh4475F50",
        geom_pln_sh4475F50.axes(),
        50.799999999999997,
        508.,
        180.,
    );
    settolerance(s, "sh4475F50", 5.0799999999999996e-006);
    copy(s, "sh4475F50", "sh4475F50_copy");
    ttranslate(
        s,
        "sh4475F50_copy",
        [177.79999999999998, 381., -2.8421709430404007e-014],
    );
    trotate(s, "sh4475F50_copy", [0., 0., 0.], [0., 1., 0.], 90.);
    ttranslate(s, "sh4475F50_copy", [254., 0., -127.]);
    let geom_pln_sh4478090 = plane_axes([-0., -0., -254.], [0., 0., 1.], [1., 0., 0.]);
    pcylinder_on(
        s,
        "sh4478090",
        geom_pln_sh4478090.axes(),
        50.799999999999997,
        508.,
        180.,
    );
    settolerance(s, "sh4478090", 5.0799999999999996e-006);
    copy(s, "sh4478090", "sh4478090_copy");
    ttranslate(
        s,
        "sh4478090_copy",
        [
            177.79999999999998,
            431.80000000000001,
            -2.8421709430404007e-014,
        ],
    );
    trotate(s, "sh4478090_copy", [0., 0., 0.], [0., 1., 0.], 90.);
    ttranslate(s, "sh4478090_copy", [254., 0., -127.]);
    copy(s, "sh4472C20", "sh4472C20_copy");
    trotate(
        s,
        "sh4472C20_copy",
        [0., 0., 0.],
        [5.5511151231257827e-017, 1.6653345369377343e-016, -1.],
        90.,
    );
    ttranslate(
        s,
        "sh4472C20_copy",
        [
            177.79999999999998,
            406.39999999999998,
            -2.8421709430404007e-014,
        ],
    );
    trotate(s, "sh4472C20_copy", [0., 0., 0.], [0., 1., 0.], 90.);
    ttranslate(s, "sh4472C20_copy", [254., 0., -127.]);
    bfuse(s, "sh43FFED0", "sh4472C20_copy", "sh4475F50_copy");
    copy(s, "sh43FFED0", "sh43FFED0_copy");
    bfuse(s, "sh440E4F0", "sh43FFED0_copy", "sh4478090_copy");
    copy(s, "sh440E4F0", "sh440E4F0_copy");
    make_box(
        s,
        "sh4402DA0",
        [-25.399999999999999, -50.799999999999997, -254.],
        [50.799999999999997, 101.59999999999999, 508.],
    );
    let geom_pln_sh4406060 = plane_axes(
        [-0., -0., -254.],
        [0., 0., 1.],
        [-1., 1.2246467991473532e-016, 0.],
    );
    pcylinder_on(
        s,
        "sh4406060",
        geom_pln_sh4406060.axes(),
        50.799999999999997,
        508.,
        180.,
    );
    settolerance(s, "sh4406060", 5.0799999999999996e-006);
    copy(s, "sh4406060", "sh4406060_copy");
    ttranslate(
        s,
        "sh4406060_copy",
        [177.79999999999998, 127., -2.8421709430404007e-014],
    );
    trotate(s, "sh4406060_copy", [0., 0., 0.], [0., 1., 0.], 90.);
    ttranslate(s, "sh4406060_copy", [254., 0., -127.]);
    let geom_pln_sh440F520 = plane_axes([-0., -0., -254.], [0., 0., 1.], [1., 0., 0.]);
    pcylinder_on(
        s,
        "sh440F520",
        geom_pln_sh440F520.axes(),
        50.799999999999997,
        508.,
        180.,
    );
    settolerance(s, "sh440F520", 5.0799999999999996e-006);
    copy(s, "sh440F520", "sh440F520_copy");
    ttranslate(
        s,
        "sh440F520_copy",
        [
            177.79999999999998,
            177.80000000000001,
            -2.8421709430404007e-014,
        ],
    );
    trotate(s, "sh440F520_copy", [0., 0., 0.], [0., 1., 0.], 90.);
    ttranslate(s, "sh440F520_copy", [254., 0., -127.]);
    copy(s, "sh4402DA0", "sh4402DA0_copy");
    trotate(
        s,
        "sh4402DA0_copy",
        [0., 0., 0.],
        [5.5511151231257827e-017, 1.6653345369377343e-016, -1.],
        90.,
    );
    ttranslate(
        s,
        "sh4402DA0_copy",
        [
            177.79999999999998,
            152.40000000000001,
            -2.8421709430404007e-014,
        ],
    );
    trotate(s, "sh4402DA0_copy", [0., 0., 0.], [0., 1., 0.], 90.);
    ttranslate(s, "sh4402DA0_copy", [254., 0., -127.]);
    bfuse(s, "sh48388E0", "sh4402DA0_copy", "sh4406060_copy");
    copy(s, "sh48388E0", "sh48388E0_copy");
    bfuse(s, "sh4869BA0", "sh48388E0_copy", "sh440F520_copy");
    copy(s, "sh4869BA0", "sh4869BA0_copy");
    make_box(
        s,
        "sh4415C60",
        [-25.399999999999999, -50.799999999999997, -254.],
        [50.799999999999997, 101.59999999999999, 508.],
    );
    let geom_pln_sh440C3B0 = plane_axes(
        [-0., -0., -254.],
        [0., 0., 1.],
        [-1., 1.2246467991473532e-016, 0.],
    );
    pcylinder_on(
        s,
        "sh440C3B0",
        geom_pln_sh440C3B0.axes(),
        50.799999999999997,
        508.,
        180.,
    );
    settolerance(s, "sh440C3B0", 5.0799999999999996e-006);
    copy(s, "sh440C3B0", "sh440C3B0_copy");
    ttranslate(
        s,
        "sh440C3B0_copy",
        [
            177.79999999999998,
            -177.80000000000001,
            -2.8421709430404007e-014,
        ],
    );
    trotate(s, "sh440C3B0_copy", [0., 0., 0.], [0., 1., 0.], 90.);
    ttranslate(s, "sh440C3B0_copy", [254., 0., -127.]);
    let geom_pln_sh4417160 = plane_axes([-0., -0., -254.], [0., 0., 1.], [1., 0., 0.]);
    pcylinder_on(
        s,
        "sh4417160",
        geom_pln_sh4417160.axes(),
        50.799999999999997,
        508.,
        180.,
    );
    settolerance(s, "sh4417160", 5.0799999999999996e-006);
    copy(s, "sh4417160", "sh4417160_copy");
    ttranslate(
        s,
        "sh4417160_copy",
        [177.79999999999998, -127., -2.8421709430404007e-014],
    );
    trotate(s, "sh4417160_copy", [0., 0., 0.], [0., 1., 0.], 90.);
    ttranslate(s, "sh4417160_copy", [254., 0., -127.]);
    copy(s, "sh4415C60", "sh4415C60_copy");
    trotate(
        s,
        "sh4415C60_copy",
        [0., 0., 0.],
        [5.5511151231257827e-017, 1.6653345369377343e-016, -1.],
        90.,
    );
    ttranslate(
        s,
        "sh4415C60_copy",
        [
            177.79999999999998,
            -152.40000000000001,
            -2.8421709430404007e-014,
        ],
    );
    trotate(s, "sh4415C60_copy", [0., 0., 0.], [0., 1., 0.], 90.);
    ttranslate(s, "sh4415C60_copy", [254., 0., -127.]);
    bfuse(s, "sh48693C0", "sh4415C60_copy", "sh440C3B0_copy");
    copy(s, "sh48693C0", "sh48693C0_copy");
    bfuse(s, "sh441C0B0", "sh48693C0_copy", "sh4417160_copy");
    copy(s, "sh441C0B0", "sh441C0B0_copy");
    make_box(
        s,
        "sh441BCC0",
        [-25.399999999999999, -50.799999999999997, -254.],
        [50.799999999999997, 101.59999999999999, 508.],
    );
    let geom_pln_sh441EB90 = plane_axes(
        [-0., -0., -254.],
        [0., 0., 1.],
        [-1., 1.2246467991473532e-016, 0.],
    );
    pcylinder_on(
        s,
        "sh441EB90",
        geom_pln_sh441EB90.axes(),
        50.799999999999997,
        508.,
        180.,
    );
    settolerance(s, "sh441EB90", 5.0799999999999996e-006);
    copy(s, "sh441EB90", "sh441EB90_copy");
    ttranslate(
        s,
        "sh441EB90_copy",
        [
            177.79999999999998,
            -431.80000000000001,
            -2.8421709430404007e-014,
        ],
    );
    trotate(s, "sh441EB90_copy", [0., 0., 0.], [0., 1., 0.], 90.);
    ttranslate(s, "sh441EB90_copy", [254., 0., -127.]);
    let geom_pln_sh4420CD0 = plane_axes([-0., -0., -254.], [0., 0., 1.], [1., 0., 0.]);
    pcylinder_on(
        s,
        "sh4420CD0",
        geom_pln_sh4420CD0.axes(),
        50.799999999999997,
        508.,
        180.,
    );
    settolerance(s, "sh4420CD0", 5.0799999999999996e-006);
    copy(s, "sh4420CD0", "sh4420CD0_copy");
    ttranslate(
        s,
        "sh4420CD0_copy",
        [177.79999999999998, -381., -2.8421709430404007e-014],
    );
    trotate(s, "sh4420CD0_copy", [0., 0., 0.], [0., 1., 0.], 90.);
    ttranslate(s, "sh4420CD0_copy", [254., 0., -127.]);
    copy(s, "sh441BCC0", "sh441BCC0_copy");
    trotate(
        s,
        "sh441BCC0_copy",
        [0., 0., 0.],
        [5.5511151231257827e-017, 1.6653345369377343e-016, -1.],
        90.,
    );
    ttranslate(
        s,
        "sh441BCC0_copy",
        [
            177.79999999999998,
            -406.39999999999998,
            -2.8421709430404007e-014,
        ],
    );
    trotate(s, "sh441BCC0_copy", [0., 0., 0.], [0., 1., 0.], 90.);
    ttranslate(s, "sh441BCC0_copy", [254., 0., -127.]);
    bfuse(s, "sh4437FD0", "sh441BCC0_copy", "sh441EB90_copy");
    copy(s, "sh4437FD0", "sh4437FD0_copy");
    bfuse(s, "sh4530B10", "sh4437FD0_copy", "sh4420CD0_copy");
    copy(s, "sh4530B10", "sh4530B10_copy");
    copy(s, "sh44CB200", "sh44CB200_copy");
    trotate(s, "sh44CB200_copy", [0., 0., 0.], [0., 1., 0.], 90.);
    ttranslate(s, "sh44CB200_copy", [254., 0., -127.]);
    bfuse(s, "sh452E260", "sh44CB200_copy", "sh44CA860_copy");
    copy(s, "sh452E260", "sh452E260_copy");
    bfuse(s, "sh45321D0", "sh452E260_copy", "sh44CD3B0_copy");
    copy(s, "sh45321D0", "sh45321D0_copy");
    bfuse(s, "sh452D540", "sh45321D0_copy", "sh44CE610_copy");
    copy(s, "sh452D540", "sh452D540_copy");
    bfuse(s, "sh45293A0", "sh452D540_copy", "sh4471A30_copy");
    copy(s, "sh45293A0", "sh45293A0_copy");
    bfuse(s, "sh4527500", "sh45293A0_copy", "sh440E4F0_copy");
    copy(s, "sh4527500", "sh4527500_copy");
    bfuse(s, "sh4524EF0", "sh4527500_copy", "sh4869BA0_copy");
    copy(s, "sh4524EF0", "sh4524EF0_copy");
    bfuse(s, "sh452F840", "sh4524EF0_copy", "sh441C0B0_copy");
    copy(s, "sh452F840", "sh452F840_copy");
    bfuse(s, "sh452FBC0", "sh452F840_copy", "sh4530B10_copy");
    copy(s, "sh452FBC0", "sh452FBC0_copy");
    add(s, "sh452FBC0_copy", "result");
}
