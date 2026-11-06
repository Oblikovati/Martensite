//! `StepDataStepDumper`: dumps entities in the form of a STEP file.
use super::{StepDataProtocolOps, StepDataStepModel, StepDataStepWriter, StepDataWriterLib};
use martensite_tkernel::standard::{Failure, Handle, StandardTransient, exceptions};
use martensite_tkxsbase::interface::{
    InterfaceEntityIterator, InterfaceGeneralLib, InterfaceModelEntities,
};
use std::io::Write;

/// The identifiers listed by a dump, by entity number: 0 none, -1 no label, else the label.
type DumpIds = Vec<i32>;

/// ```
/// use martensite_tkdestep::step_data::{StepData, StepDataStepDumper, StepDataStepModel};
/// let mut dumper = StepDataStepDumper::new(&StepDataStepModel::new(), &StepData::protocol(), 0);
/// let mut text = Vec::new();
/// assert!(!dumper.dump_number(&mut text, 1, 0).unwrap());
/// ```
pub struct StepDataStepDumper {
    themodel: Handle<StepDataStepModel>,
    theslib: InterfaceGeneralLib,
    thewlib: StepDataWriterLib,
    thewriter: StepDataStepWriter,
}

fn io_failure(error: std::io::Error) -> Failure {
    exceptions::domain_error(&format!("StepDataStepDumper: the stream failed: {error}"))
}

impl StepDataStepDumper {
    /// `StepDataStepDumper(amodel, protocol, mode = 0)`: a positive mode writes labels.
    pub fn new(
        amodel: &Handle<StepDataStepModel>,
        protocol: &Handle<dyn StepDataProtocolOps>,
        mode: i32,
    ) -> Self {
        let mut thewriter = StepDataStepWriter::new(amodel);
        if mode > 0 {
            *thewriter.label_mode() = 2;
        }
        Self {
            themodel: amodel.clone(),
            theslib: InterfaceGeneralLib::new(&protocol.cast()),
            thewlib: StepDataWriterLib::new(protocol),
            thewriter,
        }
    }
    /// `step_writer`.
    pub fn step_writer(&mut self) -> &mut StepDataStepWriter {
        &mut self.thewriter
    }
    fn model(&self) -> Result<&StepDataStepModel, Failure> {
        self.themodel.get().ok_or_else(|| {
            exceptions::null_object("StepDataStepDumper: null model; expected a STEP model")
        })
    }
    /// `dump(S, ent, level)`: level 0 the type only, 1 the entity, more the entity with what
    /// it refers to at first level.
    pub fn dump(
        &mut self,
        s: &mut dyn Write,
        ent: &Handle<StandardTransient>,
        level: i32,
    ) -> Result<bool, Failure> {
        let model = self.model()?;
        let nb = model.nb_entities().max(0) as usize;
        let mut ids: DumpIds = vec![0; nb + 1];
        let num = model.number(ent);
        let nlab = model.ident_label(ent);
        ids[num.max(0) as usize] = if nlab > 0 { nlab } else { -1 };
        if level <= 0 {
            self.dump_type(s, ent, num, nlab).map_err(io_failure)?;
        } else if level == 1 {
            self.note_shared(ent, &mut ids)?;
            self.thewriter.send_entity_number(num, &self.thewlib)?;
        } else {
            self.dump_with_shared(ent, num, nb, &mut ids)?;
        }
        Self::print_summary(s, &ids).map_err(io_failure)?;
        if level > 0 {
            self.thewriter.print(s).map_err(io_failure)?;
        }
        Ok(true)
    }
    /// `dump(S, num, level)`: false out of the model.
    pub fn dump_number(
        &mut self,
        s: &mut dyn Write,
        num: i32,
        level: i32,
    ) -> Result<bool, Failure> {
        if num <= 0 || num > self.model()?.nb_entities() {
            return Ok(false);
        }
        let ent = self.model()?.value(num)?;
        self.dump(s, &ent, level)
    }
    fn dump_type(
        &self,
        s: &mut dyn Write,
        ent: &Handle<StandardTransient>,
        num: i32,
        nlab: i32,
    ) -> std::io::Result<()> {
        if num > 0 {
            write!(s, "#{num} = ")?;
        } else {
            write!(s, "#??? = ")?;
        }
        let name = ent.dynamic_type().map_or("", |kind| kind.name());
        match self.thewlib.select(ent) {
            Some((module, cn)) if module.is_complex(cn) => {
                let mut types = Vec::new();
                if module.complex_type(cn, &mut types) {
                    for found in &types {
                        s.write_all(found.as_bytes())?;
                        write!(s, " (...)")?;
                    }
                    writeln!(s)?;
                } else {
                    writeln!(s, "(Complex Type : ask level > 0) cdl = {name} (...);")?;
                }
            }
            Some((module, cn)) => writeln!(s, "{} (...);", module.step_type(cn))?,
            None => writeln!(s, "(Unrecognized Type for protocol) cdl = {name} (...);")?,
        }
        if nlab > 0 {
            writeln!(s, "/*   Ident in file for {num} : #{nlab}   */")?;
        }
        Ok(())
    }
    fn shared_of(&self, ent: &Handle<StandardTransient>) -> Vec<Handle<StandardTransient>> {
        let Some((module, cn)) = self.theslib.select(ent) else {
            return Vec::new();
        };
        let mut iter = InterfaceEntityIterator::new();
        module.fill_shared_case(cn, ent, &mut iter);
        module.list_implied_case(cn, ent, &mut iter);
        iter.content()
    }
    fn note_shared(
        &self,
        ent: &Handle<StandardTransient>,
        ids: &mut DumpIds,
    ) -> Result<(), Failure> {
        let model = self.model()?;
        for anent in self.shared_of(ent) {
            let nlab = model.ident_label(&anent);
            ids[model.number(&anent).max(0) as usize] = if nlab > 0 { nlab } else { -1 };
        }
        Ok(())
    }
    fn dump_with_shared(
        &mut self,
        ent: &Handle<StandardTransient>,
        num: i32,
        nb: usize,
        ids: &mut DumpIds,
    ) -> Result<(), Failure> {
        let model = self.themodel.clone();
        let live = model.get().ok_or_else(|| {
            exceptions::null_object("StepDataStepDumper: null model; expected a STEP model")
        })?;
        let mut tab = vec![false; nb + 1];
        tab[num.max(0) as usize] = true;
        for anent in self.shared_of(ent) {
            tab[live.number(&anent).max(0) as usize] = true;
        }
        for (rank, _) in tab
            .iter()
            .enumerate()
            .skip(1)
            .filter(|(_, wanted)| **wanted)
        {
            let anent = live.value(rank as i32)?;
            self.thewriter
                .send_entity_number(rank as i32, &self.thewlib)?;
            self.note_shared(&anent, ids)?;
        }
        Ok(())
    }
    /// The ending. The counters of "no ident" and "ident = num" stay 0, so
    /// only the proper ident line can appear.
    fn print_summary(s: &mut dyn Write, ids: &DumpIds) -> std::io::Result<()> {
        let proper = ids
            .iter()
            .enumerate()
            .skip(1)
            .filter(|(rank, label)| **label > 0 && **label != *rank as i32)
            .count();
        if proper > 0 {
            writeln!(
                s,
                "In dump, iii:#jjj means : entity rank iii has step ident #jjj"
            )?;
        }
        Ok(())
    }
}
