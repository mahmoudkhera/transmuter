use std::{
    collections::HashMap,
    io::{self, Write},
};

use crate::{
    decode::{Arguments, Format, split_first_token},
    field::{FieldType, parse_format_tail},
};

/// Pattern: instruction pattern line
#[derive(Debug, Clone)]
pub struct Pattern {
    pub name: String,
    pub format: String,
    pub fixedbits: u64,
    pub fixedmask: u64,
}

impl Pattern {
    pub fn output(&self, writer: &mut dyn Write) -> io::Result<()> {
        writeln!(
            writer,
            "// Pattern: {} (format: {}, mask: 0x{:08x}, bits: 0x{:08x})",
            self.name, self.format, self.fixedmask, self.fixedbits
        )?;
        Ok(())
    }
}

/// Group type for pattern groups
#[derive(Debug, Clone, PartialEq)]
pub enum GroupType {
    Overlap,   // { } - patterns checked in order, can overlap
    NoOverlap, // [ ] - patterns are mutually exclusive
}

/// Pattern group for hierarchical pattern organization
#[derive(Debug, Clone)]
pub struct PatternGroup {
    pub group_type: GroupType,
    pub patterns: HashMap<String, Pattern>,
    pub subgroups: Vec<PatternGroup>,
    pub indent_level: usize,
}

impl PatternGroup {
    pub fn new(group_type: GroupType, indent_level: usize) -> Self {
        Self {
            group_type,
            patterns: HashMap::new(),
            subgroups: Vec::new(),
            indent_level,
        }
    }

    pub fn add_pattern(&mut self, pattern: Pattern) {
        self.patterns.insert(pattern.name.clone(), pattern);
    }

    pub fn add_subgroup(&mut self, group: PatternGroup) {
        self.subgroups.push(group);
    }

    /// Generate decoder code for this group
    pub fn generate_decoder(&self, writer: &mut dyn Write, var_name: &str) -> io::Result<()> {
        let indent = "    ".repeat(self.indent_level);

        match self.group_type {
            GroupType::Overlap => {
                writeln!(writer, "{}// Overlap group - check in order", indent)?;

                // Process subgroups first
                for subgroup in &self.subgroups {
                    subgroup.generate_decoder(writer, var_name)?;
                }

                // Then check patterns in order
                for (_, pattern) in self.patterns.iter() {
                    writeln!(
                        writer,
                        "{}if ({} & 0x{:08x}) == 0x{:08x} {{",
                        indent, var_name, pattern.fixedmask, pattern.fixedbits
                    )?;
                    writeln!(writer, "{}    // Matched: {}", indent, pattern.name)?;
                    writeln!(
                        writer,
                        "{}    let args = extract_{}({});",
                        indent, pattern.format, var_name
                    )?;
                    writeln!(
                        writer,
                        "{}    return Some(Instruction::{} {{ args }});",
                        indent, pattern.name
                    )?;
                    writeln!(writer, "{}}}", indent)?;
                }
            }

            GroupType::NoOverlap => {
                writeln!(
                    writer,
                    "{}// No-overlap group - mutually exclusive patterns",
                    indent
                )?;

                if !self.patterns.is_empty() {
                    // Find the common mask for all patterns
                    let common_mask = self
                        .patterns
                        .iter()
                        .map(|(_, p)| p.fixedmask)
                        .fold(u64::MAX, |acc, mask| acc & mask);

                    writeln!(
                        writer,
                        "{}match {} & 0x{:08x} {{",
                        indent, var_name, common_mask
                    )?;

                    for (_, pattern) in &self.patterns {
                        writeln!(
                            writer,
                            "{}    0x{:08x} => {{  // {}",
                            indent,
                            pattern.fixedbits & common_mask,
                            pattern.name
                        )?;
                        writeln!(
                            writer,
                            "{}        let args = extract_{}({});",
                            indent, pattern.format, var_name
                        )?;
                        writeln!(
                            writer,
                            "{}        return Some(Instruction::{} {{ args }});",
                            indent, pattern.name
                        )?;
                        writeln!(writer, "{}    }}", indent)?;
                    }

                    writeln!(writer, "{}    _ => {{}}", indent)?;
                    writeln!(writer, "{}}}", indent)?;
                }

                // Process subgroups
                for subgroup in &self.subgroups {
                    subgroup.generate_decoder(writer, var_name)?;
                }
            }
        }

        Ok(())
    }

    /// Output documentation for this group
    pub fn output_doc(&self, writer: &mut dyn Write) -> io::Result<()> {
        let indent = "// ".to_string() + &"  ".repeat(self.indent_level);

        let group_name = match self.group_type {
            GroupType::Overlap => "Overlap Group",
            GroupType::NoOverlap => "No-Overlap Group",
        };

        writeln!(writer, "{}{} {{", indent, group_name)?;

        for (name, _) in &self.patterns {
            writeln!(writer, "{}  {}", indent, name)?;
        }

        for subgroup in &self.subgroups {
            subgroup.output_doc(writer)?;
        }

        writeln!(writer, "{}}}", indent)?;

        Ok(())
    }

    pub fn output_instruction_variant(
        &self,
        writer: &mut dyn Write,
        formats: &HashMap<String, Format>,
    ) -> io::Result<()> {
        writeln!(writer, "pub enum Instruction {{")?;

        for (name, pattern) in &self.patterns {
            println!("name of the pattern {}", name);
            if let Some(format) = formats.get(&pattern.format) {
                writeln!(writer, "{} {{args : arg_{} }},", name, format.base)?;
            }
        }

        for sub in &self.subgroups {
            sub.sub_output_instruction_variant(writer, formats)?;
        }
        writeln!(writer, "}}")?;

        Ok(())
    }

    fn sub_output_instruction_variant(
        &self,
        writer: &mut dyn Write,
        formats: &HashMap<String, Format>,
    ) -> io::Result<()> {
        for (name, pattern) in &self.patterns {
            println!("name of the pattern {}", name);
            if let Some(format) = formats.get(&pattern.format) {
                writeln!(writer, "{} {{args : arg_{} }},", name, format.base)?;
            }
        }
         for sub in &self.subgroups {
            sub.sub_output_instruction_variant(writer, formats)?;
        }

        Ok(())
    }
}

/// Parse a pattern line and extract its information
pub fn parse_pattern(
    line: &str,
    formats: &mut HashMap<String, Format>,
    fields: &mut HashMap<String, FieldType>,
    args_map: &mut HashMap<String, Arguments>,
) -> Option<Pattern> {
    let (name, rest) = split_first_token(line);
    let (bit_tokens, base, assigns) = parse_format_tail(&name, &rest);
    let mut current_pos: isize = 31;
    let mut pattern_fields: HashMap<String, FieldType> = HashMap::new();
    let mut fixedmask: u64 = 0;
    let mut fixedbits: u64 = 0;

    // Parse bit tokens
    for token in &bit_tokens {
        if token.chars().all(|c| c == '.' || c == '-' || c == '_') {
            current_pos -= token.len() as isize;
            println!("current pos {}   len {}", current_pos, token.len());

            continue;
        }

        // Handle field definitions
        if token.contains(':') && !token.starts_with('%') {
            let parts: Vec<&str> = token.split(':').collect();
            if parts.len() == 2 {
                let field_name = parts[0].to_string();
                let len_spec = parts[1];

                let (is_signed, len_str) = if len_spec.starts_with('s') {
                    (true, &len_spec[1..])
                } else {
                    (false, len_spec)
                };

                if let Ok(len) = len_str.parse::<isize>() {
                    let pos = (current_pos - (len - 1)) as usize;
                    let mask = (((1u64 << len) - 1) as u64) << pos;

                    let field = crate::field::Field {
                        name: field_name.clone(),
                        pos,
                        len: len as usize,
                        mask,
                        is_signed,
                    };

                    pattern_fields.insert(field_name, FieldType::Simple(field));
                    current_pos -= len;
                    continue;
                }
            }
        }

        // Handle fixed bits
        for ch in token.chars() {
            if ch == '@' || (ch != '1' || ch != '1') {
                break;
            }
            match ch {
                '0' => {
                    let pos = current_pos as usize;

                    fixedmask |= 1u64 << pos;
                    current_pos -= 1;
                }
                '1' => {
                    let pos = current_pos as usize;
                    fixedmask |= 1u64 << pos;
                    fixedbits |= 1u64 << pos;
                    current_pos -= 1;
                }
                '.' | '-' | '_' => {
                    current_pos -= 1;
                }
                _ => {
                    current_pos -= 1;
                }
            }
        }
    }

    for (fname, valstr) in &assigns {
        if valstr.starts_with('%') {
            let ref_name = &valstr[1..];
            if let Some(field) = fields.get(ref_name) {
                pattern_fields.insert(fname.clone(), field.clone());
            }
        } else if valstr.starts_with('!') {
            let func_name = &valstr[1..];
            let param_field = crate::field::ParameterField {
                function: func_name.to_string(),
            };
            pattern_fields.insert(fname.clone(), FieldType::Parameter(param_field));
        } else if let Ok(val) = valstr.parse::<i64>() {
            let const_field = crate::field::ConstField {
                value: val,
                mask: 0,
            };
            pattern_fields.insert(fname.clone(), FieldType::Const(const_field));
        }
    }

    // Infer argument set if it doesn't exist
    if !args_map.contains_key(&base) && !pattern_fields.is_empty() {
        println!("  Inferring argument set from pattern: {}", base);
        let inferred_args = Arguments::new(
            base.clone(),
            pattern_fields
                .keys()
                .map(|k| (k.clone(), "u32".to_string()))
                .collect(),
            false,
        );
        args_map.insert(base.clone(), inferred_args);
    }

    if pattern_fields.is_empty() {
        let fmt = Format {
            name: name.clone(),
            base: base.clone(),
            fields: pattern_fields.clone(),
            fixedbits,
            fixedmask,
        };

        formats.insert(name.clone(), fmt);
    }
    if let Some(format) = formats.get_mut(&base) {
        format.fields.extend(pattern_fields);
    }

    Some(Pattern {
        name,
        format: base,
        fixedbits,
        fixedmask,
    })
}
