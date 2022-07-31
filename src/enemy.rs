use sdl2::rect::Rect;
use rand::Rng;
use std::vec::Vec;

// TODO: Figure out how to have global constants across multiple files
pub static SCREEN_X:u16 = 1200;
pub static SCREEN_Y:u16 = 800;

pub struct Enemy {
    x:i16,
    y:i16,
    x_vel:i16,
    y_vel:i16,
    size:u8,
}

impl Enemy {
    // Constructor
    pub fn new(x:i16, y:i16, x_vel:i16, y_vel:i16, size:u8 ) -> Self{
        Self { x, y, x_vel, y_vel, size }
    }

    pub fn get_rect(self: &Self)-> Rect {
        let _enemy_rect = Rect::new(self.x as i32, self.y as i32, 
                                    self.size as u32, self.size as u32);
        return _enemy_rect;
    }

    pub fn print_pos(self: &Self){
        println!("Enemy: ({},{})", self.x, self.y);
    }

    pub fn handle_movement(self: &mut Self) {
        self.bounce();
        self.x = self.x + self.x_vel;
        self.y = self.y + self.y_vel;
    }
    
    fn bounce(self: &mut Self){
        if self.x <= 0 || self.x + self.size as i16 >= SCREEN_X as i16 {
            self.x_vel *= -1;
        }
        else if self.y <= 0 || self.y + self.size as i16 >= SCREEN_Y as i16 {
            self.y_vel *= -1;
        }
    }

    // Delete stray enemies
    fn check_bounds(self: &Self){
        if self.x <= -100 || self.x >= (SCREEN_X + 100 ) as i16 ||
        self.y <= -100 || self.y >= (SCREEN_Y + 100) as i16 {
            drop(self);
        }
    }
}

pub struct Spawner {
    enemy_vect: Vec<Enemy>, 
    enemy_size: usize,
    max_enemies: u16,
}

impl Spawner {
    pub fn new(enemy_size: usize, max_enemies: u16) -> Self {
        let enemy_vect: Vec<Enemy> = Vec::new();
        Self { enemy_size, enemy_vect, max_enemies }
    } 

    pub fn make_enemy(self: &mut Self) {
        let mut rng = rand::thread_rng();
        let _x = rng.gen_range((self.enemy_size as u16)..(SCREEN_X-self.enemy_size as u16));
        let _y = rng.gen_range(((self.enemy_size*2) as u16)..(SCREEN_Y-(self.enemy_size*2) as u16));
        let _x_vel;
        let _y_vel;
        if _x < SCREEN_X / 2 {
            _x_vel = rng.gen_range(5..15);
        } else {
            _x_vel = rng.gen_range(-15..-5);
        }

        if _y < SCREEN_Y / 2 {
            _y_vel = rng.gen_range(-15..-5);
        } else {
            _y_vel = rng.gen_range(5..15);
        }

        let _y_vel = rng.gen_range(5..15);
        let _new_enemy:Enemy = Enemy::new(self.enemy_size as i16, _y as i16, // x, y
                                          _x_vel, _y_vel, // x_vel, y_vel
                                          self.enemy_size as u8); // size
        self.enemy_vect.push(_new_enemy);
    }

    pub fn handle_enemies(self: &mut Self, curr_cycle: i64,
                          _rect_vect: &mut Vec<Rect>){
        if self.enemy_vect.len() < self.max_enemies as usize && 
        curr_cycle % 10 == 0 {
            self.make_enemy();
        }

        for _i in 0..self.enemy_vect.len() {
            self.enemy_vect[_i].handle_movement();
            self.enemy_vect[_i].check_bounds();
            let _enemy_rect = self.enemy_vect[_i].get_rect();
            _rect_vect.push(_enemy_rect);
        }
    }
}
