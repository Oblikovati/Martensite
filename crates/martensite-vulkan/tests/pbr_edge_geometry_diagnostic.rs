//! Diagnostic: the submitted sphere triangles and the edge-free and edged dumps of
//! The reference scene pbr_variants/pbr_variants, for the mesh-edge distance analysis on RADV.
#[allow(dead_code)] // Only the device options and the view dump are used.
#[path = "support/native_dump_grading.rs"]
mod grading;
#[allow(dead_code)]
#[path = "support/pbr_sphere_scene.rs"]
mod pbr_sphere_scene;
#[allow(dead_code)]
#[path = "case_host/snap_pixels.rs"]
mod snap_pixels;
#[allow(dead_code)]
#[path = "case_host/tie_edges.rs"]
mod tie_edges;
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkernel::standard::Handle;
use martensite_tkservice::aspect::AspectInteriorStyle;
use martensite_tkservice::graphic3d::*;
use martensite_tkv3d::prs_mgr::*;
use martensite_vulkan::{Graphic3dRenderDriver, Graphic3dRenderView};
use pbr_sphere_scene::{PbrScene, SIZE};

fn set_edges(scene: &PbrScene, edges: bool) {
    let drawer = scene.sphere.attributes();
    drawer
        .setup_own_shading_aspect(&scene.context.default_drawer())
        .unwrap();
    let aspect = drawer.shading_aspect().aspect();
    aspect.set_draw_edges(edges);
    aspect.set_edge_color(Color::named(NamedColor::RED));
    aspect.set_interior_style(AspectInteriorStyle::Solid);
    scene.sphere.synchronize_aspects();
}

fn triangles_json(scene: &PbrScene) -> String {
    let render: Handle<Graphic3dRenderView> = scene.view.get().unwrap().view().cast();
    let render = render.get().unwrap();
    let record = tie_edges::SubmittedDraws::install(render);
    scene.view.redraw();
    render.set_frame_hooks(None);
    let rows: Vec<String> = record
        .triangles(SIZE.0 as f64, SIZE.1 as f64)
        .iter()
        .map(|t| {
            format!(
                "[[{},{}],[{},{}],[{},{}]]",
                t[0][0], t[0][1], t[1][0], t[1][1], t[2][0], t[2][1]
            )
        })
        .collect();
    format!("[{}]", rows.join(","))
}

#[test]
#[ignore = "diagnostic: set MARTENSITE_PBR_EDGE_DIAGNOSTIC_DIR"]
fn pbr_edge_geometry_diagnostic() {
    let folder = std::env::var("MARTENSITE_PBR_EDGE_DIAGNOSTIC_DIR").unwrap();
    let driver = Graphic3dRenderDriver::new(grading::validation_options()).unwrap();
    let scene = PbrScene::new(&driver);
    scene.set_material(Graphic3dNameOfMaterial::Plastified);
    scene.view().fit_all(0.01, false).unwrap();
    let mut dumps = Vec::new();
    let mut names = Vec::new();
    for (model, label) in [
        (Graphic3dTypeOfShadingModel::Pbr, "pbr"),
        (Graphic3dTypeOfShadingModel::PbrFacet, "pbrfacet"),
    ] {
        scene.set_shading(model);
        set_edges(&scene, false);
        dumps.push(scene.dump());
        names.push(format!("{label}-face"));
        set_edges(&scene, true);
        dumps.push(scene.dump());
        names.push(format!("{label}-edges"));
        std::fs::write(
            format!("{folder}/{label}-triangles.json"),
            triangles_json(&scene),
        )
        .unwrap();
    }
    let header = format!("P6\n{} {}\n255\n", SIZE.0, SIZE.1);
    for (name, rgb) in names.iter().zip(&dumps) {
        std::fs::write(
            format!("{folder}/{name}.ppm"),
            [header.as_bytes(), rgb].concat(),
        )
        .unwrap();
    }
    println!("{}", driver.device_name());
    assert_eq!(driver.validation_errors().unwrap(), Vec::<String>::new());
}
