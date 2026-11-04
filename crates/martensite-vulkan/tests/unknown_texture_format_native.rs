//! A surface texture whose pixmap has the unknown image format, against native dumps.
//!
//! the recorded unknown_texture_format capture (llvmpipe SSE2/128 and
//! radeonsi) maps a 4x4 ImageFormat_UNKNOWN wrapper, between two RGB controls, on an unlit,
//! white, non-modulated quad over a grey background. GlTextureFormat::FindFormat has no GL
//! format for it, GlTexture::Init fails ("No suitable texture format") and BindTextures
//! binds nothing: the program samples the empty unit as opaque black, the fixed-function
//! pipeline draws the untextured white quad, and the next RGB texture draws as before. A texture
//! of a missing file (a null image, the same GlTexture::Init failure) follows, then another
//! RGB control.
#[path = "support/native_dump_grading.rs"]
#[allow(dead_code)]
mod grading;
use martensite_tkernel::quantity::{Color, ColorSpace, NamedColor};
use martensite_tkernel::standard::Handle;
use martensite_tkmath::gp::{Direction3, Point3};
use martensite_tkservice::aspect::AspectWindowMembers as _;
use martensite_tkservice::aspect::{AspectInteriorStyle, AspectNeutralWindow};
use martensite_tkservice::graphic3d::*;
use martensite_tkservice::image::{ImageFormat, ImagePixMap};
use martensite_tkv3d::v3d::*;
use martensite_vulkan::{
    Graphic3dRenderDriver, RenderLightingPipeline, RenderMeshEdgePipeline,
    RenderTextureCoordinatePipeline,
};

const SIZE: (usize, usize) = (200, 200);
const NAMES: [&str; 5] = [
    "rgb-control",
    "unknown-format",
    "rgb-after-unknown",
    "missing-file",
    "rgb-after-missing",
];

macro_rules! native_dumps {
    ($dir:literal) => {
        [
            include_bytes!(concat!(
                "fixtures/unknown_texture_format/",
                $dir,
                "/rgb-control.ppm"
            ))
            .as_slice(),
            include_bytes!(concat!(
                "fixtures/unknown_texture_format/",
                $dir,
                "/unknown-format.ppm"
            ))
            .as_slice(),
            include_bytes!(concat!(
                "fixtures/unknown_texture_format/",
                $dir,
                "/rgb-after-unknown.ppm"
            ))
            .as_slice(),
            include_bytes!(concat!(
                "fixtures/unknown_texture_format/",
                $dir,
                "/missing-file.ppm"
            ))
            .as_slice(),
            include_bytes!(concat!(
                "fixtures/unknown_texture_format/",
                $dir,
                "/rgb-after-missing.ppm"
            ))
            .as_slice(),
        ]
    };
}
const LLVMPIPE_PROGRAMMABLE: [&[u8]; 5] = native_dumps!("llvmpipe-programmable");
const LLVMPIPE_FIXED: [&[u8]; 5] = native_dumps!("llvmpipe-fixed");
const AMD_PROGRAMMABLE: [&[u8]; 5] = native_dumps!("amd-programmable");
const AMD_FIXED: [&[u8]; 5] = native_dumps!("amd-fixed");

/// 4x4 pixmaps: the unknown format (one byte per pixel) and the red/blue RGB checker.
fn probe_image(format: ImageFormat) -> Handle<ImagePixMap> {
    let bytes: Vec<u8> = match format {
        ImageFormat::Unknown => (0..16).map(|i| (i * 16 + 8) as u8).collect(),
        _ => (0..16)
            .flat_map(|i| match ((i / 4) + (i % 4)) % 2 == 0 {
                true => [230, 40, 20],
                false => [20, 40, 230],
            })
            .collect(),
    };
    let row = bytes.len() / 4;
    let image = Handle::new(ImagePixMap::default());
    assert!(image.init_wrapper(format, Some(bytes), 4, 4, row));
    image
}

/// A probe texture: a 4x4 pixmap of `format`, or (`None`) a file that does not exist, whose
/// GetImage returns a null image.
fn probe_texture(format: Option<ImageFormat>) -> Handle<Graphic3dTexture2D> {
    match format {
        Some(format) => Handle::new(Graphic3dTexture2D::from_pix_map(probe_image(format))),
        None => Handle::new(Graphic3dTexture2D::from_file(
            &"/nonexistent/martensite-missing.png".into(),
        )),
    }
}

fn textured_aspect(format: Option<ImageFormat>) -> Handle<Graphic3dAspectFillArea3d> {
    let texture = probe_texture(format);
    texture.get_params().set_modulate(false);
    let set = Graphic3dTextureSet::with_size(1).unwrap();
    set.set_value(0, texture.cast()).unwrap();
    let fill = Handle::new(Graphic3dAspectFillArea3d::default());
    fill.set_interior_style(AspectInteriorStyle::Solid);
    fill.set_shading_model(Graphic3dTypeOfShadingModel::Unlit);
    fill.set_interior_color(Color::named(NamedColor::WHITE));
    fill.set_texture_set(Handle::new(set));
    fill.set_texture_map_on(true);
    fill
}

/// The probe's quad over [-60, 60]^2 with texels spanning [0, 1]^2.
fn quad() -> Handle<Graphic3dArrayOfPrimitives> {
    let kind = Graphic3dTypeOfPrimitiveArray::Triangles;
    let flags = GRAPHIC3D_ARRAY_FLAGS_VERTEX_TEXEL;
    let array = Handle::new(Graphic3dArrayOfPrimitives::new(kind, 6, 0, 0, flags).unwrap());
    let corners = [
        [-60., -60., 0., 0.],
        [60., -60., 1., 0.],
        [60., 60., 1., 1.],
    ];
    let rest = [
        [-60., -60., 0., 0.],
        [60., 60., 1., 1.],
        [-60., 60., 0., 1.],
    ];
    for [x, y, s, t] in corners.into_iter().chain(rest) {
        array.add_vertex_texel([x, y, 0.], [s, t]).unwrap();
    }
    array
}

struct ProbeView {
    view: Handle<dyn V3dViewSource>,
    viewer: Handle<V3dViewer>,
    _window: Handle<AspectNeutralWindow>,
}

impl ProbeView {
    fn new(driver: &Handle<Graphic3dRenderDriver>) -> Self {
        let viewer = V3dViewer::new(&driver.cast()).unwrap();
        let grey = Color::new(0.25, 0.25, 0.25, ColorSpace::Srgb).unwrap();
        viewer.set_default_background_color(grey);
        let window = Handle::new(AspectNeutralWindow::default());
        window.set_size(SIZE.0 as i32, SIZE.1 as i32);
        window.set_virtual(true);
        let view = viewer.create_view().unwrap();
        view.set_window(&window.cast(), 0).unwrap();
        let v3d = view.get().unwrap();
        v3d.set_auto_z_fit_mode(false, 1.).unwrap();
        let camera = v3d.camera();
        camera.set_projection_type(Graphic3dCameraProjection::Orthographic);
        camera.set_eye(&Point3::new(0., 0., 10.)).unwrap();
        camera.set_center(&Point3::new(0., 0., 0.)).unwrap();
        camera.set_up(&Direction3::new(0., 1., 0.).unwrap());
        camera.set_scale(200.);
        camera.set_z_range(1., 20.).unwrap();
        Self {
            view,
            viewer,
            _window: window,
        }
    }

    fn dump(&self, format: Option<ImageFormat>) -> Vec<u8> {
        let structure =
            Graphic3dStructure::new(&self.viewer.structure_manager(), &Handle::default()).unwrap();
        let group = structure.new_group().unwrap();
        group.set_group_primitives_aspect(&textured_aspect(format).cast());
        group.add_primitive_array(&quad(), true);
        structure.display();
        let v3d = self.view.get().unwrap();
        v3d.redraw();
        let rgb = grading::view_rgb(v3d.view_base(), SIZE);
        structure.erase();
        rgb
    }
}

fn differing(
    driver: &Handle<Graphic3dRenderDriver>,
    native: [&[u8]; 5],
) -> Vec<(&'static str, usize)> {
    let scene = ProbeView::new(driver);
    let rgb = Some(ImageFormat::Rgb);
    let formats = [rgb, Some(ImageFormat::Unknown), rgb, None, rgb];
    let dumps: Vec<Vec<u8>> = formats
        .into_iter()
        .map(|format| scene.dump(format))
        .collect();
    grading::write_dumps("MARTENSITE_UNKNOWN_TEXTURE_DUMP_DIR", &NAMES, &dumps, SIZE);
    NAMES
        .iter()
        .zip(&dumps)
        .zip(native)
        .map(|((name, rust), expected)| {
            let expected = grading::ppm_rgb(expected, SIZE);
            let count = rust
                .chunks(3)
                .zip(expected.chunks(3))
                .filter(|(a, b)| a != b)
                .count();
            (*name, count)
        })
        .filter(|(_, count)| *count > 0)
        .collect()
}

#[test]
fn programmable_unknown_format_textures_sample_opaque_black_as_natively() {
    let driver = Graphic3dRenderDriver::new(grading::validation_options()).unwrap();
    let native = match driver.device_name().contains("llvmpipe") {
        true => LLVMPIPE_PROGRAMMABLE,
        false => AMD_PROGRAMMABLE,
    };
    let differing = differing(&driver, native);
    assert!(
        differing.is_empty(),
        "differing (dump, pixels): {differing:?}"
    );
}

#[test]
fn fixed_function_unknown_format_textures_draw_untextured_as_natively() {
    let driver = Graphic3dRenderDriver::with_render_pipelines(
        grading::validation_options(),
        RenderTextureCoordinatePipeline::GeneratedCompatibility,
        RenderMeshEdgePipeline::default(),
        RenderLightingPipeline::FixedFunction,
    )
    .unwrap();
    let native = match driver.device_name().contains("llvmpipe") {
        true => LLVMPIPE_FIXED,
        false => AMD_FIXED,
    };
    let differing = differing(&driver, native);
    assert!(
        differing.is_empty(),
        "differing (dump, pixels): {differing:?}"
    );
}
