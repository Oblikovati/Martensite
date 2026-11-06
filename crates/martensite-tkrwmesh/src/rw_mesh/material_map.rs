//! `RwMeshMaterialMap`: unique material names for the styles a writer meets.
use martensite_tkernel::impl_transient_interface;
use martensite_tkernel::ncollection::{DefaultHashPolicy, DoubleMap, HashPolicy, Map};
use martensite_tkernel::osd::CadPath;
use martensite_tkernel::standard::{Failure, Handle, StandardTransient, Transient};
use martensite_tkernel::tcollection::AsciiString;
use martensite_tklcaf::tdf::TdfAttribute;
use martensite_tkservice::image::{ImageTextureHashPolicy, ImageTextureSource};
use martensite_tkxcaf::xcaf_prs::XCAFPrsStyle;

use super::RwMesh;

/// A style as a key of the material map; `RwMeshStyleKey(style.clone())`.
#[derive(Clone)]
pub struct RwMeshStyleKey(pub XCAFPrsStyle);
impl std::fmt::Debug for RwMeshStyleKey {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "RwMeshStyleKey({})", self.0.hash_code())
    }
}

/// Hashes styles by their presentation values; `RwMeshStyleHashPolicy`.
#[derive(Clone, Copy, Debug, Default)]
pub struct RwMeshStyleHashPolicy;
impl HashPolicy<RwMeshStyleKey> for RwMeshStyleHashPolicy {
    fn hash(&self, key: &RwMeshStyleKey) -> usize {
        key.0.hash_code()
    }
    fn equal(&self, first: &RwMeshStyleKey, second: &RwMeshStyleKey) -> bool {
        first.0.is_equal(&second.0)
    }
}

/// The state of a material map; `RwMeshMaterialMapBase::new(&file)`.
pub struct RwMeshMaterialMapBase {
    /// The output folder of the model, `.` for none.
    pub folder: AsciiString,
    /// The output folder of images, full path.
    pub tex_folder: AsciiString,
    /// The output folder of images, relative to the model.
    pub tex_folder_short: AsciiString,
    /// The output file.
    pub file_name: AsciiString,
    /// The output file name without its extension.
    pub short_file_name_base: AsciiString,
    /// The prefix of generated keys.
    pub key_prefix: AsciiString,
    /// The styles processed, with their keys.
    pub styles: DoubleMap<RwMeshStyleKey, AsciiString, RwMeshStyleHashPolicy, DefaultHashPolicy>,
    /// The images that failed to copy.
    pub image_fail_map: Map<Handle<dyn ImageTextureSource>, ImageTextureHashPolicy>,
    /// The material for nodes with only a color.
    pub default_style: XCAFPrsStyle,
    /// The number of materials registered.
    pub nb_materials: i32,
    /// Whether something failed.
    pub is_failed: bool,
    /// Whether the material name is the key.
    pub mat_name_as_key: bool,
}

impl RwMeshMaterialMapBase {
    /// A map for an output file; `RwMeshMaterialMapBase::new(&AsciiString::new("out/model.obj"))`.
    pub fn new(file: &AsciiString) -> Self {
        let (folder, name) = CadPath::folder_and_file_from_path(file.as_bytes());
        let (base_name, _) = CadPath::file_name_and_extension(name.as_bytes());
        Self {
            folder: if folder.is_empty() {
                AsciiString::new(".")
            } else {
                folder
            },
            tex_folder: AsciiString::default(),
            tex_folder_short: AsciiString::default(),
            file_name: file.clone(),
            short_file_name_base: base_name,
            key_prefix: AsciiString::new("mat_"),
            styles: DoubleMap::default(),
            image_fail_map: Map::default(),
            default_style: XCAFPrsStyle::default(),
            nb_materials: 0,
            is_failed: false,
            mat_name_as_key: true,
        }
    }
}

fn concatenated(parts: &[&[u8]]) -> AsciiString {
    AsciiString::from_raw(&parts.concat())
}

fn with_counter(prefix: &AsciiString, counter: i32) -> AsciiString {
    concatenated(&[prefix.as_bytes(), counter.to_string().as_bytes()])
}

/// A named material takes its own counter from zero; others share the map's counter.
fn material_name(style: &XCAFPrsStyle) -> Result<Option<AsciiString>, Failure> {
    let Some(material) = style.material().get().filter(|m| !m.is_empty()) else {
        return Ok(None);
    };
    let label = material.attribute_base().label();
    if label.is_null() {
        return Ok(Some(AsciiString::new("mat")));
    }
    let name = RwMesh::read_name_attribute(&label)?;
    Ok(Some(if name.is_empty() && !has_name(&label)? {
        AsciiString::new("mat")
    } else {
        name
    }))
}

fn has_name(label: &martensite_tklcaf::tdf::TdfLabel) -> Result<bool, Failure> {
    label.is_attribute(&martensite_tklcaf::tdata_std::TDataStdName::get_id())
}

impl_transient_interface!(
    dyn RwMeshMaterialMap,
    "RwMeshMaterialMap",
    StandardTransient
);

/// A map of the styles a writer meets; a format implements [`Self::define_material`].
pub trait RwMeshMaterialMap: Transient {
    /// The map state; `map.material_base()`.
    fn material_base(&self) -> &RwMeshMaterialMapBase;
    /// The mutable map state; `map.material_base_mut()`.
    fn material_base_mut(&mut self) -> &mut RwMeshMaterialMapBase;
    /// Defines a new material, for example in the output file;
    /// `map.define_material(&style, &key, &name)?`.
    fn define_material(
        &mut self,
        style: &XCAFPrsStyle,
        key: &AsciiString,
        name: &AsciiString,
    ) -> Result<(), Failure>;

    /// The material of nodes with only a color; `map.default_style()`.
    fn default_style(&self) -> &XCAFPrsStyle {
        &self.material_base().default_style
    }
    /// Sets the material of nodes with only a color; `map.set_default_style(&style)`.
    fn set_default_style(&mut self, style: &XCAFPrsStyle) {
        self.material_base_mut().default_style = style.clone();
    }
    /// The key of a registered material, empty when unknown; `map.find_material(&style)`.
    fn find_material(&self, style: &XCAFPrsStyle) -> AsciiString {
        self.material_base()
            .styles
            .seek1(&RwMeshStyleKey(style.clone()))
            .cloned()
            .unwrap_or_default()
    }
    /// Registers a material and returns its key; `map.add_material(&style)?`.
    fn add_material(&mut self, style: &XCAFPrsStyle) -> Result<AsciiString, Failure> {
        add_material_to(self, style)
    }
    /// Creates the texture folder `<model>_textures` beside the model, which is not cleared;
    /// `map.create_texture_folder()`.
    fn create_texture_folder(&mut self) -> bool {
        super::material_map_files::create_texture_folder(self.material_base_mut())
    }
    /// Copies a texture, returning its path relative to the model through `result`;
    /// `map.copy_texture(&mut path, &texture, &key)`.
    fn copy_texture(
        &mut self,
        result: &mut AsciiString,
        texture: &Handle<dyn ImageTextureSource>,
        key: &AsciiString,
    ) -> bool {
        self.create_texture_folder();
        super::material_map_files::copy_texture(self.material_base(), result, texture, key)
    }
    /// True once something failed; `map.is_failed()`.
    fn is_failed(&self) -> bool {
        self.material_base().is_failed
    }
}

/// Registers a style under a key unique in the map, which a map's own
/// [`RwMeshMaterialMap::add_material`] can call after its preparations;
/// `add_material_to(&mut map, &style)?`.
pub fn add_material_to<M: RwMeshMaterialMap + ?Sized>(
    map: &mut M,
    style: &XCAFPrsStyle,
) -> Result<AsciiString, Failure> {
    let style_key = RwMeshStyleKey(style.clone());
    if let Some(key) = map.material_base().styles.seek1(&style_key) {
        return Ok(key.clone());
    }
    let (key, name) = new_key(map.material_base_mut(), style)?;
    map.material_base_mut()
        .styles
        .bind(style_key, key.clone())?;
    map.define_material(style, &key, &name)?;
    Ok(key)
}

fn new_key(
    base: &mut RwMeshMaterialMapBase,
    style: &XCAFPrsStyle,
) -> Result<(AsciiString, AsciiString), Failure> {
    if !base.mat_name_as_key {
        return Ok(new_numeric_key(base));
    }
    let named = material_name(style)?;
    let is_named = named.is_some();
    let (suffix, mut name) = match named {
        Some(name) => (name.clone(), name),
        None => {
            base.nb_materials += 1;
            (
                base.key_prefix.clone(),
                with_counter(&base.key_prefix, base.nb_materials),
            )
        }
    };
    let mut counter = 0;
    while base.styles.is_bound2(&name) {
        let current = if is_named {
            &mut counter
        } else {
            &mut base.nb_materials
        };
        name = with_counter(&suffix, *current);
        *current += 1;
    }
    Ok((name.clone(), name))
}

fn new_numeric_key(base: &mut RwMeshMaterialMapBase) -> (AsciiString, AsciiString) {
    let mut key = AsciiString::from_integer(base.nb_materials);
    base.nb_materials += 1;
    while base.styles.is_bound2(&key) {
        key = AsciiString::from_integer(base.nb_materials);
        base.nb_materials += 1;
    }
    (
        key.clone(),
        concatenated(&[base.key_prefix.as_bytes(), key.as_bytes()]),
    )
}
