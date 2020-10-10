
#[allow(unused_imports)]
#[allow(unused_variables)]
#[allow(dead_code)]

mod cpu;

use std::{ env, fs::File, io::{ Read, Error } };
use cpu::Cpu;



fn main() -> Result<(), Error>
{
    let args: Vec<String> = env::args().collect();

    if args.len() != 2
    {
        panic!("Binary file to load is missing.");
    }

    let mut file = File::open(&args[1])?;
    let mut binary = Vec::new();

    file.read_to_end(&mut binary)?;

    let mut cpu = Cpu::new(binary);

    while cpu.pc < cpu.memory.len()
    {
        let instruction = cpu.fetch();

        cpu.pc = cpu.pc + 4;
        cpu.execute(&instruction);
    }

    //println!("{:?}", cpu);

    Ok(())
}
