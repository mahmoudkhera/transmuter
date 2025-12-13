use core::{
    arm32arch::{Instruction, decode_instruction},
    armmeme::{MemoryInterface, SimpleMemory},
};

fn main() {
    let instructions: Vec<u8> = vec![
        // LDR R1, [R0]
        0b11100101, 0b10000000, 0b00000000, 0b00000001, // ADD R1, R1, #3
        0b11100010, 0b10000001, 0b00000000, 0b00000011, // STR R1, [R0]
       0b11100101, 0, 0, 1,
    ];

    let mut memory = SimpleMemory::new(20);

    memory.load_program(0, &instructions);

    let raw_insts = get_insts(&mut memory);

    println!(" inst  {:?}", raw_insts);
}

fn get_insts(meme: &mut SimpleMemory) -> Vec<Instruction> {
    let mut inst = Vec::new();

    let mut addr = 0;

    for _ in 0..3 {

        let mem_read=meme.read_u32(addr).unwrap();
        println!(" inst {:0b}",mem_read);
        let decode_inst = decode_instruction(mem_read).unwrap();
        addr += 4;

        inst.push(decode_inst);
    }

    inst
}
