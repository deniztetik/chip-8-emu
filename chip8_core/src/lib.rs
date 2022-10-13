pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

const RAM_SIZE: usize = 4096;
const NUM_REGS: usize = 16;
const STACK_SIZE: usize = 16;
const NUM_KEYS: usize = 16;

pub struct Emu {
    pc: u16, // Program Counter register
    ram: [u8; RAM_SIZE],
    screen: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
    v_reg: [u8; NUM_REGS],    // Registers
    i_reg: u16,               // I Register
    sp: u16,                  // Stack Pointer
    stack: [u16; STACK_SIZE], // Stack
    keys: [bool; NUM_KEYS],
    dt: u8, // Delay Timer
    st: u8, // Sound Timer
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}
