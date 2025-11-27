use std::{
    collections::HashMap,
    fs::{self, File},
};

use transmuter::decode::{self, Arguments, get_args};

fn main() {
    let mut args_map: HashMap<String, Arguments> = HashMap::new();

    // read the snippet from a file or paste here
    let input = fs::read_to_string("a32.decode").expect("read file");
    // pre-process continuation lines ending with '\'
    let joined = decode::join_continuations(&input);

    let mut file = File::create("output.rs").unwrap();

    for raw_line in joined.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if line.starts_with('&') {
            let arg = get_args(line);
            arg.output(&mut file).unwrap();
            args_map.insert(arg.name.clone(), arg);
        }
    }
}
