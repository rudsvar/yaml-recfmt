use clap::Parser;
use std::{
    fs::File,
    io::{Read, Write},
};

/// Recursively format YAML files.
#[derive(Parser)]
pub struct Args {
    /// The input file to read from. Defaults to standard in.
    file: Option<String>,
    /// Overwrite the file in-place.
    #[arg(short, long)]
    in_place: bool,
}

fn read_input(args: &Args) -> std::io::Result<String> {
    let input: Box<dyn Read> = match &args.file {
        Some(file) => Box::new(File::open(file)?),
        None => Box::new(std::io::stdin()),
    };
    std::io::read_to_string(input)
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
        Some(file) if args.in_place => Box::new(File::create(file)?),
        _ => Box::new(std::io::stdout()),
    };

    // Write to output
    output.write_all(formatted.as_bytes())?;

    Ok(())
}
