use core::{
    armdecoder::DecodeContext, armmeme::SimpleMemory, translate::armtranslator::ArmTranslator,
};

fn main() {
    let program = [
        0x00, 0x00, 0x81, 0xE0, // ADD R0, R1, R2
    ];

    let mut memory = SimpleMemory::new(4);

    memory.load_program(0, &program);

    let mut dectx = DecodeContext::new(0);

    let mut arm_insts = Vec::new();

    while let Some(arm_inst) = dectx.get_arm_inst(&memory) {
        arm_insts.push(arm_inst);
    }

    println!("arm inst len {}",arm_insts.len());

    let mut arm_translator = ArmTranslator::new(arm_insts);

    arm_translator.translate_instructions().unwrap();

    let ir_program = arm_translator.finalize();

    println!("ir entry {}", ir_program.entry);

    for block in ir_program.blocks.iter(){
        println!("ir blocks {:?} ", block);
    }
    
}
