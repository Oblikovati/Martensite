// The RW member bodies several classes repeat.

macro_rules! rw_block_b_0183 {
    ($a_context_of_items:ident, $a_items:ident, $a_name:ident, $ach:ident, $data:ident, $ent:ident, $num:ident) => {
        let mut $a_context_of_items: Handle<StepReprRepresentationContext> = Default::default();
        $data.read_entity(
            $num,
            3,
            "context_of_items",
            $ach,
            StepReprRepresentationContext::static_type(),
            &mut $a_context_of_items,
        )?;
        $ent.init(&$a_name, &$a_items, &$a_context_of_items);
    };
}
pub(crate) use rw_block_b_0183;

macro_rules! rw_block_b_0184 {
    ($ent:ident, $is2:ident, $iter:ident, $nb_elem2:ident) => {
        $iter.get_one_item(&$ent.assigned_approval().cast());
        let $nb_elem2: i32 = $ent.nb_items()?;
        for $is2 in 1..=$nb_elem2 {
            $iter.get_one_item(&$ent.items_value($is2)?.value());
        }
    };
}
pub(crate) use rw_block_b_0184;

macro_rules! rw_block_b_0185 {
    ($ent:ident, $is3:ident, $iter:ident, $nb_elem3:ident) => {
        $iter.get_one_item(&$ent.role().cast());
        let $nb_elem3: i32 = $ent.nb_items()?;
        for $is3 in 1..=$nb_elem3 {
            $iter.get_one_item(&$ent.items_value($is3)?.value());
        }
    };
}
pub(crate) use rw_block_b_0185;

macro_rules! rw_block_b_0186 {
    ($ent:ident, $is1:ident, $iter:ident, $nb_elem1:ident) => {
        let $nb_elem1: i32 = $ent.nb_styles();
        for $is1 in 1..=$nb_elem1 {
            $iter.get_one_item(&$ent.styles_value($is1)?.cast());
        }
        $iter.get_one_item(&$ent.item().cast());
    };
}
pub(crate) use rw_block_b_0186;

macro_rules! rw_block_b_0187 {
    ($a_name:ident, $ach:ident, $data:ident, $ent:ident, $num:ident) => {
        let mut $a_name: Handle<HAsciiString> = Default::default();
        $data.read_string($num, 1, "name", $ach, &mut $a_name)?;
        step_live(&$ent.get_pre_defined_item(), "ReadStep")?.init(&$a_name);
    };
}
pub(crate) use rw_block_b_0187;

macro_rules! rw_block_b_0188 {
    ($ent:ident, $is1:ident, $iter:ident, $nb_elem1:ident) => {
        let $nb_elem1: i32 = $ent.nb_items();
        for $is1 in 1..=$nb_elem1 {
            $iter.get_one_item(&$ent.items_value($is1)?.cast());
        }
        $iter.get_one_item(&$ent.context_of_items().cast());
    };
}
pub(crate) use rw_block_b_0188;

macro_rules! rw_block_b_0189 {
    ($a_item:ident, $a_section_ref:ident, $ach:ident, $data:ident, $ent:ident, $num:ident) => {
        let mut $a_item: Handle<StepElementAnalysisItemWithinRepresentation> = Default::default();
        $data.read_entity(
            $num,
            2,
            "item",
            $ach,
            StepElementAnalysisItemWithinRepresentation::static_type(),
            &mut $a_item,
        )?;
        $ent.init(&$a_section_ref, &$a_item);
    };
}
pub(crate) use rw_block_b_0189;

macro_rules! rw_block_b_0190 {
    ($an_ent:ident, $an_items:ident, $nb_sub:ident) => {
        let mut $an_items: Handle<HArray1<Handle<StepReprShapeAspect>>> = Default::default();
        let mut $an_ent: Handle<StepReprShapeAspect> = Default::default();
        let mut $nb_sub: i32 = Default::default();
    };
}
pub(crate) use rw_block_b_0190;

macro_rules! rw_block_b_0191 {
    ($a_description:ident, $a_name:ident, $ach:ident, $data:ident, $num:ident) => {
        let mut $a_name: Handle<HAsciiString> = Default::default();
        $data.read_string($num, 1, "name", $ach, &mut $a_name)?;
        let mut $a_description: Handle<HAsciiString> = Default::default();
    };
}
pub(crate) use rw_block_b_0191;

macro_rules! rw_block_b_0192 {
    ($a_shape_aspect_description:ident, $a_shape_aspect_name:ident, $ach:ident, $data:ident, $num:ident) => {
        let mut $a_shape_aspect_name: Handle<HAsciiString> = Default::default();
        $data.read_string(
            $num,
            1,
            "shape_aspect.name",
            $ach,
            &mut $a_shape_aspect_name,
        )?;
        let mut $a_shape_aspect_description: Handle<HAsciiString> = Default::default();
    };
}
pub(crate) use rw_block_b_0192;

macro_rules! rw_block_b_0193 {
    ($a_group_description:ident, $a_group_name:ident, $ach:ident, $data:ident, $num:ident) => {
        let mut $a_group_name: Handle<HAsciiString> = Default::default();
        $data.read_string($num, 1, "group.name", $ach, &mut $a_group_name)?;
        let mut $a_group_description: Handle<HAsciiString> = Default::default();
    };
}
pub(crate) use rw_block_b_0193;

macro_rules! rw_block_b_0194 {
    ($ent:ident, $i1:ident, $sw:ident) => {
        $sw.open_sub()?;
        for $i1 in 1..=$ent.nb_invisible_items()? {
            $sw.send_entity(&$ent.invisible_items_value($i1)?.value())?;
        }
        $sw.close_sub()?;
    };
}
pub(crate) use rw_block_b_0194;

macro_rules! rw_block_b_0195 {
    ($ent:ident, $i2:ident, $sw:ident) => {
        $sw.open_sub()?;
        for $i2 in 1..=$ent.nb_items()? {
            $sw.send_entity(&$ent.items_value($i2)?.value())?;
        }
        $sw.close_sub()?;
    };
}
pub(crate) use rw_block_b_0195;

macro_rules! rw_block_b_0196 {
    ($ent:ident, $i:ident, $sw:ident) => {
        $sw.open_sub()?;
        for $i in 1..=$ent.nb_elements() {
            $sw.send_entity(&$ent.elements_value($i)?.value())?;
        }
        $sw.close_sub()?;
    };
}
pub(crate) use rw_block_b_0196;

macro_rules! rw_block_b_0197 {
    ($a_shape_clipping:ident, $an_ent:ident, $nb_sub:ident) => {
        let mut $a_shape_clipping: Handle<
            HArray1<StepVisualCameraModelD3MultiClippingInterectionSelect>,
        > = Default::default();
        let mut $an_ent: StepVisualCameraModelD3MultiClippingInterectionSelect = Default::default();
        let mut $nb_sub: i32 = Default::default();
    };
}
pub(crate) use rw_block_b_0197;

macro_rules! rw_block_b_0198 {
    ($a_styles:ident, $a_styles_item:ident, $nsub1:ident) => {
        let mut $a_styles: Handle<HArray1<StepVisualPresentationStyleSelect>> = Default::default();
        let mut $a_styles_item: StepVisualPresentationStyleSelect = Default::default();
        let mut $nsub1: i32 = Default::default();
    };
}
pub(crate) use rw_block_b_0198;
