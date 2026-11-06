// The RW member bodies several classes repeat.

macro_rules! rw_block_b_0154 {
    ($a_representation_items:ident, $a_representation_name:ident, $ach:ident, $data:ident, $num:ident, $sub2:ident) => {
        let mut $a_representation_name: Handle<HAsciiString> = Default::default();
        $data.read_string(
            $num,
            1,
            "representation.name",
            $ach,
            &mut $a_representation_name,
        )?;
        let mut $a_representation_items: Handle<HArray1<Handle<StepReprRepresentationItem>>> =
            Default::default();
        let mut $sub2: i32 = 0;
    };
}
pub(crate) use rw_block_b_0154;

macro_rules! rw_block_b_0155 {
    ($ent:ident, $sw:ident) => {
        $sw.send_entity(&$ent.parent.name())?;
        $sw.send_entity(&$ent.parent.description())?;
        $sw.send_entity(&$ent.parent.magnitude())?;
        $sw.send_entity(&$ent.parent.toleranced_shape_aspect().value())?;
    };
}
pub(crate) use rw_block_b_0155;

macro_rules! rw_block_b_0156 {
    ($a_ambient_reflectance:ident, $a_diffuse_reflectance:ident, $the_ach:ident, $the_data:ident, $the_num:ident) => {
        let mut $a_ambient_reflectance: f64 = Default::default();
        $the_data.read_real(
            $the_num,
            1,
            "ambient_reflectance",
            $the_ach,
            &mut $a_ambient_reflectance,
        )?;
        let mut $a_diffuse_reflectance: f64 = Default::default();
        $the_data.read_real(
            $the_num,
            2,
            "diffuse_reflectance",
            $the_ach,
            &mut $a_diffuse_reflectance,
        )?;
    };
}
pub(crate) use rw_block_b_0156;

macro_rules! rw_block_b_0157 {
    ($ent:ident, $i2:ident, $sw:ident) => {
        $sw.send_entity(&$ent.name())?;
        $sw.open_sub()?;
        for $i2 in 1..=$ent.nb_collected_text()? {
            $sw.send_entity(&$ent.collected_text_value($i2)?.value())?;
        }
        $sw.close_sub()?;
    };
}
pub(crate) use rw_block_b_0157;

macro_rules! rw_block_b_0158 {
    ($ent:ident, $sw:ident) => {
        $sw.send_entity(&$ent.parent.name())?;
        $sw.send_entity(&$ent.parent.description())?;
        $sw.send_entity(&$ent.parent.of_shape())?;
        $sw.send_logical($ent.parent.product_definitional())?;
    };
}
pub(crate) use rw_block_b_0158;

macro_rules! rw_block_b_0159 {
    ($ent:ident, $i2:ident, $sw:ident) => {
        $sw.send_entity(&$ent.assigned_approval())?;
        rw_block_b_0195!($ent, $i2, $sw);
    };
}
pub(crate) use rw_block_b_0159;

macro_rules! rw_block_b_0160 {
    ($ent:ident, $i3:ident, $sw:ident) => {
        $sw.send_entity(&$ent.role())?;
        $sw.open_sub()?;
        for $i3 in 1..=$ent.nb_items()? {
            $sw.send_entity(&$ent.items_value($i3)?.value())?;
        }
        $sw.close_sub()?;
    };
}
pub(crate) use rw_block_b_0160;

macro_rules! rw_block_b_0161 {
    ($ent:ident, $sw:ident) => {
        $sw.send_entity(&$ent.name())?;
        $sw.send_entity(&$ent.description())?;
        $sw.send_entity(&$ent.magnitude())?;
        $sw.send_entity(&$ent.toleranced_shape_aspect().value())?;
    };
}
pub(crate) use rw_block_b_0161;

macro_rules! rw_block_b_0162 {
    ($ent:ident, $sw:ident) => {
        $sw.send_entity(&$ent.parent.parent.name())?;
        $sw.send_entity(&$ent.parent.parent.description())?;
        $sw.send_entity(&$ent.parent.model_ref())?;
        $sw.open_sub()?;
    };
}
pub(crate) use rw_block_b_0162;

macro_rules! rw_block_b_0163 {
    ($ent:ident, $i:ident, $sw:ident) => {
        $sw.open_sub()?;
        for $i in 1..=$ent.nb_styles() {
            $sw.send_entity(&$ent.styles_value($i)?)?;
        }
        $sw.close_sub()?;
        $sw.send_entity(&$ent.item())?;
    };
}
pub(crate) use rw_block_b_0163;

macro_rules! rw_block_b_0164 {
    ($ent:ident, $i:ident, $sw:ident) => {
        $sw.send_entity(&$ent.zone())?;
        $sw.open_sub()?;
        for $i in 1..=$ent.nb_boundaries() {
            $sw.send_entity(&$ent.boundaries_value($i)?)?;
        }
        $sw.close_sub()?;
    };
}
pub(crate) use rw_block_b_0164;

macro_rules! rw_block_b_0165 {
    ($ent:ident, $sw:ident) => {
        $sw.send_entity(&$ent.name())?;
        $sw.send_entity(&$ent.description())?;
        $sw.send_entity(&$ent.of_shape())?;
        $sw.send_logical($ent.product_definitional())?;
    };
}
pub(crate) use rw_block_b_0165;

macro_rules! rw_block_b_0166 {
    ($the_ent:ident, $the_sw:ident) => {
        $the_sw.send_entity(&$the_ent.name())?;
        $the_sw.send_entity(&$the_ent.coordinates())?;
        $the_sw.send_integer($the_ent.pnmax())?;
        $the_sw.open_sub()?;
    };
}
pub(crate) use rw_block_b_0166;

macro_rules! rw_block_b_0167 {
    ($a_base_type:ident, $an_array:ident, $ent:ident, $i:ident, $iter:ident, $nb:ident) => {
        $iter.add_item(&$ent.of_shape().cast());
        let $a_base_type: i32 = $ent.base().case_num(&$ent.base().value());
        if $a_base_type == 1 {
            $iter.add_item(&$ent.base().datum().cast());
        } else {
            if $a_base_type == 2 {
                let $an_array: Handle<HArray1<Handle<StepDimTolDatumReferenceElement>>> =
                    $ent.base().common_datum_list();
                let $i: i32 = Default::default();
                let $nb: i32 = if $an_array.is_null() {
                    0
                } else {
                    $an_array.array1().length()
                };
                for $i in 1..=$nb {
                    $iter.add_item(&step_array1_item(&$an_array, $i, "Share")?.cast());
                }
            }
        }
    };
}
pub(crate) use rw_block_b_0167;
