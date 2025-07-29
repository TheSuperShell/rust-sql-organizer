use std::path::{Path, PathBuf};

use clap::Parser;
use rust_sql_organizer::searcher::{get_all_files, EmptyFileExtensionError, FileExtension};
use rust_sql_organizer::sorter::{Key, SORTING_STRATEGIES};

#[derive(Clone, Debug)]
enum CliError {
    ExtensionError,
    FileError,
    SortingStratError,
}

#[derive(Parser)]
struct Cli {
    /// Path, where the file search should accure
    path: Option<PathBuf>,

    /// File extensions that will be used
    #[arg(short, long)]
    extension: Option<Vec<String>>,

    /// Sorting Strategy. Default: first_number, folder
    #[arg(short, long)]
    sorter: Option<Vec<String>>,
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
    let mut files = match get_all_files(&path, &extensions) {
        Ok(files) => files,
        Err(_) => return Err(CliError::FileError),
    };
    let sorting_strats: Vec<&fn(&PathBuf) -> Key> = match args
        .sorter
        .unwrap_or(vec!["folder".to_string(), "first_number".to_string()])
        .iter()
        .map(|strat| SORTING_STRATEGIES.get(strat))
        .rev()
        .collect()
    {
        Some(strats) => strats,
        None => return Err(CliError::SortingStratError),
    };

    for sorting_start in sorting_strats {
        files.sort_by_key(|path| sorting_start(path));
    }
    for file in files {
        println!("File: {:?}", file)
    }
    Ok(())
}
