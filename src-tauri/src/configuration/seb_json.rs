use std::cmp::Ordering;
use std::fmt::Write;

use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use plist::{Dictionary, Value};

/// Serializes settings to the SEB-JSON string the Config Key is hashed from.
pub fn to_seb_json(settings: &Dictionary) -> String {
    let mut out = String::new();
    write_dict(&mut out, settings);
    out
}

fn write_dict(out: &mut String, dict: &Dictionary) {
    let mut entries: Vec<_> = dict.iter().collect();
    entries.sort_by(|a, b| compare_keys(a.0, b.0));
    let last = entries.len().saturating_sub(1);

    out.push('{');
    for (i, (key, value)) in entries.into_iter().enumerate() {
        let empty_dict = matches!(value, Value::Dictionary(d) if d.is_empty());
        if empty_dict || key.eq_ignore_ascii_case("originatorVersion") {
            continue;
        }
        write!(out, "\"{key}\":").unwrap();
        write_value(out, value);
        // Quirk: the comma decision looks at the position before skipped entries are removed.
        if i != last {
            out.push(',');
        }
    }
    out.push('}');
}

fn write_value(out: &mut String, value: &Value) {
    match value {
        Value::Boolean(b) => write!(out, "{b}").unwrap(),
        Value::Integer(n) => write!(out, "{n}").unwrap(),
        Value::Real(x) => out.push_str(&format_real(*x)),
        Value::Date(d) => {
            // shortcut: sub-second precision is dropped; SEB XML dates carry whole seconds.
            write!(out, "{}.0000000Z", &d.to_xml_format()[..19]).unwrap();
        }
        Value::Data(bytes) => write!(out, "\"{}\"", STANDARD.encode(bytes)).unwrap(),
        // No escaping, by design of SEB-JSON.
        Value::String(s) => write!(out, "\"{s}\"").unwrap(),
        Value::Array(items) => {
            out.push('[');
            for (i, item) in items.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                write_value(out, item);
            }
            out.push(']');
        }
        Value::Dictionary(dict) => write_dict(out, dict),
        _ => out.push_str("\"\""),
    }
}

/// Mimics .NET `StringComparer.InvariantCulture` for ASCII keys: case-insensitive with
/// symbols < digits < letters, then lowercase before uppercase.
fn compare_keys(a: &str, b: &str) -> Ordering {
    let primary = |c: char| {
        let class = if c.is_alphabetic() {
            2
        } else if c.is_ascii_digit() {
            1
        } else {
            0
        };
        (class, c.to_ascii_lowercase())
    };
    a.chars()
        .map(primary)
        .cmp(b.chars().map(primary))
        .then_with(|| {
            a.chars()
                .map(char::is_uppercase)
                .cmp(b.chars().map(char::is_uppercase))
        })
}

/// .NET Framework `double.ToString()`: "G" with 15 significant digits.
fn format_real(x: f64) -> String {
    match x {
        _ if x.is_nan() => return "NaN".into(),
        f64::INFINITY => return "Infinity".into(),
        f64::NEG_INFINITY => return "-Infinity".into(),
        0.0 => return "0".into(),
        _ => {}
    }

    let scientific = format!("{:.14e}", x.abs());
    let (mantissa, exponent) = scientific.split_once('e').expect("exponent format");
    let exponent: i32 = exponent.parse().expect("integer exponent");
    let digits = mantissa.replace('.', "");
    let digits = digits.trim_end_matches('0');

    let body = if exponent <= -5 || exponent >= 15 {
        let (first, rest) = digits.split_at(1);
        let dot = if rest.is_empty() { "" } else { "." };
        let exp_sign = if exponent < 0 { '-' } else { '+' };
        format!("{first}{dot}{rest}E{exp_sign}{:02}", exponent.abs())
    } else if exponent < 0 {
        format!("0.{}{digits}", "0".repeat((-exponent - 1) as usize))
    } else {
        let width = exponent as usize + 1;
        let padded = format!("{digits:0<width$}");
        let (int, frac) = padded.split_at(width);
        if frac.is_empty() {
            int.to_string()
        } else {
            format!("{int}.{frac}")
        }
    };
    if x < 0.0 { format!("-{body}") } else { body }
}
