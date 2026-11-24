//! The OCCT parity pipeline with recorded commands and a temporary workspace.
use super::{
    ParityRequest, ParityToolkit, bench_executable, case_command, compile_command, cpp_sources,
    run_occt_parity,
};
use crate::context::tests::context_at;
use crate::fake_runner::RecordingRunner;
use std::path::{Path, PathBuf};

const CARGO_BENCH: &str = "cargo bench --locked --no-run --package martensite-tkernel \
                           --bench occt_parity --message-format=json";

fn request(prefix: &str) -> ParityRequest {
    ParityRequest {
        toolkit: ParityToolkit::Tkernel,
        occt_prefix: PathBuf::from(prefix),
        compiler: "c++".into(),
        cpus: Some("8-15".into()),
        rounds: 2,
        filter: None,
        frames: None,
        tolerance: 0.05,
    }
}

fn bench_messages() -> String {
    [
        r#"{"reason":"compiler-artifact","target":{"name":"martensite_tkernel","kind":["lib"]},"executable":null}"#,
        r#"{"reason":"compiler-artifact","target":{"name":"occt_parity","kind":["bench"]},"executable":"/t/occt_parity-1"}"#,
        r#"{"reason":"build-finished","success":true}"#,
    ]
    .join("\n")
}

/// A workspace with the toolkit's C++ directory, as `git rev-parse` reports it.
fn workspace() -> tempfile::TempDir {
    let root = tempfile::tempdir().unwrap();
    let cpp = root.path().join(ParityToolkit::Tkernel.cases().cpp_dir);
    std::fs::create_dir_all(&cpp).unwrap();
    for file in ["main.cpp", "collections.cpp", "parity.hxx"] {
        std::fs::write(cpp.join(file), "").unwrap();
    }
    root
}

#[test]
fn the_bench_executable_comes_from_its_compiler_artifact() {
    let path = bench_executable(&bench_messages(), "occt_parity").unwrap();
    assert_eq!(path, PathBuf::from("/t/occt_parity-1"));
}

#[test]
fn a_missing_bench_artifact_names_the_target() {
    let error = bench_executable(r#"{"reason":"build-finished"}"#, "occt_parity").unwrap_err();
    assert!(
        error
            .to_string()
            .contains("no executable for bench target occt_parity")
    );
}

#[test]
fn only_cpp_files_compile_in_name_order() {
    let root = workspace();
    let dir = root.path().join(ParityToolkit::Tkernel.cases().cpp_dir);
    let names: Vec<String> = cpp_sources(&dir)
        .unwrap()
        .iter()
        .map(|path| path.file_name().unwrap().to_string_lossy().into_owned())
        .collect();
    assert_eq!(names, ["collections.cpp", "main.cpp"]);
}

#[test]
fn the_compile_command_optimizes_and_links_the_toolkit_libraries() {
    let sources = [PathBuf::from("/w/a.cpp"), PathBuf::from("/w/b.cpp")];
    let spec = compile_command(
        Path::new("/w"),
        &request("/occt"),
        &ParityToolkit::Tkernel.cases(),
        &sources,
        Path::new("/w/out/occt_parity"),
    );
    assert_eq!(
        spec.display(),
        "c++ -O3 -DNDEBUG -std=c++17 -I/occt/include/opencascade /w/a.cpp /w/b.cpp \
         -L/occt/lib -Wl,--disable-new-dtags -Wl,-rpath,/occt/lib -lTKernel -o /w/out/occt_parity"
    );
}

#[test]
fn the_vulkan_twins_link_the_viewer_toolkits_xlib_and_glx() {
    let cases = ParityToolkit::Vulkan.cases();
    assert_eq!(cases.package, "martensite-vulkan");
    assert_eq!(
        cases.cpp_dir,
        "crates/martensite-vulkan/benches/occt_parity/occt"
    );
    let spec = compile_command(
        Path::new("/w"),
        &request("/occt"),
        &cases,
        &[PathBuf::from("/w/main.cpp")],
        Path::new("/w/out/occt_parity"),
    );
    let command = spec.display();
    for library in ["-lTKV3d", "-lTKOpenGl", "-lTKService", "-lX11", "-lGL"] {
        assert!(command.contains(library), "{command}; expected {library}");
    }
    assert!(command.ends_with("-lGL -o /w/out/occt_parity"), "{command}");
}

#[test]
fn case_programs_run_pinned_and_filtered() {
    let mut pinned = request("/occt");
    pinned.filter = Some("map".into());
    let spec = case_command(Path::new("/w"), &pinned, Path::new("/t/cases"));
    assert_eq!(
        spec.display(),
        "taskset -c 8-15 /t/cases --bench --filter map"
    );
    pinned.cpus = None;
    pinned.filter = None;
    let spec = case_command(Path::new("/w"), &pinned, Path::new("/t/cases"));
    assert_eq!(spec.display(), "/t/cases --bench");
    pinned.frames = Some(10);
    let spec = case_command(Path::new("/w"), &pinned, Path::new("/t/cases"));
    assert_eq!(spec.display(), "/t/cases --bench --frames 10");
}

#[test]
fn a_slower_case_fails_after_the_report_is_written() {
    let root = workspace();
    let occt = root.path().join("target/occt-parity/tkernel/occt_parity");
    let runner = RecordingRunner::default()
        .with_stdout(
            "git rev-parse --show-toplevel",
            &format!("{}\n", root.path().display()),
        )
        .with_stdout(CARGO_BENCH, &bench_messages())
        .with_stdout(
            &format!("taskset -c 8-15 {} --bench", occt.display()),
            "a\t10\t1\nb\t10\t2\n",
        )
        .with_stdout(
            "taskset -c 8-15 /t/occt_parity-1 --bench",
            "a\t30\t1\nb\t5\t2\n",
        );
    let context = context_at(&runner, root.path());
    let error = run_occt_parity(&context, &request("/occt")).unwrap_err();
    assert!(
        error
            .to_string()
            .contains("1 of 2 cases are slower than OCCT: a (3.00x)"),
        "{error}"
    );
    let report =
        std::fs::read_to_string(root.path().join("target/occt-parity/tkernel/report.json"));
    let report: serde_json::Value = serde_json::from_str(&report.unwrap()).unwrap();
    assert_eq!(report["cases"][1]["standing"], "faster");
    assert_eq!(report["rounds"], 2);
}

#[test]
fn each_round_runs_occt_before_rust() {
    let root = workspace();
    let occt = root.path().join("target/occt-parity/tkernel/occt_parity");
    let occt_run = format!("taskset -c 8-15 {} --bench", occt.display());
    let rust_run = "taskset -c 8-15 /t/occt_parity-1 --bench";
    let runner = RecordingRunner::default()
        .with_stdout(
            "git rev-parse --show-toplevel",
            &format!("{}\n", root.path().display()),
        )
        .with_stdout(CARGO_BENCH, &bench_messages())
        .with_stdout(&occt_run, "a\t10\t1\n")
        .with_stdout(rust_run, "a\t10\t1\n");
    let context = context_at(&runner, root.path());
    run_occt_parity(&context, &request("/occt")).unwrap();
    let runs: Vec<String> = runner
        .commands()
        .into_iter()
        .filter(|command| command.ends_with("--bench"))
        .collect();
    assert_eq!(
        runs,
        [occt_run.clone(), rust_run.into(), occt_run, rust_run.into()]
    );
}
