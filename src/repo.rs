use ini::Ini;

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