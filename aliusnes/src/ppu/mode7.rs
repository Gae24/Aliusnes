pub struct Mode7 {
    matrix_a: i16,
    matrix_b: i16,
    mode_7_latch: u8,
}

impl Mode7 {
    pub fn new() -> Self {
        Self {
            matrix_a: 0,
            matrix_b: 0,
            mode_7_latch: 0,
        }
    }

    pub fn set_mode_7_matrix_a(&mut self, data: u8) {
        self.matrix_a = i16::from(data) << 8 | i16::from(self.mode_7_latch);
        self.mode_7_latch = data;
    }

    pub fn set_mode_7_matrix_b(&mut self, data: u8) {
        self.matrix_b = i16::from(data) << 8 | i16::from(self.mode_7_latch);
        self.mode_7_latch = data;
    }

    pub fn do_multiplication(&self) -> i32 {
        i32::from(self.matrix_a) * i32::from(self.matrix_b & 0xFF)
    }
}
