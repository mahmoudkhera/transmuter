use core::{
    armdecoder::DecodeContext, armmeme::SimpleMemory, armprocessor::interpreter::IRInterpreter,
    translate::armtranslator::ArmTranslator,
};

fn main() {
    let program: Vec<u8> = vec![
        // ADD r0, r1, #1, S=1 (immediate, rotate=0)
        0x01, 0x00, 0x91, 0xE2, // r0 = r1 + 1
        // SUB r2, r0, #2, S=1 (immediate, rotate=0)
        0x02, 0x20, 0x50, 0xE2, // r2 = r0 - 2  022052E2,
        // MOV r3, #0xFF, S=1 (immediate, rotate=0)
        0xFF, 0x30, 0xB0, 0xE3, // r3 = 0xFF
        // MVN r4, #0x0F, S=1 (immediate, rotate=0)
        0x0F, 0x40, 0xF0, 0xE3, // r4 = ~0x0F
        // AND r5, r3, r4, S=1, no shift
        0x03, 0x50, 0x04, 0xE0, // r5 = r3 & r4
        // ORR r6, r3, r4, S=1, no shift
        0x04, 0x60, 0x93, 0xE1, // r6 = r3 | r4
        // EOR r7, r3, r4, S=1, no shift
        0x04, 0x70, 0x33, 0xE0, // r7 = r3 ^ r4
        // BIC r8, r3, r4, S=1, no shift
        0x04, 0x80, 0xD3, 0xE1, // r8 = r3 & ~r4
        0x04, 0x00, 0x13, 0xE1, // TST r3, r4 (flags only)
        0x04, 0x00, 0x33, 0xE1, // TEQ r3, r4 (flags only)
        0x04, 0x00, 0x53, 0xE1, // CMP r3, r4 (flags only)
        0x04, 0x00, 0x73, 0xE1, // CMN r3, r4 (flags only)
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
