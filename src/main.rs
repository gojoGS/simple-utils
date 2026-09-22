use anyhow::{Context, Result};
use clap::Parser;
use std::io::{self, BufRead, Write};

/// Align whitespace-separated tokens from stdin into vertical columns.
#[derive(Debug, Parser)]
#[command(name = "tabulate", version, about, long_about = None)]
struct Cli {
    /// Split on this exact character instead of runs of whitespace.
    #[arg(short, long, value_name = "CHAR")]
    delimiter: Option<char>,

    /// String placed between output columns.
    #[arg(long, value_name = "STR", default_value = "  ")]
    separator: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let rows = parse_rows(io::stdin().lock(), cli.delimiter)?;
    let rendered = render(&rows, &cli.separator);

    io::stdout()
        .lock()
        .write_all(rendered.as_bytes())
        .context("failed to write output")?;
    Ok(())
}

/// Read every line and split it into fields.
///
/// `delimiter == None` splits on runs of whitespace; `Some(c)` splits on an
/// exact character and trims surrounding whitespace from each field.
fn parse_rows<R: BufRead>(reader: R, delimiter: Option<char>) -> Result<Vec<Vec<String>>> {
    let mut rows = Vec::new();
    for line in reader.lines() {
        let line = line.context("failed to read a line from stdin")?;
        if line.is_empty() {
            rows.push(Vec::new());
            continue;
        }
        let fields = match delimiter {
            Some(c) => line.split(c).map(|f| f.trim().to_string()).collect(),
            None => line.split_whitespace().map(str::to_string).collect(),
        };
        rows.push(fields);
    }
    Ok(rows)
}

/// Pad each column to the width of its widest cell.
///
/// The final field of each row is never padded, so no trailing whitespace is
/// emitted.
fn render(rows: &[Vec<String>], separator: &str) -> String {
    let columns = rows.iter().map(Vec::len).max().unwrap_or(0);

    let mut widths = vec![0usize; columns];
    for row in rows {
        for (i, field) in row.iter().enumerate() {
            widths[i] = widths[i].max(field.chars().count());
        }
    }

    let mut out = String::new();
    for row in rows {
        for (i, field) in row.iter().enumerate() {
            out.push_str(field);
            if i + 1 < row.len() {
                let padding = widths[i] - field.chars().count();
                out.extend(std::iter::repeat_n(' ', padding));
                out.push_str(separator);
            }
        }
        out.push('\n');
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rows(input: &str, delimiter: Option<char>) -> Vec<Vec<String>> {
        parse_rows(input.as_bytes(), delimiter).unwrap()
    }

    #[test]
    fn aligns_columns() {
        let input = "a bb ccc\ndddd e f\n";
        let got = render(&rows(input, None), "  ");
        assert_eq!(got, "a     bb  ccc\ndddd  e   f\n");
    }

    #[test]
    fn last_column_is_not_padded() {
        let input = "short verylongvalue\nmuchlonger x\n";
        let got = render(&rows(input, None), "  ");
        assert_eq!(got, "short       verylongvalue\nmuchlonger  x\n");
    }

    #[test]
    fn honours_custom_separator() {
        let input = "a bb\nccc d\n";
        let got = render(&rows(input, None), " | ");
        assert_eq!(got, "a   | bb\nccc | d\n");
    }

    #[test]
    fn supports_exact_delimiter() {
        let input = "a,b\nccc,d\n";
        let got = render(&rows(input, Some(',')), "  ");
        assert_eq!(got, "a    b\nccc  d\n");
    }

    #[test]
    fn preserves_blank_lines() {
        let input = "a b\n\nccc d\n";
        let got = render(&rows(input, None), "  ");
        assert_eq!(got, "a    b\n\nccc  d\n");
    }

    #[test]
    fn handles_ragged_rows() {
        let input = "a b c\nonlyone\nx y\n";
        let got = render(&rows(input, None), "  ");
        assert_eq!(got, "a        b  c\nonlyone\nx        y\n");
    }

    #[test]
    fn counts_unicode_by_char() {
        let input = "é a\nbbb c\n";
        let got = render(&rows(input, None), " ");
        assert_eq!(got, "é   a\nbbb c\n");
    }

    #[test]
    fn empty_input_is_empty_output() {
        let got = render(&rows("", None), "  ");
        assert_eq!(got, "");
    }
}
