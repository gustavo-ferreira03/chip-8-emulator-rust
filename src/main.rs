use std::{fs::File, io::Read, time::Instant};

use sdl2::{video::Window, pixels::{Color, PixelFormatEnum}, rect::Rect, event::Event, keyboard::Keycode};

mod chip8;
use crate::chip8::Chip8;

fn main() {
    let mut chip8 = Chip8::new();
    let mut file = File::open("tetris.ch8").unwrap();
    file.read(&mut chip8.memory[512..]).unwrap();

    // println!("{:#x?}", chip8.memory);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let audio_subsystem = sdl_context.audio().unwrap();

    let window = video_subsystem
        .window(
            "chip8-rs",
            64*10,
            32*10,
        )
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string()).unwrap();

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string()).unwrap();
    let mut pixel = Rect::new(0, 0, 10, 10);

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut key: u8;
    
    'running: loop {
        let now = Instant::now();
        if chip8.read_opcode() == 0 {
            break;
        }
        
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,

                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    key = match keycode {
                        Keycode::Num1 => 0x1,
                        Keycode::Num2 => 0x2,
                        Keycode::Num3 => 0x3,
                        Keycode::Num4 => 0xC,
                        Keycode::Q => 0x4,
                        Keycode::W => 0x5,
                        Keycode::E => 0x6,
                        Keycode::R => 0xD,
                        Keycode::A => 0x7,
                        Keycode::S => 0x8,
                        Keycode::D => 0x9,
                        Keycode::F => 0xE,
                        Keycode::Z => 0xA,
                        Keycode::X => 0x0,
                        Keycode::C => 0xB,
                        Keycode::V => 0xF,
                        _ => 0,
                    };
                    chip8.key_down(key);
                },
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => {
                    key = match keycode {
                        Keycode::Num1 => 0x1,
                        Keycode::Num2 => 0x2,
                        Keycode::Num3 => 0x3,
                        Keycode::Num4 => 0xC,
                        Keycode::Q => 0x4,
                        Keycode::W => 0x5,
                        Keycode::E => 0x6,
                        Keycode::R => 0xD,
                        Keycode::A => 0x7,
                        Keycode::S => 0x8,
                        Keycode::D => 0x9,
                        Keycode::F => 0xE,
                        Keycode::Z => 0xA,
                        Keycode::X => 0x0,
                        Keycode::C => 0xB,
                        Keycode::V => 0xF,
                        _ => 0,
                    };
                    chip8.key_up(key);
                },
                _ => {}
            }
        }
        chip8.cycle();

        if chip8.draw {
            canvas.clear();
            for y in 0..32 {
                for x in 0..64 {
                    pixel.set_x(x*10);
                    pixel.set_y(y*10);
    
                    if chip8.display[y as usize][x as usize] == 1 {
                        canvas.set_draw_color(Color::RGB(255, 255, 255));
                    }
                    else {
                        canvas.set_draw_color(Color::RGB(0, 0, 0));
                    }
                    let _ = canvas.fill_rect(pixel);
                }
            }
        }
        
        canvas.present();
        chip8.elapsed_time += now.elapsed();
    }
}
