use std::fs::File;
use std::io::Write;
use super::note::Note;
use super::shellout::LSEntry;
use std::path::PathBuf;

pub fn load(entry: &LSEntry, root: Option<PathBuf>) {
    let work_dir = match root {
        Some(path) => path,
        None => std::env::current_dir().expect("No current dir found?"),
    };

    let note = entry.load().expect("Failed to load entry.");
    let note = serde_json::from_str::<Note>(&note).expect("Failed to parse note.");

    for config_file in note.config_files.iter() {
        if config_file.path.is_absolute() {
            println!("{} - is an absolute path, skipping.", config_file.path.to_str().unwrap());
            continue;
        }

        if config_file.path.is_file() {
            println!("{} - already exists, skipping. (delete it and run this again to overwrite)", config_file.path.to_str().unwrap());
            continue;
        }

        let mut file = File::create(work_dir.join(&config_file.path)).expect("Could not open file");
        file.write_all(config_file.secrets.join("\n").as_bytes()).expect("Could not write to file");

        println!("{} - successfully loaded", config_file.path.to_str().unwrap());
    }
}