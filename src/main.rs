use std::path::{Path, PathBuf};

use clap::Parser;
use rust_sql_organizer::file_combiner::combine_sql_files;
use rust_sql_organizer::file_formatter::{
    FileFormatters, FILE_FORMATTER_NO_COMMENTS, STANDARD_FILE_FORMATTER,
};
use rust_sql_organizer::searcher::{self, get_all_files, FileExtension};
use rust_sql_organizer::sorter::{OrdFn, SORTING_STRATEGIES};
use rust_sql_organizer::sql_file::{self, SqlFile};

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

    /// Target file
    #[arg(short, long)]
    target: Option<PathBuf>,

    /// Overwrite the target file
    #[arg(short, long, default_value_t = false)]
    overwrite: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let extensions = args
        .extension
        .unwrap_or(vec!["sql".to_string()])
        .iter()
        .map(|x| FileExtension::new(x))
        .collect::<Result<Vec<FileExtension>, searcher::error::Error>>()?;
    let path = args.path.unwrap_or(Path::new(".").to_path_buf());
    let mut files = get_all_files(&path, &extensions)?;
    let sorting_strats: Vec<&OrdFn> = args
        .sorter
        .unwrap_or(vec!["folder".to_string(), "first_number".to_string()])
        .iter()
        .map(|strat| SORTING_STRATEGIES.get(strat))
        .rev()
        .collect::<Option<Vec<&OrdFn>>>()
        .expect("Invalud strategy name");

    for sorting_strat in sorting_strats {
        files.sort_by_key(|path| sorting_strat(path));
    }
    let sql_files = files
        .iter()
        .map(|file| SqlFile::new(&file))
        .collect::<Result<Vec<SqlFile>, sql_file::error::Error>>()?;
    let sql_file_formatter: &FileFormatters = match args.remove_comments {
        true => &FILE_FORMATTER_NO_COMMENTS,
        false => &STANDARD_FILE_FORMATTER,
    };
    let target = args
        .target
        .unwrap_or(Path::new("./target.sql").to_path_buf());

    if !args.overwrite && target.exists() {
        eprintln!("Target table already exists. Please use --overwrite flag in order to ovdrwrite this file");
        return Ok(());
    }

    combine_sql_files(&target, &sql_files, sql_file_formatter)?;
    println!("Success!");
    Ok(())
}
