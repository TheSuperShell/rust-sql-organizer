use std::{fs, path::Path};

#[derive(Debug, Clone)]
pub struct SqlFileError(String);

#[derive(Debug, Clone)]
pub struct SqlFile {
    file_name: String,
    sql_text: String,
}

impl SqlFile {
    pub fn new(path: &Path) -> Result<SqlFile, SqlFileError> {
        let file_name = match path.file_stem() {
            Some(name) => name.to_str().unwrap().to_string(),
            None => return Err(SqlFileError("Could not get file name".to_string())),
        };
        let sql_text = match fs::read_to_string(path) {
            Ok(val) => val,
            Err(e) => return Err(SqlFileError(format!("Could not open the file: {}", e))),
        };
        Ok(SqlFile {
            file_name,
            sql_text,
        })
    }

    pub fn get_file_name(&self) -> &str {
        self.file_name.as_str()
    }

    pub fn get_sql_text(&self) -> &str {
        self.sql_text.as_str()
    }
}

#[cfg(test)]
mod test_sql_file {
    use std::path::Path;
    #[cfg(test)]
    use std::path::PathBuf;

    #[cfg(test)]
    use tempdir::TempDir;

    use super::SqlFile;
    use std::fs::File;
    use std::io::Write;

    #[cfg(test)]
    fn create_temp_file(prefix: &str, file_name: &str) -> (TempDir, PathBuf) {
        let tmp_dir = TempDir::new(prefix).unwrap();
        let file_path = tmp_dir.path().join(Path::new(file_name));
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "SELECT 1;").unwrap();
        (tmp_dir, file_path)
    }

    #[test]
    fn test_sql_file() {
        let (tmp_dir, file) = create_temp_file("test_sql_file", "test.sql");
        let result = SqlFile::new(&file);
        assert!(result.is_ok());
        let sql_file = result.unwrap();
        assert_eq!(sql_file.get_file_name(), "test");
        assert_eq!(sql_file.get_sql_text().trim(), "SELECT 1;");
        drop(tmp_dir);
    }

    #[test]
    fn test_sql_file_error() {
        let result = SqlFile::new(Path::new("not_exists.sql"));
        assert!(result.is_err())
    }
}
