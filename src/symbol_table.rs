use std::collections::HashMap;

#[derive(Default)]
pub struct SymbolTable {
    table: HashMap<String, u32>,
    current_address: u32,
}

impl SymbolTable {
    /// Create a new SymbolTable with the default values.
    pub fn new() -> Self {
        Self {
            current_address: 16,
            table: [
                (String::from("R0"), 0),
                (String::from("R1"), 1),
                (String::from("R2"), 2),
                (String::from("R3"), 3),
                (String::from("R4"), 4),
                (String::from("R5"), 5),
                (String::from("R6"), 6),
                (String::from("R7"), 7),
                (String::from("R8"), 8),
                (String::from("R9"), 9),
                (String::from("R10"), 10),
                (String::from("R11"), 11),
                (String::from("R12"), 12),
                (String::from("R13"), 13),
                (String::from("R14"), 14),
                (String::from("R15"), 15),
                (String::from("SP"), 0),
                (String::from("LCL"), 1),
                (String::from("ARG"), 2),
                (String::from("THIS"), 3),
                (String::from("THAT"), 4),
                (String::from("SCREEN"), 16384),
                (String::from("KBD"), 24576),
            ]
            .into_iter()
            .collect(),
        }
    }

    /// Add a label to the symbol table.
    pub fn add_label(&mut self, symbol: String, address: u32) {
        self.table.insert(symbol, address);
    }

    /// Add a variable to the symbol table, using the current address.
    pub fn add_variable(&mut self, symbol: String) -> u32 {
        self.table.insert(symbol, self.current_address);
        self.current_address += 1;
        self.current_address - 1
    }

    /// Get the address of a symbol in the symbol table.
    /// If the symbol is not in the table, return None.
    pub fn address(&self, symbol: &str) -> Option<&u32> {
        self.table.get(symbol)
    }
}
