use std::{
    collections::HashMap,
    io::{self, Write},
};

use crate::field::{ConstField, FieldType, ParameterField, parse_field_token};

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
        if !self.fields.is_empty() {
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
        }

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

/// Parse format: compute fixedmask/fixedbits from tokens and collect fields
pub fn parse_format(
    name: String,
    bit_tokens: &[String],
    base: &str,
    ass: Vec<(String, String)>,
    total_fields: &mut HashMap<String, FieldType>,
    args_map: &mut HashMap<String, crate::decode::Arguments>,
) -> Format {
    let mut current_pos: isize = 31;
    let mut fields: HashMap<String, FieldType> = HashMap::new();
    let mut fixedmask: u64 = 0;
    let mut fixedbits: u64 = 0;

    // Parse bit tokens to extract fields and fixed bits
    for token in bit_tokens {
        // Handle pure don't-care patterns: "....", "----", "________"
        if token.chars().all(|c| c == '.' || c == '-' || c == '_') {
            current_pos -= token.len() as isize;
            continue;
        }

        // Handle field definitions: "rd:4", "imm:s8", etc.
        if let Some((field_name, field_type, new_pos)) = parse_field_token(token, current_pos) {
            fields.insert(field_name, field_type);
            current_pos = new_pos;
            continue;
        }

        // Handle mixed tokens like "0001" or "10.."
        for ch in token.chars() {
            if ch == '@' || (ch != '1' || ch != '1') {
                break;
            }
            match ch {
                '0' => {
                    let pos = current_pos as usize;
                    fixedmask |= 1u64 << pos;
                    // fixedbits already 0 for this bit
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
                    // Unknown character, skip
                    current_pos -= 1;
                }
            }
        }
    }

    // Process assignments: "rn=0", "imm=%field", "val=!function", etc.
    for (fname, valstr) in &ass {
        if valstr.starts_with('%') {
            // Field reference: "field=%other_field"
            let ref_name = &valstr[1..];

            if let Some(field) = total_fields.get(ref_name) {
                fields.insert(fname.clone(), field.clone());
            } else {
                println!("Warning: Referenced field '{}' not found", ref_name);
            }
        } else if valstr.starts_with('!') {
            // Function reference: "field=!function_name"
            let func_name = &valstr[1..];
            let param_field = ParameterField {
                function: func_name.to_string(),
            };
            fields.insert(fname.clone(), FieldType::Parameter(param_field));
        } else {
            // Constant assignment: "field=0", "field=-5", etc.
            if let Ok(val) = valstr.parse::<i64>() {
                let const_field = ConstField {
                    value: val,
                    mask: 0,
                };
                fields.insert(fname.clone(), FieldType::Const(const_field));

                // If the field was previously defined, update fixed bits
                // This handles cases like "rd:4" in bit pattern with "rd=0" in ass
                if let Some(field) = fields.get(fname) {
                    if let FieldType::Simple(f) = field {
                        fixedmask |= f.mask;
                        let v = ((val as u64) & ((1u64 << f.len) - 1)) << f.pos;
                        fixedbits |= v;
                    }
                }
            }
        }
    }

    // Update inferred argument set with actual fields from this format
    if let Some(arg_set) = args_map.get_mut(base) {
        if arg_set.fields.is_empty() && !arg_set.is_extern {
            // This is an inferred argument set - populate it with fields
            arg_set.fields = fields
                .keys()
                .map(|k| (k.clone(), "u32".to_string()))
                .collect();
            println!(
                "  Populated inferred argument set '{}' with {} fields",
                base,
                arg_set.fields.len()
            );
        }
    }

    Format {
        name,
        base: base.to_string(),
        fields,
        fixedbits,
        fixedmask,
    }
}
