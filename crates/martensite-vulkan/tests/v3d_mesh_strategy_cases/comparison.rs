use super::*;
use crate::case_view::viewer_session::SurfaceAspects;
use crate::case_view::viewer_session::{DisplayRequest, ViewerSession};
use crate::dataset_shape::read_dataset_shape;
use crate::primitive_shapes::{box_at, sphere, translated};
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPICopy, BRepBuilderAPIMakeShape};
#[path = "../v3d_state_curve_cases/mesh_information.rs"]
mod mesh_information;

fn heading() {
    banner(&[
        "========",
        "0028089: Mesh - New algorithm for triangulation of 2d polygons",
        "========",
        "",
    ]);
}

/// Preserve one multi-object display before fitting and setting edge aspects.
/// Example: `display_edges(&mut session, &[("s1", &shape)])?`.
pub(super) fn display_edges(session: &mut ViewerSession, shapes: &[(&str, &Shape)]) -> Outcome {
    session.set_auto_triangulation(false);
    // Both cases display all names in one call before fitting the view.
    session.display_many(shapes, &DisplayRequest::default().with_display_mode(1))?;
    session.fit()?;
    for (name, _) in shapes {
        session.set_surface_aspects(
            name,
            &SurfaceAspects {
                draw_edges: Some(true),
                ..Default::default()
            },
        )?;
    }
    Ok(())
}

pub(super) fn terrain_pair() -> Result<[Shape; 2], Failure> {
    let first = read_dataset_shape("terrain.brep")?;
    crate::shape_mesh::remove_triangulation(&first)?;
    let mut copier = BRepBuilderAPICopy::with_shape(&first, true, false)?;
    let second = copier.shape()?.clone();
    Ok([
        translated(&first, [0.; 3])?,
        translated(&second, [8000., 0., 0.])?,
    ])
}

fn mesh_terrain(first: &Shape, second: &Shape) -> Result<(), Failure> {
    for (shape, algorithm) in [
        (first, MeshAlgoType::Watson),
        (second, MeshAlgoType::Delabella),
    ] {
        meshing::mesh_with_strategy(
            shape,
            MeshRequest::new(1000.).with_angle_degrees(50.),
            algorithm,
        )?;
        mesh_information::print_information(shape)?;
    }
    Ok(())
}

/// Compare both authored algorithm selections on independently copied terrain shapes.
/// Example: `terrain_algorithms()?`.
pub(crate) fn terrain_algorithms() -> Outcome {
    heading();
    let [first, second] = terrain_pair()?;
    mesh_terrain(&first, &second)?;
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    run.session()
        .set_standard_projection(martensite_tkv3d::v3d::V3dTypeOfOrientation::ZUP_TOP)?;
    display_edges(run.session(), &[("t1", &first), ("t2", &second)])?;
    run.finish_without_image()
}

pub(super) fn primitive_inputs() -> Result<[Shape; 4], Failure> {
    let mut shapes = [
        sphere(1.)?,
        sphere(1.)?,
        box_at([0., 2., 0.], [1., 2., 2.])?,
        box_at([2., 2., 0.], [1., 2., 2.])?,
    ];
    shapes[1] = translated(&shapes[1], [2., 0., 0.])?;
    Ok(shapes)
}

fn mesh_primitives(shapes: &[Shape; 4]) -> Result<(), Failure> {
    for (index, shape) in shapes.iter().enumerate() {
        let algorithm = if index % 2 == 0 {
            MeshAlgoType::Watson
        } else {
            MeshAlgoType::Delabella
        };
        let request = if index < 2 {
            MeshRequest::new(0.5).with_angle_degrees(50.)
        } else {
            MeshRequest::new(0.5)
        };
        meshing::mesh_with_strategy(shape, request, algorithm)?;
    }
    Ok(())
}

fn show_primitives(run: &mut SessionRun, shapes: &[Shape; 4]) -> Outcome {
    display_edges(
        run.session(),
        &[
            ("s1", &shapes[0]),
            ("s2", &shapes[1]),
            ("b1", &shapes[2]),
            ("b2", &shapes[3]),
        ],
    )?;
    Ok(())
}

/// Preserve each shape's algorithm, angular input and own shaded display mode.
/// Example: `primitive_algorithms()?`.
pub(crate) fn primitive_algorithms() -> Outcome {
    heading();
    let shapes = primitive_inputs()?;
    mesh_primitives(&shapes)?;
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    show_primitives(&mut run, &shapes)?;
    run.dump("bug28089_2.png")?;
    run.finish()
}
