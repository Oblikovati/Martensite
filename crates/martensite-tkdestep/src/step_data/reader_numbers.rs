//! The resolution of references: the entity numbers of the parameters are turned from
//! file idents into record numbers. A first pass resolves everything by a map; when an ident
//! is defined several times (as with scopes), a second pass searches by scanning the
//! sequence of entities and scopes around the reference. The sub-lists are recorded
//! before they are referenced, so a stack of them resolves their parameters.
use super::StepDataStepReaderData;
use super::reader_data::c_atoi;
use martensite_tkernel::{
    message::Message,
    ncollection::IndexedMap,
    standard::{Failure, exceptions},
};
use martensite_tkxsbase::interface::{
    InterfaceFileReaderData, InterfaceFileReaderDataParams, InterfaceParamType as Param,
};
use std::fmt::Write as _;

/// The tables of the second pass: `inds` record or chained scope, `indi` ident or scope
/// mark (-1 SCOPE, -3 ENDSCOPE), `indr` the inverse of `inds`, `indx` the EXPORT lists.
pub(super) struct ScanTables {
    pub(super) inds: Vec<i32>,
    pub(super) indi: Vec<i32>,
    pub(super) indr: Vec<i32>,
    pub(super) indx: Option<Vec<i32>>,
    pub(super) nbseq: i32,
}

/// The first pass's map, shared with the second.
pub(super) struct IdentMap {
    imap: IndexedMap<i32>,
    pub(super) indm: Vec<i32>,
    nbmap: usize,
}

impl IdentMap {
    /// Adds an ident; `Some(index)` of a first definition, `None` for an overlap.
    fn add(&mut self, ident: i32) -> Option<usize> {
        let index = self.imap.add(ident);
        if index <= self.nbmap {
            return None;
        }
        self.nbmap = index;
        Some(index)
    }
    pub(super) fn index_of(&self, ident: i32) -> usize {
        self.imap.find_index(&ident)
    }
}

fn bad_scope(num: i32) -> Failure {
    exceptions::no_such_object(&format!(
        "StepDataStepReaderData: ENDSCOPE at record {num}; expected an open SCOPE"
    ))
}

fn bad_sub_stack(num: i32) -> Failure {
    exceptions::range_error(&format!(
        "StepDataStepReaderData: a sub-list parameter of record {num} has no recorded sub-list"
    ))
}

impl StepDataStepReaderData {
    /// `find_entity_number(num, id)`: the entity number of the parameter of `num` that
    /// refers to `#id`, 0 when none.
    pub(super) fn find_entity_number(&self, num: i32, id: i32) -> Result<i32, Failure> {
        if num == 0 {
            return Ok(0);
        }
        for rank in 1..=self.nb_params(num)? {
            let param = self.param(num, rank)?;
            if param.param_type() != Param::ParamIdent {
                continue;
            }
            if c_atoi(param.c_value().get(1..).unwrap_or_default()) == id {
                return Ok(param.entity_number());
            }
        }
        Ok(0)
    }
    /// `set_entity_numbers(withmap = true)`.
    pub fn set_entity_numbers(&self, withmap: bool) -> Result<(), Failure> {
        let nbdirec = self.nb_records();
        let (idents, thelastn, thenbents, thenbscop) = {
            let state = self.state.read();
            (
                state.theidents.clone(),
                state.thelastn,
                state.thenbents,
                state.thenbscop,
            )
        };
        let mut map = IdentMap {
            imap: IndexedMap::new(thenbents.max(1) as usize),
            indm: vec![0; nbdirec.max(0) as usize + 1],
            nbmap: 0,
        };
        let mut pbmap = false;
        for num in 1..=nbdirec {
            let ident = idents[num as usize];
            if ident <= 0 {
                continue;
            }
            match map.add(ident) {
                Some(index) => map.indm[index] = num,
                None => {
                    let index = map.index_of(ident);
                    map.indm[index] = -1;
                    pbmap = true;
                }
            }
        }
        self.first_pass(&idents, thelastn, &map, pbmap)?;
        if !pbmap {
            return Ok(());
        }
        let _ = write!(Message::send_trace(), " --  2nd pass required --");
        map.imap.clear(false);
        map.nbmap = 0;
        map.indm.iter_mut().for_each(|value| *value = 0);
        let tables = self.build_scan_tables(&idents, thenbents + 2 * thenbscop, &mut map)?;
        if tables.indx.is_some() {
            self.resolve_exports(&tables)?;
        }
        self.second_pass(&tables, &map, withmap)
    }
    /// The parameters by the map; a conflict is left negated for the second pass.
    fn first_pass(
        &self,
        idents: &[i32],
        thelastn: i32,
        map: &IdentMap,
        pbmap: bool,
    ) -> Result<(), Failure> {
        let mut subn = vec![0; thelastn.max(0) as usize + 1];
        for num in 1..=self.nb_records() {
            let ident = idents[num as usize];
            if ident < -2 {
                subn[(-(ident + 2)) as usize] = num;
            }
            let nda = if num == 1 {
                0
            } else {
                self.param_first_rank(num - 1)?
            };
            for na in (1..=self.nb_params(num)?).rev() {
                self.first_pass_param(num, na, nda + na, ident, thelastn, &subn, map, pbmap)?;
            }
        }
        Ok(())
    }
    #[allow(clippy::too_many_arguments)]
    fn first_pass_param(
        &self,
        num: i32,
        na: i32,
        rank: i32,
        ident: i32,
        thelastn: i32,
        subn: &[i32],
        map: &IdentMap,
        pbmap: bool,
    ) -> Result<(), Failure> {
        let (kind, number) =
            self.change_parameter(rank, |fp| (fp.param_type(), fp.entity_number()))?;
        if kind == Param::ParamSub {
            if number > thelastn {
                let _ = writeln!(
                    Message::send_info(),
                    "Bad Sub.N0, Record {num} Param {na}:${number}"
                );
                return Ok(());
            }
            let resolved = subn[number.max(0) as usize];
            return self.change_parameter(rank, |fp| fp.set_entity_number(resolved));
        }
        if kind != Param::ParamIdent {
            return Ok(());
        }
        let index = map.index_of(number);
        if index > 0 {
            let num0 = map.indm[index];
            let resolved = if num0 > 0 { num0 } else { -number };
            return self.change_parameter(rank, |fp| fp.set_entity_number(resolved));
        }
        if pbmap {
            return self.change_parameter(rank, |fp| fp.set_entity_number(-number));
        }
        self.thecheck.add_fail_text(
            &format!("Unresolved Reference, Ent.Id.#{ident} Param.n0 {na} (Id.#{number})"),
            "Unresolved Reference",
        );
        let _ = writeln!(
            Message::send_trace(),
            "*** ERR StepReaderData *** Entite #{ident}\n    Type:{}  Param.n0 {na}: #{number} Not found",
            self.c_type(num)
        );
        Ok(())
    }
    /// The sequence of entities and scopes of the second pass, and its map.
    fn build_scan_tables(
        &self,
        idents: &[i32],
        nbseq: i32,
        map: &mut IdentMap,
    ) -> Result<ScanTables, Failure> {
        let nbdirec = self.nb_records();
        let thenbscop = self.state.read().thenbscop;
        let places = nbseq.max(0) as usize + 1;
        let mut tables = ScanTables {
            inds: vec![0; places],
            indi: vec![0; places],
            indr: vec![0; nbdirec.max(0) as usize + 1],
            indx: None,
            nbseq,
        };
        let mut scopile: Vec<i32> = Vec::new();
        let mut nr = 0;
        for num in 1..=nbdirec {
            let ident = idents[num as usize];
            if ident < -2 {
                tables.indr[num as usize] = nr + 1;
            } else if ident >= 0 {
                nr += 1;
                tables.inds[nr as usize] = num;
                tables.indi[nr as usize] = ident;
                tables.indr[num as usize] = nr;
                if ident > 0 {
                    self.map_ident(ident, nr, thenbscop, &tables, map);
                }
            } else if ident == -1 {
                nr += 1;
                tables.inds[nr as usize] = num;
                tables.indi[nr as usize] = -1;
                scopile.push(nr);
            } else {
                let nscop = scopile.pop().ok_or_else(|| bad_scope(num))?;
                nr += 1;
                tables.inds[nr as usize] = nscop;
                tables.indi[nr as usize] = -3;
                tables.inds[nscop as usize] = nr;
                if self.nb_params(num)? > 0 {
                    let indx = tables.indx.get_or_insert_with(|| vec![0; places]);
                    indx[nr as usize] = num;
                    indx[nscop as usize] = num;
                }
            }
        }
        Ok(tables)
    }
    /// Records an ident in the second map; several times in one scope group is a fail.
    fn map_ident(
        &self,
        ident: i32,
        nr: i32,
        thenbscop: i32,
        tables: &ScanTables,
        map: &mut IdentMap,
    ) {
        if let Some(index) = map.add(ident) {
            map.indm[index] = nr;
            return;
        }
        let index = map.index_of(ident);
        let errorscope = thenbscop == 0 || same_scope(tables, nr, map.indm[index].abs());
        if errorscope {
            let line = format!("Ident defined SEVERAL TIMES : #{ident}");
            self.thecheck
                .add_fail_text(&line, "Ident defined SEVERAL TIMES : #%d");
            let _ = writeln!(
                Message::send_trace(),
                "StepReaderData : SetEntityNumbers, {line}"
            );
        }
        if map.indm[index] > 0 {
            map.indm[index] = -map.indm[index];
        }
    }
    /// The idents of the EXPORT lists, searched back from their ENDSCOPE. Quirk
    /// kept: the test on the mark reads `indx == 0 && indi != -3`.
    fn resolve_exports(&self, tables: &ScanTables) -> Result<(), Failure> {
        let indx = tables.indx.as_deref().unwrap_or_default();
        for nr in 0..=tables.nbseq {
            if indx[nr as usize] == 0 && tables.indi[nr as usize] != -3 {
                continue;
            }
            let num = indx[nr as usize];
            for na in 1..=self.nb_params(num)? {
                let param = self.param(num, na)?;
                if param.param_type() != Param::ParamIdent {
                    continue;
                }
                let id = -param.entity_number();
                if id >= 0
                    && let Some(resolved) = self.search_export(tables, indx, nr, id)?
                {
                    self.change_param(num, na, |fp| fp.set_entity_number(resolved))?;
                }
            }
        }
        Ok(())
    }
    fn search_export(
        &self,
        tables: &ScanTables,
        indx: &[i32],
        nr: i32,
        id: i32,
    ) -> Result<Option<i32>, Failure> {
        let mut n0 = nr - 1;
        if tables.indi[n0.max(0) as usize] == -3 {
            n0 -= 1;
        }
        while n0 > 0 {
            let irec = tables.indi[n0 as usize];
            if irec == id {
                return Ok(Some(tables.inds[n0 as usize]));
            }
            if irec == -1 {
                return Ok(None);
            }
            if irec == -3 {
                let nok = self.find_entity_number(indx[n0 as usize], id)?;
                if nok > 0 {
                    return Ok(Some(nok));
                }
                n0 = tables.inds[n0 as usize];
            }
            n0 -= 1;
        }
        Ok(None)
    }
    /// The parameters read from the end, which resolves the sub-lists on the fly.
    fn second_pass(&self, tables: &ScanTables, map: &IdentMap, iamap: bool) -> Result<(), Failure> {
        let nbdirec = self.nb_records();
        let mut subpile: Vec<i32> = Vec::with_capacity(30);
        for num in 1..=nbdirec {
            let nr = tables.indr[num as usize];
            if nr == 0 {
                continue;
            }
            for na in (1..=self.nb_params(num)?).rev() {
                let (kind, number) =
                    self.change_param(num, na, |fp| (fp.param_type(), fp.entity_number()))?;
                if kind == Param::ParamSub {
                    let top = subpile.pop().ok_or_else(|| bad_sub_stack(num))?;
                    self.change_param(num, na, |fp| fp.set_entity_number(top))?;
                } else if kind == Param::ParamIdent && -number >= 0 {
                    self.resolve_by_scan(tables, map, iamap, num, na, nr, -number)?;
                }
            }
            if tables.inds[nr as usize] != num {
                subpile.push(num);
            }
        }
        Ok(())
    }
    #[allow(clippy::too_many_arguments)]
    fn resolve_by_scan(
        &self,
        tables: &ScanTables,
        map: &IdentMap,
        iamap: bool,
        num: i32,
        na: i32,
        nr: i32,
        id: i32,
    ) -> Result<(), Failure> {
        let nok = self.locate_ident(tables, map, iamap, nr, id)?;
        if nok > 0 {
            let resolved = tables.inds[nok as usize];
            return self.change_param(num, na, |fp| fp.set_entity_number(resolved));
        }
        self.report_unresolved(tables, num, na, nr, id)?;
        self.change_param(num, na, |fp| fp.set_entity_number(0))
    }
}

/// Whether two definitions of an ident are in the same scope group: back from `nr` over
/// the chained sequence, a scope that opens before `toscope` separates them.
fn same_scope(tables: &ScanTables, nr: i32, toscope: i32) -> bool {
    let mut fromscope = nr;
    loop {
        fromscope -= 1;
        if fromscope <= toscope {
            return true;
        }
        let idtest = tables.indi[fromscope as usize];
        if idtest >= 0 {
            continue;
        }
        if idtest == -1 {
            return false;
        }
        if idtest == -3 {
            fromscope = tables.inds[fromscope as usize];
            if fromscope < toscope {
                return false;
            }
        }
    }
}
