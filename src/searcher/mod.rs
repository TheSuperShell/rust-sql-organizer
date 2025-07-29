use glob::glob;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct EmptyFileExtensionError;

#[derive(Clone, Debug)]
pub struct FileExtension {
    extension: String,
}

impl FileExtension {
    pub fn new(extension: &str) -> Result<FileExtension, EmptyFileExtensionError> {
        let extension = extension.trim();
        if extension.len() == 0 {
            return Err(EmptyFileExtensionError);
        }
        Ok(FileExtension {
            extension: extension.to_string(),
        })
    }

    fn get_glob(&self) -> String {
        return format!("**/*.{}", self.extension);
    }
}

pub fn get_all_files(
    path: &Path,
    file_formats: &[FileExtension],
) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let mut result: Vec<PathBuf> = Vec::new();
    for file_format in file_formats {
        let glob_str = file_format.get_glob();
        let pattern_path = path.join(Path::new(&glob_str));
        let pattern = match pattern_path.to_str() {
            Some(patter_str) => patter_str,
            None => continue,
        };
        let glob_result = glob(&pattern)?;
        for path_result in glob_result {
            match path_result {
                Ok(new_path) => result.push(new_path),
                Err(_) => continue,
            }
        }
    }
    Ok(result)
}

#[cfg(test)]
mod searcher_test {
    use super::{get_all_files, FileExtension};
    use std::fs::File;
    use std::path::Path;
    use tempdir::TempDir;

    #[test]
    fn test_file_extension() {
        let result = FileExtension::new("sql");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().extension, "sql")
    }

    #[test]
    fn test_file_extension_error() {
        let result = FileExtension::new("");
        assert!(result.is_err());
    }

    #[test]
    fn test_file_extension_get_glob() {
        let file_extension = FileExtension {
            extension: "sql".to_string(),
        };
        let glob = file_extension.get_glob();
        assert_eq!(glob, "**/*.sql")
    }

    #[cfg(test)]
    fn create_temp_files(prefix: &str, file_names: &[&str]) -> TempDir {
        let tmp_dir = TempDir::new(prefix).unwrap();
        for &file_name in file_names {
            File::create(tmp_dir.path().join(Path::new(file_name))).unwrap();
        }
        tmp_dir
    }

    #[test]
    fn test_get_all_files() {
        let files = ["test.sql", "test_2.sql", "test_3.txt", "test_4.snowsql"];
        let expected_files = ["test.sql", "test_2.sql", "test_4.snowsql"];
        let tmp_dir = create_temp_files("test_get_all_files", &files);
        let file_extensions = [
            FileExtension {
                extension: "sql".to_string(),
            },
            FileExtension {
                extension: "snowsql".to_string(),
            },
        ];
        let all_files = get_all_files(&tmp_dir.path(), &file_extensions);
        assert!(all_files.is_ok());
        for file in all_files.unwrap() {
            assert!(expected_files.contains(&file.file_name().unwrap().to_str().unwrap()));
            assert!(file.file_name().unwrap().to_str().unwrap() != "test_3.txt");
        }
    }
}
