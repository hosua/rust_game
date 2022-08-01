use sdl2::pixels;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use std::time::Duration;

mod background;
mod player;
mod enemy;
mod playfield;
use playfield::PlayField;
use playfield::Platform;
use player::Player;
use enemy::Enemy;
use enemy::Spawner;

const SCREEN_X:u16 = 1200;
const SCREEN_Y:u16 = 800;

use std::env;

pub fn main() {
    // Debugging environment variables
    env::set_var("RUST_BACKTRACE", "1");
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-game demo", SCREEN_X as u32, SCREEN_Y as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(pixels::Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut _bg_color: background::Color = background::Color::new(202, 42, 68, 
                                                                -1, 1, -1);

    let mut _player:Player = Player::new(SCREEN_X as i16 / 2, SCREEN_Y as i16 / 2, 
                                         1, // movespeed
                                         20); // size

    // let mut _enemy_spawner: Spawner = Spawner::new(20, // Size
    //                                                100); // Capacity of enemies for this spawner

    let mut _enemy_rect_vect:Vec<Rect> = Vec::new(); 
    let mut _cycles: i64 = 0;

    let mut playfield = PlayField::new();

    let _platform_1 = Platform::new(100, SCREEN_Y as i32 - 50,
                                    50, 25);

    'running: loop {
        _bg_color.get_next_nums();
        canvas.set_draw_color(pixels::Color::RGB(_bg_color.rgb[0], _bg_color.rgb[1], _bg_color.rgb[2]));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                    },
                _ => {}
            }
            _player.handle_input(event);
        }
        // The rest of the game loop goes here...
        _player.handle_movement();
        // _player.print_info();
        // _player.print_keys();
        // TODO: Figure out how to pass canvas as an argument to functions, it would clean the code
        // up a lot.
        // _enemy_spawner.handle_enemies(_cycles, &mut _enemy_rect_vect);
        _platform_1.set_to_field(&mut playfield.field);

        let _player_rect = _player.get_rect();
        _player.set_to_field(&mut playfield.field);
        canvas.set_draw_color(pixels::Color::RGB(255, 255, 255));
        canvas.fill_rect(_player_rect).ok();

        let _platform_rect = _platform_1.get_rect();
        canvas.fill_rect(_platform_rect).ok();
        // let _enemy_rect = _enemy.get_rect();
        canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
        for _i in 0.._enemy_rect_vect.len() {
            canvas.fill_rect(_enemy_rect_vect[_i]).ok();
        }
        // canvas.fill_rect(_enemy_rect).ok();

        canvas.present();
        // Clear enemy rects
        _enemy_rect_vect = Vec::new(); 
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        playfield.clear();
        _cycles += 1;
    }
}
