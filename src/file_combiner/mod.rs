use std::{fs::OpenOptions, io::Write, path::PathBuf};

use crate::{file_formatter::FileFormatters, sql_file::SqlFile};

#[derive(Clone, Debug)]
pub struct CombineError;

pub fn combine_sql_files(
    target: &PathBuf,
    files: &[SqlFile],
    formatter: &FileFormatters,
) -> Result<(), CombineError> {
    let mut file = match OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(target)
    {
        Ok(f) => f,
        Err(_) => return Err(CombineError),
    };
    let _: Vec<_> = match files
        .iter()
        .map(|x| formatter.format(x))
        .map(move |x| file.write(x.as_bytes()))
        .collect()
    {
        Ok(a) => a,
        Err(_) => return Err(CombineError),
    };
    Ok(())
}

#[cfg(test)]
mod test_file_combiner {
    use std::{
        fs::{self, File},
        io::Write,
        path::Path,
    };

    use tempdir::TempDir;

    use crate::{
        file_formatter::{format_name, FileFormatters},
        sql_file::SqlFile,
    };

    use super::combine_sql_files;

    #[test]
    fn test_combine_sql_files() {
        let tmp_dir = TempDir::new("test_combine_sql_files").unwrap();
        let mut files: Vec<SqlFile> = Vec::new();
        for i in 1..3 {
            let file_path = tmp_dir.path().join(Path::new(&format!("test_{}", i)));
            let mut file = File::create(&file_path).unwrap();
            file.write(b"SELECT 1;").unwrap();
            files.push(SqlFile::new(&file_path).unwrap());
        }
        let target = tmp_dir.path().join(Path::new("target.sql"));
        let result = combine_sql_files(&target, &files, &FileFormatters::test_new(format_name));
        assert!(result.is_ok());
        assert_eq!(fs::read_to_string(target).unwrap(), "-- test_1-- test_2");
    }
}
