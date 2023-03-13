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

fn read_input(args: &Args) -> std::io::Result<String> {
    let input: Box<dyn Read> = match &args.file {
        Some(file) => Box::new(File::open(file)?),
        None => Box::new(std::io::stdin()),
    };
    read_to_string(input)
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .init();

    let args = Args::parse();
    let input = read_input(&args)?;

    // Format the input
    let formatted = yaml_recfmt::format(&input)?;

    // Find out where to write to
    let mut output: Box<dyn Write> = match &args.file {
        Some(file) => Box::new(File::create(file)?),
        None => Box::new(std::io::stdout()),
    };

    // Write to output
    output.write_all(formatted.as_bytes())?;

    Ok(())
}
