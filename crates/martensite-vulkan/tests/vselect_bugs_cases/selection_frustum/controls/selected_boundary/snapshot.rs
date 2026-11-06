//! Freeze actual camera, authored owners and presentation/aspect identities around the control.
use super::*;
use martensite_tkservice::graphic3d::*;
use martensite_tkv3d::{ais::AisPoint, prs_mgr::PrsMgrPresentableObjectMembers as _};

#[derive(Debug)]
pub(super) struct SceneSnapshot {
    camera: [u64; 32],
    selection: (i32, usize),
    objects: Vec<String>,
    groups: Vec<GroupSnapshot>,
}
#[derive(Debug)]
struct GroupSnapshot {
    identity: (String, usize, i32, usize, usize),
    aspects: Handle<dyn Graphic3dAspectsSource>,
}
impl PartialEq for SceneSnapshot {
    fn eq(&self, other: &Self) -> bool {
        self.camera == other.camera
            && self.selection == other.selection
            && self.objects == other.objects
            && self.groups.len() == other.groups.len()
            && self
                .groups
                .iter()
                .zip(&other.groups)
                .all(|(a, b)| a.identity == b.identity && a.aspects.is_equal(&*b.aspects))
    }
}
impl SceneSnapshot {
    pub fn capture(session: &ViewerSession) -> Self {
        let objects = ["b", "bz", "p1", "p"]
            .map(|name| object_state(session, name))
            .to_vec();
        let groups = ["b", "bz", "p1", "p"]
            .into_iter()
            .flat_map(|name| group_states(session, name))
            .collect();
        Self {
            camera: camera_bits(session),
            selection: (
                session.context().nb_selected(),
                session.context().first_selected_object().object_address() as usize,
            ),
            objects,
            groups,
        }
    }

    pub fn assert_except_role_aspects(&self, other: &Self, role: &SelectedBoundaryRole) {
        let result = self.check_except_role_aspects(other, role);
        assert!(result.is_ok(), "{result:?}");
    }

    pub fn check_except_role_aspects(
        &self,
        other: &Self,
        role: &SelectedBoundaryRole,
    ) -> Result<(), Failure> {
        let frame_matches = self.camera == other.camera
            && self.selection == other.selection
            && self.objects == other.objects
            && self.groups.len() == other.groups.len();
        let groups_match = self.role_group_transitions_match(other, role);
        if frame_matches && groups_match {
            return Ok(());
        }
        Err(program_error(&format!(
            "selected boundary snapshot {other:?}; expected role-only transition from {self:?}"
        )))
    }
    fn role_group_transitions_match(&self, other: &Self, role: &SelectedBoundaryRole) -> bool {
        self.groups
            .iter()
            .zip(&other.groups)
            .all(|(before, after)| {
                let targeted = role
                    .groups
                    .iter()
                    .any(|group| group.object_address() as usize == before.identity.3);
                group_transition_matches(before, after, targeted)
            })
    }
}

fn object_state(session: &ViewerSession, name: &str) -> String {
    let object = frustum_operand(session, name).unwrap();
    let persistence = object.transform_persistence();
    let pers = persistence
        .get()
        .map(|p| *p.transform_pers_base().state.read());
    let colors = color_bits(&object);
    let handles = object_handles(&object);
    let modes = [object.display_mode(), object.hilight_mode()];
    let transforms = (
        transform_bits(object.local_transformation()),
        transform_bits(object.transformation()),
    );
    format!(
        "{name}: handles={handles:?} modes={modes:?} selected={} transforms={transforms:?} persistence={:?}/{pers:?} rgba={colors:?} point={:?}",
        session.context().is_selected_object(&object),
        persistence.object_address(),
        marker_bits(&object)
    )
}

fn marker_bits(object: &Handle<dyn AisInteractiveObject>) -> Option<[u64; 3]> {
    let point = object.cast::<AisPoint>();
    point.get().map(|value| {
        let p = value.component().pnt();
        [p.x(), p.y(), p.z()].map(f64::to_bits)
    })
}

fn transform_bits(transform: martensite_tkmath::gp::Transform3) -> [u64; 12] {
    std::array::from_fn(|index| {
        transform
            .value((index / 4) as i32 + 1, (index % 4) as i32 + 1)
            .unwrap()
            .to_bits()
    })
}

fn group_states(session: &ViewerSession, name: &str) -> Vec<GroupSnapshot> {
    let object = frustum_operand(session, name).unwrap();
    object
        .presentations()
        .into_iter()
        .flat_map(|presentation| {
            let identity = presentation.object_address() as usize;
            let mode = presentation.mode();
            presentation
                .groups()
                .into_iter()
                .map(move |group| group_state(name, identity, mode, &group))
        })
        .collect()
}

fn frozen_aspect(
    actual: &Handle<dyn Graphic3dAspectsSource>,
) -> Handle<dyn Graphic3dAspectsSource> {
    let copy = Handle::new(Graphic3dAspects::default());
    *copy.aspects_base().fields.write() = actual.aspects_base().fields.read().clone();
    copy.cast()
}

fn group_transition_matches(before: &GroupSnapshot, after: &GroupSnapshot, targeted: bool) -> bool {
    if group_origin(before) != group_origin(after) {
        return false;
    }
    if !targeted {
        return before.identity == after.identity && before.aspects.is_equal(&*after.aspects);
    }
    let expected = frozen_aspect(&before.aspects);
    expected.set_line_type(martensite_tkservice::aspect::AspectTypeOfLine::Empty);
    before.identity.4 != after.identity.4 && expected.is_equal(&*after.aspects)
}

fn camera_bits(session: &ViewerSession) -> [u64; 32] {
    let camera = session.view().camera();
    let mut bits = camera
        .orientation_matrix()
        .data()
        .map(f64::to_bits)
        .to_vec();
    bits.extend(camera.projection_matrix().data().map(f64::to_bits));
    bits.try_into().unwrap()
}
fn object_handles(object: &Handle<dyn AisInteractiveObject>) -> [usize; 5] {
    [
        object.object_address() as usize,
        object.attributes().object_address() as usize,
        object.hilight_attributes().object_address() as usize,
        object.dynamic_hilight_attributes().object_address() as usize,
        object.global_sel_owner().object_address() as usize,
    ]
}
fn color_bits(object: &Handle<dyn AisInteractiveObject>) -> [u64; 4] {
    let color = object.color();
    [
        color.red(),
        color.green(),
        color.blue(),
        object.transparency(),
    ]
    .map(f64::to_bits)
}
fn group_state(
    name: &str,
    presentation: usize,
    mode: i32,
    group: &Handle<dyn Graphic3dGroup>,
) -> GroupSnapshot {
    let actual = group.aspects();
    GroupSnapshot {
        identity: (
            name.to_owned(),
            presentation,
            mode,
            group.object_address() as usize,
            actual.object_address() as usize,
        ),
        aspects: frozen_aspect(&actual),
    }
}
fn group_origin(group: &GroupSnapshot) -> (&str, usize, i32, usize) {
    (
        &group.identity.0,
        group.identity.1,
        group.identity.2,
        group.identity.3,
    )
}
