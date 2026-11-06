// The RW member bodies several classes repeat.

macro_rules! rw_block_b_0204 {
    ($i1:ident, $iter:ident, $the_ent:ident, $var0:ident) => {
        for $i1 in 1..=step_live(&$the_ent.parent.items(), "Share")?
            .array1()
            .length()
        {
            let $var0: Handle<StepReprRepresentationItem> =
                step_array1_item(&$the_ent.parent.items(), $i1, "Share")?;
            $iter.add_item(&$var0.cast());
        }
        $iter.add_item(&$the_ent.parent.context_of_items().cast());
    };
}
pub(crate) use rw_block_b_0204;

macro_rules! rw_block_b_0205 {
    ($ent:ident, $i2:ident, $iter:ident, $var0:ident) => {
        $iter.add_item(&$ent.parent.assigned_action().cast());
        for $i2 in 1..=step_live(&$ent.items(), "Share")?.array1().length() {
            let $var0: StepAP203WorkItem = step_array1_item(&$ent.items(), $i2, "Share")?;
            $iter.add_item(&$var0.value());
        }
    };
}
pub(crate) use rw_block_b_0205;

macro_rules! rw_block_b_0206 {
    ($ent:ident, $i1:ident, $iter:ident, $var0:ident) => {
        for $i1 in 1..=$ent.parent.parent.nb_items() {
            let $var0: Handle<StepReprRepresentationItem> =
                step_array1_item(&$ent.parent.parent.items(), $i1, "Share")?;
            $iter.add_item(&$var0.cast());
        }
        $iter.add_item(&$ent.parent.parent.context_of_items().cast());
    };
}
pub(crate) use rw_block_b_0206;

macro_rules! rw_block_b_0207 {
    ($ent:ident, $i1:ident, $iter:ident, $var0:ident) => {
        for $i1 in 1..=$ent.parent.nb_items() {
            let $var0: Handle<StepReprRepresentationItem> =
                step_array1_item(&$ent.parent.items(), $i1, "Share")?;
            $iter.add_item(&$var0.cast());
        }
        $iter.add_item(&$ent.parent.context_of_items().cast());
    };
}
pub(crate) use rw_block_b_0207;

macro_rules! rw_block_b_0208 {
    ($the_ent:ident, $the_sw:ident) => {
        if $the_ent.has_lower_limit_actual_rotation() {
            $the_sw.send_real($the_ent.lower_limit_actual_rotation())?;
        } else {
            $the_sw.send_undef()?;
        }
        if $the_ent.has_upper_limit_actual_rotation() {
            $the_sw.send_real($the_ent.upper_limit_actual_rotation())?;
        } else {
            $the_sw.send_undef()?;
        }
    };
}
pub(crate) use rw_block_b_0208;

macro_rules! rw_block_b_0209 {
    ($the_ent:ident, $the_sw:ident) => {
        if $the_ent.has_lower_limit_roll() {
            $the_sw.send_real($the_ent.lower_limit_roll())?;
        } else {
            $the_sw.send_undef()?;
        }
        if $the_ent.has_upper_limit_roll() {
            $the_sw.send_real($the_ent.upper_limit_roll())?;
        } else {
            $the_sw.send_undef()?;
        }
    };
}
pub(crate) use rw_block_b_0209;

macro_rules! rw_block_b_0210 {
    ($the_ent:ident, $the_sw:ident) => {
        if $the_ent.has_lower_limit_yaw() {
            $the_sw.send_real($the_ent.lower_limit_yaw())?;
        } else {
            $the_sw.send_undef()?;
        }
        if $the_ent.has_upper_limit_yaw() {
            $the_sw.send_real($the_ent.upper_limit_yaw())?;
        } else {
            $the_sw.send_undef()?;
        }
    };
}
pub(crate) use rw_block_b_0210;

macro_rules! rw_block_b_0211 {
    ($ent:ident, $iter:ident) => {
        $iter.add_item(
            &step_live(&$ent.externally_defined_item(), "Share")?
                .item_id()
                .value(),
        );
        $iter.add_item(
            &step_live(&$ent.externally_defined_item(), "Share")?
                .source()
                .cast(),
        );
    };
}
pub(crate) use rw_block_b_0211;

macro_rules! rw_block_b_0212 {
    ($iter:ident, $the_ent:ident) => {
        $iter.add_item(
            &step_live(&$the_ent.item_defined_transformation(), "Share")?
                .transform_item1()
                .cast(),
        );
        $iter.add_item(
            &step_live(&$the_ent.item_defined_transformation(), "Share")?
                .transform_item2()
                .cast(),
        );
    };
}
pub(crate) use rw_block_b_0212;

macro_rules! rw_block_b_0213 {
    ($ent:ident, $sw:ident) => {
        $sw.send_entity(
            &step_live(&$ent.externally_defined_item(), "WriteStep")?
                .item_id()
                .value(),
        )?;
        $sw.send_entity(&step_live(&$ent.externally_defined_item(), "WriteStep")?.source())?;
    };
}
pub(crate) use rw_block_b_0213;

macro_rules! rw_block_b_0214 {
    ($ent:ident, $sw:ident) => {
        $sw.send_entity(&$ent.parent.name())?;
        if $ent.parent.has_description() {
            $sw.send_entity(&$ent.parent.description())?;
        } else {
            $sw.send_undef()?;
        }
    };
}
pub(crate) use rw_block_b_0214;

macro_rules! rw_block_b_0215 {
    ($the_ent:ident, $the_iter:ident) => {
        $the_iter.add_item(&$the_ent.parent.coordinates().cast());
        if $the_ent.parent.has_geometric_link() {
            $the_iter.add_item(&$the_ent.parent.geometric_link().value());
        }
    };
}
pub(crate) use rw_block_b_0215;

macro_rules! rw_block_b_0216 {
    ($ent:ident, $i3:ident, $iter:ident) => {
        $iter.add_item(&$ent.assigned_document().cast());
        for $i3 in 1..=$ent.nb_items() {
            $iter.add_item(&$ent.items_value($i3)?.value());
        }
    };
}
pub(crate) use rw_block_b_0216;
