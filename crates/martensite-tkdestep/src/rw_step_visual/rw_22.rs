#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepVisualRwTessellatedShell,
    "RwStepVisualRwTessellatedShell"
);
impl RwStepVisualRwTessellatedShell {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_check: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepVisualTessellatedShell>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(the_num, 3, the_check, "tessellated_shell")? {
            return Ok(());
        }
        rw_block_b_0099!(
            a_items,
            a_representation_item_name,
            an_it0,
            i0,
            nb0,
            num2,
            sub2,
            the_check,
            the_data,
            the_num
        );
        let mut a_topological_link: Handle<StepShapeConnectedFaceSet> = Default::default();
        let mut has_topological_link: bool = true;
        if the_data.is_param_defined(the_num, 3)? {
            the_data.read_entity(
                the_num,
                3,
                "topological_link",
                the_check,
                StepShapeConnectedFaceSet::static_type(),
                &mut a_topological_link,
            )?;
        } else {
            has_topological_link = false;
            a_topological_link = Default::default();
        }
        the_ent.init(
            &a_representation_item_name,
            &a_items,
            has_topological_link,
            &a_topological_link,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepVisualTessellatedShell>,
    ) -> Result<(), Failure> {
        rw_block_b_0131!(i1, the_ent, the_sw, var0);
        if the_ent.has_topological_link() {
            the_sw.send_entity(&the_ent.topological_link())?;
        } else {
            the_sw.send_undef()?;
        }
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepVisualTessellatedShell>,
        the_iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0219!(i1, the_ent, the_iter, var0);
        if the_ent.has_topological_link() {
            the_iter.add_item(&the_ent.topological_link().cast());
        }
        Ok(())
    }
}

step_rw_class!(
    RwStepVisualRwTessellatedSolid,
    "RwStepVisualRwTessellatedSolid"
);
impl RwStepVisualRwTessellatedSolid {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_check: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepVisualTessellatedSolid>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(the_num, 3, the_check, "tessellated_solid")? {
            return Ok(());
        }
        rw_block_b_0099!(
            a_items,
            a_representation_item_name,
            an_it0,
            i0,
            nb0,
            num2,
            sub2,
            the_check,
            the_data,
            the_num
        );
        let mut a_geometric_link: Handle<StepShapeManifoldSolidBrep> = Default::default();
        let mut has_geometric_link: bool = true;
        if the_data.is_param_defined(the_num, 3)? {
            the_data.read_entity(
                the_num,
                3,
                "geometric_link",
                the_check,
                StepShapeManifoldSolidBrep::static_type(),
                &mut a_geometric_link,
            )?;
        } else {
            has_geometric_link = false;
            a_geometric_link = Default::default();
        }
        the_ent.init(
            &a_representation_item_name,
            &a_items,
            has_geometric_link,
            &a_geometric_link,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepVisualTessellatedSolid>,
    ) -> Result<(), Failure> {
        rw_block_b_0131!(i1, the_ent, the_sw, var0);
        if the_ent.has_geometric_link() {
            the_sw.send_entity(&the_ent.geometric_link())?;
        } else {
            the_sw.send_undef()?;
        }
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepVisualTessellatedSolid>,
        the_iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0219!(i1, the_ent, the_iter, var0);
        if the_ent.has_geometric_link() {
            the_iter.add_item(&the_ent.geometric_link().cast());
        }
        Ok(())
    }
}

step_rw_class!(
    RwStepVisualRwTessellatedStructuredItem,
    "RwStepVisualRwTessellatedStructuredItem"
);
impl RwStepVisualRwTessellatedStructuredItem {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_check: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepVisualTessellatedStructuredItem>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(the_num, 1, the_check, "tessellated_structured_item")? {
            return Ok(());
        }
        let mut a_representation_item_name: Handle<HAsciiString> = Default::default();
        the_data.read_string(
            the_num,
            1,
            "representation_item.name",
            the_check,
            &mut a_representation_item_name,
        )?;
        the_ent.init(&a_representation_item_name);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepVisualTessellatedStructuredItem>,
    ) -> Result<(), Failure> {
        the_sw.send_entity(&the_ent.name())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        _unused_0: &Handle<StepVisualTessellatedStructuredItem>,
        _unused_1: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
}

step_rw_class!(
    RwStepVisualRwTessellatedVertex,
    "RwStepVisualRwTessellatedVertex"
);
impl RwStepVisualRwTessellatedVertex {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_check: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepVisualTessellatedVertex>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(the_num, 4, the_check, "tessellated_vertex")? {
            return Ok(());
        }
        rw_block_b_0138!(
            a_coordinates,
            a_representation_item_name,
            the_check,
            the_data,
            the_num
        );
        let mut a_topological_link: Handle<StepShapeVertexPoint> = Default::default();
        let mut has_topological_link: bool = true;
        if the_data.is_param_defined(the_num, 3)? {
            the_data.read_entity(
                the_num,
                3,
                "topological_link",
                the_check,
                StepShapeVertexPoint::static_type(),
                &mut a_topological_link,
            )?;
        } else {
            has_topological_link = false;
            a_topological_link = Default::default();
        }
        let mut a_point_index: i32 = Default::default();
        the_data.read_integer(the_num, 4, "point_index", the_check, &mut a_point_index)?;
        the_ent.init(
            &a_representation_item_name,
            &a_coordinates,
            has_topological_link,
            &a_topological_link,
            a_point_index,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepVisualTessellatedVertex>,
    ) -> Result<(), Failure> {
        the_sw.send_entity(&the_ent.name())?;
        the_sw.send_entity(&the_ent.coordinates())?;
        if the_ent.has_topological_link() {
            the_sw.send_entity(&the_ent.topological_link())?;
        } else {
            the_sw.send_undef()?;
        }
        the_sw.send_integer(the_ent.point_index())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepVisualTessellatedVertex>,
        the_iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        the_iter.add_item(&the_ent.coordinates().cast());
        if the_ent.has_topological_link() {
            the_iter.add_item(&the_ent.topological_link().cast());
        }
        Ok(())
    }
}
