
struct Note {
    config_files: Vec<ConfigFile>,
    env_variables: std::collections::HashMap<String, String>,
}

struct ConfigFile {
    path: std::path::PathBuf,
    secrets: Vec<String>,
}

