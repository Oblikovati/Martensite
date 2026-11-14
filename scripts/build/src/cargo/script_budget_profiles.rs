//! Match local script-budget tests to the existing package CI policy (#447).
use super::*;
const SCRIPT_PACKAGES: [&str; 11] = [
    "martensite-tkbo",
    "martensite-boolean-cases",
    "martensite-tkbool",
    "martensite-tkfillet",
    "martensite-tkfeat",
    "martensite-tkoffset",
    "martensite-tkdestl",
    "martensite-tkmeshvs",
    "martensite-tkdestep",
    "martensite-tkdeiges",
    "martensite-tkdeobj",
];
#[derive(Clone, Copy)]
pub(super) enum Optimization {
    Inherited,
    ScriptBudget,
}
impl Optimization {
    pub(super) fn apply(self, spec: CommandSpec) -> CommandSpec {
        match self {
            Self::Inherited => spec,
            Self::ScriptBudget => spec
                .environment("CARGO_PROFILE_DEV_OPT_LEVEL", Some("1"))
                .environment("CARGO_PROFILE_TEST_OPT_LEVEL", Some("1")),
        }
    }
}
pub(super) struct TestGroup {
    pub selection: PackageSelection,
    pub optimization: Optimization,
}
pub(super) fn groups(
    selection: &PackageSelection,
    runner: &dyn CommandRunner,
    root: &Path,
) -> BuildResult<Vec<TestGroup>> {
    let names = match selection {
        PackageSelection::Packages(names) => names.clone(),
        _ => crate::workspace::CargoWorkspace::load(runner, root)?
            .names()
            .into_iter()
            .filter(|name| selection.includes_package(name))
            .collect(),
    };
    Ok(explicit(&names))
}

fn explicit(names: &[String]) -> Vec<TestGroup> {
    let (script, ordinary): (Vec<_>, Vec<_>) = names
        .iter()
        .cloned()
        .partition(|name| SCRIPT_PACKAGES.contains(&name.as_str()));
    batches(PackageSelection::Packages(ordinary), script)
}
fn batches(ordinary: PackageSelection, script: Vec<String>) -> Vec<TestGroup> {
    let groups = [
        TestGroup {
            selection: ordinary,
            optimization: Optimization::Inherited,
        },
        TestGroup {
            selection: PackageSelection::Packages(script),
            optimization: Optimization::ScriptBudget,
        },
    ];
    groups
        .into_iter()
        .filter(|group| {
            !matches!(&group.selection,
        PackageSelection::Packages(names) if names.is_empty())
        })
        .collect()
}
