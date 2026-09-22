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

    let input = parse(io::stdin().lock(), cli.delimiter)?;
    let rendered = render(&input, &cli.separator);

    io::stdout()
        .lock()
        .write_all(rendered.as_bytes())
        .context("failed to write output")?;
    Ok(())
}

/// Parsed input: the tabulated rows plus the indentation to reapply.
#[derive(Debug, Default)]
struct Input {
    /// One entry per input line; empty for blank lines.
    rows: Vec<Vec<String>>,
    /// Leading whitespace of the first non-blank line.
    indent: String,
}

/// Read every line, split it into fields, and capture the base indentation.
///
/// `delimiter == None` splits on runs of whitespace; `Some(c)` splits on an
/// exact character and trims surrounding whitespace from each field.
fn parse<R: BufRead>(reader: R, delimiter: Option<char>) -> Result<Input> {
    let mut rows = Vec::new();
    let mut indent: Option<String> = None;

    for line in reader.lines() {
        let line = line.context("failed to read a line from stdin")?;

        if line.trim().is_empty() {
            rows.push(Vec::new());
            continue;
        }

        let lead: String = line.chars().take_while(|c| c.is_whitespace()).collect();
        let fields = match delimiter {
            Some(c) => line.split(c).map(|f| f.trim().to_string()).collect(),
            None => line.split_whitespace().map(str::to_string).collect(),
        };

        if indent.is_none() {
            indent = Some(lead);
        }
        rows.push(fields);
    }

    Ok(Input {
        rows,
        indent: indent.unwrap_or_default(),
    })
}

/// Pad each column to the width of its widest cell.
///
/// Every non-blank line is prefixed with the captured indentation, and the
/// final field of each row is never padded, so no trailing whitespace is
/// emitted.
fn render(input: &Input, separator: &str) -> String {
    let Input { rows, indent } = input;

    let columns = rows.iter().map(Vec::len).max().unwrap_or(0);

    let mut widths = vec![0usize; columns];
    for row in rows {
        for (i, field) in row.iter().enumerate() {
            widths[i] = widths[i].max(field.chars().count());
        }
    }

    let mut out = String::new();
    for row in rows {
        if row.is_empty() {
            out.push('\n');
            continue;
        }

        out.push_str(indent);
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

    fn run(input: &str, delimiter: Option<char>, separator: &str) -> String {
        let parsed = parse(input.as_bytes(), delimiter).unwrap();
        render(&parsed, separator)
    }

    #[test]
    fn aligns_columns() {
        let got = run("a bb ccc\ndddd e f\n", None, "  ");
        assert_eq!(got, "a     bb  ccc\ndddd  e   f\n");
    }

    #[test]
    fn last_column_is_not_padded() {
        let got = run("short verylongvalue\nmuchlonger x\n", None, "  ");
        assert_eq!(got, "short       verylongvalue\nmuchlonger  x\n");
    }

    #[test]
    fn honours_custom_separator() {
        let got = run("a bb\nccc d\n", None, " | ");
        assert_eq!(got, "a   | bb\nccc | d\n");
    }

    #[test]
    fn supports_exact_delimiter() {
        let got = run("a,b\nccc,d\n", Some(','), "  ");
        assert_eq!(got, "a    b\nccc  d\n");
    }

    #[test]
    fn preserves_blank_lines() {
        let got = run("a b\n\nccc d\n", None, "  ");
        assert_eq!(got, "a    b\n\nccc  d\n");
    }

    #[test]
    fn handles_ragged_rows() {
        let got = run("a b c\nonlyone\nx y\n", None, "  ");
        assert_eq!(got, "a        b  c\nonlyone\nx        y\n");
    }

    #[test]
    fn counts_unicode_by_char() {
        let got = run("é a\nbbb c\n", None, " ");
        assert_eq!(got, "é   a\nbbb c\n");
    }

    #[test]
    fn empty_input_is_empty_output() {
        let got = run("", None, "  ");
        assert_eq!(got, "");
    }

    #[test]
    fn inherits_first_line_indent() {
        let got = run("    first\n    second\n  third\n", None, "  ");
        assert_eq!(got, "    first\n    second\n    third\n");
    }

    #[test]
    fn indent_applies_before_column_padding() {
        let got = run("  a bb\n  ccc d\n", None, "  ");
        assert_eq!(got, "  a    bb\n  ccc  d\n");
    }

    #[test]
    fn blank_lines_get_no_indent() {
        let got = run("  a b\n\n  c d\n", None, "  ");
        assert_eq!(got, "  a  b\n\n  c  d\n");
    }

    #[test]
    fn first_blank_line_does_not_set_indent() {
        let got = run("\n    a b\n    c d\n", None, "  ");
        assert_eq!(got, "\n    a  b\n    c  d\n");
    }

    #[test]
    fn tabs_are_preserved_as_indent() {
        let got = run("\ta b\n   c d\n", None, " ");
        assert_eq!(got, "\ta b\n\tc d\n");
    }
}
