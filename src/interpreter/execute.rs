use rand::{thread_rng, Rng};

use super::interpreter::Interpreter;
impl Interpreter {
    /// 00E0 - CLS
    ///
    /// Clear the display.
    pub fn clear_display(&mut self) {
        self.video_output = [0; 32];
    }

    /// 00EE - RET
    ///
    /// Return from a subroutine.
    pub fn return_from_subroutine(&mut self) {
        self.stack_pointer -= 1;
        self.program_counter = self.stack[self.stack_pointer];
    }

    /// 1nnn - JP addr
    ///
    /// Jump to location nnn.
    pub fn jump_to_address(&mut self, nnn: u16) {
        self.program_counter = nnn;
    }

    /// 2nnn - CALL addr
    ///
    /// Call subroutine at nnn.
    pub fn call_subroutine(&mut self, nnn: u16) {
        self.stack[self.stack_pointer] = self.program_counter;
        self.stack_pointer += 1;
        self.program_counter = nnn;
    }

    /// 3xnn - SE Vx, byte
    ///
    /// Skip next instruction if Vx = nn.
    pub fn skip_if_vx_equal_nn(&mut self, vx: usize, nn: u8) {
        if self.registers[vx] == nn {
            self.program_counter += 2;
        }
    }

    /// 4xnn - SNE Vx, byte
    ///
    /// Skip next instruction if Vx != nn.
    pub fn skip_if_vx_not_equal_nn(&mut self, vx: usize, nn: u8) {
        if self.registers[vx] != nn {
            self.program_counter += 2;
        }
    }

    /// 5xy0 - SE Vx, Vy
    ///
    /// Skip next instruction if Vx = Vy.
    pub fn skip_if_vx_equal_vy(&mut self, vx: usize, vy: usize) {
        if self.registers[vx] == self.registers[vy] {
            self.program_counter += 2;
        }
    }

    /// 6xnn - LD Vx, byte
    ///
    /// Set Vx = nn.
    pub fn vx_set_nn(&mut self, vx: usize, nn: u8) {
        self.registers[vx] = nn;
    }

    /// 7xnn - ADD Vx, byte
    ///
    /// Set Vx = Vx + nn.
    pub fn vx_add_nn(&mut self, vx: usize, nn: u8) {
        let sum = self.registers[vx] as u16 + nn as u16;
        self.registers[vx] = (sum & 0xff) as u8;
    }

    /// 8xy0 - LD Vx, Vy
    ///
    /// Set Vx = Vy.
    pub fn vx_set_vy(&mut self, vx: usize, vy: usize) {
        self.registers[vx] = self.registers[vy];
    }

    /// 8xy1 - OR Vx, Vy
    ///
    /// Set Vx = Vx OR Vy.
    pub fn vx_or_vy(&mut self, vx: usize, vy: usize) {
        self.registers[vx] |= self.registers[vy];
    }

    /// 8xy2 - AND Vx, Vy
    ///
    /// Set Vx = Vx AND Vy.
    pub fn vx_and_vy(&mut self, vx: usize, vy: usize) {
        self.registers[vx] &= self.registers[vy];
    }

    /// 8xy3 - XOR Vx, Vy
    ///
    /// Set Vx = Vx XOR Vy.
    pub fn vx_xor_vy(&mut self, vx: usize, vy: usize) {
        self.registers[vx] ^= self.registers[vy];
    }

    /// 8xy4 - ADD Vx, Vy
    ///
    /// Set Vx = Vx + Vy, set Vf = carry.
    pub fn vx_add_vy(&mut self, vx: usize, vy: usize) {
        let sum = self.registers[vx] as u16 + self.registers[vy] as u16;
        self.registers[0xf] = if sum > 255 { 1 } else { 0 };
        self.registers[vx] = (sum & 0xff) as u8;
    }

    /// 8xy5 - SUB Vx, Vy
    ///
    /// Set Vx = Vx - Vy, set Vf = NOT borrow.
    pub fn vx_sub_vy(&mut self, vx: usize, vy: usize) {
        self.registers[0xf] = if self.registers[vx] > self.registers[vy] {
            1
        } else {
            0
        };
        self.registers[vx] = self.registers[vx].wrapping_sub(self.registers[vy]);
    }

    /// 8xy6 - SHR Vx {, Vy}
    ///
    /// Set Vx = Vx SHR 1.
    pub fn vx_shift_right(&mut self, vx: usize) {
        self.registers[0xf] = self.registers[vx] & 1;
        self.registers[vx] >>= 1;
    }

    /// 8xy7 - SUBN Vx, Vy
    ///
    /// Set Vx = Vy - Vx, set Vf = NOT borrow.
    pub fn vx_subn_vy(&mut self, vx: usize, vy: usize) {
        self.registers[0xf] = if self.registers[vx] < self.registers[vy] {
            1
        } else {
            0
        };
        self.registers[vx] = self.registers[vy].wrapping_sub(self.registers[vx]);
    }

    /// 8xyE - SHL Vx {, Vy}
    ///
    /// Set Vx = Vx SHL 1.
    pub fn vx_shift_left(&mut self, vx: usize) {
        self.registers[0xf] = (self.registers[vx] & 0b10000000) >> 7;
        self.registers[vx] <<= 1;
    }

    /// 9xy0 - SNE Vx, Vy
    ///
    /// Skip next instruction if Vx != Vy.
    pub fn skip_if_vx_not_equal_vy(&mut self, vx: usize, vy: usize) {
        if self.registers[vx] != self.registers[vy] {
            self.program_counter += 2;
        }
    }

    /// Annn - LD I, addr
    ///
    /// Set I = nnn.
    pub fn index_set_nnn(&mut self, nnn: u16) {
        self.index = nnn;
    }

    /// Bnnn - JP V0, addr
    ///
    /// Jump to location nnn + V0.
    pub fn jump_with_offset(&mut self, nnn: u16) {
        self.program_counter = self.registers[0] as u16 + nnn;
    }

    /// Cxnn - RND Vx, byte
    ///
    /// Set Vx = random byte AND nn.
    pub fn vx_set_rand_and_nn(&mut self, vx: usize, nn: u8) {
        self.registers[vx] = thread_rng().gen::<u8>() & nn;
    }

    /// Dxyn - DRW Vx, Vy, nibble
    ///
    /// Display n-byte sprite starting at memory location I at (Vx, Vy), set Vf = collision.
    pub fn display_sprite(&mut self, vx: usize, vy: usize, n: u8) {
        let x = self.registers[vx] % 64;
        let y = self.registers[vy] % 32;

        for byte in 0..n {
            let sprite = self.memory[(self.index + byte as u16) as usize];
            for bit in 0..8 {
                let color = (sprite & (0x80 >> bit)) as u64;
                self.registers[0xf] |= ((self.video_output[(y as usize + byte as usize) % 32]
                    & (color << x % 64))
                    >> x % 64) as u8
                    >> 7 - bit;

                self.video_output[(y as usize + byte as usize) % 32] ^=
                    (color >> 7 - bit) << (x + bit) % 64;
            }
        }
    }

    /// Ex9E - SKP Vx
    ///
    /// Skip next instruction if key with the value of Vx is pressed.
    pub fn skip_if_key(&mut self, vx: usize) {
        if self.keypad[self.registers[vx] as usize] {
            self.program_counter += 2;
        }
    }

    /// ExA1 - SKNP Vx
    ///
    /// Skip next instruction if key with the value of Vx is not pressed.
    pub fn skip_if_not_key(&mut self, vx: usize) {
        if !self.keypad[self.registers[vx] as usize] {
            self.program_counter += 2;
        }
    }

    /// fx07 - LD Vx, DT
    ///
    /// Set Vx = delay timer value.
    pub fn vx_set_delay_timer(&mut self, vx: usize) {
        self.registers[vx] = self.delay_timer;
    }

    /// fx0A - LD Vx, K
    ///
    /// Wait for a key press, store the value of the key in Vx.
    pub fn wait_for_key(&mut self, vx: usize) {
        let mut found_key = false;
        for key in 0..16 {
            if self.keypad[key] {
                found_key = true;
                self.registers[vx] = key as u8;
            }
        }
        if !found_key {
            self.program_counter -= 2;
        }
    }

    /// fx15 - LD DT, Vx
    ///
    /// Set delay timer = Vx.
    pub fn delay_timer_set_vx(&mut self, vx: usize) {
        self.delay_timer = self.registers[vx];
    }

    /// fx18 - LD ST, Vx
    ///
    /// Set sound timer = Vx.
    pub fn sound_timer_set_vx(&mut self, vx: usize) {
        self.sound_timer = self.registers[vx];
    }

    /// fx1E - ADD I, Vx
    ///
    /// Set I = I + Vx.
    pub fn index_add_vx(&mut self, vx: usize) {
        self.index += self.registers[vx] as u16;
    }

    /// fx29 - LD f, Vx
    ///
    /// Set I = location of sprite for digit Vx.
    pub fn index_set_font(&mut self, vx: usize) {
        self.index = 0x50 + (5 * self.registers[vx] as u16);
    }

    /// fx33 - LD B, Vx
    ///
    /// Store BCD representation of Vx in memory locations I, I+1, and I+2.
    pub fn index_set_decimal(&mut self, vx: usize) {
        let mut value = self.registers[vx];
        for i in 0..3 {
            self.memory[self.index as usize + 2 - i] = value % 10;
            value /= 10;
        }
    }

    /// fx55 - LD [I], Vx
    ///
    /// Store registers V0 through Vx in memory starting at location I.
    pub fn write_memory(&mut self, vx: usize) {
        for i in 0..(vx + 1) {
            self.memory[self.index as usize + i] = self.registers[i];
        }
    }

    /// fx65 - LD Vx, [I]
    ///
    /// Read registers V0 through Vx from memory starting at location I.
    pub fn load_memory(&mut self, vx: usize) {
        for i in 0..(vx + 1) {
            self.registers[i] = self.memory[self.index as usize + i];
        }
    }
}
