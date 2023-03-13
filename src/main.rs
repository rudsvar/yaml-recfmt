use clap::Parser;
use std::{fs::File, io::Write};
use tracing_subscriber::EnvFilter;
use walkdir::{DirEntry, WalkDir};

/// Formats YAML-files.
#[derive(Debug, Parser)]
pub struct Args {
    /// The input file to read from. Defaults to standard in.
    files: Vec<String>,
    /// Overwrite the file in-place.
    #[arg(short, long)]
    in_place: bool,
    /// Recursively format YAML-formatted strings.
    #[arg(short, long)]
    recursive: bool,
}

/// Read from stdin and write to stdout
fn pipe() -> color_eyre::Result<()> {
    tracing::info!("Reading from stdin");
    let input = std::io::read_to_string(std::io::stdin())?;
    let formatted = yaml_recfmt::format_recursive(&input)?;
    tracing::info!("Writing to stdout");
    print!("{formatted}");
    Ok(())
}

/// Read from a file and write to stdout or back to the file.
fn read_from_file(path: &str, args: &Args) -> color_eyre::Result<()> {
    tracing::info!("Reading from {path}");

    // Format content of file
    let input = std::fs::read_to_string(path)?;
    let formatted = if args.recursive {
        yaml_recfmt::format_recursive(&input)
    } else {
        yaml_recfmt::format(&input)
    }?;

    // Find out where to write to
    let mut output: Box<dyn Write> = if args.in_place {
        tracing::info!("Writing to {path}");
        Box::new(File::create(path)?)
    } else {
        tracing::info!("Writing to stdout");
        Box::new(std::io::stdout())
    };

    // Write to output
    output.write_all(formatted.as_bytes())?;
    Ok(())
}

/// Checks if a file is hidden.
fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let args = Args::parse();
    tracing::debug!("{args:?}");

    if args.files.is_empty() {
        pipe()?;
    } else {
        args.files.iter().for_each(|root| {
            let paths = WalkDir::new(root)
                .into_iter()
                .filter_entry(|e| !is_hidden(e))
                .filter_map(|e| e.ok())
                .filter(|e| e.file_type().is_file())
                .filter_map(|e| e.path().to_str().map(|s| s.to_string()));
            for path in paths {
                if let Err(e) = read_from_file(&path, &args) {
                    tracing::warn!("Failed to process {path}: {}", e);
                }
            }
        });
    }

    Ok(())
}
