//! Vertex draw mode assignments and queries preserve the complete seven-image program.
use super::Outcome;
use crate::{
    case_view::viewer_session::{DisplayRequest, ViewerSession},
    object_hlr_commands,
    primitive_shapes::{box_sized, compound_of},
    session_run::{SessionRun, banner},
};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::Failure;
use martensite_tkmath::gp::Point3;
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeVertex};
use martensite_tkv3d::{
    prs_mgr::PrsMgrPresentableObjectMembers, prs3d::Prs3dVertexDrawMode as Mode,
};

pub(super) fn vertex_shapes() -> Result<(Shape, Shape), Failure> {
    let vertex = |x, y, z| BRepBuilderAPIMakeVertex::new(Point3::new(x, y, z))?.vertex();
    let v1 = vertex(0., -1., 0.)?;
    let v2 = vertex(0., -1., 5.)?;
    let v3 = vertex(-1., -1., 0.)?;
    let v4 = vertex(-1., -1., 5.)?;
    let edge = BRepBuilderAPIMakeEdge::from_vertices(&v3, &v4)?.edge()?;
    let first = box_sized([1., 2., 3.])?;
    let second = box_sized([3., 4., 2.])?;
    let compound = compound_of(&[
        v1.as_ref().clone(),
        v2.as_ref().clone(),
        v3.as_ref().clone(),
        v4.as_ref().clone(),
        edge.as_ref().clone(),
        first,
    ])?;
    Ok((compound, second))
}
pub(super) fn assign(session: &ViewerSession, name: &str, mode: Mode) -> Result<(), Failure> {
    let shape = object_hlr_commands::shape(session, name)?;
    session.update_view(|_| {
        shape.attributes().set_vertex_draw_mode(mode);
        session.context().redisplay(&shape.cast(), false, false)?;
        Ok(())
    })
}
fn query(session: &ViewerSession, name: Option<&str>) -> Result<(), Failure> {
    let mode = match name {
        Some(n) => object_hlr_commands::shape(session, n)?
            .attributes()
            .vertex_draw_mode(),
        None => session.context().default_drawer().vertex_draw_mode(),
    };
    let prefix = name.map_or("Default", |_| "Object's");
    println!(
        "{prefix} vertex draw mode: '{}'",
        if mode == Mode::Isolated {
            "isolated"
        } else {
            "all"
        }
    );
    Ok(())
}
/// Seven authored captures retain inherited versus own drawer modes; `vertex_draw_modes()?`.
pub(crate) fn vertex_draw_modes() -> Outcome {
    banner(&[
        "============",
        "CR25099: Test vertex draw modes",
        "============",
        "",
    ]);
    let mut run = SessionRun::new()?;
    let (compound, second) = vertex_shapes()?;
    run.session()
        .display("c", &compound, &DisplayRequest::default())?;
    run.session().fit()?;
    query(run.session(), None)?;
    run.dump("bug25099_default_wf_isolated.png")?;
    run.session().set_object_display_mode("c", 1)?;
    run.dump("bug25099_default_shading_isolated.png")?;
    all_vertices(&mut run, &compound, &second)?;
    run.finish()
}
fn all_vertices(run: &mut SessionRun, compound: &Shape, second: &Shape) -> Outcome {
    run.session().clear()?;
    set_default_all(run.session());
    run.session()
        .display("c", compound, &DisplayRequest::default())?;
    query(run.session(), None)?;
    run.dump("bug25099_default_wf_all1.png")?;
    run.session()
        .display("b2", second, &DisplayRequest::default())?;
    run.session().fit()?;
    query(run.session(), Some("b2"))?;
    run.dump("bug25099_default_wf_all2.png")?;
    run.session().set_display_mode(1)?;
    run.dump("bug25099_default_shading_all.png")?;
    individual_modes(run)
}
fn individual_modes(run: &mut SessionRun) -> Outcome {
    assign(run.session(), "b2", Mode::Isolated)?;
    query(run.session(), Some("b2"))?;
    run.dump("bug25099_individual.png")?;
    assign(run.session(), "b2", Mode::Inherited)?;
    query(run.session(), Some("b2"))?;
    run.dump("bug25099_inherited.png")?;
    Ok(())
}

fn set_default_all(session: &ViewerSession) {
    session
        .context()
        .default_drawer()
        .set_vertex_draw_mode(Mode::All);
    println!("Setting the default vertex draw mode and updating all displayed objects...");
    session.view().redraw();
}
