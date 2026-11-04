//! Real selection calls, process heap measurements and CPU timers keep the original work observable.
mod geometry;
use crate::{
    case_view::viewer_session::{DisplayRequest, ViewerSession},
    session_run::{SessionRun, banner},
};
use martensite_tkernel::osd::{MemCounter, MemInfo, Timer};
type Outcome = Result<(), Box<dyn std::error::Error>>;

fn heap_bytes() -> usize {
    let mut info = MemInfo::new(false);
    info.set_active(false);
    info.set_counter_active(MemCounter::HeapUsage, true);
    info.update();
    info.value(MemCounter::HeapUsage)
}

fn report_heap(label: &str, bytes: usize) {
    println!(
        "{label} mem: {} MiB ({})",
        bytes as f64 / (1024. * 1024.),
        bytes as f64
    );
}

fn transformed_view(session: &ViewerSession) -> Outcome {
    println!("\nApplying transformations...");
    session.view().view_base().translate(1., 0., 0., true)?;
    session.rotate(100., 100., 100.)?;
    Ok(())
}

fn timed_cursor(session: &mut ViewerSession, name: &str, x: i32, y: i32) -> Outcome {
    let mut timer = Timer::new(false);
    timer.restart();
    session.move_cursor(x, y)?;
    timer.stop();
    println!("COUNTER {name}: {}", timer.show_values().3);
    Ok(())
}

/// Run the actual hundred-sphere grid, selection and camera changes; `sphere_grid()?`.
pub(crate) fn sphere_grid() -> Outcome {
    banner(&["============", "OCC24623_1", "============", ""]);
    let shapes = geometry::spheres()?;
    println!("Total spheres number:{}", shapes.len());
    let mut run = SessionRun::new()?;
    report_heap("Initial", heap_bytes());
    run.session().set_display_mode(1)?;
    let _source_second_initial_heap = heap_bytes();
    display_grid(run.session(), &shapes)?;
    sphere_selection(&mut run)?;
    run.dump("bug24623_1.png")?;
    run.finish()
}

fn display_grid(
    session: &mut ViewerSession,
    shapes: &[(String, martensite_tkbrep::topods::Shape)],
) -> Outcome {
    let named: Vec<_> = shapes
        .iter()
        .map(|(name, shape)| (name.as_str(), shape))
        .collect();
    session.display_many(&named, &DisplayRequest::default())?;
    session.fit()?;
    Ok(())
}

fn sphere_selection(run: &mut SessionRun) -> Outcome {
    println!("Selection of spheres without BVH trees built:");
    run.session().move_cursor(224, 269)?;
    transformed_view(run.session())?;
    println!("\nSelection time with transformations applied without BVH built:");
    timed_cursor(run.session(), "vmoveto", 102, 224)?;
    report_heap("Selection", heap_bytes());
    run.session().check_color(115, 221, [0., 1., 1.])?;
    Ok(())
}

/// Run the actual thousand-point spiral and repeated neutral selection; `spiral()?`.
pub(crate) fn spiral() -> Outcome {
    banner(&["============", "OCC24623_2", "============", ""]);
    let points = geometry::spiral_points();
    let mut run = SessionRun::new()?;
    report_heap("Initial", heap_bytes());
    let wire = geometry::spiral_wire(&points)?;
    run.session()
        .display("p", &wire, &DisplayRequest::default())?;
    run.session().fit()?;
    for (x, y) in [(223, 236), (0, 0), (223, 236)] {
        run.session().move_cursor(x, y)?;
    }
    transformed_view(run.session())?;
    spiral_selection(&mut run)?;
    run.dump("bug24623_2.png")?;
    run.finish()
}

fn spiral_selection(run: &mut SessionRun) -> Outcome {
    println!("\nSelection time after the transformations:");
    timed_cursor(run.session(), "vmoveto_1", 115, 160)?;
    run.session().move_cursor(0, 0)?;
    timed_cursor(run.session(), "vmoveto_2", 115, 160)?;
    report_heap("Selection", heap_bytes());
    run.session().check_color(131, 195, [0., 1., 1.])?;
    Ok(())
}
