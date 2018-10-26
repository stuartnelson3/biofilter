extern crate clap;
use clap::{App, Arg};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, BufWriter, Lines, Result, Write};

fn main() {
    let matches = App::new("biofilter")
        .version("0.1.0")
        .author("Stuart Nelson <stuartnelson3@gmail.com>")
        .about("Filter files with based on a match")
        .arg(
            Arg::with_name("search")
                .short("s")
                .long("search")
                .value_name("TERM")
                .help("Text to search for. Case-insensitive.")
                .takes_value(true)
                .required(true),
        ).arg(
            Arg::with_name("input") // assign if let statement to file/stdin
                               .short("i")
                               .long("input")
                               .value_name("FILE")
                               .help("Input file to be filtered. If not set, reads from stdin.")
                               .takes_value(true),
        ).arg(
            Arg::with_name("output") // assign if let statement to file/stdout
                .short("o")
                .long("output")
                .value_name("FILE")
                .help("Output file to write to. If not set, writes to stdout.")
                .takes_value(true),
        ).get_matches();

    let r = Reader::new(matches.value_of("input").unwrap_or(""));
    let mut w = Writer::new(matches.value_of("output").unwrap_or(""));

    for line in r.lines() {
        match line {
            Ok(line) => w.write(line.as_bytes()),
            Err(_err) => continue,
        };
    }
}

struct Reader(Box<dyn BufRead>);

impl Reader {
    fn new(file_name: &str) -> Self {
        if let Ok(file) = File::open(file_name) {
            Reader(Box::new(BufReader::new(file)))
        } else {
            Reader(Box::new(BufReader::new(io::stdin())))
        }
    }

    fn lines(self) -> Lines<Box<dyn BufRead>> {
        self.0.lines()
    }
}

struct Writer(Box<dyn Write>);

impl Writer {
    fn new(file_name: &str) -> Self {
        if let Ok(file) = File::create(file_name) {
            Writer(Box::new(BufWriter::new(file)))
        } else {
            Writer(Box::new(BufWriter::new(io::stdout())))
        }
    }

    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.0.write(buf)
    }
}
