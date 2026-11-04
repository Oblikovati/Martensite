//! Texture sets with emissive, occlusion, normal and metallic-roughness maps against native dumps.
//!
//! the recorded texture_maps_capture capture ran on llvmpipe
//! (SSE2/128) and AMD radeonsi. A shaded unit sphere (AisShape with computed UV) carries four
//! texture sets built from fixtures/texture_maps/*.ppm: `base` (base colour only), `maps` (base colour, emissive,
//! occlusion), `normal` (base colour, normal) and `all` (every unit). Each set is dumped under
//! the Phong, PhongFacet, Gouraud, Unlit, PBR and PbrFacet models; the facet, Gouraud and unlit
//! programs ignore the maps they do not declare.
#[path = "support/native_dump_grading.rs"]
mod grading;
#[path = "support/shift_control.rs"]
mod shift_control;
#[path = "case_host/snap_pixels.rs"]
mod snap_pixels;
#[allow(dead_code)] // Only the frame recorder serves the snap coverage rule here.
#[path = "case_host/tie_edges.rs"]
mod tie_edges;
#[path = "support/view_snap_pixels.rs"]
mod view_snap_pixels;
use grading::{DumpReport, validation_options};
use martensite_tkernel::ncollection::Vec3;
use martensite_tkernel::quantity::{Color, ColorSpace, NamedColor};
use martensite_tkernel::standard::Handle;
use martensite_tkservice::aspect::{AspectNeutralWindow, AspectWindowMembers as _};
use martensite_tkservice::graphic3d::*;
use martensite_tkservice::image::{ImageFormat, ImagePixMap};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape as _;
use martensite_tkv3d::ais::{AisInteractiveContext, AisInteractiveObject, AisShape};
use martensite_tkv3d::prs_mgr::*;
use martensite_tkv3d::v3d::*;
use martensite_vulkan::Graphic3dRenderDriver;

const SIZE: (usize, usize) = (160, 160);
const SETS: [&str; 4] = ["base", "maps", "normal", "all"];
const MODELS: [(&str, Graphic3dTypeOfShadingModel); 6] = [
    ("phong", Graphic3dTypeOfShadingModel::Phong),
    ("facet", Graphic3dTypeOfShadingModel::PhongFacet),
    ("gouraud", Graphic3dTypeOfShadingModel::Gouraud),
    ("unlit", Graphic3dTypeOfShadingModel::Unlit),
    ("pbr", Graphic3dTypeOfShadingModel::Pbr),
    ("pbrfacet", Graphic3dTypeOfShadingModel::PbrFacet),
];
/// Map files per set in unit order: base colour, emissive, occlusion, normal, metal-roughness.
const SET_MAPS: [[Option<&[u8]>; 5]; 4] = [
    [Some(BASE), None, None, None, None],
    [Some(BASE), Some(EMISSIVE), Some(OCCLUSION), None, None],
    [Some(BASE), None, None, Some(NORMAL), None],
    [
        Some(BASE),
        Some(EMISSIVE),
        Some(OCCLUSION),
        Some(NORMAL),
        Some(METAL_ROUGH),
    ],
];
const BASE: &[u8] = include_bytes!("fixtures/texture_maps/base.ppm");
const EMISSIVE: &[u8] = include_bytes!("fixtures/texture_maps/emissive.ppm");
const OCCLUSION: &[u8] = include_bytes!("fixtures/texture_maps/occlusion.ppm");
const NORMAL: &[u8] = include_bytes!("fixtures/texture_maps/normal.ppm");
const METAL_ROUGH: &[u8] = include_bytes!("fixtures/texture_maps/metal_rough.ppm");
include!("fixtures/texture_maps/native_dumps.rs");

/// Dump names in capture order: every model for each set; `dump_names()[0] == "phong-maps"`.
fn dump_names() -> Vec<&'static str> {
    let names = SETS
        .iter()
        .flat_map(|set| MODELS.iter().map(move |(model, _)| (*model, *set)));
    names
        .map(|(model, set)| &*format!("{model}-{set}").leak())
        .collect()
}

/// A 64x64 P6 map as a top-down RGBA pixmap with opaque alpha, as the capture reads it.
fn map_pixmap(ppm: &[u8]) -> Handle<ImagePixMap> {
    let rgb = grading::ppm_rgb(ppm, (64, 64));
    let rgba = rgb
        .as_chunks::<3>()
        .0
        .iter()
        .flat_map(|[r, g, b]| [*r, *g, *b, 255])
        .collect();
    let image = Handle::new(ImagePixMap::default());
    assert!(image.init_wrapper(ImageFormat::Rgba, Some(rgba), 64, 64, 256));
    image.set_top_down(true);
    image
}
/// One texture per present unit with modulation; XCAFPrs_Texture colour-map policy.
fn texture_set(maps: &[Option<&[u8]>; 5]) -> Handle<Graphic3dTextureSet> {
    let textures: Vec<Handle<dyn Graphic3dTextureMap>> = (0..5)
        .filter_map(|unit| maps[unit].map(|ppm| map_texture(ppm, unit)))
        .collect();
    let set = Graphic3dTextureSet::with_size(textures.len() as i32).unwrap();
    for (index, texture) in textures.into_iter().enumerate() {
        set.set_value(index as i32, texture).unwrap();
    }
    Handle::new(set)
}
fn map_texture(ppm: &[u8], unit: usize) -> Handle<dyn Graphic3dTextureMap> {
    let texture = Handle::new(Graphic3dTexture2D::from_pix_map(map_pixmap(ppm)));
    let unit_value = [
        Graphic3dTextureUnit::BASE_COLOR,
        Graphic3dTextureUnit::EMISSIVE,
        Graphic3dTextureUnit::OCCLUSION,
        Graphic3dTextureUnit::NORMAL,
        Graphic3dTextureUnit::METALLIC_ROUGHNESS,
    ][unit];
    texture.get_params().set_texture_unit(unit_value);
    texture.get_params().set_modulate(true);
    texture.set_color_map(unit < 2);
    texture.cast()
}
/// Plastified with an emissive colour; its PBR record is a white metal of unit roughness.
fn map_material() -> Graphic3dMaterialAspect {
    let mut material = Graphic3dMaterialAspect::new(Graphic3dNameOfMaterial::Plastified);
    material.set_emissive_color(Color::new(0.5, 0.4, 0.3, ColorSpace::Rgb).unwrap());
    let mut pbr = Graphic3dPbrMaterial::default();
    pbr.set_color(Color::new(0.9, 0.9, 0.9, ColorSpace::Rgb).unwrap());
    pbr.set_metallic(1.).unwrap();
    pbr.set_roughness(1.).unwrap();
    pbr.set_emission(Vec3::new(0.5, 0.4, 0.3)).unwrap();
    material.set_pbr_material(pbr);
    material
}

/// The viewer, context and view of the capture; `MapScene::new(&driver)`.
struct MapScene {
    _viewer: Handle<V3dViewer>,
    context: Handle<AisInteractiveContext>,
    view: Handle<dyn V3dViewSource>,
    _window: Handle<AspectNeutralWindow>,
}
impl MapScene {
    fn new(driver: &Handle<Graphic3dRenderDriver>) -> Self {
        let viewer = V3dViewer::new(&driver.cast()).unwrap();
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
            _viewer: viewer,
            context,
            view,
            _window: window,
        }
    }
    fn view(&self) -> &V3dView {
        self.view.get().unwrap().view_base()
    }
    /// Display the textured sphere of one set; `scene.display(&SET_MAPS[0])`.
    fn display(&self, maps: &[Option<&[u8]>; 5]) -> Handle<dyn AisInteractiveObject> {
        let shape = AisShape::new(&sphere());
        let drawer = shape.attributes();
        drawer
            .setup_own_shading_aspect(&self.context.default_drawer())
            .unwrap();
        shape.set_material(&map_material()).unwrap();
        let aspect = shape.attributes().shading_aspect().aspect();
        aspect.set_texture_map_on(true);
        aspect.set_texture_set(texture_set(maps));
        let object: Handle<dyn AisInteractiveObject> = shape.cast();
        object.get().unwrap().set_display_mode(1);
        self.context
            .display_with_modes(&object, 1, 0, false, PrsMgrDisplayStatus::None)
            .unwrap();
        object
    }
    fn dump(&self, model: Graphic3dTypeOfShadingModel) -> Vec<u8> {
        self.view()
            .change_rendering_params(&mut |params| params.shading_model = model);
        self.view.redraw();
        grading::view_rgb(self.view(), SIZE)
    }
}
fn sphere() -> martensite_tkbrep::topods::Shape {
    let mut maker = martensite_tkprim::brep_prim_api::BRepPrimAPIMakeSphere::new(1.).unwrap();
    maker.shape().unwrap().clone()
}
/// Run the capture on `driver`; every dump in `dump_names()` order.
fn texture_map_dumps(driver: &Handle<Graphic3dRenderDriver>) -> Vec<Vec<u8>> {
    shifted_texture_map_dumps(driver, [0., 0.])
}
/// The dumps with the camera shifted by `shift` pixels after the fit (`shift_control`).
fn shifted_texture_map_dumps(
    driver: &Handle<Graphic3dRenderDriver>,
    shift: [f64; 2],
) -> Vec<Vec<u8>> {
    let scene = MapScene::new(driver);
    let mut dumps = Vec::new();
    for (index, maps) in SET_MAPS.iter().enumerate() {
        let object = scene.display(maps);
        if index == 0 {
            scene.view().fit_all(0.01, false).unwrap();
            shift_control::shift_camera(scene.view(), shift, SIZE.0);
        }
        dumps.extend(MODELS.iter().map(|(_, model)| scene.dump(*model)));
        scene.context.remove(&object, false).unwrap();
    }
    scene.view.remove();
    assert_eq!(driver.validation_errors().unwrap(), Vec::<String>::new());
    dumps
}

/// Pixels that no approved residue explains, per device profile:
/// - llvmpipe (80,52) of `pbrfacet-all`: one level in green, where the bottom-up control equals
///   the Rust image, not native (the residue of the PBR sphere's `pbr-transparent`/`pbr-lights`).
///
/// AMD (93,126), formerly listed in 11 dumps, lies 0.0021 px from a front facet edge; a
/// half-step camera shift on RADV reproduces native there in every dump (`shift_control`).
const RECORDED_FAILURES: [grading::RecordedFailure; 1] =
    [("llvmpipe", "pbrfacet-all", Some((80, 52)))];
/// Grade every dump on the selected device; `(device, report)`.
fn graded_texture_maps() -> (String, Vec<DumpReport>) {
    let driver = Graphic3dRenderDriver::new(validation_options()).unwrap();
    let dumps = texture_map_dumps(&driver);
    let control = grading::rounding_control(&driver, texture_map_dumps);
    let shifts = shift_control::shift_controls(&driver, shifted_texture_map_dumps);
    let device = driver.device_name();
    let snaps = (!device.contains("llvmpipe")).then(|| sphere_snaps(&driver));
    let native = if device.contains("llvmpipe") {
        NATIVE_LLVMPIPE
    } else {
        NATIVE_AMD
    };
    let names = dump_names();
    grading::write_dumps("MARTENSITE_TEXTURE_MAPS_DUMP_DIR", &names, &dumps, SIZE);
    let report: Vec<DumpReport> = dumps
        .iter()
        .enumerate()
        .map(|(index, rgb)| {
            let expected = grading::ppm_rgb(native[index], SIZE);
            let control = control.as_ref().map(|dumps| dumps[index].as_slice());
            let shifted = shift_control::dump_shifts(&shifts, index);
            grading::grade_dump(
                names[index],
                rgb,
                expected,
                control,
                snaps.as_ref(),
                &shifted,
                SIZE.0,
            )
        })
        .collect();
    println!("{{\"texture_maps_device\":{device:?},\"dumps\":{report:?}}}");
    (device, report)
}
#[test]
fn texture_map_sets_match_the_native_dumps() {
    let (device, report) = graded_texture_maps();
    let failing = grading::unrecorded_pixels(&report, &device, &RECORDED_FAILURES);
    assert!(
        failing.is_empty(),
        "device={device}; expected byte-exact dumps or approved residues: {failing:?}"
    );
}

#[test]
#[ignore = "recorded failures: RECORDED_FAILURES lists the pixels no approved residue explains"]
fn texture_map_recorded_failures_match_the_native_dumps() {
    let (device, report) = graded_texture_maps();
    let failing = grading::unrecorded_pixels(&report, &device, &[]);
    assert!(
        failing.is_empty(),
        "device={device}; expected every dump within an approved bound: {failing:?}"
    );
}

/// Snap-dependent pixels of the fitted sphere (every dump shares its tessellation and camera).
fn sphere_snaps(driver: &Handle<Graphic3dRenderDriver>) -> std::collections::BTreeSet<(i32, i32)> {
    let scene = MapScene::new(driver);
    let object = scene.display(&SET_MAPS[0]);
    scene.view().fit_all(0.01, false).unwrap();
    let snaps = view_snap_pixels::view_snap_pixels(&scene.view, SIZE);
    scene.context.remove(&object, false).unwrap();
    scene.view.remove();
    snaps
}
