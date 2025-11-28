


#[cfg(test)]
mod tests {
    use crate::decode::parse_format_tail;

    

    #[test]
    fn test_basic_format_with_base_and_assignments() {
        let input = "---- ... s:1 rn:4 &s_rrr_shi rn=0 s=1";
        let (bits, base, assigns) = parse_format_tail(input);
        
        assert_eq!(bits, vec!["----", "...", "s:1", "rn:4"]);
        assert_eq!(base, "s_rrr_shi");
        assert_eq!(assigns, vec![
            ("rn".to_string(), "0".to_string()),
            ("s".to_string(), "1".to_string())
        ]);
    }

    #[test]
    fn test_format_with_only_base_no_assignments() {
        let input = "---- ... .... s:1 rn:4 rd:4 shim:5 shty:2 . rm:4 &s_rrr_shi";
        let (bits, base, assigns) = parse_format_tail(input);
        
        assert_eq!(bits, vec!["----", "...", "....", "s:1", "rn:4", "rd:4", "shim:5", "shty:2", ".", "rm:4"]);
        assert_eq!(base, "s_rrr_shi");
        assert_eq!(assigns.len(), 0);
    }

    
}