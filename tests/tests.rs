extern crate qualcosa;

#[cfg(test)]

pub mod tests {
    use std::fs::File;
    use std::io::{self};

    use qualcosa::HexDump;

    #[test]
    fn txt_reading() -> io::Result<()> {
        let file =
            File::open("tests/data/test.txt").expect("Something went wrong when opening the file");

        let mut hex_dump = HexDump::new();
        hex_dump.fill_bytes(&file, true);
        hex_dump.print_bytes_str();
        //assert_eq!(hex_dump.get_bytes_str(), &["FF", "02", "01", "03", "07"]);

        Ok(())
    }
}
