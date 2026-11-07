use crate::table::{RecordedVerdict, ScriptCase, Step};

script_case!(
    de_wrapper_iges_a1,
    ScriptCase {
        source: "tests/de_wrapper/iges/A1",
        cpu_seconds: None,
        recorded: RecordedVerdict::Ok,
        steps: &[
            Step::Run("puts", &["============"]),
            Step::Run(
                "puts",
                &[
                    "DEWrapper - Implementation of a common toolkit for importing and exporting CAD files"
                ]
            ),
            Step::Run("puts", &["============"]),
            Step::Run("puts", &[""]),
            Step::Catch("Close", &["D_First"]),
            Step::Catch("Close", &["D_Second"]),
            Step::Run("ReadIges", &["D_First", "@data/bug22869_Spring.igs"]),
            Step::Run("XGetOneShape", &["S_First", "D_First"]),
            Step::Run("ReadFile", &["D_Second", "@data/bug22869_Spring.igs"]),
            Step::Run("XGetOneShape", &["S_Second", "D_Second"]),
            Step::Run("checknbshapes_like", &["S_Second", "S_First"]),
            Step::Run("Close", &["D_First"]),
            Step::Run("Close", &["D_Second"]),
        ],
    }
);
script_case!(
    de_wrapper_iges_a2,
    ScriptCase {
        source: "tests/de_wrapper/iges/A2",
        cpu_seconds: None,
        recorded: RecordedVerdict::Ok,
        steps: &[
            Step::Run("puts", &["============"]),
            Step::Run(
                "puts",
                &[
                    "DEWrapper - Implementation of a common toolkit for importing and exporting CAD files"
                ]
            ),
            Step::Run("puts", &["============"]),
            Step::Run("puts", &[""]),
            Step::Catch("Close", &["D_First"]),
            Step::Catch("Close", &["D_Second"]),
            Step::Run("ReadIges", &["D_First", "@data/bug22869_Spring.igs"]),
            Step::Run("WriteIges", &["D_First", "@image/A2.igs"]),
            Step::Run("Close", &["D_First"]),
            Step::Run("ReadIges", &["D_First", "@image/A2.igs"]),
            Step::Run("XGetOneShape", &["S_First", "D_First"]),
            Step::Run("WriteFile", &["D_First", "@image/A2.igs"]),
            Step::Run("ReadIges", &["D_Second", "@image/A2.igs"]),
            Step::Run("XGetOneShape", &["S_Second", "D_Second"]),
            Step::Run("checknbshapes_like", &["S_Second", "S_First"]),
            Step::Run("file_delete", &["@image/A2.igs"]),
            Step::Run("Close", &["D_First"]),
            Step::Run("Close", &["D_Second"]),
        ],
    }
);
script_case!(
    de_wrapper_iges_a3,
    ScriptCase {
        source: "tests/de_wrapper/iges/A3",
        cpu_seconds: None,
        recorded: RecordedVerdict::Ok,
        steps: &[
            Step::Run("puts", &["============"]),
            Step::Run(
                "puts",
                &[
                    "DEWrapper - Implementation of a common toolkit for importing and exporting CAD files"
                ]
            ),
            Step::Run("puts", &["============"]),
            Step::Run("puts", &[""]),
            Step::Catch("Close", &["D_First"]),
            Step::Catch("Close", &["D_Second"]),
            Step::Run("ReadIges", &["D_First", "@data/bug22869_Spring.igs"]),
            Step::Run("WriteIges", &["D_First", "@image/A3.igs"]),
            Step::Run("Close", &["D_First"]),
            Step::Run("ReadIges", &["D_First", "@image/A3.igs"]),
            Step::Run("XGetOneShape", &["S_First", "D_First"]),
            Step::Run("writefile", &["S_First", "@image/A3.igs"]),
            Step::Run("ReadIges", &["D_Second", "@image/A3.igs"]),
            Step::Run("XGetOneShape", &["S_Second", "D_Second"]),
            Step::Run("checknbshapes_like", &["S_Second", "S_First"]),
            Step::Run("file_delete", &["@image/A3.igs"]),
            Step::Run("Close", &["D_First"]),
            Step::Run("Close", &["D_Second"]),
        ],
    }
);
script_case!(
    de_wrapper_iges_a4,
    ScriptCase {
        source: "tests/de_wrapper/iges/A4",
        cpu_seconds: None,
        recorded: RecordedVerdict::Ok,
        steps: &[
            Step::Run("puts", &["============"]),
            Step::Run(
                "puts",
                &[
                    "DEWrapper - Implementation of a common toolkit for importing and exporting CAD files"
                ]
            ),
            Step::Run("puts", &["============"]),
            Step::Run("puts", &[""]),
            Step::Catch("Close", &["D_First"]),
            Step::Run("ReadIges", &["D_First", "@data/bug22869_Spring.igs"]),
            Step::Run("XGetOneShape", &["S_First", "D_First"]),
            Step::Run("WriteIges", &["D_First", "@image/A4.igs"]),
            Step::Run("Close", &["D_First"]),
            Step::Run("ReadIges", &["D_First", "@image/A4.igs"]),
            Step::Run("XGetOneShape", &["S_Second", "D_First"]),
            Step::Run("readfile", &["S_Third", "@data/bug22869_Spring.igs"]),
            Step::Run("writefile", &["S_Third", "@image/A4.igs"]),
            Step::Run("readfile", &["S_Fourth", "@image/A4.igs"]),
            Step::Run("checknbshapes_like", &["S_First", "S_Third"]),
            Step::Run("checknbshapes_like", &["S_Second", "S_Fourth"]),
            Step::Run("file_delete", &["@image/A4.igs"]),
            Step::Run("Close", &["D_First"]),
        ],
    }
);
