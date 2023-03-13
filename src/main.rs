use clap::Parser;
use std::{
    fs::File,
    io::{read_to_string, Read, Write},
};

#[derive(Parser)]
pub struct Args {
    // The input file to read from. Standard in by default.
    file: Option<String>,
    // The output file to write to. '-' means standard out.
    #[arg(short, long)]
    output: Option<String>,
}

fn read_input(args: &Args) -> color_eyre::Result<String> {
    let input: Box<dyn Read> = match &args.file {
        Some(file) => Box::new(File::open(file)?),
        None => Box::new(std::io::stdin()),
    };
    Ok(read_to_string(input)?)
}

fn main() -> color_eyre::Result<()> {
    tracing_subscriber::fmt().init();

    let args = Args::parse();

    let input = read_input(&args)?;

    // Find out where to write to
    let output: Box<dyn Write> = match &args.file {
        Some(file) => Box::new(File::create(file)?),
        None => Box::new(std::io::stdout()),
    };

    yaml_recfmt::run_format(&input, output)?;

    Ok(())
}
