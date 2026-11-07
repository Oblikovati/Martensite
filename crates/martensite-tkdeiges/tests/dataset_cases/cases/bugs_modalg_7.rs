use crate::table::{RecordedVerdict, ScriptCase, Step};

script_case!(
    bugs_modalg_7_bug23171_2,
    ScriptCase {
        source: "tests/bugs/modalg_7/bug23171_2",
        cpu_seconds: Some(1000),
        recorded: RecordedVerdict::Ok,
        steps: &[
            Step::Run("puts", &["========"]),
            Step::Run("puts", &["Regression case"]),
            Step::Run("puts", &["========"]),
            Step::Run("puts", &[""]),
            Step::Run("igesread", &["@data/bug23171_tube_surf.igs", "s", "*"]),
            Step::Run("sewing", &["r", "0.1", "s"]),
            Step::Run("checkshape", &["r"]),
            Step::Run("checknbshapes", &["r", "-vertex", "4", "-edge", "6"]),
            Step::Run("checkmaxtol", &["r", "-min_tol", "0.2"]),
        ],
    }
);
