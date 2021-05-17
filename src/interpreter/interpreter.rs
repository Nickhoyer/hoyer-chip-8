pub struct Interpreter {
    pub memory: [u8; 4096],
    pub program_counter: u16,
    pub index: u16,
    pub registers: [u8; 16],
    pub stack: [u16; 16],
    pub stack_pointer: usize,
    pub sound_timer: u8,
    pub delay_timer: u8,
    pub video_output: [u64; 32],
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            video_output: [0; 32],
            memory: [0; 4096],
            registers: [0; 16],
            stack: [0; 16],
            program_counter: 0x200,
            stack_pointer: 0,
            index: 0,
            sound_timer: 0,
            delay_timer: 0,
        }
    }
}
