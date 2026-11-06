// The RW member bodies several classes repeat.

macro_rules! rw_block_0066 {
    ($a_elements:ident, $a_elements_item:ident, $a_name:ident, $ach:ident, $data:ident, $ent:ident, $i2:ident, $nb2:ident, $nsub2:ident, $num:ident) => {
        let mut $a_name: Handle<HAsciiString> = Default::default();
        $data.read_string($num, 1, "name", $ach, &mut $a_name)?;
        let mut $a_elements: Handle<HArray1<StepShapeGeometricSetSelect>> = Default::default();
        let mut $a_elements_item: StepShapeGeometricSetSelect = Default::default();
        let mut $nsub2: i32 = Default::default();
        if $data.read_sub_list($num, 2, "elements", $ach, &mut $nsub2, false, 0, 0)? {
            let $nb2: i32 = $data.nb_params($nsub2)?;
            $a_elements = step_new_array1::<StepShapeGeometricSetSelect>(1, $nb2)?;
            for $i2 in 1..=$nb2 {
                if $data.read_entity_select($nsub2, $i2, "elements", $ach, &mut $a_elements_item)? {
                    step_array1_set(&$a_elements, $i2, $a_elements_item.clone(), "ReadStep")?;
                }
            }
        }
        $ent.init(&$a_name, &$a_elements);
    };
}
pub(crate) use rw_block_0066;

macro_rules! rw_block_0067 {
    ($a_basis_surface:ident, $a_name:ident, $a_reference_to_curve:ident, $ach:ident, $data:ident, $ent:ident, $num:ident) => {
        rw_block_0081!(
            $a_basis_surface,
            $a_name,
            $a_reference_to_curve,
            $ach,
            $data,
            $num
        );
        $ent.init(&$a_name, &$a_basis_surface, &$a_reference_to_curve);
    };
}
pub(crate) use rw_block_0067;

macro_rules! rw_block_0068 {
    ($a_description:ident, $a_id:ident, $a_name:ident, $ach:ident, $data:ident, $has_description:ident, $num:ident) => {
        rw_block_0110!($a_description, $a_id, $a_name, $ach, $data, $num);
        let mut $has_description: bool = true;
        if $data.is_param_defined($num, 3)? {
            $data.read_string($num, 3, "description", $ach, &mut $a_description)?;
        } else {
            $has_description = false;
        }
    };
}
pub(crate) use rw_block_0068;

macro_rules! rw_block_0069 {
    ($a_discipline_type:ident, $a_frame_of_reference:ident, $a_name:ident, $ach:ident, $data:ident, $ent:ident, $num:ident) => {
        rw_block_0129!($a_frame_of_reference, $a_name, $ach, $data, $num);
        let mut $a_discipline_type: Handle<HAsciiString> = Default::default();
        $data.read_string($num, 3, "discipline_type", $ach, &mut $a_discipline_type)?;
        $ent.init(&$a_name, &$a_frame_of_reference, &$a_discipline_type);
    };
}
pub(crate) use rw_block_0069;

macro_rules! rw_block_0070 {
    ($ent:ident, $i8:ident, $j8:ident, $sw:ident) => {
        step_start_entity($sw, "RATIONAL_B_SPLINE_SURFACE")?;
        $sw.open_sub()?;
        for $i8 in 1..=$ent.nb_weights_data_i()? {
            $sw.new_line(false)?;
            $sw.open_sub()?;
            for $j8 in 1..=$ent.nb_weights_data_j()? {
                $sw.send_real($ent.weights_data_value($i8, $j8)?)?;
                $sw.join_last(false);
            }
            $sw.close_sub()?;
        }
        $sw.close_sub()?;
        step_start_entity($sw, "REPRESENTATION_ITEM")?;
        $sw.send_entity(&$ent.name())?;
        step_start_entity($sw, "SURFACE")?;
    };
}
pub(crate) use rw_block_0070;

macro_rules! rw_block_0071 {
    ($a_name:ident, $a_position:ident, $a_radius:ident, $ach:ident, $data:ident, $ent:ident, $num:ident) => {
        rw_block_0085!($a_name, $a_position, $a_radius, $ach, $data, $num);
        $ent.init(&$a_name, &$a_position, $a_radius);
    };
}
pub(crate) use rw_block_0071;

macro_rules! rw_block_0072 {
    ($a_bound:ident, $a_name:ident, $a_orientation:ident, $ach:ident, $data:ident, $ent:ident, $num:ident) => {
        let mut $a_name: Handle<HAsciiString> = Default::default();
        $data.read_string($num, 1, "name", $ach, &mut $a_name)?;
        let mut $a_bound: Handle<StepShapeLoop> = Default::default();
        $data.read_entity(
            $num,
            2,
            "bound",
            $ach,
            StepShapeLoop::static_type(),
            &mut $a_bound,
        )?;
        let mut $a_orientation: bool = Default::default();
        $data.read_boolean($num, 3, "orientation", $ach, &mut $a_orientation)?;
        $ent.init(&$a_name, &$a_bound, $a_orientation);
    };
}
pub(crate) use rw_block_0072;

macro_rules! rw_block_0073 {
    ($a_conversion_factor:ident, $a_name:ident, $ach:ident, $data:ident, $num:ident, $num0:ident) => {
        let mut $num: i32 = $num0;
        rw_block_0083!($a_conversion_factor, $a_name, $ach, $data, $num);
    };
}
pub(crate) use rw_block_0073;

macro_rules! rw_block_0074 {
    ($a_control_points_list:ident, $a_degree:ident, $a_name:ident, $ach:ident, $anent3:ident, $data:ident, $nsub3:ident, $num:ident) => {
        let mut $a_name: Handle<HAsciiString> = Default::default();
        $data.read_string($num, 1, "name", $ach, &mut $a_name)?;
        let mut $a_degree: i32 = Default::default();
        $data.read_integer($num, 2, "degree", $ach, &mut $a_degree)?;
        let mut $a_control_points_list: Handle<HArray1<Handle<StepGeomCartesianPoint>>> =
            Default::default();
        let mut $anent3: Handle<StepGeomCartesianPoint> = Default::default();
        let mut $nsub3: i32 = Default::default();
    };
}
pub(crate) use rw_block_0074;

macro_rules! rw_block_0075 {
    ($ent:ident, $i2:ident, $sw:ident) => {
        step_start_entity($sw, "GLOBAL_UNIT_ASSIGNED_CONTEXT")?;
        $sw.open_sub()?;
        for $i2 in 1..=$ent.nb_units()? {
            $sw.send_entity(&$ent.units_value($i2)?)?;
        }
        $sw.close_sub()?;
        step_start_entity($sw, "REPRESENTATION_CONTEXT")?;
        $sw.send_entity(&$ent.context_identifier())?;
        $sw.send_entity(&$ent.context_type())?;
    };
}
pub(crate) use rw_block_0075;

macro_rules! rw_block_0076 {
    ($ent:ident, $sw:ident) => {
        step_start_entity($sw, "COMPOSITE_SHAPE_ASPECT")?;
        step_start_entity($sw, "DATUM_FEATURE")?;
        step_start_entity($sw, "SHAPE_ASPECT")?;
        rw_block_0166!($ent, $sw);
    };
}
pub(crate) use rw_block_0076;

macro_rules! rw_block_0077 {
    ($a_bounds:ident, $a_name:ident, $ach:ident, $anent2:ident, $data:ident, $i2:ident, $nb2:ident, $nsub2:ident, $num:ident) => {
        rw_block_0109!($a_bounds, $a_name, $ach, $anent2, $data, $nsub2, $num);
        if $data.read_sub_list($num, 2, "bounds", $ach, &mut $nsub2, false, 0, 0)? {
            let $nb2: i32 = $data.nb_params($nsub2)?;
            $a_bounds = step_new_array1::<Handle<StepShapeFaceBound>>(1, $nb2)?;
            for $i2 in 1..=$nb2 {
                if $data.read_entity(
                    $nsub2,
                    $i2,
                    "face_bound",
                    $ach,
                    StepShapeFaceBound::static_type(),
                    &mut $anent2,
                )? {
                    step_array1_set(&$a_bounds, $i2, $anent2.clone(), "ReadStep")?;
                }
            }
        }
    };
}
pub(crate) use rw_block_0077;
