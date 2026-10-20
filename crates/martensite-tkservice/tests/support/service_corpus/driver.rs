//! The Graphic3d driver model on the null driver.
use super::camera::read_camera;
use super::common::{Reader, end, flag, int, real, text};
use super::print::Output;
use super::tried;
use martensite_tkernel::ncollection::Vec2;
use martensite_tkernel::standard::dump::DumpStream;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkmath::bnd::BoundingBox3;
use martensite_tkmath::gp::{Axis2, Transform3, Vector3, dz};
use martensite_tkmath::toploc::Datum3;
use martensite_tkservice::aspect::{
    AspectNeutralWindow, AspectTypeOfTriedronPosition, AspectWindow,
};
use martensite_tkservice::graphic3d::{
    GRAPHIC3D_DISPLAY_PRIORITY_NB, Graphic3dArrayOfPoints, Graphic3dBndBox3d,
    Graphic3dCStructureMembers, Graphic3dCView, Graphic3dCViewComputed, Graphic3dCViewMembers,
    Graphic3dCViewStructures, Graphic3dCamera, Graphic3dCullingTool, Graphic3dDisplayPriority as P,
    Graphic3dGraphicDriver, Graphic3dGraphicDriverMembers, Graphic3dGroupMembers, Graphic3dLayer,
    Graphic3dRenderingParamsFrustumCulling as Culling, Graphic3dStructure,
    Graphic3dStructureBounds, Graphic3dStructureDisplay, Graphic3dStructureManager,
    Graphic3dStructureMembers, Graphic3dStructureNetwork, Graphic3dStructureSource, Graphic3dText,
    Graphic3dTransModeFlags, Graphic3dTransformPers, Graphic3dTransformPersSource,
    Graphic3dTypeOfConnection, Graphic3dTypeOfStructure,
};
use martensite_tkservice::null_driver::{NullGraphicDriver, NullView};

/// Prints a `Bnd_Box`: void and whole flags, then the limits of a non-void box.
fn bnd(output: &Output, bounds: &BoundingBox3) {
    flag(output, bounds.is_void());
    flag(output, bounds.is_whole());
    if let Ok(limits) = bounds.get() {
        limits.into_iter().for_each(|value| real(output, value));
    }
}

/// Prints a BVH box: valid flag and corners.
fn bvh_box(output: &Output, bounds: &Graphic3dBndBox3d) {
    flag(output, bounds.is_valid());
    if bounds.is_valid() {
        let (min, max) = (bounds.corner_min(), bounds.corner_max());
        [min.x, min.y, min.z, max.x, max.y, max.z]
            .into_iter()
            .for_each(|value| real(output, value));
    }
}

struct Scene {
    manager: Handle<Graphic3dStructureManager>,
    view: Handle<dyn Graphic3dCView>,
    structs: Vec<Handle<dyn Graphic3dStructureSource>>,
    size: [i32; 2],
}

fn structure_at(scene: &Scene, r: &mut Reader<'_>) -> Handle<dyn Graphic3dStructureSource> {
    let index = r.integer();
    usize::try_from(index)
        .ok()
        .and_then(|i| scene.structs.get(i))
        .cloned()
        .unwrap_or_default()
}

fn persistence(r: &mut Reader<'_>) -> Result<Handle<dyn Graphic3dTransformPersSource>, Failure> {
    let (mode, point) = (r.integer(), r.pnt());
    let flags = Graphic3dTransModeFlags(mode);
    let pers = match mode {
        0 => return Ok(Handle::default()),
        64 | 32 => Graphic3dTransformPers::with_corner(
            flags,
            AspectTypeOfTriedronPosition::LEFT_LOWER,
            Vec2::new(point.x() as i32, point.y() as i32),
        )?,
        _ => Graphic3dTransformPers::with_point(flags, &point)?,
    };
    Ok(Handle::new(pers).cast())
}

fn cull(output: &Output, scene: &Scene, state: i32) {
    let mut tool = Graphic3dCullingTool::default();
    tool.set_view_volume(
        &scene.view.camera(),
        &martensite_tkernel::ncollection::Mat4::identity(),
    );
    tool.set_viewport_size(scene.size[0], scene.size[1], 1.0);
    tool.cache_clip_pts_projections();
    let state = [Culling::Off, Culling::On, Culling::NoUpdate][state.clamp(0, 2) as usize];
    for layer in scene.view.layers().iter().filter_map(Handle::get) {
        tried(
            output,
            layer.update_culling(scene.view.identification(), &tool, state),
        );
    }
}

fn create(scene: &mut Scene, r: &mut Reader<'_>) {
    let structure: Handle<dyn Graphic3dStructureSource> =
        Graphic3dStructure::new(&scene.manager, &Handle::default())
            .map(|s| s.cast())
            .unwrap_or_default();
    let (min, max) = (r.pnt(), r.pnt());
    if let Ok(group) = structure.new_group() {
        group.set_min_max_values([min.x(), min.y(), min.z(), max.x(), max.y(), max.z()]);
    }
    scene.structs.push(structure);
}

fn add_text(s: &dyn Graphic3dStructureSource, r: &mut Reader<'_>) {
    let text = Graphic3dText::new(10.0);
    let point = r.pnt();
    text.set_position(&point);
    if r.flag()
        && let Ok(axes) = Axis2::new(point, dz())
    {
        text.set_orientation(&axes);
    }
    if let Ok(group) = s.current_group() {
        group.add_text(&Handle::new(text), true);
    }
}

fn add_points(s: &dyn Graphic3dStructureSource, r: &mut Reader<'_>) {
    let count = r.integer();
    let points: Vec<_> = (0..count).map(|_| r.pnt()).collect();
    let Ok(array) = Graphic3dArrayOfPoints::new(count, 0) else {
        return;
    };
    for point in &points {
        let _ = array.add_vertex(point.x() as f32, point.y() as f32, point.z() as f32);
    }
    if let Ok(group) = s.new_group() {
        group.add_primitive_array(&Handle::new(array).cast(), true);
    }
}

fn structure_op(
    output: &Output,
    scene: &Scene,
    op: i32,
    handle: &Handle<dyn Graphic3dStructureSource>,
    r: &mut Reader<'_>,
) {
    let Some(s) = handle.get() else { return };
    match op {
        2 => s.display(),
        3 => s.erase(),
        4 => tried(output, s.set_display_priority(priority(r.integer()))),
        5 => s.set_z_layer(r.integer()),
        6 => s.set_infinite_state(r.flag()),
        7 => {
            let other = structure_at(scene, r);
            let connection = if r.integer() == 0 {
                Graphic3dTypeOfConnection::Ancestor
            } else {
                Graphic3dTypeOfConnection::Descendant
            };
            s.connect(&other, connection, r.flag());
        }
        8 => s.disconnect(&structure_at(scene, r)),
        9 => s.set_visible(r.flag()),
        10 => s.highlight(&Handle::default(), true),
        11 => s.unhighlight(),
        12 => {
            if let Ok(pers) = persistence(r) {
                s.set_transform_persistence(&pers);
            }
        }
        13 => {
            let mut trsf = Transform3::default();
            let v = r.xyz();
            trsf.set_translation(Vector3::new(v.x(), v.y(), v.z()));
            s.set_transformation(&Handle::new(Datum3::new(trsf)));
        }
        14 => s.remove(),
        15 => s.set_mutable(r.flag()),
        17 => add_text(s, r),
        18 => add_points(s, r),
        20 => s.set_is_for_highlight(r.flag()),
        21 => s.clear(r.flag()),
        22 => {
            let index = r.integer();
            let groups = s.groups();
            if index >= 1
                && index as usize <= groups.len()
                && let Some(group) = groups[index as usize - 1].get()
            {
                group.remove();
            }
        }
        23 => s.set_visual(visual(r.integer())),
        _ => {}
    }
}

fn priority(value: i32) -> P {
    const ALL: [P; 11] = [
        P::Bottom,
        P::AlmostBottom,
        P::Below2,
        P::Below1,
        P::Below,
        P::Normal,
        P::Above,
        P::Above1,
        P::Above2,
        P::Highlight,
        P::Topmost,
    ];
    usize::try_from(value)
        .ok()
        .and_then(|i| ALL.get(i).copied())
        .unwrap_or(P::Invalid)
}

fn visual(value: i32) -> Graphic3dTypeOfStructure {
    [
        Graphic3dTypeOfStructure::Wireframe,
        Graphic3dTypeOfStructure::Shading,
        Graphic3dTypeOfStructure::Computed,
        Graphic3dTypeOfStructure::All,
    ][value.clamp(0, 3) as usize]
}

fn scene_op(output: &Output, scene: &mut Scene, r: &mut Reader<'_>) {
    match r.integer() {
        1 => create(scene, r),
        19 => cull(output, scene, r.integer()),
        24 => scene.view.set_computed_mode(r.flag()),
        25 => {
            if r.flag() {
                scene.view.activate()
            } else {
                scene.view.deactivate()
            }
        }
        op => {
            let handle = structure_at(scene, r);
            structure_op(output, scene, op, &handle, r);
        }
    }
}

fn print_structure(output: &Output, scene: &Scene, handle: &Handle<dyn Graphic3dStructureSource>) {
    let Some(s) = handle.get() else { return };
    flag(output, s.is_deleted());
    if s.is_deleted() {
        return;
    }
    let cstructure = s.cstructure();
    let Some(c) = cstructure.get() else { return };
    int(output, s.identification());
    [s.is_displayed(), s.is_visible(), s.is_highlighted()]
        .into_iter()
        .for_each(|value| flag(output, value));
    int(output, s.display_priority() as i32);
    int(output, s.get_z_layer());
    [s.is_empty(), s.is_infinite(), s.is_transformed()]
        .into_iter()
        .for_each(|value| flag(output, value));
    int(output, s.number_of_groups());
    [
        c.is_culled(),
        c.cstructure_base().fields.read().is_2d_text,
        c.is_always_rendered(),
        scene.view.is_displayed(handle),
    ]
    .into_iter()
    .for_each(|value| flag(output, value));
    bvh_box(output, &c.bounding_box());
    bnd(output, &s.min_max_values(false));
    bnd(output, &s.min_max_values(true));
    for group in s.groups().iter().filter_map(Handle::get) {
        group
            .min_max_values()
            .into_iter()
            .for_each(|value| real(output, value));
        flag(output, group.is_empty());
    }
    let mut set = Vec::new();
    s.descendants(&mut set);
    int(output, set.len() as i32);
}

fn print_layer(output: &Output, scene: &Scene, layer: &Graphic3dLayer) {
    int(output, layer.layer_id());
    int(output, layer.nb_structures());
    int(output, layer.nb_structures_not_culled());
    int(output, layer.nb_of_transform_persistence_objects());
    flag(output, layer.is_immediate());
    for p in 0..GRAPHIC3D_DISPLAY_PRIORITY_NB {
        let map = layer.structures(priority(p));
        int(output, map.extent() as i32);
        for weak in map.iter() {
            int(
                output,
                weak.upgrade().get().map_or(-1, |c| c.identification()),
            );
        }
    }
    let camera: Handle<Graphic3dCamera> = scene.view.camera();
    for aux in [false, true] {
        super::print::show(
            output,
            layer.bounding_box(scene.view.identification(), &camera, scene.size, aux),
            |b| bnd(output, &b),
        );
    }
    super::print::show(
        output,
        layer.consider_zoom_persistence_objects(scene.view.identification(), &camera, scene.size),
        |value| real(output, value),
    );
}

/// Replaces every printed address ("0x...") by "?", then masks the uninitialised render
/// parameter and camera tile values, as `sv_mask_dump`.
fn mask_dump(text: &[u8]) -> Vec<u8> {
    let head = b"\": \"0x";
    let mut out = text.to_vec();
    let mut from = 0;
    while let Some(found) = out[from..].windows(head.len()).position(|w| w == head) {
        let start = from + found + 4;
        let end = out[start..]
            .iter()
            .position(|&b| b == b'"')
            .map_or(out.len(), |e| start + e);
        out.splice(start..end, *b"?");
        from += found + 1;
    }
    let masked =
        super::params::masked(&super::params::masked(&out, "ChartSize"), "Params.Params3d");
    super::params::masked(
        &super::params::masked(&masked, "Tile"),
        "WorldViewProjState",
    )
}

/// Prints a masked dump.
fn dump_masked(output: &Output, write: impl FnOnce(&mut DumpStream) -> std::io::Result<()>) {
    let mut stream = DumpStream::default();
    match write(&mut stream) {
        Ok(()) => text(output, &mask_dump(stream.bytes())),
        Err(error) => output.write(format_args!("dump-error:{error} ")),
    }
}

/// The dumps of the live structures, the driver, the view and (with one displayed and one
/// highlighted structure at most) the manager.
fn print_dumps(output: &Output, scene: &Scene, driver: &Handle<dyn Graphic3dGraphicDriver>) {
    for structure in scene
        .structs
        .iter()
        .filter_map(Handle::get)
        .filter(|s| !s.is_deleted())
    {
        dump_masked(output, |s| structure.dump_json(s, -1));
    }
    if let Some(driver) = driver.get() {
        dump_masked(output, |s| driver.dump_json(s, -1));
    }
    if let Some(view) = scene.view.get() {
        dump_masked(output, |s| view.cview_dump_json(s, -1));
    }
    let manager = &scene.manager;
    if manager.displayed_structures().len() <= 1 && manager.highlighted_structures().len() <= 1 {
        dump_masked(output, |s| manager.dump_json(s, -1));
    }
}

/// `sv_scene`: a scene script on the null driver, then its structures, view and layers.
pub fn scene_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let camera = Handle::new(read_camera(output, &mut r));
    let size = [r.integer(), r.integer()];
    let driver = NullGraphicDriver::new_handle();
    let Ok(manager) = Graphic3dStructureManager::new(&driver) else {
        return;
    };
    let manager = Handle::new(manager);
    let Ok(view) = driver.create_view(&manager) else {
        return;
    };
    let window = AspectNeutralWindow::default();
    window.set_size(size[0], size[1]);
    let window: Handle<dyn AspectWindow> = Handle::new(window).cast();
    view.cast::<NullView>().set_null_window(&window);
    view.set_camera(&camera);
    view.activate();
    let mut scene = Scene {
        manager,
        view,
        structs: Vec::new(),
        size,
    };
    for _ in 0..r.integer() {
        scene_op(output, &mut scene, &mut r);
    }
    int(output, scene.structs.len() as i32);
    scene
        .structs
        .iter()
        .for_each(|s| print_structure(output, &scene, s));
    super::print::show(output, scene.view.min_max_values(false), |b| {
        bnd(output, &b)
    });
    super::print::show(output, scene.view.min_max_values(true), |b| bnd(output, &b));
    super::print::show(
        output,
        scene.view.consider_zoom_persistence_objects(),
        |value| real(output, value),
    );
    int(output, scene.view.number_of_displayed_structures());
    for layer in scene.view.layers().iter().filter_map(Handle::get) {
        print_layer(output, &scene, layer);
    }
    print_dumps(output, &scene, &driver);
    end(output);
}

#[cfg(test)]
mod tests {
    #[test]
    fn original_address_and_uninitialized_field_masks_preserve_other_text() {
        let value = br#""Owner": "0xABC", "Other": "0x123", "ChartSize": {"x": 1}, "Params.Params3d": {"v": 2}, "Tile": {"v": 3}, "WorldViewProjState": {"v": 4}, "Keep": {"v": 5}"#;
        let expected = br#""Owner": "?", "Other": "?", "ChartSize": ?, "Params.Params3d": ?, "Tile": ?, "WorldViewProjState": ?, "Keep": {"v": 5}"#;
        assert_eq!(super::mask_dump(value), expected);
    }
}
