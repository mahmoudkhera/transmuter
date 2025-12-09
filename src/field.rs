use std::{
    collections::HashMap,
    io::{self, Write},
};

#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub pos: usize,
    pub len: usize,
    pub mask: u64,
    pub is_signed: bool,
}
impl Field {
    pub fn output(writer: &mut dyn Write) -> io::Result<()> {
        writeln!(
            writer,
            " fn extract_simple(inst:u32,pos:u32,len:u32) -> u32 {{"
        )?;
        writeln!(writer, "    (inst >> pos) & ((1u32 << len) - 1)")?;
        writeln!(writer, " }}")?;
        writeln!(writer)?;

        writeln!(
            writer,
            " fn extract_signed(inst:u32,pos:u32,len:u32) -> i32 {{"
        )?;
        writeln!(writer, "    let val = (inst >> pos) & ((1u32 << len) - 1);")?;
        writeln!(writer, "    // Sign extend")?;
        writeln!(writer, "    if (val & (1u32 << (len - 1))) != 0 {{")?;
        writeln!(writer, "        (val | (!((1u32 << len) - 1))) as i32")?;
        writeln!(writer, "    }} else {{")?;
        writeln!(writer, "        val as i32")?;
        writeln!(writer, "    }}")?;
        writeln!(writer, " }}")?;
        Ok(())
    }

    

    pub fn get_pos_len(&self) -> (usize, usize) {
        (self.pos, self.len)
    }
}

#[derive(Debug, Clone)]
pub struct MultiField {
    pub subs: Vec<Box<Field>>,
    pub mask: u64,
    pub function: Option<String>,
}

impl MultiField {
    pub fn output(writer: &mut dyn Write) -> io::Result<()> {
        // Generate multi-field extraction for 2 fields
        writeln!(
            writer,
            " fn extract_mul2(inst:u32,pos1:u32,len1:u32,pos2:u32,len2:u32) -> u32 {{"
        )?;
        writeln!(writer, "    let mask1 = (1u32 << len1) - 1;")?;
        writeln!(writer, "    let mask2 = (1u32 << len2) - 1;")?;
        writeln!(writer, "    let field1 = (inst >> pos1) & mask1;")?;
        writeln!(writer, "    let field2 = (inst >> pos2) & mask2;")?;
        writeln!(
            writer,
            "    // concatenate field1 (lower bits) and field2 (upper bits)"
        )?;
        writeln!(writer, "    field1 | (field2 << len1)")?;
        writeln!(writer, " }}")?;
        writeln!(writer)?;

        // Generate multi-field extraction for 3 fields
        writeln!(
            writer,
            " fn extract_mul3(inst:u32,pos1:u32,len1:u32,pos2:u32,len2:u32,pos3:u32,len3:u32) -> u32 {{"
        )?;
        writeln!(writer, "    let mask1 = (1u32 << len1) - 1;")?;
        writeln!(writer, "    let mask2 = (1u32 << len2) - 1;")?;
        writeln!(writer, "    let mask3 = (1u32 << len3) - 1;")?;
        writeln!(writer, "    let field1 = (inst >> pos1) & mask1;")?;
        writeln!(writer, "    let field2 = (inst >> pos2) & mask2;")?;
        writeln!(writer, "    let field3 = (inst >> pos3) & mask3;")?;
        writeln!(
            writer,
            "    field1 | (field2 << len1) | (field3 << (len1 + len2))"
        )?;
        writeln!(writer, " }}")?;
        writeln!(writer)?;

        // Generate multi-field extraction for 4 fields
        writeln!(
            writer,
            " fn extract_mul4(inst:u32,pos1:u32,len1:u32,pos2:u32,len2:u32,pos3:u32,len3:u32,pos4:u32,len4:u32) -> u32 {{"
        )?;
        writeln!(writer, "    let mask1 = (1u32 << len1) - 1;")?;
        writeln!(writer, "    let mask2 = (1u32 << len2) - 1;")?;
        writeln!(writer, "    let mask3 = (1u32 << len3) - 1;")?;
        writeln!(writer, "    let mask4 = (1u32 << len4) - 1;")?;
        writeln!(writer, "    let field1 = (inst >> pos1) & mask1;")?;
        writeln!(writer, "    let field2 = (inst >> pos2) & mask2;")?;
        writeln!(writer, "    let field3 = (inst >> pos3) & mask3;")?;
        writeln!(writer, "    let field4 = (inst >> pos4) & mask4;")?;
        writeln!(
            writer,
            "    field1 | (field2 << len1) | (field3 << (len1 + len2)) | (field4 << (len1 + len2 + len3))"
        )?;
        writeln!(writer, " }}")?;

        Ok(())
    }

    pub fn output_functions(writer: &mut dyn Write) -> io::Result<()> {
        // Common transformation functions
        writeln!(writer, " fn times_2(val:u32) -> u32 {{")?;
        writeln!(writer, "    val << 1")?;
        writeln!(writer, " }}")?;
        writeln!(writer)?;

        writeln!(writer, " fn times_4(val:u32) -> u32 {{")?;
        writeln!(writer, "    val << 2")?;
        writeln!(writer, " }}")?;
        writeln!(writer)?;

        writeln!(writer, " fn times_8(val:u32) -> u32 {{")?;
        writeln!(writer, "    val << 3")?;
        writeln!(writer, " }}")?;
        writeln!(writer)?;

        // ARM-specific functions
        writeln!(writer, " fn expand_imm(val:u32) -> u32 {{")?;
        writeln!(writer, "    // ARM immediate expansion logic")?;
        writeln!(writer, "    let rotate = (val >> 8) & 0xF;")?;
        writeln!(writer, "    let imm = val & 0xFF;")?;
        writeln!(writer, "    imm.rotate_right(rotate * 2)")?;
        writeln!(writer, " }}")?;
        writeln!(writer)?;

        writeln!(writer, " fn negate(val:u32) -> u32 {{")?;
        writeln!(writer, "    (!val).wrapping_add(1)")?;
        writeln!(writer, " }}")?;

        Ok(())
    }

    fn get_subs_parameters(&self) -> Vec<(usize, usize, bool)> {
        self.subs
            .iter()
            .map(|field| (field.pos, field.len, field.is_signed))
            .collect()
    }

    pub fn get_name(&self) -> String {
        self.subs[0].name.clone()
    }
}

#[derive(Debug, Clone)]
pub struct ConstField {
    pub value: i64,
    pub mask: u64,
}

#[derive(Debug, Clone)]
pub struct ParameterField {
    pub function: String,
}

#[derive(Debug, Clone)]
pub enum FieldType {
    Simple(Field),
    Multi(MultiField),
    Const(ConstField),
    Parameter(ParameterField),
}

impl FieldType {
    pub fn output(&self, name: &str, writer: &mut dyn Write) -> io::Result<()> {
        match self {
            FieldType::Simple(f) => Self::output_simple(f, name, writer),
            FieldType::Multi(m) => Self::output_multi(m, name, writer),
            FieldType::Const(c) => Self::output_const(c, name, writer),
            FieldType::Parameter(p) => Self::output_parameter(p, name, writer),
        }
    }
    fn output_simple(f: &Field, name: &str, writer: &mut dyn Write) -> io::Result<()> {
        if f.is_signed {
            writeln!(
                writer,
                "{} : extract_signed(inst,{},{}) as u32,",
                name, f.pos, f.len
            )?;
        } else {
            writeln!(
                writer,
                "{} : extract_simple(inst,{},{}),",
                name, f.pos, f.len
            )?;
        }
        Ok(())
    }

    fn output_multi(
        multi_field: &MultiField,
        name: &str,
        writer: &mut dyn Write,
    ) -> io::Result<()> {
        let subs_params = multi_field.get_subs_parameters();
        let num_fields = subs_params.len();

        match num_fields {
            // Single field (with or without function)
            1 => {
                let (pos, len, is_signed) = subs_params[0];

                write!(writer, "{} : ", name)?;

                // If there's a function, wrap the extraction
                if let Some(func) = &multi_field.function {
                    write!(writer, "{}(", func)?;
                }

                // Extract the field
                if is_signed {
                    write!(writer, "extract_signed(inst,{},{}) as u32", pos, len)?;
                } else {
                    write!(writer, "extract_simple(inst,{},{})", pos, len)?;
                }

                // Close function call if present
                if multi_field.function.is_some() {
                    write!(writer, ")")?;
                }

                writeln!(writer, ",")?;
            }

            // Multiple fields (with or without function)
            2 => {
                let (pos1, len1, _) = subs_params[0];
                let (pos2, len2, _) = subs_params[1];

                write!(writer, "{} : ", name)?;

                // If there's a function, wrap the extraction
                if let Some(func) = &multi_field.function {
                    write!(writer, "{}(", func)?;
                }

                write!(
                    writer,
                    "extract_mul2(inst,{},{},{},{})",
                    pos1, len1, pos2, len2
                )?;

                // Close function call if present
                if multi_field.function.is_some() {
                    write!(writer, ")")?;
                }

                writeln!(writer, ",")?;
            }

            3 => {
                let (pos1, len1, _) = subs_params[0];
                let (pos2, len2, _) = subs_params[1];
                let (pos3, len3, _) = subs_params[2];

                write!(writer, "{} : ", name)?;

                if let Some(func) = &multi_field.function {
                    write!(writer, "{}(", func)?;
                }

                write!(
                    writer,
                    "extract_mul3(inst,{},{},{},{},{},{})",
                    pos1, len1, pos2, len2, pos3, len3
                )?;

                if multi_field.function.is_some() {
                    write!(writer, ")")?;
                }

                writeln!(writer, ",")?;
            }

            4 => {
                let (pos1, len1, _) = subs_params[0];
                let (pos2, len2, _) = subs_params[1];
                let (pos3, len3, _) = subs_params[2];
                let (pos4, len4, _) = subs_params[3];

                write!(writer, "{} : ", name)?;

                if let Some(func) = &multi_field.function {
                    write!(writer, "{}(", func)?;
                }

                write!(
                    writer,
                    "extract_mul4(inst,{},{},{},{},{},{},{},{})",
                    pos1, len1, pos2, len2, pos3, len3, pos4, len4
                )?;

                if multi_field.function.is_some() {
                    write!(writer, ")")?;
                }

                writeln!(writer, ",")?;
            }

            _ => {
                // For more than 4 fields, generate generic extraction code
                write!(writer, "{} : {{", name)?;
                writeln!(writer, "    let mut result = 0u32;")?;
                writeln!(writer, "    let mut shift = 0u32;")?;

                for (pos, len, _) in subs_params {
                    writeln!(
                        writer,
                        "    result |= ((inst >> {}) & ((1u32 << {}) - 1)) << shift;",
                        pos, len
                    )?;
                    writeln!(writer, "    shift += {};", len)?;
                }

                if let Some(func) = &multi_field.function {
                    writeln!(writer, "    {}(result)", func)?;
                } else {
                    writeln!(writer, "    result")?;
                }

                writeln!(writer, "}},")?;
            }
        }

        Ok(())
    }

    fn output_const(c: &ConstField, name: &str, writer: &mut dyn Write) -> io::Result<()> {
        writeln!(writer, "{}:{},", name, c.value)?;
        Ok(())
    }

    fn output_parameter(p: &ParameterField, name: &str, writer: &mut dyn Write) -> io::Result<()> {
        writeln!(writer, "{} : {}(ctx),", name, p.function)?;
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
        if p.starts_with('&')||p.starts_with('@') {
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

        // Check if it's signed (starts with 's')
        let (is_signed, len_str) = if len_spec.starts_with('s') {
            (true, &len_spec[1..])
        } else {
            (false, len_spec)
        };

        if let Ok(len) = len_str.parse::<isize>() {
            let pos = (current_pos - (len - 1)) as usize;
            let mask = (((1u64 << len) - 1) as u64) << pos;

            let field = Field {
                name: field_name.clone(),
                pos,
                len: len as usize,
                mask,
                is_signed,
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
    args_map: &mut HashMap<String, crate::decode::Arguments>,
) -> crate::decode::Format {
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

    crate::decode::Format {
        name,
        base: base.to_string(),
        fields,
        fixedbits,
        fixedmask,
    }
}

// Parse a multi-field definition (for compound fields)
/// Examples:
/// - "%a32extrot  8:4 !function=times_2"         (single field + function)
/// - "%imm24  0:8 8:8 16:8"                      (multiple fields, no function)
/// - "%disp12  0:s1 1:1 2:10 !function=process"  (multiple fields + function)
/// - "%some_param !function=get_value"           (parameter with no fields)

pub fn parse_multi_field(s: &str) -> (String, FieldType) {
    let mut parts = s.split_whitespace();
    let name = parts.next().unwrap_or("").to_string();
    let name = name[1..].to_string();

    let mut subs = Vec::new();
    let mut multi_mask = 0u64;
    let mut function = None;

    for part in parts {
        // Check if this is a function specification
        if part.starts_with("!function=") {
            function = Some(part[10..].to_string());
            continue;
        }

        let token: Vec<&str> = part.split(':').collect();
        if token.len() != 2 {
            continue;
        }

        let pos: usize = token[0].parse().unwrap_or(0);
        let len_spec = token[1];

        // Check for signed fields
        let (is_signed, len_str) = if len_spec.starts_with('s') {
            (true, &len_spec[1..])
        } else {
            (false, len_spec)
        };

        let len: usize = len_str.parse().unwrap_or(0);
        let mask = (((1u64 << len) - 1) as u64) << pos;
        multi_mask |= mask;

        let field = Field {
            name: name.clone(),
            pos,
            len,
            mask,
            is_signed,
        };

        subs.push(Box::new(field));
    }

    // Check if this is a parameter (function with no fields)
    if subs.is_empty() && function.is_some() {
        let param = ParameterField {
            function: function.unwrap(),
        };
        return (name, FieldType::Parameter(param));
    }

    let multi = MultiField {
        subs,
        mask: multi_mask,
        function,
    };

    (name, FieldType::Multi(multi))
}
