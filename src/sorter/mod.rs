use phf::{phf_map, Map};
use regex::Regex;
use std::{cmp::Ordering, path::PathBuf};

use lazy_static::lazy_static;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Key {
    Num(i32),
    Text(String),
}

impl Ord for Key {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Key::Num(a), Key::Num(b)) => a.cmp(b),
            (Key::Text(a), Key::Text(b)) => a.cmp(b),
            (Key::Num(_), Key::Text(_)) => Ordering::Less,
            (Key::Text(_), Key::Num(_)) => Ordering::Greater,
        }
    }
}

impl PartialOrd for Key {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub type OrdFn = fn(&PathBuf) -> Key;

lazy_static! {
    static ref NUMBER_RE: Regex = Regex::new("[0-9]+").unwrap();
    pub static ref SORTING_STRATEGIES: Map<&'static str, OrdFn> = phf_map! {
        "first_number" => |path| {
            let file_name = path.file_name().unwrap().to_str().unwrap_or("");
            let num: i32 = match NUMBER_RE.find(file_name) {
                Some(m) => m.as_str().parse().unwrap(),
                None => 0,
            };
            Key::Num(num)
        },
        "last_number" => |path| {
            let file_name = path.file_name().unwrap().to_str().unwrap_or("");
            let num: i32 = match NUMBER_RE.find_iter(file_name).last() {
                Some(m) => m.as_str().parse().unwrap(),
                None => 0,
            };
            Key::Num(num)
        },
        "folder" => |path| {
            let last_folder_name = match path.parent() {
                Some(p) => match p.file_name() {
                    Some(name) => name.to_str().unwrap(),
                    None => return Key::Num(0),
                },
                None => return Key::Num(0),
            };
            Key::Text(last_folder_name.to_string())
        }
    };
}

#[cfg(test)]
mod test_sorters {
    use std::path::Path;

    use super::{Key, SORTING_STRATEGIES};

    #[test]
    fn test_first_number() {
        let first_number_strat = SORTING_STRATEGIES.get("first_number").unwrap();
        assert_eq!(
            first_number_strat(&Path::new("folder/100_deploy_20.sql").to_path_buf()),
            Key::Num(100)
        );
    }

    #[test]
    fn test_last_number() {
        let strat = SORTING_STRATEGIES.get("last_number").unwrap();
        assert_eq!(
            strat(&Path::new("folder/100_deploy_20.sql").to_path_buf()),
            Key::Num(20)
        );
    }

    #[test]
    fn test_folder() {
        let strat = SORTING_STRATEGIES.get("folder").unwrap();
        assert_eq!(
            strat(&Path::new("folder/100_deploy_20.sql").to_path_buf()),
            Key::Text("folder".to_string())
        )
    }

    #[test]
    fn test_folder_no_folder() {
        let strat = SORTING_STRATEGIES.get("folder").unwrap();
        assert_eq!(
            strat(&Path::new("/100_deploy_20.sql").to_path_buf()),
            Key::Num(0)
        )
    }
}
