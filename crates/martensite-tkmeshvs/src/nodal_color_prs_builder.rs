// SPDX-License-Identifier: GPL-2.0-only
//! `MeshVSNodalColorPrsBuilder`: faces and volumes colored by node, by
//! interpolated vertex colors or by a one-dimensional color-map texture. It excludes the
//! elements it draws from the next builders.
mod build;
mod texture;
mod volume;

pub(crate) use build::{color_materials, line_type};
pub use texture::{MeshVSImageTexture2D, get_nearest_pow2};
pub use volume::MeshVSNodalColorScale;

use crate::data_source::{MeshVSDataSource, MeshVSPackedMap};
use crate::flags::MESHVS_DMF_NODAL_COLOR_DATA_PRS;
use crate::mesh::MeshVSMeshSource;
use crate::prs_builder::{MeshVSPrsBuilder, MeshVSPrsBuilderBase, MeshVSPrsBuilderMembers};
use crate::selection_policy::MESHVS_BP_NODAL_COLOR;
use martensite_tkernel::ncollection::{DataMap, Sequence};
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkernel::standard::{Failure, Handle, SharedState, StandardTransient};
use martensite_tkservice::graphic3d::Graphic3dStructureSource;

/// The private fields of `MeshVSNodalColorPrsBuilder`.
pub struct MeshVSNodalColorFields {
    /// The node color map.
    pub node_colors: DataMap<i32, Color>,
    /// Whether to use the texture.
    pub use_texture: bool,
    /// The texture color map.
    pub texture_color_map: Sequence<Color>,
    /// The texture coords.
    pub texture_coords: DataMap<i32, f64>,
    /// The invalid color.
    pub invalid_color: Color,
}

/// The builder of node colors; `MeshVSNodalColorPrsBuilder::with_defaults(&mesh)`.
pub struct MeshVSNodalColorPrsBuilder {
    transient: StandardTransient,
    base: MeshVSPrsBuilderBase,
    /// The private fields; `builder.fields.read().use_texture`.
    pub fields: SharedState<MeshVSNodalColorFields>,
}
martensite_tkernel::impl_transient!(MeshVSNodalColorPrsBuilder, "MeshVSNodalColorPrsBuilder", dyn MeshVSPrsBuilder, transient,
    interfaces: [dyn MeshVSPrsBuilder]);

impl MeshVSNodalColorPrsBuilder {
    /// Colors, gray invalid
    /// color, excluding.
    pub fn new(
        parent: &Handle<dyn MeshVSMeshSource>,
        flags: i32,
        data_source: &Handle<dyn MeshVSDataSource>,
        id: i32,
        priority: i32,
    ) -> Self {
        let builder = Self {
            transient: StandardTransient::default(),
            base: MeshVSPrsBuilderBase::new(parent, flags, data_source, id, priority),
            fields: SharedState::new(MeshVSNodalColorFields {
                node_colors: DataMap::default(),
                use_texture: false,
                texture_color_map: Sequence::default(),
                texture_coords: DataMap::default(),
                invalid_color: Color::named(NamedColor::GRAY),
            }),
        };
        builder.set_excluding(true);
        builder
    }
    /// The default arguments: `MESHVS_DMF_NODAL_COLOR_DATA_PRS`, no data source, a free id
    /// and `MESHVS_BP_NODAL_COLOR`. Example: `MeshVSNodalColorPrsBuilder::with_defaults(&mesh)`.
    pub fn with_defaults(parent: &Handle<dyn MeshVSMeshSource>) -> Self {
        Self::new(
            parent,
            MESHVS_DMF_NODAL_COLOR_DATA_PRS,
            &Handle::default(),
            -1,
            MESHVS_BP_NODAL_COLOR,
        )
    }
    /// `get_colors`: a copy of the node colors; `builder.get_colors().extent()`.
    pub fn get_colors(&self) -> DataMap<i32, Color> {
        let mut copy = DataMap::default();
        copy.assign(&self.fields.read().node_colors);
        copy
    }
    /// `set_colors`; `builder.set_colors(&map)`.
    pub fn set_colors(&self, map: &DataMap<i32, Color>) {
        self.fields.write().node_colors.assign(map);
    }
    /// `has_colors`; `builder.has_colors()`.
    pub fn has_colors(&self) -> bool {
        self.fields.read().node_colors.extent() > 0
    }
    /// `get_color`: the color of a node, the output kept without; `builder.get_color(3, &mut c)`.
    pub fn get_color(&self, id: i32, color: &mut Color) -> bool {
        match self.fields.read().node_colors.seek(&id) {
            Some(found) => {
                *color = *found;
                true
            }
            None => false,
        }
    }
    /// `set_color`: binds or replaces the color of a node; `builder.set_color(3, &red)`.
    pub fn set_color(&self, id: i32, color: &Color) {
        let mut fields = self.fields.write();
        match fields.node_colors.change_find(&id) {
            Ok(slot) => *slot = *color,
            Err(_) => {
                fields.node_colors.bind(id, *color);
            }
        }
    }
    /// `use_texture`: switching on clears the node colors, off clears the color map.
    pub fn use_texture(&self, to_use: bool) {
        let mut fields = self.fields.write();
        fields.use_texture = to_use;
        if to_use {
            fields.node_colors.clear(true);
        } else {
            fields.texture_color_map = Sequence::default();
        }
    }
    /// `is_use_texture`; `builder.is_use_texture()`.
    pub fn is_use_texture(&self) -> bool {
        self.fields.read().use_texture
    }
    /// `set_color_map`; `builder.set_color_map(&colors)`.
    pub fn set_color_map(&self, colors: &Sequence<Color>) {
        let mut copy = Sequence::default();
        for color in colors.iter() {
            copy.append(*color);
        }
        self.fields.write().texture_color_map = copy;
    }
    /// `get_color_map`: a copy of the texture colors; `builder.get_color_map().length()`.
    pub fn get_color_map(&self) -> Sequence<Color> {
        let mut copy = Sequence::default();
        for color in self.fields.read().texture_color_map.iter() {
            copy.append(*color);
        }
        copy
    }
    /// `set_invalid_color`; `builder.set_invalid_color(&gray)`.
    pub fn set_invalid_color(&self, color: &Color) {
        self.fields.write().invalid_color = *color;
    }
    /// `get_invalid_color`; `builder.get_invalid_color()`.
    pub fn get_invalid_color(&self) -> Color {
        self.fields.read().invalid_color
    }
    /// `set_texture_coords`; `builder.set_texture_coords(&map)`.
    pub fn set_texture_coords(&self, map: &DataMap<i32, f64>) {
        self.fields.write().texture_coords.assign(map);
    }
    /// `get_texture_coords`: a copy; `builder.get_texture_coords().extent()`.
    pub fn get_texture_coords(&self) -> DataMap<i32, f64> {
        let mut copy = DataMap::default();
        copy.assign(&self.fields.read().texture_coords);
        copy
    }
    /// `set_texture_coord`: binds the coordinate of a node (`bind` replaces).
    pub fn set_texture_coord(&self, id: i32, coord: f64) {
        self.fields.write().texture_coords.bind(id, coord);
    }
    /// `get_texture_coord`: the coordinate of a node, -1 without; `builder.get_texture_coord(3)`.
    pub fn get_texture_coord(&self, id: i32) -> f64 {
        self.fields
            .read()
            .texture_coords
            .seek(&id)
            .copied()
            .unwrap_or(-1.0)
    }
}

impl MeshVSPrsBuilder for MeshVSNodalColorPrsBuilder {
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
        build::build_nodal_colors(self, prs, ids, ids_to_exclude, is_element, display_mode)
    }
}
