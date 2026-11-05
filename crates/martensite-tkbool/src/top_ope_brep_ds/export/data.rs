//! The data of interferences and transitions (FDSSetT, FDS_hasUNK, FDS_copy,
//! FDS_assign, FUN_ds_samRk, FDSTdata, FDS_data, FDSIdata, FDSParameter,
//! FUN_selectTRAINTinterference).
use super::iref;
use crate::top_ope_brep_ds::{
    ListOfInterference, TopOpeBRepDSCurvePointInterference, TopOpeBRepDSDataStructure,
    TopOpeBRepDSEdgeVertexInterference, TopOpeBRepDSInterference, TopOpeBRepDSKind,
    TopOpeBRepDSTransition,
};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::ncollection::List;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum, State};

/// The kinds and indices of geometry and support: `(GT, G, ST, S)`.
pub type KindIndices = (TopOpeBRepDSKind, i32, TopOpeBRepDSKind, i32);

/// The shapes and indices of a transition: `(SB, IB, SA, IA)`.
pub type TransitionData = (ShapeEnum, i32, ShapeEnum, i32);

/// `FDSSetT(T, T0)`: the states of `t0` set on `t` before and after on FACE (indices
/// kept);
/// `fds_set_t(&mut t, &t0)`.
pub fn fds_set_t(t: &mut TopOpeBRepDSTransition, t0: &TopOpeBRepDSTransition) {
    t.set_before(t0.before(), ShapeEnum::Face);
    t.set_after(t0.after(), ShapeEnum::Face);
}

/// `FDS_hasUNK(T)`: a state UNKNOWN before or after; `fds_has_unk(&t)`.
pub fn fds_has_unk(t: &TopOpeBRepDSTransition) -> bool {
    t.before() == State::Unknown || t.after() == State::Unknown
}

/// `FDS_copy(LI, LII)`: the items of `li` appended to `lii`; `fds_copy(&l, &mut out)`.
pub fn fds_copy<T: Clone>(li: &List<T>, lii: &mut List<T>) {
    for x in li.iter() {
        lii.append(x.clone());
    }
}

/// `FDS_assign(LI, LII)`: `lii` cleared, then the items of `li` appended;
/// `fds_assign(&l, &mut out)`.
pub fn fds_assign<T: Clone>(li: &List<T>, lii: &mut List<T>) {
    lii.clear();
    fds_copy(li, lii);
}

/// `FUN_ds_samRk(BDS, Rk, LI, LIsrk)`: the shapes of ancestor rank `rk` moved from `li`
/// to the cleared `lisrk`; `fun_ds_sam_rk(&ds, 1, &mut l, &mut l1)`.
pub fn fun_ds_sam_rk(
    bds: &TopOpeBRepDSDataStructure,
    rk: i32,
    li: &mut List<Shape>,
    lisrk: &mut List<Shape>,
) {
    lisrk.clear();
    li.retain(|s| {
        if bds.ancestor_rank(s) == rk {
            lisrk.append(s.clone());
            return false;
        }
        true
    });
}

/// `FDSTdata(I, SB, IB, SA, IA)`: the shapes and indices of the transition; a null
/// handle leaves the outputs, (SHAPE, 0, SHAPE, 0) here as its callers initialize them;
/// `let (sb, ib, sa, ia) = fds_tdata(&i);`.
pub fn fds_tdata(i: &Handle<TopOpeBRepDSInterference>) -> TransitionData {
    let mut out = (ShapeEnum::Shape, 0, ShapeEnum::Shape, 0);
    fds_tdata_into(i, &mut out);
    out
}

/// Reads transition fields without overwriting caller outputs for a null handle;
/// `fds_tdata_into(&interference, &mut transition_fields)`.
pub fn fds_tdata_into(i: &Handle<TopOpeBRepDSInterference>, out: &mut TransitionData) {
    let Some(ii) = i.get() else { return };
    let t = ii.transition();
    *out = (
        t.shape_before(),
        t.index_before(),
        t.shape_after(),
        t.index_after(),
    );
}

/// `FDS_data(I, GT1, G1, ST1, S1)`: the geometry and support; a null handle leaves the
/// outputs, (UNKNOWN, 0, UNKNOWN, 0) here; `let (gt, g, st, s) = fds_data(&i);`.
pub fn fds_data(i: &Handle<TopOpeBRepDSInterference>) -> KindIndices {
    use TopOpeBRepDSKind::Unknown;
    let mut out = (Unknown, 0, Unknown, 0);
    fds_data_into(i, &mut out);
    out
}

/// Reads geometry/support fields while retaining outputs for a null handle;
/// `fds_data_into(&interference, &mut geometry_support)`.
pub fn fds_data_into(i: &Handle<TopOpeBRepDSInterference>, out: &mut KindIndices) {
    let Some(ii) = i.get() else { return };
    *out = (
        ii.geometry_type(),
        ii.geometry(),
        ii.support_type(),
        ii.support(),
    );
}

/// `FDS_data(it1, I1, GT1, G1, ST1, S1)`: at an iterator's current interference
/// (`cur`, `None` at the end: false and the outputs unchanged), `i1` and the data are
/// set; whether its transition is unknown; StandardNullObject for a null current
/// handle; `fds_data_iter(l.iter().next(), &mut i1, &mut d)?`.
pub fn fds_data_iter(
    cur: Option<&Handle<TopOpeBRepDSInterference>>,
    i1: &mut Handle<TopOpeBRepDSInterference>,
    data: &mut KindIndices,
) -> Result<bool, Failure> {
    let Some(cur) = cur else {
        return Ok(false);
    };
    *i1 = cur.clone();
    let t1 = iref(i1)?.transition();
    fds_data_into(i1, data);
    Ok(t1.is_unknown())
}

/// `FDSIdata(I, SB, IB, SA, IA, GT1, G1, ST1, S1)`: the transition data then the
/// geometry and support; `let ((sb, ib, sa, ia), (gt, g, st, s)) = fds_idata(&i);`.
pub fn fds_idata(i: &Handle<TopOpeBRepDSInterference>) -> (TransitionData, KindIndices) {
    (fds_tdata(i), fds_data(i))
}

/// Reads both field groups, preserving both groups when the handle is null;
/// `fds_idata_into(&interference, &mut transition_fields, &mut geometry_support)`.
pub fn fds_idata_into(
    i: &Handle<TopOpeBRepDSInterference>,
    transition: &mut TransitionData,
    geometry_support: &mut KindIndices,
) {
    fds_tdata_into(i, transition);
    fds_data_into(i, geometry_support);
}

/// `FDSParameter(I, par)`: the parameter of an edge/vertex or curve/point
/// interference (`None` for another kind); `fds_parameter_checked(&i)?`.
pub fn fds_parameter_checked(i: &Handle<TopOpeBRepDSInterference>) -> Result<Option<f64>, Failure> {
    iref(i)?;
    let is_evi = i.is_kind_name("TopOpeBRepDSEdgeVertexInterference");
    let is_cpi = i.is_kind_name("TopOpeBRepDSCurvePointInterference");
    if !is_evi && !is_cpi {
        return Ok(None);
    }
    fds_parameter(i).map(Some)
}

/// `FDSParameter(I)`: the parameter of an edge/vertex or curve/point interference;
/// StandardFailure for another kind; `fds_parameter(&i)?`.
pub fn fds_parameter(i: &Handle<TopOpeBRepDSInterference>) -> Result<f64, Failure> {
    iref(i)?;
    let evi: Handle<TopOpeBRepDSEdgeVertexInterference> = i.cast();
    if let Some(evi) = evi.get() {
        return Ok(evi.parameter());
    }
    let cpi: Handle<TopOpeBRepDSCurvePointInterference> = i.cast();
    if let Some(cpi) = cpi.get() {
        return Ok(cpi.parameter());
    }
    Err(Failure::new("FDS_Parameter"))
}

/// `FUN_selectTRAINTinterference(li, liINTERNAL)`: the cleared `li_internal` filled
/// with the interferences INTERNAL seen from IN; whether there is any;
/// `fun_select_traint_interference(&l, &mut lint)?`.
pub fn fun_select_traint_interference(
    li: &ListOfInterference,
    li_internal: &mut ListOfInterference,
) -> Result<bool, Failure> {
    li_internal.clear();
    for i in li.iter() {
        let ori = iref(i)?
            .transition()
            .orientation(State::In, ShapeEnum::Face);
        if ori == Orientation::Internal {
            li_internal.append(i.clone());
        }
    }
    Ok(!li_internal.is_empty())
}
