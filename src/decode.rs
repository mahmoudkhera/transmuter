use std::{
    collections::HashMap,
    fs::{self, File},
    io::{self, Write},
};

use crate::field::{
    Field, FieldType, MultiField, parse_format, parse_format_tail, parse_multi_field,
};
pub fn genrate_decode_file() -> io::Result<()> {
    // read the snippet from a file or paste here
    let input = fs::read_to_string("a32.decode").expect("read file");
    // pre-process continuation lines ending with '\'
    let joined = join_continuations(&input);

    let mut args_map: HashMap<String, Arguments> = HashMap::new();
    let mut fields: HashMap<String, FieldType> = HashMap::new();
    let mut formats: HashMap<String, Format> = HashMap::new();

    for raw_line in joined.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if line.starts_with('&') {
            let arg = get_args(line);
            args_map.insert(arg.name.clone(), arg);
        } else if line.starts_with('%') {
            let (name, field) = parse_multi_field(&line);
            fields.insert(name.to_string(), field);
        } else if line.starts_with('@') {
            let (name, rest) = split_first_token(&line[1..]);
            let (bit_tokens, base, assigns) = parse_format_tail(&name, &rest);

            // Check if argument set exists
            if !args_map.contains_key(&base) {
                println!("  Inferring argument set: {}", base);

                // Create empty placeholder
                let inferred_args = Arguments {
                    name: base.clone(),
                    fields: Vec::new(), // Empty - will be filled later
                    is_extern: false,
                };
                args_map.insert(base.clone(), inferred_args);
            }

            // Parse format with mutable args_map
            let fmt = parse_format(
                name,
                &bit_tokens,
                &base,
                assigns,
                &mut fields,
                &mut args_map,
            );
            formats.insert(fmt.name.clone(), fmt);
        }
    }

    // Create output file
    let output_path = "output.rs";
    let mut file = File::create(output_path)?;

    // Write header
    write_header(&mut file)?;

    // Write extraction helper functions
    writeln!(file, "// ===== Extraction Helper Functions =====")?;
    writeln!(file)?;
    Field::output(&mut file)?;
    writeln!(file)?;
    MultiField::output(&mut file)?;
    writeln!(file)?;
    MultiField::output_functions(&mut file)?;
    writeln!(file)?;

    // Write argument structures
    writeln!(file, "// ===== Argument Structures =====")?;
    writeln!(file)?;
    for (_, arg) in &args_map {
        arg.output(&mut file)?;
    }

    // Write format extraction functions
    writeln!(file, "// ===== Format Extraction Functions =====")?;
    writeln!(file)?;
    for (_, fmt) in &formats {
        fmt.output(&mut file)?;
    }

    Ok(())
}

/// Arguments: &name line
#[derive(Debug, Clone)]
pub struct Arguments {
    pub name: String,
    pub fields: Vec<(String, String)>, // (field_name, field_type)
    pub is_extern: bool,
}

impl Default for Arguments {
    fn default() -> Self {
        Self {
            name: Default::default(),
            fields: Default::default(),
            is_extern: false,
        }
    }
}

impl Arguments {
    /// Create a new Arguments instance
    pub fn new(name: String, fields: Vec<(String, String)>, is_extern: bool) -> Self {
        Self {
            name,
            fields,
            is_extern,
        }
    }

    /// Generate Rust struct code for this argument set
    pub fn output(&self, writer: &mut dyn Write) -> io::Result<()> {
        // Don't generate struct for extern arguments
        if self.is_extern {
            writeln!(writer, "// extern struct: {}", self.name)?;
            return Ok(());
        }

        // Skip empty argument sets
        if self.fields.is_empty() {
            writeln!(writer, "// Warning: Empty argument set: {}", self.name)?;
            return Ok(());
        }

        writeln!(writer, "#[derive(Debug, Clone, PartialEq)]")?;
        writeln!(writer, "pub struct arg_{} {{", self.name)?;

        for (field_name, field_type) in &self.fields {
            writeln!(writer, "    pub {}: {},", field_name, field_type)?;
        }

        writeln!(writer, "}}")?;
        writeln!(writer)?;

        Ok(())
    }
}

/// Parse &args line with optional types and !extern flag
/// Examples:
///   &reg3       ra rb rc
///   &loadstore  reg base offset
///   &longldst   reg base offset:int64_t
///   &extern_arg field1 field2 !extern
pub fn get_args(line: &str) -> Arguments {
    let mut parts = line.split_whitespace();

    if let Some(name) = parts.next() {
        // Remove leading '&'
        let clean_name = name.trim_start_matches('&').to_string();

        let mut fields = Vec::new();
        let mut is_extern = false;

        for part in parts {
            if part == "!extern" {
                is_extern = true;
                continue;
            }

            // Check if field has explicit type: "field:type"
            if let Some(colon_pos) = part.find(':') {
                let field_name = part[..colon_pos].to_string();
                let field_type = part[colon_pos + 1..].to_string();
                fields.push((field_name, field_type));
            } else {
                // Default type is u32
                fields.push((part.to_string(), "u32".to_string()));
            }
        }

        return Arguments {
            name: clean_name,
            fields,
            is_extern,
        };
    }

    Arguments::default()
}

/// Format: @name line (bit template + &base and optional assignments)
#[derive(Debug, Clone)]
pub struct Format {
    pub name: String,
    pub base: String,
    pub fields: HashMap<String, FieldType>,
    pub fixedbits: u64,
    pub fixedmask: u64,
}

impl Format {
    pub fn output(&self, writer: &mut dyn Write) -> io::Result<()> {
        writeln!(
            writer,
            "pub fn extract_{}(inst: u32) -> arg_{} {{",
            self.name, self.base
        )?;

        writeln!(writer, "    arg_{} {{", self.base)?;

        for (name, field) in &self.fields {
            write!(writer, "        ")?;
            field.output(name, writer)?;
        }

        writeln!(writer, "    }}")?;
        writeln!(writer, "}}")?;
        writeln!(writer)?;

        Ok(())
    }
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

/// Write the file header with metadata and compiler directives
fn write_header(writer: &mut dyn Write) -> io::Result<()> {
    writeln!(writer, "// Auto-generated from a32.decode")?;
    writeln!(writer, "// Do not edit manually")?;
    writeln!(writer)?;
    writeln!(writer, "#![allow(non_camel_case_types)]")?;
    writeln!(writer, "#![allow(clippy::all)]")?;
    writeln!(writer)?;
    Ok(())
}

/// Split first token and the rest of the line
pub fn split_first_token(s: &str) -> (String, String) {
    let mut it = s.split_whitespace();
    let name = it.next().unwrap_or("").to_string();

    let rest = it.collect::<Vec<_>>().join(" ");

    (name, rest)
}
