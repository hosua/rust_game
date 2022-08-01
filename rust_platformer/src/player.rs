use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;

pub struct Player {
    x:i16,
    y:i16,
    size:u8,
    movespeed:u8,
    keys: [bool; 4],
    jump_timer:u8,
}

pub enum KeyNum {
    NONE = -1,
    LEFT = 0,
    DOWN = 1,
    RIGHT = 2,
    UP = 3,
}

const SCREEN_X:u16 = 1200;
const SCREEN_Y:u16 = 800;

pub fn get_key_num(kcode:Keycode) -> KeyNum{
    let mut out: KeyNum = KeyNum::NONE;
    match kcode {
        Keycode::Left => {
            out = KeyNum::LEFT;
        },
        Keycode::Down => {
            out = KeyNum::DOWN;
        },
        Keycode::Right => {
            out = KeyNum::RIGHT;
        },
        Keycode::Up => {
            out = KeyNum::UP;
        },
        _=> {}
    }
    return out;
}

impl Player {
    // An example of a constructor
    // See: https://rust-unofficial.github.io/patterns/idioms/ctor.html
    pub fn new(x: i16, y: i16, movespeed: u8, size: u8) -> Self {
        let keys: [bool; 4] = [false, false, false, false];
        let jump_timer = 0;
        Self { x, y, size, movespeed, keys, jump_timer }
    }

    pub fn get_rect(self: &Self)-> Rect {
        let _player_rect = Rect::new(self.x as i32, self.y as i32, 
                                    self.size as u32, self.size as u32);
        return _player_rect;
    }

    pub fn set_to_field(self: &Self, _field: &mut [[bool; SCREEN_Y as usize]; SCREEN_X as usize]){
        for _i in self.y..self.y + self.size as i16 { 
            for _j in self.x..self.x + self.size as i16 {
                _field[_j as usize][_i as usize] = true; 
            }
        }
    }
    
    pub fn handle_input(self: &mut Self, event: sdl2::event::Event){
        match event {
            Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                let index = get_key_num(Keycode::Left) as usize;
                self.keys[index] = true;
            },

            Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                let index = get_key_num(Keycode::Down) as usize;
                self.keys[index] = true;
            },
            Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                let index = get_key_num(Keycode::Right) as usize;
                self.keys[index] = true;
            },
            Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                let index = get_key_num(Keycode::Up) as usize;
                self.keys[index] = true;
            },
            // Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
            //     self.jump();
            // },
            Event::KeyUp { keycode: Some(Keycode::Left), .. } => {
                let index = get_key_num(Keycode::Left) as usize;
                self.keys[index] = false;
            },
            Event::KeyUp { keycode: Some(Keycode::Down), .. } => {
                let index = get_key_num(Keycode::Down) as usize;
                self.keys[index] = false;
            },
            Event::KeyUp { keycode: Some(Keycode::Right), .. } => {
                let index = get_key_num(Keycode::Right) as usize;
                self.keys[index] = false;
            },
            // Event::KeyUp { keycode: Some(Keycode::Up), .. } => {
            //     let index = get_key_num(Keycode::Up) as usize;
            //     self.keys[index] = false;
            // },

            _ => {}
        }
    }

    pub fn handle_movement(self: &mut Self) {
        if self.keys[get_key_num(Keycode::Left) as usize]{
            if self.x - (self.size * self.movespeed) as i16 >= 0 {
                self.x -= (self.size * self.movespeed) as i16;
            }
        }
        if self.keys[get_key_num(Keycode::Down) as usize]{
            if (self.y + (self.size * self.movespeed) as i16) < SCREEN_Y as i16{
                self.y += (self.size * self.movespeed) as i16;
            }

        }
        if self.keys[get_key_num(Keycode::Right) as usize]{
            if (self.x + (self.size * self.movespeed) as i16) < SCREEN_X as i16{
                self.x += (self.size * self.movespeed) as i16;
            }

        }
        if self.keys[get_key_num(Keycode::Up) as usize]{
            if self.y - (self.size * self.movespeed) as i16 >= 0 {
                self.y -= (self.size * self.movespeed) as i16;
            }
        }

        // Gravity
        if self.y < (SCREEN_Y - self.size as u16) as i16{
            self.y += 10;
            // Countdown the jump timer
            if self.y > 0 && self.jump_timer > 0 {
                self.jump_timer -= 1;
            }
        }

        let jump_index = get_key_num(Keycode::Up) as usize;
        // Start jump countdown timer
        if self.y == (SCREEN_Y - self.size as u16) as i16 {
            self.jump_timer = 10;
            // self.keys[jump_index] = true;
        }
        // Stop jump motion when timer hits 0
        if self.jump_timer == 0 {
            self.keys[jump_index] = false;
        }
        println!("jump timer: {}", self.jump_timer);
        self.print_info();
    }

    pub fn print_keys(self: &Self) {
        for _key in &self.keys {
            print!("{} ", _key); 
        }
        println!();
    }

    pub fn print_info(self: &Self) {
        println!("({},{})", self.x, self.y);
    }

}
