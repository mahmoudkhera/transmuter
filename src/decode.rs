use std::{
    collections::HashMap,
    io::{self, Write},
};

/// Simple field representation (contiguous)
#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub pos: usize, // LSB bit index (0..31)
    pub len: usize,
    pub mask: u64, // ((1<<len)-1) << pos
}

/// Arguments: &name line
#[derive(Debug, Clone)]
pub struct Arguments {
    pub name: String,
    pub fields: Vec<String>, // order
}

impl Default for Arguments {
    fn default() -> Self {
        Self {
            name: Default::default(),
            fields: Default::default(),
        }
    }
}

impl Arguments {
    pub fn output(&self, writer: &mut dyn Write) -> io::Result<()> {
        writeln!(writer, "pub struct {} {{", self.name)?;

        for field in &self.fields {
            writeln!(writer, "    pub {}: u32,", field)?;
        }
        writeln!(writer, "}}")?;

        Ok(())
    }
}

pub fn get_args(line: &str) -> Arguments {
    let mut parts = line.split_whitespace();

    if let Some(name) = parts.next() {
        // remove leading '&'
        let clean_name = name.trim_start_matches('&').to_string();

        let fields: Vec<String> = parts.map(|s| s.to_string()).collect();

        return Arguments {
            name: clean_name,
            fields,
        };
    }

    Arguments::default()
}

/// Format: @name line (bit template + &base and optional assignments)
#[derive(Debug, Clone)]
struct Format {
    name: String,
    base: String,                   // reference to &args name
    fields: HashMap<String, Field>, // extracted fields with pos+len
    fixedbits: u64,
    fixedmask: u64,
}

//helper functions

/// Put the line after the '/' and add it  to the previous line
pub fn join_continuations(s: &str) -> String {
    let mut output = String::new();
    let mut pending = String::new();
    for line in s.lines() {
        let trimmed_line = line.trim_end();

        if trimmed_line.ends_with('\\') {
            pending.push_str(trimmed_line.trim_end_matches('\\'));
            pending.push(' ');
        } else {
            pending.push_str(trimmed_line);
            output.push_str(&pending);
            output.push('\n');
            pending.clear();
        }
    }

    if !pending.is_empty() {
        output.push_str(&pending);
        output.push('\n');
    }

    output
}

/// Split first token and the rest of the line

fn split_first_token(s: &str) -> (String, String) {
    let mut it = s.split_whitespace();
    let name = it.next().unwrap_or("").to_string();

    let rest = it.collect::<Vec<_>>().join(" ");

    (name, rest)
}

/// Parse format tail: bit tokens (left) and &base and assignments (right)
/// Example rest: "---- ... s:1 rn:4 ... &s_rrr_shi rn=0"

pub fn parse_format_tail(s: &str) -> (Vec<String>, String, Vec<(String, String)>) {
    let parts = s
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

    // find token that starts with '&'
    let mut amp_index = None;
    for (i, p) in parts.iter().enumerate() {
        if p.starts_with('&') {
            amp_index = Some(i);
            break;
        }
    }

    if let Some(ai) = amp_index {
        let bit_tokens = parts[0..ai].to_vec(); //"---- s:1 rn:4 &s_rrr_shi rn=0 s=1"

        let base = parts[ai][1..].to_string(); //extract base "s_rrr_shi"

        let mut assigns = Vec::new();

        for tok in parts.iter().skip(ai + 1) {
            if let Some(eq) = tok.find('=') {
                let name = tok[..eq].to_string();
                let val = tok[eq + 1..].to_string();
                assigns.push((name, val));
            }
        }

        (bit_tokens, base, assigns)
    } else {
        // no & found (shouldn't happen for valid format)
        (parts, String::new(), Vec::new())
    }
}
