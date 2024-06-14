use clap::{Parser, Subcommand};
use std::error::Error;
use std::fs::File;
use std::io;
use std::path::Path;

use crate::dot_renderer::render_dot;
use crate::parser::{parse_program, Parser as RpgleParser};
use crate::texttree_renderer::render_text_tree;
use rpgle_lexer::new_lexer;

#[derive(Subcommand, Debug)]
enum Command {
    TextTree,
    Dot,
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// command: what action to perform
    #[command(subcommand)]
    command: Command,

    /// input: either stdin or filepath
    #[clap(value_parser, default_value = "-", global = true)]
    input: String,

    /// output: either stdout or filepath
    #[clap(value_parser, default_value = "-", global = true)]
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

pub fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    match args.command {
        Command::TextTree => {
            let input = read_input(args.input.as_str())?;
            let lexer = new_lexer(&input);
            let parser = RpgleParser::new(&lexer).unwrap();
            let pgm = parse_program(&parser)?;
            let output = render_text_tree(pgm);
            write_output(&output, args.output.as_str())?;
        }
        Command::Dot => {
            let input = read_input(args.input.as_str())?;
            let lexer = new_lexer(&input);
            let parser = RpgleParser::new(&lexer).unwrap();
            let pgm = parse_program(&parser)?;
            let output = render_dot(pgm);
            write_output(&output, args.output.as_str())?;
        }
    }
    Ok(())
}
