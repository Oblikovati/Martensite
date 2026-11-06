//! Surface-texture generation modes Sphere and Sprite against native dumps, on the programmable
//! path and under the fixed-function option.
//!
//! the recorded sphere_texgen capture ran on llvmpipe
//! (SSE2/128) and AMD radeonsi, with `ffpEnable` off (programmable) and on (fixed). A unit UV
//! sphere (24 slices, 12 stacks, nodal normals and UVs) is drawn unlit and white with a 64x64
//! bilinear ramp texture, rotated and translated, under a perspective camera. Dumps: sprite (no
//! generation mode set yet), sphere, sprite-after-sphere and manual. The programmable pipeline
//! ignores GenMode, so every dump there equals the manual one; the fixed-function pipeline
//! generates `GL_SPHERE_MAP` coordinates for Sphere.
#[path = "support/native_dump_grading.rs"]
#[allow(dead_code)]
mod grading;
#[path = "support/reference_cpu.rs"]
mod reference_cpu;
#[path = "support/sphere_map_rim.rs"]
mod sphere_map_rim;
#[path = "support/uv_seam.rs"]
mod uv_seam;
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkernel::standard::Handle;
use martensite_tkmath::gp::{Axis1, Direction3, Point3, Transform3, Vector3};
use martensite_tkservice::aspect::{
    AspectInteriorStyle, AspectNeutralWindow, AspectWindowMembers as _,
};
use martensite_tkservice::graphic3d::*;
use martensite_tkservice::image::{ImageFormat, ImagePixMap};
use martensite_tkv3d::v3d::*;
use martensite_vulkan::{
    Graphic3dRenderDriver, RenderLightingPipeline, RenderMeshEdgePipeline,
    RenderTextureCoordinatePipeline,
};

const SIZE: (usize, usize) = (200, 200);
const DUMP_NAMES: [&str; 4] = ["sprite", "sphere", "sprite-after-sphere", "manual"];
/// The native dump each Rust dump must equal, per driver and path: programmable dumps all equal
/// the manual dump (GenMode is ignored); under ffpEnable, sprite-after-sphere equals sprite (the
/// Sphere mode does not carry over).
macro_rules! native_dumps {
    ($device:literal) => {{
        let fixture = |path: &str| -> &'static [u8] {
            match path {
                "programmable/manual" => include_bytes!(concat!(
                    "fixtures/sphere_texgen/",
                    $device,
                    "/programmable/manual.ppm"
                )),
                "fixed/sprite" => include_bytes!(concat!(
                    "fixtures/sphere_texgen/",
                    $device,
                    "/fixed/sprite.ppm"
                )),
                "fixed/sphere" => include_bytes!(concat!(
                    "fixtures/sphere_texgen/",
                    $device,
                    "/fixed/sphere.ppm"
                )),
                _ => include_bytes!(concat!(
                    "fixtures/sphere_texgen/",
                    $device,
                    "/fixed/manual.ppm"
                )),
            }
        };
        (
            [fixture("programmable/manual"); 4],
            [
                fixture("fixed/sprite"),
                fixture("fixed/sphere"),
                fixture("fixed/sprite"),
                fixture("fixed/manual"),
            ],
        )
    }};
}

/// The probe's 64x64 ramp (red = 4 * column, green = 4 * row, blue 96) in bottom-up rows.
fn ramp_image() -> Handle<ImagePixMap> {
    let logical: Vec<u8> = (0..64_u32)
        .flat_map(|row| {
            (0..64_u32).flat_map(move |column| [(column * 4) as u8, (row * 4) as u8, 96])
        })
        .collect();
    let memory: Vec<u8> = logical.chunks(192).rev().flatten().copied().collect();
    let image = Handle::new(ImagePixMap::default());
    assert!(image.init_wrapper(ImageFormat::Rgb, Some(memory), 64, 64, 192));
    image.set_top_down(false);
    image
}

/// The probe's unit UV sphere; positions and normals are the gpDir of each node.
fn uv_sphere() -> Handle<Graphic3dArrayOfPrimitives> {
    let (slices, stacks) = (24, 12);
    let nodes = (slices + 1) * (stacks + 1);
    let flags = GRAPHIC3D_ARRAY_FLAGS_VERTEX_NORMAL | GRAPHIC3D_ARRAY_FLAGS_VERTEX_TEXEL;
    let kind = Graphic3dTypeOfPrimitiveArray::Triangles;
    let array = Graphic3dArrayOfPrimitives::new(kind, nodes, 0, slices * stacks * 6, flags);
    let array = Handle::new(array.unwrap());
    for stack in 0..=stacks {
        let v = f64::from(stack) / f64::from(stacks);
        let phi = std::f64::consts::PI * v;
        for slice in 0..=slices {
            let u = f64::from(slice) / f64::from(slices);
            let theta = 2. * std::f64::consts::PI * u;
            let d = Direction3::new(
                martensite_cmath::sin(phi) * martensite_cmath::cos(theta),
                martensite_cmath::sin(phi) * martensite_cmath::sin(theta),
                martensite_cmath::cos(phi),
            );
            let d = d.unwrap();
            let xyz = [d.x(), d.y(), d.z()];
            array
                .add_vertex_normal_texel_f64(xyz, xyz, [u, 1. - v])
                .unwrap();
        }
    }
    for stack in 0..stacks {
        for slice in 0..slices {
            let first = stack * (slices + 1) + slice + 1;
            let below = first + slices + 1;
            array.add_edges3(first, below, first + 1).unwrap();
            array.add_edges3(first + 1, below, below + 1).unwrap();
        }
    }
    array
}

fn sphere_placement() -> Transform3 {
    let axis = Axis1::new(Point3::default(), Direction3::new(1., 1., 0.).unwrap());
    let mut transform = Transform3::default();
    transform.set_rotation(axis, 0.7).unwrap();
    transform.set_translation_part(Vector3::new(0.2, 0.1, -0.3));
    transform
}

fn sphere_transform() -> Handle<martensite_tkmath::toploc::Datum3> {
    Handle::new(martensite_tkmath::toploc::Datum3::new(sphere_placement()))
}

/// The probe's sphere node at (stack, slice) placed by the structure transformation, and its UV.
fn placed_node(stack: u32, slice: u32) -> ([f64; 3], [f64; 2]) {
    let (u, v) = (f64::from(slice) / 24., f64::from(stack) / 12.);
    let (phi, theta) = (std::f64::consts::PI * v, 2. * std::f64::consts::PI * u);
    let d = Point3::new(
        martensite_cmath::sin(phi) * martensite_cmath::cos(theta),
        martensite_cmath::sin(phi) * martensite_cmath::sin(theta),
        martensite_cmath::cos(phi),
    );
    let placed = d.transformed(sphere_placement());
    ([placed.x(), placed.y(), placed.z()], [u, 1. - v])
}

/// The probe's sphere as drawn in `view`, for the UV-seam rule: the u = 0/1 meridian and the
/// poles are its seam vertices.
fn seam_mesh(view: &V3dView) -> uv_seam::SeamMesh {
    let camera = view.camera();
    let eye = camera.eye();
    let nodes: Vec<([f64; 3], [f64; 2])> = (0..=12)
        .flat_map(|stack| (0..=24).map(move |slice| placed_node(stack, slice)))
        .collect();
    let pixel = |[x, y, z]: [f64; 3]| {
        let ndc = camera.project(&Point3::new(x, y, z));
        let (w, h) = (SIZE.0 as f64, SIZE.1 as f64);
        [(ndc.x() + 1.) * 0.5 * w, h - (ndc.y() + 1.) * 0.5 * h]
    };
    let quads = (0..12).flat_map(|stack| (0..24).map(move |slice| stack * 25 + slice));
    let triangles = quads
        .flat_map(|first| {
            [
                [first, first + 25, first + 1],
                [first + 1, first + 25, first + 26],
            ]
        })
        .collect();
    uv_seam::SeamMesh {
        pixels: nodes.iter().map(|node| pixel(node.0)).collect(),
        positions: nodes.iter().map(|node| node.0).collect(),
        uvs: nodes.iter().map(|node| node.1).collect(),
        triangles,
        eye: [eye.x(), eye.y(), eye.z()],
    }
}

fn fix_camera(view: &V3dView) {
    view.set_auto_z_fit_mode(false, 1.).unwrap();
    let camera = view.camera();
    camera.set_projection_type(Graphic3dCameraProjection::Perspective);
    camera.set_eye(&Point3::new(0.3, -0.4, 4.)).unwrap();
    camera.set_center(&Point3::new(0.1, 0.05, 0.)).unwrap();
    camera.set_up(&Direction3::new(0., 1., 0.).unwrap());
    camera.set_fov_y(45.);
    camera.set_z_range(1., 10.).unwrap();
}

/// The probe's scene on `driver`: the view, the sphere's group and its unlit white aspect.
struct SphereScene {
    view: Handle<dyn V3dViewSource>,
    group: Handle<dyn Graphic3dGroup>,
    aspect: Handle<Graphic3dAspects>,
    _viewer: Handle<V3dViewer>,
    _window: Handle<AspectNeutralWindow>,
}
impl SphereScene {
    fn new(driver: &Handle<Graphic3dRenderDriver>, texture: &Handle<Graphic3dTexture2D>) -> Self {
        let viewer = V3dViewer::new(&driver.cast()).unwrap();
        viewer.set_default_background_color(Color::named(NamedColor::BLACK));
        let window = Handle::new(AspectNeutralWindow::default());
        window.set_size(SIZE.0 as i32, SIZE.1 as i32);
        window.set_virtual(true);
        let view = viewer.create_view().unwrap();
        view.view_base().set_window(&window.cast(), 0).unwrap();
        fix_camera(view.view_base());
        let aspect = Handle::new(Graphic3dAspects::default());
        aspect.set_interior_style(AspectInteriorStyle::Solid);
        aspect.set_shading_model(Graphic3dTypeOfShadingModel::Unlit);
        aspect.set_interior_color(Color::named(NamedColor::WHITE));
        aspect.set_texture_map(texture.cast());
        aspect.set_texture_map_on(true);
        let structure = Graphic3dStructure::new(&viewer.structure_manager(), &Handle::default());
        let structure = structure.unwrap();
        let group = structure.new_group().unwrap();
        group.set_group_primitives_aspect(&aspect.cast());
        group.add_primitive_array(&uv_sphere(), true);
        structure.set_transformation(&sphere_transform());
        structure.display();
        Self {
            view,
            group,
            aspect,
            _viewer: viewer,
            _window: window,
        }
    }
    /// Set `mode` on `texture`, redraw and dump.
    fn dump(
        &self,
        texture: &Handle<Graphic3dTexture2D>,
        mode: Graphic3dTypeOfTextureMode,
    ) -> Vec<u8> {
        let [s, t] = [[1., 0., 0., 0.], [0., 1., 0., 0.]]
            .map(martensite_tkernel::ncollection::Vec4::from_array);
        texture.get_params().set_gen_mode(mode, s, t);
        self.group.synchronize_aspects();
        self.view.redraw();
        grading::view_rgb(self.view.view_base(), SIZE)
    }
}

/// A bilinear texture of `image`, as the probe's textures.
fn bilinear_texture(image: Handle<ImagePixMap>) -> Handle<Graphic3dTexture2D> {
    let texture = Handle::new(Graphic3dTexture2D::from_pix_map(image));
    texture
        .get_params()
        .set_filter(Graphic3dTypeOfTextureFilter::Bilinear);
    texture
}

/// Run the probe's sequence on `driver`; the dumps in `DUMP_NAMES` order and the projected seam.
fn texgen_dumps(
    driver: &Handle<Graphic3dRenderDriver>,
) -> (Vec<Vec<u8>>, Vec<uv_seam::SeamSegment>) {
    let texture = bilinear_texture(ramp_image());
    let scene = SphereScene::new(driver, &texture);
    let seam = uv_seam::seam_segments(&seam_mesh(scene.view.view_base()));
    let modes = [
        Graphic3dTypeOfTextureMode::Sprite,
        Graphic3dTypeOfTextureMode::Sphere,
        Graphic3dTypeOfTextureMode::Sprite,
        Graphic3dTypeOfTextureMode::Manual,
    ];
    let dumps = modes.map(|mode| scene.dump(&texture, mode));
    scene.view.remove();
    assert_eq!(driver.validation_errors().unwrap(), Vec::<String>::new());
    (dumps.to_vec(), seam)
}

/// The probe's uniform 4x4 RGBA texture of (200, 100, 50, 128).
fn translucent_image() -> Handle<ImagePixMap> {
    let image = Handle::new(ImagePixMap::default());
    let texels = [200, 100, 50, 128].repeat(16);
    assert!(image.init_wrapper(ImageFormat::Rgba, Some(texels), 4, 4, 16));
    image
}

/// The fixed run's sprite-rgba (opaque aspect) and sprite-rgba-blend (Graphic3dAlphaMode_Blend)
/// dumps of the translucent texture in Sprite mode.
fn rgba_sprite_dumps(driver: &Handle<Graphic3dRenderDriver>) -> [Vec<u8>; 2] {
    let texture = bilinear_texture(translucent_image());
    let scene = SphereScene::new(driver, &texture);
    let opaque = scene.dump(&texture, Graphic3dTypeOfTextureMode::Sprite);
    scene.aspect.set_alpha_mode(Graphic3dAlphaMode::Blend, 0.5);
    let blended = scene.dump(&texture, Graphic3dTypeOfTextureMode::Sprite);
    scene.view.remove();
    assert_eq!(driver.validation_errors().unwrap(), Vec::<String>::new());
    [opaque, blended]
}

/// GL_REPLACE of a Sprite-mode RGBA texture under the fixed-function option: the texel colour
/// and alpha (the recorded sphere_texgen capture, fixed run): (200, 100, 50) opaque; with
/// blending both faces of the sphere blend the texel by its alpha, (150, 75, 37/38).
#[test]
fn fixed_function_sprites_replace_with_the_rgba_texel() {
    let driver = fixed_driver();
    let dumps = rgba_sprite_dumps(&driver);
    let names = ["sprite-rgba", "sprite-rgba-blend"];
    grading::write_dumps("MARTENSITE_SPRITE_RGBA_DUMP_DIR", &names, &dumps, SIZE);
    let device = driver.device_name();
    let native: [&[u8]; 2] = match device.contains("llvmpipe") {
        true => [
            include_bytes!("fixtures/sphere_texgen/llvmpipe/fixed/sprite-rgba.ppm"),
            include_bytes!("fixtures/sphere_texgen/llvmpipe/fixed/sprite-rgba-blend.ppm"),
        ],
        false => [
            include_bytes!("fixtures/sphere_texgen/amd/fixed/sprite-rgba.ppm"),
            include_bytes!("fixtures/sphere_texgen/amd/fixed/sprite-rgba-blend.ppm"),
        ],
    };
    for ((name, rust), expected) in names.iter().zip(&dumps).zip(native) {
        let expected = grading::ppm_rgb(expected, SIZE);
        let differing = rust
            .chunks(3)
            .zip(expected.chunks(3))
            .filter(|(a, b)| a != b);
        assert_eq!(
            differing.count(),
            0,
            "device={device} dump={name}; expected byte-exact"
        );
    }
}

/// Recorded failures: (device profile, dump, pixel), measured and not explained by an approved
/// rule. On llvmpipe the two silhouette pixels at (163, 94..95) pass the UV-seam rule: they lie on
/// the silhouette edge of back-facing triangles touching the u = 0/1 seam, the Rust sample there
/// sits on the texture's wrap (a column-63/column-0 blend), and the bottom-up control reproduces
/// native. AMD has no bottom-up control, so the same pixels stay recorded:
/// - AMD (RADV vs radeonsi), every UV-sampled dump: the seam pixels (163, 94..95) and one
///   two-level texel step at (137, 124) next to the u = 0 pole.
const RECORDED: [grading::RecordedFailure; 14] = [
    ("AMD-programmable", "sprite", Some((163, 94))),
    ("AMD-programmable", "sprite", Some((163, 95))),
    ("AMD-programmable", "sprite", Some((137, 124))),
    ("AMD-programmable", "sphere", Some((163, 94))),
    ("AMD-programmable", "sphere", Some((163, 95))),
    ("AMD-programmable", "sphere", Some((137, 124))),
    ("AMD-programmable", "sprite-after-sphere", Some((163, 94))),
    ("AMD-programmable", "sprite-after-sphere", Some((163, 95))),
    ("AMD-programmable", "sprite-after-sphere", Some((137, 124))),
    ("AMD-programmable", "manual", Some((163, 94))),
    ("AMD-programmable", "manual", Some((163, 95))),
    ("AMD-programmable", "manual", Some((137, 124))),
    // The fixed path's sprite dumps sample one texel and pass; its manual dump keeps the seam.
    ("AMD-fixed", "manual", Some((163, 94))),
    ("AMD-fixed", "manual", Some((163, 95))),
];

/// The same-driver bottom-up control (LegacyBottomUpV1, last provoking vertex) of `lighting` and
/// `coordinates`, lavapipe only; `bottom_up_control(&driver, coordinates, lighting)`.
fn bottom_up_control(
    driver: &Graphic3dRenderDriver,
    coordinates: RenderTextureCoordinatePipeline,
    lighting: RenderLightingPipeline,
) -> Option<Vec<Vec<u8>>> {
    let run = |control: &Handle<Graphic3dRenderDriver>| texgen_dumps(control).0;
    grading::pipeline_rounding_control(driver, (coordinates, lighting), run)
}

/// Grade every dump; the Sphere dump of the fixed path (`sphere_mapped`) also takes the
/// sphere-map rim rule and every other dump the UV-seam rule on the projected `seam`.
/// `profile` names the device and path for `RECORDED`.
fn assert_graded(
    profile: &str,
    (dumps, seam): (&[Vec<u8>], &[uv_seam::SeamSegment]),
    native: &[&[u8]; 4],
    control: Option<Vec<Vec<u8>>>,
    sphere_mapped: bool,
) {
    let report: Vec<grading::DumpReport> = DUMP_NAMES
        .iter()
        .enumerate()
        .map(|(index, name)| {
            let expected = grading::ppm_rgb(native[index], SIZE);
            let control = control.as_ref().map(|dumps| dumps[index].as_slice());
            let rust = &dumps[index];
            let (name, count, pixels) =
                grading::grade_dump(name, rust, expected, control, None, &[], SIZE.0);
            let pixels = match sphere_mapped && name == "sphere" {
                true => {
                    let all = |_, _| true;
                    sphere_map_rim::grade_sphere_map_rim(
                        pixels, rust, expected, control, SIZE.0, all,
                    )
                }
                false => uv_seam::grade_uv_seam(pixels, rust, expected, control, SIZE.0, seam),
            };
            (name, count, pixels.0)
        })
        .collect();
    let left = grading::unrecorded_pixels(&report, profile, &RECORDED);
    reference_cpu::require_exact(left.is_empty(), || {
        format!("profile={profile}; unrecorded pixels {left:?}")
    });
}

#[test]
fn programmable_textures_sample_the_array_uvs_under_every_generation_mode() {
    let driver = Graphic3dRenderDriver::new(grading::validation_options()).unwrap();
    let (dumps, seam) = texgen_dumps(&driver);
    grading::write_dumps(
        "MARTENSITE_SPHERE_TEXGEN_DUMP_DIR",
        &DUMP_NAMES,
        &dumps,
        SIZE,
    );
    let device = driver.device_name();
    let native = match device.contains("llvmpipe") {
        true => native_dumps!("llvmpipe").0,
        false => native_dumps!("amd").0,
    };
    let control = bottom_up_control(
        &driver,
        RenderTextureCoordinatePipeline::Programmable,
        RenderLightingPipeline::default(),
    );
    let profile = device.replace("llvmpipe", "llvmpipe-programmable");
    let profile = profile.replace("AMD", "AMD-programmable");
    assert_graded(&profile, (&dumps, &seam), &native, control, false);
}

/// Under the fixed-function option Sphere generates GL_SPHERE_MAP coordinates per vertex and
/// Sprite samples the texel at the centre on faces (Mesa's GL_COORD_REPLACE outside points).
#[test]
fn fixed_function_textures_generate_sphere_and_sprite_coordinates() {
    let driver = fixed_driver();
    let (dumps, seam) = texgen_dumps(&driver);
    grading::write_dumps(
        "MARTENSITE_SPHERE_TEXGEN_FIXED_DUMP_DIR",
        &DUMP_NAMES,
        &dumps,
        SIZE,
    );
    let device = driver.device_name();
    let native = match device.contains("llvmpipe") {
        true => native_dumps!("llvmpipe").1,
        false => native_dumps!("amd").1,
    };
    let control = bottom_up_control(
        &driver,
        RenderTextureCoordinatePipeline::GeneratedCompatibility,
        RenderLightingPipeline::FixedFunction,
    );
    let profile = device
        .replace("llvmpipe", "llvmpipe-fixed")
        .replace("AMD", "AMD-fixed");
    assert_graded(&profile, (&dumps, &seam), &native, control, true);
}

fn fixed_driver() -> Handle<Graphic3dRenderDriver> {
    Graphic3dRenderDriver::with_render_pipelines(
        grading::validation_options(),
        RenderTextureCoordinatePipeline::GeneratedCompatibility,
        RenderMeshEdgePipeline::default(),
        RenderLightingPipeline::FixedFunction,
    )
    .unwrap()
}
