//! The modifiers that rebuild the drawings and the groups a partial copy has broken.
use super::IgesSelectModelModifier;
use super::IgesSelectViewSorter;
use super::modifier_bases::{iges_model_modifier, modifier_base};
use crate::iges_basic::{
    IgesBasicGroup, IgesBasicGroupWithoutBackP, IgesBasicOrderedGroup,
    IgesBasicOrderedGroupWithoutBackP,
};
use crate::iges_data::{
    IgesDataIgesEntity, IgesDataIgesModel, IgesDataViewKindEntity, iges_array1_set, iges_live,
    iges_new_array1,
};
use crate::iges_draw::{IgesDrawDrawing, IgesDrawDrawingWithRotation};
use martensite_tkernel::{
    ncollection::HArray1,
    standard::{Failure, Handle, RuntimeTyped, StandardTransient},
    tcollection::AsciiString,
};
use martensite_tkmath::gp::Xy;
use martensite_tkxsbase::{
    if_select::IFSelectContextModif,
    interface::{
        InterfaceCopyTool, InterfaceEntityIterator, InterfaceInterfaceModel, InterfaceModelEntities,
    },
};

/// A list of entities as an array from 1, null when empty.
fn entity_array(
    list: &[Handle<StandardTransient>],
) -> Result<Handle<HArray1<Handle<IgesDataIgesEntity>>>, Failure> {
    if list.is_empty() {
        return Ok(Handle::default());
    }
    let array = iges_new_array1(1, list.len() as i32)?;
    for (rank, item) in list.iter().enumerate() {
        iges_array1_set(&array, rank as i32 + 1, item.cast(), "rebuild")?;
    }
    Ok(array)
}

iges_model_modifier! {
    /// Rebuilds the drawings of which an element was copied, with their views and what was copied
    /// of their annotations, and attaches the entities to the new views.
    ///
    /// ```
    /// use martensite_tkdeiges::iges_select::IgesSelectRebuildDrawings;
    /// use martensite_tkxsbase::if_select::IFSelectGeneralModifier;
    /// assert!(IgesSelectRebuildDrawings::new().may_change_graph());
    /// ```
    IgesSelectRebuildDrawings, "IgesSelectRebuildDrawings", {},
    self => AsciiString::new("Rebuild Drawings (with empty views)")
}

impl Default for IgesSelectRebuildDrawings {
    fn default() -> Self {
        Self::new()
    }
}

impl IgesSelectRebuildDrawings {
    /// The modifier.
    pub fn new() -> Self {
        Self {
            transient: StandardTransient::default(),
            base: modifier_base(true),
        }
    }
}

/// The views of a drawing and, rank for rank, their origins.
type DrawingViews = (
    Handle<HArray1<Handle<dyn IgesDataViewKindEntity>>>,
    Handle<HArray1<Xy>>,
);

/// The copied views of a drawing, added to the target, with their origins.
fn copied_views(
    nbv: i32,
    view: &dyn Fn(i32) -> Result<Handle<dyn IgesDataViewKindEntity>, Failure>,
    origin: &dyn Fn(i32) -> Result<Xy, Failure>,
    target: &IgesDataIgesModel,
    tc: &mut InterfaceCopyTool,
) -> Result<DrawingViews, Failure> {
    if nbv <= 0 {
        return Ok((Handle::default(), Handle::default()));
    }
    let views = iges_new_array1(1, nbv)?;
    let origins = iges_new_array1(1, nbv)?;
    for rank in 1..=nbv {
        let aview = tc
            .transferred(&view(rank)?.cast())?
            .cast::<dyn IgesDataViewKindEntity>();
        iges_array1_set(&views, rank, aview.clone(), "rebuild drawings")?;
        target.add_entity(&aview.cast())?;
        iges_array1_set(&origins, rank, origin(rank)?, "rebuild drawings")?;
    }
    Ok((views, origins))
}

/// What was copied of the annotations of a drawing.
fn copied_frame(
    nba: i32,
    annotation: &dyn Fn(i32) -> Result<Handle<IgesDataIgesEntity>, Failure>,
    tc: &InterfaceCopyTool,
) -> Result<Handle<HArray1<Handle<IgesDataIgesEntity>>>, Failure> {
    let mut framelist = InterfaceEntityIterator::new();
    for rank in 1..=nba {
        let mut annot = Handle::default();
        if tc.search(&annotation(rank)?.cast(), &mut annot)? {
            framelist.get_one_item(&annot);
        }
    }
    entity_array(&framelist.content())
}

/// Rebuilds a drawing of one class or the other, and adds it to the target.
fn rebuild_drawing(
    item: &Handle<IgesDataIgesEntity>,
    target: &IgesDataIgesModel,
    tc: &mut InterfaceCopyTool,
) -> Result<(), Failure> {
    if let Some(draw) = item.cast::<IgesDrawDrawing>().get() {
        let (views, origins) = copied_views(
            draw.nb_views()?,
            &|rank| draw.view_item(rank),
            &|rank| Ok(draw.view_origin(rank)?.xy()),
            target,
            tc,
        )?;
        let frame = copied_frame(draw.nb_annotations()?, &|rank| draw.annotation(rank), tc)?;
        return Handle::new(IgesDrawDrawing::new())
            .get()
            .map_or(Ok(()), |fresh| fresh.init(&views, &origins, &frame));
    }
    if let Some(draw) = item.cast::<IgesDrawDrawingWithRotation>().get() {
        let nbv = draw.nb_views()?;
        let (views, origins) = copied_views(
            nbv,
            &|rank| draw.view_item(rank),
            &|rank| Ok(draw.view_origin(rank)?.xy()),
            target,
            tc,
        )?;
        let rotations = if nbv > 0 {
            let array = iges_new_array1(1, nbv)?;
            for rank in 1..=nbv {
                iges_array1_set(
                    &array,
                    rank,
                    draw.orientation_angle(rank)?,
                    "rebuild drawings",
                )?;
            }
            array
        } else {
            Handle::default()
        };
        let frame = copied_frame(draw.nb_annotations()?, &|rank| draw.annotation(rank), tc)?;
        return Handle::new(IgesDrawDrawingWithRotation::new())
            .get()
            .map_or(Ok(()), |fresh| {
                fresh.init(&views, &origins, &rotations, &frame)
            });
    }
    Ok(())
}

/// Attaches the entities of a set to the copies of their views.
fn attach_to_new_views(
    setl: &InterfaceEntityIterator,
    tc: &InterfaceCopyTool,
) -> Result<(), Failure> {
    for item in setl.content() {
        let entity = item.cast::<IgesDataIgesEntity>();
        let Some(ent) = entity.get() else {
            continue;
        };
        let vieworig = ent.view();
        if vieworig.is_null() {
            continue;
        }
        let mut copied = Handle::default();
        if !tc.search(&vieworig.cast(), &mut copied)? {
            continue;
        }
        let viewnew = copied.cast::<dyn IgesDataViewKindEntity>();
        if viewnew.is_some() {
            ent.init_view(&viewnew);
        }
    }
    Ok(())
}

impl IgesSelectModelModifier for IgesSelectRebuildDrawings {
    fn performing(
        &self,
        ctx: &mut IFSelectContextModif,
        target: &Handle<IgesDataIgesModel>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let original = ctx.original_model().cast::<IgesDataIgesModel>();
        let targ = iges_live(target, "rebuild drawings")?;
        let sorter = IgesSelectViewSorter::default();
        sorter.set_model(&original);
        sorter.add(&original.cast());
        sorter.sort_drawings(ctx.original_graph())?;
        let sets = sorter.sets(true)?;
        let packets = iges_live(&sets, "rebuild drawings")?;
        for is in 1..=packets.nb_packets() {
            let setl = packets.entities(is)?;
            let mut copied_any = false;
            for member in setl.content() {
                let mut newent = Handle::default();
                copied_any |= tc.search(&member, &mut newent)?;
            }
            if !copied_any {
                continue;
            }
            let item = sorter.set_item(is, true)?;
            if item.get().is_none_or(|live| live.type_number() != 404) {
                continue;
            }
            rebuild_drawing(&item, targ, tc)?;
            attach_to_new_views(&setl, tc)?;
        }
        Ok(())
    }
}

iges_model_modifier! {
    /// Rebuilds the groups of which more than one member was copied.
    ///
    /// ```
    /// use martensite_tkdeiges::iges_select::IgesSelectRebuildGroups;
    /// use martensite_tkxsbase::if_select::IFSelectGeneralModifier;
    /// assert_eq!(IgesSelectRebuildGroups::new().label().as_bytes(), b"Rebuild Groups");
    /// ```
    IgesSelectRebuildGroups, "IgesSelectRebuildGroups", {},
    self => AsciiString::new("Rebuild Groups")
}

impl Default for IgesSelectRebuildGroups {
    fn default() -> Self {
        Self::new()
    }
}

impl IgesSelectRebuildGroups {
    /// The modifier.
    pub fn new() -> Self {
        Self {
            transient: StandardTransient::default(),
            base: modifier_base(true),
        }
    }
}

/// What was copied of the members of a group; `None` when the class is not one of the groups.
/// The classes derive from the group, so the most derived decides the case.
fn copied_members(
    ent: &Handle<IgesDataIgesEntity>,
    tc: &InterfaceCopyTool,
) -> Result<InterfaceEntityIterator, Failure> {
    let mut newlist = InterfaceEntityIterator::new();
    if let Some(group) = ent.cast::<IgesBasicGroup>().get() {
        for rank in 1..=group.nb_entities() {
            let mut newent = Handle::default();
            if tc.search(&group.value(rank)?, &mut newent)? {
                newlist.get_one_item(&newent);
            }
        }
    }
    Ok(newlist)
}

/// Adds to the target a new group of the class of `ent`.
fn add_new_group(
    ent: &Handle<IgesDataIgesEntity>,
    members: &Handle<HArray1<Handle<IgesDataIgesEntity>>>,
    target: &IgesDataIgesModel,
) -> Result<(), Failure> {
    let is = |kind: &martensite_tkernel::standard::RuntimeType| ent.is_instance(kind);
    if is(IgesBasicOrderedGroupWithoutBackP::static_type()) {
        let group = Handle::new(IgesBasicOrderedGroupWithoutBackP::new());
        iges_live(&group, "rebuild groups")?.init(members)?;
        return target.add_entity(&group.cast());
    }
    if is(IgesBasicOrderedGroup::static_type()) {
        let group = Handle::new(IgesBasicOrderedGroup::new());
        iges_live(&group, "rebuild groups")?.init(members)?;
        return target.add_entity(&group.cast());
    }
    if is(IgesBasicGroupWithoutBackP::static_type()) {
        let group = Handle::new(IgesBasicGroupWithoutBackP::new());
        iges_live(&group, "rebuild groups")?.init(members)?;
        return target.add_entity(&group.cast());
    }
    let group = Handle::new(IgesBasicGroup::new());
    iges_live(&group, "rebuild groups")?.init(members)?;
    target.add_entity(&group.cast())
}

impl IgesSelectModelModifier for IgesSelectRebuildGroups {
    fn performing(
        &self,
        ctx: &mut IFSelectContextModif,
        target: &Handle<IgesDataIgesModel>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let original = ctx.original_model().cast::<IgesDataIgesModel>();
        let model = iges_live(&original, "rebuild groups")?;
        let targ = iges_live(target, "rebuild groups")?;
        for i in 1..=model.nb_entities() {
            let ent = model.entity(i);
            if ent.get().is_none_or(|live| live.type_number() != 402) {
                continue;
            }
            let mut newent = Handle::default();
            if tc.search(&ent.cast(), &mut newent)? {
                continue;
            }
            let newlist = copied_members(&ent, tc)?;
            if newlist.nb_entities() <= 1 {
                continue;
            }
            add_new_group(&ent, &entity_array(&newlist.content())?, targ)?;
        }
        Ok(())
    }
}
