use ini::Ini;
use std::process::{Command, Stdio};
use std::path::PathBuf;

pub fn get_origin() -> Result<String, String> {
    match std::env::current_dir() {
        Ok(dir) => {
            let git_path = dir.join(".git/config");

            match Ini::load_from_file(git_path) {
                Ok(file) => {
                    for (sec, prop) in file.iter() {
                        if let Some(sec) = sec {
                            if sec != "remote \"origin\"" {
                                continue;
                            }

                            if let Some(value) = prop.get("url") {
                                return Ok(value.clone());
                            } else {
                                return Err("No url found in master".to_owned())
                            }
                        }
                    }

                    Err("Could not find origin path in .git/config".to_owned())
                }
                Err(err) => Err(format!("Could not load .git/config: {}", err))
            }
        },
        Err(err) => {
            Err(format!("Could not find current dir: {}", err))
        },
    }
}

fn get_folder_from_path(path: &str) -> String {
    let mut result: String = path.split('/').last().unwrap().to_owned();

    if result.ends_with(".git") {
        result.drain(result.len() - 4..);
    }

    result
}

fn exec_git(args: &[&str]) -> Result<(), String> {
    match Command::new("git").args(args.iter()).stdout(Stdio::inherit()).stderr(Stdio::inherit()).output() {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("git: {}", err)),
    }
}

pub fn git_clone(repository: &str, folder: &Option<String>) -> Result<PathBuf, String>  {
    let mut args: Vec<&str> = Vec::with_capacity(3);
    args.push("clone");
    args.push(repository);

    let mut folder_buf = String::with_capacity(64);
    let path = if let Some(folder) = folder {
        folder_buf.push_str(&folder);
        args.push(&folder_buf);

        std::env::current_dir().unwrap().join(folder.clone())
    } else {
        let folder = get_folder_from_path(repository);

        folder_buf.push_str(&folder);
        args.push(&folder_buf);

        std::env::current_dir().unwrap().join(folder.clone())
    };

    exec_git(&args)?;

    Ok(path)
}
