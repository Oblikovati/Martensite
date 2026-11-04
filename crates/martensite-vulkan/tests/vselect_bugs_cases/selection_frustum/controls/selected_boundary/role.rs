//! Resolve authored selection and actual aspect identity before observing emitted draws.
use super::*;
use martensite_tkservice::graphic3d::*;
use martensite_tkv3d::prs_mgr::PrsMgrPresentableObjectMembers as _;
use martensite_vulkan::Graphic3dDrawGroup;

pub(super) struct SelectedBoundaryRole {
    pub object: Handle<dyn AisInteractiveObject>,
    pub groups: Vec<Handle<dyn Graphic3dGroup>>,
}
impl SelectedBoundaryRole {
    pub fn resolve(session: &ViewerSession, name: &str) -> Result<Self, Failure> {
        let object = frustum_operand(session, name)?;
        require_selected_object(session, name, &object)?;
        require_presentation_modes(session, name, &object)?;
        let groups = role_groups(&object);
        if groups.len() != 1 {
            return Err(program_error(&format!(
                "role group count {}; expected one actual unfree-boundary group",
                groups.len()
            )));
        }
        Ok(Self { object, groups })
    }

    pub fn draws(
        &self,
        session: &ViewerSession,
    ) -> Result<Vec<PrimitiveDraw>, Box<dyn std::error::Error>> {
        let camera = session.view().camera();
        let mut draws = Vec::new();
        for group in &self.groups {
            let actual = renderer_role_group(group)?;
            draws.extend(actual.primitive_draws_for_view(&camera, [409, 409])?);
        }
        Ok(draws)
    }

    pub fn ordered_join(
        &self,
        actual: &[PrimitiveDraw],
        role: &[PrimitiveDraw],
    ) -> Result<Vec<usize>, Failure> {
        role.iter()
            .map(|draw| unique_draw_position(actual, draw))
            .collect()
    }

    pub fn without_role(
        &self,
        actual: &[PrimitiveDraw],
        role: &[PrimitiveDraw],
    ) -> Result<Vec<PrimitiveDraw>, Failure> {
        let joined = self.ordered_join(actual, role)?;
        Ok(actual
            .iter()
            .enumerate()
            .filter(|(index, _)| !joined.contains(index))
            .map(|(_, draw)| draw.clone())
            .collect())
    }
}

fn role_groups(object: &Handle<dyn AisInteractiveObject>) -> Vec<Handle<dyn Graphic3dGroup>> {
    let role = object
        .attributes()
        .un_free_boundary_aspect()
        .aspect()
        .object_address();
    object
        .presentations()
        .into_iter()
        .filter(|prs| prs.mode() == 0)
        .flat_map(|prs| prs.groups())
        .filter(|group| group.aspects().object_address() == role)
        .collect()
}

fn require_selected_object(
    session: &ViewerSession,
    name: &str,
    object: &Handle<dyn AisInteractiveObject>,
) -> Result<(), Failure> {
    let context = session.context();
    if context.nb_selected() == 1
        && context.first_selected_object() == *object
        && context.is_selected_object(object)
    {
        return Ok(());
    }
    Err(program_error(&format!(
        "selected target {name:?}; expected sole actual selected authored object"
    )))
}
fn require_presentation_modes(
    session: &ViewerSession,
    name: &str,
    object: &Handle<dyn AisInteractiveObject>,
) -> Result<(), Failure> {
    let context = session.context();
    if context.is_displayed_mode(object, 1)
        && context.main_prs_mgr().is_highlighted(&object.cast(), 0)
    {
        return Ok(());
    }
    Err(program_error(&format!(
        "selected target {name:?}; expected normal1 and highlighted0 presentations"
    )))
}
fn unique_draw_position(actual: &[PrimitiveDraw], role: &PrimitiveDraw) -> Result<usize, Failure> {
    let matches: Vec<_> = actual
        .iter()
        .enumerate()
        .filter_map(|(index, draw)| {
            (draw == role && format!("{draw:?}") == format!("{role:?}")).then_some(index)
        })
        .collect();
    if matches.len() == 1 {
        return Ok(matches[0]);
    }
    Err(program_error(&format!(
        "role full-draw matches {}; expected unique emitted draw",
        matches.len()
    )))
}

fn renderer_role_group(
    group: &Handle<dyn Graphic3dGroup>,
) -> Result<Handle<Graphic3dDrawGroup>, Failure> {
    let actual = group.cast::<Graphic3dDrawGroup>();
    if !actual.is_null() {
        return Ok(actual);
    }
    Err(program_error(&format!(
        "actual role group {:?}; expected injected renderer Graphic3dDrawGroup",
        group.object_address()
    )))
}
