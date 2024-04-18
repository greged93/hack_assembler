pub mod assembler;
pub mod code;
pub mod parser;
pub mod symbol_table;

use std::path::PathBuf;

use assembler::Assembler;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the input file
    #[arg(short, long)]
    input: PathBuf,
}

fn main() {
    let args = Args::parse();

    let assembler = Assembler::new(args.input);
    let assembler = assembler.fill_symbol_table();
    assembler.compile();
}
