pub struct HexDump {
    rows: Vec<u8>,
    bytes: Vec<u8>,
    strings: Vec<char>,
}

impl HexDump {
    pub fn new() -> Self {
        HexDump {
            rows: vec![],
            bytes: vec![],
            strings: vec![],
        }
    }
    pub fn fill_strings(&mut self, characters: Vec<char>) {
        for character in characters {
            self.strings.push(character)
        }
    }

    pub fn fill_rows(&mut self, rows: Vec<u8>) {
        for row in rows {
            self.rows.push(row)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut hex_dump = HexDump::new();
        hex_dump.fill_rows([9, 2, 1, 3, 7].to_vec());
        assert_eq!(hex_dump.rows, [9, 2, 1, 3, 7]);
    }
}
