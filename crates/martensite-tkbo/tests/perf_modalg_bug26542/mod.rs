//! Preserve actual first-operation side effects before copying the original edges.
mod checks;
mod operations;

/// Replay every original command without a renderer; `run()?` stays under CPU500.
pub(super) fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("============\n0026452: Infinite loop in Edge/Edge intersection\n============\n");
    let (first, second) = operations::case_edges();
    let mut commands = operations::FuseSession::new();
    let first_result = commands.timed_fuse(&first, &second, "COUNTER1")?;
    // Copy the actual post-Fuse bindings; a second restore would lose shared-input effects.
    let first_copy = operations::copy_binding(&first)?;
    let second_copy = operations::copy_binding(&second)?;
    let second_result = commands.timed_fuse(&first_copy, &second_copy, "COUNTER2")?;
    checks::case_checks(&first_result, &second_result);
    // Native -b has no V3d view; perf/end queries it, so its conditional screenshot is inactive.
    println!("TEST COMPLETED");
    Ok(())
}
