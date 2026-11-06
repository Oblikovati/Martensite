// The RW member bodies several classes repeat.

macro_rules! rw_block_b_0000 {
    ($a_item_defined_transformation_description:ident, $a_item_defined_transformation_name:ident, $a_item_defined_transformation_transform_item1:ident, $a_item_defined_transformation_transform_item2:ident, $a_kinematic_pair_joint:ident, $a_low_order_kinematic_pair_rx:ident, $a_low_order_kinematic_pair_ry:ident, $a_low_order_kinematic_pair_rz:ident, $a_low_order_kinematic_pair_tx:ident, $a_low_order_kinematic_pair_ty:ident, $a_low_order_kinematic_pair_tz:ident, $a_lower_limit_actual_translation:ident, $a_representation_item_name:ident, $a_upper_limit_actual_translation:ident, $has_item_defined_transformation_description:ident, $has_lower_limit_actual_translation:ident, $has_upper_limit_actual_translation:ident, $the_arch:ident, $the_data:ident, $the_num:ident) => {
        rw_block_b_0005!(
            $a_item_defined_transformation_description,
            $a_item_defined_transformation_name,
            $a_item_defined_transformation_transform_item1,
            $a_item_defined_transformation_transform_item2,
            $a_kinematic_pair_joint,
            $a_low_order_kinematic_pair_rx,
            $a_low_order_kinematic_pair_ry,
            $a_low_order_kinematic_pair_rz,
            $a_low_order_kinematic_pair_tx,
            $a_low_order_kinematic_pair_ty,
            $a_low_order_kinematic_pair_tz,
            $a_representation_item_name,
            $has_item_defined_transformation_description,
            $the_arch,
            $the_data,
            $the_num
        );
        let mut $a_lower_limit_actual_translation: f64 = Default::default();
        let mut $has_lower_limit_actual_translation: bool = true;
        if $the_data.is_param_defined($the_num, 13)? {
            $the_data.read_real(
                $the_num,
                13,
                "lower_limit_actual_translation",
                $the_arch,
                &mut $a_lower_limit_actual_translation,
            )?;
        } else {
            $has_lower_limit_actual_translation = false;
            $a_lower_limit_actual_translation = 0.0;
        }
        let mut $a_upper_limit_actual_translation: f64 = Default::default();
        let mut $has_upper_limit_actual_translation: bool = true;
        if $the_data.is_param_defined($the_num, 14)? {
            $the_data.read_real(
                $the_num,
                14,
                "upper_limit_actual_translation",
                $the_arch,
                &mut $a_upper_limit_actual_translation,
            )?;
        } else {
            $has_upper_limit_actual_translation = false;
            $a_upper_limit_actual_translation = 0.0;
        }
    };
}
pub(crate) use rw_block_b_0000;

macro_rules! rw_block_b_0001 {
    ($a_item_defined_transformation_description:ident, $a_item_defined_transformation_name:ident, $a_item_defined_transformation_transform_item1:ident, $a_item_defined_transformation_transform_item2:ident, $a_kinematic_pair_joint:ident, $a_low_order_kinematic_pair_rx:ident, $a_low_order_kinematic_pair_ry:ident, $a_low_order_kinematic_pair_rz:ident, $a_low_order_kinematic_pair_tx:ident, $a_low_order_kinematic_pair_ty:ident, $a_low_order_kinematic_pair_tz:ident, $a_lower_limit_actual_rotation:ident, $a_representation_item_name:ident, $a_upper_limit_actual_rotation:ident, $has_item_defined_transformation_description:ident, $has_lower_limit_actual_rotation:ident, $has_upper_limit_actual_rotation:ident, $the_arch:ident, $the_data:ident, $the_num:ident) => {
        rw_block_b_0005!(
            $a_item_defined_transformation_description,
            $a_item_defined_transformation_name,
            $a_item_defined_transformation_transform_item1,
            $a_item_defined_transformation_transform_item2,
            $a_kinematic_pair_joint,
            $a_low_order_kinematic_pair_rx,
            $a_low_order_kinematic_pair_ry,
            $a_low_order_kinematic_pair_rz,
            $a_low_order_kinematic_pair_tx,
            $a_low_order_kinematic_pair_ty,
            $a_low_order_kinematic_pair_tz,
            $a_representation_item_name,
            $has_item_defined_transformation_description,
            $the_arch,
            $the_data,
            $the_num
        );
        let mut $a_lower_limit_actual_rotation: f64 = Default::default();
        let mut $has_lower_limit_actual_rotation: bool = true;
        if $the_data.is_param_defined($the_num, 13)? {
            $the_data.read_real(
                $the_num,
                13,
                "lower_limit_actual_rotation",
                $the_arch,
                &mut $a_lower_limit_actual_rotation,
            )?;
        } else {
            $has_lower_limit_actual_rotation = false;
            $a_lower_limit_actual_rotation = 0.0;
        }
        let mut $a_upper_limit_actual_rotation: f64 = Default::default();
        let mut $has_upper_limit_actual_rotation: bool = true;
        if $the_data.is_param_defined($the_num, 14)? {
            $the_data.read_real(
                $the_num,
                14,
                "upper_limit_actual_rotation",
                $the_arch,
                &mut $a_upper_limit_actual_rotation,
            )?;
        } else {
            $has_upper_limit_actual_rotation = false;
            $a_upper_limit_actual_rotation = 0.0;
        }
    };
}
pub(crate) use rw_block_b_0001;

macro_rules! rw_block_b_0002 {
    ($a_item_defined_transformation_description:ident, $a_item_defined_transformation_name:ident, $a_item_defined_transformation_transform_item1:ident, $a_item_defined_transformation_transform_item2:ident, $a_kinematic_pair_joint:ident, $a_low_order_kinematic_pair_rx:ident, $a_low_order_kinematic_pair_ry:ident, $a_low_order_kinematic_pair_rz:ident, $a_low_order_kinematic_pair_tx:ident, $a_low_order_kinematic_pair_ty:ident, $a_low_order_kinematic_pair_tz:ident, $a_lower_limit_yaw:ident, $a_representation_item_name:ident, $a_upper_limit_yaw:ident, $has_item_defined_transformation_description:ident, $has_lower_limit_yaw:ident, $has_upper_limit_yaw:ident, $the_arch:ident, $the_data:ident, $the_num:ident) => {
        rw_block_b_0005!(
            $a_item_defined_transformation_description,
            $a_item_defined_transformation_name,
            $a_item_defined_transformation_transform_item1,
            $a_item_defined_transformation_transform_item2,
            $a_kinematic_pair_joint,
            $a_low_order_kinematic_pair_rx,
            $a_low_order_kinematic_pair_ry,
            $a_low_order_kinematic_pair_rz,
            $a_low_order_kinematic_pair_tx,
            $a_low_order_kinematic_pair_ty,
            $a_low_order_kinematic_pair_tz,
            $a_representation_item_name,
            $has_item_defined_transformation_description,
            $the_arch,
            $the_data,
            $the_num
        );
        let mut $a_lower_limit_yaw: f64 = Default::default();
        let mut $has_lower_limit_yaw: bool = true;
        if $the_data.is_param_defined($the_num, 13)? {
            $the_data.read_real(
                $the_num,
                13,
                "lower_limit_yaw",
                $the_arch,
                &mut $a_lower_limit_yaw,
            )?;
        } else {
            $has_lower_limit_yaw = false;
            $a_lower_limit_yaw = 0.0;
        }
        let mut $a_upper_limit_yaw: f64 = Default::default();
        let mut $has_upper_limit_yaw: bool = true;
        if $the_data.is_param_defined($the_num, 14)? {
            $the_data.read_real(
                $the_num,
                14,
                "upper_limit_yaw",
                $the_arch,
                &mut $a_upper_limit_yaw,
            )?;
        } else {
            $has_upper_limit_yaw = false;
            $a_upper_limit_yaw = 0.0;
        }
    };
}
pub(crate) use rw_block_b_0002;

macro_rules! rw_block_b_0003 {
    ($a_item_defined_transformation_description:ident, $a_item_defined_transformation_name:ident, $a_item_defined_transformation_transform_item1:ident, $a_item_defined_transformation_transform_item2:ident, $a_kinematic_pair_joint:ident, $a_low_order_kinematic_pair_rx:ident, $a_low_order_kinematic_pair_ry:ident, $a_low_order_kinematic_pair_rz:ident, $a_low_order_kinematic_pair_tx:ident, $a_low_order_kinematic_pair_ty:ident, $a_low_order_kinematic_pair_tz:ident, $a_representation_item_name:ident, $a_universal_pair_input_skew_angle:ident, $has_item_defined_transformation_description:ident, $has_universal_pair_input_skew_angle:ident, $the_arch:ident, $the_data:ident, $the_num:ident) => {
        rw_block_b_0005!(
            $a_item_defined_transformation_description,
            $a_item_defined_transformation_name,
            $a_item_defined_transformation_transform_item1,
            $a_item_defined_transformation_transform_item2,
            $a_kinematic_pair_joint,
            $a_low_order_kinematic_pair_rx,
            $a_low_order_kinematic_pair_ry,
            $a_low_order_kinematic_pair_rz,
            $a_low_order_kinematic_pair_tx,
            $a_low_order_kinematic_pair_ty,
            $a_low_order_kinematic_pair_tz,
            $a_representation_item_name,
            $has_item_defined_transformation_description,
            $the_arch,
            $the_data,
            $the_num
        );
        let mut $a_universal_pair_input_skew_angle: f64 = Default::default();
        let mut $has_universal_pair_input_skew_angle: bool = true;
        if $the_data.is_param_defined($the_num, 13)? {
            $the_data.read_real(
                $the_num,
                13,
                "universal_pair.input_skew_angle",
                $the_arch,
                &mut $a_universal_pair_input_skew_angle,
            )?;
        } else {
            $has_universal_pair_input_skew_angle = false;
            $a_universal_pair_input_skew_angle = 0.0;
        }
    };
}
pub(crate) use rw_block_b_0003;

macro_rules! rw_block_b_0004 {
    ($a_item_defined_transformation_description:ident, $a_item_defined_transformation_name:ident, $a_item_defined_transformation_transform_item1:ident, $a_item_defined_transformation_transform_item2:ident, $a_kinematic_pair_joint:ident, $a_low_order_kinematic_pair_rx:ident, $a_low_order_kinematic_pair_ry:ident, $a_low_order_kinematic_pair_rz:ident, $a_low_order_kinematic_pair_tx:ident, $a_low_order_kinematic_pair_ty:ident, $a_low_order_kinematic_pair_tz:ident, $a_representation_item_name:ident, $has_item_defined_transformation_description:ident, $the_arch:ident, $the_data:ident, $the_ent:ident, $the_num:ident) => {
        rw_block_b_0005!(
            $a_item_defined_transformation_description,
            $a_item_defined_transformation_name,
            $a_item_defined_transformation_transform_item1,
            $a_item_defined_transformation_transform_item2,
            $a_kinematic_pair_joint,
            $a_low_order_kinematic_pair_rx,
            $a_low_order_kinematic_pair_ry,
            $a_low_order_kinematic_pair_rz,
            $a_low_order_kinematic_pair_tx,
            $a_low_order_kinematic_pair_ty,
            $a_low_order_kinematic_pair_tz,
            $a_representation_item_name,
            $has_item_defined_transformation_description,
            $the_arch,
            $the_data,
            $the_num
        );
        $the_ent.init(
            &$a_representation_item_name,
            &$a_item_defined_transformation_name,
            $has_item_defined_transformation_description,
            &$a_item_defined_transformation_description,
            &$a_item_defined_transformation_transform_item1,
            &$a_item_defined_transformation_transform_item2,
            &$a_kinematic_pair_joint,
            $a_low_order_kinematic_pair_tx,
            $a_low_order_kinematic_pair_ty,
            $a_low_order_kinematic_pair_tz,
            $a_low_order_kinematic_pair_rx,
            $a_low_order_kinematic_pair_ry,
            $a_low_order_kinematic_pair_rz,
        );
    };
}
pub(crate) use rw_block_b_0004;

macro_rules! rw_block_b_0005 {
    ($a_item_defined_transformation_description:ident, $a_item_defined_transformation_name:ident, $a_item_defined_transformation_transform_item1:ident, $a_item_defined_transformation_transform_item2:ident, $a_kinematic_pair_joint:ident, $a_low_order_kinematic_pair_rx:ident, $a_low_order_kinematic_pair_ry:ident, $a_low_order_kinematic_pair_rz:ident, $a_low_order_kinematic_pair_tx:ident, $a_low_order_kinematic_pair_ty:ident, $a_low_order_kinematic_pair_tz:ident, $a_representation_item_name:ident, $has_item_defined_transformation_description:ident, $the_arch:ident, $the_data:ident, $the_num:ident) => {
        rw_block_b_0022!(
            $a_item_defined_transformation_description,
            $a_item_defined_transformation_name,
            $a_item_defined_transformation_transform_item1,
            $a_item_defined_transformation_transform_item2,
            $a_kinematic_pair_joint,
            $a_representation_item_name,
            $has_item_defined_transformation_description,
            $the_arch,
            $the_data,
            $the_num
        );
        let mut $a_low_order_kinematic_pair_tx: bool = Default::default();
        $the_data.read_boolean(
            $the_num,
            7,
            "low_order_kinematic_pair.t_x",
            $the_arch,
            &mut $a_low_order_kinematic_pair_tx,
        )?;
        let mut $a_low_order_kinematic_pair_ty: bool = Default::default();
        $the_data.read_boolean(
            $the_num,
            8,
            "low_order_kinematic_pair.t_y",
            $the_arch,
            &mut $a_low_order_kinematic_pair_ty,
        )?;
        let mut $a_low_order_kinematic_pair_tz: bool = Default::default();
        $the_data.read_boolean(
            $the_num,
            9,
            "low_order_kinematic_pair.t_z",
            $the_arch,
            &mut $a_low_order_kinematic_pair_tz,
        )?;
        let mut $a_low_order_kinematic_pair_rx: bool = Default::default();
        $the_data.read_boolean(
            $the_num,
            10,
            "low_order_kinematic_pair.r_x",
            $the_arch,
            &mut $a_low_order_kinematic_pair_rx,
        )?;
        let mut $a_low_order_kinematic_pair_ry: bool = Default::default();
        $the_data.read_boolean(
            $the_num,
            11,
            "low_order_kinematic_pair.r_y",
            $the_arch,
            &mut $a_low_order_kinematic_pair_ry,
        )?;
        let mut $a_low_order_kinematic_pair_rz: bool = Default::default();
        $the_data.read_boolean(
            $the_num,
            12,
            "low_order_kinematic_pair.r_z",
            $the_arch,
            &mut $a_low_order_kinematic_pair_rz,
        )?;
    };
}
pub(crate) use rw_block_b_0005;

macro_rules! rw_block_b_0006 {
    ($a_item_defined_transformation_description:ident, $a_item_defined_transformation_name:ident, $a_item_defined_transformation_transform_item1:ident, $a_item_defined_transformation_transform_item2:ident, $a_kinematic_pair_joint:ident, $a_planar_curve_pair_curve1:ident, $a_planar_curve_pair_curve2:ident, $a_planar_curve_pair_orientation:ident, $a_representation_item_name:ident, $has_item_defined_transformation_description:ident, $the_arch:ident, $the_data:ident, $the_ent:ident, $the_num:ident) => {
        rw_block_b_0008!(
            $a_item_defined_transformation_description,
            $a_item_defined_transformation_name,
            $a_item_defined_transformation_transform_item1,
            $a_item_defined_transformation_transform_item2,
            $a_kinematic_pair_joint,
            $a_planar_curve_pair_curve1,
            $a_planar_curve_pair_curve2,
            $a_planar_curve_pair_orientation,
            $a_representation_item_name,
            $has_item_defined_transformation_description,
            $the_arch,
            $the_data,
            $the_num
        );
        $the_ent.init(
            &$a_representation_item_name,
            &$a_item_defined_transformation_name,
            $has_item_defined_transformation_description,
            &$a_item_defined_transformation_description,
            &$a_item_defined_transformation_transform_item1,
            &$a_item_defined_transformation_transform_item2,
            &$a_kinematic_pair_joint,
            &$a_planar_curve_pair_curve1,
            &$a_planar_curve_pair_curve2,
            $a_planar_curve_pair_orientation,
        );
    };
}
pub(crate) use rw_block_b_0006;

macro_rules! rw_block_b_0007 {
    ($a_item_defined_transformation_description:ident, $a_item_defined_transformation_name:ident, $a_item_defined_transformation_transform_item1:ident, $a_item_defined_transformation_transform_item2:ident, $a_kinematic_pair_joint:ident, $a_representation_item_name:ident, $a_surface_pair_orientation:ident, $a_surface_pair_surface1:ident, $a_surface_pair_surface2:ident, $has_item_defined_transformation_description:ident, $the_arch:ident, $the_data:ident, $the_ent:ident, $the_num:ident) => {
        rw_block_b_0009!(
            $a_item_defined_transformation_description,
            $a_item_defined_transformation_name,
            $a_item_defined_transformation_transform_item1,
            $a_item_defined_transformation_transform_item2,
            $a_kinematic_pair_joint,
            $a_representation_item_name,
            $a_surface_pair_orientation,
            $a_surface_pair_surface1,
            $a_surface_pair_surface2,
            $has_item_defined_transformation_description,
            $the_arch,
            $the_data,
            $the_num
        );
        $the_ent.init(
            &$a_representation_item_name,
            &$a_item_defined_transformation_name,
            $has_item_defined_transformation_description,
            &$a_item_defined_transformation_description,
            &$a_item_defined_transformation_transform_item1,
            &$a_item_defined_transformation_transform_item2,
            &$a_kinematic_pair_joint,
            &$a_surface_pair_surface1,
            &$a_surface_pair_surface2,
            $a_surface_pair_orientation,
        );
    };
}
pub(crate) use rw_block_b_0007;

macro_rules! rw_block_b_0008 {
    ($a_item_defined_transformation_description:ident, $a_item_defined_transformation_name:ident, $a_item_defined_transformation_transform_item1:ident, $a_item_defined_transformation_transform_item2:ident, $a_kinematic_pair_joint:ident, $a_planar_curve_pair_curve1:ident, $a_planar_curve_pair_curve2:ident, $a_planar_curve_pair_orientation:ident, $a_representation_item_name:ident, $has_item_defined_transformation_description:ident, $the_arch:ident, $the_data:ident, $the_num:ident) => {
        rw_block_b_0022!(
            $a_item_defined_transformation_description,
            $a_item_defined_transformation_name,
            $a_item_defined_transformation_transform_item1,
            $a_item_defined_transformation_transform_item2,
            $a_kinematic_pair_joint,
            $a_representation_item_name,
            $has_item_defined_transformation_description,
            $the_arch,
            $the_data,
            $the_num
        );
        let mut $a_planar_curve_pair_curve1: Handle<StepGeomCurve> = Default::default();
        $the_data.read_entity(
            $the_num,
            7,
            "planar_curve_pair.curve1",
            $the_arch,
            StepGeomCurve::static_type(),
            &mut $a_planar_curve_pair_curve1,
        )?;
        let mut $a_planar_curve_pair_curve2: Handle<StepGeomCurve> = Default::default();
        $the_data.read_entity(
            $the_num,
            8,
            "planar_curve_pair.curve2",
            $the_arch,
            StepGeomCurve::static_type(),
            &mut $a_planar_curve_pair_curve2,
        )?;
        let mut $a_planar_curve_pair_orientation: bool = Default::default();
        $the_data.read_boolean(
            $the_num,
            9,
            "planar_curve_pair.orientation",
            $the_arch,
            &mut $a_planar_curve_pair_orientation,
        )?;
    };
}
pub(crate) use rw_block_b_0008;

macro_rules! rw_block_b_0009 {
    ($a_item_defined_transformation_description:ident, $a_item_defined_transformation_name:ident, $a_item_defined_transformation_transform_item1:ident, $a_item_defined_transformation_transform_item2:ident, $a_kinematic_pair_joint:ident, $a_representation_item_name:ident, $a_surface_pair_orientation:ident, $a_surface_pair_surface1:ident, $a_surface_pair_surface2:ident, $has_item_defined_transformation_description:ident, $the_arch:ident, $the_data:ident, $the_num:ident) => {
        rw_block_b_0022!(
            $a_item_defined_transformation_description,
            $a_item_defined_transformation_name,
            $a_item_defined_transformation_transform_item1,
            $a_item_defined_transformation_transform_item2,
            $a_kinematic_pair_joint,
            $a_representation_item_name,
            $has_item_defined_transformation_description,
            $the_arch,
            $the_data,
            $the_num
        );
        let mut $a_surface_pair_surface1: Handle<StepGeomSurface> = Default::default();
        $the_data.read_entity(
            $the_num,
            7,
            "surface_pair.surface1",
            $the_arch,
            StepGeomSurface::static_type(),
            &mut $a_surface_pair_surface1,
        )?;
        let mut $a_surface_pair_surface2: Handle<StepGeomSurface> = Default::default();
        $the_data.read_entity(
            $the_num,
            8,
            "surface_pair.surface2",
            $the_arch,
            StepGeomSurface::static_type(),
            &mut $a_surface_pair_surface2,
        )?;
        let mut $a_surface_pair_orientation: bool = Default::default();
        $the_data.read_boolean(
            $the_num,
            9,
            "surface_pair.orientation",
            $the_arch,
            &mut $a_surface_pair_orientation,
        )?;
    };
}
pub(crate) use rw_block_b_0009;
