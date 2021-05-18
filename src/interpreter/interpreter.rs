use std::fs::read;

use winit::event::VirtualKeyCode;
use winit_input_helper::WinitInputHelper;

use crate::constants::FONTSET;

pub struct Interpreter {
    pub memory: [u8; 4096],
    pub program_counter: u16,
    pub index: u16,
    pub registers: [u8; 16],
    pub stack: [u16; 16],
    pub stack_pointer: usize,
    pub sound_timer: u8,
    pub delay_timer: u8,
    pub keypad: [bool; 16],
    pub video_output: [u64; 32],
}

impl Interpreter {
    pub fn new(rom: &str) -> Interpreter {
        let mut interpreter = Interpreter {
            video_output: [0; 32],
            keypad: [false; 16],
            memory: [0; 4096],
            registers: [0; 16],
            stack: [0; 16],
            program_counter: 0x200,
            stack_pointer: 0,
            index: 0,
            sound_timer: 0,
            delay_timer: 0,
        };
        for i in 0..FONTSET.len() {
            interpreter.memory[0x50 + i] = FONTSET[i];
        }
        interpreter.load(rom);
        return interpreter;
    }

    pub fn load(&mut self, rom: &str) {
        match read(rom) {
            Ok(bytes) => {
                for i in 0..bytes.len() {
                    self.memory[0x200 + i] = bytes[i];
                }
                self.program_counter = 0x200;
            }
            Err(e) => {
                if e.kind() == std::io::ErrorKind::PermissionDenied {
                    return;
                }
                panic!("{}", e);
            }
        }
    }

    pub fn update(&mut self) {
        let opcode = (self.memory[self.program_counter as usize] as u16) << 8
            | self.memory[self.program_counter as usize + 1] as u16;
        self.program_counter += 2;
        self.decode_and_execute(opcode);
        if self.delay_timer > 0 {
            self.delay_timer -= 1
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1
        }
    }

    pub fn draw(&mut self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = i % 64;
            let y = (i as f32 / 64.0).floor() as usize;
            let output = self.video_output[y] & (1 << x);

            let rgba = if output != 0 {
                [0xff, 0xff, 0xff, 0xff]
            } else {
                [0x15, 0x15, 0x15, 0xff]
            };
            pixel.copy_from_slice(&rgba);
        }
    }

    pub fn update_inputs(&mut self, input: &WinitInputHelper) {
        self.keypad[0] =
            input.key_held(VirtualKeyCode::Key1) || input.key_pressed(VirtualKeyCode::Key1);
        self.keypad[1] =
            input.key_held(VirtualKeyCode::Key2) || input.key_pressed(VirtualKeyCode::Key2);
        self.keypad[2] =
            input.key_held(VirtualKeyCode::Key3) || input.key_pressed(VirtualKeyCode::Key3);
        self.keypad[3] =
            input.key_held(VirtualKeyCode::Key4) || input.key_pressed(VirtualKeyCode::Key4);

        self.keypad[4] = input.key_held(VirtualKeyCode::Q) || input.key_pressed(VirtualKeyCode::Q);
        self.keypad[5] = input.key_held(VirtualKeyCode::W) || input.key_pressed(VirtualKeyCode::W);
        self.keypad[6] = input.key_held(VirtualKeyCode::E) || input.key_pressed(VirtualKeyCode::E);
        self.keypad[7] = input.key_held(VirtualKeyCode::R) || input.key_pressed(VirtualKeyCode::R);

        self.keypad[8] = input.key_held(VirtualKeyCode::A) || input.key_pressed(VirtualKeyCode::A);
        self.keypad[9] = input.key_held(VirtualKeyCode::S) || input.key_pressed(VirtualKeyCode::S);
        self.keypad[10] = input.key_held(VirtualKeyCode::D) || input.key_pressed(VirtualKeyCode::D);
        self.keypad[11] = input.key_held(VirtualKeyCode::F) || input.key_pressed(VirtualKeyCode::F);

        self.keypad[12] = input.key_held(VirtualKeyCode::Z) || input.key_pressed(VirtualKeyCode::Z);
        self.keypad[13] = input.key_held(VirtualKeyCode::X) || input.key_pressed(VirtualKeyCode::X);
        self.keypad[14] = input.key_held(VirtualKeyCode::C) || input.key_pressed(VirtualKeyCode::C);
        self.keypad[15] = input.key_held(VirtualKeyCode::V) || input.key_pressed(VirtualKeyCode::V);
    }
}
