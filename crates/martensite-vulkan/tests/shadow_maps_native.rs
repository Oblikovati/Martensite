//! Shadow maps (LightShadow.glsl, GlShadowMap) against native dumps.
//!
//! The reference scene shadow_maps/shadow_maps was recorded on llvmpipe
//! (SSE2/128) and AMD radeonsi: the opengl/data/shadows dir1, dirhead and spotlight scenes
//! (a 1x2x3 box on a 10x10 preview rectangle) without ray tracing, at 256x256, with a casting
//! directional light (world and head) and two casting spot lights, under Phong, PhongFacet,
//! Gouraud (no shadows natively) and PBR, with other biases and resolutions and shadows off.
#[path = "support/native_dump_grading.rs"]
// The per-pixel recorded-failure helpers are unused here.
#[allow(dead_code)]
mod grading;
use grading::{DumpReport, validation_options};
#[path = "support/shadow_flip_reference.rs"]
mod shadow_flip_reference;
#[cfg(test)]
#[path = "support/shadow_flip_reference_tests.rs"]
mod shadow_flip_reference_tests;
#[path = "support/shadow_residue_grade.rs"]
mod shadow_residue_grade;
#[path = "support/shadow_silhouette_reference.rs"]
mod shadow_silhouette_reference;
#[cfg(test)]
#[path = "support/shadow_texel_flip_tests.rs"]
mod shadow_texel_flip_tests;
#[path = "support/shadow_view_snap.rs"]
mod shadow_view_snap;
#[allow(dead_code)] // Only the triangle recorder serves the view-snap rule here.
#[path = "case_host/snap_pixels.rs"]
mod snap_pixels;
#[allow(dead_code)] // Only the triangle recorder serves the view-snap rule here.
#[path = "case_host/tie_edges.rs"]
mod tie_edges;
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkernel::standard::Handle;
use martensite_tkmath::gp::{Direction3, Point3};
use martensite_tkservice::aspect::{AspectNeutralWindow, AspectWindowMembers as _};
use martensite_tkservice::graphic3d::*;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape as _;
use martensite_tkv3d::ais::*;
use martensite_tkv3d::prs_mgr::*;
use martensite_tkv3d::v3d::*;
use martensite_vulkan::Graphic3dRenderDriver;
use shadow_flip_reference::{FlatFace, FlipLight, ShadowFlipScene};
use shadow_residue_grade::{ResidueEvidence, without_proven_residues};

const SIZE: (usize, usize) = (256, 256);
const DUMP_NAMES: [&str; 12] = [
    "sm-dir1-phong-bias0005",
    "sm-dir1-phong",
    "sm-dir1-phong-facet",
    "sm-dir1-gouraud",
    "sm-dir1-pbr",
    "sm-dir1-phong-res128",
    "sm-dir1-phong-off",
    "sm-dirhead-phong",
    "sm-dirhead-pbr",
    "sm-spot-two",
    "sm-spot-phong",
    "sm-spot-pbr",
];
macro_rules! dumps {
    ($device:literal) => {
        [
            include_bytes!(concat!(
                "fixtures/shadow_maps/",
                $device,
                "/sm-dir1-phong-bias0005.ppm"
            ))
            .as_slice(),
            include_bytes!(concat!(
                "fixtures/shadow_maps/",
                $device,
                "/sm-dir1-phong.ppm"
            ))
            .as_slice(),
            include_bytes!(concat!(
                "fixtures/shadow_maps/",
                $device,
                "/sm-dir1-phong-facet.ppm"
            ))
            .as_slice(),
            include_bytes!(concat!(
                "fixtures/shadow_maps/",
                $device,
                "/sm-dir1-gouraud.ppm"
            ))
            .as_slice(),
            include_bytes!(concat!(
                "fixtures/shadow_maps/",
                $device,
                "/sm-dir1-pbr.ppm"
            ))
            .as_slice(),
            include_bytes!(concat!(
                "fixtures/shadow_maps/",
                $device,
                "/sm-dir1-phong-res128.ppm"
            ))
            .as_slice(),
            include_bytes!(concat!(
                "fixtures/shadow_maps/",
                $device,
                "/sm-dir1-phong-off.ppm"
            ))
            .as_slice(),
            include_bytes!(concat!(
                "fixtures/shadow_maps/",
                $device,
                "/sm-dirhead-phong.ppm"
            ))
            .as_slice(),
            include_bytes!(concat!(
                "fixtures/shadow_maps/",
                $device,
                "/sm-dirhead-pbr.ppm"
            ))
            .as_slice(),
            include_bytes!(concat!(
                "fixtures/shadow_maps/",
                $device,
                "/sm-spot-two.ppm"
            ))
            .as_slice(),
            include_bytes!(concat!(
                "fixtures/shadow_maps/",
                $device,
                "/sm-spot-phong.ppm"
            ))
            .as_slice(),
            include_bytes!(concat!(
                "fixtures/shadow_maps/",
                $device,
                "/sm-spot-pbr.ppm"
            ))
            .as_slice(),
        ]
    };
}
const NATIVE_LLVMPIPE: [&[u8]; 12] = dumps!("llvmpipe");
const NATIVE_AMD: [&[u8]; 12] = dumps!("amd");

/// A 256x256 view of the capture.
struct ShadowScene {
    context: Handle<AisInteractiveContext>,
    view: Handle<dyn V3dViewSource>,
    viewer: Handle<V3dViewer>,
    _window: Handle<AspectNeutralWindow>,
}
impl ShadowScene {
    fn new(driver: &Handle<Graphic3dRenderDriver>) -> Self {
        let viewer = V3dViewer::new(&driver.cast()).unwrap();
        // A new view clears to black.
        viewer.set_default_background_color(Color::named(NamedColor::BLACK));
        viewer.set_default_lights().unwrap();
        viewer.set_light_on().unwrap();
        let context = AisInteractiveContext::new(&viewer).unwrap();
        let window = Handle::new(AspectNeutralWindow::default());
        window.set_size(SIZE.0 as i32, SIZE.1 as i32);
        window.set_virtual(true);
        let view = viewer.create_view().unwrap();
        view.set_window(&window.cast(), 0).unwrap();
        Self {
            context,
            view,
            viewer,
            _window: window,
        }
    }
    fn view(&self) -> &V3dView {
        self.view.get().unwrap().view_base()
    }
    /// The dump and the geometry of its shadow-flip bound.
    fn dump(&self) -> (Vec<u8>, ShadowFlipScene) {
        let render: Handle<martensite_vulkan::Graphic3dRenderView> =
            self.view.get().unwrap().view().cast();
        let render = render
            .get()
            .expect("a Vulkan render view behind the V3d view");
        let record = tie_edges::SubmittedDraws::install(render);
        self.view.redraw();
        render.set_frame_hooks(None);
        let mut scene = self.flip_scene();
        scene.view_triangles = record.triangles(SIZE.0 as f64, SIZE.1 as f64);
        (grading::view_rgb(self.view(), SIZE), scene)
    }
    /// The submitted faces (the 1x2x3 box and the 10x10 floor facing +Z), the view camera and
    /// the casting lights with the matrices their maps were rendered and sampled with.
    fn flip_scene(&self) -> ShadowFlipScene {
        let camera = self.view().camera();
        let view = camera.projection_matrix() * camera.orientation_matrix();
        let render: Handle<martensite_vulkan::Graphic3dRenderView> =
            self.view.get().unwrap().view().cast();
        let maps = render.rendered_shadow_maps();
        let mut faces = shadow_flip_reference::box_faces([0., 0., 0.], [1., 2., 3.]);
        faces.push(FlatFace {
            corners: [[-5., -5., 0.], [5., -5., 0.], [5., 5., 0.], [-5., 5., 0.]],
            normal: [0., 0., 1.],
        });
        let lights = maps.as_ref().map_or(Vec::new(), |maps| {
            let positions = self.casting_light_positions(&camera);
            let matrices = maps
                .maps()
                .iter()
                .map(|map| map.light_source_matrix().map(f64::from));
            matrices
                .zip(positions)
                .map(|(matrix, position)| FlipLight { matrix, position })
                .collect()
        });
        ShadowFlipScene {
            faces,
            view_inverse: shadow_flip_reference::invert(&view.data()),
            view_depth_range: [
                if camera.is_zero_to_one_depth() {
                    0.
                } else {
                    -1.
                },
                1.,
            ],
            size: SIZE,
            lights,
            rendered: maps.as_ref().map_or(Vec::new(), |maps| {
                let depth = maps.maps().iter().map(|map| map.depth().to_vec());
                depth
                    .map(|depth| shadow_flip_reference::RenderedMap { depth })
                    .collect()
            }),
            resolution: maps.as_ref().map_or(1, |maps| maps.resolution() as usize),
            bias: maps.map_or(0., |maps| f64::from(maps.bias())),
            // The default shaded aspect: AspectPOM_Fill, factor 1, units 0.
            slope_factor: 1.,
            view_triangles: Vec::new(),
        }
    }
    /// `occLight_Position` of each casting light in caster order: the world direction towards a
    /// directional light (a headlight's turned by the inverse view orientation), the position of
    /// a spot light.
    fn casting_light_positions(&self, camera: &Handle<Graphic3dCamera>) -> Vec<[f64; 3]> {
        let inverse = shadow_flip_reference::invert(&camera.orientation_matrix().data());
        let casters = self.view().active_lights().into_iter().filter(|light| {
            let light = light.get().unwrap();
            light.is_enabled()
                && light.to_cast_shadows()
                && light.light_type() != Graphic3dTypeOfLightSource::Ambient
        });
        casters
            .map(|light| {
                let light = light.get().unwrap();
                if light.light_type() != Graphic3dTypeOfLightSource::Directional {
                    let point = light.position();
                    return [point.x(), point.y(), point.z()];
                }
                let d = light.direction().unwrap();
                let towards = [-d.x(), -d.y(), -d.z()];
                if !light.is_headlight() {
                    return towards;
                }
                let turned = (0..3).map(|r| (0..3).map(|c| inverse[4 * c + r] * towards[c]).sum());
                let turned: Vec<f64> = turned.collect();
                let length = turned.iter().map(|v| v * v).sum::<f64>().sqrt();
                [turned[0] / length, turned[1] / length, turned[2] / length]
            })
            .collect()
    }
    /// Display the shape shaded.
    fn display(
        &self,
        shape: &martensite_tkbrep::topods::Shape,
    ) -> Handle<dyn AisInteractiveObject> {
        let object: Handle<dyn AisInteractiveObject> = AisShape::new(shape).cast();
        object.get().unwrap().set_display_mode(1);
        self.context
            .display_with_modes(&object, 1, 0, false, PrsMgrDisplayStatus::None)
            .unwrap();
        object
    }
    /// Give the object the STONE material and, optionally, a colour.
    fn stone(&self, object: &Handle<dyn AisInteractiveObject>, color: Option<NamedColor>) {
        let material = Graphic3dMaterialAspect::new(Graphic3dNameOfMaterial::Stone);
        self.context.set_material(object, &material, false).unwrap();
        if let Some(color) = color {
            self.context
                .set_color(object, Color::named(color), false)
                .unwrap();
        }
    }
    fn params(&self, change: impl Fn(&mut Graphic3dRenderingParams)) {
        self.view()
            .change_rendering_params(&mut |params| change(params));
    }
    /// Change light 0: cast shadows along `direction`, with `head` as its headlight flag.
    fn change_light_0(&self, direction: [f64; 3], head: bool) {
        let light = self.view().active_lights()[0].clone();
        let light = light.get().unwrap();
        light.set_cast_shadows(true).unwrap();
        let [x, y, z] = direction;
        light
            .set_direction(&Direction3::new(x, y, z).unwrap())
            .unwrap();
        light.set_headlight(head).unwrap();
    }
    /// Add a shadow-casting spot light at `position` along `direction`, with optional `intensity`.
    fn add_spot(&self, direction: [f64; 3], position: [f64; 3], intensity: Option<f32>) {
        let spot = V3dSpotLight::new(
            &Point3::new(0., 0., 0.),
            V3dTypeOfOrientation::XnegYnegZpos,
            Color::named(NamedColor::WHITE),
        )
        .unwrap();
        let light: Handle<Graphic3dCLight> = Handle::new(spot).cast();
        let value = light.get().unwrap();
        value.set_cast_shadows(true).unwrap();
        let [x, y, z] = direction;
        value
            .set_direction(&Direction3::new(x, y, z).unwrap())
            .unwrap();
        let [x, y, z] = position;
        value.set_position(&Point3::new(x, y, z)).unwrap();
        if let Some(intensity) = intensity {
            value.set_intensity(intensity).unwrap();
        }
        self.viewer.add_light(&light);
        self.viewer.set_light_on_one(&light).unwrap();
    }
    fn fit(&self) {
        self.view().fit_all(0.01, false).unwrap();
    }
}

fn shading(model: Graphic3dTypeOfShadingModel) -> impl Fn(&mut Graphic3dRenderingParams) {
    move |params| params.shading_model = model
}

/// The twelve dumps with the shadow maps rasterized `shift` texels off (zero natively).
fn shadow_dumps(
    driver: &Handle<Graphic3dRenderDriver>,
    shift: [f64; 2],
) -> Vec<(Vec<u8>, ShadowFlipScene)> {
    use Graphic3dTypeOfShadingModel as Model;
    let scene = ShadowScene::new(driver);
    let render: Handle<martensite_vulkan::Graphic3dRenderView> =
        scene.view.get().unwrap().view().cast();
    assert!(render.set_shadow_map_raster_shift([1., 0.]).is_err());
    render.set_shadow_map_raster_shift(shift).unwrap();
    let mut b = martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox::at_point(
        Point3::new(0., 0., 0.),
        1.,
        2.,
        3.,
    )
    .unwrap();
    let mut bb = martensite_tkprim::brep_preview_api::BRepPreviewAPIMakeBox::new();
    bb.init_at_point(Point3::new(-5., -5., 0.), 10., 10., 0.)
        .unwrap();
    scene.params(shading(Model::Phong));
    let b = scene.display(&b.shape().unwrap().clone());
    let bb = scene.display(&bb.shape().unwrap().clone());
    scene.stone(&bb, None);
    scene.fit();
    scene.change_light_0([1., 1., -1.], false);
    let mut dumps = vec![scene.dump()];
    scene.params(|params| params.shadow_map_bias = 0.01);
    dumps.push(scene.dump());
    for model in [Model::PhongFacet, Model::Gouraud, Model::Pbr] {
        scene.params(shading(model));
        dumps.push(scene.dump());
    }
    scene.params(|params| {
        params.shading_model = Model::Phong;
        params.shadow_map_resolution = 128;
    });
    dumps.push(scene.dump());
    scene.params(|params| {
        params.shadow_map_resolution = 1024;
        params.ray_tracing.is_shadow_enabled = false;
    });
    dumps.push(scene.dump());
    scene.params(|params| params.ray_tracing.is_shadow_enabled = true);
    // Perspective camera, then fit.
    scene
        .view()
        .camera()
        .set_projection_type(Graphic3dCameraProjection::Perspective);
    scene.fit();
    scene.change_light_0([-0.2, 0.2, -1.], true);
    dumps.push(scene.dump());
    scene.params(shading(Model::Pbr));
    dumps.push(scene.dump());
    scene
        .view()
        .camera()
        .set_projection_type(Graphic3dCameraProjection::Orthographic);
    scene.params(|params| {
        params.shading_model = Model::Phong;
        params.shadow_map_bias = 0.005;
    });
    scene.stone(&b, Some(NamedColor::BLUE));
    scene.stone(&bb, Some(NamedColor::RED));
    scene.fit();
    // Remove every light.
    for light in scene.view().active_lights() {
        scene.viewer.del_light(&light).unwrap();
    }
    scene.add_spot([1., 1., -1.], [-10., -10., 10.], None);
    scene.add_spot([-1., -1., -1.], [10., 10., 10.], Some(1000.));
    dumps.push(scene.dump());
    let lamp1 = scene.view().active_lights()[0].clone();
    scene.viewer.del_light(&lamp1).unwrap();
    scene.params(|params| params.shadow_map_bias = 0.001);
    dumps.push(scene.dump());
    scene.params(shading(Model::Pbr));
    dumps.push(scene.dump());
    scene.view.remove();
    assert_eq!(driver.validation_errors().unwrap(), Vec::<String>::new());
    dumps
}

fn graded_shadows() -> (String, Vec<DumpReport>) {
    let driver = Graphic3dRenderDriver::new(validation_options()).unwrap();
    let (dumps, scenes): (Vec<_>, Vec<_>) = shadow_dumps(&driver, [0., 0.]).into_iter().unzip();
    let device = driver.device_name();
    let llvmpipe = device.contains("llvmpipe");
    let controls = if llvmpipe {
        Vec::new()
    } else {
        shifted_raster_controls(&driver)
    };
    let limits = driver.device_limits().unwrap();
    let sub_texel = martensite_cmath::powi(0.5f64, limits.sub_texel_precision_bits() as i32);
    let native = if llvmpipe {
        NATIVE_LLVMPIPE
    } else {
        NATIVE_AMD
    };
    grading::write_dumps("MARTENSITE_SHADOW_MAPS_DUMP_DIR", &DUMP_NAMES, &dumps, SIZE);
    let report: Vec<DumpReport> = dumps
        .iter()
        .enumerate()
        .map(|(index, rgb)| {
            let expected = grading::ppm_rgb(native[index], SIZE);
            let report =
                grading::grade_dump(DUMP_NAMES[index], rgb, expected, None, None, &[], SIZE.0);
            // The cross-driver shadow-flip bounds apply between the AMD drivers only.
            if llvmpipe {
                return report;
            }
            let controls: Vec<Vec<u8>> = controls.iter().map(|set| set[index].clone()).collect();
            let evidence = ResidueEvidence {
                rust: rgb,
                native: expected,
                controls: &controls,
                size: SIZE,
                sub_texel,
            };
            without_proven_residues(report, &scenes[index], &evidence)
        })
        .collect();
    let counts: Vec<_> = report
        .iter()
        .map(|(name, differing, pixels)| (*name, *differing, pixels.len()))
        .collect();
    println!("{{\"shadow_maps_device\":{device:?},\"differing_unexplained\":{counts:?}}}");
    (device, report)
}

/// The dumps under every shifted-raster control of support/shadow_silhouette_reference.rs.
fn shifted_raster_controls(driver: &Handle<Graphic3dRenderDriver>) -> Vec<Vec<Vec<u8>>> {
    let shifts = shadow_silhouette_reference::CONTROL_SHIFTS.iter();
    let sets = shifts.map(|shift| shadow_dumps(driver, *shift).into_iter().map(|(rgb, _)| rgb));
    sets.map(Iterator::collect).collect()
}

/// The recorded AMD failures (fixtures/shadow_maps/amd-recorded.txt, `dump x y` lines).
fn recorded_failures() -> Vec<grading::RecordedFailure> {
    include_str!("fixtures/shadow_maps/amd-recorded.txt")
        .lines()
        .filter(|line| !line.starts_with('#'))
        .map(|line| {
            let fields: Vec<&str> = line.split(' ').collect();
            let name = DUMP_NAMES.iter().find(|name| **name == fields[0]).unwrap();
            let pixel = (fields[1].parse().unwrap(), fields[2].parse().unwrap());
            ("AMD", *name, Some(pixel))
        })
        .collect()
}

#[test]
fn shadow_maps_match_the_native_dumps() {
    let (device, report) = graded_shadows();
    let failing = grading::unrecorded_pixels(&report, &device, &recorded_failures());
    assert!(
        failing.is_empty(),
        "device={device}; expected byte-exact dumps or approved residues: {failing:?}"
    );
}

#[test]
#[ignore = "recorded failures: fixtures/shadow_maps/amd-recorded.txt lists the AMD pixels no approved residue explains"]
fn shadow_maps_recorded_failures_match_the_native_dumps() {
    let (device, report) = graded_shadows();
    let failing = grading::unrecorded_pixels(&report, &device, &[]);
    assert!(
        failing.is_empty(),
        "device={device}; expected every dump within an approved bound: {failing:?}"
    );
}
