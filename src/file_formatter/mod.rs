use lazy_static::lazy_static;

use crate::sql_file::SqlFile;
use regex::Regex;

type FileFormatterFunc = fn(&SqlFile) -> String;

pub struct FileFormatters {
    formatters: Vec<FileFormatterFunc>,
}

impl FileFormatters {
    pub fn format(&self, sql_file: &SqlFile) -> String {
        let strings: Vec<String> = self.formatters.iter().map(|&fmt| fmt(sql_file)).collect();
        strings.join("\n")
    }
}

pub fn format_name(sql_file: &SqlFile) -> String {
    format!("-- {}", sql_file.get_file_name())
}

pub fn format_split(_: &SqlFile) -> String {
    "\n".to_string()
}

pub fn format_plain_text(sql_file: &SqlFile) -> String {
    sql_file.get_sql_text().to_string()
}

pub fn format_no_comments(sql_file: &SqlFile) -> String {
    REMOVE_COMMENTS_RE
        .replace(sql_file.get_sql_text(), "USE")
        .to_string()
}

pub fn format_endln(sql_file: &SqlFile) -> String {
    format!(
        "--____________________ End of {} ____________________--",
        sql_file.get_file_name()
    )
}

lazy_static! {
    static ref REMOVE_COMMENTS_RE: Regex = Regex::new(r"(?mi)--\s+use").unwrap();
    pub static ref STANDARD_FILE_FORMATTER: FileFormatters = FileFormatters {
        formatters: vec![
            format_name,
            format_split,
            format_split,
            format_plain_text,
            format_split,
            format_split,
            format_endln,
            format_split
        ]
    };
    pub static ref FILE_FORMATTER_NO_COMMENTS: FileFormatters = FileFormatters {
        formatters: vec![
            format_name,
            format_split,
            format_split,
            format_no_comments,
            format_split,
            format_split,
            format_endln,
            format_split
        ]
    };
}

#[cfg(test)]
mod test_file_formatter {
    use crate::sql_file::SqlFile;

    use super::{format_endln, format_name, format_no_comments, format_plain_text, format_split};

    #[test]
    fn test_format_name() {
        assert_eq!(
            format_name(&SqlFile::test_new("test", "SELECT 1;")),
            "-- test"
        )
    }

    #[test]
    fn test_format_split() {
        assert_eq!(format_split(&SqlFile::test_new("test", "SELECT 1;")), "\n")
    }

    #[test]
    fn test_format_plain_text() {
        assert_eq!(
            format_plain_text(&SqlFile::test_new("test", "SELECT 1;")),
            "SELECT 1;"
        )
    }

    #[test]
    fn test_format_no_comments() {
        assert_eq!(
            format_no_comments(&SqlFile::test_new(
                "test",
                "-- use ROLE ACCOUNTADMIN;\nSELECT 1;"
            )),
            "USE ROLE ACCOUNTADMIN;\nSELECT 1;"
        )
    }

    #[test]
    fn test_format_endln() {
        assert_eq!(
            format_endln(&SqlFile::test_new("test", "SELECT 1;")),
            "--____________________ End of test ____________________--"
        )
    }
}
