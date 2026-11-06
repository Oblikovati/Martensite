// The RW member bodies several classes repeat.

macro_rules! rw_block_0154 {
    ($ent:ident, $has_aaxis:ident, $sw:ident) => {
        $sw.send_entity(&$ent.name())?;
        $sw.send_entity(&$ent.location())?;
        let $has_aaxis: bool = $ent.has_axis();
        if $has_aaxis {
            $sw.send_entity(&$ent.axis())?;
        } else {
            $sw.send_undef()?;
        }
    };
}
pub(crate) use rw_block_0154;

macro_rules! rw_block_0155 {
    ($a_representation_items:ident, $a_representation_name:ident, $ach:ident, $data:ident, $num:ident, $sub2:ident) => {
        rw_block_0185!(
            $a_representation_items,
            $a_representation_name,
            $ach,
            $data,
            $num
        );
        let mut $sub2: i32 = 0;
    };
}
pub(crate) use rw_block_0155;

macro_rules! rw_block_0156 {
    ($a_coordinate_space_dimension:ident, $ach:ident, $data:ident, $num:ident, $num0:ident) => {
        let mut $num: i32 = $num0;
        if !$data.check_nb_params($num, 1, $ach, "geometric_representation_context")? {
            return Ok(());
        }
        let mut $a_coordinate_space_dimension: i32 = Default::default();
        $data.read_integer(
            $num,
            1,
            "coordinate_space_dimension",
            $ach,
            &mut $a_coordinate_space_dimension,
        )?;
    };
}
pub(crate) use rw_block_0156;

macro_rules! rw_block_0157 {
    ($a_day_component:ident, $a_year_component:ident, $ach:ident, $data:ident, $num:ident) => {
        let mut $a_year_component: i32 = Default::default();
        $data.read_integer($num, 1, "year_component", $ach, &mut $a_year_component)?;
        let mut $a_day_component: i32 = Default::default();
        $data.read_integer($num, 2, "day_component", $ach, &mut $a_day_component)?;
    };
}
pub(crate) use rw_block_0157;

macro_rules! rw_block_0158 {
    ($ent:ident, $sw:ident) => {
        $sw.send_entity(&$ent.parent.name())?;
        $sw.send_entity(&$ent.parent.description())?;
        $sw.send_entity(&$ent.parent.of_shape())?;
        $sw.send_logical($ent.parent.product_definitional())?;
    };
}
pub(crate) use rw_block_0158;

macro_rules! rw_block_0159 {
    ($ent:ident, $i2:ident, $sw:ident) => {
        $sw.send_entity(&$ent.name())?;
        $sw.open_sub()?;
        for $i2 in 1..=$ent.nb_elements() {
            $sw.send_entity(&$ent.elements_value($i2)?.value())?;
        }
        $sw.close_sub()?;
    };
}
pub(crate) use rw_block_0159;

macro_rules! rw_block_0160 {
    ($ach:ident, $data:ident, $num:ident, $num0:ident) => {
        rw_block_0206!($ach, $data, $num, $num0);
        $num = $data.next_for_complex($num);
    };
}
pub(crate) use rw_block_0160;

macro_rules! rw_block_0161 {
    ($ent:ident, $i2:ident, $sw:ident) => {
        $sw.send_entity(&$ent.name())?;
        $sw.open_sub()?;
        for $i2 in 1..=$ent.nb_cfs_faces()? {
            $sw.send_entity(&$ent.cfs_faces_value($i2)?)?;
        }
        $sw.close_sub()?;
    };
}
pub(crate) use rw_block_0161;

macro_rules! rw_block_0162 {
    ($ent:ident, $i2:ident, $sw:ident) => {
        $sw.send_entity(&$ent.name())?;
        $sw.open_sub()?;
        for $i2 in 1..=$ent.nb_bounds()? {
            $sw.send_entity(&$ent.bounds_value($i2)?)?;
        }
        $sw.close_sub()?;
    };
}
pub(crate) use rw_block_0162;

macro_rules! rw_block_0163 {
    ($ent:ident, $i:ident, $sw:ident) => {
        $sw.open_sub()?;
        for $i in 1..=$ent.nb_items() {
            $sw.send_entity(&$ent.items_value($i)?)?;
        }
        $sw.close_sub()?;
        $sw.send_entity(&$ent.context_of_items())?;
    };
}
pub(crate) use rw_block_0163;

macro_rules! rw_block_0164 {
    ($ent:ident, $sw:ident) => {
        $sw.send_entity(&$ent.id())?;
        $sw.send_entity(&$ent.description())?;
        $sw.send_entity(&$ent.formation())?;
        $sw.send_entity(&$ent.frame_of_reference())?;
    };
}
pub(crate) use rw_block_0164;

macro_rules! rw_block_0165 {
    ($ent:ident, $sw:ident) => {
        $sw.send_entity(&$ent.name())?;
        $sw.send_entity(&$ent.description())?;
        $sw.send_entity(&$ent.rep1())?;
        $sw.send_entity(&$ent.rep2())?;
    };
}
pub(crate) use rw_block_0165;

macro_rules! rw_block_0166 {
    ($ent:ident, $sw:ident) => {
        $sw.send_entity(&$ent.name())?;
        $sw.send_entity(&$ent.description())?;
        $sw.send_entity(&$ent.of_shape())?;
        $sw.send_logical($ent.product_definitional())?;
    };
}
pub(crate) use rw_block_0166;

macro_rules! rw_block_0167 {
    ($ent:ident, $sw:ident) => {
        $sw.send_entity(&$ent.name())?;
        $sw.send_entity(&$ent.position())?;
        $sw.send_real($ent.major_radius())?;
        $sw.send_real($ent.minor_radius())?;
    };
}
pub(crate) use rw_block_0167;

macro_rules! rw_block_0168 {
    ($ent:ident, $sw:ident) => {
        $sw.send_entity(&$ent.name())?;
        $sw.send_entity(&$ent.position())?;
        $sw.send_real($ent.height())?;
        $sw.send_real($ent.radius())?;
    };
}
pub(crate) use rw_block_0168;
