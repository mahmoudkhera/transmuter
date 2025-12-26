use core::{
    armdecoder::DecodeContext, armmeme::SimpleMemory, armprocessor::interpreter::IRInterpreter,
    translate::armtranslator::ArmTranslator,
};

fn main() {
    const COMP_PROGRAM: &[u8] = &[
        0x0b, 0x00, 0x7c, 0xe1, // CMN R12, #1         (Imm)
    ];
    let mut memory = SimpleMemory::new(COMP_PROGRAM.len());

    memory.load_program(0, &COMP_PROGRAM);

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

    let mut inter = IRInterpreter::new();
    inter.cpu.write_reg(12, 0x7FFFFFFF);
    inter.cpu.write_reg(11, 0x1);

    inter.excute(&ir_program).unwrap();
}
