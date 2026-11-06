// SPDX-License-Identifier: GPL-2.0-only
//! `MeshVSVectorPrsBuilder`: the vectors of nodes and elements as lines with
//! an arrow head (eight points joined by segments) or, in the simple mode, a thick arrow line.
//! The lengths map the magnitudes linearly to at most `MeshVSDrawerAttribute::VectorMaxLength`.
mod drawing;

pub use drawing::calculate_arrow;

use crate::MeshVSDrawerAttribute as Key;
use crate::data_source::{MeshVSDataSource, MeshVSPackedMap};
use crate::flags::*;
use crate::mesh::MeshVSMeshSource;
use crate::prs_builder::{MeshVSPrsBuilder, MeshVSPrsBuilderBase, MeshVSPrsBuilderMembers};
use crate::selection_policy::MESHVS_BP_VECTOR;
use martensite_tkernel::ncollection::DataMap;
use martensite_tkernel::quantity::Color;
use martensite_tkernel::standard::{Failure, Handle, SharedState, StandardTransient};
use martensite_tkmath::gp::Vector3;
use martensite_tkservice::graphic3d::Graphic3dStructureSource;

/// The vectors and simple-mode parameters of `MeshVSVectorPrsBuilder`.
pub struct MeshVSVectorFields {
    /// Whether it is simple presentation.
    pub is_simple_prs: bool,
    /// The simple width parameter.
    pub simple_width_prm: f64,
    /// The simple start parameter.
    pub simple_start_prm: f64,
    /// The simple end parameter.
    pub simple_end_prm: f64,
    /// The node vector map.
    pub node_vectors: DataMap<i32, Vector3>,
    /// The element vector map.
    pub elem_vectors: DataMap<i32, Vector3>,
}

impl MeshVSVectorFields {
    pub(crate) fn of(&self, is_element: bool) -> &DataMap<i32, Vector3> {
        if is_element {
            &self.elem_vectors
        } else {
            &self.node_vectors
        }
    }
}

/// The builder of mesh vectors; `MeshVSVectorPrsBuilder::with_defaults(&mesh, 10.0, &color)`.
pub struct MeshVSVectorPrsBuilder {
    transient: StandardTransient,
    base: MeshVSPrsBuilderBase,
    /// The vector fields; `builder.fields.read().is_simple_prs`.
    pub fields: SharedState<MeshVSVectorFields>,
}
martensite_tkernel::impl_transient!(MeshVSVectorPrsBuilder, "MeshVSVectorPrsBuilder", dyn MeshVSPrsBuilder, transient,
    interfaces: [dyn MeshVSPrsBuilder]);

impl MeshVSVectorPrsBuilder {
    /// A builder of a parent mesh: the maximal length, the color and the arrow part 0.1 go into the drawer
    /// of the builder (the mesh drawer).
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        parent: &Handle<dyn MeshVSMeshSource>,
        max_length: f64,
        vector_color: &Color,
        flags: i32,
        data_source: &Handle<dyn MeshVSDataSource>,
        id: i32,
        priority: i32,
        is_simple_prs: bool,
    ) -> Self {
        let builder = Self {
            transient: StandardTransient::default(),
            base: MeshVSPrsBuilderBase::new(parent, flags, data_source, id, priority),
            fields: SharedState::new(MeshVSVectorFields {
                is_simple_prs,
                simple_width_prm: 2.5,
                simple_start_prm: 0.85,
                simple_end_prm: 0.95,
                node_vectors: DataMap::default(),
                elem_vectors: DataMap::default(),
            }),
        };
        if let Some(drawer) = builder.get_drawer().get() {
            drawer.set_double(Key::VectorMaxLength as i32, max_length);
            drawer.set_color(Key::VectorColor as i32, vector_color);
            drawer.set_double(Key::VectorArrowPart as i32, 0.1);
        }
        builder
    }
    /// The default arguments: `MESHVS_DMF_VECTOR_DATA_PRS`, no data source, a free id,
    /// `MESHVS_BP_VECTOR` and the arrow mode. Example: `MeshVSVectorPrsBuilder::with_defaults(&mesh, 10.0, &color)`.
    pub fn with_defaults(
        parent: &Handle<dyn MeshVSMeshSource>,
        max_length: f64,
        vector_color: &Color,
    ) -> Self {
        Self::new(
            parent,
            max_length,
            vector_color,
            MESHVS_DMF_VECTOR_DATA_PRS,
            &Handle::default(),
            -1,
            MESHVS_BP_VECTOR,
            false,
        )
    }
    /// `get_vectors`: a copy of the node or element vectors; `builder.get_vectors(false)`.
    pub fn get_vectors(&self, is_element: bool) -> DataMap<i32, Vector3> {
        let mut copy = DataMap::default();
        copy.assign(self.fields.read().of(is_element));
        copy
    }
    /// `set_vectors`; `builder.set_vectors(true, &map)`.
    pub fn set_vectors(&self, is_element: bool, map: &DataMap<i32, Vector3>) {
        let mut fields = self.fields.write();
        let target = if is_element {
            &mut fields.elem_vectors
        } else {
            &mut fields.node_vectors
        };
        target.assign(map);
    }
    /// `has_vectors`; `builder.has_vectors(false)`.
    pub fn has_vectors(&self, is_element: bool) -> bool {
        self.fields.read().of(is_element).extent() > 0
    }
    /// `get_vector`: the vector of an id, the output kept without; `builder.get_vector(false, 3, &mut v)`.
    pub fn get_vector(&self, is_element: bool, id: i32, vect: &mut Vector3) -> bool {
        match self.fields.read().of(is_element).seek(&id) {
            Some(found) => {
                *vect = *found;
                true
            }
            None => false,
        }
    }
    /// `set_vector`: binds or replaces the vector of an id; `builder.set_vector(false, 3, &v)`.
    pub fn set_vector(&self, is_element: bool, id: i32, vect: &Vector3) {
        let mut fields = self.fields.write();
        let map = if is_element {
            &mut fields.elem_vectors
        } else {
            &mut fields.node_vectors
        };
        match map.change_find(&id) {
            Ok(slot) => *slot = *vect,
            Err(_) => {
                map.bind(id, *vect);
            }
        }
    }
    /// `get_min_max_vector_value`: the extreme magnitudes, the outputs kept without vectors.
    /// Example: `builder.get_min_max_vector_value(false, &mut min, &mut max)`.
    pub fn get_min_max_vector_value(
        &self,
        is_element: bool,
        min_value: &mut f64,
        max_value: &mut f64,
    ) {
        let fields = self.fields.read();
        let mut values = fields.of(is_element).values().map(Vector3::magnitude);
        let Some(first) = values.next() else {
            return;
        };
        (*min_value, *max_value) = (first, first);
        for value in std::iter::once(first).chain(values) {
            if *min_value > value {
                *min_value = value;
            }
            if *max_value < value {
                *max_value = value;
            }
        }
    }
    /// `set_simple_prs_mode`; `builder.set_simple_prs_mode(true)`.
    pub fn set_simple_prs_mode(&self, is_simple_arrow: bool) {
        self.fields.write().is_simple_prs = is_simple_arrow;
    }
    /// `set_simple_prs_params`: the line width factor and the arrow start and end parameters.
    pub fn set_simple_prs_params(&self, line_width: f64, start: f64, end: f64) {
        let mut fields = self.fields.write();
        (
            fields.simple_width_prm,
            fields.simple_start_prm,
            fields.simple_end_prm,
        ) = (line_width, start, end);
    }
}

impl MeshVSPrsBuilder for MeshVSVectorPrsBuilder {
    fn prs_builder_base(&self) -> &MeshVSPrsBuilderBase {
        &self.base
    }
    fn build(
        &self,
        prs: &Handle<dyn Graphic3dStructureSource>,
        ids: &MeshVSPackedMap,
        ids_to_exclude: &mut MeshVSPackedMap,
        is_element: bool,
        display_mode: i32,
    ) -> Result<(), Failure> {
        drawing::build_vectors(self, prs, ids, ids_to_exclude, is_element, display_mode)
    }
}
