use std::env;
use rom_reader::ROMReader;
use chip_8::{Chip8, Command};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::Duration;

mod rom_reader;
mod chip_8;

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
            dbg!(&event);
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown { keycode, .. } => {
                    match keycode.unwrap() {
                        Keycode::Num1 => chip_8.on_key(0x1, true),
                        Keycode::Num2 => chip_8.on_key(0x2, true),
                        Keycode::Num3 => chip_8.on_key(0x3, true),
                        Keycode::Num4 => chip_8.on_key(0xC, true),

                        Keycode::Q => chip_8.on_key(0x4, true),
                        Keycode::W => chip_8.on_key(0x5, true),
                        Keycode::E => chip_8.on_key(0x6, true),
                        Keycode::R => chip_8.on_key(0xD, true),

                        Keycode::A => chip_8.on_key(0x7, true),
                        Keycode::S => chip_8.on_key(0x8, true),
                        Keycode::D => chip_8.on_key(0x9, true),
                        Keycode::F => chip_8.on_key(0xE, true),

                        Keycode::Z => chip_8.on_key(0xA, true),
                        Keycode::X => chip_8.on_key(0x0, true),
                        Keycode::C => chip_8.on_key(0xB, true),
                        Keycode::V => chip_8.on_key(0xF, true),

                        _ => {}
                    }
                }
                Event::KeyUp { keycode , .. } => {
                    match keycode.unwrap() {
                        Keycode::Num1 => chip_8.on_key(0x1, false),
                        Keycode::Num2 => chip_8.on_key(0x2, false),
                        Keycode::Num3 => chip_8.on_key(0x3, false),
                        Keycode::Num4 => chip_8.on_key(0xC, false),

                        Keycode::Q => chip_8.on_key(0x4, false),
                        Keycode::W => chip_8.on_key(0x5, false),
                        Keycode::E => chip_8.on_key(0x6, false),
                        Keycode::R => chip_8.on_key(0xD, false),

                        Keycode::A => chip_8.on_key(0x7, false),
                        Keycode::S => chip_8.on_key(0x8, false),
                        Keycode::D => chip_8.on_key(0x9, false),
                        Keycode::F => chip_8.on_key(0xE, false),

                        Keycode::Z => chip_8.on_key(0xA, false),
                        Keycode::X => chip_8.on_key(0x0, false),
                        Keycode::C => chip_8.on_key(0xB, false),
                        Keycode::V => chip_8.on_key(0xF, false),

                        _ => {}
                    }
                }
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