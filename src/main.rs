use std::env;
use rom_reader::ROMReader;
use chip_8::{Chip8, Command};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::Duration;

mod rom_reader;
mod chip_8;

// fn main() {
//     let args: Vec<String> = env::args().collect();

//     let rom_path = args.get(1).expect("ERROR: invalid ROM file path");

//     let rom = ROMReader::read(rom_path);

//    let mut chip_8 = Chip8::new(&rom);

   
//    loop {
//        chip_8.emulate_cycle();
//        if chip_8.is_halt { break; }
//    }
// }

pub fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    let rom_path = args.get(1).expect("ERROR: invalid ROM file path");

    let rom = ROMReader::read(rom_path);

   let mut chip_8 = Chip8::new(&rom);

    let sdl_context = sdl2::init()?;
    let mut canvas = get_canvas(&sdl_context)?;
    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        match chip_8.tick() {
            Command::Draw(display) => {
                update_screen(&mut canvas, display);
            }
            Command::Nothing => { /* noop */ }
            Command::Beep => {}
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }

    Ok(())
}

fn update_screen(canvas: &mut Canvas<Window>, display: [[bool; 64]; 32]) {
    canvas.clear();
    for (i, row) in display.iter().enumerate() {
        for (j, is_pixel_colored) in row.iter().enumerate() {
            let x = i32::try_from(j * 10).unwrap();
            let y = i32::try_from(i * 10).unwrap();
            let color = if *is_pixel_colored { Color::WHITE } else { Color::BLACK };
            canvas.set_draw_color(color);
            canvas.draw_rect(sdl2::rect::Rect::new(x, y, 10, 10)).unwrap();
        }
    }
    canvas.present();
}

fn get_canvas(sdl_context: &sdl2::Sdl) -> Result<sdl2::render::Canvas<sdl2::video::Window>, String> {
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("CHIP-8 Emulator", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    Ok(canvas)
}