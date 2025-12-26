use core::{
    armdecoder::DecodeContext, armmeme::SimpleMemory, armprocessor::interpreter::IRInterpreter,
    translate::armtranslator::ArmTranslator,
};

fn main() {
       const LOGICAL_PROGRAM: &[u8] = &[
        0x02, 0x01, 0x11, 0xE2, // 0: ANDS R0, R1, #0x80000000
        0x04, 0x20, 0x03, 0xE0, // 1: AND R2, R3, R4
        0x02, 0x51, 0x96, 0xE3, // 2: ORRS R5, R6, #0x80000000
        0x09, 0x70, 0x88, 0xE1, // 3: ORR R7, R8, R9, LSL #1
        0x0A, 0xA0, 0x3A, 0xE0, // 4: EORS R10, R10, R10
        0xFF, 0xB0, 0xD1, 0xE3, // 5: BICS R11, R12, #0xFF
    ];
    let mut memory = SimpleMemory::new(LOGICAL_PROGRAM.len());

    memory.load_program(0, &LOGICAL_PROGRAM);

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
    inter.excute(&ir_program).unwrap();
}
