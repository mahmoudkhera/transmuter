use core::{
    armdecoder::DecodeContext, armmeme::SimpleMemory, armprocessor::interpreter::IRInterpreter,
    translate::armtranslator::ArmTranslator,
};

fn main() {
    // A collection of ARM32 instructions in Little-Endian format
    let program: Vec<u8> = vec![
        // --- LOGICAL OPERATIONS ---
        0x03, 0x00, 0x01, 0xE0, // AND r0, r1, r3        (Reg: r0 = r1 & r3)
        0x03, 0x10, 0x21, 0xE0, // EOR r1, r1, r3        (Reg: r1 = r1 ^ r3)
        0xFF, 0x20, 0x81, 0xE3, // ORR r2, r1, #0xFF     (Imm: r2 = r1 | 255)
        0x0F, 0x30, 0xC2, 0xE3, // BIC r3, r2, #0x0F     (Imm: r3 = r2 & ~15)
        // --- ARITHMETIC OPERATIONS ---
        0x04, 0x40, 0x83, 0xE0, // ADD r4, r3, r4        (Reg: r4 = r3 + r4)
        0x05, 0x50, 0x44, 0xE0, // SUB r5, r4, r5        (Reg: r5 = r4 - r5)
        0x01, 0x60, 0x75, 0xE2, // RSB r6, r5, #1        (Imm: r6 = 1 - r5)
        0x00, 0x70, 0xA0, 0xE3, // MOV r7, #0            (Imm: r7 = 0)
        0x07, 0x80, 0xF0, 0xE1, // MVN r8, r7            (Reg: r8 = ~r7)
        // --- WITH CARRY/FLAGS (S=1) ---
        0x02, 0x90, 0x91, 0xE0, // ADDS r9, r1, r2       (Update Flags)
        0x04, 0xA0, 0xB1, 0xE0, // ADCS r10, r1, r4      (Add with Carry)
        0x04, 0xB0, 0xD1, 0xE0, // SBCS r11, r1, r4      (Sub with Carry)
        // --- COMPARISONS (Flags only, Rd is 0) ---
        0x04, 0x00, 0x11, 0xE1, // TST r1, r4            (Test bits)
        0x04, 0x00, 0x31, 0xE1, // TEQ r1, r4            (Test equivalence)
        0x0A, 0x00, 0x51, 0xE3, // CMP r1, #10           (Compare)
        0x00, 0x00, 0x71, 0xE3, // CMN r1, #0            (Compare negative)
    ];

    let mut memory = SimpleMemory::new(program.len());

    memory.load_program(0, &program);

    let mut dectx = DecodeContext::new(0);

    let mut arm_insts = Vec::new();

    while let Some(arm_inst) = dectx.get_arm_inst(&memory) {
        arm_insts.push(arm_inst);
    }

    println!("arm inst len {}", arm_insts.len());

    let mut arm_translator = ArmTranslator::new(arm_insts);

    arm_translator.translate_instructions().unwrap();

    let ir_program = arm_translator.finalize();

    println!("ir entry {}", ir_program.entry);

    // for (_, block) in ir_program.blocks.iter() {
    //     println!("block {:?}", block);
    // }

    let mut inter = IRInterpreter::new();
    inter.excute(&ir_program).unwrap();
}
