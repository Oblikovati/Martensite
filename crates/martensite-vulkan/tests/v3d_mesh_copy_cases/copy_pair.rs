use super::*;
use crate::case_view::viewer_session::DisplayRequest;
use crate::dataset_shape::read_dataset_shape;

fn remeshed_copy_pair() -> Result<(Shape, TriangulationSummary), Box<dyn std::error::Error>> {
    let original = read_dataset_shape("bug25055.brep")?;
    let copy = copied_shape(&original, false)?;
    remove_triangulation(&original)?;
    mesh_shape(&original, &MeshRequest::new(0.1))?;
    mesh_information::print_information(&original)?;
    let baseline = TriangulationSummary::of(&original)?;
    remove_triangulation(&copy)?;
    mesh_shape(&copy, &MeshRequest::new(0.1))?;
    Ok((copy, baseline))
}

const COPY_BANNER: &[&str] = &[
    "##################################################",
    "0025055: Incorrect shape copying in BRepBuilderAPI_Copy",
    "##################################################",
];

/// Reproduce `tcopy` followed by independent cleaning and meshing of both shapes.
/// Example: `independently_remeshed_copy()?`.
pub(crate) fn independently_remeshed_copy() -> Outcome {
    banner(COPY_BANNER);
    let mut run = SessionRun::new()?;
    run.session().set_auto_triangulation(false);
    let (copy, baseline) = remeshed_copy_pair()?;
    run.session().clear()?;
    display_copied_mesh(run.session(), &copy)?;
    run.session().fit()?;
    run.dump("bug25055.png")?;
    if !crate::mesh_topology_check::check_mesh_topology(&copy)?.is_clean() {
        println!("Error : Result tricheck is BAD");
    }
    check_triangulation(&copy, &count_expectation(&baseline))?;
    run.finish()
}

/// Apply the authored per-object shaded display without changing context defaults.
/// Example: `display_copied_mesh(&mut session, &copy)?`.
pub(super) fn display_copied_mesh(
    session: &mut crate::case_view::viewer_session::ViewerSession,
    copy: &Shape,
) -> Result<(), Failure> {
    session.display("b", copy, &DisplayRequest::default().with_display_mode(1))
}
