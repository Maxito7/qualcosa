#![deny(dead_code, missing_docs, missing_debug_implementations)]

use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Debug)]
/// We define the HexDump struct
pub struct HexDump {
    bytes: Vec<u8>,
    bytes_str: Vec<String>,
    total_bytes: u64,
    strings: Vec<char>,
}

impl Default for HexDump {
    fn default() -> Self {
        Self::new()
    }
}

impl HexDump {
    /// Associated function to create a new HexDump element
    pub fn new() -> Self {
        HexDump {
            bytes: vec![],
            bytes_str: vec![],
            total_bytes: 0,
            strings: vec![],
        }
    }

    /// Method to fill the HexDump object with the strings translated from the bytes
    pub fn fill_strings(&mut self, characters: Vec<char>) {
        for character in characters {
            self.strings.push(character);
            self.total_bytes = self.strings.len() as u64;
        }
    }

    /// Method to fill the HexDump object with the bytes read from the file
    pub fn fill_bytes(&mut self, file: &File, store_strings: bool) {
        let mut f = BufReader::new(file);
        let mut buf = Vec::<u8>::new();
        while f.read_until(b'\n', &mut buf).expect("read_until failed!") != 0 {
            let s = String::from_utf8(buf).expect("from_utf8 failed");
            for c in s.chars() {
                println!("Character: {}", c);
                self.bytes.push(c as u8);
            }
            buf = s.into_bytes();
            buf.clear();
        }
        // Here we store the bytes formatted to strings in case it is requested
        if store_strings {
            self.bytes_str = Vec::with_capacity(self.bytes.len());
            self.bytes_str = self
                .bytes
                .iter()
                .map(|byte| format!("{:02X}", byte))
                .collect();
        }
    }

    /// Method to get a reference to the stored bytes in numerical value
    pub fn get_bytes(&self) -> &Vec<u8> {
        &self.bytes
    }

    /// Method to get a reference to the stored bytes in string value
    pub fn get_bytes_str(&self) -> &Vec<String> {
        &self.bytes_str
    }

    /// Prints the bytes in string form
    pub fn print_bytes_str(&self) {
        for byte in &self.bytes_str {
            println!("{:?}", byte)
        }
    }
}
