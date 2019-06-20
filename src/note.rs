use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use std::fs::File;
use std::io::{Read, BufReader, BufRead};
use core::borrow::{BorrowMut, Borrow};

#[derive(Serialize, Deserialize, Debug)]
pub struct Note {
    pub config_files: Vec<ConfigFile>,
    pub env_variables: std::collections::HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigFile {
    pub path: std::path::PathBuf,
    pub secrets: Vec<String>,
}

impl Note {
    fn new(config_files: Vec<ConfigFile>, env_variables: HashMap<String, String>) -> Note {
        Note {
            config_files,
            env_variables,
        }
    }

    pub fn from(paths: Vec<PathBuf>, vars: Vec<String>) -> Note {
        Note::new(parse_files(paths), parse_env_vars(vars))
    }
}

impl PartialEq for ConfigFile {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path &&
            self.secrets == other.secrets
    }
}

fn parse_files(paths: Vec<PathBuf>) -> Vec<ConfigFile> {
    let mut configs = Vec::new();
    for p in paths.iter() {
        let file = File::open(p).unwrap();
        let reader = BufReader::new(file);
        let mut lines = Vec::new();
        for line in reader.lines() {
            let s = line.unwrap();
            lines.push(s);
        }
        let config = ConfigFile { path: PathBuf::from(p), secrets: lines };
        configs.push(config);
    }
    return configs;
}

fn parse_env_vars(vars: Vec<String>) -> std::collections::HashMap<String, String> {
    let mut env_vars = HashMap::new();
    for vars in vars.iter() {
        let split = vars.split("=").collect::<Vec<&str>>();
        env_vars.insert(String::from(split[0]), String::from(split[1]));
    }
    return env_vars;
}

#[test]
fn read_file() {
    let expected = vec![
        ConfigFile {
            path: PathBuf::from("/home/kollstrom/Programming/personal/hamr/README.md"),
            secrets: vec![
                String::from("# hamr"),
                String::from("The build tool that let's you create, share and import secrets with your Git repositories easily via LastPass.")
            ],
        }
    ];
    let parsed =
        parse_files(vec![PathBuf::from("/home/kollstrom/Programming/personal/hamr/README.md")]);
    assert_eq!(parsed, expected);
}

#[test]
fn split_vars() {
    let mut expected = HashMap::new();
    expected.insert(String::from("DB_USER"), String::from("admin"));
    expected.insert(String::from("DB_PASSWORD"), String::from("scrt"));
    let result = parse_env_vars(vec![String::from("DB_USER=admin"), String::from("DB_PASSWORD=scrt")]);
    assert_eq!(result, expected)
}

#[test]
fn serialize() {
    let expected = r#"{
      "config_files": [
          {
            "path": "~/my-project/src/resources/application-dev.properties",
            "secrets": ["PORT:8998"]
          }
        ],
      "env_variables": {
          "DB_USERNAME": "admin",
          "DB_PASSWORD": "sprscrt"
        }
    }"#;
    let expected = expected.replace("\n", "").replace(" ", "");
    let mut env_vars = HashMap::new();
    env_vars.insert(String::from("DB_USERNAME"), String::from("admin"));
    env_vars.insert(String::from("DB_PASSWORD"), String::from("sprscrt"));
    let note = Note {
        config_files: vec![
            ConfigFile {
                path: std::path::PathBuf::from("~/my-project/src/resources/application-dev.properties"),
                secrets: vec![String::from("PORT: 8998")],
            }
        ],
        env_variables: env_vars,
    };
    let serialized = serde_json::to_string(&note).unwrap();
    assert_eq!(serialized.replace(" ", ""), expected.as_str())
}
