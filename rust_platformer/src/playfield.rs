use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;

const SCREEN_X:u16 = 1200;
const SCREEN_Y:u16 = 800;

pub struct PlayField {
    // 2D array
    pub field:[[bool; SCREEN_Y as usize];SCREEN_X as usize],
}

impl PlayField {
    pub fn new() -> Self {
        let field = [[false; SCREEN_Y as usize]; SCREEN_X as usize];
        Self { field }
    }

    pub fn clear(self: &mut Self) {
        self.field = [[false; SCREEN_Y as usize]; SCREEN_X as usize];
    }
}

pub struct Platform {
    x: i32,
    y: i32,
    w: usize,
    h: usize,
}

impl Platform {
    pub fn new(x: i32, y: i32,
               w: usize, h: usize) -> Self{
        Self { x, y, w, h }
    }

    pub fn get_rect(self: &Self) -> Rect {
        let _platform_rect: Rect = Rect::new(self.x, self.y, 
                                             self.w as u32, self.h as u32);
        return _platform_rect;
    }

    pub fn set_to_field(self: &Self, _field: &mut [[bool; SCREEN_Y as usize]; SCREEN_X as usize]){
        for _i in self.y..self.y + self.h as i32 { 
            for _j in self.x..self.x + self.w as i32 {
                _field[_j as usize][_i as usize] = true; 
            }
        }
    }
}
