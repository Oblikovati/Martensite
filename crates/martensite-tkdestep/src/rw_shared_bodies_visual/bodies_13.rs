// The RW member bodies several classes repeat.

macro_rules! rw_block_b_0109 {
    ($a_items:ident, $a_name:ident, $ach:ident, $anent2:ident, $data:ident, $nsub2:ident, $num:ident) => {
        let mut $a_name: Handle<HAsciiString> = Default::default();
        $data.read_string($num, 1, "name", $ach, &mut $a_name)?;
        let mut $a_items: Handle<HArray1<Handle<StepReprRepresentationItem>>> = Default::default();
        let mut $anent2: Handle<StepReprRepresentationItem> = Default::default();
        let mut $nsub2: i32 = Default::default();
    };
}
pub(crate) use rw_block_b_0109;

macro_rules! rw_block_b_0110 {
    ($a_fea_constants:ident, $a_representation_item_name:ident, $ach:ident, $data:ident, $ent:ident, $num:ident) => {
        rw_block_b_0152!(
            $a_fea_constants,
            $a_representation_item_name,
            $ach,
            $data,
            $num
        );
        $ent.init(&$a_representation_item_name, &$a_fea_constants);
    };
}
pub(crate) use rw_block_b_0110;

macro_rules! rw_block_b_0111 {
    ($a_fea_constants:ident, $a_representation_item_name:ident, $ach:ident, $data:ident, $ent:ident, $num:ident) => {
        let mut $a_representation_item_name: Handle<HAsciiString> = Default::default();
        $data.read_string(
            $num,
            1,
            "representation_item.name",
            $ach,
            &mut $a_representation_item_name,
        )?;
        let mut $a_fea_constants: StepFEASymmetricTensor42d = Default::default();
        $data.read_entity_select($num, 2, "fea_constants", $ach, &mut $a_fea_constants)?;
        $ent.init(&$a_representation_item_name, &$a_fea_constants);
    };
}
pub(crate) use rw_block_b_0111;

macro_rules! rw_block_b_0112 {
    ($a_fea_constant:ident, $a_representation_item_name:ident, $ach:ident, $data:ident, $ent:ident, $num:ident) => {
        let mut $a_representation_item_name: Handle<HAsciiString> = Default::default();
        $data.read_string(
            $num,
            1,
            "representation_item.name",
            $ach,
            &mut $a_representation_item_name,
        )?;
        let mut $a_fea_constant: f64 = Default::default();
        $data.read_real($num, 2, "fea_constant", $ach, &mut $a_fea_constant)?;
        $ent.init(&$a_representation_item_name, $a_fea_constant);
    };
}
pub(crate) use rw_block_b_0112;

macro_rules! rw_block_b_0113 {
    ($ent:ident, $i3:ident, $sw:ident) => {
        $sw.send_entity(&$ent.assigned_date_and_time())?;
        rw_block_b_0160!($ent, $i3, $sw);
    };
}
pub(crate) use rw_block_b_0113;

macro_rules! rw_block_b_0114 {
    ($ent:ident, $i3:ident, $sw:ident) => {
        $sw.send_entity(&$ent.assigned_date())?;
        rw_block_b_0160!($ent, $i3, $sw);
    };
}
pub(crate) use rw_block_b_0114;

macro_rules! rw_block_b_0115 {
    ($ent:ident, $i3:ident, $sw:ident) => {
        $sw.send_entity(&$ent.assigned_organization())?;
        rw_block_b_0160!($ent, $i3, $sw);
    };
}
pub(crate) use rw_block_b_0115;

macro_rules! rw_block_b_0116 {
    ($ent:ident, $i3:ident, $sw:ident) => {
        $sw.send_entity(&$ent.assigned_person_and_organization())?;
        rw_block_b_0160!($ent, $i3, $sw);
    };
}
pub(crate) use rw_block_b_0116;

macro_rules! rw_block_b_0117 {
    ($ent:ident, $i3:ident, $sw:ident) => {
        $sw.send_entity(&$ent.assigned_document())?;
        $sw.send_entity(&$ent.source())?;
        $sw.open_sub()?;
        for $i3 in 1..=$ent.nb_items() {
            $sw.send_entity(&$ent.items_value($i3)?.value())?;
        }
        $sw.close_sub()?;
    };
}
pub(crate) use rw_block_b_0117;

macro_rules! rw_block_b_0118 {
    ($ent:ident, $sw:ident) => {
        rw_block_b_0161!($ent, $sw);
        $sw.send_entity(&$ent.unit_size())?;
    };
}
pub(crate) use rw_block_b_0118;

macro_rules! rw_block_b_0119 {
    ($ent:ident, $i2:ident, $sw:ident) => {
        $sw.send_entity(&$ent.name())?;
        $sw.open_sub()?;
        for $i2 in 1..=$ent.nb_styles() {
            $sw.send_entity(&$ent.styles_value($i2)?)?;
        }
        $sw.close_sub()?;
        $sw.send_entity(&$ent.item())?;
    };
}
pub(crate) use rw_block_b_0119;

macro_rules! rw_block_b_0120 {
    ($ent:ident, $i2:ident, $sw:ident) => {
        $sw.send_entity(&$ent.name())?;
        $sw.open_sub()?;
        for $i2 in 1..=$ent.nb_items() {
            $sw.send_entity(&$ent.items_value($i2)?)?;
        }
        $sw.close_sub()?;
        $sw.send_entity(&$ent.context_of_items())?;
    };
}
pub(crate) use rw_block_b_0120;

macro_rules! rw_block_b_0121 {
    ($ent:ident, $sw:ident) => {
        step_start_entity($sw, "GEOMETRIC_TOLERANCE")?;
        rw_block_b_0161!($ent, $sw);
    };
}
pub(crate) use rw_block_b_0121;

macro_rules! rw_block_b_0122 {
    ($a_first:ident, $a_last:ident, $a_type:ident, $ach:ident, $data:ident, $num0:ident) => {
        let $a_first: Vec<u8> = $data.first_complex_type($num0)?;
        let $a_last: Vec<u8> = $data.last_complex_type($num0)?;
        rw_block_b_0200!($a_first, $a_last, $a_type, $ach);
    };
}
pub(crate) use rw_block_b_0122;

macro_rules! rw_block_b_0123 {
    ($a_items:ident, $a_items_item:ident, $ach:ident, $data:ident, $i3:ident, $nb3:ident, $nsub3:ident, $num:ident, $stat3:ident) => {
        let mut $a_items: Handle<HArray1<StepAP214AutoDesignGeneralOrgItem>> = Default::default();
        let mut $a_items_item: StepAP214AutoDesignGeneralOrgItem = Default::default();
        let mut $nsub3: i32 = Default::default();
        if $data.read_sub_list($num, 3, "items", $ach, &mut $nsub3, false, 0, 0)? {
            let $nb3: i32 = $data.nb_params($nsub3)?;
            $a_items = step_new_array1::<StepAP214AutoDesignGeneralOrgItem>(1, $nb3)?;
            for $i3 in 1..=$nb3 {
                let $stat3: bool =
                    $data.read_entity_select($nsub3, $i3, "items", $ach, &mut $a_items_item)?;
                if $stat3 {
                    step_array1_set(&$a_items, $i3, $a_items_item.clone(), "ReadStep")?;
                }
            }
        }
    };
}
pub(crate) use rw_block_b_0123;
