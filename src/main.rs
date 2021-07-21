mod isa;
mod lexer;
mod cpu;

// use std::env;
// use std::io::Write;
// use std::fs;
use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

fn main()
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
        sp: 0,
        pc: 0x600,
        sr: 0,
        mem: [0;1<<16],
    };

    let data = fs::read(&args[1]).expect("Unable to read file");

    for i in 0..data.len()
    {
        cpu.mem[usize::from(cpu.pc + i as u16)] = data[i];
    }

    // Initialize sdl2
    // let sdl_context = sdl2::init()?;
    // let video_subsystem = sdl_context.video()?;

    while cpu.pc < ((data.len() as u16) + 0x600)
    {
        cpu.step();
        cpu.print_regs();
        println!("--------------");
    }
}
