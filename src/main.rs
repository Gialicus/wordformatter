use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
};

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author = "Gialicus", version = "0.0.1", about = "raw line stack to quoted comma separated", long_about = None)]
struct Args {
    #[clap(value_parser, short, long)]
    input: String,
    #[clap(value_parser, short, long)]
    output: String,
}

fn reader(filename: &str) -> BufReader<File> {
    let file = File::open(filename).expect("File should be exists");
    let reader = BufReader::new(file);
    reader
}

fn process_line(reader: BufReader<File>, output: &str) {
    let mut file_write = File::create(output).expect("Cannot create file");
    for (index, line) in reader.lines().enumerate() {
        let line = line.expect(format!("Cant read line at index: {}", index).as_str());
        writeln!(file_write, "\"{line}\",").expect("Cannot write to file");
    }
}

fn main() {
    let args = Args::parse();
    let reader = reader(&args.input);
    process_line(reader, &args.output);
}
