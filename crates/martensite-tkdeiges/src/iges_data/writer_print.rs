//! The printing of the sections an IGES writer holds.
use super::writer::{MAXCARS_G, MAXCARS_P};
use super::{IgesDataIgesWriter, IgesDataTextLines};
use martensite_tkernel::{
    standard::{Failure, Handle},
    tcollection::HAsciiString,
};
use martensite_tkxsbase::interface::interface_error;
use std::io::{self, Write};

/// Writes a line as it is, or enciphered: each character is exclusive-ored with 150 to 153 in turn.
fn write_piece(s: &mut dyn Write, fnes: bool, ligne: &[u8]) -> io::Result<()> {
    if !fnes {
        return s.write_all(ligne);
    }
    let coded: Vec<u8> = ligne
        .iter()
        .take(80)
        .enumerate()
        .map(|(i, byte)| *byte ^ (150 + (i & 3) as u8))
        .collect();
    s.write_all(&coded)
}

fn line_at(lines: &Handle<IgesDataTextLines>, rank: i32) -> Option<Handle<HAsciiString>> {
    let live = lines.get()?;
    live.sequence(|items| items.value(usize::try_from(rank).ok()?).ok().cloned())
}

fn nb_lines(lines: &Handle<IgesDataTextLines>) -> i32 {
    lines
        .get()
        .map_or(0, |live| live.sequence(|items| items.length()))
}

/// Writes a section line: its text, blanks up to `width`, then the end of line.
fn write_section_line(
    s: &mut dyn Write,
    fnes: bool,
    line: &Handle<HAsciiString>,
    width: i32,
    finlin: &str,
) -> io::Result<()> {
    let content = line
        .get()
        .map(|text| text.string().as_bytes().to_vec())
        .unwrap_or_default();
    write_piece(s, fnes, &content)?;
    let missing = usize::try_from(width - content.len() as i32).unwrap_or(0);
    // The blanks are enciphered on their own, from the start of the pattern.
    write_piece(s, fnes, &vec![b' '; missing])?;
    write_piece(s, fnes, finlin.as_bytes())?;
    s.write_all(b"\n")
}

fn text_field(text: &[u8; 9]) -> String {
    let end = text.iter().position(|byte| *byte == 0).unwrap_or(8);
    format!("{:>8}", String::from_utf8_lossy(&text[..end]))
}

/// `%2.2d`: two digits at least, the sign apart.
fn two_digits(value: i32) -> String {
    if value < 0 {
        format!("-{:02}", -i64::from(value))
    } else {
        format!("{value:02}")
    }
}

impl IgesDataIgesWriter {
    /// Prints the file; false when the stream fails.
    ///
    /// # Errors
    /// An interface failure when the sections are not all there.
    pub fn print(&self, s: &mut dyn Write) -> Result<bool, Failure> {
        if self.thesect != 4 {
            return Err(interface_error(&format!(
                "IGES writer not ready to print: at section {}; expected 4",
                self.thesect
            )));
        }
        Ok(self.print_sections(s).is_ok())
    }
    fn print_sections(&self, s: &mut dyn Write) -> io::Result<()> {
        let fnes = self.themodew >= 10;
        let nbs = self.print_start(s, fnes)?;
        let nbg = self.print_lines(s, fnes, &self.thehead, 'G')?;
        let nbd = self.thedirs.len() as i32 - 1;
        self.print_directory(s, fnes, nbd)?;
        self.print_parameters(s, fnes, nbd)?;
        let last = self.thepnum.last().copied().unwrap_or(1) - 1;
        let terminal = format!(
            "S{nbs:7}G{nbg:7}D{:7}P{last:7}                                        T0000001",
            nbd * 2
        );
        write_piece(s, fnes, terminal.as_bytes())?;
        s.write_all(b"\n")?;
        s.flush()
    }
    fn print_start(&self, s: &mut dyn Write, fnes: bool) -> io::Result<i32> {
        if self.thestar.is_some() {
            return self.print_lines(s, fnes, &self.thestar, 'S');
        }
        if fnes {
            s.write_all(
                b"                              ***  EUCLID/STRIM  DESKTOP CLIPBOARD  ***\n",
            )?;
        }
        write_piece(s, fnes, format!("{:72}S0000001", "").as_bytes())?;
        s.write_all(b"\n")?;
        Ok(1)
    }
    fn print_lines(
        &self,
        s: &mut dyn Write,
        fnes: bool,
        lines: &Handle<IgesDataTextLines>,
        section: char,
    ) -> io::Result<i32> {
        let nb = nb_lines(lines);
        for i in 1..=nb {
            let finlin = format!("{section}{i:07}");
            let line = line_at(lines, i).unwrap_or_default();
            write_section_line(s, fnes, &line, MAXCARS_G, &finlin)?;
        }
        Ok(nb)
    }
    fn print_directory(&self, s: &mut dyn Write, fnes: bool, nbd: i32) -> io::Result<()> {
        for i in 1..=nbd {
            let fields = self.thedirs[i as usize].values();
            let v = fields.values;
            let start = self.thepnum.get(i as usize - 1).copied().unwrap_or(0);
            let next = self.thepnum.get(i as usize).copied().unwrap_or(0);
            let first = format!(
                "{:8}{start:8}{:8}{:8}{:8}{:8}{:8}{:8}{}{}{}{}D{:07}",
                v[0],
                v[2],
                v[3],
                v[4],
                v[5],
                v[6],
                v[7],
                two_digits(v[8]),
                two_digits(v[9]),
                two_digits(v[10]),
                two_digits(v[11]),
                2 * i - 1
            );
            write_piece(s, fnes, first.as_bytes())?;
            s.write_all(b"\n")?;
            let second = format!(
                "{:8}{:8}{:8}{:8}{:8}{}{}{}{}D{:07}",
                v[0],
                v[13],
                v[14],
                next - start,
                v[16],
                text_field(&fields.res1),
                text_field(&fields.res2),
                text_field(&fields.label),
                text_field(&fields.subscript),
                2 * i
            );
            write_piece(s, fnes, second.as_bytes())?;
            s.write_all(b"\n")?;
        }
        Ok(())
    }
    fn print_parameters(&self, s: &mut dyn Write, fnes: bool, nbd: i32) -> io::Result<()> {
        for i in 1..=nbd {
            let start = self.thepnum.get(i as usize - 1).copied().unwrap_or(0);
            let next = self.thepnum.get(i as usize).copied().unwrap_or(0);
            for j in start..next {
                let finlin = format!(" {:07}P{j:07}", 2 * i - 1);
                let line = line_at(&self.thepars, j).unwrap_or_default();
                write_section_line(s, fnes, &line, MAXCARS_P, &finlin)?;
            }
        }
        Ok(())
    }
}
