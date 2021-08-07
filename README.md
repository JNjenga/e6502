# e6502 
**e6502** is an emulator that emulates the [6502 processor](https://en.wikipedia.org/wiki/MOS_Technology_6502). While it's being developed for recreational , it could be used as a teaching tool.

> The project is still being developed, might want to comeback after some time.

## Demo

Watch demo below :

[![Demo](https://img.youtube.com/vi/_xA88DMFr9M/0.jpg)](https://www.youtube.com/watch?v=_xA88DMFr9M)

## Features

- 32x32 screen display
- Step through code
- Registers and memory editing and viewing
  
## Tech Stack

- [Rust](https://www.rust-lang.org/)
- [Rust-SDL2](https://github.com/Rust-SDL2/rust-sdl2)

## Installation

The program has two binaries, a "compiler" called **e6502c** and the emulator called **e6502**

To build the compiler

```
cargo build --bin e6502c
```

To build the emulator 

```
cargo build --bin e6502
```

## Running
The compiler expects two cli arguments `source_code_path` and `binary_output`

```
e6502c tests/asmcode.asm out.bin
```

The emulator expects one cli argument, `binary_program_path`

```
e6502 out.bin
```

## Resources

 - [6502.org](6502.org/tutorials/6502opcodes.html)
 - [6502 assembly wikibook](https://en.wikibooks.org/wiki/6502_Assembly)
 - [6502 Instruction Set - Masswek.at](https://www.masswerk.at/6502/6502_instruction_set.html)

  
## Authors

- [@jnjenga](https://www.github.com/jnjenga)

  
## Feedback

If you have any feedback, please reach out to me via mail `me at jnjenga.com`
  
## Contributing

Contributions are always welcome!
 
## License

[MIT](https://choosealicense.com/licenses/mit/)



