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
