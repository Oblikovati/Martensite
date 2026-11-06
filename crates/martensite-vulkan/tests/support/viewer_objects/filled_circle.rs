// Typed stand-in for the viewer command's private FilledCircle object (ViewerObjectCommands; the test hosts build it from toolkit APIs only,
// as the circle command does, because the command crate keeps it private.

//! `FilledCircle`: a circle (or a circular sector) shaded as a face and selected as a filled
//! circle.
use martensite_tkbrep::topods::Face;
use martensite_tkernel::standard::{Failure, Handle, HandleReference, StandardTransient};
use martensite_tkg3d::geom::{GeomCircle, GeomConic as _, GeomCurve as _};
use martensite_tkmath::gp::resolution;
use martensite_tkservice::graphic3d::Graphic3dStructureSource;
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeFace, BRepBuilderAPIMakeWire,
};
use martensite_tkv3d::ais::{AisInteractiveObject, AisInteractiveObjectBase};
use martensite_tkv3d::prs_mgr::{
    PrsMgrPresentableObject, PrsMgrPresentableObjectBase, PrsMgrPresentableObjectMembers as _,
    PrsMgrPresentationManager, PrsMgrTypeOfPresentation3d, prs_mgr_presentable_bind,
};
use martensite_tkv3d::select_mgr::{
    SelectMgrEntityOwner, SelectMgrSelectableObject, SelectMgrSelectableObjectBase,
    SelectMgrSelection,
};
use martensite_tkv3d::select3d::{Select3DSensitiveCircle, Select3DSensitivePoly};
use martensite_tkv3d::std_prs::{StdPrsShadedShape, StdPrsVolume};
use std::f64::consts::PI;

/// `FilledCircle`: the circle, its parameter range and its filled selection.
pub struct FilledCircle {
    transient: StandardTransient,
    base: AisInteractiveObjectBase,
    circle: Handle<GeomCircle>,
    u_start: f64,
    u_end: f64,
    filled_status: bool,
}
martensite_tkernel::impl_transient!(FilledCircle, "FilledCircle", dyn AisInteractiveObject, transient,
    interfaces: [dyn AisInteractiveObject, dyn SelectMgrSelectableObject, dyn PrsMgrPresentableObject]);

impl FilledCircle {
    /// The filled circle of a circle between two parameters.
    pub fn new(circle: &Handle<GeomCircle>, u_start: f64, u_end: f64) -> Handle<Self> {
        let object = Handle::new(Self {
            transient: StandardTransient::default(),
            base: AisInteractiveObjectBase::new(PrsMgrTypeOfPresentation3d::AllView),
            circle: circle.clone(),
            u_start,
            u_end,
            filled_status: true,
        });
        prs_mgr_presentable_bind(&object.cast());
        object
    }
    /// Whether the parameter range is not the whole circle.
    fn is_sector(&self) -> bool {
        ((self.u_end - self.u_start).abs() - 2.0 * PI).abs() > resolution()
    }
    /// `ComputeFace`: the face of the circle, or of the sector closed through the center.
    fn compute_face(&self) -> Result<Face, Failure> {
        let circle = self
            .circle
            .get()
            .ok_or_else(|| Failure::with_message("Standard_NullObject", "Geom_Circle"))?;
        let edge =
            BRepBuilderAPIMakeEdge::from_circ_params(circle.circ()?, self.u_start, self.u_end)?
                .edge()?;
        let wire = if self.is_sector() {
            let end_center =
                BRepBuilderAPIMakeEdge::from_points(circle.value(self.u_end)?, circle.location())?
                    .edge()?;
            let start_center = BRepBuilderAPIMakeEdge::from_points(
                circle.location(),
                circle.value(self.u_start)?,
            )?
            .edge()?;
            BRepBuilderAPIMakeWire::from_three_edges(&edge, &end_center, &start_center)?.wire()?
        } else {
            BRepBuilderAPIMakeWire::from_edge(&edge)?.wire()?
        };
        BRepBuilderAPIMakeFace::from_wire(&wire, false)?.face()
    }
}

impl PrsMgrPresentableObject for FilledCircle {
    fn presentable_base(&self) -> &PrsMgrPresentableObjectBase {
        &self.base.presentable
    }
    fn compute(
        &self,
        _: &dyn HandleReference<PrsMgrPresentationManager>,
        presentation: &dyn HandleReference<dyn Graphic3dStructureSource>,
        mode: i32,
    ) -> Result<(), Failure> {
        presentation.observe().clear(true);
        let face = self.compute_face()?;
        if face.is_null() || mode != 0 {
            return Ok(());
        }
        StdPrsShadedShape::add(
            presentation,
            face.as_ref(),
            &self.attributes(),
            StdPrsVolume::Autodetection,
            &Handle::default(),
        )
    }
    martensite_tkv3d::ais_presentable_overrides!();
}

impl SelectMgrSelectableObject for FilledCircle {
    fn selectable_base(&self) -> &SelectMgrSelectableObjectBase {
        &self.base.selectable
    }
    fn compute_selection(
        &self,
        selection: &dyn HandleReference<SelectMgrSelection>,
        _: i32,
    ) -> Result<(), Failure> {
        let this: Handle<dyn SelectMgrSelectableObject> = self.presentable_base().this().cast();
        let owner = SelectMgrEntityOwner::with_selectable(&this, 0).into_cast();
        let circ = self
            .circle
            .get()
            .ok_or_else(|| Failure::with_message("Standard_NullObject", "Geom_Circle"))?
            .circ()?;
        let sensitive = if self.is_sector() {
            Handle::new(Select3DSensitivePoly::circle(
                &owner,
                &circ,
                self.u_start,
                self.u_end,
                self.filled_status,
                12,
            )?)
            .cast()
        } else {
            Handle::new(Select3DSensitiveCircle::new(
                &owner,
                &circ,
                self.filled_status,
            ))
            .cast()
        };
        selection.observe().add(&sensitive)
    }
}

impl AisInteractiveObject for FilledCircle {
    fn interactive_base(&self) -> &AisInteractiveObjectBase {
        &self.base
    }
}
