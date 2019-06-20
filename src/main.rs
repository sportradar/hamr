mod shellout;
mod note;
mod repo;

use structopt::StructOpt;
use failure::ResultExt;
use exitfailure::ExitFailure;
use std::path::PathBuf;
use std::io::Write;

#[derive(StructOpt)]
#[structopt(name = "hamr", rename_all = "kebab-case",
about = "The build tool that let's you create, share and import secrets with your Git repositories easily via LastPass")]
enum Hamr {
    #[structopt(name = "login", about = "Logs in to LastPass")]
    Login {
        username: String,
    },
    #[structopt(name = "save", about = "Saves files and environment variables to LastPass")]
    Save {
        /// Add the files to be uploaded
        #[structopt(parse(from_os_str), long = "files", short = "f")]
        files: Vec<PathBuf>,
        /// Add the environment variables to be uploaded
        #[structopt(long = "env", short = "e")]
        env_variables: Vec<String>,
    },
    #[structopt(name = "load", about = "Load files and environment variables from LastPass")]
    Load {

    },
}

fn main() {
    let args = Hamr::from_args();

    match args {
        Hamr::Login { username } => {
            shellout::login(&username).expect("Could not login using lpass");
        }
        Hamr::Save { files, env_variables } => {
            let note = note::Note::from(files, env_variables);
            shellout::save_data(
                repo::get_origin().unwrap().as_str(),
                serde_json::to_string(&note).unwrap().as_str()).expect("Could not save note")
            );
        },
        Hamr::Load {} => {
            let repo_name = repo::get_origin().expect("Could not find .git/origin (are you in repo root?)");
            let entry = match shellout::find_note(&repo_name).expect("Search for note failed.") {
                Some(entry) => entry,
                None => {
                    println!("No note exist for this repository (Looked for {:?}).", shellout::note_name(&repo_name));
                    std::process::exit(1);
                },
            };

            let work_dir = std::env::current_dir().expect("No current dir found?");

            let note = entry.load().expect("Failed to load entry.");
            let note = serde_json::from_str::<note::Note>(&note).expect("Failed to parse note.");

            for config_file in note.config_files.iter() {
                if config_file.path.is_absolute() {
                    println!("{} - is an absolute path, skipping.", config_file.path.to_str().unwrap());
                    continue;
                }

                if config_file.path.is_file() {
                    println!("{} - already exists, skipping. (delete it and run this again to overwrite)", config_file.path.to_str().unwrap());
                    continue;
                }

                let mut file = std::fs::File::create(work_dir.join(&config_file.path)).expect("Could not open file");
                file.write_all(config_file.secrets.join("\n").as_bytes()).expect("Could not write to file");

                println!("{} - successfully loaded", config_file.path.to_str().unwrap());
            }
        }
    }
}
