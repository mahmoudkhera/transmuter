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

    fn run_interpreter(program: &[u8], init_regs: Vec<(usize, u32)>, carry: bool) -> IRInterpreter {
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
        inter.cpu.cpsr.set_nzcv(false, false, carry, false);

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

        let inter = run_interpreter(ANDS_PROGRAM, init, false);

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
            (9, 0x1),     // R8, R9: For Shifted ORR
            (10, 0xAAAA), // R10: For Self-EOR
        ];

        let inter = run_interpreter(LOGICAL_PROGRAM, init, false);

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

        assert!(n, "should be 1")
    }

    const ARITH_PROGRAM: &[u8] = &[
        0x01, 0x00, 0x91, 0xE2, // 0: ADDS R0, R1, #1
        0x04, 0x20, 0xB3, 0xE0, // 1: ADCS R2, R3, R4
        0x87, 0x51, 0x56, 0xE0, // 2: SUBS R5, R6, R7, LSL #3
        0x0A, 0x80, 0xD9, 0xE2, // 3: SBCS R8, R9, #10
        0x00, 0xA0, 0x7B, 0xE2, // 4: RSBS R10, R11, #0
        0x02, 0xC0, 0xF1, 0xE0, // 5: RSCS R12, R1, R2
    ];
    #[test]
    fn test_arithmetic_logic_and_flags() {
        let init = vec![
            (1, 0x7FFFFFFF), // R1: Max Positive (To trigger Overflow on ADD)
            (3, 10),
            (4, 20), // R3, R4: For ADC
            (6, 100),
            (7, 5),  // R6 - (5 << 2) = 100 - 20 = 80
            (9, 20), // R9: For SBC
            (11, 5), // R11: For RSB (0 - 5 = -5)
        ];

        // Start with Carry = 1 (No Borrow)
        let inter = run_interpreter(ARITH_PROGRAM, init, false);
        let (n, _, _, _) = inter.cpu.cpsr.get_nzcv();

        // --- ADDS (Immediate) ---
        // 0x7FFFFFFF + 1 = 0x80000000.
        // Result is Negative, and Pos+Pos=Neg triggers Overflow (V).
        assert_eq!(inter.cpu.read_reg(0), 0x80000000);

        // --- ADCS (Register) ---
        // 10 + 20 + Carry(0 from prev ADDS unsigned overflow) = 30
        // Note: ADDS 0x7FFFFFFF+1 does NOT carry out of bit 31, so C=0.
        assert_eq!(inter.cpu.read_reg(2), 30);

        // --- SUBS (Shifted Register) ---
        // 100 - (5 << 2) = 80.
        // 80 is positive, and 100 >= 20, so C=1 (No borrow).
        assert_eq!(inter.cpu.read_reg(5), 60);

        // --- SBCS (Immediate) ---
        // 20 - 10 - (1 - 1) = 10.
        assert_eq!(inter.cpu.read_reg(8), 10);

        // --- RSBS (Immediate) ---
        // 0 - 5 = -5 (0xFFFFFFFB)
        assert_eq!(inter.cpu.read_reg(10), 0xFFFFFFFB);
        assert!(n, "n must 1");
    }

    //

    const COMP_PROGRAM: &[u8] = &[
        // --- TST (Test bits: AND logic) ---
        0x02, 0x01, 0x11, 0xE3, // TST R1, #0x80000000 (Imm)
        // --- TEQ (Test Equivalence: EOR logic) ---
        0x08, 0x00, 0x37, 0xE1, // TEQ R7, R8          (Reg)
        // --- CMP (Compare: SUB logic) ---
        0x0A, 0x00, 0x59, 0xE3, // CMP R9, #10         (Imm)
        0x0b, 0x00, 0x7c, 0xe1, // CMP R12,R11
    ];

    #[test]
    fn test_tst_instructions() {
        let init = vec![
            (1, 0x80000000), // R1: Has MSB set
        ];
        let inter = run_interpreter(&COMP_PROGRAM[0..4], init, false);

        let (n, _, _, _) = inter.cpu.cpsr.get_nzcv();

        // Case 1: R1 & 0x80000000 -> Result 0x80000000
        assert!(n, "TST Imm failed to set N flag");
        assert_eq!(inter.cpu.read_reg(1), 0x80000000, "TST must not modify Rn");
    }

    #[test]
    fn test_teq_instructions() {
        let init = vec![
            // R7 ^ R8 -> Result 0xFFFFFFFF
            (7, 0x55555555),
            (8, 0x55555555),
        ];
        let inter = run_interpreter(&COMP_PROGRAM[4..8], init, false);

        let (_, z, _, _) = inter.cpu.cpsr.get_nzcv();
        assert!(z, "TEQ Imm failed to set Z flag");
    }
    #[test]
    fn test_cmp_instructions() {
        let init = vec![(9, 0x1)];

        let inter = run_interpreter(&COMP_PROGRAM[8..12], init, false);

        let (n, _, _, _) = inter.cpu.cpsr.get_nzcv();
        assert!(n, " CMP Imm failed to set v flag");
    }
    #[test]

    fn test_cmn_instructions() {
        let init = vec![(12, 0x7FFFFFFF), (11, 0x1)];

        let inter = run_interpreter(&COMP_PROGRAM[12..16], init, false);

        let (n, _, _, v) = inter.cpu.cpsr.get_nzcv();
        assert!(n, " CMN Imm failed to set v flag");
        assert!(v, " CMN Imm failed to set v flag");
    }
    const ARTHIMETIC: &[u8] = &[
        0x02, 0x30, 0x91, 0xe0, // ADDS r3,r1,r2
        0x02, 0x40, 0xb3, 0xe0, //  ADCs r4,r3,r2
        0x03, 0x50, 0xd4, 0xe0, // SBCS r5,r4,r3
        0x04, 0x60, 0x55, 0xe0, //SUB r6,r5,r4  note that rsb and rsc are just the same
    ];

    #[test]
    fn test_adds_instructions() {
        let init = vec![(1, 0x7FFFFFFF), (2, 0x1)];

        let inter = run_interpreter(&ARTHIMETIC[0..4], init, false);

        let (n, _, _, v) = inter.cpu.cpsr.get_nzcv();
        assert!(n, "n must be set ");
        assert!(v, " CMN Imm failed to set v flag");
    }

    #[test]
    fn test_adcs_instructions() {
        let init = vec![(3, 0x80000000), (2, 0x80000000)];

        let inter = run_interpreter(&ARTHIMETIC[4..8], init, true);

        // assert_eq!(inter.cpu.read_reg(4), 0x1);

        let (_, _, c, v) = inter.cpu.cpsr.get_nzcv();
        assert!(c, "  c must be set");
        assert!(v, "  v must be set");
    }
    #[test]
    fn test_subcs_instructions() {
        let init = vec![(4, 0), (3, 0x80000000)];

        let inter = run_interpreter(&ARTHIMETIC[8..12], init, false);

        let (_, _, _, v) = inter.cpu.cpsr.get_nzcv();
        assert!(v, "  v must be set");
    }
    #[test]
    fn test_subs_instructions() {
        let init = vec![(4, 0x1), (5, 0x0)];

        let inter = run_interpreter(&ARTHIMETIC[12..16], init, true);

        assert_eq!(inter.cpu.read_reg(6), 0xFFFFFFFF);

        let (n, _, c, _) = inter.cpu.cpsr.get_nzcv();
        assert!(n, "  n must be set");
        assert!(!c, "  c must  reset");
    }
}
