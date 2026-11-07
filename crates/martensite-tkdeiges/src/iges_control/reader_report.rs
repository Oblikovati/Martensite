//! The report of a transfer: counts, results and messages of a reader.
use super::IgesControlReader;
use crate::iges_data::{IgesDataIgesEntity, iges_live};
use martensite_tkernel::message::{Gravity, Msg};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkxsbase::if_select::{IFSelectPrintCount, IFSelectPrintFail};
use martensite_tkxsbase::interface::{InterfaceInterfaceModel, InterfaceModelEntities};
use martensite_tkxsbase::transfer::TransferTransientProcess;
use std::collections::BTreeMap;
use std::fmt::Write;

const WHERE: &str = "IgesControlReader";

/// What the entries of a report count.
#[derive(Default)]
struct TransferTally {
    nb_warnings: i32,
    nb_fails: i32,
    /// A message line, its count and the numbers of the entities it is on.
    items: BTreeMap<String, (i32, Vec<i32>)>,
    /// A result type or a mapping line, and its count.
    results: BTreeMap<String, i32>,
    nb_results: i32,
}

fn send_key(process: &TransferTransientProcess, key: &str) {
    process
        .messenger()
        .send_extended(Msg::from_key(&key.into()).value(), Gravity::Info);
}

fn send_key_count(
    process: &TransferTransientProcess,
    key: &str,
    value: i32,
) -> Result<(), Failure> {
    let mut message = Msg::from_key(&key.into());
    message.arg_integer(value)?;
    process
        .messenger()
        .send_extended(message.value(), Gravity::Info);
    Ok(())
}

impl TransferTally {
    fn add_item(&mut self, line: String, number: i32) {
        let entry = self.items.entry(line).or_default();
        entry.0 += 1;
        entry.1.push(number);
    }
    fn add_checks(
        &mut self,
        process: &TransferTransientProcess,
        model: &Handle<dyn InterfaceInterfaceModel>,
        with_warnings: bool,
    ) -> Result<(), Failure> {
        let live_model = iges_live(model, WHERE)?;
        let checks = process.check_list(false);
        checks.start();
        while checks.more() {
            let check = checks.value()?;
            let entity: Handle<IgesDataIgesEntity> = live_model.value(checks.number()?)?.cast();
            let body = iges_live(&entity, WHERE)?;
            let (kind, form) = (body.type_number(), body.form_number());
            let number = live_model.number(&entity.cast()) * 2 - 1;
            let live_check = iges_live(&check, WHERE)?;
            let (nb_warnings, nb_fails) = (live_check.nb_warnings(), live_check.nb_fails());
            if with_warnings {
                for rank in 1..=nb_warnings {
                    let text = live_check.c_warning(rank, true)?;
                    let text = String::from_utf8_lossy(text.as_bytes()).into_owned();
                    self.add_item(format!("\t W\t{kind}\t{form}\t{text}"), number);
                }
            }
            for rank in 1..=nb_fails {
                let text = live_check.c_fail(rank, true)?;
                let text = String::from_utf8_lossy(text.as_bytes()).into_owned();
                self.add_item(format!("\t F\t{kind}\t{form}\t{text}"), number);
            }
            self.nb_warnings += nb_warnings;
            self.nb_fails += nb_fails;
            checks.next();
        }
        Ok(())
    }
    fn add_results(
        &mut self,
        process: &TransferTransientProcess,
        mode: IFSelectPrintCount,
    ) -> Result<(), Failure> {
        let mut results = process.root_result(true);
        results.start();
        while results.more() {
            self.nb_results += 1;
            let binder = results.value()?;
            let type_name = iges_live(&binder, WHERE)?.result_type_name();
            if mode == IFSelectPrintCount::ResultCount {
                *self.results.entry(format!("\t{type_name}")).or_default() += 1;
            } else if mode == IFSelectPrintCount::Mapping {
                let entity: Handle<IgesDataIgesEntity> = results.starting()?.cast();
                let body = iges_live(&entity, WHERE)?;
                let line = format!(
                    "{}\t{}\t%d\t{type_name}",
                    body.type_number(),
                    body.form_number()
                );
                *self.results.entry(line).or_default() += 1;
            }
            results.next();
        }
        Ok(())
    }
}

fn send_items(
    process: &TransferTransientProcess,
    tally: &TransferTally,
    with_list: bool,
) -> Result<(), Failure> {
    for (line, (count, numbers)) in &tally.items {
        let mut text = format!("{count}{line}\n");
        if with_list {
            send_key(process, "IGES_3035");
            text.push_str("\t\t\t");
            for (rank, number) in numbers.iter().enumerate() {
                let _ = write!(text, "\t {number}");
                if (rank + 1) % 6 == 0 {
                    text.push_str("\n\t\t\t");
                }
            }
            text.push('\n');
        }
        let _ = process.messenger().send_info().write_str(&text);
    }
    Ok(())
}

fn send_mapping(
    process: &TransferTransientProcess,
    tally: &mut TransferTally,
    nb_roots: i32,
) -> Result<(), Failure> {
    if nb_roots != tally.nb_results {
        for rank in 1..=nb_roots {
            let root: Handle<IgesDataIgesEntity> = process.root(rank)?.cast();
            if !process.is_bound(&root.cast()) {
                let body = iges_live(&root, WHERE)?;
                let line = format!(
                    "{}\t{} \t%d\tFailed",
                    body.type_number(),
                    body.form_number()
                );
                *tally.results.entry(line).or_default() += 1;
            }
        }
    }
    for (line, count) in &tally.results {
        let text = line.replacen("%d", &count.to_string(), 1);
        let _ = writeln!(process.messenger().send_info(), "{text}");
    }
    Ok(())
}

impl IgesControlReader {
    /// Reports the last transfer: general counts, messages by item, results or the mapping;
    /// `reader.print_transfer_info(IFSelectPrintFail::FailOnly, IFSelectPrintCount::GeneralInfo)?`.
    ///
    /// # Errors
    /// A failure of the session or of a check of the process.
    pub fn print_transfer_info(
        &self,
        fails_only: IFSelectPrintFail,
        mode: IFSelectPrintCount,
    ) -> Result<(), Failure> {
        let ws = self.reader.ws();
        let reader = iges_live(&ws, WHERE)?.transfer_reader();
        let process_handle = iges_live(&reader, WHERE)?.transient_process();
        let process = iges_live(&process_handle, WHERE)?;
        let model = process.model();
        let Some(live_model) = model.get() else {
            return Ok(());
        };
        let nb_roots = process.nb_roots();
        let mut tally = TransferTally::default();
        tally.add_results(process, mode)?;
        tally.add_checks(
            process,
            &model,
            fails_only == IFSelectPrintFail::FailAndWarn,
        )?;
        send_key(process, "IGES_3000");
        match mode {
            IFSelectPrintCount::GeneralInfo => {
                send_key(process, "IGES_3005");
                send_key_count(process, "IGES_3010", live_model.nb_entities())?;
                send_key_count(process, "IGES_3011", nb_roots)?;
                send_key_count(process, "IGES_3015", tally.nb_results)?;
                send_key_count(process, "IGES_3020", tally.nb_warnings)?;
                send_key_count(process, "IGES_3025", tally.nb_fails)?;
            }
            IFSelectPrintCount::CountByItem | IFSelectPrintCount::ListByItem => {
                send_key(process, "IGES_3030");
                send_items(process, &tally, mode == IFSelectPrintCount::ListByItem)?;
            }
            IFSelectPrintCount::ResultCount => {
                send_key(process, "IGES_3040");
                send_key_count(process, "IGES_3011", nb_roots)?;
                send_key_count(process, "IGES_3015", tally.nb_results)?;
                send_key(process, "IGES_3045");
                for (line, count) in &tally.results {
                    let _ = writeln!(process.messenger().send_info(), "{line}{count}");
                }
            }
            IFSelectPrintCount::Mapping => {
                send_key(process, "IGES_3050");
                send_key_count(process, "IGES_3011", nb_roots)?;
                send_key_count(process, "IGES_3015", tally.nb_results)?;
                send_key(process, "IGES_3055");
                send_mapping(process, &mut tally, nb_roots)?;
            }
            _ => {}
        }
        Ok(())
    }
}
