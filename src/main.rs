use std::path::{Path, PathBuf};

use clap::Parser;
use log::{debug, error, info};
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

    #[command(flatten)]
    verbosity: clap_verbosity_flag::Verbosity,

    /// Write output to the log file
    out: Option<PathBuf>,
}

fn create_output_file(target: PathBuf) -> std::io::Result<std::fs::File> {
    std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(target)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let mut log_builder = env_logger::Builder::new();
    match args.out {
        Some(t) => log_builder.target(env_logger::Target::Pipe(Box::new(create_output_file(t)?))),
        None => log_builder.target(env_logger::Target::Stderr),
    };
    log_builder.filter_level(args.verbosity.into()).init();
    info!("Starting the app");
    let extensions = args
        .extension
        .unwrap_or(vec!["sql".to_string()])
        .iter()
        .map(|x| FileExtension::new(x))
        .collect::<Result<Vec<FileExtension>, searcher::error::Error>>()?;
    debug!("Collected all the extensions");
    let path = args.path.unwrap_or(Path::new(".").to_path_buf());
    debug!("Collected the path");
    let mut files = get_all_files(&path, &extensions)?;
    debug!("Found all the files");
    let sorting_strats: Vec<&OrdFn> = args
        .sorter
        .unwrap_or(vec!["folder".to_string(), "first_number".to_string()])
        .iter()
        .map(|strat| SORTING_STRATEGIES.get(strat))
        .rev()
        .collect::<Option<Vec<&OrdFn>>>()
        .expect("Invalud strategy name");
    debug!("Collected all the soring strategies");

    for sorting_strat in sorting_strats {
        files.sort_by_key(|path| sorting_strat(path));
    }
    debug!("Sorted the files");
    let sql_files = files
        .iter()
        .map(|file| SqlFile::new(&file))
        .collect::<Result<Vec<SqlFile>, sql_file::error::Error>>()?;
    debug!("Collected all the sql files");
    let sql_file_formatter: &FileFormatters = match args.remove_comments {
        true => &FILE_FORMATTER_NO_COMMENTS,
        false => &STANDARD_FILE_FORMATTER,
    };
    debug!("Collected the file formatters");
    let target = args
        .target
        .unwrap_or(Path::new("./target.sql").to_path_buf());
    debug!("Collected the target file");

    if !args.overwrite && target.exists() {
        error!("Target table already exists. Please use --overwrite flag in order to overwrite this file");
        return Ok(());
    }

    combine_sql_files(&target, &sql_files, sql_file_formatter)?;
    info!("Successfully created the file {:?}", target);
    Ok(())
}
