use std::path::{Path, PathBuf};

use clap::Parser;
use rust_sql_organizer::searcher::{get_all_files, EmptyFileExtensionError, FileExtension};

#[derive(Clone, Debug)]
enum CliError {
    ExtensionError,
    FileError,
}

#[derive(Parser)]
struct Cli {
    /// Path, where the file search should accure
    path: Option<PathBuf>,

    /// File extensions that will be used
    #[arg(short = 'e', long)]
    extension: Option<Vec<String>>,
}

fn main() -> Result<(), CliError> {
    let args = Cli::parse();
    let extensions_result: Result<Vec<FileExtension>, EmptyFileExtensionError> = args
        .extension
        .unwrap_or(vec!["sql".to_string()])
        .iter()
        .map(|x| FileExtension::new(x))
        .collect();
    let extensions: Vec<FileExtension> = match extensions_result {
        Ok(ext) => ext,
        Err(_) => return Err(CliError::ExtensionError),
    };
    let path = args.path.unwrap_or(Path::new(".").to_path_buf());
    let files = match get_all_files(&path, &extensions) {
        Ok(files) => files,
        Err(_) => return Err(CliError::FileError),
    };
    for file in files {
        println!("File: {:?}", file)
    }
    Ok(())
}
