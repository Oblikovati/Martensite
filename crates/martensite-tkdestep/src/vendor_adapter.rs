// SPDX-License-Identifier: GPL-2.0-only
//! Vendor-specific B-rep import adapter mapping external named bodies (such as
//! Inventor/Corliss assemblies) to deterministic named Martensite shapes (#545).
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::ShapeEnum;
use std::collections::BTreeMap;

/// Origin CAD dialect for a vendor import adapter (#545).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum VendorSourceKind {
    /// Autodesk Inventor B-rep / assembly export.
    Inventor,
    /// STEP AP214 exchange file.
    StepAp214,
    /// STEP AP242 exchange file.
    StepAp242,
    /// Neutral B-rep stream.
    Neutral,
}

/// Deterministic named Martensite shape produced by [`VendorShapeAdapter`] (#545).
#[derive(Clone, Debug)]
pub struct NamedVendorShape {
    /// Original vendor body name.
    pub source_name: String,
    /// Sanitized canonical Martensite identifier.
    pub canonical_name: String,
    /// Imported B-rep shape.
    pub shape: Shape,
    /// Number of faces present in `shape`.
    pub face_count: usize,
}

/// Converts external named B-reps into deterministic named Martensite shapes (#545).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VendorShapeAdapter {
    source_kind: VendorSourceKind,
}

impl VendorShapeAdapter {
    /// Creates an adapter for `source_kind`.
    pub fn new(source_kind: VendorSourceKind) -> Self {
        Self { source_kind }
    }

    /// Vendor dialect handled by this adapter.
    pub fn source_kind(&self) -> VendorSourceKind {
        self.source_kind
    }

    /// Converts `(source_name, Shape)` pairs into deterministically ordered [`NamedVendorShape`]s,
    /// rejecting empty or duplicate canonical names and null shapes (#545).
    pub fn convert_named_breps(
        &self,
        entries: &[(String, Shape)],
    ) -> Result<Vec<NamedVendorShape>, Failure> {
        let mut ordered = BTreeMap::new();
        for (source_name, shape) in entries {
            if shape.is_null() {
                return Err(Failure::with_message(
                    "VendorAdapter_NullShape",
                    &format!("vendor entry '{source_name}' has a null shape"),
                ));
            }
            let canonical_name = canonicalize_vendor_name(source_name);
            if canonical_name.is_empty() {
                return Err(Failure::with_message(
                    "VendorAdapter_EmptyName",
                    "vendor entry name must contain at least one alphanumeric character",
                ));
            }
            let face_count = count_faces(shape)?;
            let record = NamedVendorShape {
                source_name: source_name.clone(),
                canonical_name: canonical_name.clone(),
                shape: shape.clone(),
                face_count,
            };
            if ordered.insert(canonical_name.clone(), record).is_some() {
                return Err(Failure::with_message(
                    "VendorAdapter_DuplicateName",
                    &format!("duplicate canonical vendor shape name '{canonical_name}'"),
                ));
            }
        }
        Ok(ordered.into_values().collect())
    }
}

fn canonicalize_vendor_name(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    let mut last_sep = false;
    for ch in raw.trim().chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
            last_sep = false;
        } else if !last_sep && !out.is_empty() {
            out.push('_');
            last_sep = true;
        }
    }
    while out.ends_with('_') {
        out.pop();
    }
    out
}

fn count_faces(shape: &Shape) -> Result<usize, Failure> {
    let mut count = 0usize;
    let mut explorer = Explorer::new(shape, ShapeEnum::Face, ShapeEnum::Shape)?;
    while explorer.more() {
        count += 1;
        explorer.next()?;
    }
    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;
    use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
    use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;

    #[test]
    fn converts_named_inventor_breps_deterministically_and_rejects_duplicates() {
        let box_shape = BRepPrimAPIMakeBox::from_sizes(10.0, 20.0, 30.0)
            .expect("box")
            .shape()
            .expect("shape")
            .clone();
        let adapter = VendorShapeAdapter::new(VendorSourceKind::Inventor);
        let converted = adapter
            .convert_named_breps(&[
                ("Corliss Flywheel #1".to_string(), box_shape.clone()),
                ("Corliss Base-Plate".to_string(), box_shape.clone()),
            ])
            .expect("convert");
        assert_eq!(converted.len(), 2);
        assert_eq!(converted[0].canonical_name, "corliss_base_plate");
        assert_eq!(converted[0].face_count, 6);
        assert_eq!(converted[1].canonical_name, "corliss_flywheel_1");

        let dup = adapter.convert_named_breps(&[
            ("Part A".to_string(), box_shape.clone()),
            ("part_a".to_string(), box_shape),
        ]);
        assert!(dup.is_err());
    }
}
