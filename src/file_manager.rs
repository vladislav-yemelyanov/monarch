use std::{
    fs::File,
    io::{self, BufRead, BufReader, Seek, SeekFrom, Write},
};

pub fn read(file: &File) -> io::Result<Vec<String>> {
    let reader = BufReader::new(file);
    reader.lines().collect()
}

pub fn write(file: &mut File, lines: &[String]) -> io::Result<()> {
    file.set_len(0)?;
    file.seek(SeekFrom::Start(0))?;
    for line in lines {
        writeln!(file, "{}", line)?;
    }
    Ok(())
}
