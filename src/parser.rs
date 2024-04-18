use std::{iter::Peekable, path::PathBuf, vec::IntoIter};

#[derive(Clone)]
pub struct Parser {
    /// An iterator over the program lines.
    program: Peekable<IntoIter<String>>,
    /// The current instruction.
    current_instruction: Option<String>,
    /// The current line number.
    instruction_index: u32,
}

/// The type of instruction.
#[derive(PartialEq, Debug)]
pub enum InstructionType {
    A,
    C,
    L,
}

impl Parser {
    /// Create a new parser from a file path.
    pub fn new(path: PathBuf) -> Self {
        let program = std::fs::read_to_string(path).expect("failed to read file");

        let mut lines = Vec::with_capacity(program.lines().count());
        for line in program.lines() {
            lines.push(line.to_string())
        }

        let iterator = lines.into_iter().peekable();

        Self {
            program: iterator,
            current_instruction: None,
            instruction_index: 0,
        }
    }

    /// Returns wether the program has remaining lines.
    pub fn has_more_lines(&mut self) -> bool {
        self.program.peek().is_some()
    }

    /// Advance the program to the next executable instruction.
    /// Skips comments and empty lines.
    pub fn advance(&mut self) {
        while self
            .program
            .peek()
            .map(|line| line.replace(' ', ""))
            .map(|line| line.is_empty() || line.starts_with("//"))
            .unwrap_or_default()
        {
            self.program.next();
        }

        self.current_instruction = self.program.next().map(|c| c.replace(' ', ""));
        // We don't need to increment the line on L instructions
        if !matches!(self.instruction_type(), InstructionType::L) {
            self.instruction_index += 1;
        }
    }

    /// Returns the current instruction type.
    ///
    /// # Panic
    ///
    /// - Panics if there is no current instruction.
    /// - Panics if the current instruction is invalid (neither A, L, or C)
    pub fn instruction_type(&self) -> InstructionType {
        if let Some(instruction) = &self.current_instruction {
            if instruction.starts_with('@') {
                InstructionType::A
            } else if instruction.starts_with('(') {
                InstructionType::L
            } else if instruction.contains('=') || instruction.contains(';') {
                InstructionType::C
            } else {
                panic!("invalid instruction");
            }
        } else {
            panic!("no current instruction");
        }
    }

    /// Returns the current instruction symbol.
    ///
    /// # Panic
    ///
    /// Panics if the current instruction is not an A or L instruction.
    pub fn symbol(&self) -> String {
        let instruction_type = self.instruction_type();
        let instruction = self.current_instruction();
        let symbol = match instruction_type {
            InstructionType::A => instruction.trim_start_matches('@'),
            InstructionType::L => instruction.trim_start_matches('(').trim_end_matches(')'),
            InstructionType::C => panic!("symbol cannot be called on C instruction type"),
        };
        symbol.to_string()
    }

    /// Return the dest for a C instruction.
    /// C instructions are in the form of `dest=comp;jump`
    /// where `dest` and `jump` are optional.
    ///
    /// # Panic
    ///
    /// Panics if the current instruction is not a C instruction.
    pub fn dest(&self) -> String {
        self.assert_current_instruction(InstructionType::C);

        let instruction = self.current_instruction();
        if !instruction.contains('=') {
            return String::default();
        }
        instruction
            .split('=')
            .next()
            .expect("missing dest item")
            .to_string()
    }

    /// Return the comp for a C instruction.
    /// C instructions are in the form of `dest=comp;jump`
    /// where `dest` and `jump` are optional.
    ///
    /// # Panic
    ///
    /// - Panics if the current instruction is not a C instruction.
    /// - Panics if the comp item is missing.
    pub fn comp(&self) -> String {
        self.assert_current_instruction(InstructionType::C);

        let instruction = self.current_instruction();
        let comp = if instruction.contains('=') {
            instruction.split('=').nth(1)
        } else if instruction.contains(';') {
            instruction.split(';').next()
        } else {
            panic!("failed to find comp item");
        };

        comp.expect("missing comp item").to_string()
    }

    /// Return the jump for a C instruction.
    /// C instructions are in the form of `dest=comp;jump`
    /// where `dest` and `jump` are optional.
    ///
    /// # Panic
    ///
    /// Panics if the current instruction is not a C instruction.
    pub fn jump(&self) -> String {
        self.assert_current_instruction(InstructionType::C);

        let instruction = self.current_instruction();
        if !instruction.contains(';') {
            return String::default();
        }
        instruction
            .split(';')
            .last()
            .expect("missing jump item")
            .to_string()
    }

    fn assert_current_instruction(&self, expected_instruction_type: InstructionType) {
        if self.instruction_type() != expected_instruction_type {
            panic!(
                "expected {:?} got {:?}",
                expected_instruction_type,
                self.instruction_type()
            )
        }
    }

    /// Returns the index of the current instruction.
    pub fn instruction_index(&self) -> u32 {
        self.instruction_index
    }

    /// Returns the current instruction.
    ///
    /// # Panic
    ///
    /// Panics if there is no current instruction.
    fn current_instruction(&self) -> &str {
        self.current_instruction
            .as_ref()
            .expect("expected instruction")
            .as_str()
    }
}
