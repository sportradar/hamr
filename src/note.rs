use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Note {
    pub repository: String,
    pub config_files: Vec<ConfigFile>,
    pub env_variables: std::collections::HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigFile {
    pub path: std::path::PathBuf,
    pub secrets: Vec<String>,
}

#[test]
fn serialize() {
    let expected = r#"{
      "repository": "git@github.com:kollstrom/hamr.git",
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
    assert_eq!(serialized.replace(" ", ""), expected.as_str())

}
