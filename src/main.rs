use clap::Parser;
use std::{
    fs::File,
    io::{Read, Write},
};

#[derive(Parser)]
pub struct Args {
    // The input file to read from. '-' means standard in.
    #[arg(short, long, default_value = "-")]
    input: String,
    // The output file to write to. '-' means standard out.
    #[arg(short, long, default_value = "-")]
    output: String,
}

fn main() -> color_eyre::Result<()> {
    tracing_subscriber::fmt().init();

    let args = Args::parse();

    // Find out where to read from
    let input: Box<dyn Read> = if args.input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(args.input)?)
    };

    // Find out where to write to
    let output: Box<dyn Write> = if args.output == "-" {
        Box::new(std::io::stdout())
    } else {
        Box::new(File::create(args.output)?)
    };

    yaml_recfmt::run_format(input, output)?;

    Ok(())
}
