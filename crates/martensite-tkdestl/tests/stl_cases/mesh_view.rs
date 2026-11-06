//! The mesh display commands of the script cases, on the real viewer.
use super::real_view::ViewerHost;
use super::stl_data_source::StlDataSource;
use martensite_tkdestl::rw_stl::RwStl;
use martensite_tkernel::{
    message::ProgressRange,
    ncollection::{Array1, DataMap, Sequence, Vec2, Vec4},
    osd::CadPath,
    quantity::{Color, ColorSpace, NamedColor},
    standard::{Handle, RuntimeTyped},
    tcol_std::HPackedMapOfInteger,
    tcollection::AsciiString,
};
use martensite_tkmath::{
    gp::{Point2, Transform3, Vector3},
    poly::Triangulation,
    toploc::Location,
};
use martensite_tkmeshvs::{
    MeshVSDataSource, MeshVSElementalColorPrsBuilder, MeshVSMesh, MeshVSMeshEntityOwner,
    MeshVSMeshMembers as _, MeshVSMeshPrsBuilder, MeshVSMeshSource, MeshVSNodalColorPrsBuilder,
    MeshVSPrsBuilder, MeshVSPrsBuilderMembers as _, MeshVSTextPrsBuilder, MeshVSVectorPrsBuilder,
    flags::{MeshVSDrawerAttribute as Key, *},
    selection_policy::{MESHVS_BP_ELEM_COLOR, MESHVS_BP_NODAL_COLOR, MESHVS_BP_VECTOR},
};
use martensite_tkservice::graphic3d::Graphic3dClipPlane;
use martensite_tkv3d::ais::{AisInteractiveObject, AisSelectionScheme};
use martensite_tkv3d::v3d::V3dView;

/// A mesh on the view.
pub struct ShownMesh {
    pub mesh: Handle<MeshVSMesh>,
    pub object: Handle<dyn AisInteractiveObject>,
    pub triangulation: Handle<Triangulation>,
}

fn owner_handle(mesh: &Handle<MeshVSMesh>) -> Handle<dyn MeshVSMeshSource> {
    mesh.clone().cast()
}

/// Nodes hidden and not selectable, no active selection, view fitted.
pub fn mesh_from_stl(
    host: &ViewerHost,
    file: &AsciiString,
    progress: &ProgressRange<'_>,
) -> ShownMesh {
    let path = CadPath::new(file.as_bytes());
    let triangulation = RwStl::read_path(&path, progress).unwrap();
    let source: Handle<dyn MeshVSDataSource> =
        Handle::new(StlDataSource::new(&triangulation)).cast();
    let mesh = MeshVSMesh::new(false).unwrap();
    mesh.set_data_source(&source);
    let builder = MeshVSMeshPrsBuilder::with_defaults(&owner_handle(&mesh));
    mesh.add_builder(&Handle::new(builder).cast(), true);
    let drawer = mesh.get_drawer();
    drawer.set_color(Key::EdgeColor as i32, &Color::named(NamedColor::YELLOW));
    let nodes = hidden_nodes(triangulation.get().unwrap().nb_nodes());
    mesh.set_hidden_nodes(&nodes);
    mesh.set_selectable_nodes(&nodes);
    let object: Handle<dyn AisInteractiveObject> = mesh.clone().cast();
    host.context.display(&object, true).unwrap();
    host.context.deactivate(&object).unwrap();
    host.view.fit_all(0.01, true).unwrap();
    ShownMesh {
        mesh,
        object,
        triangulation,
    }
}

fn hidden_nodes(nb_nodes: i32) -> Handle<HPackedMapOfInteger> {
    let nodes = Handle::new(HPackedMapOfInteger::default());
    for index in 1..=nb_nodes {
        nodes.change_map().add(index);
    }
    nodes
}

/// `vselmode name mode 1`.
pub fn select_mode_on(host: &ViewerHost, object: &Handle<dyn AisInteractiveObject>, mode: i32) {
    host.context.activate(object, mode, false).unwrap();
}

/// `vselmode name mode 0`.
pub fn select_mode_off(host: &ViewerHost, object: &Handle<dyn AisInteractiveObject>, mode: i32) {
    host.context.deactivate_mode(object, mode).unwrap();
}

/// A click at a pixel.
pub fn select_point(host: &ViewerHost, x: i32, y: i32, scheme: AisSelectionScheme) {
    host.context.move_to(x, y, &host.view, false).unwrap();
    host.context.select_detected(scheme).unwrap();
    host.context.update_current_viewer();
}

/// The rubber band of a drag in the direction that allows no overlap.
pub fn select_rectangle(host: &ViewerHost, first: [i32; 2], second: [i32; 2]) {
    let minimum = Vec2::new(first[0].min(second[0]), first[1].min(second[1]));
    let maximum = Vec2::new(first[0].max(second[0]), first[1].max(second[1]));
    host.context
        .select_rectangle(minimum, maximum, &host.view, AisSelectionScheme::Replace)
        .unwrap();
    host.context.update_current_viewer();
}

/// A closed polygon of pixels.
pub fn select_polygon(host: &ViewerHost, points: &[[i32; 2]]) {
    let mut polygon = Array1::with_bounds(1, points.len() as i32).unwrap();
    for (index, [x, y]) in points.iter().enumerate() {
        let pixel = Point2::new(f64::from(*x), f64::from(*y));
        polygon.set_value(index as i32 + 1, pixel).unwrap();
    }
    host.context
        .select_polygon(&polygon, &host.view, AisSelectionScheme::Replace)
        .unwrap();
    host.context.update_current_viewer();
}

/// The node and element identifiers of the selected mesh owners.
fn selected_entities(host: &ViewerHost) -> Vec<(bool, i32)> {
    let mut entities = Vec::new();
    host.context.init_selected();
    while host.context.more_selected() {
        let owner = host
            .context
            .selected_owner()
            .unwrap()
            .cast::<MeshVSMeshEntityOwner>();
        if let Some(owner) = owner.get() {
            entities.push((owner.kind() == MESHVS_ET_NODE, owner.id()));
        }
        host.context.next_selected();
    }
    entities
}

/// Adds the selected entities to what is hidden.
pub fn hide_selected(host: &ViewerHost, shown: &ShownMesh) {
    let nodes = owned_or_new(shown.mesh.get_hidden_nodes());
    let elements = owned_or_new(shown.mesh.get_hidden_elems());
    for (is_node, id) in selected_entities(host) {
        if is_node {
            nodes.change_map().add(id);
        } else {
            elements.change_map().add(id);
        }
    }
    host.context.clear_selected(false).unwrap();
    shown.mesh.set_hidden_nodes(&nodes);
    shown.mesh.set_hidden_elems(&elements);
    host.context.redisplay(&shown.object, true, false).unwrap();
}

fn owned_or_new(map: Handle<HPackedMapOfInteger>) -> Handle<HPackedMapOfInteger> {
    if map.is_null() {
        Handle::new(HPackedMapOfInteger::default())
    } else {
        map
    }
}

/// Hides everything but the selected entities.
pub fn show_only_selected(host: &ViewerHost, shown: &ShownMesh) {
    let source = shown.mesh.get_data_source();
    let nodes = Handle::new(HPackedMapOfInteger::from_map_ref(&source.get_all_nodes()));
    let elements = Handle::new(HPackedMapOfInteger::from_map_ref(
        &source.get_all_elements(),
    ));
    for (is_node, id) in selected_entities(host) {
        if is_node {
            nodes.change_map().remove(id);
        } else {
            elements.change_map().remove(id);
        }
    }
    shown.mesh.set_hidden_nodes(&nodes);
    shown.mesh.set_hidden_elems(&elements);
    host.context.redisplay(&shown.object, true, false).unwrap();
}

/// Nothing hidden.
pub fn show_all(host: &ViewerHost, shown: &ShownMesh) {
    shown
        .mesh
        .set_hidden_nodes(&Handle::new(HPackedMapOfInteger::default()));
    shown
        .mesh
        .set_hidden_elems(&Handle::new(HPackedMapOfInteger::default()));
    host.context.redisplay(&shown.object, true, false).unwrap();
}

/// Erases the mesh from the view.
pub fn erase(host: &ViewerHost, shown: &ShownMesh) {
    host.context.erase(&shown.object, true).unwrap();
}

/// `vsetdispmode name mode`.
pub fn display_mode(host: &ViewerHost, shown: &ShownMesh, mode: i32) {
    host.context
        .set_display_mode(&shown.object, mode, true)
        .unwrap();
}

/// `mesh_edge_width name width`.
pub fn edge_width(host: &ViewerHost, shown: &ShownMesh, width: f64) {
    shown
        .mesh
        .get_drawer()
        .set_double(Key::EdgeWidth as i32, width);
    host.context.redisplay(&shown.object, true, false).unwrap();
}

/// Sets whether the mesh counts as closed.
pub fn closed(host: &ViewerHost, shown: &ShownMesh, is_closed: bool) {
    shown
        .mesh
        .get_drawer()
        .set_boolean(Key::SupressBackFaces as i32, is_closed);
    host.context.redisplay(&shown.object, true, false).unwrap();
}

fn remove_color_builders(mesh: &Handle<MeshVSMesh>) {
    for _ in 0..mesh.get_builders_count() {
        for kind in [
            MeshVSElementalColorPrsBuilder::static_type(),
            MeshVSNodalColorPrsBuilder::static_type(),
        ] {
            let builder = mesh.find_builder(kind);
            if let Some(builder) = builder.get() {
                mesh.remove_builder_by_id(builder.get_id());
            }
        }
    }
}

/// The colour of an identifier in the coloured modes: its name's position modulo `WHITE`.
fn identifier_color(id: i32) -> Color {
    Color::named(NamedColor::from_index(id % NamedColor::WHITE.index()).unwrap())
}

const NODAL_FLAGS: i32 = MESHVS_DMF_NODAL_COLOR_DATA_PRS | MESHVS_DMF_BASE_MODE_MASK;
const ELEMENTAL_FLAGS: i32 = MESHVS_DMF_ELEMENTAL_COLOR_DATA_PRS | MESHVS_DMF_BASE_MODE_MASK;

/// `elem1`, `elem2`, `nodal` or `nodaltex`.
pub fn colors(host: &ViewerHost, shown: &ShownMesh, mode: &str, reflection: bool) {
    remove_color_builders(&shown.mesh);
    match mode {
        "elem1" | "elem2" => add_elemental_colors(shown, mode),
        "nodal" => add_nodal_colors(shown),
        "nodaltex" => add_texture_colors(shown),
        other => panic!("meshcolors mode {other:?}; expected elem1, elem2, nodal or nodaltex"),
    }
    shown
        .mesh
        .get_drawer()
        .set_boolean(Key::ColorReflection as i32, reflection);
    host.context.redisplay(&shown.object, true, false).unwrap();
}

fn add_elemental_colors(shown: &ShownMesh, mode: &str) {
    let builder = MeshVSElementalColorPrsBuilder::new(
        &owner_handle(&shown.mesh),
        ELEMENTAL_FLAGS,
        &Handle::default(),
        -1,
        MESHVS_BP_ELEM_COLOR,
    );
    for id in shown.mesh.get_data_source().get_all_elements().iter() {
        if mode == "elem1" {
            builder.set_color1(id, &identifier_color(id));
        } else {
            builder.set_color2_colors(
                id,
                &Color::named(NamedColor::BLUE1),
                &Color::named(NamedColor::RED1),
            );
        }
    }
    shown
        .mesh
        .add_builder(&Handle::new(builder).cast::<dyn MeshVSPrsBuilder>(), true);
}

fn nodal_builder(shown: &ShownMesh) -> Handle<MeshVSNodalColorPrsBuilder> {
    let builder = MeshVSNodalColorPrsBuilder::new(
        &owner_handle(&shown.mesh),
        NODAL_FLAGS,
        &Handle::default(),
        -1,
        MESHVS_BP_NODAL_COLOR,
    );
    let builder = Handle::new(builder);
    shown.mesh.add_builder(&builder.clone().cast(), true);
    builder
}

fn add_nodal_colors(shown: &ShownMesh) {
    let builder = nodal_builder(shown);
    for id in shown.mesh.get_data_source().get_all_nodes().iter() {
        builder.set_color(id, &identifier_color(id));
    }
    shown.mesh.add_builder(&builder.clone().cast(), true);
}

/// Colours interpolated from the lowest to the highest X.
fn add_texture_colors(shown: &ShownMesh) {
    let builder = nodal_builder(shown);
    builder.use_texture(true);
    let mut map = Sequence::default();
    for name in [NamedColor::RED, NamedColor::YELLOW, NamedColor::BLUE1] {
        map.append(Color::named(name));
    }
    let source = shown.mesh.get_data_source();
    let [x_min, _, _, x_max, _, _] = source.get_bounding_box().get().unwrap();
    let delta = x_max - x_min;
    let mesh = shown.triangulation.get().unwrap();
    let mut scale = DataMap::default();
    for id in source.get_all_nodes().iter() {
        let x = mesh.node(id).unwrap().x();
        let value = if delta > 1.0e-7 {
            (x - x_min) / delta
        } else {
            0.0
        };
        scale.bind(id, value);
    }
    builder.set_color_map(&map);
    builder.set_invalid_color(&Color::named(NamedColor::BLACK));
    builder.set_texture_coords(&scale);
    shown.mesh.add_builder(&builder.clone().cast(), true);
}

/// `meshvectors name -mode elem|nodal -maxlen -color -arrowpart -issimple`.
pub fn vectors(
    host: &ViewerHost,
    shown: &ShownMesh,
    is_element: bool,
    max_length: f64,
    color: NamedColor,
    arrow_part: f64,
    is_simple: bool,
) {
    let found = shown
        .mesh
        .find_builder(MeshVSVectorPrsBuilder::static_type());
    if let Some(found) = found.get() {
        shown.mesh.remove_builder_by_id(found.get_id());
    }
    let builder = MeshVSVectorPrsBuilder::new(
        &owner_handle(&shown.mesh),
        max_length,
        &Color::named(color),
        MESHVS_DMF_VECTOR_DATA_PRS,
        &Handle::default(),
        -1,
        MESHVS_BP_VECTOR,
        is_simple,
    );
    let source = shown.mesh.get_data_source();
    let ids = if is_element {
        source.get_all_elements()
    } else {
        source.get_all_nodes()
    };
    let mesh = shown.triangulation.get().unwrap();
    for id in ids.iter() {
        builder.set_vector(is_element, id, &unit_vector(&*source, mesh, is_element, id));
    }
    shown
        .mesh
        .add_builder(&Handle::new(builder).cast::<dyn MeshVSPrsBuilder>(), false);
    shown
        .mesh
        .get_drawer()
        .set_double(Key::VectorArrowPart as i32, arrow_part);
    host.context.redisplay(&shown.object, true, false).unwrap();
}

/// A face normal or a node's position, normalized; (0, 0, 1) when it has no length.
fn unit_vector(
    source: &dyn MeshVSDataSource,
    mesh: &Triangulation,
    is_element: bool,
    id: i32,
) -> Vector3 {
    let (mut x, mut y, mut z) = (0.0, 0.0, 0.0);
    if is_element {
        source.get_normal(id, 3, &mut x, &mut y, &mut z);
    } else {
        let node = mesh.node(id).unwrap();
        (x, y, z) = (node.x(), node.y(), node.z());
    }
    let vector = Vector3::new(x, y, z);
    if vector.magnitude() < f64::MIN_POSITIVE {
        return Vector3::new(0., 0., 1.);
    }
    vector.normalized().unwrap()
}

/// Labels the triangles with their identifiers, at height 20, in yellow.
pub fn text_labels(shown: &ShownMesh) {
    let builder = MeshVSTextPrsBuilder::with_defaults(
        &owner_handle(&shown.mesh),
        20.0,
        &Color::named(NamedColor::YELLOW),
    );
    let mut labels = DataMap::default();
    for id in 1..=shown.mesh.get_data_source().get_all_elements().extent() {
        labels.bind(id, AsciiString::from_integer(id));
    }
    builder.set_texts(true, &labels);
    shown
        .mesh
        .add_builder(&Handle::new(builder).cast::<dyn MeshVSPrsBuilder>(), false);
}

/// `vsetlocation name dx dy dz`.
pub fn move_by(host: &ViewerHost, shown: &ShownMesh, offset: [f64; 3]) {
    let location = Location::new(Transform3::translation(offset));
    host.context.set_location(&shown.object, &location).unwrap();
    host.context.update_current_viewer();
}

/// Fits the view to the selected entities.
pub fn fit_selected(host: &ViewerHost) {
    let view = host.view.clone().cast::<V3dView>();
    host.context
        .fit_selected(view.get().unwrap(), 0.01, false)
        .unwrap();
}

fn clip_plane(equation: [f64; 4]) -> Handle<Graphic3dClipPlane> {
    let [a, b, c, d] = equation;
    Handle::new(Graphic3dClipPlane::from_equation(Vec4::new(a, b, c, d)).unwrap())
}

/// `vclipplane create p`, `change p equation 0 1 0 0`, capping on in gray 0.9, `set p view`.
pub fn clip_with_capping(host: &ViewerHost) {
    let plane = clip_plane([0., 1., 0., 0.]);
    plane.set_capping(true);
    plane
        .set_capping_color(Color::new(0.9, 0.9, 0.9, ColorSpace::Rgb).unwrap())
        .unwrap();
    host.view.add_clip_plane(&plane);
    host.context.update_current_viewer();
}

/// `vclipplane c -set name -equation a b c d`.
pub fn clip_object(shown: &ShownMesh, equation: [f64; 4]) {
    shown.object.add_clip_plane(&clip_plane(equation));
}
