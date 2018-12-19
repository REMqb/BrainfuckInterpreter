mod bfi;

use std::io::BufReader;
use std::fs::File;
use std::io::Read;
use bfi::brain_fuck_interpreter::BrainFuckInterpreter;

use std::env;

fn main() -> Result<(), i32> {
    let mut args = env::args();

    println!("{}", b);

    if args.len() == 1 {
        println!("Brainfuck interpreter");
        println!();
        println!("Usage: brainfuck_interpreter <path>");

        return Ok(());
    }

    let file = File::open(args.next().unwrap());
    let mut reader = BufReader::new(file.unwrap());

    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer).unwrap();



    let mut interpreter = BrainFuckInterpreter::new();

    interpreter.load(buffer);

    match interpreter.run() {
        Ok(()) => return Ok(()),
        Err(e) => {
            println!("An error happened: {}", e);
            return  Err(1)
        }
    }

}