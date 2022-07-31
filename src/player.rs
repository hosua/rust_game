use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;


static SCREEN_X:u16 = 1200;
static SCREEN_Y:u16 = 800;

pub struct Player {
    x:u16,
    y:u16,
    size:u8,
    movespeed:u8,
    keys: [bool; 4],
    // 0, Left
    // 1, Down
    // 2, Right
    // 3, Up
}

pub fn get_key_num(kcode:Keycode) -> u8{
    let mut out: u8 = 5;
    match kcode {
        Keycode::Left => {
            out = 0;
        },
        Keycode::Down => {
            out = 1;
        },
        Keycode::Right => {
            out = 2;
        },
        Keycode::Up => {
            out = 3;
        },
        _=> {}
    }
    return out;
}

impl Player {
    // An example of a constructor
    // See: https://rust-unofficial.github.io/patterns/idioms/ctor.html
    pub fn new(x: u16, y: u16, size: u8, movespeed: u8) -> Self {
        let keys: [bool; 4] = [false, false, false, false];
        Self { x, y, size, movespeed, keys }
    }

    pub fn get_rect(self: &Self) -> Rect {
        let _player_rect = Rect::new(self.x as i32, self.y as i32, 
                                    self.size as u32, self.size as u32);
        return _player_rect;
    }

    pub fn print_keys(self: &Self) {
        for _key in &self.keys {
            print!("{} ", _key); 
        }
        println!();
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
            Event::KeyUp { keycode: Some(Keycode::Up), .. } => {
                let index = get_key_num(Keycode::Up) as usize;
                self.keys[index] = false;
            },

            _ => {}
        }
    }

    pub fn handle_movement(self: &mut Self) {
        if self.keys[get_key_num(Keycode::Left) as usize]{
            if self.x - (self.size * self.movespeed) as u16 > 0 {
                self.x -= (self.size * self.movespeed) as u16;
            }
        }
        if self.keys[get_key_num(Keycode::Down) as usize]{
            if (self.y + (self.size * self.movespeed) as u16) < SCREEN_Y {
                self.y += (self.size * self.movespeed) as u16;
            }

        }
        if self.keys[get_key_num(Keycode::Right) as usize]{
            if (self.x + (self.size * self.movespeed) as u16) < SCREEN_X {
                self.x += (self.size * self.movespeed) as u16;
            }

        }
        if self.keys[get_key_num(Keycode::Up) as usize]{
            if self.y - (self.size * self.movespeed) as u16 > 0 {
                self.y -= (self.size * self.movespeed) as u16;
            }

        }
    }

    pub fn print_info(self: &Self) {
        println!("({},{})", self.x, self.y);
    }

}
