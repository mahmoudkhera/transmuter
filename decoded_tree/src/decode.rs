use crate::{
    field::{Field, FieldType, MultiField, parse_multi_field},
    format::{Format, join_continuations, parse_format},
    pattern::{GroupType, PatternGroup, parse_pattern},
    utils::{parse_format_tail, split_first_token},
};
use std::{
    collections::HashMap,
    fs::{self, File},
    io::{self, Write},
    path::Path,
};


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

pub fn generate_decode_file<P: AsRef<Path>>(output_path: P,input_path:P) -> io::Result<()> {
    // Step 1: Read and preprocess file
    let input = fs::read_to_string(input_path)?;
    let joined = join_continuations(&input);

    // Step 2: Prepare storage
    let mut args_map: HashMap<String, Arguments> = HashMap::new();
    let mut fields: HashMap<String, FieldType> = HashMap::new();
    let mut formats: HashMap<String, Format> = HashMap::new();

    // Pattern group structure
    let mut root_group = PatternGroup::new(GroupType::Overlap, 0);
    let mut group_stack = vec![root_group.clone()];
    let mut indent = 0;

    // Step 3: Main parsing loop
    for raw in joined.lines() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Group start
        if line == "{" || line == "[" {
            handle_group_start(line, &mut group_stack, &mut indent);
            continue;
        }

        // Group end
        if line == "}" || line == "]" {
            handle_group_end(line, &mut group_stack, &mut indent);
            continue;
        }

        // Argument set
        if line.starts_with('&') {
            handle_argument_line(line, &mut args_map);
            continue;
        }

        // Multi field
        if line.starts_with('%') {
            handle_multi_field_line(line, &mut fields);
            continue;
        }

        // Format definition
        if line.starts_with('@') {
            handle_format_line(line, &mut args_map, &mut fields, &mut formats);
            continue;
        }

        // Pattern line
        handle_pattern_line(
            line,
            &mut group_stack,
            &mut formats,
            &mut fields,
            &mut args_map,
            indent,
        );
    }

    // Step 4: Collapse unclosed groups
    while group_stack.len() > 1 {
        let last = group_stack.pop().unwrap();
        eprintln!("Warning: Unclosed group at end of file");

        if let Some(parent) = group_stack.last_mut() {
            parent.add_subgroup(last);
        }
    }
    root_group = group_stack.pop().unwrap();

    // Step 5: Write output
    write_output_file(output_path, &root_group, &args_map, &fields, &formats)?;

    // Step 6: Statistics

    println!("Successfully generated!");

    println!("  - Argument sets: {}", args_map.len());
    println!("  - Fields: {}", fields.len());
    println!("  - Formats: {}", formats.len());

    fn count_patterns(g: &PatternGroup) -> usize {
        g.patterns.len() + g.subgroups.iter().map(count_patterns).sum::<usize>()
    }

    println!("  - Patterns: {}", count_patterns(&root_group));

    Ok(())
}

fn handle_group_start(token: &str, stack: &mut Vec<PatternGroup>, indent: &mut usize) {
    *indent += 1;

    let group_type = if token == "{" {
        GroupType::Overlap
    } else {
        GroupType::NoOverlap
    };

    let new_group = PatternGroup::new(group_type, *indent);
    println!("{}[GROUP {}] Start", "  ".repeat(*indent), token);

    stack.push(new_group);
}

fn handle_group_end(token: &str, stack: &mut Vec<PatternGroup>, indent: &mut usize) {
    if stack.len() <= 1 {
        eprintln!("Warning: Unmatched group delimiter: {}", token);
        return;
    }

    let finished = stack.pop().unwrap();
    println!("{}[GROUP {}] End", "  ".repeat(*indent), token);

    *indent -= 1;

    if let Some(parent) = stack.last_mut() {
        parent.add_subgroup(finished);
    }
}
fn handle_argument_line(line: &str, args: &mut HashMap<String, Arguments>) {
    let arg = get_args(line);
    args.insert(arg.name.clone(), arg);
}
fn handle_multi_field_line(line: &str, fields: &mut HashMap<String, FieldType>) {
    let (name, field) = parse_multi_field(line);
    fields.insert(name.to_string(), field);
}
fn handle_format_line(
    line: &str,
    args: &mut HashMap<String, Arguments>,
    fields: &mut HashMap<String, FieldType>,
    formats: &mut HashMap<String, Format>,
) {
    let (name, rest) = split_first_token(&line[1..]);
    let (bits, base, assigns) = parse_format_tail(&name, &rest);

    if !args.contains_key(&base) {
        println!("Inferring argument set: {}", base);
        args.insert(
            base.clone(),
            Arguments {
                name: base.clone(),
                fields: vec![],
                is_extern: false,
            },
        );
    }

    let fmt = parse_format(name, &bits, &base, assigns, fields, args);
    formats.insert(fmt.name.clone(), fmt);
}

fn handle_pattern_line(
    line: &str,
    stack: &mut Vec<PatternGroup>,
    formats: &mut HashMap<String, Format>,
    fields: &mut HashMap<String, FieldType>,
    args: &mut HashMap<String, Arguments>,
    _indent: usize,
) {
    if let Some(pat) = parse_pattern(line, formats, fields, args) {
      

        if let Some(group) = stack.last_mut() {
            group.add_pattern(pat);
        }
    }
}

fn write_output_file<P: AsRef<Path>>(
    output_path: P,
    root: &PatternGroup,
    args: &HashMap<String, Arguments>,
    _fields: &HashMap<String, FieldType>,
    formats: &HashMap<String, Format>,
) -> io::Result<()> {
    let mut file = File::create(output_path)?;

    write_header(&mut file)?;

    writeln!(file, "// ===== Extraction Helpers =====")?;
    writeln!(file)?;
    Field::output(&mut file)?;
    MultiField::output(&mut file)?;
    MultiField::output_functions(&mut file)?;
    writeln!(file)?;

    writeln!(file, "// ===== Argument Sets =====")?;
    for a in args.values() {
        a.output(&mut file)?;
    }

    writeln!(file, "// ===== Format Extraction =====")?;
    for fmt in formats.values() {
        fmt.output(&mut file)?;
    }

    writeln!(file, "// ===== Pattern Group =====")?;
    root.output_instruction_variant(&mut file, formats)?;
    root.generate_decoder(&mut file, "inst", formats)?;

    Ok(())
}
