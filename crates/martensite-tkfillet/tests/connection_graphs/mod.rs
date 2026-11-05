//! Fixed native valid graphs call the production API; no interpreter or native library runs.
mod catalog;
mod roles;
mod spec;
mod variants;
mod witnesses;
use martensite_tkernel::storage::format_general;
use martensite_tkfillet::chfi3d::define_connect_type;
use spec::{CaseRow, FixtureSpec};
type PublicConnectionEntry = fn(
    &martensite_tkbrep::topods::Edge,
    &martensite_tkbrep::topods::Face,
    &martensite_tkbrep::topods::Face,
    f64,
    bool,
) -> Result<
    martensite_tkfillet::chfi_ds::TypeOfConcavity,
    martensite_tkernel::standard::Failure,
>;

fn observed_result(spec: &FixtureSpec, roles: &roles::GraphRoles, row: &CaseRow) -> i32 {
    let inputs = variants::inputs(roles, row);
    let _: &(dyn Send + Sync) = &inputs;
    let result = define_connect_type(
        &inputs.edge,
        &inputs.first,
        &inputs.second,
        row.tolerance,
        row.correct,
    )
    .unwrap();
    let actual = result as i32;
    assert_eq!(
        actual, row.result,
        "{} flags={:?} correction={} sine={}: expected independent native result",
        spec.name, row.flags, row.correct, row.tolerance
    );
    actual
}

fn emit_row(spec: &FixtureSpec, row: &CaseRow, result: i32) {
    println!(
        "{{\"case\":{:?},\"variant\":[{},{},{},{},{}],\"correct\":{},\"sine_tolerance\":{},\"result\":{result}}}",
        spec.name,
        u8::from(row.flags[0]),
        u8::from(row.flags[1]),
        u8::from(row.flags[2]),
        u8::from(row.flags[3]),
        u8::from(row.flags[4]),
        u8::from(row.correct),
        format_general(row.tolerance, 17)
    );
}

fn run_fixture(spec: &FixtureSpec) -> usize {
    let roles = roles::read(spec);
    let _: &(dyn Send + Sync) = &roles;
    witnesses::check(spec, &roles);
    let before = roles::serialized(&roles.root);
    let rows: Vec<CaseRow> = spec.expected.lines().map(CaseRow::parse).collect();
    for row in &rows {
        let _: &(dyn Send + Sync) = row;
        emit_row(spec, row, observed_result(spec, &roles, row));
    }
    assert_eq!(
        roles::serialized(&roles.root),
        before,
        "{}: classifier changed authored inputs",
        spec.name
    );
    rows.len()
}

/// Exercise all22 fixed graphs and1428 actual production calls; `run() == 1428`.
pub(super) fn run() -> usize {
    let _: PublicConnectionEntry = martensite_tkfillet::chfi3d::define_connect_type;
    let mut checked = 0;
    let mut names = std::collections::BTreeSet::new();
    for fixture in catalog::fixtures() {
        assert!(
            names.insert(fixture.name),
            "duplicate authored case {:?}",
            fixture.name
        );
        let _: &(dyn Send + Sync) = fixture;
        let _: &(dyn Send + Sync) = &fixture.first;
        checked += run_fixture(fixture);
    }
    assert_eq!(names.len(), 22);
    checked
}
