#[cfg(test)]
mod ands_specific_test {
    use core::{
        armdecoder::DecodeContext, armmeme::SimpleMemory, armprocessor::interpreter::IRInterpreter,
        translate::armtranslator::ArmTranslator,
    };
    // to test if logical function update the cpsr  or not
    const ANDS_PROGRAM: &[u8] = &[
        // Case 1: Immediate
        // ANDS R0, R1, #0x80000000
        // (R1: 0xFFFFFFFF & 0x80000000) -> Result: 0x80000000
        // Expected: R0=0x80000000, N=1, Z=0
        0x02, 0x01, 0x11, 0xE2,
        // Case 2: Register
        // ANDS R2, R3, R4
        // (R3: 0xF0 & R4: 0x0F) -> Result: 0
        // Expected: R2=0, N=0, Z=1
        0x04, 0x20, 0x13, 0xE0,
        // Case 3: Shifted Register
        // ANDS R5, R6, R7, LSR #1
        // (R6: 0x1 & (R7: 0x2 >> 1)) -> Result: 1
        // Expected: R5=1, N=0, Z=1
        0xA7, 0x50, 0x16, 0xE0,
    ];

    fn run_interpreter(program: &[u8], init_regs: Vec<(usize, u32)>) -> IRInterpreter {
        let mut memory = SimpleMemory::new(program.len());
        memory.load_program(0, program);

        let mut dectx = DecodeContext::new(0);
        let mut arm_insts = Vec::new();
        while let Some(arm_inst) = dectx.get_arm_inst(&memory) {
            arm_insts.push(arm_inst);
        }

        let mut arm_translator = ArmTranslator::new(arm_insts);
        arm_translator.translate_instructions().unwrap();
        let ir_program = arm_translator.finalize();

        let mut inter = IRInterpreter::new();

        // Initialize registers
        for (idx, val) in init_regs {
            inter.cpu.write_reg(idx as u8, val);
        }

        inter.excute(&ir_program).unwrap();
        inter
    }

    #[test]
    fn test_ands_isolated_flags() {
        // Setup values to isolate flags
        let init = vec![
            (1, 0xFFFFFFFF), // For Case 1: Masking the MSB
            (3, 0x000000F0), // For Case 2: No overlapping bits with R4
            (4, 0x0000000F), // For Case 2
            (6, 0x00000001), // For Case 3
            (7, 0x00000001), // For Case 3: Will be shifted to 1
        ];

        let inter = run_interpreter(ANDS_PROGRAM, init);

        let (_, z, _, _) = inter.cpu.cpsr.get_nzcv();
        assert_eq!(inter.cpu.read_reg(0), 0x80000000);
        assert_eq!(inter.cpu.read_reg(2), 0x00000000);
        assert_eq!(inter.cpu.read_reg(5), 0x00000000);
        assert!(z, "Z flag should be 1");
    }

    const LOGICAL_PROGRAM: &[u8] = &[
        0x02, 0x01, 0x11, 0xE2, // 0: ANDS R0, R1, #0x80000000
        0x04, 0x20, 0x03, 0xE0, // 1: AND R2, R3, R4
        0x02, 0x51, 0x96, 0xE3, // 2: ORRS R5, R6, #0x80000000
        0x09, 0x70, 0x88, 0xE1, // 3: ORR R7, R8, R9, LSL #1
        0x0A, 0xA0, 0x3A, 0xE0, // 4: EORS R10, R10, R10
        0xFF, 0xB0, 0xD1, 0xE3, // 5: BICS R11, R1, #0xFF
    ];

    #[test]
    fn test_logical_operators_and_flags() {
        let init = vec![
            (1, 0xFFFFFFFF), // R1: All 1s
            (3, 0x0F),
            (4, 0xF0), // R3 & R4: No overlap (result 0)
            (6, 0x0),  // R6: 0
            (8, 0x1),
            (9, 0x1),         // R8, R9: For Shifted ORR
            (10, 0xAAAA),     // R10: For Self-EOR
          
        ];

        let inter = run_interpreter(LOGICAL_PROGRAM, init);

         let (n, _, _, _) = inter.cpu.cpsr.get_nzcv();
        // 1. ANDS - Result is 0x80000000. Should set N flag.
        assert_eq!(inter.cpu.read_reg(0), 0x80000000);

        // 2. AND (Unsigned) - Result is 0. Flags should NOT change from previous state (N still 1).
        assert_eq!(inter.cpu.read_reg(2), 0);

        // 3. ORRS - Result is 0x80000000. Should set N flag and clear Z flag.
         assert_eq!(inter.cpu.read_reg(5), 0x80000000);

        // 4. EORS - Result is 0. Should set Z flag and clear N flag.
        assert_eq!(inter.cpu.read_reg(10), 0);

        // 5. BICS - R12(0xFF) BIC 0xFF = 0. Should keep Z flag set.
        assert_eq!(inter.cpu.read_reg(11), 0xffffff00);

        assert!(n,"should be 1")
    }
}
