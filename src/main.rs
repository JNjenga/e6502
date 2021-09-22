extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

mod isa;
mod lexer;
mod cpu;

// use std::env;
// use std::io::Write;
// use std::fs;
use std::env;
use std::fs;
use rand::Rng;

#[inline(always)]
fn get_pixel_color(value : u8) -> Color
{
    match value & 0b00001111
    {

        0 => { return Color::RGB(0, 0, 0); },        // Black
        1 => { return Color::RGB(255, 255, 255); },  // White
        2 => { return Color::RGB(255, 0, 0); },      // Red
        3 => { return Color::RGB(0, 255, 255); },    // Cyan
        4 => { return Color::RGB(128, 0, 128); },    // Purple
        5 => { return Color::RGB(0, 255, 0); },      // Green
        6 => { return Color::RGB(0, 0, 255); },      // Blue
        7 => { return Color::RGB(255, 255, 0); },    // Yellow
        8 => { return Color::RGB(255, 165, 0); },    // Orange
        9 => { return Color::RGB(165, 42, 42); },    // Brown
        10 => { return Color::RGB(255, 119, 119); }, // Light Red
        11 => { return Color::RGB(169,169, 169); },  // Dark Grey 
        12 => { return Color::RGB(128, 128, 128); }, // Grey
        13 => { return Color::RGB(144, 238, 144); }, // Light green
        14 => { return Color::RGB(173, 216, 230); }, // Light blue
        15 => { return Color::RGB(211, 211, 211); }, // Light grey 
        _ => { return Color::RGB(0, 0, 0); }
    }
}

fn main() -> Result<(), String>
{
    let args: Vec<String> = env::args().collect();

    if args.len() < 2
    {
        // TODO(James) : Better arg handling/Errors?
        println!("Example : e6502 program.bin");
        panic!();
    }

    // Initialize cpu
    let mut cpu = cpu::Cpu
    {
        a: 0,
        x: 0,
        y: 0,
        sp: 0xff,
        pc: 0x600,
        sr: 0b00110000,
        mem: [0;1<<16],
    };

    let data = fs::read(&args[1]).expect("Unable to read file");

    for i in 0..data.len()
    {
        cpu.mem[usize::from(cpu.pc + i as u16)] = data[i];
    }

    const SCREEN_HEIGHT: u32 = 640;
    const SCREEN_WIDTH: u32 = 640;

    // Initialize sdl2
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("6502 Emulator by JNjenga", SCREEN_WIDTH, SCREEN_HEIGHT)
        .resizable()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .build()
        .map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(255, 0, 0));
    canvas.clear();
    let mut event_pump = sdl_context.event_pump()?;

    let pc_max = data.len() as u16 + 0x600 + 0x01;
    let mut rng = rand::thread_rng();

    'running: loop {
        if cpu.pc < pc_max && cpu.pc >= 0x600
        {
            cpu.mem[0xfe] = rng.gen();

            cpu.step();
            // cpu.print_regs();

            canvas.set_draw_color(Color::RGB(255, 0, 0));

            // Draw monitor
            for row in 0..32
            {
                for col in 0..32
                {
                    let color = get_pixel_color(cpu.mem[0x200 + (32 * row + col)]);
                    canvas.set_draw_color(color);
                    canvas.fill_rect(Rect::new(col as i32 * 20_i32, row as i32 * 20_i32, 20, 20))?;
                }
            }

            canvas.present();
        }

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

        // ::std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 3));
    }

    Ok(())
}
