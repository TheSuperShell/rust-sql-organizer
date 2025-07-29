use std::path::{Path, PathBuf};

use clap::Parser;
use rust_sql_organizer::file_formatter::{
    FileFormatters, FILE_FORMATTER_NO_COMMENTS, STANDARD_FILE_FORMATTER,
};
use rust_sql_organizer::searcher::{get_all_files, EmptyFileExtensionError, FileExtension};
use rust_sql_organizer::sorter::{Key, SORTING_STRATEGIES};
use rust_sql_organizer::sql_file::SqlFile;

#[derive(Clone, Debug)]
enum CliError {
    ExtensionError,
    FileError,
    SortingStratError,
    SqlFileError,
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

    /// Remove the sql comments from USE statements
    #[arg(short = 'r', long, default_value_t = false)]
    remove_comments: bool,
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
    let sql_files: Vec<SqlFile> = match files.iter().map(|file| SqlFile::new(&file)).collect() {
        Ok(files) => files,
        Err(_) => return Err(CliError::SqlFileError),
    };
    let sql_file_formatter: &FileFormatters = match args.remove_comments {
        true => &FILE_FORMATTER_NO_COMMENTS,
        false => &STANDARD_FILE_FORMATTER,
    };

    for file in sql_files {
        println!("File: {}", file.get_file_name())
    }
    Ok(())
}
