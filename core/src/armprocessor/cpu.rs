use core::fmt;

/// ARM32 CPU core state (AArch32)
///
/// This struct models the architectural state of a single ARM CPU core,
/// including general registers, banked registers, status registers,
/// floating-point state, and exclusive monitors.

#[derive(Debug, Clone)]
pub struct CPUState {
    //  General-purpose registers
    /// R0–R15 (R15 = PC)
    regs: [u32; 16],

    //   Banked registers (privileged modes)
    /// FIQ mode: R8–R14
    pub regs_fiq: [u32; 7],

    /// SVC, ABT, UND, IRQ: R13–R14
    pub regs_svc: [u32; 2],
    pub regs_abt: [u32; 2],
    pub regs_und: [u32; 2],
    pub regs_irq: [u32; 2],

    //  * Status registers
    /// Current Program Status Register
    pub cpsr: CPSR,

    /// Saved Program Status Registers
    pub spsr_fiq: u32,
    pub spsr_svc: u32,
    pub spsr_abt: u32,
    pub spsr_und: u32,
    pub spsr_irq: u32,

    //  Floating-point / SIMD
    /// VFP / NEON registers (D0–D31)
    pub vfp_regs: Option<[u64; 32]>,

    //  * Exclusive monitor (LDREX/STREX)
    pub exclusive_addr: u32,
    pub exclusive_val: u32,
}

impl CPUState {
    /// Create a CPU in reset-like state
    pub fn new() -> Self {
        Self {
            regs: [0; 16],

            regs_fiq: [0; 7],
            regs_svc: [0; 2],
            regs_abt: [0; 2],
            regs_und: [0; 2],
            regs_irq: [0; 2],

            // SVC mode, IRQ/FIQ disabled
            cpsr: CPSR::new(0x0000_00D3),

            spsr_fiq: 0,
            spsr_svc: 0,
            spsr_abt: 0,
            spsr_und: 0,
            spsr_irq: 0,

            vfp_regs: Some([0; 32]),

            exclusive_addr: u32::MAX,
            exclusive_val: 0,
        }
    }

    //  General register access

    /// Read a general-purpose register (R0–R15)
    pub fn read_reg(&self, reg: u8) -> u32 {
        match reg {
            0..=14 => self.regs[reg as usize],
            15 => self.read_pc(),
            _ => panic!("Invalid register {}", reg),
        }
    }

    /// Write a general-purpose register (R0–R15)
    pub fn write_reg(&mut self, reg: u8, value: u32) {
        match reg {
            0..=14 => self.regs[reg as usize] = value,
            15 => self.write_pc(value),
            _ => panic!("Invalid register {}", reg),
        }
    }

    //  Special registers

    /// Read Program Counter (ARM state: PC + 8)
    pub fn read_pc(&self) -> u32 {
        self.regs[15].wrapping_add(8)
    }

    /// Write Program Counter (branch)
    pub fn write_pc(&mut self, value: u32) {
        // Clear Thumb bit; CPSR.T controls instruction set
        self.regs[15] = value & !1;
    }

    /// Read Stack Pointer (R13)
    pub fn read_sp(&self) -> u32 {
        self.regs[13]
    }

    /// Write Stack Pointer (R13)
    pub fn write_sp(&mut self, value: u32) {
        self.regs[13] = value;
    }

    /// Read Link Register (R14)
    pub fn read_lr(&self) -> u32 {
        self.regs[14]
    }

    /// Write Link Register (R14)
    pub fn write_lr(&mut self, value: u32) {
        self.regs[14] = value;
    }
}

/// ARM processor operating modes.
///
/// These values correspond to the CPSR mode bits `M[4:0]`.
/// The CPU switches modes automatically when exceptions
/// or interrupts occur.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ARMMode {
    /// User mode (unprivileged).
    ///
    /// - Used by normal applications
    /// - No access to system control registers
    /// - Cannot disable interrupts or change CPU mode
    User = 0b10000,

    /// Fast Interrupt Request (FIQ) mode.
    ///
    /// - High-priority interrupt handling
    /// - Extra banked registers (r8–r14) for fast context switching
    /// - Used for latency-critical tasks
    FIQ = 0b10001,

    /// Interrupt Request (IRQ) mode.
    ///
    /// - Handles standard hardware interrupts
    /// - Banked stack pointer (r13) and link register (r14)
    /// - Used for timers, I/O, peripherals
    IRQ = 0b10010,

    /// Supervisor (SVC) mode.
    ///
    /// - Entered via the `SVC` instruction (system calls)
    /// - Used by the operating system kernel
    /// - Also the mode after CPU reset
    Supervisor = 0b10011,

    /// Abort mode.
    ///
    /// - Entered on memory access faults
    /// - Handles data aborts and prefetch aborts
    /// - Typically used for MMU/page fault handling
    Abort = 0b10111,

    /// Undefined Instruction mode.
    ///
    /// - Triggered when an invalid or unsupported instruction is executed
    /// - Can be used for instruction emulation or debugging
    Undefined = 0b11011,

    /// System mode.
    ///
    /// - Privileged version of User mode
    /// - Uses the same registers as User mode
    /// - Often used by kernels instead of Supervisor mode
    System = 0b11111,
}

impl ARMMode {
    pub fn from_bits(bits: u32) -> Option<Self> {
        match bits & 0x1F {
            0b10000 => Some(ARMMode::User),
            0b10001 => Some(ARMMode::FIQ),
            0b10010 => Some(ARMMode::IRQ),
            0b10011 => Some(ARMMode::Supervisor),
            0b10111 => Some(ARMMode::Abort),
            0b11011 => Some(ARMMode::Undefined),
            0b11111 => Some(ARMMode::System),
            _ => None,
        }
    }

    pub fn is_privileged(&self) -> bool {
        !matches!(self, ARMMode::User)
    }
}

///  CPSR (Current Program Status Register) — ARM32
///
/// Holds the processor state, including:
/// - Condition flags (N, Z, C, V, Q)
/// - Thumb instruction state
/// - Interrupt masks (A/I/F)
/// - CPU mode
///
#[derive(Debug, Clone, Copy)]
pub struct CPSR {
    bits: u32,
}

impl CPSR {
    /// Creates a new CPSR from a raw 32-bit value
    pub fn new(bits: u32) -> Self {
        Self { bits }
    }

    // Condition flags

    /// Negative flag (N) — set if last operation was negative
    pub fn n(&self) -> bool {
        (self.bits >> 31) & 1 != 0
    }

    /// Zero flag (Z) — set if last operation was zero
    pub fn z(&self) -> bool {
        (self.bits >> 30) & 1 != 0
    }

    /// Carry flag (C) — set on unsigned overflow/borrow
    pub fn c(&self) -> bool {
        (self.bits >> 29) & 1 != 0
    }

    /// Overflow flag (V) — set on signed overflow
    pub fn v(&self) -> bool {
        (self.bits >> 28) & 1 != 0
    }

    /// Saturation flag (Q) — set if saturating arithmetic overflowed
    pub fn q(&self) -> bool {
        (self.bits >> 27) & 1 != 0
    }

    // Instruction & execution state

    /// Thumb state bit (T)
    /// - `false` = ARM mode (32-bit instructions)
    /// - `true`  = Thumb mode (16/32-bit instructions)
    pub fn t(&self) -> bool {
        (self.bits >> 5) & 1 != 0
    }

    // Interrupt and abort masks

    /// Asynchronous abort disable (A)
    pub fn a(&self) -> bool {
        (self.bits >> 8) & 1 != 0
    }

    /// IRQ interrupt disable (I)
    pub fn i(&self) -> bool {
        (self.bits >> 7) & 1 != 0
    }

    /// FIQ interrupt disable (F)
    pub fn f(&self) -> bool {
        (self.bits >> 6) & 1 != 0
    }

    // CPU mode

    /// Returns the current CPU mode as `ARMMode`
    pub fn mode(&self) -> ARMMode {
        ARMMode::from_bits(self.bits).unwrap_or(ARMMode::User)
    }

    // Flag mutation helper

    /// Sets the N, Z, C, and V flags
    ///
    /// Preserves all other CPSR bits (mode, interrupts, Thumb state)
    pub fn set_nzcv(&mut self, n: bool, z: bool, c: bool, v: bool) {
        // Clear existing NZCV bits
        self.bits &= 0x0FFF_FFFF;

        if n {
            self.bits |= 1 << 31;
        }
        if z {
            self.bits |= 1 << 30;
        }
        if c {
            self.bits |= 1 << 29;
        }
        if v {
            self.bits |= 1 << 28;
        }
    }
    /// Checks if the condition is met based on the current state of the
    /// N, Z, C, and V flags in the CPSR.
    pub fn evaluat_cond(&mut self, cond: Condition) -> bool {
        match cond {
            // Unsigned Comparisons
            Condition::EQ => self.z(),  // Z == 1
            Condition::NE => !self.z(), // Z == 0
            Condition::CS => self.c(),  // C == 1
            Condition::CC => !self.c(), // C == 0

            // Single Flag Checks
            Condition::MI => self.n(),  // N == 1
            Condition::PL => !self.n(), // N == 0
            Condition::VS => self.v(),  // V == 1
            Condition::VC => !self.v(), // V == 0

            // Complex Unsigned Comparisons
            Condition::HI => self.c() && !self.z(), // C == 1 AND Z == 0 (Higher)
            Condition::LS => !self.c() || self.z(), // C == 0 OR Z == 1 (Lower or Same)

            // Signed Comparisons
            Condition::GE => self.n() == self.v(), // N == V (Greater than or Equal)
            Condition::LT => self.n() != self.v(), // N != V (Less than)
            Condition::GT => !self.z() && (self.n() == self.v()), // Z == 0 AND N == V (Greater than)
            Condition::LE => self.z() || (self.n() != self.v()), // Z == 1 OR N != V (Less than or Equal)

            // Unconditional/Reserved
            Condition::AL => true,
            Condition::NV => false, // Always fails
        }
    }
}

/// Condition codes (Cond Field) from the ARM instruction format.
/// These codes determine if a conditional instruction should execute
/// based on the status of the N, Z, C, and V flags in the CPSR.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Condition {
    // Standard Condition Codes (0x0 - 0xD)
    EQ = 0x0, // Equal (Zero flag Z)
    NE = 0x1, // Not Equal (Zero flag Z)
    CS = 0x2, // Carry Set (Carry flag C)
    CC = 0x3, // Carry Clear (Carry flag C)
    MI = 0x4, // Minus/Negative (Negative flag N)
    PL = 0x5, // Plus/Positive (Negative flag N)
    VS = 0x6, // Overflow Set (Overflow flag V)
    VC = 0x7, // Overflow Clear (Overflow flag V)
    HI = 0x8, // Higher(Unsigned Comparison C and Z)
    LS = 0x9, //  Lower or Same (Unsigned Comparison C and Z)
    GE = 0xA, // Greater than or Equal (Signed Comparison N and V)
    LT = 0xB, // Less than (Signed Comparison N and V)
    GT = 0xC, // Greater than(Signed Comparison N, Z, and V)
    LE = 0xD, // Greater than (Signed Comparison N, Z, and V)

    // Unconditional/Reserved
    AL = 0xE, // Always (Unconditional execution)
    NV = 0xF, // Never (Deprecated/Reserved, usually treated as false)
}

impl Condition {
    /// Creates a Condition enum from the 4-bit condition field of a 32-bit instruction word.
    pub fn from_u32(val: u32) -> Self {
        // Mask to isolate the 4-bit condition code (bits 31:28 of the instruction)
        // If the 4-bit field is at the lowest bits, the mask should be `val & 0xF`.
        match (val & 0xF) as u8 {
            0x0 => Condition::EQ,
            0x1 => Condition::NE,
            0x2 => Condition::CS,
            0x3 => Condition::CC,
            0x4 => Condition::MI,
            0x5 => Condition::PL,
            0x6 => Condition::VS,
            0x7 => Condition::VC,
            0x8 => Condition::HI,
            0x9 => Condition::LS,
            0xA => Condition::GE,
            0xB => Condition::LT,
            0xC => Condition::GT,
            0xD => Condition::LE,
            0xE => Condition::AL,
            0xF => Condition::NV,

            // This case should be unreachable due to the `& 0xF` mask.
            _ => unreachable!(),
        }
    }

    /// Checks if the condition is met based on the current state of the
    /// N, Z, C, and V flags in the CPSR.
    pub fn passes(&self, cpsr: &CPSR) -> bool {
        match self {
            // Unsigned Comparisons
            Condition::EQ => cpsr.z(),  // Z == 1
            Condition::NE => !cpsr.z(), // Z == 0
            Condition::CS => cpsr.c(),  // C == 1
            Condition::CC => !cpsr.c(), // C == 0

            // Single Flag Checks
            Condition::MI => cpsr.n(),  // N == 1
            Condition::PL => !cpsr.n(), // N == 0
            Condition::VS => cpsr.v(),  // V == 1
            Condition::VC => !cpsr.v(), // V == 0

            // Complex Unsigned Comparisons
            Condition::HI => cpsr.c() && !cpsr.z(), // C == 1 AND Z == 0 (Higher)
            Condition::LS => !cpsr.c() || cpsr.z(), // C == 0 OR Z == 1 (Lower or Same)

            // Signed Comparisons
            Condition::GE => cpsr.n() == cpsr.v(), // N == V (Greater than or Equal)
            Condition::LT => cpsr.n() != cpsr.v(), // N != V (Less than)
            Condition::GT => !cpsr.z() && (cpsr.n() == cpsr.v()), // Z == 0 AND N == V (Greater than)
            Condition::LE => cpsr.z() || (cpsr.n() != cpsr.v()), // Z == 1 OR N != V (Less than or Equal)

            // Unconditional/Reserved
            Condition::AL => true,
            Condition::NV => false, // Always fails
        }
    }
}

impl fmt::Display for Condition {
    /// Formats the condition code as its standard ARM mnemonic (e.g., "EQ", "AL").
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Condition::EQ => "EQ",
            Condition::NE => "NE",
            Condition::CS => "CS",
            Condition::CC => "CC",
            Condition::MI => "MI",
            Condition::PL => "PL",
            Condition::VS => "VS",
            Condition::VC => "VC",
            Condition::HI => "HI",
            Condition::LS => "LS",
            Condition::GE => "GE",
            Condition::LT => "LT",
            Condition::GT => "GT",
            Condition::LE => "LE",
            Condition::AL => "",   // AL is conventionally empty in disassembly
            Condition::NV => "NV", // Never
        };
        write!(f, "{}", s)
    }
}
/// ARM CPU feature flags for production apps.
///
/// This struct represents common ARM CPU capabilities,
#[derive(Debug, Clone, Copy)]
pub struct ARMFeatures {
    pub thumb2: bool, // Supports Thumb-2 (16/32 bit mix)
    pub v7: bool,     // ARMv7 architecture
                      // Other features not yet implemented
}

impl Default for ARMFeatures {
    fn default() -> Self {
        Self {
            thumb2: true,
            v7: true,
        }
    }
}
