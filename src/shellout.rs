///

use std::error::Error;
use std::process::{Command, Stdio};
use std::fmt;

fn exec_lpass(args: &[&str]) -> Result<String, LPassError> {
    match Command::new("lpass").args(args.iter()).output() {
        Ok(out) => {
            match String::from_utf8(out.stdout) {
                Ok(out) => Ok(out),
                Err(err) => Err(LPassError::new("Could not parse stdout as valid utf8", Some(Box::from(err)))),
            }
        },
        Err(err) => Err(LPassError::new("Could not exec command", Some(Box::from(err)))),
    }
}

fn exec_lpass_no_output(args: &[&str]) -> Result<(), LPassError> {
    match Command::new("lpass").args(args.iter()).stdout(Stdio::inherit()).stderr(Stdio::inherit()).output() {
        Ok(_) => Ok(()),
        Err(err) => Err(LPassError::new("Could not exec command", Some(Box::from(err))))
    }
}

pub fn login(username: &str) -> Result<(), LPassError> {
    return exec_lpass_no_output(&["login", username])
}

pub fn ls() -> Result<Vec<LSEntry>, LPassError> {
    let stdout = exec_lpass(&["ls", "--color=never"])?;
    let mut result: Vec<LSEntry> = Vec::with_capacity(64);

    for line in stdout.lines() {
        let mut tokens = line.split(" [id: ");
        let mut current = LSEntry{name: String::new(), folders: Vec::with_capacity(2), id: 0};

        if let Some(name) = tokens.next() {
            for path_token in name.split('/') {
                if current.name.len() == 0 {
                    current.name = String::from(path_token);
                } else {
                    current.folders.push(current.name);
                    current.name = String::from(path_token)
                }
            }
        } else {
            return Err(LPassError::new(&format!("Invalid line: {}", line), None))
        }

        if let Some(id_str) = tokens.next() {
            if id_str.len() <= 1 {
                return Err(LPassError::new(&format!("No id in line: {}", line), None))
            }

            let without_brackets = &id_str[0..id_str.len() - 1];

            match without_brackets.parse::<u64>() {
                Ok(n) => {current.id = n;},
                Err(err) => {
                    return Err(LPassError::new(&format!("Invalid id in line: {}", line), Some(Box::new(err))))
                },
            }
        } else {
            return Err(LPassError::new(&format!("No id in line: {}", line), None))
        }

        result.push(current);
    }

    Ok(result)
}

#[derive(Debug)]
pub struct LSEntry {
    pub id: u64,
    pub name: String,
    pub folders: Vec<String>,
}

#[derive(Debug)]
pub struct LPassError{
    msg: String,
    cause: Option<Box<Error>>,
}

impl LPassError {
    fn new(msg: &str, e: Option<Box<Error>>) -> LPassError {
        LPassError{msg: String::from(msg), cause: e}
    }
}

impl fmt::Display for LPassError {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        if let Some(cause) = &self.cause {
            write!(f, "(LPass) {}: {}", self.msg, cause)
        } else {
            write!(f, "(LPass) {}", self.msg)
        }
    }
}
