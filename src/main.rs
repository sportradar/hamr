mod shellout;
mod note;
mod repo;
mod loader;

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
    Clone {
        /// The repository to download.
        repository: String,

        /// A folder to put the content in.
        folder: Option<String>,
    }
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
                &shellout::note_name(repo::get_origin().unwrap().as_str()),
                serde_json::to_string(&note).unwrap().as_str(),
            ).expect("Could not save note");
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

            loader::load(&entry, None);
        }
        Hamr::Clone {repository, folder} => {
            let entry = match shellout::find_note(&repository).expect("Search for note failed.") {
                Some(entry) => entry,
                None => {
                    println!("No note exist for this repository, use git clone instead if that's expected (Looked for {:?}).", shellout::note_name(&repository));
                    std::process::exit(1);
                },
            };

            let path = match repo::git_clone(&repository, &folder) {
                Ok(path) => path,
                Err(err) => {
                    println!("Could not run git clone: {}", err);
                    std::process::exit(1);
                }
            };

            loader::load(&entry, Some(path));
        }
    }
}
