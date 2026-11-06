//! Complete transformed-sphere selection, with the original mesh and analytic branches.
use super::{
    datum_objects,
    ray_command::pick_axis,
    transformed_sphere_input::{transformed_sphere_source, triangulated_sphere_copy},
};
use crate::{
    case_view::viewer_session::DisplayRequest,
    session_run::{SessionRun, banner, check_point},
};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkservice::{aspect::AspectTypeOfMarker, graphic3d::Graphic3dCLight};
use martensite_tkv3d::std_select::StdSelectTypeOfSelectionImage;
use std::error::Error;

type Outcome = Result<(), Box<dyn Error>>;
struct SphereProbe {
    object: &'static str,
    label: &'static str,
    entity: &'static str,
    error: &'static str,
    image: &'static str,
    normals: &'static str,
}
const TRIANGULATED: SphereProbe = SphereProbe {
    object: "ss",
    label: "aPntTris_p",
    entity: "Select3DSensitiveTriangulation",
    error: "Error: triangulation should be detected",
    image: "trsf_prs_tris.png",
    normals: "trsf_selnorm_tris.png",
};
const ANALYTIC: SphereProbe = SphereProbe {
    object: "s",
    label: "aPntSph_p",
    entity: "Select3DSensitiveSphere",
    error: "Error: sphere should be detected",
    image: "trsf_prs_sph.png",
    normals: "trsf_selnorm_sph.png",
};

/// Complete original transformed sphere, without qualifying its unrun images; `sphere_trsf()`.
pub(crate) fn sphere_trsf() -> Outcome {
    banner(&[
        "=================================",
        "0032652: Visualization - Select3DSensitiveCylinder returns wrong 3D point on transformed shape",
        "Check picking of transformed sphere",
        "=================================",
    ]);
    let source = transformed_sphere_source()?;
    let mut run = SessionRun::new()?;
    source_headlight(&mut run)?;
    triangulation_branch(&mut run, &source)?;
    run.session().clear()?;
    sphere_probe(&mut run, &source, &ANALYTIC)?;
    run.finish()
}

fn source_headlight(run: &mut SessionRun) -> Outcome {
    let view = run.session().view().clone();
    let view = view.get().ok_or("view=null; expected live source View1")?;
    disable_named_headlight(&view.view_base().active_lights())?;
    Ok(())
}

fn disable_named_headlight(lights: &[Handle<Graphic3dCLight>]) -> Result<(), Failure> {
    let headlight = lights
        .iter()
        .find_map(|light| {
            light
                .get()
                .filter(|light| light.name().as_bytes() == b"headlight")
        })
        .ok_or_else(|| {
            Failure::with_message(
                "Standard_NoSuchObject",
                "light name=headlight; expected named active source light",
            )
        })?;
    headlight.set_headlight(false)
}

fn triangulation_branch(run: &mut SessionRun, source: &Shape) -> Outcome {
    run.session().clear()?;
    run.session().set_axonometric()?;
    let copy = triangulated_sphere_copy(source)?;
    sphere_probe(run, &copy, &TRIANGULATED)
}

fn sphere_probe(run: &mut SessionRun, shape: &Shape, probe: &SphereProbe) -> Outcome {
    let point = detected_sphere_point(run, shape, probe.object)?;
    datum_objects::point(run.session(), "pp", point, AspectTypeOfMarker::Plus, None)?;
    check_point(probe.label, point, [2500.54, 3499.41, 1000.6], 0.1);
    let types = run.session().picked_entity_state_types()?;
    if !types.iter().any(|name| name.contains(probe.entity)) {
        println!("{}", probe.error);
    }
    run.session().fit()?;
    run.dump(probe.image)?;
    run.selection_dump(probe.normals, StdSelectTypeOfSelectionImage::SURFACE_NORMAL)
}

fn detected_sphere_point(
    run: &mut SessionRun,
    shape: &Shape,
    name: &str,
) -> Result<[f64; 3], Box<dyn Error>> {
    run.session()
        .display(name, shape, &DisplayRequest::default().with_display_mode(1))?;
    run.session().fit()?;
    pick_axis(run, [2500., 3498., 1000.], [0., 1., 0.], "a", true)?;
    run.session().move_cursor(200, 200)?;
    Ok(run.session().detected_point()?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use martensite_tkservice::graphic3d::Graphic3dTypeOfLightSource;

    fn named_directional(name: &str) -> Handle<Graphic3dCLight> {
        let light = Graphic3dCLight::new(Graphic3dTypeOfLightSource::Directional);
        light.set_name(&name.into());
        light.set_headlight(true).unwrap();
        Handle::new(light)
    }

    #[test]
    fn named_headlight_is_resolved_by_name_instead_of_activation_index() {
        let other = named_directional("other");
        let headlight = named_directional("headlight");
        disable_named_headlight(&[other.clone(), headlight.clone()]).unwrap();
        assert!(other.is_headlight());
        assert!(!headlight.is_headlight());
        assert!(disable_named_headlight(&[other]).is_err());
    }

    #[test]
    fn both_case_probe_labels_types_and_image_names_remain_distinct() {
        assert_eq!(TRIANGULATED.label, "aPntTris_p");
        assert_eq!(ANALYTIC.label, "aPntSph_p");
        assert_eq!(TRIANGULATED.entity, "Select3DSensitiveTriangulation");
        assert_eq!(ANALYTIC.entity, "Select3DSensitiveSphere");
        assert_eq!(
            [
                TRIANGULATED.image,
                TRIANGULATED.normals,
                ANALYTIC.image,
                ANALYTIC.normals
            ],
            [
                "trsf_prs_tris.png",
                "trsf_selnorm_tris.png",
                "trsf_prs_sph.png",
                "trsf_selnorm_sph.png"
            ]
        );
    }
}
