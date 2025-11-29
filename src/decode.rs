use std::{
    collections::HashMap,
    fs::{self, File},
    io::{self, Write},
};

use crate::field::{
    Field, FieldType, MultiField, parse_format, parse_format_tail, parse_multi_field,
};

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
        println!("argswork");

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
            "pub fn extract_{}(inst:u32)->{}{{",
            self.name, self.base
        )?;

        writeln!(writer, "{} {{ ", self.base)?;
        for (name, field) in &self.fields {
            field.output(&name, writer)?;
        }

        writeln!(writer, "}}}}\n ")?;

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

/// Split first token and the rest of the line

pub fn split_first_token(s: &str) -> (String, String) {
    let mut it = s.split_whitespace();
    let name = it.next().unwrap_or("").to_string();

    let rest = it.collect::<Vec<_>>().join(" ");

    (name, rest)
}

pub fn genrate_decode_tree() {
    // read the snippet from a file or paste here
    let input = fs::read_to_string("a32.decode").expect("read file");
    // pre-process continuation lines ending with '\'
    let joined = join_continuations(&input);

    let mut args_map: HashMap<String, Arguments> = HashMap::new();
    let mut fields: HashMap<String, FieldType> = HashMap::new();
    let mut formats: HashMap<String, Format> = HashMap::new();

    let mut file = File::create("output.rs").unwrap();

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
            // @formatname <bittemplate tokens...> &base maybe assignments
            let (name, rest) = split_first_token(&line[1..]);
            let (bit_tokens, base, assigns) = parse_format_tail(&name, &rest);
            let fmt = parse_format(name.to_string(), &bit_tokens, &base, assigns, &mut fields);
            formats.insert(fmt.name.clone(), fmt);
        }
    }
    let mut file = File::create("output.rs").unwrap();

    Field::output(&mut file).unwrap();
    MultiField::output(&mut file).unwrap();
    for (_, arg) in args_map {
        arg.output(&mut file).unwrap();
    }
    for (_, fmt) in formats {
        fmt.output(&mut file).unwrap();
    }
}
