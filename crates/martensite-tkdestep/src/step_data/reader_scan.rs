//! The second pass's search of an ident in the sequence of entities and scopes.
use super::StepDataStepReaderData;
use super::reader_numbers::{IdentMap, ScanTables};
use martensite_tkernel::{message::Message, standard::Failure};
use martensite_tkxsbase::interface::InterfaceFileReaderData;
use std::fmt::Write as _;

impl StepDataStepReaderData {
    /// The rank in the sequence of `#id`: by the map, else backward then forward from `nr`.
    pub(super) fn locate_ident(
        &self,
        tables: &ScanTables,
        map: &IdentMap,
        iamap: bool,
        nr: i32,
        id: i32,
    ) -> Result<i32, Failure> {
        if iamap {
            let index = map.index_of(id);
            if index > 0 && map.indm[index] >= 0 {
                return Ok(map.indm[index]);
            }
        }
        let mut sens = 0;
        for _ in 0..2 {
            let nok = if sens == 0 && nr > 1 {
                self.scan_backward(tables, nr, id)?
            } else if nr < tables.nbseq {
                self.scan_forward(tables, nr, id)?
            } else {
                0
            };
            if nok > 0 {
                return Ok(nok);
            }
            sens = 1 - sens;
        }
        Ok(0)
    }
    fn scan_backward(&self, tables: &ScanTables, nr: i32, id: i32) -> Result<i32, Failure> {
        let mut n0 = nr - 1;
        if tables.indi[n0 as usize] == -3 {
            n0 -= 1;
        }
        while n0 > 0 {
            let irec = tables.indi[n0 as usize];
            if irec == id {
                return Ok(n0);
            }
            if irec == -3 {
                if let Some(indx) = tables.indx.as_deref() {
                    let nok = self.find_entity_number(indx[n0 as usize], id)?;
                    if nok > 0 {
                        return Ok(nok);
                    }
                }
                n0 = tables.inds[n0 as usize];
            }
            n0 -= 1;
        }
        Ok(0)
    }
    fn scan_forward(&self, tables: &ScanTables, nr: i32, id: i32) -> Result<i32, Failure> {
        let mut n0 = nr + 1;
        while n0 <= tables.nbseq {
            let irec = tables.indi[n0 as usize];
            if irec == id {
                return Ok(n0);
            }
            if irec == -1 {
                if let Some(indx) = tables.indx.as_deref() {
                    let nok = self.find_entity_number(indx[n0 as usize], id)?;
                    if nok > 0 {
                        return Ok(nok);
                    }
                }
                n0 = tables.inds[n0 as usize];
            }
            n0 += 1;
        }
        Ok(0)
    }
    pub(super) fn report_unresolved(
        &self,
        tables: &ScanTables,
        num: i32,
        na: i32,
        nr: i32,
        id: i32,
    ) -> Result<(), Failure> {
        let nument = (1..=nr)
            .filter(|rank| tables.indi[*rank as usize] > 0)
            .count();
        let mut ident = self.record_ident(num);
        if ident < 0 {
            for rank in num + 1..=self.nb_records() {
                ident = self.record_ident(rank);
                if ident > 0 {
                    break;
                }
            }
        }
        self.thecheck.add_fail_text(
            &format!(
                "Unresolved Reference, Ent.n0 {nument} (Id.#{ident}) Param.n0 {na} (Id.#{id})"
            ),
            "Unresolved Reference",
        );
        let _ = writeln!(
            Message::send_trace(),
            "*** ERR StepReaderData *** Entite {nument}, a {}% de DATA : #{ident}\n    Type:{}  Param.n0 {na}: #{id} Not found",
            i64::from(nr) * 100 / i64::from(tables.nbseq.max(1)),
            self.c_type(num)
        );
        Ok(())
    }
}
