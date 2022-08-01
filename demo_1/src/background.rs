pub struct Color {
    pub rgb: [u8; 3],
    pub dirs: [i8; 3],
}

impl Color {
    pub fn new(r:u8, g:u8, b:u8, r_dir:i8, g_dir:i8, b_dir:i8) -> Self {
        let rgb: [u8; 3] = [r, g, b];
        let dirs: [i8; 3] = [r_dir, g_dir, b_dir];
        Self { rgb, dirs }
    }
    
    pub fn get_next_nums(self: &mut Self) {
        let rs: usize = self.rgb.len();
        for i in 0..rs {
            if self.rgb[i] <= 0 {
                self.dirs[i] = 1; 
            }
            if self.rgb[i] >= 255 {
                self.dirs[i] = -1; 
            }
            self.rgb[i] = ((self.rgb[i] as i16) + self.dirs[i] as i16) as u8;
        }
    }
}
