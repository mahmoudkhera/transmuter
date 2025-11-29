use std::{
    collections::HashMap,
    io::{self, Write},
};

use crate::decode::Format;

#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub pos: usize,
    pub len: usize,
    pub mask: u64,
}
impl Field {
    pub fn output(writer: &mut dyn Write) -> io::Result<()> {
        writeln!(writer, " fn extract_simple(inst:u32,len:u32,pos:u32) -> u32 {{")?;

        writeln!(writer, " (inst >> len) & ((1u32 << pos) - 1)")?;
        writeln!(writer, "}}")?;

        Ok(())
    }

    fn get_pos_len(&self) -> (usize, usize) {
        (self.pos, self.len)
    }
}

#[derive(Debug, Clone)]
pub struct MultiField {
    pub subs: Vec<Box<Field>>,

    pub mask: u64,
}

impl MultiField {
    pub fn output(writer: &mut dyn Write) -> io::Result<()> {
        writeln!(
            writer,
            " fn extract_mul(inst:u32,len1:u32,pos1:u32,len2:u32,pos2:u32) -> u32 {{"
        )?;

        writeln!(
            writer,
            "    // mask = (1 << len) - 1
    let mask1 = (1u32 << len1) - 1;
    let mask2 = (1u32 << len2) - 1;

    let field1 = (inst >> pos1) & mask1;
    let field2 = (inst >> pos2) & mask2;

    // concatenate field1 (lower bits) and field2 (upper bits)
    field1 | (field2 << len1)"
        )?;

        writeln!(writer, "}}")?;

        Ok(())
    }

    fn get_subs_paramaters(&self) -> Vec<(usize, usize)> {
        self.subs
            .iter()
            .map(|field| (field.pos, field.len))
            .collect()
    }

    fn get_name(&self) -> String {
        self.subs[1].name.clone()
    }
}

#[derive(Debug, Clone)]
pub struct ConstField {
    pub value: i64,
    pub mask: u64,
}

#[derive(Debug, Clone)]
pub enum FieldType {
    Simple(Field),
    Multi(MultiField),
    Const(ConstField),
}
impl FieldType {
    pub fn output(&self, name: &str, writer: &mut dyn Write) -> io::Result<()> {
        match self {
            FieldType::Simple(f) => Self::output_simple(f, name, writer),
            FieldType::Multi(m) => Self::output_multi(m, name, writer),
            FieldType::Const(c) => Self::output_const(c, name, writer),
        }
    }

    fn output_simple(f: &Field, name: &str, writer: &mut dyn Write) -> io::Result<()> {
        writeln!(writer, "{} : extract_simple(inst,{},{}),", f.name, f.len, f.pos)?;
        Ok(())
    }

    fn output_multi(
        multi_field: &MultiField,
        name: &str,
        writer: &mut dyn Write,
    ) -> io::Result<()> {
        let subsparmaters = multi_field.get_subs_paramaters();

        write!(writer, "{} : extract_mul(inst", name)?;

        for (len, pos) in subsparmaters {

                println!("{}  {}",len,pos);

            write!(writer, ",{} ,{}", len, pos)?;
        }
        writeln!(writer, "),")?;
        Ok(())
    }

    fn output_const(c: &ConstField, name: &str, writer: &mut dyn Write) -> io::Result<()> {
        writeln!(writer, "{}:{},",name, c.value)?;

        Ok(())
    }
}

/// Split first token from string
pub fn split_first_token(s: &str) -> (String, String) {
    let mut it = s.split_whitespace();
    let name = it.next().unwrap_or("").to_string();
    let rest = it.collect::<Vec<_>>().join(" ");
    (name, rest)
}

/// Parse format tail: bit tokens (left) and &base and asments (right)
/// Example rest: "---- ... s:1 rn:4 ... &s_rrr_shi rn=0"
pub fn parse_format_tail(name: &str, s: &str) -> (Vec<String>, String, Vec<(String, String)>) {
    let parts = s
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

    // Find token that starts with '&'
    let mut amp_index = None;
    for (i, p) in parts.iter().enumerate() {
        if p.starts_with('&') {
            amp_index = Some(i);
            break;
        }
    }

    if let Some(ai) = amp_index {
        let bit_tokens = parts[0..ai].to_vec();
        let base = parts[ai][1..].to_string();

        let mut ass = Vec::new();
        for tok in parts.iter().skip(ai + 1) {
            if let Some(eq) = tok.find('=') {
                let name = tok[..eq].to_string();
                let val = tok[eq + 1..].to_string();

                ass.push((name, val));
            }
        }

        (bit_tokens, base, ass)
    } else {
        // No & found - format has no explicit base
        // Use format name itself as base (inferred argument set)
        let base = name.to_string();
        (parts, base, Vec::new())
    }
}

/// Parse a single field token and return FieldType
fn parse_field_token(token: &str, current_pos: isize) -> Option<(String, FieldType, isize)> {
    // Handle field definitions like "rd:4" or "imm:s8" or "name:s4"
    if token.contains(':') && !token.starts_with('%') {
        let parts: Vec<&str> = token.split(':').collect();
        if parts.len() != 2 {
            return None;
        }

        let field_name = parts[0].to_string();
        let len_spec = parts[1];

        if let Ok(len) = len_spec.parse::<isize>() {
            let pos = (current_pos - (len - 1)) as usize;
            let mask = (((1u64 << len) - 1) as u64) << pos;

            let field = Field {
                name: field_name.clone(),
                pos,
                len: len as usize,
                mask,
            };

            return Some((field_name, FieldType::Simple(field), current_pos - len));
        }
    }

    None
}

/// Parse format: compute fixedmask/fixedbits from tokens and collect fields
pub fn parse_format(
    name: String,
    bit_tokens: &[String],
    base: &str,
    ass: Vec<(String, String)>,
    total_fields: &mut HashMap<String, FieldType>,
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

        // Handle field definitions: "rd:4", "imm:8", etc.
        if let Some((field_name, field_type, new_pos)) = parse_field_token(token, current_pos) {
            fields.insert(field_name, field_type);
            current_pos = new_pos;
            continue;
        }

        // Handle mixed tokens like "0001" or "10.."
        for ch in token.chars() {
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
    println!("{:?}", ass);
    // Process asments: "rn=0", "imm=%field", "val=!function", etc.
    for (fname, valstr) in &ass {
        if valstr.starts_with('%') {
            // Field reference: "field=%other_field"
            // This creates a reference to another field (handled later)
            // For now, treat as NamedField
            let ref_name = &valstr[1..];
            // We'd need to look up the referenced field's properties

            let field = total_fields.get(ref_name).unwrap().clone();
            fields.insert(fname.clone(), field);

            println!("{:?}", fields);
        } else {
            // Constant asment: "field=0", "field=-5", etc.
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

    Format {
        name,
        base: base.to_string(),
        fields,
        fixedbits,
        fixedmask,
    }
}

/// Parse a multi-field definition (for compound fields)
/// Example: "%imm24  0:s8 8:8 16:8"
pub fn parse_multi_field(s: &str) -> (String, FieldType) {
    let mut parts = s.split_whitespace();
    let name = parts.next().unwrap_or("").to_string();
    let name = name[1..].to_string();

    let mut subs = Vec::new();
    let mut multi_mask = 0u64;

    for part in parts {
        let token: Vec<&str> = part.split(':').collect();
        if token.len() != 2 {
            continue;
        }

        let pos: usize = token[0].parse().unwrap_or(0);
        let len_spec = token[1];

        let len: usize = len_spec.parse().unwrap_or(0);
        let mask = (((1u64 << len) - 1) as u64) << pos;
        multi_mask |= mask;

        let field = Field {
            name: name.clone(),
            pos,
            len,
            mask,
        };

        subs.push(Box::new(field));
    }

    let multi = MultiField {
        subs,
        mask: multi_mask,
    };

    (name, FieldType::Multi(multi))
}
