/// # MemoryInterface Trait
///
/// Defines the essential read and write operations for interacting with
/// the CPU's memory space. All memory implementations (RAM, ROM, MMIO)
/// should implement this trait.
pub trait MemoryInterface {
    /// Reads a 32-bit (4-byte) word from the specified memory address.
    /// The ARM architecture typically uses word-aligned accesses.
    ///
    /// Returns `Result<u32, String>`: The data read, or an error message
    /// if the access is invalid (e.g., out of bounds, unaligned).
    fn read_u32(&self, addr: u32) -> Result<u32, String>;

    /// Reads a 16-bit (2-byte) half-word from the specified memory address.
    fn read_u16(&self, addr: u32) -> Result<u16, String>;

    /// Writes a 32-bit (4-byte) word to the specified memory address.
    ///
    /// Returns `Result<(), String>`: Success, or an error message if the
    /// write is invalid (e.g., trying to write to ROM).
    fn write_u32(&mut self, addr: u32, value: u32) -> Result<(), String>;

    /// Writes a 16-bit (2-byte) half-word to the specified memory address.
    fn write_u16(&mut self, addr: u32, value: u16) -> Result<(), String>;

    // NOTE: For completeness, read_u8 and write_u8 should also be added.
    // We omit them here for brevity and focus on the provided methods.
}

///
/// A basic, contiguous block of RAM (Read-Write Memory) implemented using
/// a standard Rust `Vec<u8>`. This is a foundation for a simulated memory system.
pub struct SimpleMemory {
    /// The underlying storage for the memory data.
    data: Vec<u8>,
}

impl SimpleMemory {
    /// Creates a new SimpleMemory instance initialized to a given size (in bytes).
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![0; size], // Initialize all bytes to zero
        }
    }

    /// Loads a byte array (e.g., a compiled program or firmware) into memory
    /// starting at the specified address.
    pub fn load_program(&mut self, addr: u32, program: &[u8]) {
        let start = addr as usize;
        let end = start + program.len();

        // Safety check is omitted here, assuming the caller ensures the program
        // fits. In production, this should include bounds checking.
        if end > self.data.len() {
            // A proper implementation might panic or return an error here
            panic!(
                "Program load out of bounds: Address 0x{:X} too large for memory size 0x{:X}",
                addr,
                self.data.len()
            );
        }

        self.data[start..end].copy_from_slice(program);
    }
}

impl MemoryInterface for SimpleMemory {
    /// Reads a 32-bit word, respecting ARM's typical Little-Endian byte order.
    fn read_u32(&self, addr: u32) -> Result<u32, String> {
        let addr = addr as usize;

        // 1. Bounds Check: Ensure the access is within the allocated memory block
        if addr + 4 > self.data.len() {
            return Err(format!("Memory read out of bounds: 0x{:08X}", addr));
        }
        println!("read u32 ");

        // 2. Read bytes and assemble them in Little-Endian (LE) order.
        // ARM CPUs can be configured for Big-Endian, but Little-Endian is the default.
        Ok(u32::from_le_bytes([
            self.data[addr],     // Byte 0 (Least Significant)
            self.data[addr + 1], // Byte 1
            self.data[addr + 2], // Byte 2
            self.data[addr + 3], // Byte 3 (Most Significant)
        ]))
    }

    /// Reads a 16-bit half-word, respecting Little-Endian byte order.
    fn read_u16(&self, addr: u32) -> Result<u16, String> {
        let addr = addr as usize;

        // 1. Bounds Check
        if addr + 2 > self.data.len() {
            return Err(format!("Memory read out of bounds: 0x{:08X}", addr));
        }

        // 2. Read bytes and assemble them in Little-Endian (LE) order.
        Ok(u16::from_be_bytes([self.data[addr], self.data[addr + 1]]))
    }

    /// Writes a 32-bit word, converting the value to bytes in Little-Endian order.
    fn write_u32(&mut self, addr: u32, value: u32) -> Result<(), String> {
        let addr = addr as usize;

        // 1. Bounds Check
        if addr + 4 > self.data.len() {
            return Err(format!("Memory write out of bounds: 0x{:08X}", addr));
        }

        // 2. Convert the u32 value into a 4-byte array (Little-Endian)
        let bytes = value.to_le_bytes();

        // 3. Copy the byte slice into the memory vector
        self.data[addr..addr + 4].copy_from_slice(&bytes);

        Ok(())
    }

    /// Writes a 16-bit half-word, converting the value to bytes in Little-Endian order.
    fn write_u16(&mut self, addr: u32, value: u16) -> Result<(), String> {
        let addr = addr as usize;

        // 1. Bounds Check
        if addr + 2 > self.data.len() {
            return Err(format!("Memory write out of bounds: 0x{:08X}", addr));
        }

        // 2. Convert the u16 value into a 2-byte array (Little-Endian)
        let bytes = value.to_le_bytes();

        // 3. Copy the byte slice into the memory vector
        self.data[addr..addr + 2].copy_from_slice(&bytes);

        Ok(())
    }
}
