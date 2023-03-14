use clap::Parser;
use color_eyre::eyre::Context;
use ignore::{DirEntry, Walk};
use std::{fs::File, io::Write};
use tracing::metadata::LevelFilter;
use tracing_subscriber::EnvFilter;

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
    tracing::info!("Processing stdin");
    let input = std::io::read_to_string(std::io::stdin())?;
    let formatted = yaml_recfmt::format::format_recursive(&input)?;
    print!("{formatted}");
    Ok(())
}

/// Read from a file and write to stdout or back to the file.
fn read_from_file(path: &str, args: &Args) -> color_eyre::Result<()> {
    tracing::info!("Processing {path}");

    // Format content of file
    let input = std::fs::read_to_string(path)?;
    let formatted = if args.recursive {
        yaml_recfmt::format::format_recursive(&input)
    } else {
        yaml_recfmt::format::format(&input)
    }?;

    // Find out where to write to
    let mut output: Box<dyn Write> = if args.in_place {
        Box::new(File::create(path)?)
    } else {
        Box::new(std::io::stdout())
    };

    // Write to output
    output.write_all(formatted.as_bytes())?;
    Ok(())
}

/// Check if a file is (likely) YAML.
fn is_yaml(entry: &DirEntry) -> bool {
    let ext = entry.path().extension().and_then(|e| e.to_str());
    ext == Some("yml") || ext == Some("yaml")
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env()
        .context("invalid RUST_LOG value")?;
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_env_filter(env_filter)
        .init();

    let args = Args::parse();
    tracing::debug!("{args:?}");

    if args.files.is_empty() {
        pipe()?;
    } else {
        // Iterate through file list
        args.files.iter().for_each(|root| {
            // If directory, recurse into it
            let paths = Walk::new(root)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(is_yaml)
                .filter_map(|e| e.path().to_str().map(|s| s.to_string()));
            // Try formatting all found files
            for path in paths {
                if let Err(e) = read_from_file(&path, &args) {
                    tracing::warn!("Failed to process {path}: {}", e);
                }
            }
        });
    }

    Ok(())
}
