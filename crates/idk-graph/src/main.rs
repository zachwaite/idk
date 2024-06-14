mod cli;
mod cst;
mod dot_renderer;
mod parser;
mod texttree_renderer;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    cli::main()?;
    Ok(())
}
