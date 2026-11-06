// The RW member bodies several classes repeat.

macro_rules! rw_block_0215 {
    ($ent:ident, $sw:ident) => {
        if $ent.angle_selection() == StepShapeAngleRelator::Equal {
            $sw.send_enum(b".EQUAL.")?;
        } else {
            if $ent.angle_selection() == StepShapeAngleRelator::Large {
                $sw.send_enum(b".LARGE.")?;
            } else {
                if $ent.angle_selection() == StepShapeAngleRelator::Small {
                    $sw.send_enum(b".SMALL.")?;
                }
            }
        }
    };
}
pub(crate) use rw_block_0215;
