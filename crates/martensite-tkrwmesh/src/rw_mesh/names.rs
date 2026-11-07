//! `RwMesh`: node names of a document.
use super::RwMeshNameFormat;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkernel::tcollection::{AsciiString, ExtendedString};
use martensite_tklcaf::tdata_std::{TDataStdGenericExtString, TDataStdName};
use martensite_tklcaf::tdf::{TdfLabel, TdfTool};

/// The name tools of mesh formats; `RwMesh::read_name_attribute(&label)?`.
pub struct RwMesh;

fn utf8_of(name: &ExtendedString) -> AsciiString {
    AsciiString::from_extended(name, None)
}

fn joined_names(product: &AsciiString, instance: &AsciiString) -> AsciiString {
    let mut text = product.as_bytes().to_vec();
    text.extend_from_slice(b" [");
    text.extend_from_slice(instance.as_bytes());
    text.push(b']');
    AsciiString::from_raw(&text)
}

fn with_entry(name: AsciiString, label: &TdfLabel) -> AsciiString {
    let mut entry = AsciiString::default();
    TdfTool::entry(label, &mut entry);
    joined_names(&name, &entry)
}

fn first_non_empty(first: &TdfLabel, second: &TdfLabel) -> Result<AsciiString, Failure> {
    let name = RwMesh::read_name_attribute(first)?;
    if !name.is_empty() {
        return Ok(name);
    }
    RwMesh::read_name_attribute(second)
}

impl RwMesh {
    /// The name on a label, empty without one; `RwMesh::read_name_attribute(&label)?`.
    pub fn read_name_attribute(label: &TdfLabel) -> Result<AsciiString, Failure> {
        let mut name: Handle<TDataStdName> = Handle::default();
        if !label.find_attribute_as(&TDataStdName::get_id(), &mut name)? {
            return Ok(AsciiString::default());
        }
        Ok(name
            .get()
            .map_or_else(AsciiString::default, |n| utf8_of(&n.get())))
    }
    /// The name of a node by format; an unknown format gives an empty name;
    /// `RwMesh::format_name(RwMeshNameFormat::PRODUCT, &instance, &product)?`.
    pub fn format_name(
        format: RwMeshNameFormat,
        label: &TdfLabel,
        ref_label: &TdfLabel,
    ) -> Result<AsciiString, Failure> {
        match format {
            RwMeshNameFormat::PRODUCT => Self::read_name_attribute(ref_label),
            RwMeshNameFormat::INSTANCE => Self::read_name_attribute(label),
            RwMeshNameFormat::INSTANCE_OR_PRODUCT => first_non_empty(label, ref_label),
            RwMeshNameFormat::PRODUCT_OR_INSTANCE => first_non_empty(ref_label, label),
            RwMeshNameFormat::PRODUCT_AND_INSTANCE => product_and_instance(label, ref_label),
            RwMeshNameFormat::PRODUCT_AND_INSTANCE_AND_OCAF => Ok(with_entry(
                product_and_instance_raw(label, ref_label)?,
                label,
            )),
            _ => Ok(AsciiString::default()),
        }
    }
}

fn product_and_instance(label: &TdfLabel, ref_label: &TdfLabel) -> Result<AsciiString, Failure> {
    product_and_instance_raw(label, ref_label)
}

fn product_and_instance_raw(
    label: &TdfLabel,
    ref_label: &TdfLabel,
) -> Result<AsciiString, Failure> {
    let instance = RwMesh::read_name_attribute(label)?;
    let product = RwMesh::read_name_attribute(ref_label)?;
    if !instance.is_empty() && product != instance {
        return Ok(joined_names(&product, &instance));
    }
    Ok(product)
}
