use sdl2::pixels;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

mod background;
mod player;
use player::Player;

// https://docs.rs/sdl2/0.30.0/sdl2/render/struct.Canvas.html
static SCREEN_X:u16 = 1200;
static SCREEN_Y:u16 = 800;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", SCREEN_X as u32, SCREEN_Y as u32)
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

    let mut _player:Player = Player::new(SCREEN_X / 2, SCREEN_Y / 2, 
                                         10, 2);

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
        _player.handle_movement();
        _player.print_info();
        _player.print_keys();
        // The rest of the game loop goes here...
        let _player_rect = _player.get_rect();
        canvas.set_draw_color(pixels::Color::RGB(255, 255, 255));
        canvas.fill_rect(_player_rect).ok();

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
