mod shellout;
mod note;

use structopt::StructOpt;
use failure::ResultExt;
use exitfailure::ExitFailure;
use std::path::PathBuf;

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
}

fn main() {
    let args = Hamr::from_args();

    match args {
        Hamr::Login { username } => {
            shellout::login(&username).expect("Could not login using lpass");
        }
        Hamr::Save { files, env_variables } => {
            for file in files.iter() {
                // TODO: Add file to Note
                println!("Filename {:?}", file.to_str())
            }
            for var in env_variables.iter() {
                // TODO: Add vars to Note instance
                println!("Environment variable {}", var)
            }
            // TODO: call shellout method for saving of note
        }
    }
}
