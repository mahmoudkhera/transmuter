use std::{
    collections::HashMap,
    fs::{self, File},
};

use transmuter::decode::{
    self, Arguments, Field, Format, get_args, parse_format, parse_format_tail, split_first_token,
};

fn main() {
    let mut args_map: HashMap<String, Arguments> = HashMap::new();
    let mut formats: HashMap<String, Format> = HashMap::new();

    // read the snippet from a file or paste here
    let input = fs::read_to_string("a32.decode").expect("read file");
    // pre-process continuation lines ending with '\'
    let joined = decode::join_continuations(&input);

    let mut file = File::create("output.rs").unwrap();
    Field::output(&mut file).unwrap();

    for raw_line in joined.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if line.starts_with('&') {
            let arg = get_args(line);
            arg.output(&mut file).unwrap();
            args_map.insert(arg.name.clone(), arg);
        } else if line.starts_with('@') {
            // @formatname <bittemplate tokens...> &base maybe assignments
            let (name, rest) = split_first_token(&line[1..]);
            let (bit_tokens, base, assigns) = parse_format_tail(&name, &rest);
            let fmt = parse_format(name.to_string(), &bit_tokens, &base, assigns);
            fmt.output(&mut file).unwrap();
            formats.insert(fmt.name.clone(), fmt);
        }
    }

    // println!("\nFormats:");
    // for (k,v) in &formats { println!("  @{} -> {:?}", k, v); }
}
