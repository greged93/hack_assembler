/// Convert Hack assembly language A-instruction to binary
pub fn a_value_to_binary(instruction: String) -> String {
    let val = str::parse::<u32>(&instruction).expect("failed to parse A instruction");
    format!("{:016b}", val)
}

/// Convert Hack assembly language C-instruction dest part to binary
pub fn dest_to_binary(instruction: String) -> String {
    let mut dest: u8 = if instruction.contains('M') { 1 } else { 0 };
    dest += if instruction.contains('D') { 2 } else { 0 };
    dest += if instruction.contains('A') { 4 } else { 0 };

    format!("{:03b}", dest)
}

/// Convert Hack assembly language C-instruction comp part to binary
pub fn comp_to_binary(instruction: String) -> String {
    let prefix = if instruction.contains('M') { "1" } else { "0" }.to_string();
    let comp = match instruction.as_str() {
        "0" => "101010",
        "1" => "111111",
        "-1" => "111010",
        "D" => "001100",
        "A" | "M" => "110000",
        "!D" => "001101",
        "!A" | "!M" => "110001",
        "-D" => "001111",
        "-A" | "-M" => "110011",
        "D+1" => "011111",
        "A+1" | "M+1" => "110111",
        "D-1" => "001110",
        "A-1" | "M-1" => "110010",
        "D+A" | "D+M" => "000010",
        "D-A" | "D-M" => "010011",
        "A-D" | "M-D" => "000111",
        "D&A" | "D&M" => "000000",
        "D|A" | "D|M" => "010101",
        _ => panic!("unexpected comp"),
    };

    prefix + comp
}

/// Convert Hack assembly language C-instruction jump part to binary
pub fn jump_to_binary(instruction: String) -> String {
    let dest = match instruction.as_str() {
        "" => "000",
        "JGT" => "001",
        "JEQ" => "010",
        "JGE" => "011",
        "JLT" => "100",
        "JNE" => "101",
        "JLE" => "110",
        "JMP" => "111",
        _ => panic!("unexpected dest"),
    };

    dest.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a_value_to_binary() {
        //Given
        let value = String::from("12345");

        // When
        let binary = a_value_to_binary(value);

        // Then
        assert_eq!("0011000000111001", binary);
    }
}
