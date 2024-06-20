mod cli;
mod cst;
mod parser;
mod renderer;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    cli::main()?;
    Ok(())
}
