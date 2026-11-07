//! `RwObjObjMaterialMap`: writing the MTL file of an OBJ export.
use super::writer_context::{EXPORT_HEADER, c_fixed};
use martensite_tkernel::impl_transient;
use martensite_tkernel::message::Message;
use martensite_tkernel::ncollection::DataMap;
use martensite_tkernel::quantity::{Color, ColorSpace};
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkrwmesh::rw_mesh::{RwMeshMaterialMap, RwMeshMaterialMapBase, add_material_to};
use martensite_tkservice::image::{ImageTextureHashPolicy, ImageTextureSource};
use martensite_tkxcaf::xcaf_doc::{XCAFDocVisMaterial, XCAFDocVisMaterialCommon};
use martensite_tkxcaf::xcaf_prs::XCAFPrsStyle;
use std::fs::File;
use std::io::{BufWriter, Write};

/// Writes materials to an MTL file, created when the first one arrives;
/// `RwObjObjMaterialMap::new(&AsciiString::new("model.mtl"))`.
pub struct RwObjObjMaterialMap {
    transient: StandardTransient,
    base: RwMeshMaterialMapBase,
    file: Option<BufWriter<File>>,
    image_map: DataMap<Handle<dyn ImageTextureSource>, AsciiString, ImageTextureHashPolicy>,
}

impl_transient!(RwObjObjMaterialMap, "RwObjObjMaterialMap", dyn RwMeshMaterialMap, transient, interfaces: [dyn RwMeshMaterialMap]);

/// The values of a material statement.
struct MaterialValues {
    ambient: Color,
    diffuse: Color,
    specular: Color,
    transparency: f32,
    shininess: f32,
    is_defined: bool,
}

fn common_of(material: &Handle<XCAFDocVisMaterial>) -> Result<XCAFDocVisMaterialCommon, Failure> {
    match material.get() {
        Some(material) => material.convert_to_common_material(),
        None => Ok(XCAFDocVisMaterialCommon::default()),
    }
}

fn scaled_color(color: Color, factor: f32) -> Result<Color, Failure> {
    let [r, g, b] = color.rgb().map(|channel| f64::from(channel * factor));
    Color::new(r, g, b, ColorSpace::Rgb)
}

fn material_values(
    style: &XCAFPrsStyle,
    default_style: &XCAFPrsStyle,
) -> Result<MaterialValues, Failure> {
    let default = common_of(default_style.material())?;
    let mut values = MaterialValues {
        ambient: default.ambient_color,
        diffuse: default.diffuse_color,
        specular: default.specular_color,
        transparency: 0.0,
        shininess: default.shininess * 1000.0,
        is_defined: false,
    };
    if style
        .material()
        .get()
        .is_some_and(|material| !material.is_empty())
    {
        let common = common_of(style.material())?;
        values.ambient = common.ambient_color;
        values.diffuse = common.diffuse_color;
        values.specular = common.specular_color;
        values.transparency = common.transparency;
        values.shininess = common.shininess * 1000.0;
        values.is_defined = true;
    }
    if style.is_set_color_surf() {
        values.is_defined = true;
        values.diffuse = style.get_color_surf();
        values.ambient = scaled_color(style.get_color_surf(), 0.25)?;
        let alpha = style.get_color_surf_rgba().alpha();
        if alpha < 1.0 {
            values.transparency = 1.0 - alpha;
        }
    }
    Ok(values)
}

fn color_statement(key: &str, color: Color) -> String {
    let [r, g, b] = color.values(ColorSpace::Srgb).map(c_fixed);
    format!("{key} {r} {g} {b}\n")
}

fn statements_of(values: &MaterialValues) -> String {
    let mut text = color_statement("Ka", values.ambient);
    text += &color_statement("Kd", values.diffuse);
    text += &color_statement("Ks", values.specular);
    text += &format!("Ns {}\n", c_fixed(f64::from(values.shininess)));
    if values.transparency >= 0.0001 {
        text += &format!("Tr {}\n", c_fixed(f64::from(values.transparency)));
    }
    text
}

impl RwObjObjMaterialMap {
    /// A map writing the named file; `RwObjObjMaterialMap::new(&file)`.
    pub fn new(file: &AsciiString) -> Self {
        Self {
            transient: StandardTransient::default(),
            base: RwMeshMaterialMapBase::new(file),
            file: None,
            image_map: DataMap::default(),
        }
    }
    fn write_bytes(&mut self, bytes: &[u8]) {
        let Some(file) = self.file.as_mut() else {
            return;
        };
        if file.write_all(bytes).is_err() {
            self.base.is_failed = true;
        }
    }
    fn open_file(&mut self) {
        if self.file.is_some() || self.base.is_failed {
            return;
        }
        let path = String::from_utf8_lossy(self.base.file_name.as_bytes()).into_owned();
        self.file = File::create(path).map(BufWriter::new).ok();
        self.base.is_failed = self.file.is_none();
        self.write_bytes(format!("{EXPORT_HEADER}\n").as_bytes());
    }
    /// The texture of a style written next to the model, empty when it could not be.
    fn texture_path(&mut self, texture: &Handle<dyn ImageTextureSource>) -> AsciiString {
        if let Some(known) = self.image_map.seek(texture) {
            return known.clone();
        }
        let mut path = AsciiString::default();
        if self.base.image_fail_map.contains(texture) {
            return path;
        }
        let key = AsciiString::from_integer(self.image_map.len() as i32 + 1);
        if self.copy_texture(&mut path, texture, &key) {
            self.image_map.bind(texture.clone(), path.clone());
        } else {
            self.base.image_fail_map.add(texture.clone());
        }
        path
    }
}

impl RwMeshMaterialMap for RwObjObjMaterialMap {
    fn material_base(&self) -> &RwMeshMaterialMapBase {
        &self.base
    }
    fn material_base_mut(&mut self) -> &mut RwMeshMaterialMapBase {
        &mut self.base
    }
    fn add_material(&mut self, style: &XCAFPrsStyle) -> Result<AsciiString, Failure> {
        self.open_file();
        if self.file.is_none() {
            return Ok(AsciiString::default());
        }
        add_material_to(self, style)
    }
    fn define_material(
        &mut self,
        style: &XCAFPrsStyle,
        key: &AsciiString,
        _name: &AsciiString,
    ) -> Result<(), Failure> {
        self.write_bytes(&[b"newmtl ", key.as_bytes(), b"\n"].concat());
        let values = material_values(style, &self.base.default_style)?;
        if values.is_defined {
            self.write_bytes(statements_of(&values).as_bytes());
        }
        let texture = style.base_color_texture();
        if texture.is_some() {
            let path = self.texture_path(&texture);
            if !path.is_empty() {
                self.write_bytes(&[b"map_Kd ", path.as_bytes(), b"\n"].concat());
            }
        }
        Ok(())
    }
}

impl Drop for RwObjObjMaterialMap {
    /// Closes the file, reporting anything that did not reach it.
    fn drop(&mut self) {
        if let Some(mut file) = self.file.take() {
            self.base.is_failed |= file.flush().is_err();
        }
        if self.base.is_failed {
            let name = String::from_utf8_lossy(self.base.file_name.as_bytes()).into_owned();
            Message::send_fail_text(&AsciiString::new(&format!(
                "File cannot be written\n{name}"
            )));
        }
    }
}
