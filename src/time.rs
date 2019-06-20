use std::process::{Command, Stdio};

fn exec_ffplay(args: &[&str]) -> Result<(), String> {
    match Command::new("ffplay").args(args.iter()).stdout(Stdio::inherit()).stderr(Stdio::inherit()).output() {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("ffplay: {}", err)),
    }
}

pub fn run(file: &str) -> Result<(), String> {
    exec_ffplay(&["-hide_banner", "-loglevel", "warning", "-nodisp", "-autoexit", file])
}