//! The dump of the global section of a model.
use super::{IgesDataBasicEditor, IgesDataGlobalSection};
use martensite_tkernel::{standard::Handle, storage::format_general, tcollection::HAsciiString};
use std::io::{self, Write};

fn dump_text(
    s: &mut dyn Write,
    label: &str,
    text: &Handle<HAsciiString>,
    end: &str,
) -> io::Result<()> {
    if let Some(live) = text.get() {
        write!(s, "{label}")?;
        s.write_all(live.string().as_bytes())?;
        write!(s, "{end}")?;
    }
    Ok(())
}

fn dump_date(s: &mut dyn Write, label: &str, date: &Handle<HAsciiString>) -> io::Result<()> {
    let Some(live) = date.get() else {
        return Ok(());
    };
    write!(s, "{label}")?;
    s.write_all(live.string().as_bytes())?;
    write!(s, "  i.e. ")?;
    if let Some(readable) = IgesDataGlobalSection::new_date_string_from(date, 1).get() {
        s.write_all(readable.string().as_bytes())?;
    }
    writeln!(s)
}

fn real(value: f64) -> String {
    format_general(value, 6)
}

pub(super) fn dump_global(s: &mut dyn Write, h: &IgesDataGlobalSection) -> io::Result<()> {
    writeln!(s, "\n****    Global Section    ****")?;
    let sep = h.separator() as char;
    let kind = if sep == ',' {
        "    Default"
    } else {
        "Non Default"
    };
    write!(s, "[ 1]  {kind} Separator : {sep}")?;
    let emk = h.end_mark() as char;
    let kind = if emk == ';' {
        "    Default"
    } else {
        "Non Default"
    };
    writeln!(s, "        [ 2]  {kind} End Mark  : {emk}")?;
    dump_text(s, "[ 3]  Sender                : ", &h.send_name(), "\n")?;
    dump_text(s, "[ 4]  (recorded) File Name  : ", &h.file_name(), "\n")?;
    dump_text(s, "[ 5]  System Identification : ", &h.system_id(), "\n")?;
    dump_text(
        s,
        "[ 6]  Interface Version     : ",
        &h.interface_version(),
        "\n",
    )?;
    writeln!(s)?;
    writeln!(
        s,
        "[ 7]  Integer Bits          : {}          Features for Reals : ",
        h.integer_bits()
    )?;
    writeln!(
        s,
        "[ 8]  Single Max.Power(10)  : {}         [ 9]  Digits   : {}",
        h.max_power10_single(),
        h.max_digits_single()
    )?;
    writeln!(
        s,
        "[10]  Double Max.Power(10)  : {}         [11]  Digits   : {}\n",
        h.max_power10_double(),
        h.max_digits_double()
    )?;
    dump_text(s, "[12]  Receiver              : ", &h.receive_name(), "\n")?;
    writeln!(s, "[13]  Scale                 : {}", real(h.scale()))?;
    writeln!(
        s,
        "[14]  Unit  Flag            : {}    -> Value (in Martensite units) = {}",
        h.unit_flag(),
        real(h.unit_value())
    )?;
    dump_text(s, "[15]  Unit  Name            : ", &h.unit_name(), "\n\n")?;
    writeln!(s, "[16]  Line Weight  Gradient : {}", h.line_weight_grad())?;
    writeln!(
        s,
        "[17]  Line Weight  Max Value: {}",
        real(h.max_line_weight())
    )?;
    dump_date(s, "[18]  (Creation) Date       : ", &h.date())?;
    writeln!(s, "[19]  Resolution            : {}", real(h.resolution()))?;
    if h.has_max_coord() {
        writeln!(s, "[20]  Maximum Coord         : {}\n", real(h.max_coord()))?;
    } else {
        writeln!(s, "[20]  Maximum Coord           not defined\n")?;
    }
    dump_text(s, "[21]  Author                : ", &h.author_name(), "\n")?;
    dump_text(s, "[22]  Company               : ", &h.company_name(), "\n")?;
    let num = h.iges_version();
    write!(
        s,
        "[23]  IGES Version Number   : {num}   -> Name : {}",
        IgesDataBasicEditor::iges_version_name(num)
    )?;
    let num = h.drafting_standard();
    write!(s, "\n[24]  Drafting Standard     : {num}")?;
    if num > 0 {
        write!(
            s,
            "   -> Name : {}",
            IgesDataBasicEditor::drafting_name(num)
        )?;
    }
    writeln!(s)?;
    if h.has_last_change_date() {
        dump_date(s, "[25]  Last Change Date      : ", &h.last_change_date())?;
    } else {
        writeln!(
            s,
            "[25]  Last Change Date        not defined (version IGES < 5.1)"
        )?;
    }
    dump_text(
        s,
        "[26]  Application Protocol  : ",
        &h.application_protocol(),
        "\n",
    )?;
    writeln!(s, " ****     End of Dump      ****")
}
