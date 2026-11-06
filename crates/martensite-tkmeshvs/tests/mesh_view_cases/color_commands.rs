// SPDX-License-Identifier: GPL-2.0-only
//! The color steps of the cases: a color per element, a color per node, or a color per node
//! read from a texture along the X extent of the mesh.
use crate::mesh_commands::DisplayedMesh;
use crate::view_host::ViewHost;
use MeshVSDrawerAttribute as Key;
use martensite_tkernel::ncollection::{Array1, DataMap, Sequence};
use martensite_tkernel::precision;
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkernel::standard::{Handle, RuntimeTyped};
use martensite_tkmeshvs::flags::*;
use martensite_tkmeshvs::selection_policy::{MESHVS_BP_ELEM_COLOR, MESHVS_BP_NODAL_COLOR};
use martensite_tkmeshvs::*;

/// The color an id names: its value wrapped below the white enumerator.
fn color_of_id(id: i32) -> Color {
    let name = NamedColor::from_index(id % NamedColor::WHITE.index()).unwrap();
    Color::named(name)
}

/// Removes one builder of each color kind per pass, for as many passes as builders remain.
fn remove_color_builders(mesh: &DisplayedMesh) {
    let mut pass = 0;
    while pass < mesh.mesh.get_builders_count() {
        for kind in [
            MeshVSElementalColorPrsBuilder::static_type(),
            MeshVSNodalColorPrsBuilder::static_type(),
        ] {
            let builder = mesh.mesh.find_builder(kind);
            if let Some(builder) = builder.get() {
                mesh.mesh.remove_builder_by_id(builder.get_id());
            }
        }
        pass += 1;
    }
}

fn finish_colors(host: &ViewHost, mesh: &DisplayedMesh, reflection: bool) {
    mesh.mesh
        .get_drawer()
        .set_boolean(Key::ColorReflection as i32, reflection);
    host.context.redisplay(&mesh.object, true, false).unwrap();
}

/// A color per element.
/// Example: `color_elements(&host, &m, false)`.
pub fn color_elements(host: &ViewHost, mesh: &DisplayedMesh, reflection: bool) {
    remove_color_builders(mesh);
    let builder = MeshVSElementalColorPrsBuilder::new(
        &mesh.parent,
        MESHVS_DMF_ELEMENTAL_COLOR_DATA_PRS | MESHVS_DMF_BASE_MODE_MASK,
        &Handle::default(),
        -1,
        MESHVS_BP_ELEM_COLOR,
    );
    for id in mesh.mesh.get_data_source().get_all_elements().iter() {
        builder.set_color1(id, &color_of_id(id));
    }
    mesh.mesh.add_builder(&Handle::new(builder).cast(), true);
    finish_colors(host, mesh, reflection);
}

fn new_nodal_builder(mesh: &DisplayedMesh) -> Handle<MeshVSNodalColorPrsBuilder> {
    Handle::new(MeshVSNodalColorPrsBuilder::new(
        &mesh.parent,
        MESHVS_DMF_NODAL_COLOR_DATA_PRS | MESHVS_DMF_BASE_MODE_MASK,
        &Handle::default(),
        -1,
        MESHVS_BP_NODAL_COLOR,
    ))
}

/// A color per node. The builder goes on the mesh both before and after it is filled, so the mesh
/// holds it twice, which the reference images include.
/// Example: `color_nodes(&host, &m, true)`.
pub fn color_nodes(host: &ViewHost, mesh: &DisplayedMesh, reflection: bool) {
    remove_color_builders(mesh);
    let builder = new_nodal_builder(mesh);
    mesh.mesh.add_builder(&builder.clone().cast(), true);
    for id in mesh.mesh.get_data_source().get_all_nodes().iter() {
        builder.set_color(id, &color_of_id(id));
    }
    mesh.mesh.add_builder(&builder.cast(), true);
    finish_colors(host, mesh, reflection);
}

/// Each node's place between the least and the greatest X of the mesh, from 0 to 1.
fn x_scale_of_nodes(mesh: &DisplayedMesh) -> DataMap<i32, f64> {
    let source = mesh.mesh.get_data_source();
    let source = source.get().unwrap();
    let [min_x, _, _, max_x, _, _] = source.get_bounding_box().get().unwrap();
    let delta = max_x - min_x;
    let mut scale = DataMap::default();
    for id in source.get_all_nodes().iter() {
        let mut coords = Array1::<f64>::with_bounds(1, 3).unwrap();
        let (mut count, mut kind) = (0, MeshVSEntityType::default());
        source.get_geom(id, false, &mut coords, &mut count, &mut kind);
        let value = if delta > precision::confusion() {
            (coords[1] - min_x) / delta
        } else {
            0.
        };
        scale.bind(id, value);
    }
    scale
}

/// A red, yellow and blue ramp along X, read from a
/// texture; the builder is added before and after being filled, as for `nodal`.
/// Example: `color_nodes_by_texture(&host, &m, true)`.
pub fn color_nodes_by_texture(host: &ViewHost, mesh: &DisplayedMesh, reflection: bool) {
    remove_color_builders(mesh);
    let builder = new_nodal_builder(mesh);
    mesh.mesh.add_builder(&builder.clone().cast(), true);
    builder.use_texture(true);
    let mut ramp = Sequence::default();
    for name in [NamedColor::RED, NamedColor::YELLOW, NamedColor::BLUE1] {
        ramp.append(Color::named(name));
    }
    builder.set_color_map(&ramp);
    builder.set_invalid_color(&Color::named(NamedColor::BLACK));
    builder.set_texture_coords(&x_scale_of_nodes(mesh));
    mesh.mesh.add_builder(&builder.cast(), true);
    finish_colors(host, mesh, reflection);
}
