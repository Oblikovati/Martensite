use crate::table::{RecordedVerdict, ScriptCase, Step};

script_case!(
    bugs_iges_bug131_1,
    ScriptCase {
        source: "tests/bugs/iges/bug131_1",
        cpu_seconds: Some(1000),
        recorded: RecordedVerdict::Ok,
        steps: &[
            Step::Run("puts", &["========"]),
            Step::Run("puts", &["Regression case"]),
            Step::Run("puts", &["========"]),
            Step::Run("puts", &[""]),
            Step::Run("igesbrep", &["@data/OCC131-1.igs", "a", "*"]),
            Step::Run("tpcompound", &["result"]),
            Step::Run("checkshape", &["result"]),
            Step::Run("checkshape", &["result", "r"]),
            Step::Run("checkview", &["result", "bug131_1"]),
        ],
    }
);
script_case!(
    bugs_iges_bug131_2,
    ScriptCase {
        source: "tests/bugs/iges/bug131_2",
        cpu_seconds: Some(1000),
        recorded: RecordedVerdict::Ok,
        steps: &[
            Step::Run("puts", &["========"]),
            Step::Run("puts", &["Regression case"]),
            Step::Run("puts", &["========"]),
            Step::Run("puts", &[""]),
            Step::Run("igesbrep", &["@data/OCC131-2.igs", "a", "*"]),
            Step::Run("tpcompound", &["result"]),
            Step::Run("checkshape", &["result"]),
            Step::Run("checkshape", &["result", "r"]),
            Step::Run("checkview", &["result", "bug131_2"]),
        ],
    }
);
script_case!(
    bugs_iges_bug131_3,
    ScriptCase {
        source: "tests/bugs/iges/bug131_3",
        cpu_seconds: Some(1000),
        recorded: RecordedVerdict::Ok,
        steps: &[
            Step::Run("puts", &["========"]),
            Step::Run("puts", &["Regression case"]),
            Step::Run("puts", &["========"]),
            Step::Run("puts", &[""]),
            Step::Run("igesbrep", &["@data/OCC131-3.igs", "a", "*"]),
            Step::Run("tpcompound", &["result"]),
            Step::Run("checkshape", &["result"]),
            Step::Run("checkshape", &["result", "r"]),
            Step::Run("checkview", &["result", "bug131_3"]),
        ],
    }
);
script_case!(
    bugs_iges_bug131_6,
    ScriptCase {
        source: "tests/bugs/iges/bug131_6",
        cpu_seconds: Some(1000),
        recorded: RecordedVerdict::Ok,
        steps: &[
            Step::Run("puts", &["========"]),
            Step::Run("puts", &["Regression case"]),
            Step::Run("puts", &["========"]),
            Step::Run("puts", &[""]),
            Step::Run("igesbrep", &["@data/OCC131-6.igs", "a", "*"]),
            Step::Run("tpcompound", &["result"]),
            Step::Run("checkshape", &["result"]),
            Step::Run("checkshape", &["result", "r"]),
            Step::Run("checkview", &["result", "bug131_6"]),
        ],
    }
);
script_case!(
    bugs_iges_bug17026_1,
    ScriptCase {
        source: "tests/bugs/iges/bug17026_1",
        cpu_seconds: Some(1000),
        recorded: RecordedVerdict::Ok,
        steps: &[
            Step::Run("puts", &["============"]),
            Step::Run("puts", &["Regression case"]),
            Step::Run("puts", &["============"]),
            Step::Run("puts", &[""]),
            Step::Run("igesread", &["@data/OCC17026-1.igs", "a", "*"]),
            Step::Run("tpcompound", &["result"]),
            Step::Run("checkprops", &["result", "-s", "9.0463e+009"]),
            Step::Run("checkshape", &["result"]),
            Step::Run(
                "checknbshapes",
                &[
                    "result",
                    "-vertex",
                    "8",
                    "-edge",
                    "12",
                    "-wire",
                    "4",
                    "-face",
                    "4",
                    "-shell",
                    "0",
                    "-solid",
                    "0",
                    "-compsolid",
                    "0",
                    "-compound",
                    "2",
                    "-shape",
                    "30"
                ]
            ),
            Step::Run("checkview", &["result", "bug17026_1"]),
        ],
    }
);
script_case!(
    bugs_iges_bug17026_2,
    ScriptCase {
        source: "tests/bugs/iges/bug17026_2",
        cpu_seconds: Some(1000),
        recorded: RecordedVerdict::Ok,
        steps: &[
            Step::Run("puts", &["============"]),
            Step::Run("puts", &["Regression case"]),
            Step::Run("puts", &["============"]),
            Step::Run("puts", &[""]),
            Step::Run("igesread", &["@data/OCC17026-2.igs", "a", "*"]),
            Step::Run("tpcompound", &["result"]),
            Step::Run("checkprops", &["result", "-s", "8.80533e+09"]),
            Step::Run("checkshape", &["result"]),
            Step::Run(
                "checknbshapes",
                &[
                    "result",
                    "-vertex",
                    "8",
                    "-edge",
                    "12",
                    "-wire",
                    "4",
                    "-face",
                    "4",
                    "-shell",
                    "0",
                    "-solid",
                    "0",
                    "-compsolid",
                    "0",
                    "-compound",
                    "2",
                    "-shape",
                    "30"
                ]
            ),
            Step::Run("checkview", &["result", "bug17026_2"]),
        ],
    }
);
script_case!(
    bugs_iges_bug25632_1,
    ScriptCase {
        source: "tests/bugs/iges/bug25632_1",
        cpu_seconds: Some(1000),
        recorded: RecordedVerdict::Ok,
        steps: &[
            Step::Run("puts", &["============"]),
            Step::Run("puts", &["IGES writer looses face orientation"]),
            Step::Run("puts", &["============"]),
            Step::Run("puts", &[""]),
            Step::Run("plane", &["p", "0", "0", "0", "0", "0", "1", "1", "0", "0"]),
            Step::Run("mkface", &["f", "p", "0", "100", "0", "100"]),
            Step::Run("smallview", &[]),
            Step::Run("donly", &["f"]),
            Step::Run("normals", &["f", "100"]),
            Step::Run("treverse", &["f"]),
            Step::Run("normals", &["f", "100"]),
            Step::Run("xwd", &["@image/bug25632_1_1.png"]),
            Step::Run("testwriteiges", &["@image/bug25632_1.igs", "f"]),
            Step::Run("testreadiges", &["@image/bug25632_1.igs", "ff"]),
            Step::Run("file_delete", &["@image/bug25632_1.igs"]),
            Step::Run("donly", &["ff"]),
            Step::Run("normals", &["ff", "100"]),
            Step::Run("xwd", &["@image/bug25632_1_2.png"]),
        ],
    }
);
script_case!(
    bugs_iges_bug25632_2,
    ScriptCase {
        source: "tests/bugs/iges/bug25632_2",
        cpu_seconds: Some(1000),
        recorded: RecordedVerdict::Ok,
        steps: &[
            Step::Run("puts", &["============"]),
            Step::Run("puts", &["IGES writer looses face orientation"]),
            Step::Run("puts", &["============"]),
            Step::Run("puts", &[""]),
            Step::Run("plane", &["p", "0", "0", "0", "0", "0", "1", "1", "0", "0"]),
            Step::Run("mkface", &["face", "p", "0", "100", "0", "100"]),
            Step::Run("smallview", &[]),
            Step::Run("donly", &["face"]),
            Step::Run("setflags", &["face", "locked"]),
            Step::Run("nurbsconvert", &["r", "face"]),
            Step::Run("mksurface", &["s", "r"]),
            Step::Run("mkface", &["f", "s", "10", "20", "30", "40"]),
            Step::Run("treverse", &["f"]),
            Step::Run("normals", &["f", "100"]),
            Step::Run("xwd", &["@image/bug25632_2_1.png"]),
            Step::Run("testwriteiges", &["@image/bug25632_2.igs", "f"]),
            Step::Run("testreadiges", &["@image/bug25632_2.igs", "ff"]),
            Step::Run("file_delete", &["@image/bug25632_2.igs"]),
            Step::Run("donly", &["ff"]),
            Step::Run("normals", &["ff", "100"]),
            Step::Run("xwd", &["@image/bug25632_2_2.png"]),
        ],
    }
);
script_case!(
    bugs_iges_bug25816,
    ScriptCase {
        source: "tests/bugs/iges/bug25816",
        cpu_seconds: Some(1000),
        recorded: RecordedVerdict::Ok,
        steps: &[
            Step::Run("circle", &["c", "0", "0", "0", "0", "0", "1", "1"]),
            Step::Run("mkedge", &["e", "c"]),
            Step::Run("compound", &["e", "s"]),
            Step::Run("newmodel", &[]),
            Step::Run("param", &["write.iges.brep.mode", "1"]),
            Step::Run("brepiges", &["s", "@image/bug25816.igs"]),
            Step::Run("igesbrep", &["@image/bug25816.igs", "a", "*"]),
            Step::Run(
                "checknbshapes",
                &[
                    "a",
                    "-vertex",
                    "1",
                    "-edge",
                    "1",
                    "-wire",
                    "0",
                    "-face",
                    "0",
                    "-shell",
                    "0",
                    "-solid",
                    "0",
                    "-compsolid",
                    "0",
                    "-compound",
                    "1",
                    "-shape",
                    "3",
                    "-t",
                    "-m",
                    "Number of sub-shapes in result after saving to IGES and reading back"
                ]
            ),
        ],
    }
);
script_case!(
    bugs_iges_bug26419_1,
    ScriptCase {
        source: "tests/bugs/iges/bug26419_1",
        cpu_seconds: Some(1000),
        recorded: RecordedVerdict::Ok,
        steps: &[
            Step::Run("puts", &["========"]),
            Step::Run("puts", &["Export of a reversed face leads to c r a s h"]),
            Step::Run("puts", &["========"]),
            Step::Run("puts", &[""]),
            Step::Run("restore", &["@data/bug26419_a-fwd.brep", "af"]),
            Step::Run("brepiges", &["af", "@image/bug26419_1.igs"]),
            Step::Run("file_delete", &["@image/bug26419_1.igs"]),
        ],
    }
);
script_case!(
    bugs_iges_bug26419_2,
    ScriptCase {
        source: "tests/bugs/iges/bug26419_2",
        cpu_seconds: Some(1000),
        recorded: RecordedVerdict::Ok,
        steps: &[
            Step::Run("puts", &["========"]),
            Step::Run("puts", &["Export of a reversed face leads to c r a s h"]),
            Step::Run("puts", &["========"]),
            Step::Run("puts", &[""]),
            Step::Run("restore", &["@data/bug26419_a-rev.brep", "ar"]),
            Step::Run("brepiges", &["ar", "@image/bug26419_2.igs"]),
            Step::Run("file_delete", &["@image/bug26419_2.igs"]),
        ],
    }
);
script_case!(
    bugs_iges_bug26931,
    ScriptCase {
        source: "tests/bugs/iges/bug26931",
        cpu_seconds: Some(1000),
        recorded: RecordedVerdict::Ok,
        steps: &[
            Step::Run("puts", &["Exporting a face throws an e x c e p t i o n"]),
            Step::Run("restore", &["@data/bug26931.brep", "face"]),
            Step::Run("brepiges", &["face", "@image/bug26931.igs"]),
            Step::Run("igesbrep", &["@image/bug26931.igs", "result", "*"]),
            Step::Run("file_delete", &["@image/bug26931.igs"]),
        ],
    }
);
script_case!(
    bugs_iges_bug26989,
    ScriptCase {
        source: "tests/bugs/iges/bug26989",
        cpu_seconds: Some(1000),
        recorded: RecordedVerdict::Ok,
        steps: &[
            Step::Run("puts", &["Export of a reversed face leads to wrong data"]),
            Step::Run("restore", &["@data/bug26989.brep", "a"]),
            Step::Run("newmodel", &[]),
            Step::Run("brepiges", &["a", "@image/bug26989.igs"]),
            Step::Run("igesbrep", &["@image/bug26989.igs", "result", "*"]),
            Step::Run("file_delete", &["@image/bug26989.igs"]),
            Step::Run("checkprops", &["result", "-s", "56.5487"]),
        ],
    }
);
