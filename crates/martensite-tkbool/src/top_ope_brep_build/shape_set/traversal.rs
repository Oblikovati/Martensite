//! Detached positions retain node identity while their owning lists remain in the set.
use super::super::list_cursor_copy::copied_cursor;
use super::*;
use martensite_tkernel::ncollection::ListCursor;

#[derive(Clone, Default)]
pub(crate) enum IncidentSource {
    #[default]
    Owned,
    Indexed(Shape),
    Current,
    External,
}

#[derive(Clone, Default)]
pub(crate) struct ShapeSetTraversal {
    pub(crate) explorer: Explorer,
    pub(crate) starts: ListCursor,
    pub(crate) shapes: ListCursor,
    pub(crate) incident: ListCursor,
    pub(crate) source: IncidentSource,
    pub(crate) owned: List<Shape>,
    pub(crate) shapes_owner: Option<List<Shape>>,
    pub(crate) starts_owner: Option<List<Shape>>,
}

pub(super) fn cursor_value<'a>(
    cursor: ListCursor,
    list: &'a List<Shape>,
    member: &str,
) -> Result<&'a Shape, Failure> {
    cursor.value(list).ok_or_else(|| {
        Failure::with_type(
            "Standard_NoSuchObject",
            &format!("{member}: no current shape; expected More()"),
            "TopOpeBRepBuildShapeSet",
        )
    })
}

impl TopOpeBRepBuildShapeSetBase {
    pub(crate) fn incident_source(&self) -> Result<&List<Shape>, Failure> {
        match &self.traversal.source {
            IncidentSource::Owned => Ok(&self.traversal.owned),
            IncidentSource::Current => Ok(&self.my_current_shape_neighbours),
            IncidentSource::Indexed(key) => self.my_sub_shape_map.find_from_key(key),
            IncidentSource::External => Err(Failure::with_message(
                "Standard_ProgramError",
                "external neighbour provider requires its retained source",
            )),
        }
    }

    pub(crate) fn shape_iteration_source(&self) -> &List<Shape> {
        self.traversal
            .shapes_owner
            .as_ref()
            .unwrap_or(&self.my_shapes)
    }

    pub(crate) fn start_iteration_source(&self) -> &List<Shape> {
        self.traversal
            .starts_owner
            .as_ref()
            .unwrap_or(&self.my_start_shapes)
    }

    fn copied_state(&self) -> Self {
        Self {
            my_shape_type: self.my_shape_type,
            my_sub_shape_type: self.my_sub_shape_type,
            traversal: self.traversal.clone(),
            my_start_shapes: self.my_start_shapes.clone(),
            my_sub_shape_map: self.my_sub_shape_map.clone(),
            my_shapes: self.my_shapes.clone(),
            my_current_shape: self.my_current_shape.clone(),
            my_current_shape_neighbours: self.my_current_shape_neighbours.clone(),
            my_deb_number: self.my_deb_number,
            my_deb_name: self.my_deb_name.clone(),
            my_omss: self.my_omss.clone(),
            my_omes: self.my_omes.clone(),
            my_omsh: self.my_omsh.clone(),
            my_check_shape: self.my_check_shape,
        }
    }
}

impl Clone for TopOpeBRepBuildShapeSetBase {
    fn clone(&self) -> Self {
        let mut copied = self.copied_state();
        copied.traversal.starts = copied_cursor(
            self.traversal.starts,
            self.start_iteration_source(),
            copied.start_iteration_source(),
        );
        copied.traversal.shapes = copied_cursor(
            self.traversal.shapes,
            self.shape_iteration_source(),
            copied.shape_iteration_source(),
        );
        if let (Ok(before), Ok(after)) = (self.incident_source(), copied.incident_source()) {
            copied.traversal.incident = copied_cursor(self.traversal.incident, before, after);
        }
        copied
    }
}

pub(super) fn active_incident_source<T: TopOpeBRepBuildShapeSet + ?Sized>(
    set: &T,
) -> Result<&List<Shape>, Failure> {
    if matches!(
        set.shape_set_base().traversal.source,
        IncidentSource::External
    ) {
        return set.neighbours_source();
    }
    set.shape_set_base().incident_source()
}
