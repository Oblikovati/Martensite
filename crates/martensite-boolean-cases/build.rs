//! Tells the case scripts' tests whether this build can hold a script's own CPU budget or time
//! limit. Those budgets are facts about an optimized build on Linux: a case that exceeds its budget
//! elsewhere is ignored there, with the reason, and runs in every optimized Linux build with nothing
//! to remember.
fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rustc-check-cfg=cfg(martensite_outside_script_budgets)");
    let unoptimized = std::env::var("OPT_LEVEL").as_deref() == Ok("0");
    let linux = std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("linux");
    if unoptimized || !linux {
        println!("cargo::rustc-cfg=martensite_outside_script_budgets");
    }
}
