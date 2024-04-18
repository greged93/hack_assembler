use std::{iter::Peekable, path::PathBuf, vec::IntoIter};

#[derive(Clone)]
pub struct Parser {
    program: Peekable<IntoIter<String>>,
    current_instruction: Option<String>,
    current_line: u32,
}

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
            current_line: 0,
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
            self.current_line += 1;
        }
    }

    /// Returns the current instruction type.
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
    pub fn symbol(&self) -> String {
        let instruction_type = self.instruction_type();
        let instruction = self
            .current_instruction
            .clone()
            .expect("expected instruction");
        let symbol = match instruction_type {
            InstructionType::A => instruction.trim_start_matches('@'),
            InstructionType::L => instruction.trim_start_matches('(').trim_end_matches(')'),
            InstructionType::C => panic!("symbol cannot be called on C instruction type"),
        };
        symbol.to_string()
    }

    /// Return the dest for a C instruction.
    ///
    /// # Panic
    ///
    /// Panics if the current instruction is not a C instruction.
    pub fn dest(&self) -> String {
        self.assert_is_instruction(InstructionType::C);

        let instruction = self
            .current_instruction
            .clone()
            .expect("expected instruction");
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
    ///
    /// # Panic
    ///
    /// Panics if the current instruction is not a C instruction.
    pub fn comp(&self) -> String {
        self.assert_is_instruction(InstructionType::C);

        let instruction = self
            .current_instruction
            .clone()
            .expect("expected instruction");
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
    ///
    /// # Panic
    ///
    /// Panics if the current instruction is not a C instruction.
    pub fn jump(&self) -> String {
        self.assert_is_instruction(InstructionType::C);

        let instruction = self
            .current_instruction
            .clone()
            .expect("expected instruction");
        if !instruction.contains(';') {
            return String::default();
        }
        instruction
            .split(';')
            .last()
            .expect("missing jump item")
            .to_string()
    }

    fn assert_is_instruction(&self, expected_instruction_type: InstructionType) {
        if self.instruction_type() != expected_instruction_type {
            panic!(
                "expected {:?} got {:?}",
                expected_instruction_type,
                self.instruction_type()
            )
        }
    }

    /// Returns the current line.
    pub fn current_line(&self) -> u32 {
        self.current_line
    }
}
