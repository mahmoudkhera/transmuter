use core::{
    arm32arch::{Instruction, decode_instruction},
    armmeme::{MemoryInterface, SimpleMemory},
};

fn main() {
    let program = [
        0x02, 0x00, 0x81, 0xE0, // ADD R0, R1, R2
        0x02, 0x00, 0x50, 0xE1, // CMP R0, R2
        0x02, 0x30, 0x40, 0xC0, // SUBGT R3, R0, R2
    ];

    let mut memory = SimpleMemory::new(20);

    memory.load_program(0, &program);

    let raw_insts = get_insts(&mut memory);

    println!(" inst  {:?}", raw_insts);
}

fn get_insts(meme: &mut SimpleMemory) -> Vec<Instruction> {
    let mut inst = Vec::new();

    let mut addr = 0;

    for _ in 0..3 {
        let mem_read = meme.read_u32(addr).unwrap();
        println!(" inst {:0b}", mem_read);
        let decode_inst = decode_instruction(mem_read).unwrap();
        addr += 4;

        inst.push(decode_inst);
    }

    inst
}
