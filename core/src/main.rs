use core::{
    armdecoder::DecodeContext, armmeme::SimpleMemory, armprocessor::interpreter::IRInterpreter,
    translate::armtranslator::ArmTranslator,
};

fn main() {
    let program = [
        0x00, 0x20, 0xE0, 0xE3, // mvn r2 0xFFFF_FFFF
        0x01, 0x00, 0x92, 0xE2, //add r2, r2, #1, S=1 must update the  carry flag
    ];

    let mut memory = SimpleMemory::new(8);

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

    for (_, block) in ir_program.blocks.iter() {
        println!("block {:?}", block);
    }

    let mut inter = IRInterpreter::new();
    inter.excute(&ir_program).unwrap();
}
