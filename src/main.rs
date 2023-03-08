use std::{fs::File, io::Read};

use sdl2::{video::Window, pixels::{Color, PixelFormatEnum}, rect::Rect};

mod chip8;
use crate::chip8::Chip8;

fn main() {
    let mut chip8 = Chip8::new();
    let mut file = File::open("ibm_logo.ch8").unwrap();
    file.read(&mut chip8.memory).unwrap();

    println!("{:x?}", chip8.memory);

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
    
    loop {
        chip8.cycle().unwrap();
    }
}
