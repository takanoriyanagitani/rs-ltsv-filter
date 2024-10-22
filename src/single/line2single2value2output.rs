use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Read;
use std::io::Write;

pub fn line2single2value2output<O>(
    line: &[u8],
    field_separator: u8,
    value_separator: u8,
    single_label: &[u8],
    output: &mut O,
) -> Result<(), io::Error>
where
    O: FnMut(&[u8]) -> Result<(), io::Error>,
{
    let fields = line.split(|u| field_separator.eq(u));
    for pair in fields {
        let mut splited = pair.splitn(2, |u| value_separator.eq(u));
        let label: &[u8] = splited.next().unwrap_or_default();
        if label != single_label {
            continue;
        }

        let value: &[u8] = splited.next().unwrap_or_default();
        output(value)?;
    }
    Ok(())
}

pub fn reader2single2value2output<R, O>(
    rdr: R,
    line_separator: u8,
    field_separator: u8,
    value_separator: u8,
    single_label: &[u8],
    output: &mut O,
) -> Result<(), io::Error>
where
    R: Read,
    O: FnMut(&[u8]) -> Result<(), io::Error>,
{
    let br = BufReader::new(rdr);
    let lines = br.split(line_separator);
    for rline in lines {
        let line: &[u8] = &rline?;
        line2single2value2output(line, field_separator, value_separator, single_label, output)?;
    }
    Ok(())
}

pub fn reader2writer<R, W>(
    rdr: R,
    mut wtr: W,
    line_separator: u8,
    field_separator: u8,
    value_separator: u8,
    single_label: &[u8],
) -> Result<(), io::Error>
where
    R: Read,
    W: Write,
{
    {
        let mut bw = BufWriter::new(wtr.by_ref());
        reader2single2value2output(
            rdr,
            line_separator,
            field_separator,
            value_separator,
            single_label,
            &mut |val: &[u8]| {
                bw.write_all(val)?;
                bw.write_all(b"\n")?;
                Ok(())
            },
        )?;
        bw.flush()?;
    }
    wtr.flush()
}
