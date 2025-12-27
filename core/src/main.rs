use core::{
    armdecoder::DecodeContext, armmeme::SimpleMemory, armprocessor::interpreter::IRInterpreter,
    translate::armtranslator::ArmTranslator,
};

fn main() {
    const COMP_PROGRAM: &[u8] = &[ 0x04, 0x60, 0x55, 0xe0, ];
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
    inter.cpu.cpsr.set_nzcv(false, false, true, false);

    inter.cpu.write_reg(5, 5);
    inter.cpu.write_reg(4, 5);

    inter.excute(&ir_program).unwrap();
}
