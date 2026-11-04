//! The hidden QA commands mutate the actual selected cursor after printing its initial count.
use super::*;
use martensite_tkernel::standard::Handle;
use martensite_tkmath::gp::Point3;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape as _;
use martensite_tkv3d::ais::{AisInteractiveContext, AisInteractiveObject, AisShape};

/// Display the three corner-defined source boxes without updates; `authored_boxes(&context)?`.
pub(super) fn authored_boxes(
    context: &AisInteractiveContext,
) -> Result<Vec<Handle<dyn AisInteractiveObject>>, Failure> {
    let mut objects = Vec::new();
    for (low, high) in [(0., 100.), (120., 300.), (320., 500.)] {
        let mut maker = BRepPrimAPIMakeBox::from_points(
            Point3::new(low, low, low),
            Point3::new(high, high, high),
        )?;
        let object = AisShape::new(maker.shape()?).cast();
        context.display(&object, false)?;
        objects.push(object);
    }
    Ok(objects)
}

/// Build the command result while removing actual current owners; `destructive_walk(&context, false)?`.
pub(super) fn destructive_walk(
    context: &AisInteractiveContext,
    local: bool,
) -> Result<String, Failure> {
    let objects = authored_boxes(context)?;
    for object in objects {
        context.add_or_remove_selected_object(&object, false)?;
    }
    let label = if local { "selected" } else { "currents" };
    let mut report = format!("\n No of {label} = {}", context.nb_selected());
    drain_selected(context, local, &mut report)?;
    context.update_current_viewer();
    Ok(report)
}

fn drain_selected(
    context: &AisInteractiveContext,
    local: bool,
    report: &mut String,
) -> Result<(), Failure> {
    context.init_selected();
    let mut count = 1;
    while context.more_selected() {
        let literal = if local { "%d" } else { "" };
        report.push_str(&format!("\n count is = {literal}{count}"));
        count += 1;
        let actual = context.selected_interactive()?;
        context.add_or_remove_selected_object(&actual, false)?;
        context.init_selected();
    }
    Ok(())
}

/// Read the source's count header; `source_count("No of currents = 3", false)`.
pub(super) fn source_count(report: &str, local: bool) -> Option<i32> {
    let label = if local {
        "No of selected"
    } else {
        "No of currents"
    };
    report.lines().find_map(|line| {
        line.trim()
            .strip_prefix(label)?
            .trim()
            .strip_prefix('=')?
            .trim()
            .parse()
            .ok()
    })
}

/// Apply both source transcript predicates; `report_selection(&report, false)`.
pub(super) fn report_selection(report: &str, local: bool) {
    print!("{report}");
    if report.split_whitespace().count() < 5 {
        println!("OCC138: Error (case 1)");
    } else if source_count(report, local) != Some(3) {
        println!("OCC138: Error (case 2)");
    }
}

const WALK_TITLES: [&str; 2] = [
    "OCC138: Viewer problem: Selection of objects on global context",
    "OCC138",
];
const WALK_IMAGES: [&str; 2] = ["bug138_1.png", "bug138_2.png"];

/// Run the complete QA selection case and final dump; `selected_walk(false)?`.
pub(crate) fn selected_walk(local: bool) -> Outcome {
    let title = WALK_TITLES[usize::from(local)];
    banner(&["========", title, "========", ""]);
    let mut run = SessionRun::new()?;
    let report = destructive_walk(run.session().context(), local)?;
    report_selection(&report, local);
    run.session().fit()?;
    run.dump(WALK_IMAGES[usize::from(local)])?;
    run.finish()
}
