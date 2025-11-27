use std::io::{self, Write};

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
