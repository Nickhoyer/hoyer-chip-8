use super::interpreter::Interpreter;

impl Interpreter {
    pub fn decode_and_execute(&mut self, opcode: u16) {
        let hex_digits = (
            ((opcode & 0xf000) >> 12) as u8,
            ((opcode & 0x0f00) >> 8) as u8,
            ((opcode & 0x00f0) >> 4) as u8,
            ((opcode & 0x000f) >> 0) as u8,
        );
        let vx = hex_digits.1 as usize;
        let vy = hex_digits.2 as usize;
        let n = hex_digits.3;
        let nn = (opcode & 0x00ff) as u8;
        let nnn = opcode & 0x0fff;

        match hex_digits {
            (0x00, 0x00, 0x0e, 0x00) => self.clear_display(),
            (0x00, 0x00, 0x0e, 0x0e) => self.return_from_subroutine(),
            (0x01, _, _, _) => self.jump_to_address(nnn),
            (0x02, _, _, _) => self.call_subroutine(nnn),
            (0x03, _, _, _) => self.skip_if_vx_equal_nn(vx, nn),
            (0x04, _, _, _) => self.skip_if_vx_not_equal_nn(vx, nn),
            (0x05, _, _, 0x00) => self.skip_if_vx_equal_vy(vx, vy),
            (0x06, _, _, _) => self.vx_set_nn(vx, nn),
            (0x07, _, _, _) => self.vx_add_nn(vx, nn),
            (0x08, _, _, 0x00) => self.vx_set_vy(vx, vy),
            (0x08, _, _, 0x01) => self.vx_or_vy(vx, vy),
            (0x08, _, _, 0x02) => self.vx_and_vy(vx, vy),
            (0x08, _, _, 0x03) => self.vx_xor_vy(vx, vy),
            (0x08, _, _, 0x04) => self.vx_add_vy(vx, vy),
            (0x08, _, _, 0x05) => self.vx_sub_vy(vx, vy),
            (0x08, _, _, 0x06) => self.vx_shift_right(vx),
            (0x08, _, _, 0x07) => self.vx_subn_vy(vx, vy),
            (0x08, _, _, 0x0e) => self.vx_shift_left(vx),
            (0x09, _, _, 0x00) => self.skip_if_vx_not_equal_vy(vx, vy),
            (0x0a, _, _, _) => self.index_set_nnn(nnn),
            (0x0b, _, _, _) => self.jump_with_offset(nnn),
            (0x0c, _, _, _) => self.vx_set_rand_and_nn(vx, nn),
            (0x0d, _, _, _) => self.display_sprite(vx, vy, n),
            (0x0e, _, 0x09, 0x0e) => self.skip_if_key(vx),
            (0x0e, _, 0x0a, 0x01) => self.skip_if_not_key(vx),
            (0x0f, _, 0x00, 0x07) => self.vx_set_delay_timer(vx),
            (0x0f, _, 0x00, 0x0a) => self.wait_for_key(vx),
            (0x0f, _, 0x01, 0x05) => self.delay_timer_set_vx(vx),
            (0x0f, _, 0x01, 0x08) => self.sound_timer_set_vx(vx),
            (0x0f, _, 0x01, 0x0e) => self.index_add_vx(vx),
            (0x0f, _, 0x02, 0x09) => self.index_set_font(vx),
            (0x0f, _, 0x03, 0x03) => self.index_set_decimal(vx),
            (0x0f, _, 0x05, 0x05) => self.write_memory(vx),
            (0x0f, _, 0x06, 0x05) => self.load_memory(vx),
            _ => {}
        }
    }
}
