// The RW member bodies several classes repeat.

macro_rules! rw_block_0027 {
    ($ent:ident, $i4:ident, $j4:ident, $sw:ident) => {
        rw_block_0035!($ent, $i4, $j4, $sw);
        $sw.open_sub()?;
    };
}
pub(crate) use rw_block_0027;

macro_rules! rw_block_0028 {
    ($ent:ident, $i3:ident, $j3:ident, $sw:ident) => {
        step_start_entity($sw, "BOUNDED_SURFACE")?;
        step_start_entity($sw, "B_SPLINE_SURFACE")?;
        $sw.send_integer($ent.u_degree())?;
        $sw.send_integer($ent.v_degree())?;
        $sw.open_sub()?;
        for $i3 in 1..=$ent.nb_control_points_list_i() {
            $sw.new_line(false)?;
            $sw.open_sub()?;
            for $j3 in 1..=$ent.nb_control_points_list_j() {
                $sw.send_entity(&$ent.control_points_list_value($i3, $j3)?)?;
                $sw.join_last(false);
            }
            $sw.close_sub()?;
        }
        rw_block_0120!($ent, $sw);
    };
}
pub(crate) use rw_block_0028;

macro_rules! rw_block_0029 {
    ($ent:ident, $i2:ident, $sw:ident) => {
        rw_block_0050!($ent, $i2, $sw);
        step_start_entity($sw, "CURVE")?;
        step_start_entity($sw, "GEOMETRIC_REPRESENTATION_ITEM")?;
    };
}
pub(crate) use rw_block_0029;

macro_rules! rw_block_0030 {
    ($a_description:ident, $a_name:ident, $a_products:ident, $ach:ident, $anent3:ident, $data:ident, $ent:ident, $has_adescription:ident, $i3:ident, $nb3:ident, $nsub3:ident, $num:ident) => {
        rw_block_0098!(
            $a_description,
            $a_name,
            $ach,
            $data,
            $has_adescription,
            $num
        );
        let mut $a_products: Handle<HArray1<Handle<StepBasicProduct>>> = Default::default();
        let mut $anent3: Handle<StepBasicProduct> = Default::default();
        let mut $nsub3: i32 = Default::default();
        if $data.read_sub_list($num, 3, "products", $ach, &mut $nsub3, false, 0, 0)? {
            let $nb3: i32 = $data.nb_params($nsub3)?;
            $a_products = step_new_array1::<Handle<StepBasicProduct>>(1, $nb3)?;
            for $i3 in 1..=$nb3 {
                if $data.read_entity(
                    $nsub3,
                    $i3,
                    "product",
                    $ach,
                    StepBasicProduct::static_type(),
                    &mut $anent3,
                )? {
                    step_array1_set(&$a_products, $i3, $anent3.clone(), "ReadStep")?;
                }
            }
        }
        $ent.init(&$a_name, $has_adescription, &$a_description, &$a_products);
    };
}
pub(crate) use rw_block_0030;

macro_rules! rw_block_0031 {
    ($a_description:ident, $a_name:ident, $a_related_shape_aspect:ident, $a_relating_shape_aspect:ident, $ach:ident, $data:ident, $ent:ident, $has_description:ident, $num:ident) => {
        rw_block_0102!($a_description, $a_name, $ach, $data, $has_description, $num);
        let mut $a_relating_shape_aspect: Handle<StepReprShapeAspect> = Default::default();
        $data.read_entity(
            $num,
            3,
            "relating_shape_aspect",
            $ach,
            StepReprShapeAspect::static_type(),
            &mut $a_relating_shape_aspect,
        )?;
        let mut $a_related_shape_aspect: Handle<StepReprShapeAspect> = Default::default();
        $data.read_entity(
            $num,
            4,
            "related_shape_aspect",
            $ach,
            StepReprShapeAspect::static_type(),
            &mut $a_related_shape_aspect,
        )?;
        $ent.init(
            &$a_name,
            $has_description,
            &$a_description,
            &$a_relating_shape_aspect,
            &$a_related_shape_aspect,
        );
    };
}
pub(crate) use rw_block_0031;

macro_rules! rw_block_0032 {
    ($a_shape_aspect_relationship_description:ident, $a_shape_aspect_relationship_name:ident, $a_shape_aspect_relationship_related_shape_aspect:ident, $a_shape_aspect_relationship_relating_shape_aspect:ident, $ach:ident, $data:ident, $ent:ident, $has_shape_aspect_relationship_description:ident, $num:ident) => {
        rw_block_0043!(
            $a_shape_aspect_relationship_description,
            $a_shape_aspect_relationship_name,
            $a_shape_aspect_relationship_related_shape_aspect,
            $a_shape_aspect_relationship_relating_shape_aspect,
            $ach,
            $data,
            $has_shape_aspect_relationship_description,
            $num
        );
        $ent.init(
            &$a_shape_aspect_relationship_name,
            $has_shape_aspect_relationship_description,
            &$a_shape_aspect_relationship_description,
            &$a_shape_aspect_relationship_relating_shape_aspect,
            &$a_shape_aspect_relationship_related_shape_aspect,
        );
    };
}
pub(crate) use rw_block_0032;

macro_rules! rw_block_0033 {
    ($a_description:ident, $a_name:ident, $a_of_shape:ident, $a_product_definitional:ident, $ach:ident, $data:ident, $ent:ident, $num:ident) => {
        if !$data.check_nb_params($num, 4, $ach, "shape_aspect")? {
            return Ok(());
        }
        rw_block_0126!($a_description, $a_name, $ach, $data, $num);
        let mut $a_of_shape: Handle<StepReprProductDefinitionShape> = Default::default();
        $data.read_entity(
            $num,
            3,
            "of_shape",
            $ach,
            StepReprProductDefinitionShape::static_type(),
            &mut $a_of_shape,
        )?;
        let mut $a_product_definitional: StepDataLogical = Default::default();
        $data.read_logical(
            $num,
            4,
            "product_definitional",
            $ach,
            &mut $a_product_definitional,
        )?;
        $ent.init(
            &$a_name,
            &$a_description,
            &$a_of_shape,
            $a_product_definitional,
        );
    };
}
pub(crate) use rw_block_0033;

macro_rules! rw_block_0034 {
    ($a_conversion_factor:ident, $a_dimensions:ident, $a_name:ident, $ach:ident, $data:ident, $num:ident, $num0:ident) => {
        let mut $num: i32 = $num0;
        rw_block_0044!(
            $a_conversion_factor,
            $a_dimensions,
            $a_name,
            $ach,
            $data,
            $num
        );
    };
}
pub(crate) use rw_block_0034;

macro_rules! rw_block_0035 {
    ($ent:ident, $i4:ident, $j4:ident, $sw:ident) => {
        $sw.send_entity(&$ent.name())?;
        $sw.send_integer($ent.u_degree())?;
        $sw.send_integer($ent.v_degree())?;
        $sw.open_sub()?;
        for $i4 in 1..=$ent.nb_control_points_list_i() {
            $sw.new_line(false)?;
            $sw.open_sub()?;
            for $j4 in 1..=$ent.nb_control_points_list_j() {
                $sw.send_entity(&$ent.control_points_list_value($i4, $j4)?)?;
                $sw.join_last(false);
            }
            $sw.close_sub()?;
        }
        rw_block_0120!($ent, $sw);
    };
}
pub(crate) use rw_block_0035;

macro_rules! rw_block_0036 {
    ($a_name:ident, $a_position:ident, $a_x:ident, $a_y:ident, $a_z:ident, $ach:ident, $data:ident, $num:ident) => {
        rw_block_0140!($a_name, $a_position, $ach, $data, $num);
        let mut $a_x: f64 = Default::default();
        $data.read_real($num, 3, "x", $ach, &mut $a_x)?;
        let mut $a_y: f64 = Default::default();
        $data.read_real($num, 4, "y", $ach, &mut $a_y)?;
        let mut $a_z: f64 = Default::default();
        $data.read_real($num, 5, "z", $ach, &mut $a_z)?;
    };
}
pub(crate) use rw_block_0036;
