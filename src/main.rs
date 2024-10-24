use std::env;
use std::io;

use std::process::ExitCode;

use rs_ltsv_filter::LINE_SEPARATOR_DEFAULT;

use rs_ltsv_filter::FIELD_SEPARATOR_DEFAULT;
use rs_ltsv_filter::VALUE_SEPARATOR_DEFAULT;

use rs_ltsv_filter::OUTPUT_EMPTY_DEFAULT;

fn stdin2stdout(
    line_sep: u8,
    field_sep: u8,
    value_sep: u8,
    label: &[u8],
    output_empty: bool,
) -> Result<(), io::Error> {
    let i = io::stdin();
    let il = i.lock();
    let o = io::stdout();
    let ol = o.lock();
    rs_ltsv_filter::single::line2single2value2output::reader2writer(
        il,
        ol,
        line_sep,
        field_sep,
        value_sep,
        label,
        output_empty,
    )
}

fn sub() -> Result<(), io::Error> {
    let line_sep: u8 = env::var("ENV_LINE_SEPARATOR_U8")
        .ok()
        .and_then(|s| str::parse(s.as_str()).ok())
        .unwrap_or(LINE_SEPARATOR_DEFAULT);

    let field_sep: u8 = env::var("ENV_FIELD_SEPARATOR_U8")
        .ok()
        .and_then(|s| str::parse(s.as_str()).ok())
        .unwrap_or(FIELD_SEPARATOR_DEFAULT);
    let value_sep: u8 = env::var("ENV_VALUE_SEPARATOR_U8")
        .ok()
        .and_then(|s| str::parse(s.as_str()).ok())
        .unwrap_or(VALUE_SEPARATOR_DEFAULT);

    let output_empty: bool = env::var("ENV_OUTPUT_EMPTY")
        .ok()
        .and_then(|s| str::parse(s.as_str()).ok())
        .unwrap_or(OUTPUT_EMPTY_DEFAULT);

    let label: String = env::var("ENV_LABEL_SINGLE").ok().unwrap_or_default();
    let lb: &[u8] = label.as_bytes();
    stdin2stdout(line_sep, field_sep, value_sep, lb, output_empty)
}

fn main() -> ExitCode {
    sub().map(|_| ExitCode::SUCCESS).unwrap_or_else(|e| {
        eprintln!("{e}");
        ExitCode::FAILURE
    })
}
