//! `BOPAlgoRemoveFeatures`: removes faces (features) from solids by extending the
//! adjacent faces over the gap and rebuilding the solids.
#[path = "remove_features/entry.rs"]
mod entry;
#[path = "remove_features/feature.rs"]
mod feature;
#[path = "remove_features/fill_gap.rs"]
mod fill_gap;
#[path = "remove_features/helpers.rs"]
mod helpers;
#[path = "remove_features/local_progress.rs"]
mod local_progress;
#[path = "remove_features/solids.rs"]
mod solids;
use super::alerts::alert;
use super::{
    AlertEmptyShape, AlertNoFacesToRemove, AlertTooFewArguments, AlertUnsupportedType, BuilderShape,
};
use crate::bop_tools::treat_compound;
use helpers::{compound_of, explore, make_removed};
use martensite_tkbrep::brep::BRepBuilder;
use martensite_tkbrep::brep_tools::History;
use martensite_tkbrep::topods::{CompSolid, Shape};
use martensite_tkbrep::toptools::{IndexedMapOfShape, MapOfShape};
use martensite_tkernel::message::AlertHandle;
#[cfg(test)]
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::ncollection::List;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::top_abs::ShapeEnum;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

/// The feature-removal algorithm: each connected block of the faces to remove is a feature,
/// whose gap is filled by the extended adjacent faces before the solids are rebuilt; the
/// history and the simplification of the result follow.
///
/// ```
/// use martensite_tkbo::bop_algo::BOPAlgoRemoveFeatures;
/// let mut remover = BOPAlgoRemoveFeatures::new();
/// assert!(remover.input_shape().is_null() && remover.faces_to_remove().is_empty());
/// ```
#[derive(Default)]
pub struct BOPAlgoRemoveFeatures {
    base: BuilderShape,
    input_shape: Shape,
    faces_to_remove: List<Shape>,
    /// The connected blocks of faces to remove (`myFeatures`).
    features: Vec<Shape>,
    /// Every sub-shape of the input shape (`myInputsMap`).
    inputs_map: IndexedMapOfShape,
}

impl Deref for BOPAlgoRemoveFeatures {
    type Target = BuilderShape;
    fn deref(&self) -> &BuilderShape {
        &self.base
    }
}

impl DerefMut for BOPAlgoRemoveFeatures {
    fn deref_mut(&mut self) -> &mut BuilderShape {
        &mut self.base
    }
}

impl BOPAlgoRemoveFeatures {
    /// An empty algorithm (`BOPAlgoRemoveFeatures()`); `BOPAlgoRemoveFeatures::new()`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the shape to remove the features from (`SetShape`); `remover.set_shape(&solid)`.
    pub fn set_shape(&mut self, shape: &Shape) {
        self.input_shape = shape.clone();
    }

    /// The shape to remove the features from (`InputShape`); `remover.input_shape()`.
    pub fn input_shape(&self) -> &Shape {
        &self.input_shape
    }

    /// Adds a face (or a shape with faces) to remove (`AddFaceToRemove`);
    /// `remover.add_face_to_remove(&face)`.
    pub fn add_face_to_remove(&mut self, face: &Shape) {
        self.faces_to_remove.append(face.clone());
    }

    /// Adds faces to remove (`AddFacesToRemove`); `remover.add_faces_to_remove(&faces)`.
    pub fn add_faces_to_remove(&mut self, faces: &List<Shape>) {
        for face in faces.iter() {
            self.faces_to_remove.append(face.clone());
        }
    }

    /// The faces to remove (`FacesToRemove`); `remover.faces_to_remove().extent()`.
    pub fn faces_to_remove(&self) -> &List<Shape> {
        &self.faces_to_remove
    }

    /// Forgets the inputs, the result and the history (`Clear`); `remover.clear()`.
    pub fn clear(&mut self) {
        self.base.clear();
        self.input_shape.nullify();
        self.base.shape.nullify();
        self.faces_to_remove.clear();
        self.features.clear();
        self.inputs_map.clear(false);
    }

    /// Accepts a solid, a compsolid, or a compound with solids, whose other shapes only
    /// warn and are removed from the history (`CheckData`).
    fn check_data(&mut self) -> Result<(), Failure> {
        self.base.shape = self.input_shape.clone();
        match self.input_shape.shape_type()? {
            ShapeEnum::Solid | ShapeEnum::CompSolid => Ok(()),
            ShapeEnum::Compound => self.check_compound(),
            _ => {
                self.base.options.add_error(&alert(AlertTooFewArguments));
                Ok(())
            }
        }
    }

    fn check_compound(&mut self) -> Result<(), Failure> {
        let (mut shapes, mut fence) = (Vec::new(), MapOfShape::default());
        treat_compound(&self.input_shape, &mut shapes, Some(&mut fence))?;
        if shapes.is_empty() {
            let alert: AlertHandle = Arc::new(AlertEmptyShape::new(&self.input_shape));
            self.base.options.add_error(&alert);
            return Ok(());
        }
        let mut solids = Vec::new();
        let mut others = Vec::new();
        for shape in shapes {
            match shape.shape_type()? {
                ShapeEnum::Solid | ShapeEnum::CompSolid => solids.push(shape),
                _ => others.push(shape),
            }
        }
        if solids.is_empty() {
            self.base.options.add_error(&alert(AlertTooFewArguments));
            return Ok(());
        }
        if others.is_empty() {
            return Ok(());
        }
        for other in &others {
            let alert: AlertHandle = Arc::new(AlertUnsupportedType::new(other));
            self.base.options.add_warning(&alert);
        }
        self.base.shape = compound_of(&solids)?;
        if let Some(history) = self.filled_history().get() {
            make_removed(&others, history, &IndexedMapOfShape::default())?;
        }
        Ok(())
    }

    /// The input faces requested for removal, grouped into connected blocks (the features);
    /// an error when none belongs to the input (`PrepareFeatures`).
    #[cfg(test)]
    fn prepare_features(&mut self) -> Result<(), Failure> {
        let unbound = ProgressRange::default();
        self.prepare_features_progress_raw(&unbound)
    }

    /// Restores the type of the input: its solid, or a compound or compsolid of the result
    /// solids (`PostTreat`).
    fn post_treat(&mut self) -> Result<(), Failure> {
        let input_type = self.input_shape.shape_type()?;
        if self.base.shape.shape_type()? == input_type {
            return Ok(());
        }
        let solids = explore(&self.base.shape, ShapeEnum::Solid)?;
        if input_type == ShapeEnum::Solid {
            // TopExpExplorer::Current raises StandardNoSuchObject on an exhausted explorer.
            self.base.shape = solids.first().cloned().ok_or_else(no_solid)?;
            return Ok(());
        }
        if input_type == ShapeEnum::Compound {
            self.base.shape = compound_of(&solids)?;
            return Ok(());
        }
        let builder = BRepBuilder::default();
        let mut compsolid = CompSolid::default();
        builder.make_comp_solid(&mut compsolid);
        let mut result = compsolid.into_shape();
        for solid in &solids {
            builder.add(&mut result, solid)?;
        }
        self.base.shape = result;
        Ok(())
    }

    /// The history handle while history filling is on (`HasHistory()` and `myHistory`); a
    /// null handle otherwise.
    fn filled_history(&self) -> Handle<History> {
        if self.base.fill_history {
            self.base.history.clone()
        } else {
            Handle::default()
        }
    }
}

/// The failure of the post-treatment of a result without solid.
fn no_solid() -> Failure {
    Failure::with_type(
        "Standard_NoSuchObject",
        "BOPAlgoRemoveFeatures::PostTreat: the result has no solid; expected one for a solid input",
        "",
    )
}
