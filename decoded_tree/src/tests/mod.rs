#[cfg(test)]
mod decode_generator_tests {
    use crate::{
        decode::*,
        field::{ConstField, FieldType},
        format::*,
        pattern::*,
    };

    use std::collections::{HashMap, HashSet};

    // Helper function to create a test buffer
    fn create_test_buffer() -> Vec<u8> {
        Vec::new()
    }

    // Helper function to convert buffer to string
    fn buffer_to_string(buffer: &[u8]) -> String {
        String::from_utf8(buffer.to_vec()).unwrap()
    }

    // ===== Test 1: Empty Arguments Detection =====

    #[test]
    fn test_empty_arguments_detection() {
        let mut args_map = HashMap::new();

        // ERET with no fields
        args_map.insert(
            "eret".to_string(),
            Arguments {
                name: "eret".to_string(),
                fields: vec![], // Empty!
                is_extern: false,
            },
        );

        // MSR_i with fields
        args_map.insert(
            "msr_i".to_string(),
            Arguments {
                name: "msr_i".to_string(),
                fields: vec![
                    ("r".to_string(), "u32".to_string()),
                    ("mask".to_string(), "u32".to_string()),
                ],
                is_extern: false,
            },
        );

        assert!(args_map.get("eret").unwrap().fields.is_empty());
        assert!(!args_map.get("msr_i").unwrap().fields.is_empty());
    }

    // ===== Test 2: Enum Generation with Mixed Arguments =====

    #[test]
    fn test_enum_generation_mixed_args() {
        let mut buffer = create_test_buffer();
        let mut formats = HashMap::new();
        let mut args_map = HashMap::new();

        formats.insert(
            "msr_i".to_string(),
            Format {
                name: "msr_i".to_string(),
                base: "msr_i".to_string(),
                fields: HashMap::from([(
                    "mask".to_string(),
                    FieldType::Const(ConstField { value: 0, mask: 0 }),
                )]),
                fixedbits: 0,
                fixedmask: 0,
            },
        );

        // MSR_i - has fields
        args_map.insert(
            "msr_i".to_string(),
            Arguments {
                name: "msr_i".to_string(),
                fields: vec![
                    ("mask".to_string(), "u32".to_string()),
                    ("imm".to_string(), "u32".to_string()),
                ],
                is_extern: false,
            },
        );

        // Create pattern group
        let mut group = PatternGroup::new(GroupType::Overlap, 0);

        group.add_pattern(Pattern {
            name: "MSR_imm".to_string(),
            format: "msr_i".to_string(),
            fixedbits: 0x0320f000,
            fixedmask: 0x0fb0f000,
        });

        // Generate enum
        group
            .output_instruction_variant(&mut buffer, &formats)
            .unwrap();

        let output = buffer_to_string(&buffer);
        println!("output {}", output);

        // Verify output
        assert!(output.contains("pub enum Instruction"));
        assert!(output.contains("MSR_imm { args: arg_msr_i }")); // Struct variant
    }

    // ===== Test 3: No Duplicate Enums =====

    #[test]
    fn test_no_duplicate_enums() {
        let mut buffer = create_test_buffer();
        let mut formats = HashMap::new();
        let mut args_map = HashMap::new();

        formats.insert(
            "add".to_string(),
            Format {
                name: "add".to_string(),
                base: "add".to_string(),
                fields: HashMap::from([(
                    "mask".to_string(),
                    FieldType::Const(ConstField { value: 0, mask: 0 }),
                )]),
                fixedbits: 0,
                fixedmask: 0,
            },
        );

        args_map.insert(
            "add".to_string(),
            Arguments {
                name: "add".to_string(),
                fields: vec![
                    ("rd".to_string(), "u32".to_string()),
                    ("rn".to_string(), "u32".to_string()),
                ],
                is_extern: false,
            },
        );

        // Create root group with subgroups
        let mut root = PatternGroup::new(GroupType::Overlap, 0);
        let mut sub1 = PatternGroup::new(GroupType::NoOverlap, 1);
        let mut sub2 = PatternGroup::new(GroupType::NoOverlap, 1);

        // Add same pattern to multiple groups
        sub1.add_pattern(Pattern {
            name: "ADD".to_string(),
            format: "add".to_string(),
            fixedbits: 0x00800000,
            fixedmask: 0x0fe00010,
        });

        sub2.add_pattern(Pattern {
            name: "ADD".to_string(), // Duplicate name!
            format: "add".to_string(),
            fixedbits: 0x00800010,
            fixedmask: 0x0fe00010,
        });

        root.add_subgroup(sub1);
        root.add_subgroup(sub2);

        // Generate enum
        root.output_instruction_variant(&mut buffer, &formats)
            .unwrap();

        let output = buffer_to_string(&buffer);

        // Should only have ONE ADD
        let add_count = output.matches("ADD {").count();
        assert_eq!(
            add_count, 1,
            "ADD should appear exactly once, found {}",
            add_count
        );
    }

    // ===== Test 4: Decoder Generation for Empty Args =====

    #[test]
    fn test_decoder_generation_empty_args() {
        let mut buffer = create_test_buffer();
        let mut formats = HashMap::new();
        let mut args_map = HashMap::new();

        formats.insert(
            "eret".to_string(),
            Format {
                name: "eret".to_string(),
                base: "eret".to_string(),
                fields: HashMap::new(),
                fixedbits: 0,
                fixedmask: 0,
            },
        );

        // ERET - no fields
        args_map.insert(
            "eret".to_string(),
            Arguments {
                name: "eret".to_string(),
                fields: vec![],
                is_extern: false,
            },
        );

        let mut group = PatternGroup::new(GroupType::Overlap, 1);
        group.add_pattern(Pattern {
            name: "ERET".to_string(),
            format: "eret".to_string(),
            fixedbits: 0x0160006e,
            fixedmask: 0x0fffffff,
        });

        // Generate decoder
        group
            .generate_decoder(&mut buffer, "inst", &formats)
            .unwrap();

        let output = buffer_to_string(&buffer);

        // Should NOT have extraction
        assert!(!output.contains("let args = extract_eret"));
        assert!(!output.contains("{ args }"));

        // Should have direct return
        assert!(output.contains("return Some(Instruction::ERET)"));
    }

    // ===== Test 5: Decoder Generation for Instructions with Args =====

    #[test]
    fn test_decoder_generation_with_args() {
        let mut buffer = create_test_buffer();
        let mut formats = HashMap::new();
        let mut args_map = HashMap::new();

        formats.insert(
            "add".to_string(),
            Format {
                name: "add".to_string(),
                base: "add".to_string(),
                fields: HashMap::from([(
                    "mask".to_string(),
                    FieldType::Const(ConstField { value: 0, mask: 0 }),
                )]),
                fixedbits: 0,
                fixedmask: 0,
            },
        );

        // ADD - has fields
        args_map.insert(
            "add".to_string(),
            Arguments {
                name: "add".to_string(),
                fields: vec![
                    ("rd".to_string(), "u32".to_string()),
                    ("rn".to_string(), "u32".to_string()),
                ],
                is_extern: false,
            },
        );

        let mut group = PatternGroup::new(GroupType::Overlap, 1);
        group.add_pattern(Pattern {
            name: "ADD".to_string(),
            format: "add".to_string(),
            fixedbits: 0x00800000,
            fixedmask: 0x0fe00010,
        });

        // Generate decoder
        group
            .generate_decoder(&mut buffer, "inst", &formats)
            .unwrap();

        let output = buffer_to_string(&buffer);

        // Should have extraction
        assert!(output.contains("let args = extract_add(inst)"));
        assert!(output.contains("return Some(Instruction::ADD { args })"));
    }

    // ===== Test 6: Distinguishing Mask Calculation =====

    #[test]
    fn test_distinguishing_mask_calculation() {
        let mut group = PatternGroup::new(GroupType::NoOverlap, 0);

        // YIELD: ...0001
        group.add_pattern(Pattern {
            name: "YIELD".to_string(),
            format: "msr_i".to_string(),
            fixedbits: 0x0320f001,
            fixedmask: 0x0fffffff,
        });

        // WFE: ...0002
        group.add_pattern(Pattern {
            name: "WFE".to_string(),
            format: "msr_i".to_string(),
            fixedbits: 0x0320f002,
            fixedmask: 0x0fffffff,
        });

        // WFI: ...0003
        group.add_pattern(Pattern {
            name: "WFI".to_string(),
            format: "msr_i".to_string(),
            fixedbits: 0x0320f003,
            fixedmask: 0x0fffffff,
        });

        let mask = group.calculate_distinguishing_mask();

        // Should be union of all masks
        assert_eq!(mask, 0x0fffffff);

        // Verify each pattern has unique masked bits
        let yield_masked = 0x0320f001u64 & mask;
        let wfe_masked = 0x0320f002u64 & mask;
        let wfi_masked = 0x0320f003u64 & mask;

        assert_ne!(yield_masked, wfe_masked);
        assert_ne!(yield_masked, wfi_masked);
        assert_ne!(wfe_masked, wfi_masked);
    }

    // ===== Test 7: No-Overlap Group Match Generation =====

    #[test]
    fn test_no_overlap_match_generation() {
        let mut buffer = create_test_buffer();
        let mut formats = HashMap::new();
        let mut args_map = HashMap::new();

        formats.insert(
            "msr_i".to_string(),
            Format {
                name: "msr_i".to_string(),
                base: "msr_i".to_string(),
                fields: HashMap::new(),
                fixedbits: 0,
                fixedmask: 0,
            },
        );

        args_map.insert(
            "msr_i".to_string(),
            Arguments {
                name: "msr_i".to_string(),
                fields: vec![], // Empty for simplicity
                is_extern: false,
            },
        );

        let mut group = PatternGroup::new(GroupType::NoOverlap, 1);

        group.add_pattern(Pattern {
            name: "YIELD".to_string(),
            format: "msr_i".to_string(),
            fixedbits: 0x0320f001,
            fixedmask: 0x0fffffff,
        });

        group.add_pattern(Pattern {
            name: "WFE".to_string(),
            format: "msr_i".to_string(),
            fixedbits: 0x0320f002,
            fixedmask: 0x0fffffff,
        });

        // Generate decoder
        group
            .generate_decoder(&mut buffer, "inst", &formats)
            .unwrap();

        let output = buffer_to_string(&buffer);

        // Should have match statement
        assert!(output.contains("match inst &"));

        // Should have unique match arms
        assert!(output.contains("0x0320f001"));
        assert!(output.contains("0x0320f002"));

        // Should NOT have duplicate values
        let lines: Vec<&str> = output.lines().collect();
        let mut match_values = Vec::new();

        for line in lines {
            if line.trim().starts_with("0x") && line.contains("=>") {
                let value = line.trim().split_whitespace().next().unwrap();
                match_values.push(value);
            }
        }

        // Check for duplicates
        let mut sorted = match_values.clone();
        sorted.sort();
        sorted.dedup();

        assert_eq!(
            match_values.len(),
            sorted.len(),
            "Found duplicate match values: {:?}",
            match_values
        );
    }

    // ===== Test 8: Pattern Collection from Nested Groups =====

    #[test]
    fn test_pattern_collection_nested() {
        let mut root = PatternGroup::new(GroupType::Overlap, 0);
        let mut sub1 = PatternGroup::new(GroupType::NoOverlap, 1);
        let mut sub2 = PatternGroup::new(GroupType::Overlap, 2);

        root.add_pattern(Pattern {
            name: "A".to_string(),
            format: "fmt_a".to_string(),
            fixedbits: 0,
            fixedmask: 0,
        });

        sub1.add_pattern(Pattern {
            name: "B".to_string(),
            format: "fmt_b".to_string(),
            fixedbits: 0,
            fixedmask: 0,
        });

        sub2.add_pattern(Pattern {
            name: "C".to_string(),
            format: "fmt_c".to_string(),
            fixedbits: 0,
            fixedmask: 0,
        });

        sub1.add_subgroup(sub2);
        root.add_subgroup(sub1);

        // Collect all names
        let mut names = HashSet::new();
        root.collect_all_pattern_names(&mut names);

        assert_eq!(names.len(), 3);
        assert!(names.contains("A"));
        assert!(names.contains("B"));
        assert!(names.contains("C"));
    }

    // ===== Test 9: Find Pattern in Nested Groups =====

    #[test]
    fn test_find_pattern_nested() {
        let mut root = PatternGroup::new(GroupType::Overlap, 0);
        let mut sub = PatternGroup::new(GroupType::NoOverlap, 1);

        let pattern_a = Pattern {
            name: "A".to_string(),
            format: "fmt_a".to_string(),
            fixedbits: 0x12345678,
            fixedmask: 0xffffffff,
        };

        let pattern_b = Pattern {
            name: "B".to_string(),
            format: "fmt_b".to_string(),
            fixedbits: 0x87654321,
            fixedmask: 0xffffffff,
        };

        root.add_pattern(pattern_a.clone());
        sub.add_pattern(pattern_b.clone());
        root.add_subgroup(sub);

        // Find patterns
        let found_a = root.find_pattern("A");
        let found_b = root.find_pattern("B");
        let found_c = root.find_pattern("C");

        assert!(found_a.is_some());
        assert_eq!(found_a.unwrap().fixedbits, 0x12345678);

        assert!(found_b.is_some());
        assert_eq!(found_b.unwrap().fixedbits, 0x87654321);

        assert!(found_c.is_none());
    }

    // ===== Test 10: Integration Test =====

    #[test]
    fn test_full_integration() {
        let mut buffer = create_test_buffer();
        let mut formats = HashMap::new();
        let mut args_map = HashMap::new();

        // Setup ERET (no args)
        formats.insert(
            "eret".to_string(),
            Format {
                name: "eret".to_string(),
                base: "eret".to_string(),
                fields: HashMap::new(),
                fixedbits: 0,
                fixedmask: 0,
            },
        );

        args_map.insert(
            "eret".to_string(),
            Arguments {
                name: "eret".to_string(),
                fields: vec![],
                is_extern: false,
            },
        );

        // Setup MSR_i (has args)
        formats.insert(
            "msr_i".to_string(),
            Format {
                name: "msr_i".to_string(),
                base: "msr_i".to_string(),
                fields: HashMap::from([(
                    "mask".to_string(),
                    FieldType::Const(ConstField { value: 0, mask: 0 }),
                )]),
                fixedbits: 0,
                fixedmask: 0,
            },
        );

        args_map.insert(
            "msr_i".to_string(),
            Arguments {
                name: "msr_i".to_string(),
                fields: vec![
                    ("mask".to_string(), "u32".to_string()),
                    ("imm".to_string(), "u32".to_string()),
                ],
                is_extern: false,
            },
        );

        // Create nested group structure
        let mut root = PatternGroup::new(GroupType::Overlap, 0);
        let mut hints = PatternGroup::new(GroupType::NoOverlap, 1);

        hints.add_pattern(Pattern {
            name: "YIELD".to_string(),
            format: "msr_i".to_string(),
            fixedbits: 0x0320f001,
            fixedmask: 0x0fffffff,
        });

        hints.add_pattern(Pattern {
            name: "WFE".to_string(),
            format: "msr_i".to_string(),
            fixedbits: 0x0320f002,
            fixedmask: 0x0fffffff,
        });

        root.add_subgroup(hints);

        root.add_pattern(Pattern {
            name: "ERET".to_string(),
            format: "eret".to_string(),
            fixedbits: 0x0160006e,
            fixedmask: 0x0fffffff,
        });

        root.add_pattern(Pattern {
            name: "MSR_imm".to_string(),
            format: "msr_i".to_string(),
            fixedbits: 0x0320f000,
            fixedmask: 0x0fb0f000,
        });

        // Generate enum
        let mut enum_buffer = create_test_buffer();
        root.output_instruction_variant(&mut enum_buffer, &formats)
            .unwrap();
        let enum_output = buffer_to_string(&enum_buffer);

        // Check enum
        assert!(enum_output.contains("  ERET ,")); // No args
        assert!(enum_output.contains("YIELD { args: arg_msr_i }")); // Has args
        assert!(enum_output.contains("WFE { args: arg_msr_i }"));
        assert!(enum_output.contains("MSR_imm { args: arg_msr_i }"));

        // Generate decoder
        root.generate_decoder(&mut buffer, "inst", &formats)
            .unwrap();
        let decoder_output = buffer_to_string(&buffer);

        // Check decoder has correct structure
        assert!(decoder_output.contains("// No-overlap group"));
        assert!(decoder_output.contains("match inst &"));
        assert!(decoder_output.contains("return Some(Instruction::ERET)")); // No args
        assert!(decoder_output.contains("let args = extract_msr_i(inst)")); // With args
    }
}
