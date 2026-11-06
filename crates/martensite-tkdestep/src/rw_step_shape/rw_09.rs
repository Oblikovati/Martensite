#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(RwStepShapeRwFace, "RwStepShapeRwFace");
impl RwStepShapeRwFace {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeFace>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "face")? {
            return Ok(());
        }
        rw_block_0077!(a_bounds, a_name, ach, anent2, data, i2, nb2, nsub2, num);
        ent.init(&a_name, &a_bounds);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepShapeFace>,
    ) -> Result<(), Failure> {
        rw_block_0162!(ent, i2, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapeFace>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let nb_elem1: i32 = ent.nb_bounds()?;
        for is1 in 1..=nb_elem1 {
            iter.get_one_item(&ent.bounds_value(is1)?.cast());
        }
        Ok(())
    }
}

step_rw_class!(
    RwStepShapeRwFaceBasedSurfaceModel,
    "RwStepShapeRwFaceBasedSurfaceModel"
);
impl RwStepShapeRwFaceBasedSurfaceModel {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeFaceBasedSurfaceModel>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "face_based_surface_model")? {
            return Ok(());
        }
        let mut a_representation_item_name: Handle<HAsciiString> = Default::default();
        data.read_string(
            num,
            1,
            "representation_item.name",
            ach,
            &mut a_representation_item_name,
        )?;
        let mut a_fbsm_faces: Handle<HArray1<Handle<StepShapeConnectedFaceSet>>> =
            Default::default();
        let mut sub2: i32 = 0;
        if data.read_sub_list(num, 2, "fbsm_faces", ach, &mut sub2, false, 0, 0)? {
            let num2: i32 = sub2;
            let nb0: i32 = data.nb_params(num2)?;
            a_fbsm_faces = step_new_array1::<Handle<StepShapeConnectedFaceSet>>(1, nb0)?;
            for i0 in 1..=nb0 {
                let mut an_it0: Handle<StepShapeConnectedFaceSet> = Default::default();
                data.read_entity(
                    num2,
                    i0,
                    "fbsm_faces",
                    ach,
                    StepShapeConnectedFaceSet::static_type(),
                    &mut an_it0,
                )?;
                step_array1_set(&a_fbsm_faces, i0, an_it0.clone(), "ReadStep")?;
            }
        }
        ent.init(&a_representation_item_name, &a_fbsm_faces);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepShapeFaceBasedSurfaceModel>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.parent.parent.name())?;
        sw.open_sub()?;
        for i1 in 1..=step_live(&ent.fbsm_faces(), "WriteStep")?.array1().length() {
            let var0: Handle<StepShapeConnectedFaceSet> =
                step_array1_item(&ent.fbsm_faces(), i1, "WriteStep")?;
            sw.send_entity(&var0)?;
        }
        sw.close_sub()?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapeFaceBasedSurfaceModel>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        for i1 in 1..=step_live(&ent.fbsm_faces(), "Share")?.array1().length() {
            let var0: Handle<StepShapeConnectedFaceSet> =
                step_array1_item(&ent.fbsm_faces(), i1, "Share")?;
            iter.add_item(&var0.cast());
        }
        Ok(())
    }
}

step_rw_class!(RwStepShapeRwFaceBound, "RwStepShapeRwFaceBound");
impl RwStepShapeRwFaceBound {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeFaceBound>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "face_bound")? {
            return Ok(());
        }
        rw_block_0072!(a_bound, a_name, a_orientation, ach, data, ent, num);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepShapeFaceBound>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.bound())?;
        sw.send_boolean(ent.orientation())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapeFaceBound>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.bound().cast());
        Ok(())
    }
}

step_rw_class!(RwStepShapeRwFaceOuterBound, "RwStepShapeRwFaceOuterBound");
impl RwStepShapeRwFaceOuterBound {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeFaceOuterBound>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "face_outer_bound")? {
            return Ok(());
        }
        rw_block_0072!(a_bound, a_name, a_orientation, ach, data, ent, num);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepShapeFaceOuterBound>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.bound())?;
        sw.send_boolean(ent.orientation())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapeFaceOuterBound>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.bound().cast());
        Ok(())
    }
}

step_rw_class!(RwStepShapeRwFaceSurface, "RwStepShapeRwFaceSurface");
impl RwStepShapeRwFaceSurface {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeFaceSurface>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "face_surface")? {
            return Ok(());
        }
        rw_block_0077!(a_bounds, a_name, ach, anent2, data, i2, nb2, nsub2, num);
        let mut a_face_geometry: Handle<StepGeomSurface> = Default::default();
        data.read_entity(
            num,
            3,
            "face_geometry",
            ach,
            StepGeomSurface::static_type(),
            &mut a_face_geometry,
        )?;
        let mut a_same_sense: bool = Default::default();
        data.read_boolean(num, 4, "same_sense", ach, &mut a_same_sense)?;
        ent.init(&a_name, &a_bounds, &a_face_geometry, a_same_sense);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepShapeFaceSurface>,
    ) -> Result<(), Failure> {
        rw_block_0094!(ent, i2, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapeFaceSurface>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_0188!(ent, is1, iter, nb_elem1);
        Ok(())
    }
}
