use core::{
    armdecoder::DecodeContext, armmeme::SimpleMemory, armprocessor::interpreter::IRInterpreter,
    translate::armtranslator::ArmTranslator,
};

fn main() {
    let program = [
        0x17, 0x20, 0xA0, 0xE3, // Mov R2, #2
    ];

    let mut memory = SimpleMemory::new(4);

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

    for (id, block) in ir_program.blocks.iter() {
        println!("block {:?}", block);
    }

    let mut inter = IRInterpreter::new();
    inter.excute(&ir_program).unwrap();
}
