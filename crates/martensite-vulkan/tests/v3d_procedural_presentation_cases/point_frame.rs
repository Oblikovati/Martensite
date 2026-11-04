//! Full RegressionCommands OCC27700_Text computation; point-style frame and text have separate groups.
use super::*;
use martensite_tkernel::{
    standard::{HandleReference, StandardTransient},
    tcollection::ExtendedString,
};
use martensite_tkmath::gp::{Axis2, Direction3, Point3};
use martensite_tkservice::{aspect::AspectInteriorStyle, graphic3d::*};
use martensite_tkv3d::v3d::V3dTypeOfOrientation;
use martensite_tkv3d::{
    ais::{AisInteractiveObject, AisInteractiveObjectBase},
    prs_mgr::*,
    prs3d::Prs3dText,
    select_mgr::*,
};
const LABEL: &str = "YOU SHOULD SEE THIS TEXT";
const BANNER: &[&str] = &[
    "========",
    "CR27700: Visualization - glPolygonMode() used for frame drawing affects label text shading",
    "========",
    "",
];

/// A private interactive fixture confined to its invocation's viewer thread.
pub(super) struct PointFrameText {
    transient: StandardTransient,
    base: AisInteractiveObjectBase,
}
martensite_tkernel::impl_transient!(PointFrameText,"OCC27700_Text",dyn AisInteractiveObject,transient,
    interfaces:[dyn AisInteractiveObject,dyn SelectMgrSelectableObject,dyn PrsMgrPresentableObject]);
impl PointFrameText {
    /// Allocate the same base presentation and bind its virtual dispatch; `PointFrameText::new()`.
    pub(super) fn new() -> Handle<Self> {
        let object = Handle::new(Self {
            transient: StandardTransient::default(),
            base: AisInteractiveObjectBase::new(PrsMgrTypeOfPresentation3d::AllView),
        });
        prs_mgr_presentable_bind(&object.cast());
        object
    }

    /// Clone live shading fields before changing only the frame style; `object.frame_aspect()`.
    pub(super) fn frame_aspect(&self) -> Handle<Graphic3dAspectFillArea3d> {
        let copied = Handle::new(Graphic3dAspectFillArea3d::default());
        let inherited = self.attributes().shading_aspect().aspect();
        *copied.aspects_base().fields.write() = inherited.aspects_base().fields.read().clone();
        copied.set_interior_style(AspectInteriorStyle::Point);
        copied
    }

    fn frame_group(&self, prs: &Handle<dyn Graphic3dStructureSource>) -> Result<(), Failure> {
        let frame = frame_triangles()?;
        let aspect = self.frame_aspect();
        let group = prs.new_group()?;
        group.add_primitive_array(&frame.cast(), true);
        group.set_group_primitives_aspect(&aspect.cast());
        Ok(())
    }

    /// Add the literal oriented text in a fresh group; `object.label_group(&presentation)?`.
    pub(super) fn label_group(
        &self,
        prs: &Handle<dyn Graphic3dStructureSource>,
    ) -> Result<Handle<Graphic3dText>, Failure> {
        let group = prs.new_group()?;
        let axes = Axis2::new(Point3::default(), Direction3::new(0., 0., 1.)?)?;
        Ok(Prs3dText::draw_oriented(
            &group,
            &self.attributes().text_aspect(),
            &ExtendedString::new(LABEL),
            &axes,
            true,
        ))
    }
}

/// Preserve the exact capacity, vertices and indexed triangles; `frame_triangles()?`.
pub(super) fn frame_triangles() -> Result<Handle<Graphic3dArrayOfTriangles>, Failure> {
    let frame = Handle::new(Graphic3dArrayOfTriangles::new(6, 6, 0)?);
    for [x, y, z] in [[-1., 0., 0.], [-1., 1., 0.], [3., 1., 0.], [3., 0., 0.]] {
        frame.add_vertex_pnt(&Point3::new(x, y, z))?;
    }
    for index in [1, 2, 3, 2, 3, 4] {
        frame.add_edge(index)?;
    }
    Ok(frame)
}

impl PrsMgrPresentableObject for PointFrameText {
    fn presentable_base(&self) -> &PrsMgrPresentableObjectBase {
        &self.base.presentable
    }
    fn compute(
        &self,
        _: &dyn HandleReference<PrsMgrPresentationManager>,
        prs: &dyn HandleReference<dyn Graphic3dStructureSource>,
        _: i32,
    ) -> Result<(), Failure> {
        let prs = prs.observe();
        self.frame_group(&prs)?;
        self.label_group(&prs)?;
        Ok(())
    }
    martensite_tkv3d::ais_presentable_overrides!();
}
impl SelectMgrSelectableObject for PointFrameText {
    fn selectable_base(&self) -> &SelectMgrSelectableObjectBase {
        &self.base.selectable
    }
    fn compute_selection(
        &self,
        _: &dyn HandleReference<SelectMgrSelection>,
        _: i32,
    ) -> Result<(), Failure> {
        Ok(())
    }
}
impl AisInteractiveObject for PointFrameText {
    fn interactive_base(&self) -> &AisInteractiveObjectBase {
        &self.base
    }
}

/// Display the actual private object before the source top/fit; `display_point_frame(session)?`.
pub(super) fn display_point_frame(
    session: &mut ViewerSession,
) -> Result<Handle<PointFrameText>, Failure> {
    session.clear()?;
    let object = PointFrameText::new();
    // The private QA command displays directly; it never registers a viewer-command name.
    session.update_context(|context| context.display(&object.cast(), false))?;
    session.set_standard_projection(V3dTypeOfOrientation::ZUP_TOP)?;
    session.fit()?;
    Ok(object)
}

/// Complete the custom QA object, top/fit and required image; `point_frame_before_text()?`.
pub(crate) fn point_frame_before_text() -> Outcome {
    banner(BANNER);
    let mut run = SessionRun::new()?;
    let object = display_point_frame(run.session())?;
    run.dump("bug27700.png")?;
    let observation = super::point_diagnostic::observe(&mut run, &object);
    run.retain_observation(observation);
    run.finish()
}
