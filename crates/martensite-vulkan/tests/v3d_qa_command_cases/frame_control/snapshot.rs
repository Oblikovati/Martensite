//! Exact group/aspect handles and owned values isolate the frame replacement.
use super::*;
use crate::case_view::SourceViewOutput;
use martensite_tkv3d::{
    ais::{AisInteractiveObjectMembers, AisSelectionMembers},
    prs_mgr::PrsMgrPresentableObjectMembers,
};
struct GroupState {
    group: Handle<dyn Graphic3dGroup>,
    aspect: Handle<dyn Graphic3dAspectsSource>,
    frozen: Handle<Graphic3dAspects>,
}
#[derive(PartialEq)]
struct StableFrame {
    presentations: Vec<(usize, i32, bool, bool)>,
    camera: [u64; 32],
    view_owners: [usize; 5],
    scale_values: String,
    scene: String,
}
pub(super) struct FrameSnapshot {
    groups: Vec<GroupState>,
    stable: StableFrame,
}
impl FrameSnapshot {
    pub(super) fn capture(session: &ViewerSession, role: &role::FrameRole) -> io::Result<Self> {
        let groups = role
            .scale
            .presentations()
            .iter()
            .flat_map(|p| p.groups())
            .map(GroupState::capture)
            .collect::<io::Result<Vec<_>>>()?;
        Ok(Self {
            groups,
            stable: StableFrame::capture(session, role)?,
        })
    }
    pub(super) fn require(
        &self,
        session: &ViewerSession,
        role: &role::FrameRole,
        replacement: Option<&Handle<dyn Graphic3dAspectsSource>>,
    ) -> io::Result<()> {
        let current = Self::capture(session, role)?;
        if self.stable != current.stable || self.groups.len() != current.groups.len() {
            return Err(refused(
                "scene state changed; expected original scale/presentations/camera/persistence",
            ));
        }
        for (before, after) in self.groups.iter().zip(&current.groups) {
            before.require(after, &role.group, replacement)?;
        }
        Ok(())
    }
}
impl StableFrame {
    fn capture(session: &ViewerSession, role: &role::FrameRole) -> io::Result<Self> {
        let presentations = presentation_states(role);
        Ok(Self {
            presentations,
            camera: camera_bits(session),
            view_owners: view_owner_handles(session),
            scale_values: scale_values(&role.scale),
            scene: scene_values(session, &role.scale)?,
        })
    }
}
fn presentation_states(role: &role::FrameRole) -> Vec<(usize, i32, bool, bool)> {
    role.scale
        .presentations()
        .iter()
        .map(|p| {
            (
                p.object_address() as usize,
                p.mode(),
                p.must_be_updated(),
                p.is_displayed(),
            )
        })
        .collect()
}

impl GroupState {
    fn capture(group: Handle<dyn Graphic3dGroup>) -> io::Result<Self> {
        if group.is_null() {
            return Err(refused("group null; expected computed existing group"));
        }
        let aspect = group.aspects();
        if aspect.is_null() {
            return Err(refused(
                "group aspect null; expected computed existing style",
            ));
        }
        Ok(Self {
            group,
            frozen: copy_aspect(&aspect),
            aspect,
        })
    }
    fn require(
        &self,
        after: &Self,
        targeted: &Handle<dyn Graphic3dGroup>,
        replacement: Option<&Handle<dyn Graphic3dAspectsSource>>,
    ) -> io::Result<()> {
        let expected = self.expected_aspect(targeted, replacement);
        self.require_binding(after, &expected)?;
        if replacement.is_some() && self.group == *targeted {
            require_empty_clone(&self.frozen, &after.aspect)?;
        }
        Ok(())
    }
    fn expected_aspect(
        &self,
        targeted: &Handle<dyn Graphic3dGroup>,
        replacement: Option<&Handle<dyn Graphic3dAspectsSource>>,
    ) -> Handle<dyn Graphic3dAspectsSource> {
        replacement
            .filter(|_| self.group == *targeted)
            .unwrap_or(&self.aspect)
            .clone()
    }
    fn require_binding(
        &self,
        after: &Self,
        expected: &Handle<dyn Graphic3dAspectsSource>,
    ) -> io::Result<()> {
        if self.group != after.group
            || *expected != after.aspect
            || !self.aspect.is_equal(&*self.frozen)
        {
            return Err(refused(
                "group/aspect changed; expected exact original handles and frame-only swap",
            ));
        }
        Ok(())
    }
}
fn require_empty_clone(
    before: &Handle<Graphic3dAspects>,
    after: &Handle<dyn Graphic3dAspectsSource>,
) -> io::Result<()> {
    let expected = copy_aspect(&before.cast());
    expected.set_line_type(martensite_tkservice::aspect::AspectTypeOfLine::Empty);
    if !after.is_equal(&*expected) {
        return Err(refused(
            "replacement fields changed; expected complete Empty clone",
        ));
    }
    Ok(())
}
fn copy_aspect(original: &Handle<dyn Graphic3dAspectsSource>) -> Handle<Graphic3dAspects> {
    let copy = Handle::new(Graphic3dAspects::default());
    *copy.aspects_base().fields.write() = original.aspects_base().fields.read().clone();
    copy
}
fn camera_bits(session: &ViewerSession) -> [u64; 32] {
    let camera = session.view().camera();
    let mut values = camera
        .orientation_matrix()
        .data()
        .map(f64::to_bits)
        .to_vec();
    values.extend(camera.projection_matrix().data().map(f64::to_bits));
    values.try_into().unwrap()
}

fn scale_values(scale: &AisColorScale) -> String {
    format!("{}|{}", scale_dimensions(scale), scale_inputs(scale))
}
fn scale_dimensions(scale: &AisColorScale) -> String {
    format!(
        "{:?}",
        (
            scale.get_size(),
            scale.get_position(),
            scale.get_text_height(),
            scale.get_range(),
            scale.get_number_of_intervals(),
            scale.get_label_position(),
            scale.get_title_position(),
            scale.is_reversed(),
            scale.is_smooth_transition(),
            scale.is_label_at_border(),
            scale.is_logarithmic()
        )
    )
}
fn scale_inputs(scale: &AisColorScale) -> String {
    format!(
        "{:?}",
        (
            scale.get_title().as_utf16().to_vec(),
            scale.get_format().as_bytes().to_vec(),
            scale.labels().clone(),
            scale.colors().clone(),
            scale.get_label_type(),
            scale.get_color_type(),
            scale.hue_range(),
            scale.color_range()
        )
    )
}
fn scene_values(session: &ViewerSession, scale: &AisColorScale) -> io::Result<String> {
    Ok(format!(
        "{:?}|{:?}|{}|{}|{}|{:?}",
        object_handles(scale),
        session.source_window_size(),
        context_values(session)?,
        default_values(session)?,
        persistence_state(scale)?,
        object_transform_bits(scale)
    ))
}
fn object_handles(scale: &AisColorScale) -> [usize; 6] {
    [
        scale as *const AisColorScale as usize,
        scale.get_context().object_address() as usize,
        scale.attributes().object_address() as usize,
        scale.hilight_attributes().object_address() as usize,
        scale.dynamic_hilight_attributes().object_address() as usize,
        scale.transform_persistence().object_address() as usize,
    ]
}
fn default_values(session: &ViewerSession) -> io::Result<String> {
    let mut defaults = martensite_tkernel::standard::dump::DumpStream::default();
    session
        .context()
        .default_drawer()
        .dump_json(&mut defaults, -1)?;
    Ok(defaults.text().to_owned())
}
fn context_values(session: &ViewerSession) -> io::Result<String> {
    let selected: Vec<_> = session
        .context()
        .selection()
        .objects()
        .iter()
        .map(|o| o.object_address() as usize)
        .collect();
    Ok(format!(
        "{:?}",
        (
            displayed_handles(session)?,
            selected,
            session.context().detected_owner().object_address() as usize
        )
    ))
}
fn displayed_handles(session: &ViewerSession) -> io::Result<Vec<usize>> {
    let mut displayed = Default::default();
    session
        .context()
        .displayed_objects(&mut displayed)
        .map_err(|e| refused(e.to_string()))?;
    Ok(displayed
        .iter()
        .map(|o| o.object_address() as usize)
        .collect())
}

fn persistence_state(scale: &AisColorScale) -> io::Result<String> {
    let persistence = scale.transform_persistence();
    if persistence.is_null() {
        return Ok("null".to_owned());
    }
    let mut stream = martensite_tkernel::standard::dump::DumpStream::default();
    persistence.dump_json(&mut stream, -1)?;
    Ok(stream.text().to_owned())
}
fn object_transform_bits(scale: &AisColorScale) -> [[u64; 12]; 2] {
    [
        transform_bits(&scale.local_transformation()),
        transform_bits(&scale.transformation()),
    ]
}
fn transform_bits(transform: &martensite_tkmath::gp::Transform3) -> [u64; 12] {
    std::array::from_fn(|index| {
        transform
            .value((index / 4 + 1) as i32, (index % 4 + 1) as i32)
            .expect("fixed 3x4 transformation index is valid")
            .to_bits()
    })
}

fn view_owner_handles(session: &ViewerSession) -> [usize; 5] {
    [
        session.view().object_address() as usize,
        session.view().view_base().window().object_address() as usize,
        session.view().camera().object_address() as usize,
        session.context().object_address() as usize,
        session.context().main_prs_mgr().object_address() as usize,
    ]
}
