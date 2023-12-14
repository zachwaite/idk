// remove before committing
#![allow(unused)]
// remove before committing

mod parser;

use clap::{Parser, Subcommand};
use std::error::Error;
use std::fs::File;
use std::io;
use std::path::Path;
use parser::debug;

#[derive(Subcommand, Debug)]
enum Command {
    Debug,
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// command: what action to perform
    #[command(subcommand)]
    command: Command,

    /// input: either stdin or filepath
    #[clap(value_parser, default_value = "-", global=true)]
    input: String,

    /// output: either stdout or filepath
    #[clap(value_parser, default_value = "-", global=true)]
    output: String,
}

fn read_input(input: &str) -> Result<String, Box<dyn Error>> {
    let mut buf = String::new();
    let mut rdr: Box<dyn io::Read> = match input {
        "-" => Box::new(io::stdin()),
        _ => Box::new(File::open(input)?),
    };
    rdr.read_to_string(&mut buf)?;
    Ok(buf)
}

fn write_output(output: &str, target: &str) -> Result<(), Box<dyn Error>> {
    let mut writer: Box<dyn io::Write> = match target {
        "-" => Box::new(io::stdout()),
        _ => Box::new(File::create(&Path::new(target))?),
    };
    writer.write(output.as_bytes())?;
    Ok(())
}


fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    match args.command {
        Command::Debug => {
            let input = read_input(args.input.as_str())?;
            let output = debug(&input)?;
            write_output(&output, args.output.as_str())?;
        }
    }
    Ok(())
}
