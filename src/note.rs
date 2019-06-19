use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Note {
    repository: String,
    config_files: Vec<ConfigFile>,
    env_variables: std::collections::HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConfigFile {
    path: std::path::PathBuf,
    secrets: Vec<String>,
}

#[test]
fn serialize() {
    let expected = r#"{
      "repository": "git@github.com:kollstrom/hamr.git",
      "config_files": [
          {
            "path": "~/my-project/src/resources/application-dev.properties",
            "secrets": ["PORT: 8998"]
          }
        ],
      "env_variables": {
          "DB_USERNAME": "admin",
          "DB_PASSWORD": "sprscrt
        }
    }"#.trim_start();
    let mut env_vars = HashMap::new();
    env_vars.insert(String::from("DB_USERNAME"), String::from("admin"));
    env_vars.insert(String::from("DB_PASSWORD"), String::from("sprscrt"));
    let note = Note {
        repository: String::from("git@github.com:kollstrom/hamr.git"),
        config_files: vec![
            ConfigFile {
                path: std::path::PathBuf::from("~/my-project/src/resources/application-dev.properties"),
                secrets: vec![String::from("PORT: 8998")]
            }
        ],
        env_variables: env_vars
    };
    let serialized = serde_json::to_string(&note).unwrap();
    assert_eq!(serialized, expected)

}
