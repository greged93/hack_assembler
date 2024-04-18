use std::{marker::PhantomData, path::PathBuf};

use crate::{
    code::{a_value_to_binary, comp_to_binary, dest_to_binary, jump_to_binary},
    parser::{InstructionType, Parser},
    symbol_table::SymbolTable,
};

pub struct Uninitialized;
pub struct Initialized;

pub struct Assembler<T> {
    parser: Parser,
    symbol_table: SymbolTable,
    output_path: PathBuf,
    _phantom: std::marker::PhantomData<T>,
}

const C_PREFIX: &str = "111";

impl Assembler<Uninitialized> {
    /// Returns a new Assembler instance with the given path.
    pub fn new(path: PathBuf) -> Self {
        let mut output_path = path.clone();
        output_path.set_extension("hack");

        let parser = Parser::new(path);
        Self {
            parser,
            symbol_table: SymbolTable::new(),
            output_path,
            _phantom: PhantomData,
        }
    }

    /// Fills the symbol table with the labels from the program.
    #[must_use]
    pub fn fill_symbol_table(mut self) -> Assembler<Initialized> {
        // Clone the parser otherwise the rest of the code will consume
        // the program.
        let mut parser = self.parser.clone();

        while parser.has_more_lines() {
            // Consumes the parser
            parser.advance();

            if matches!(parser.instruction_type(), InstructionType::L) {
                self.symbol_table
                    .add_label(parser.symbol(), parser.current_line());
            }
        }

        Assembler {
            parser: self.parser,
            symbol_table: self.symbol_table,
            output_path: self.output_path,
            _phantom: PhantomData,
        }
    }
}

impl Assembler<Initialized> {
    /// Compiles the program and writes the output to the output path.
    pub fn compile(mut self) {
        let mut compiled_output = String::new();
        while self.parser.has_more_lines() {
            self.parser.advance();
            let bits = match self.parser.instruction_type() {
                InstructionType::A => {
                    let symbol = self.parser.symbol();
                    let symbol = self.add_variable(symbol);
                    a_value_to_binary(symbol)
                }
                InstructionType::C => {
                    let dest = self.parser.dest();
                    let comp = self.parser.comp();
                    let jump = self.parser.jump();
                    C_PREFIX.to_string()
                        + &comp_to_binary(comp)
                        + &dest_to_binary(dest)
                        + &jump_to_binary(jump)
                }
                InstructionType::L => continue,
            };
            compiled_output += &(bits + "\n");
        }

        std::fs::write(self.output_path, compiled_output).expect("failed to write compiled output");
    }

    /// Adds the variable symbol to the symbol table and returns the decimal value for it.
    #[must_use]
    fn add_variable(&mut self, symbol: String) -> String {
        if let Some(x) = self.symbol_table.address(&symbol) {
            x.to_string()
        }
        // If the symbol isn't numeric, we can assume it's a variable
        else if str::parse::<u32>(&symbol).is_err() {
            self.symbol_table.add_variable(symbol).to_string()
        } else {
            symbol
        }
    }
}
