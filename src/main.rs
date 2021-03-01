use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
 
pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    
    let window_size: [u32; 2] = [1200, 800];
    let window = video_subsystem.window("Space Shooter", window_size[0], window_size[1])
        .position_centered()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap();
 
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let ship_speed: f32 = 7.5;
    let ship_size: [u32; 2] = [50, 50];
    let mut ship_acceleration: [f32; 2] = [0.0, 0.0]; 
    let mut ship_position: [i32; 2] = [575, 725];

    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        //player input
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running ,
                Event::Quit {..} => break 'running,

                Event::KeyDown { keycode: Some(Keycode::W), .. } => if ship_acceleration[1] > -25.0 {ship_acceleration[1] -= ship_speed},
                Event::KeyDown { keycode: Some(Keycode::A), .. } => if ship_acceleration[0] > -25.0 {ship_acceleration[0] -= ship_speed},
                Event::KeyDown { keycode: Some(Keycode::S), .. } => if ship_acceleration[1] < 25.0 {ship_acceleration[1] += ship_speed},
                Event::KeyDown { keycode: Some(Keycode::D), .. } => if ship_acceleration[0] < 25.0 {ship_acceleration[0] += ship_speed},
                _ => {}
            }
        }

        //draws ship
        canvas.set_draw_color(Color::RGB(255, 0, 0));

        //accelerates ship
        if ship_acceleration[0] < 0.0 {
            if ship_position[0] > 0 {
                ship_position[0] += ship_acceleration[0] as i32;
            }
            else {
                ship_acceleration[0] = 5.0;
            }
        }
        else {
            if ship_position[0] < (window_size[0] - ship_size[0]) as i32 {
                ship_position[0] += ship_acceleration[0] as i32;
            }
            else {
                ship_acceleration[0] = -5.0;
            }
        }

        if ship_acceleration[1] < 0.0 {
            if ship_position[1] > 0 {
                ship_position[1] += ship_acceleration[1] as i32;
            }
            else {
                ship_acceleration[1] = 5.0;
            }
        }
        else {
            if ship_position[1] < (window_size[1] - ship_size[1]) as i32 {
                ship_position[1] += ship_acceleration[1] as i32;
            }
            else {
                ship_acceleration[1] = -5.0;
            }
        }

        //friction Physics simulation
        if ship_acceleration[0] != 0.0 {ship_acceleration[0] /= 1.025}
        if ship_acceleration[1] != 0.0 {ship_acceleration[1] /= 1.025}

        let ship: sdl2::rect::Rect = Rect::new(ship_position[0], ship_position[1], ship_size[0], ship_size[1]);

        if canvas.fill_rect(ship) != Ok(()) {
            panic!("Failed to draw ship")
        }

        canvas.present();
        sleep(1000 / 60);
    }
}

use std::{thread, time};

pub fn sleep(millis: u64) {
    let duration = time::Duration::from_millis(millis);
    thread::sleep(duration);
}
