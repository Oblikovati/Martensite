//! The `gdml_public` case bodies, part 9; each statement is one line of its script.
use super::*;

pub fn B6(s: &mut dyn Session) {
    compound(s, &[], "result");
    let geom_pln_sh4F61F90 = plane_axes([0., 0., 0.], [0., 0., 1.], [1., 0., 0.]);
    ptorus_on(
        s,
        "sh4F61F90",
        geom_pln_sh4F61F90.axes(),
        127.,
        51.718063649999998,
        360.,
    );
    settolerance(s, "sh4F61F90", 1.2699999999999999e-006);
    let geom_pln_sh4F61900 = plane_axes([0., 0., 0.], [0., 0., 1.], [1., 0., 0.]);
    ptorus_on(
        s,
        "sh4F61900",
        geom_pln_sh4F61900.axes(),
        127.,
        51.718063649999998,
        360.,
    );
    settolerance(s, "sh4F61900", 1.2699999999999999e-006);
    copy(s, "sh4F61900", "sh4F61900_copy");
    ttranslate(s, "sh4F61900_copy", [0., 2.8199664825478976e-014, -254.]);
    trotate(s, "sh4F61900_copy", [0., 0., 0.], [1., 0., 0.], 90.);
    let geom_pln_sh4F635E0 = plane_axes([0., 0., 0.], [0., 0., 1.], [1., 0., 0.]);
    ptorus_on(
        s,
        "sh4F635E0",
        geom_pln_sh4F635E0.axes(),
        127.,
        51.718063649999998,
        360.,
    );
    settolerance(s, "sh4F635E0", 1.2699999999999999e-006);
    copy(s, "sh4F635E0", "sh4F635E0_copy");
    ttranslate(s, "sh4F635E0_copy", [0., 5.6399329650957952e-014, -508.]);
    trotate(s, "sh4F635E0_copy", [0., 0., 0.], [1., 0., 0.], 90.);
    let geom_pln_sh4F64370 = plane_axes([0., 0., 0.], [0., 0., 1.], [1., 0., 0.]);
    ptorus_on(
        s,
        "sh4F64370",
        geom_pln_sh4F64370.axes(),
        127.,
        51.718063649999998,
        360.,
    );
    settolerance(s, "sh4F64370", 1.2699999999999999e-006);
    copy(s, "sh4F64370", "sh4F64370_copy");
    ttranslate(s, "sh4F64370_copy", [0., -2.8199664825478976e-014, 254.]);
    trotate(s, "sh4F64370_copy", [0., 0., 0.], [1., 0., 0.], 90.);
    let geom_pln_sh4F65100 = plane_axes([0., 0., 0.], [0., 0., 1.], [1., 0., 0.]);
    ptorus_on(
        s,
        "sh4F65100",
        geom_pln_sh4F65100.axes(),
        127.,
        51.718063649999998,
        360.,
    );
    settolerance(s, "sh4F65100", 1.2699999999999999e-006);
    copy(s, "sh4F65100", "sh4F65100_copy");
    ttranslate(s, "sh4F65100_copy", [0., -5.6399329650957952e-014, 508.]);
    trotate(s, "sh4F65100_copy", [0., 0., 0.], [1., 0., 0.], 90.);
    copy(s, "sh4F61F90", "sh4F61F90_copy");
    trotate(s, "sh4F61F90_copy", [0., 0., 0.], [1., 0., 0.], 90.);
    bfuse(s, "sh4F66D00", "sh4F61F90_copy", "sh4F61900_copy");
    copy(s, "sh4F66D00", "sh4F66D00_copy");
    bfuse(s, "sh4BBD460", "sh4F66D00_copy", "sh4F635E0_copy");
    copy(s, "sh4BBD460", "sh4BBD460_copy");
    bfuse(s, "sh4BBD310", "sh4BBD460_copy", "sh4F64370_copy");
    copy(s, "sh4BBD310", "sh4BBD310_copy");
    bfuse(s, "sh4BBE810", "sh4BBD310_copy", "sh4F65100_copy");
    copy(s, "sh4BBE810", "sh4BBE810_copy");
    add(s, "sh4BBE810_copy", "result");
}

pub fn B7(s: &mut dyn Session) {
    compound(s, &[], "result");
    make_box(
        s,
        "sh48E68B0",
        [-190.50000009999999, -254.00000009999999, -381.],
        [381.00000019999999, 508.00000019999999, 762.],
    );
    make_box(
        s,
        "sh492B120",
        [
            -137.39791879500001,
            -137.39791879500001,
            -396.24000000000001,
        ],
        [274.79583759000002, 274.79583759000002, 792.48000000000002],
    );
    copy(s, "sh492B120", "sh492B120_copy");
    trotate(
        s,
        "sh492B120_copy",
        [0., 0., 0.],
        [-7.417649474675704e-009, -3.0724910593272844e-009, 1.],
        135.,
    );
    ttranslate(s, "sh492B120_copy", [192.4050001, 255.9050001, 0.]);
    trotate(s, "sh492B120_copy", [0., 0., 0.], [0., 1., 0.], 90.);
    ttranslate(s, "sh492B120_copy", [508., 127., -63.5]);
    make_box(
        s,
        "sh492E610",
        [
            -137.39791879500001,
            -137.39791879500001,
            -411.48000000000002,
        ],
        [274.79583759000002, 274.79583759000002, 822.96000000000004],
    );
    copy(s, "sh492E610", "sh492E610_copy");
    trotate(
        s,
        "sh492E610_copy",
        [0., 0., 0.],
        [5.5511151231257802e-017, 1.3401577416544655e-016, 1.],
        45.000000000000007,
    );
    ttranslate(s, "sh492E610_copy", [192.4050001, -255.9050001, 0.]);
    trotate(s, "sh492E610_copy", [0., 0., 0.], [0., 1., 0.], 90.);
    ttranslate(s, "sh492E610_copy", [508., 127., -63.5]);
    let geom_pln_sh4931710 = plane_axes([-0., -0., -381.], [0., 0., 1.], [1., 0., 0.]);
    pcylinder_on(s, "sh4931710", geom_pln_sh4931710.axes(), 190.5, 762., 360.);
    settolerance(s, "sh4931710", 7.6199999999999999e-006);
    make_box(
        s,
        "sh4932660",
        [-192.405, -28.177059884999998, -388.62],
        [384.81, 56.354119769999997, 777.24000000000001],
    );
    copy(s, "sh4932660", "sh4932660_copy");
    trotate(
        s,
        "sh4932660_copy",
        [0., 0., 0.],
        [5.5511151231257802e-017, 1.3401577416544655e-016, 1.],
        45.000000000000007,
    );
    ttranslate(
        s,
        "sh4932660_copy",
        [75.325809880000008, -138.82580988000001, 0.],
    );
    trotate(s, "sh4932660_copy", [0., 0., 0.], [0., 1., 0.], 90.);
    ttranslate(s, "sh4932660_copy", [508., 127., -63.5]);
    copy(s, "sh4931710", "sh4931710_copy");
    ttranslate(s, "sh4931710_copy", [190.5, -254., 0.]);
    trotate(s, "sh4931710_copy", [0., 0., 0.], [0., 1., 0.], 90.);
    ttranslate(s, "sh4931710_copy", [508., 127., -63.5]);
    bcut(s, "sh48FF270", "sh4931710_copy", "sh4932660_copy");
    copy(s, "sh48FF270", "sh48FF270_copy");
    let geom_pln_sh48FAA40 = plane_axes([-0., -0., -381.], [0., 0., 1.], [1., 0., 0.]);
    pcylinder_on(s, "sh48FAA40", geom_pln_sh48FAA40.axes(), 190.5, 762., 360.);
    settolerance(s, "sh48FAA40", 7.6199999999999999e-006);
    make_box(
        s,
        "sh48FEA90",
        [-192.405, -28.177059884999998, -388.62],
        [384.81, 56.354119769999997, 777.24000000000001],
    );
    copy(s, "sh48FEA90", "sh48FEA90_copy");
    trotate(
        s,
        "sh48FEA90_copy",
        [0., 0., 0.],
        [-3.2517679528326877e-017, 1.2449156634005056e-016, 1.],
        135.,
    );
    ttranslate(
        s,
        "sh48FEA90_copy",
        [75.325809880000008, 138.82580988000001, 0.],
    );
    trotate(s, "sh48FEA90_copy", [0., 0., 0.], [0., 1., 0.], 90.);
    ttranslate(s, "sh48FEA90_copy", [508., 127., -63.5]);
    copy(s, "sh48FAA40", "sh48FAA40_copy");
    ttranslate(s, "sh48FAA40_copy", [190.5, 254., 0.]);
    trotate(s, "sh48FAA40_copy", [0., 0., 0.], [0., 1., 0.], 90.);
    ttranslate(s, "sh48FAA40_copy", [508., 127., -63.5]);
    bcut(s, "sh44CB930", "sh48FAA40_copy", "sh48FEA90_copy");
    copy(s, "sh44CB930", "sh44CB930_copy");
    copy(s, "sh48E68B0", "sh48E68B0_copy");
    trotate(s, "sh48E68B0_copy", [0., 0., 0.], [0., 1., 0.], 90.);
    ttranslate(s, "sh48E68B0_copy", [508., 127., -63.5]);
    bcut(s, "sh4576BB0", "sh48E68B0_copy", "sh492B120_copy");
    copy(s, "sh4576BB0", "sh4576BB0_copy");
    bcut(s, "sh4585A00", "sh4576BB0_copy", "sh492E610_copy");
    copy(s, "sh4585A00", "sh4585A00_copy");
    bfuse(s, "sh44D2540", "sh4585A00_copy", "sh48FF270_copy");
    copy(s, "sh44D2540", "sh44D2540_copy");
    bfuse(s, "sh9055790", "sh44D2540_copy", "sh44CB930_copy");
    copy(s, "sh9055790", "sh9055790_copy");
    add(s, "sh9055790_copy", "result");
}

pub fn B8(s: &mut dyn Session) {
    compound(s, &[], "result");
    let geom_pln_sh4846FB0 = plane_axes(
        [-0., -0., -442.71730065000003],
        [0., 0., 1.],
        [1.1102230246251565e-016, -1., 0.],
    );
    pcylinder_on(
        s,
        "sh4846FB0",
        geom_pln_sh4846FB0.axes(),
        702.71927595,
        885.43460130000005,
        300.00000000000006,
    );
    settolerance(s, "sh4846FB0", 8.8543460130000009e-006);
    let geom_pln_sh4849710 = plane_axes(
        [-0., -0., -442.71730065000003],
        [0., 0., 1.],
        [1.1102230246251565e-016, -1., 0.],
    );
    pcylinder_on(
        s,
        "sh4849710",
        geom_pln_sh4849710.axes(),
        153.59337693000001,
        885.43460130000005,
        300.00000000000006,
    );
    settolerance(s, "sh4849710", 8.8543460130000009e-006);
    copy(s, "sh4849710", "sh4849710_copy");
    trotate(
        s,
        "sh4849710_copy",
        [0., 0., 0.],
        [1., 0., 0.],
        -161.93649349000003,
    );
    ttranslate(
        s,
        "sh4849710_copy",
        [127., -258.32178088000001, -177.99555741],
    );
    make_box(
        s,
        "sh484BA10",
        [-150.79893332500001, -150.79893332500001, -508.],
        [301.59786665000001, 301.59786665000001, 1016.],
    );
    copy(s, "sh484BA10", "sh484BA10_copy");
    trotate(
        s,
        "sh484BA10_copy",
        [0., 0., 0.],
        [
            -0.70710678118654735,
            0.70710678118654768,
            8.7731777483398112e-017,
        ],
        180.,
    );
    ttranslate(
        s,
        "sh484BA10_copy",
        [399.74468245000003, 178.4271923835945, -950.71730065346742],
    );
    trotate(
        s,
        "sh484BA10_copy",
        [0., 0., 0.],
        [1., 0., 0.],
        -161.93649349000003,
    );
    ttranslate(
        s,
        "sh484BA10_copy",
        [127., -258.32178088000001, -177.99555741],
    );
    copy(s, "sh4846FB0", "sh4846FB0_copy");
    trotate(
        s,
        "sh4846FB0_copy",
        [0., 0., 0.],
        [1., 0., 0.],
        -161.93649349000003,
    );
    ttranslate(
        s,
        "sh4846FB0_copy",
        [127., -258.32178088000001, -177.99555741],
    );
    bcut(s, "sh480D0A0", "sh4846FB0_copy", "sh4849710_copy");
    copy(s, "sh480D0A0", "sh480D0A0_copy");
    bfuse(s, "sh4811390", "sh480D0A0_copy", "sh484BA10_copy");
    copy(s, "sh4811390", "sh4811390_copy");
    add(s, "sh4811390_copy", "result");
}

pub fn B9(s: &mut dyn Session) {
    compound(s, &[], "result");
    let geom_pln_sh48005B0 = plane_axes(
        [0., 0., 2109.8582757863655],
        [0., 0., 1.],
        [-0.50000000000000044, 0.86602540378443837, 0.],
    );
    pcone_on(
        s,
        "sh48005B0",
        geom_pln_sh48005B0.axes(),
        [378.91951769999991, 466.55429973000008, 487.95842250363467],
        300.00000000000006,
    );
    settolerance(s, "sh48005B0", 2.5978166982899998e-005);
    let geom_pln_sh4802B50 = plane_axes(
        [0., 0., 31.755811739557313],
        [0., 0., 1.],
        [0.50000000000000011, -0.8660254037844386, 0.],
    );
    pcone_on(
        s,
        "sh4802B50",
        geom_pln_sh4802B50.axes(),
        [176.81924281000002, 466.55429973000008, 52.03490172044269],
        300.00000000000006,
    );
    settolerance(s, "sh4802B50", 4.6655429973000009e-006);
    copy(s, "sh4802B50", "sh4802B50_copy");
    trotate(s, "sh4802B50_copy", [0., 0., 0.], [1., 0., 0.], -180.);
    ttranslate(
        s,
        "sh4802B50_copy",
        [0., -2.2782911912599957e-007, 2681.60741175578],
    );
    trotate(
        s,
        "sh4802B50_copy",
        [0., 0., 0.],
        [1., 0., 0.],
        -161.32582176,
    );
    ttranslate(
        s,
        "sh4802B50_copy",
        [0., -746.52181700999995, 1844.22896887],
    );
    let geom_pln_sh4804C90 = plane_axes(
        [0., 0., 15.856892340754207],
        [0., 0., 1.],
        [0.50000000000000011, -0.8660254037844386, 0.],
    );
    pcone_on(
        s,
        "sh4804C90",
        geom_pln_sh4804C90.axes(),
        [88.292616169999931, 379.81136230999971, 52.355242979245794],
        300.00000000000006,
    );
    settolerance(s, "sh4804C90", 3.7981136230999974e-006);
    copy(s, "sh4804C90", "sh4804C90_copy");
    trotate(s, "sh4804C90_copy", [0., 0., 0.], [1., 0., 0.], -180.);
    ttranslate(
        s,
        "sh4804C90_copy",
        [0., -1.8272221780080145e-007, 2177.9102405158328],
    );
    trotate(
        s,
        "sh4804C90_copy",
        [0., 0., 0.],
        [1., 0., 0.],
        -161.32582176,
    );
    ttranslate(
        s,
        "sh4804C90_copy",
        [0., -746.52181700999995, 1844.22896887],
    );
    let geom_pln_sh4B8A8D0 = plane_axes(
        [0., 0., 491.62130272815972],
        [0., 0., 1.],
        [-0.50000000000000044, 0.86602540378443837, 0.],
    );
    pcone_on(
        s,
        "sh4B8A8D0",
        geom_pln_sh4B8A8D0.axes(),
        [88.292616170000002, 177.71108741, 497.8901659718403],
        300.00000000000006,
    );
    settolerance(s, "sh4B8A8D0", 9.8951146869999999e-006);
    copy(s, "sh4B8A8D0", "sh4B8A8D0_copy");
    ttranslate(
        s,
        "sh4B8A8D0_copy",
        [0., -1.3349050220767822e-007, 1665.3060030573038],
    );
    trotate(
        s,
        "sh4B8A8D0_copy",
        [0., 0., 0.],
        [1., 0., 0.],
        -161.32582176,
    );
    ttranslate(
        s,
        "sh4B8A8D0_copy",
        [0., -746.52181700999995, 1844.22896887],
    );
    copy(s, "sh48005B0", "sh48005B0_copy");
    trotate(
        s,
        "sh48005B0_copy",
        [0., 0., 0.],
        [1., 0., 0.],
        -161.32582176,
    );
    ttranslate(
        s,
        "sh48005B0_copy",
        [0., -746.52181700999995, 1844.22896887],
    );
    bfuse(s, "sh2804920", "sh48005B0_copy", "sh4802B50_copy");
    copy(s, "sh2804920", "sh2804920_copy");
    bcut(s, "sh47B1A70", "sh2804920_copy", "sh4804C90_copy");
    copy(s, "sh47B1A70", "sh47B1A70_copy");
    bcut(s, "sh952B3D0", "sh47B1A70_copy", "sh4B8A8D0_copy");
    copy(s, "sh952B3D0", "sh952B3D0_copy");
    add(s, "sh952B3D0_copy", "result");
}
