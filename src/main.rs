mod shellout;

use structopt::StructOpt;
use failure::ResultExt;
use exitfailure::ExitFailure;

#[derive(StructOpt)]
#[structopt(name = "hamr", about = "The build tool that let's you create, share and import secrets with your Git repositories easily via LastPass")]
enum Hamr {
    #[structopt(name = "login", about="Logs in to LastPass")]
    Login {
        username: String,
    }
}

fn main() {
    let args = Hamr::from_args();
}
