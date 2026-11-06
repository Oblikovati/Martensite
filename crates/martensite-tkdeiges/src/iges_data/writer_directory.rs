//! The fields of the directory entry an entity writes.
use super::{
    IgesDataDefList, IgesDataDefType, IgesDataDirValues, IgesDataIgesEntity, IgesDataIgesModel,
};

/// The fields of the directory entry of an entity in a model. The parameter start and count are
/// computed on printing.
pub(super) fn directory_values(
    live: &IgesDataIgesModel,
    ent: &IgesDataIgesEntity,
) -> IgesDataDirValues {
    let mut fields = IgesDataDirValues::default();
    let v = &mut fields.values;
    let dnum = |field: i32| live.d_num(&ent.dir_field_entity(field));
    v[0] = ent.type_number();
    v[2] = if ent.has_structure() { -dnum(3) } else { 0 };
    v[3] = match ent.def_line_font() {
        IgesDataDefType::DefReference => -dnum(4),
        IgesDataDefType::DefValue => ent.rank_line_font(),
        _ => 0,
    };
    v[4] = match ent.def_level() {
        IgesDataDefList::DefSeveral => -dnum(5),
        IgesDataDefList::DefOne => ent.level(),
        _ => 0,
    };
    v[5] = match ent.def_view() {
        IgesDataDefList::DefSeveral | IgesDataDefList::DefOne => dnum(6),
        _ => 0,
    };
    v[6] = if ent.has_transf() { dnum(7) } else { 0 };
    v[7] = if ent.has_label_display() { dnum(8) } else { 0 };
    v[8] = ent.blank_status();
    v[9] = ent.subordinate_status();
    v[10] = ent.use_flag();
    v[11] = ent.hierarchy_status();
    v[12] = v[0];
    v[13] = ent.line_weight_number();
    v[14] = match ent.def_color() {
        IgesDataDefType::DefReference => -dnum(13),
        IgesDataDefType::DefValue => ent.rank_color(),
        _ => 0,
    };
    v[16] = ent.form_number();
    ent.c_res_values(&mut fields.res1, &mut fields.res2);
    fields.label = label_field(ent);
    fields.subscript = subscript_field(ent);
    fields
}

fn label_field(ent: &IgesDataIgesEntity) -> [u8; 9] {
    let mut label = [b' '; 9];
    label[8] = 0;
    if let Some(short) = ent.short_label().get() {
        for (slot, byte) in label
            .iter_mut()
            .zip(short.string().as_bytes().iter().take(8))
        {
            *slot = *byte;
        }
    }
    label
}

fn subscript_field(ent: &IgesDataIgesEntity) -> [u8; 9] {
    let mut snum = [b' '; 9];
    snum[8] = 0;
    if !ent.has_sub_script_number() {
        return snum;
    }
    snum[7] = b'0';
    let mut sn = ent.sub_script_number();
    let mut i = 7;
    while sn != 0 && i < 8 {
        snum[i] = (sn % 10) as u8 + b'0';
        sn /= 10;
        // A subscript longer than the field keeps its low digits: the field holds eight.
        if i == 0 {
            break;
        }
        i -= 1;
    }
    snum
}
