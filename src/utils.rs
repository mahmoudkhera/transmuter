/// Split first token and the rest of the line
pub fn split_first_token(s: &str) -> (String, String) {
    let mut it = s.split_whitespace();
    let name = it.next().unwrap_or("").to_string();

    let rest = it.collect::<Vec<_>>().join(" ");

    (name, rest)
}

// Parse format tail: bit tokens (left) and &base and asments (right)
/// Example rest: "---- ... s:1 rn:4 ... &s_rrr_shi rn=0"
pub fn parse_format_tail(name: &str, s: &str) -> (Vec<String>, String, Vec<(String, String)>) {
    let parts = s
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

    // Find token that starts with '&'
    let mut amp_index = None;
    for (i, p) in parts.iter().enumerate() {
        if p.starts_with('&') || p.starts_with('@') {
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
            if tok.starts_with('%') {
                let mut chars = tok.chars();
                let ass_parm = chars.next().unwrap().to_string();
                ass.push((ass_parm, chars.collect()));
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
