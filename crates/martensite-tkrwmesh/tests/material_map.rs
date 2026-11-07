//! Unique material keys and the texture files of a map.
use martensite_tkernel::impl_transient;
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkernel::standard::StandardTransient;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkrwmesh::rw_mesh::{RwMeshMaterialMap, RwMeshMaterialMapBase, RwMeshStyleKey};
use martensite_tkservice::image::{ImageTexture, ImageTextureSource};
use martensite_tkxcaf::xcaf_doc::XCAFDocVisMaterial;
use martensite_tkxcaf::xcaf_prs::XCAFPrsStyle;

/// A map that records what it is asked to define.
struct RecordingMap {
    transient: StandardTransient,
    base: RwMeshMaterialMapBase,
    defined: Vec<(String, String)>,
}

impl_transient!(RecordingMap, "RecordingMap", dyn RwMeshMaterialMap, transient, interfaces: [dyn RwMeshMaterialMap]);

impl RwMeshMaterialMap for RecordingMap {
    fn material_base(&self) -> &RwMeshMaterialMapBase {
        &self.base
    }
    fn material_base_mut(&mut self) -> &mut RwMeshMaterialMapBase {
        &mut self.base
    }
    fn define_material(
        &mut self,
        _style: &XCAFPrsStyle,
        key: &AsciiString,
        name: &AsciiString,
    ) -> Result<(), Failure> {
        let text = |value: &AsciiString| String::from_utf8_lossy(value.as_bytes()).into_owned();
        self.defined.push((text(key), text(name)));
        Ok(())
    }
}

fn map_at(file: &str) -> RecordingMap {
    RecordingMap {
        transient: StandardTransient::default(),
        base: RwMeshMaterialMapBase::new(&AsciiString::new(file)),
        defined: Vec::new(),
    }
}

fn colored(color: NamedColor) -> XCAFPrsStyle {
    let mut style = XCAFPrsStyle::default();
    style.set_color_surf(Color::named(color));
    style
}

#[test]
fn colors_take_numbered_keys_and_repeat_their_key() {
    let mut map = map_at("out/model.obj");
    assert_eq!(map.material_base().folder.as_bytes(), b"out/");
    assert_eq!(
        map.material_base().short_file_name_base.as_bytes(),
        b"model"
    );
    let (red, blue) = (colored(NamedColor::RED), colored(NamedColor::BLUE));
    assert_eq!(map.add_material(&red).unwrap().as_bytes(), b"mat_1");
    assert_eq!(map.add_material(&blue).unwrap().as_bytes(), b"mat_2");
    assert_eq!(map.add_material(&red).unwrap().as_bytes(), b"mat_1");
    assert_eq!(map.find_material(&blue).as_bytes(), b"mat_2");
    assert!(map.find_material(&colored(NamedColor::GREEN)).is_empty());
    assert_eq!(
        map.defined,
        [
            ("mat_1".into(), "mat_1".into()),
            ("mat_2".into(), "mat_2".into())
        ]
    );
    assert!(!map.is_failed());
}

#[test]
fn named_materials_are_unique_by_a_counter_and_numeric_keys_count() {
    let mut map = map_at("model.obj");
    assert_eq!(map.material_base().folder.as_bytes(), b".");
    let mut with_material = |roughness: f32| {
        let material = XCAFDocVisMaterial::new();
        let common = martensite_tkxcaf::xcaf_doc::XCAFDocVisMaterialCommon {
            shininess: roughness,
            ..Default::default()
        };
        material.set_common_material(&common).unwrap();
        let mut style = XCAFPrsStyle::default();
        style.set_material(&Handle::new(material));
        map.add_material(&style).unwrap()
    };
    assert_eq!(with_material(0.1).as_bytes(), b"mat");
    assert_eq!(with_material(0.2).as_bytes(), b"mat0");
    let mut numeric = map_at("model.obj");
    numeric.material_base_mut().mat_name_as_key = false;
    assert_eq!(
        numeric
            .add_material(&colored(NamedColor::RED))
            .unwrap()
            .as_bytes(),
        b"0"
    );
    assert_eq!(numeric.defined[0].1, "mat_0");
}

#[test]
fn the_default_style_is_kept_and_keys_print_their_hash() {
    let mut map = map_at("model.obj");
    let red = colored(NamedColor::RED);
    map.set_default_style(&red);
    assert!(map.default_style().is_set_color_surf());
    assert!(format!("{:?}", RwMeshStyleKey(red)).starts_with("RwMeshStyleKey("));
}

#[test]
fn textures_copy_beside_the_model_and_a_missing_one_fails() {
    let folder = std::env::temp_dir().join(format!("tkrwmesh-textures-{}", std::process::id()));
    std::fs::create_dir_all(&folder).unwrap();
    let image = folder.join("wood.png");
    std::fs::write(&image, b"not really an image").unwrap();
    let mut map = map_at(folder.join("model.obj").to_str().unwrap());
    assert!(map.create_texture_folder());
    let texture: Handle<dyn ImageTextureSource> = Handle::new(ImageTexture::new(
        &AsciiString::new(image.to_str().unwrap()),
    ))
    .cast();
    let mut path = AsciiString::default();
    assert!(map.copy_texture(&mut path, &texture, &AsciiString::new("1")));
    assert_eq!(path.as_bytes(), b"model_textures/wood.png");
    assert!(folder.join("model_textures/wood.png").exists());
    let missing: Handle<dyn ImageTextureSource> =
        Handle::new(ImageTexture::new(&AsciiString::new("/nowhere/at/all.png"))).cast();
    assert!(!map.copy_texture(&mut path, &missing, &AsciiString::new("2")));
    std::fs::remove_dir_all(&folder).unwrap();
}

#[test]
fn a_texture_folder_under_a_missing_folder_fails() {
    let mut map = map_at("/nowhere/at/all/model.obj");
    assert!(!map.create_texture_folder());
}
