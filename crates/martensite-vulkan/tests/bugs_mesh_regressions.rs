//! Meshing regressions of the bugs mesh group, each in an isolated worker under the bugs policy.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
mod bugs_mesh_cases;
use bugs_mesh_cases::{MESH_TWEAKS, UV_COINCIDENCE, meshed_face_view};

fn run(identity: &str, heading: &'static [&'static str], input: &'static str, image: &'static str) {
    case_bugs_process_case::run_bugs_case(identity, || meshed_face_view(heading, input, image));
}

#[test]
fn bug25044_1() {
    run(
        "bug25044_1",
        &MESH_TWEAKS,
        "bug25044_face1.brep",
        "bug25044_1.png",
    );
}

#[test]
fn bug25044_2() {
    run(
        "bug25044_2",
        &MESH_TWEAKS,
        "bug25044_face2.brep",
        "bug25044_2.png",
    );
}

#[test]
fn bug25044_3() {
    run(
        "bug25044_3",
        &MESH_TWEAKS,
        "bug25044_face3.brep",
        "bug25044_3.png",
    );
}

#[test]
fn bug25044_4() {
    run(
        "bug25044_4",
        &MESH_TWEAKS,
        "bug25044_face4.brep",
        "bug25044_4.png",
    );
}

#[test]
fn bug25044_5() {
    run(
        "bug25044_5",
        &MESH_TWEAKS,
        "bug25044_face5.brep",
        "bug25044_5.png",
    );
}

#[test]
fn bug25044_6() {
    run(
        "bug25044_6",
        &MESH_TWEAKS,
        "bug25044_face6.brep",
        "bug25044_6.png",
    );
}

#[test]
fn bug25044_7() {
    run(
        "bug25044_7",
        &MESH_TWEAKS,
        "bug25044_face7.brep",
        "bug25044_7.png",
    );
}

#[test]
fn bug25044_8() {
    run(
        "bug25044_8",
        &MESH_TWEAKS,
        "bug25044_face8.brep",
        "bug25044_8.png",
    );
}

#[test]
fn bug25044_9() {
    run(
        "bug25044_9",
        &MESH_TWEAKS,
        "bug25044_face9.brep",
        "bug25044_9.png",
    );
}

#[test]
fn bug25044_10() {
    run(
        "bug25044_10",
        &MESH_TWEAKS,
        "bug25044_face10.brep",
        "bug25044_10.png",
    );
}

#[test]
fn bug25044_11() {
    run(
        "bug25044_11",
        &MESH_TWEAKS,
        "bug25044_face11.brep",
        "bug25044_11.png",
    );
}

#[test]
fn bug25044_12() {
    run(
        "bug25044_12",
        &MESH_TWEAKS,
        "bug25044_face12.brep",
        "bug25044_12.png",
    );
}

#[test]
fn bug25044_13() {
    run(
        "bug25044_13",
        &MESH_TWEAKS,
        "bug25044_face13.brep",
        "bug25044_13.png",
    );
}

#[test]
fn bug25044_14() {
    run(
        "bug25044_14",
        &MESH_TWEAKS,
        "bug25044_face14.brep",
        "bug25044_14.png",
    );
}

#[test]
fn bug25044_15() {
    run(
        "bug25044_15",
        &MESH_TWEAKS,
        "bug25044_face15.brep",
        "bug25044_15.png",
    );
}

#[test]
fn bug25044_16() {
    run(
        "bug25044_16",
        &MESH_TWEAKS,
        "bug25044_face16.brep",
        "bug25044_16.png",
    );
}

#[test]
fn bug25044_17() {
    run(
        "bug25044_17",
        &MESH_TWEAKS,
        "bug25044_face17.brep",
        "bug25044_17.png",
    );
}

#[test]
fn bug25044_18() {
    run(
        "bug25044_18",
        &MESH_TWEAKS,
        "bug25044_face18.brep",
        "bug25044_18.png",
    );
}

#[test]
fn bug25044_19() {
    run(
        "bug25044_19",
        &MESH_TWEAKS,
        "bug25044_face19.brep",
        "bug25044_19.png",
    );
}

#[test]
fn bug25044_20() {
    run(
        "bug25044_20",
        &MESH_TWEAKS,
        "bug25044_face20.brep",
        "bug25044_20.png",
    );
}

#[test]
fn bug25044_21() {
    run(
        "bug25044_21",
        &MESH_TWEAKS,
        "bug25044_face21.brep",
        "bug25044_21.png",
    );
}

#[test]
fn bug25044_22() {
    run(
        "bug25044_22",
        &MESH_TWEAKS,
        "bug25044_face22.brep",
        "bug25044_22.png",
    );
}

#[test]
fn bug25044_23() {
    run(
        "bug25044_23",
        &MESH_TWEAKS,
        "bug25044_face23.brep",
        "bug25044_23.png",
    );
}

#[test]
fn bug25044_24() {
    run(
        "bug25044_24",
        &MESH_TWEAKS,
        "bug25044_face24.brep",
        "bug25044_24.png",
    );
}

#[test]
fn bug25044_25() {
    run(
        "bug25044_25",
        &MESH_TWEAKS,
        "bug25044_face25.brep",
        "bug25044_25.png",
    );
}

#[test]
fn bug25044_26() {
    run(
        "bug25044_26",
        &MESH_TWEAKS,
        "bug25044_face26.brep",
        "bug25044_26.png",
    );
}

#[test]
fn bug25044_27() {
    run(
        "bug25044_27",
        &MESH_TWEAKS,
        "bug25044_face27.brep",
        "bug25044_27.png",
    );
}

#[test]
fn bug25044_28() {
    run(
        "bug25044_28",
        &MESH_TWEAKS,
        "bug25044_face28.brep",
        "bug25044_28.png",
    );
}

#[test]
fn bug25044_29() {
    run(
        "bug25044_29",
        &MESH_TWEAKS,
        "bug25044_face29.brep",
        "bug25044_29.png",
    );
}

#[test]
fn bug25044_30() {
    run(
        "bug25044_30",
        &MESH_TWEAKS,
        "bug25044_face30.brep",
        "bug25044_30.png",
    );
}

#[test]
fn bug25044_31() {
    run(
        "bug25044_31",
        &MESH_TWEAKS,
        "bug25044_face31.brep",
        "bug25044_31.png",
    );
}

#[test]
fn bug25044_32() {
    run(
        "bug25044_32",
        &MESH_TWEAKS,
        "bug25044_face32.brep",
        "bug25044_32.png",
    );
}

#[test]
fn bug25044_33() {
    run(
        "bug25044_33",
        &MESH_TWEAKS,
        "bug25044_face33.brep",
        "bug25044_33.png",
    );
}

#[test]
fn bug25044_34() {
    run(
        "bug25044_34",
        &MESH_TWEAKS,
        "bug25044_face34.brep",
        "bug25044_34.png",
    );
}

#[test]
fn bug25044_35() {
    run(
        "bug25044_35",
        &MESH_TWEAKS,
        "bug25044_face35.brep",
        "bug25044_35.png",
    );
}

#[test]
fn bug25044_36() {
    run(
        "bug25044_36",
        &MESH_TWEAKS,
        "bug25044_face36.brep",
        "bug25044_36.png",
    );
}

#[test]
fn bug25044_37() {
    run(
        "bug25044_37",
        &MESH_TWEAKS,
        "bug25044_face37.brep",
        "bug25044_37.png",
    );
}

#[test]
fn bug25044_38() {
    run(
        "bug25044_38",
        &MESH_TWEAKS,
        "bug25044_face38.brep",
        "bug25044_38.png",
    );
}

#[test]
fn bug25044_39() {
    run(
        "bug25044_39",
        &MESH_TWEAKS,
        "bug25044_face39.brep",
        "bug25044_39.png",
    );
}

#[test]
fn bug25044_40() {
    run(
        "bug25044_40",
        &MESH_TWEAKS,
        "bug25044_face40.brep",
        "bug25044_40.png",
    );
}

#[test]
fn bug25044_41() {
    run(
        "bug25044_41",
        &MESH_TWEAKS,
        "bug25044_face41.brep",
        "bug25044_41.png",
    );
}

#[test]
fn bug25044_42() {
    run(
        "bug25044_42",
        &MESH_TWEAKS,
        "bug25044_face42.brep",
        "bug25044_42.png",
    );
}

#[test]
fn bug25044_43() {
    run(
        "bug25044_43",
        &MESH_TWEAKS,
        "bug25044_face43.brep",
        "bug25044_43.png",
    );
}

#[test]
fn bug25044_44() {
    run(
        "bug25044_44",
        &MESH_TWEAKS,
        "bug25044_face44.brep",
        "bug25044_44.png",
    );
}

#[test]
fn bug25044_45() {
    run(
        "bug25044_45",
        &MESH_TWEAKS,
        "bug25044_face45.brep",
        "bug25044_45.png",
    );
}

#[test]
fn bug25044_46() {
    run(
        "bug25044_46",
        &MESH_TWEAKS,
        "bug25044_face46.brep",
        "bug25044_46.png",
    );
}

#[test]
fn bug25044_47() {
    run(
        "bug25044_47",
        &MESH_TWEAKS,
        "bug25044_face47.brep",
        "bug25044_47.png",
    );
}

#[test]
fn bug25044_48() {
    run(
        "bug25044_48",
        &MESH_TWEAKS,
        "bug25044_face48.brep",
        "bug25044_48.png",
    );
}

#[test]
fn bug25044_49() {
    run(
        "bug25044_49",
        &MESH_TWEAKS,
        "bug25044_face49.brep",
        "bug25044_49.png",
    );
}

#[test]
fn bug25044_50() {
    run(
        "bug25044_50",
        &MESH_TWEAKS,
        "bug25044_face50.brep",
        "bug25044_50.png",
    );
}

#[test]
fn bug25044_51() {
    run(
        "bug25044_51",
        &MESH_TWEAKS,
        "bug25044_face51.brep",
        "bug25044_51.png",
    );
}

#[test]
fn bug25044_52() {
    run(
        "bug25044_52",
        &MESH_TWEAKS,
        "bug25044_face52.brep",
        "bug25044_52.png",
    );
}

#[test]
fn bug25044_53() {
    run(
        "bug25044_53",
        &MESH_TWEAKS,
        "bug25044_face53.brep",
        "bug25044_53.png",
    );
}

#[test]
fn bug25044_54() {
    run(
        "bug25044_54",
        &MESH_TWEAKS,
        "bug25044_face54.brep",
        "bug25044_54.png",
    );
}

#[test]
fn bug25044_55() {
    run(
        "bug25044_55",
        &MESH_TWEAKS,
        "bug25044_face55.brep",
        "bug25044_55.png",
    );
}

#[test]
fn bug25044_56() {
    run(
        "bug25044_56",
        &MESH_TWEAKS,
        "bug25044_face56.brep",
        "bug25044_56.png",
    );
}

#[test]
fn bug25044_57() {
    run(
        "bug25044_57",
        &MESH_TWEAKS,
        "bug25044_face57.brep",
        "bug25044_57.png",
    );
}

#[test]
fn bug25044_58() {
    run(
        "bug25044_58",
        &MESH_TWEAKS,
        "bug25044_face58.brep",
        "bug25044_58.png",
    );
}

#[test]
fn bug25588_1() {
    run(
        "bug25588_1",
        &UV_COINCIDENCE,
        "1.stp_face691.brep",
        "bug25588_1.png",
    );
}

#[test]
fn bug25588_2() {
    run(
        "bug25588_2",
        &UV_COINCIDENCE,
        "1.stp_face718.brep",
        "bug25588_2.png",
    );
}

#[test]
fn bug25588_3() {
    run(
        "bug25588_3",
        &UV_COINCIDENCE,
        "1.stp_face745.brep",
        "bug25588_3.png",
    );
}

#[test]
fn bug25588_4() {
    run(
        "bug25588_4",
        &UV_COINCIDENCE,
        "1.stp_face772.brep",
        "bug25588_4.png",
    );
}

#[test]
fn bug25588_5() {
    run(
        "bug25588_5",
        &UV_COINCIDENCE,
        "1.stp_face799.brep",
        "bug25588_5.png",
    );
}

#[test]
fn bug25588_6() {
    run(
        "bug25588_6",
        &UV_COINCIDENCE,
        "1.stp_face826.brep",
        "bug25588_6.png",
    );
}

#[test]
fn bug25588_7() {
    run(
        "bug25588_7",
        &UV_COINCIDENCE,
        "1.stp_face853.brep",
        "bug25588_7.png",
    );
}

#[test]
fn bug25588_8() {
    run(
        "bug25588_8",
        &UV_COINCIDENCE,
        "1.stp_face880.brep",
        "bug25588_8.png",
    );
}

#[test]
fn bug25588_9() {
    run(
        "bug25588_9",
        &UV_COINCIDENCE,
        "1.stp_face907.brep",
        "bug25588_9.png",
    );
}

#[test]
fn bug25588_10() {
    run(
        "bug25588_10",
        &UV_COINCIDENCE,
        "1.stp_face934.brep",
        "bug25588_10.png",
    );
}

#[test]
fn bug25588_11() {
    run(
        "bug25588_11",
        &UV_COINCIDENCE,
        "1.stp_face961.brep",
        "bug25588_11.png",
    );
}

#[test]
fn bug25588_12() {
    run(
        "bug25588_12",
        &UV_COINCIDENCE,
        "1.stp_face988.brep",
        "bug25588_12.png",
    );
}

#[test]
fn bug25588_13() {
    run(
        "bug25588_13",
        &UV_COINCIDENCE,
        "1.stp_face1015.brep",
        "bug25588_13.png",
    );
}

#[test]
fn bug25588_14() {
    run(
        "bug25588_14",
        &UV_COINCIDENCE,
        "1.stp_face1042.brep",
        "bug25588_14.png",
    );
}

#[test]
fn bug25588_15() {
    run(
        "bug25588_15",
        &UV_COINCIDENCE,
        "1.stp_face1069.brep",
        "bug25588_15.png",
    );
}

#[test]
fn bug25588_16() {
    run(
        "bug25588_16",
        &UV_COINCIDENCE,
        "1.stp_face1096.brep",
        "bug25588_16.png",
    );
}

#[test]
fn bug25588_17() {
    run(
        "bug25588_17",
        &UV_COINCIDENCE,
        "1.stp_face1123.brep",
        "bug25588_17.png",
    );
}

#[test]
fn bug25588_18() {
    run(
        "bug25588_18",
        &UV_COINCIDENCE,
        "1.stp_face1150.brep",
        "bug25588_18.png",
    );
}

#[test]
fn bug25588_19() {
    run(
        "bug25588_19",
        &UV_COINCIDENCE,
        "1.stp_face1177.brep",
        "bug25588_19.png",
    );
}

#[test]
fn bug25588_20() {
    run(
        "bug25588_20",
        &UV_COINCIDENCE,
        "1.stp_face1204.brep",
        "bug25588_20.png",
    );
}

#[test]
fn bug25588_21() {
    run(
        "bug25588_21",
        &UV_COINCIDENCE,
        "1.stp_face1231.brep",
        "bug25588_21.png",
    );
}

#[test]
fn bug25588_22() {
    run(
        "bug25588_22",
        &UV_COINCIDENCE,
        "1.stp_face1258.brep",
        "bug25588_22.png",
    );
}

#[test]
fn bug25588_23() {
    run(
        "bug25588_23",
        &UV_COINCIDENCE,
        "1.stp_face1285.brep",
        "bug25588_23.png",
    );
}

#[test]
fn bug25588_24() {
    run(
        "bug25588_24",
        &UV_COINCIDENCE,
        "1.stp_face1312.brep",
        "bug25588_24.png",
    );
}

#[test]
fn bug25588_25() {
    run(
        "bug25588_25",
        &UV_COINCIDENCE,
        "1.stp_face1339.brep",
        "bug25588_25.png",
    );
}

#[test]
fn bug25588_26() {
    run(
        "bug25588_26",
        &UV_COINCIDENCE,
        "1.stp_face1366.brep",
        "bug25588_26.png",
    );
}

#[test]
fn bug25588_27() {
    run(
        "bug25588_27",
        &UV_COINCIDENCE,
        "1.stp_face1393.brep",
        "bug25588_27.png",
    );
}

#[test]
fn bug25588_28() {
    run(
        "bug25588_28",
        &UV_COINCIDENCE,
        "1.stp_face1420.brep",
        "bug25588_28.png",
    );
}

#[test]
fn bug25588_29() {
    run(
        "bug25588_29",
        &UV_COINCIDENCE,
        "1.stp_face1447.brep",
        "bug25588_29.png",
    );
}

#[test]
fn bug25588_30() {
    run(
        "bug25588_30",
        &UV_COINCIDENCE,
        "1.stp_face1474.brep",
        "bug25588_30.png",
    );
}

#[test]
fn bug25588_31() {
    run(
        "bug25588_31",
        &UV_COINCIDENCE,
        "1.stp_face1501.brep",
        "bug25588_31.png",
    );
}

#[test]
fn bug25588_32() {
    run(
        "bug25588_32",
        &UV_COINCIDENCE,
        "1.stp_face1528.brep",
        "bug25588_32.png",
    );
}
