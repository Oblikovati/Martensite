// The RW member bodies several classes repeat.

macro_rules! rw_block_b_0168 {
    ($ent:ident, $i3:ident, $iter:ident, $var0:ident) => {
        $iter.add_item(&$ent.magnitude());
        $iter.add_item(&$ent.toleranced_shape_aspect().value());
        for $i3 in 1..=step_live(
            &step_live(
                &$ent.get_geometric_tolerance_with_datum_reference(),
                "Share",
            )?
            .datum_system_ap242(),
            "Share",
        )?
        .array1()
        .length()
        {
            let $var0: StepDimTolDatumSystemOrReference = step_array1_item(
                &step_live(
                    &$ent.get_geometric_tolerance_with_datum_reference(),
                    "Share",
                )?
                .datum_system_ap242(),
                $i3,
                "Share",
            )?;
            $iter.add_item(&$var0.value());
        }
    };
}
pub(crate) use rw_block_b_0168;

macro_rules! rw_block_b_0169 {
    ($i1:ident, $iter:ident, $the_ent:ident, $var0:ident) => {
        for $i1 in 1..=step_live(&$the_ent.parent.parent.items(), "Share")?
            .array1()
            .length()
        {
            let $var0: Handle<StepReprRepresentationItem> =
                step_array1_item(&$the_ent.parent.parent.items(), $i1, "Share")?;
            $iter.add_item(&$var0.cast());
        }
        $iter.add_item(&$the_ent.parent.parent.context_of_items().cast());
        $iter.add_item(&$the_ent.parent.represented_link().cast());
    };
}
pub(crate) use rw_block_b_0169;

macro_rules! rw_block_b_0170 {
    ($ent:ident, $i1:ident, $iter:ident, $var0:ident) => {
        for $i1 in 1..=$ent.parent.parent.parent.nb_items() {
            let $var0: Handle<StepReprRepresentationItem> =
                step_array1_item(&$ent.parent.parent.parent.items(), $i1, "Share")?;
            $iter.add_item(&$var0.cast());
        }
        $iter.add_item(&$ent.parent.parent.parent.context_of_items().cast());
        $iter.add_item(&$ent.parent.parent.model_ref().cast());
    };
}
pub(crate) use rw_block_b_0170;

macro_rules! rw_block_b_0171 {
    ($i1:ident, $iter:ident, $the_ent:ident, $var0:ident) => {
        rw_block_b_0204!($i1, $iter, $the_ent, $var0);
        $iter.add_item(&$the_ent.parent().cast());
    };
}
pub(crate) use rw_block_b_0171;

macro_rules! rw_block_b_0172 {
    ($a_style:ident, $ent:ident, $i1:ident, $sw:ident) => {
        $sw.open_sub()?;
        for $i1 in 1..=$ent.nb_styles()? {
            let $a_style: StepVisualPresentationStyleSelect = $ent.styles_value($i1)?;
            if $a_style
                .value()
                .is_kind(StepVisualNullStyleMember::static_type())
            {
                $sw.open_typed_sub(b"NULL_STYLE")?;
                $sw.send_enum(b".NULL.")?;
                $sw.close_sub()?;
            } else {
                $sw.send_entity(&$a_style.value())?;
            }
        }
        $sw.close_sub()?;
    };
}
pub(crate) use rw_block_b_0172;

macro_rules! rw_block_b_0173 {
    ($ent:ident, $i3:ident, $iter:ident, $var0:ident) => {
        $iter.add_item(&$ent.parent.parent.magnitude());
        $iter.add_item(&$ent.parent.parent.toleranced_shape_aspect().value());
        for $i3 in 1..=step_live(&$ent.parent.datum_system_ap242(), "Share")?
            .array1()
            .length()
        {
            let $var0: StepDimTolDatumSystemOrReference =
                step_array1_item(&$ent.parent.datum_system_ap242(), $i3, "Share")?;
            $iter.add_item(&$var0.value());
        }
    };
}
pub(crate) use rw_block_b_0173;

macro_rules! rw_block_b_0174 {
    ($ent:ident, $i1:ident, $iter:ident, $var0:ident) => {
        rw_block_b_0206!($ent, $i1, $iter, $var0);
        $iter.add_item(&$ent.parent.model_ref().cast());
    };
}
pub(crate) use rw_block_b_0174;

macro_rules! rw_block_b_0175 {
    ($iter:ident, $the_ent:ident) => {
        rw_block_b_0212!($iter, $the_ent);
        $iter.add_item(&$the_ent.parent.parent.parent.joint().cast());
    };
}
pub(crate) use rw_block_b_0175;

macro_rules! rw_block_b_0176 {
    ($iter:ident, $the_ent:ident) => {
        rw_block_b_0212!($iter, $the_ent);
        $iter.add_item(&$the_ent.parent.parent.joint().cast());
    };
}
pub(crate) use rw_block_b_0176;

macro_rules! rw_block_b_0177 {
    ($iter:ident, $the_ent:ident) => {
        rw_block_b_0212!($iter, $the_ent);
        $iter.add_item(&$the_ent.parent.joint().cast());
    };
}
pub(crate) use rw_block_b_0177;

macro_rules! rw_block_b_0178 {
    ($ent:ident, $i:ident, $iter:ident, $nb:ident) => {
        let $i: i32 = Default::default();
        let $nb: i32 = step_live(&$ent.shape_clipping(), "Share")?
            .array1()
            .length();
        for $i in 1..=$nb {
            $iter.add_item(&step_array1_item(&$ent.shape_clipping(), $i, "Share")?.value());
        }
    };
}
pub(crate) use rw_block_b_0178;

macro_rules! rw_block_b_0179 {
    ($a_element_descriptor:ident, $a_element_representation_node_list:ident, $a_material:ident, $a_model_ref:ident, $a_property:ident, $a_representation_context_of_items:ident, $a_representation_items:ident, $a_representation_name:ident, $ach:ident, $data:ident, $ent:ident, $num:ident) => {
        let mut $a_material: Handle<StepElementElementMaterial> = Default::default();
        $data.read_entity(
            $num,
            8,
            "material",
            $ach,
            StepElementElementMaterial::static_type(),
            &mut $a_material,
        )?;
        $ent.init(
            &$a_representation_name,
            &$a_representation_items,
            &$a_representation_context_of_items,
            &$a_element_representation_node_list,
            &$a_model_ref,
            &$a_element_descriptor,
            &$a_property,
            &$a_material,
        );
    };
}
pub(crate) use rw_block_b_0179;

macro_rules! rw_block_b_0180 {
    ($ent:ident, $i:ident, $sw:ident) => {
        $sw.open_sub()?;
        for $i in 1..=step_live(&$ent.shape_clipping(), "WriteStep")?
            .array1()
            .length()
        {
            $sw.send_entity(&step_array1_item(&$ent.shape_clipping(), $i, "WriteStep")?.value())?;
        }
        $sw.close_sub()?;
    };
}
pub(crate) use rw_block_b_0180;

macro_rules! rw_block_b_0181 {
    ($the_ent:ident, $the_sw:ident) => {
        $the_sw.send_entity(&$the_ent.name())?;
        if $the_ent.has_description() {
            $the_sw.send_entity(&$the_ent.description())?;
        } else {
            $the_sw.send_undef()?;
        }
        $the_sw.send_entity(&$the_ent.definition().value())?;
    };
}
pub(crate) use rw_block_b_0181;

macro_rules! rw_block_b_0182 {
    ($a_name:ident, $a_styles:ident, $ach:ident, $data:ident, $num:ident) => {
        let mut $a_name: Handle<HAsciiString> = Default::default();
        $data.read_string($num, 1, "name", $ach, &mut $a_name)?;
        let mut $a_styles: Handle<HArray1<Handle<StepVisualPresentationStyleAssignment>>> =
            Default::default();
    };
}
pub(crate) use rw_block_b_0182;
